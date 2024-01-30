use gl_client::signer::Signer;
use gl_client::tls::TlsConfig;
use anyhow::Result;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey};

use crate::models::*;
use crate::signer_client::{SignerClient, ClientResult};

pub(crate) struct GreenlightClient {
    config: SignerClientConfig,
    signer: Signer,
}

impl GreenlightClient {
    pub async fn connect(
	    config: SignerClientConfig,
        seed: Vec<u8>,
    ) -> Result<Self> {
		let signer = Signer::new(seed.clone(), config.network.into(), TlsConfig::new()?)?;
		// Derive the encryption key from the seed
		let encryption_key = Self::derive_bip32_key(
			config.network,
			&signer,
			vec![ChildNumber::from_hardened_idx(140)?, ChildNumber::from(0)],
		)?
		.to_priv()
		.to_bytes();

		let encryption_key_slice = encryption_key.as_slice();
        // TODO: encryption_key_sliceの使い道を考える
        println!("encryption_key_slice: {:?}", encryption_key_slice);

		Ok(Self {
			config,
			signer,
		})
	}

    /*
    fn sign_invoice(&self, invoice: RawBolt11Invoice) -> NodeResult<String> {
        let hrp_bytes = invoice.hrp.to_string().as_bytes().to_vec();
        let data_bytes = invoice.data.to_base32();

        // create the message for the signer
        let msg_type: u16 = 8;
        let data_len: u16 = data_bytes.len().try_into()?;
        let mut data_len_bytes = data_len.to_be_bytes().to_vec();
        let mut data_buf = data_bytes.iter().copied().map(u5::to_u8).collect();

        let hrp_len: u16 = hrp_bytes.len().try_into()?;
        let mut hrp_len_bytes = hrp_len.to_be_bytes().to_vec();
        let mut hrp_buf = hrp_bytes.to_vec();

        let mut buf = msg_type.to_be_bytes().to_vec();
        buf.append(&mut data_len_bytes);
        buf.append(&mut data_buf);
        buf.append(&mut hrp_len_bytes);
        buf.append(&mut hrp_buf);
        // Sign the invoice using the signer
        let raw_result = self.signer.sign_invoice(buf)?;
        info!(
            "recover id: {:?} raw = {:?}",
            raw_result, raw_result[64] as i32
        );
        // contruct the RecoveryId
        let rid = RecoveryId::from_i32(raw_result[64] as i32).expect("recovery ID");
        let sig = &raw_result[0..64];
        let recoverable_sig = RecoverableSignature::from_compact(sig, rid)?;

        let signed_invoice: Result<SignedRawBolt11Invoice> = invoice.sign(|_| Ok(recoverable_sig));
        Ok(signed_invoice?.to_string())
    }
	*/

    async fn sign_message(&self, message: &str) -> ClientResult<String> {
        let (sig, recovery_id) = self.signer.sign_message(message.as_bytes().to_vec())?;
        let mut complete_signature = vec![31 + recovery_id];
        complete_signature.extend_from_slice(&sig);
        Ok(zbase32::encode_full_bytes(&complete_signature))
    }

    fn derive_bip32_key(
        network: Network,
        signer: &Signer,
        path: Vec<ChildNumber>,
    ) -> ClientResult<ExtendedPrivKey> {
        Ok(
            ExtendedPrivKey::new_master(network.into(), &signer.bip32_ext_key())?
                .derive_priv(&Secp256k1::new(), &path)?,
        )
    }
}

#[tonic::async_trait]
impl SignerClient for GreenlightClient {
    async fn sign_message(&self, message: &str) -> ClientResult<String> {
        Self::sign_message(self, message).await
    }

    fn derive_bip32_key(&self, path: Vec<ChildNumber>) -> ClientResult<ExtendedPrivKey> {
        Self::derive_bip32_key(self.config.network, &self.signer, path)
    }
}
