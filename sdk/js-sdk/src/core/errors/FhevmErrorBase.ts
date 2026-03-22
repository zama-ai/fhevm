import { version, sdkName } from "../_version.js";

import type { Prettify } from "../types/utils.js";
import type { ErrorBaseParams } from "../base/errors/ErrorBase.js";
import { ErrorBase } from "../base/errors/ErrorBase.js";

export type FhevmErrorBaseType = FhevmErrorBase & {
  name: "FhevmErrorBase";
};

export type FhevmErrorBaseParams = Prettify<
  Omit<ErrorBaseParams, "docsUrl" | "name" | "version"> & {
    readonly docsPath?: string;
    readonly docsSlug?: string;
    readonly name: string;
  }
>;

export abstract class FhevmErrorBase extends ErrorBase {
  private static readonly PKG_NAME = sdkName;
  private static readonly VERSION = version;
  private static readonly DEFAULT_DOCS_BASE_URL =
    "https//docs.zama.org" as const;
  private static readonly FULL_VERSION =
    `${FhevmErrorBase.PKG_NAME}@${FhevmErrorBase.VERSION}` as const;

  readonly #docsPath: string | undefined;

  constructor(params: FhevmErrorBaseParams) {
    let docsPath: string | undefined;
    if (params.cause instanceof FhevmErrorBase) {
      docsPath = params.docsPath ?? params.cause.docsPath;
    } else {
      docsPath = params.docsPath;
    }

    let docsUrl: string | undefined = undefined;
    if (docsPath !== undefined) {
      if (!docsPath.startsWith("/")) {
        docsPath = "/" + docsPath;
      }
      docsUrl = `${FhevmErrorBase.DEFAULT_DOCS_BASE_URL}${docsPath}${params.docsSlug !== undefined ? `#${params.docsSlug}` : ""}`;
    }

    super({
      ...params,
      name: "FhevmErrorBase",
      version: FhevmErrorBase.FULL_VERSION,
      docsUrl,
    });
  }

  public get docsPath(): string | undefined {
    return this.#docsPath;
  }
}
