import { ErrorBase } from "./ErrorBase.js";

export type FetchErrorType = FetchError & {
  name: "FetchError";
};

export type FetchErrorParams = Readonly<{
  url: string;
  message?: string;
}>;

////////////////////////////////////////////////////////////////////////////////

export class FetchError extends ErrorBase {
  readonly #url: string;
  constructor(params: FetchErrorParams) {
    super({
      ...params,
      name: "FetchError",
      message: params.message ?? "fetch error",
    });
    this.#url = params.url;
  }
  public get url(): string {
    return this.#url;
  }
}
