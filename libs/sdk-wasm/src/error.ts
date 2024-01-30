export class GenericError extends Error {
  constructor(...params: any[]) {
    super(...params);
    this.name = "Error";
  }
}
