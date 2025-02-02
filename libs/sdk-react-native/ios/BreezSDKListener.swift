import Foundation
import BreezSDK

class BreezSDKListener: EventListener {
    static let emitterName: String = "breezSdkEvent"
    
    func onEvent(e: BreezEvent) {
        RNBreezSDK.emitter.sendEvent(withName: BreezSDKListener.emitterName,
                                     body: BreezSDKMapper.dictionaryOf(breezEvent: e))
    }
}
