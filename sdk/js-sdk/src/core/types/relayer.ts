import type { Bytes65Hex, BytesHex, BytesHexNo0x } from "./primitives.js";
import type { Prettify } from "./utils.js";
import type { FhevmHandle } from "./fhevmHandle.js";
import type { KmsSigncryptedShare } from "./kms-p.js";
import type { Auth } from "./auth.js";

export type RelayerSuccessStatus = 200 | 202;
export type RelayerFailureStatus = 400 | 401 | 404 | 429 | 500 | 503;

export type FetchInputProofResult = {
  // Ordered List of hex encoded handles with 0x prefix.
  readonly handles: readonly FhevmHandle[];
  // Attestation signatures for Input verification for the ordered list of handles with 0x prefix.
  readonly signatures: readonly Bytes65Hex[];
  readonly extraData: BytesHex;
};

export type FetchPublicDecryptResult = {
  readonly signatures: readonly BytesHexNo0x[];
  readonly decryptedValue: BytesHexNo0x;
  readonly extraData: BytesHex;
};

export type FetchUserDecryptResult = readonly KmsSigncryptedShare[];
export type FetchDelegatedUserDecryptResult = readonly KmsSigncryptedShare[];

////////////////////////////////////////////////////////////////////////////////
// Options
////////////////////////////////////////////////////////////////////////////////

export type RelayerCommonOptions = {
  auth?: Auth | undefined;
  debug?: boolean | undefined;
  fetchRetries?: number | undefined;
  fetchRetryDelayInMilliseconds?: number | undefined;
  signal?: AbortSignal;
  timeout?: number;
};

export type RelayerKeyUrlOptions = Prettify<
  Omit<RelayerCommonOptions, "timeout"> & {
    onProgress?: (args: RelayerKeyUrlProgressArgs) => void;
  }
>;

export type RelayerInputProofOptions = Prettify<
  RelayerCommonOptions & {
    onProgress?: (args: RelayerInputProofProgressArgs) => void;
  }
>;

export type RelayerUserDecryptOptions = Prettify<
  RelayerCommonOptions & {
    onProgress?: (args: RelayerUserDecryptProgressArgs) => void;
  }
>;

export type RelayerDelegatedUserDecryptOptions = Prettify<
  RelayerCommonOptions & {
    onProgress?: (args: RelayerDelegatedUserDecryptProgressArgs) => void;
  }
>;

export type RelayerPublicDecryptOptions = Prettify<
  RelayerCommonOptions & {
    onProgress?: (args: RelayerPublicDecryptProgressArgs) => void;
  }
>;

////////////////////////////////////////////////////////////////////////////////
// Progress
////////////////////////////////////////////////////////////////////////////////

export type RelayerProgressTypeValue =
  | "abort"
  | "queued"
  | "failed"
  | "timeout"
  | "succeeded"
  | "throttled";

export type RelayerPostOperation =
  | "INPUT_PROOF"
  | "PUBLIC_DECRYPT"
  | "USER_DECRYPT"
  | "DELEGATED_USER_DECRYPT";

export type FetchResultOf<O extends RelayerPostOperation> =
  O extends "INPUT_PROOF"
    ? FetchInputProofResult
    : O extends "PUBLIC_DECRYPT"
      ? FetchPublicDecryptResult
      : FetchUserDecryptResult;

export type RelayerProgressArgs<O extends RelayerPostOperation> =
  | RelayerProgressQueued<O>
  | RelayerProgressThrottled<O>
  | RelayerProgressSucceeded<O>
  | RelayerProgressTimeout<O>
  | RelayerProgressAbort<O>
  | RelayerProgressFailed<O>;

export type RelayerKeyUrlProgressArgs = {
  readonly url: string;
  readonly operation: "KEY_URL";
  readonly retryCount: number;
  readonly method: "GET";
};
export type RelayerInputProofProgressArgs = RelayerProgressArgs<"INPUT_PROOF">;
export type RelayerUserDecryptProgressArgs =
  RelayerProgressArgs<"USER_DECRYPT">;
export type RelayerDelegatedUserDecryptProgressArgs =
  RelayerProgressArgs<"DELEGATED_USER_DECRYPT">;
export type RelayerPublicDecryptProgressArgs =
  RelayerProgressArgs<"PUBLIC_DECRYPT">;

export type RelayerProgressBase<
  T extends RelayerProgressTypeValue,
  O extends RelayerPostOperation,
> = {
  readonly type: T;
  readonly url: string;
  readonly method?: "POST" | "GET";
  readonly operation: O;
  readonly jobId?: string | undefined;
  readonly retryCount: number;
  readonly totalSteps: number;
  readonly step: number;
};

export type RelayerProgressStatusBase<
  T extends RelayerProgressTypeValue,
  O extends RelayerPostOperation,
  S extends RelayerSuccessStatus | RelayerFailureStatus,
> = Prettify<
  RelayerProgressBase<T, O> & {
    readonly method: "POST" | "GET";
    readonly status: S;
  }
>;

export type RelayerProgressJobIdBase<
  T extends RelayerProgressTypeValue,
  O extends RelayerPostOperation,
  S extends RelayerSuccessStatus | RelayerFailureStatus,
> = Prettify<
  RelayerProgressStatusBase<T, O, S> & {
    readonly jobId: string;
  }
>;

// 202 is GET or POST
export type RelayerProgressQueued<O extends RelayerPostOperation> = Prettify<
  RelayerProgressJobIdBase<"queued", O, 202> & {
    readonly requestId: string;
    readonly retryAfterMs: number;
    readonly elapsed: number;
  }
>;

export type RelayerProgressThrottled<O extends RelayerPostOperation> = Prettify<
  RelayerProgressStatusBase<"throttled", O, 429> & {
    readonly method: "POST";
    readonly retryAfterMs: number;
    readonly elapsed: number;
    readonly relayerApiError: {
      readonly label: string;
      readonly message: string;
    };
  }
>;

export type RelayerProgressSucceeded<O extends RelayerPostOperation> = Prettify<
  RelayerProgressJobIdBase<"succeeded", O, 200> & {
    readonly requestId: string;
    readonly elapsed: number;
    readonly result: FetchResultOf<O>;
  }
>;

export type RelayerProgressFailed<
  O extends RelayerPostOperation,
  S extends RelayerFailureStatus = RelayerFailureStatus,
> = Prettify<
  RelayerProgressStatusBase<"failed", O, S> & {
    readonly elapsed: number;
    readonly relayerApiError: {
      readonly label: string;
      readonly message: string;
    };
  }
>;

export type RelayerProgressTimeout<O extends RelayerPostOperation> = Prettify<
  RelayerProgressBase<"timeout", O>
>;

export type RelayerProgressAbort<O extends RelayerPostOperation> = Prettify<
  RelayerProgressBase<"abort", O>
>;
