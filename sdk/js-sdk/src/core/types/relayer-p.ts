import type {
  Bytes32Hex,
  Bytes65Hex,
  Bytes65HexNo0x,
  BytesHex,
  BytesHexNo0x,
  ChecksummedAddress,
} from "./primitives.js";
import type { NonEmptyExtract } from "./utils.js";
import type {
  FetchInputProofResult,
  FetchPublicDecryptResult,
  FetchUserDecryptResult,
} from "./relayer.js";
import type { FhevmHandleBytes32Hex } from "./fhevmHandle.js";

////////////////////////////////////////////////////////////////////////////////
// RelayerOperation
////////////////////////////////////////////////////////////////////////////////

export interface FetchPostOperationResultMap {
  INPUT_PROOF: FetchInputProofResult;
  PUBLIC_DECRYPT: FetchPublicDecryptResult;
  USER_DECRYPT: FetchUserDecryptResult;
  DELEGATED_USER_DECRYPT: FetchUserDecryptResult;
}

export type RelayerGetOperation = "KEY_URL";
export type RelayerPostOperation = keyof FetchPostOperationResultMap;
export type RelayerOperation = RelayerPostOperation | RelayerGetOperation;

////////////////////////////////////////////////////////////////////////////////
// Fetch Payload
////////////////////////////////////////////////////////////////////////////////

// https://github.com/zama-ai/fhevm-relayer/blob/96151ef300f787658c5fbaf1b4471263160032d5/src/http/input_http_listener.rs#L17
export type FetchInputProofPayload = {
  // Hex encoded uint256 string without prefix
  readonly contractChainId: `0x${string}`;
  // Hex encoded address with 0x prefix.
  readonly contractAddress: ChecksummedAddress;
  // Hex encoded address with 0x prefix.
  readonly userAddress: ChecksummedAddress;
  // List of hex encoded binary proof without 0x prefix
  readonly ciphertextWithInputVerification: BytesHexNo0x;
  // Hex encoded bytes with 0x prefix. Default: 0x00
  readonly extraData: BytesHex;
};

// https://github.com/zama-ai/fhevm-relayer/blob/96151ef300f787658c5fbaf1b4471263160032d5/src/http/public_decrypt_http_listener.rs#L19
export type RelayerFetchPublicDecryptPayload = {
  readonly ciphertextHandles: readonly BytesHex[];
  // Hex encoded bytes with 0x prefix. Default: 0x00
  readonly extraData: BytesHex;
};

// https://github.com/zama-ai/fhevm-relayer/blob/96151ef300f787658c5fbaf1b4471263160032d5/src/http/userdecrypt_http_listener.rs#L33
export type HandleContractPair = {
  // Hex encoded bytes32 with 0x prefix.
  readonly handle: FhevmHandleBytes32Hex;
  // Hex encoded address with 0x prefix.
  readonly contractAddress: ChecksummedAddress;
};

// https://github.com/zama-ai/fhevm-relayer/blob/96151ef300f787658c5fbaf1b4471263160032d5/src/http/userdecrypt_http_listener.rs#L20
export type FetchUserDecryptPayload = {
  readonly handleContractPairs: readonly HandleContractPair[];
  readonly requestValidity: {
    // Number as a string
    readonly startTimestamp: string;
    // Number as a string
    readonly durationDays: string;
  };
  // Number as a string
  readonly contractsChainId: string;
  // List of hex encoded addresses with 0x prefix
  readonly contractAddresses: readonly ChecksummedAddress[];
  // Hex encoded address with 0x prefix.
  readonly userAddress: ChecksummedAddress;
  // Hex encoded signature without 0x prefix.
  readonly signature: Bytes65HexNo0x;
  // Hex encoded key without 0x prefix.
  readonly publicKey: BytesHexNo0x;
  // Hex encoded bytes with 0x prefix. Default: 0x00
  readonly extraData: BytesHex;
};

export type FetchDelegatedUserDecryptPayload = {
  readonly handleContractPairs: readonly HandleContractPair[];
  // Hex encoded uint256 string without prefix
  readonly contractsChainId: string;
  // List of hex encoded addresses with 0x prefix
  readonly contractAddresses: readonly ChecksummedAddress[];
  // The address grating permission. Hex encoded address with 0x prefix.
  readonly delegatorAddress: ChecksummedAddress;
  // The address receiving permission. Hex encoded address with 0x prefix.
  readonly delegateAddress: ChecksummedAddress;
  // Number as a string
  readonly startTimestamp: string;
  // Number as a string
  readonly durationDays: string;
  // Hex encoded signature without 0x prefix.
  readonly signature: Bytes65HexNo0x;
  // Hex encoded key without 0x prefix.
  readonly publicKey: BytesHexNo0x;
  // Hex encoded bytes with 0x prefix. Default: 0x00
  readonly extraData: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////
// Relayer Async Request
////////////////////////////////////////////////////////////////////////////////

export type RelayerAsyncRequestState = {
  aborted: boolean;
  canceled: boolean;
  failed: boolean;
  fetching: boolean;
  running: boolean;
  succeeded: boolean;
  terminated: boolean;
  timeout: boolean;
};

export type RelayerTerminateReason =
  | "succeeded"
  | "failed"
  | "timeout"
  | "abort";

////////////////////////////////////////////////////////////////////////////////
// Relayer Status Codes
////////////////////////////////////////////////////////////////////////////////

export type RelayerFetchMethod = "GET" | "POST";
export type RelayerSuccessStatus = 200 | 202;
export type RelayerFailureStatus = 400 | 401 | 404 | 429 | 500 | 503;

export type RelayerPostResponseStatus =
  | NonEmptyExtract<RelayerSuccessStatus, 202>
  | NonEmptyExtract<RelayerFailureStatus, 400 | 401 | 429 | 500 | 503>;

// export type RelayerPostResponse =
//   | RelayerResponseFailed
//   | RelayerPostResponse202Queued;

// GET:  200 | 202 | 400 | 401 | 404 | 500 | 503
export type RelayerGetResponseStatus =
  | NonEmptyExtract<RelayerSuccessStatus, 200 | 202>
  | NonEmptyExtract<RelayerFailureStatus, 400 | 401 | 404 | 500 | 503>;

////////////////////////////////////////////////////////////////////////////////
// Succeeded: 200
////////////////////////////////////////////////////////////////////////////////

export interface RelayerGetResponse200Map {
  INPUT_PROOF:
    | RelayerResult200InputProofAccepted
    | RelayerResult200InputProofRejected;
  PUBLIC_DECRYPT: RelayerResult200PublicDecrypt;
  USER_DECRYPT: RelayerResult200UserDecrypt;
}

export type RelayerGetResponse200<A extends keyof RelayerGetResponse200Map> = {
  status: "succeeded";
  requestId: string; // request id field. use it for identifying the request and asking support
  result: RelayerGetResponse200Map[A];
};

export type RelayerResult200InputProofAccepted = {
  accepted: true;
  extraData: BytesHex;
  // Ordered List of hex encoded handles with 0x prefix.
  handles: Bytes32Hex[];
  // Attestation signatures for Input verification for the ordered list of handles with 0x prefix.
  signatures: Bytes65Hex[];
};

export type RelayerResult200InputProofRejected = {
  accepted: false;
  extraData: BytesHex;
};

export type RelayerResult200PublicDecrypt = {
  signatures: BytesHexNo0x[];
  decryptedValue: BytesHexNo0x;
  extraData: BytesHex;
};

export type RelayerResult200UserDecrypt = {
  result: Array<{
    payload: BytesHexNo0x;
    signature: Bytes65HexNo0x;
    //extraData: BytesHex or BytesHexNo0x ?;
  }>;
};

/**
 * Relayer 200 response for input proof requests:
 * ```json
 * {
 *   "status": "succeeded",
 *   "requestId": "string",
 *   "result": {
 *     "accepted": true,
 *     "extraData": "0x...",
 *     "handles": ["0x..."],
 *     "signatures": ["0x..."]
 *   } | {
 *     "accepted": false,
 *     "extraData": "0x..."
 *   }
 * }
 * ```
 */
export type RelayerInputProofSucceeded = RelayerGetResponse200<"INPUT_PROOF">;

/**
 * Relayer 200 response for public decrypt requests:
 * ```json
 * {
 *   "status": "succeeded",
 *   "requestId": "string",
 *   "result": {
 *     "signatures": ["hexNo0x..."],
 *     "decryptedValue": "hexNo0x...",
 *     "extraData": "0x..."
 *   }
 * }
 * ```
 */
export type RelayerPublicDecryptSucceeded =
  RelayerGetResponse200<"PUBLIC_DECRYPT">;

/**
 * Relayer 200 response for user decrypt requests:
 * ```json
 * {
 *   "status": "succeeded",
 *   "requestId": "string",
 *   "result": {
 *     "result": [{
 *       "payload": "hexNo0x...",
 *       "signature": "hexNo0x...",
 *       "extraData": "hex_or_hexNo0x_?..."
 *     }]
 *   }
 * }
 * ```
 */
export type RelayerUserDecryptSucceeded = RelayerGetResponse200<"USER_DECRYPT">;

////////////////////////////////////////////////////////////////////////////////
// Queued: 202
////////////////////////////////////////////////////////////////////////////////

/**
 * Relayer 202 get response schema:
 * ```json
 * {
 *   "result": {
 *     "status": "queued",
 *     "requestId": "string",
 *   }
 * }
 * ```
 */
export type RelayerGetResponse202Queued = {
  status: "queued";
  requestId: string; // request id field. use it for identifying the request and asking support
};

/**
 * Relayer 202 post response schema:
 * ```json
 * {
 *   "result": {
 *     "status": "queued",
 *     "requestId": "string",
 *     "result": {
 *        jobId: "string",
 *      }
 *   }
 * }
 * ```
 */
export type RelayerPostResponse202Queued = {
  status: "queued";
  requestId: string; // request id field. use it for identifying the request and asking support
  result: RelayerResult202Queued;
};

export type RelayerResult202Queued = {
  jobId: string;
};

////////////////////////////////////////////////////////////////////////////////
// Relayer Response Failed
////////////////////////////////////////////////////////////////////////////////

/**
 * Optional request id field. Would be empty in case of 429 from Cloudflare/Kong.
 * In other cases, use it for identifying the request and asking support
 */
export type RelayerResponseFailed = {
  status: "failed";
  requestId?: string;
  error: RelayerApiError;
};

////////////////////////////////////////////////////////////////////////////////
// Relayer API Errors (400 | 401 | 404 | 429 | 500 | 503)
////////////////////////////////////////////////////////////////////////////////

export type RelayerApiError =
  | RelayerApiError400
  | RelayerApiError401
  | RelayerApiError404
  | RelayerApiError429
  | RelayerApiError500
  | RelayerApiError503;

/**
 * Status: 400
 */
export type RelayerApiError400 =
  | RelayerApiError400NoDetails
  | RelayerApiError400WithDetails;

/**
 * Status: 400 (no details)
 */
export type RelayerApiError400NoDetails = {
  label: "malformed_json" | "request_error" | "not_ready_for_decryption";
  message: string;
};

/**
 * Status: 400 (with details)
 */
export type RelayerApiError400WithDetails = {
  label: "missing_fields" | "validation_failed";
  message: string;
  details: RelayerErrorDetail[];
};

export type RelayerErrorDetail = {
  field: string;
  issue: string;
};

/**
 * Status: 401
 */
export type RelayerApiError401 = {
  label: "unauthorized";
  message: string;
};

/**
 * Status: 404
 */
export type RelayerApiError404 = {
  label: "not_found";
  message: string;
};

/**
 * Status: 429
 */
export type RelayerApiError429 = {
  label: "rate_limited" | "protocol_overload";
  message: string;
};

/**
 * Status: 500
 */
export type RelayerApiError500 = {
  label: "internal_server_error";
  message: string;
};

/**
 * Status: 503
 */
export type RelayerApiError503 = {
  label:
    | "protocol_paused"
    | "gateway_not_reachable"
    | "readiness_check_timed_out"
    | "response_timed_out";
  message: string;
};

export type FetchKeyUrlResult = {
  readonly fheKeyInfo: [
    {
      readonly fhePublicKey: {
        readonly dataId: string;
        readonly urls: readonly [string];
      };
    },
  ];
  readonly crs: {
    readonly 2048: {
      readonly dataId: string;
      readonly urls: readonly [string];
    };
  };
};

/**
 * Parameters for fetching TFHE resources with retry support.
 */
export type TfheFetchParams = {
  /** Optional fetch init options (headers, signal, etc.) */
  init?: RequestInit | undefined;
  /** Number of retry attempts on network failure (default: 3) */
  retries?: number | undefined;
  /** Delay in milliseconds between retries (default: 1000) */
  retryDelayMs?: number | undefined;
};
