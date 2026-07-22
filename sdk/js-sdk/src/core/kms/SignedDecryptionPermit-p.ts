import type { SignedDecryptionPermit } from '../types/signedDecryptionPermit.js';
import type { Eip712Like } from '../types/kms.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { NativeSigner } from '../modules/ethereum/types.js';
import type { TransportKeyPair } from './TransportKeyPair-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { BytesHex } from '../types/primitives.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import {
  isSignedDecryptionPermitV1,
  parseSignedDecryptionPermitV1,
  signDecryptionPermitV1,
} from './SignedDecryptionPermitV1-p.js';
import {
  isSignedDecryptionPermitV2,
  parseSignedDecryptionPermitV2,
  signDecryptionPermitV2,
} from './SignedDecryptionPermitV2-p.js';
import { isRecordUintNumberProperty, isUintNumber } from '../base/uint.js';
import { SDK_PROTOCOL_API_MAJOR_VERSION, SDK_PROTOCOL_API_MINOR_VERSION } from '../runtime/sdkProtocolApiVersion.js';
import { createKmsExtraDataFromBytesHex, EXTRA_DATA_V2 } from './kmsExtraData-p.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Returns a JSON-safe copy of the permit's EIP-712 typed data: the domain's
 * `chainId` (a bigint internally) is emitted as a decimal string so the result
 * survives `JSON.stringify()`. Everything else in the typed data is already
 * made of JSON-safe primitives. `parseSignedDecryptionPermit` converts the
 * string back to a bigint (see `_normalizeSerializedPermitDomainChainId`).
 */
function _toJsonSafeEip712(eip712: {
  readonly domain: Record<string, unknown>;
  readonly types: Eip712Like['types'];
  readonly primaryType?: string | undefined;
  readonly message: Record<string, unknown>;
}): Eip712Like {
  const chainId = eip712.domain.chainId;
  const domain = {
    ...eip712.domain,
    chainId: typeof chainId === 'bigint' ? chainId.toString() : chainId,
  };
  const jsonSafe = { ...eip712, domain };
  Object.freeze(jsonSafe);
  Object.freeze(domain);
  return jsonSafe;
}

/**
 * Serializes a signed decryption permit to a plain object suitable for
 * JSON serialization (all values are JSON-safe primitives — the domain's
 * bigint `chainId` is emitted as a decimal string). The result can be passed
 * through `JSON.stringify()`/`JSON.parse()` (e.g. localStorage) and restored
 * with `parseSignedDecryptionPermit`. Uses the public getters — does not
 * access private fields.
 *
 * `toJSON()` is intentionally not on the class to prevent accidental
 * serialization of sensitive data via `JSON.stringify(permit)`.
 */
export function serializeSignedDecryptionPermitToJSON(permit: SignedDecryptionPermit):
  | {
      version: 1;
      eip712: Eip712Like;
      signature: string;
      signerAddress: string;
    }
  | {
      version: 2;
      eip712: Eip712Like;
      signature: string;
      signerAddress: string;
    } {
  assertIsSignedDecryptionPermit(permit, {});

  // Defensive check
  const version = permit.version as unknown as number;
  if (version !== 1 && version !== 2) {
    throw new Error(`Unsupported permit version: ${version}. Supported versions are 1 and 2.`);
  }

  // This if branch is needed for tsc
  if (permit.version === 1) {
    return {
      version: 1,
      eip712: _toJsonSafeEip712(permit.eip712),
      signature: permit.signature,
      signerAddress: permit.signerAddress,
    };
  }

  return {
    version: 2,
    eip712: _toJsonSafeEip712(permit.eip712),
    signature: permit.signature,
    signerAddress: permit.signerAddress,
  };
}

////////////////////////////////////////////////////////////////////////////////
// isSignedDecryptionPermit
////////////////////////////////////////////////////////////////////////////////

export function isSignedDecryptionPermit(value: unknown): value is SignedDecryptionPermit {
  return isSignedDecryptionPermitV1(value) || isSignedDecryptionPermitV2(value);
}

/** Throws {@link InvalidTypeError} if value is not a valid {@link SignedDecryptionPermit}. */
export function assertIsSignedDecryptionPermit(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is SignedDecryptionPermit {
  if (!isSignedDecryptionPermit(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'SignedDecryptionPermit',
      },
      options,
    );
  }
}

export type KmsSignDecryptionPermitContext = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

export type KmsSignDecryptionPermitParameters = {
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationSeconds: number;
  readonly signerAddress: string;
  readonly signer: NativeSigner;
  readonly delegatorAddress?: string | undefined;
  readonly transportKeyPair: TransportKeyPair;
  readonly fhevmContext: FhevmClientFrozenContext;
};

/**
 * Creates a signed decryption permit by constructing the EIP-712 typed data
 * and signing it with the provided signer.
 *
 * If `delegatorAddress` is provided, creates a delegated permit that allows the signer
 * to decrypt encrypted values belonging to the `delegatorAddress` account.
 * Otherwise, creates a standard permit where the signer decrypts their own values.
 *
 * The EIP-712 message includes the key pair's public key, allowing the gateway
 * to encrypt the decrypted result for this specific key pair.
 *
 * @throws If the signer, address, or key pair is invalid.
 * @throws If the signature verification fails.
 */
export async function signDecryptionPermit(
  context: KmsSignDecryptionPermitContext,
  parameters: KmsSignDecryptionPermitParameters,
): Promise<SignedDecryptionPermit> {
  // V1 permits are always created here: the current protocol API version this
  // SDK is using is 0.13.0, so the SDK does not know the 0.14.0 API (which is
  // what introduces V2 permits). Once the SDK adopts API 0.14.0, this branch
  // falls through to signDecryptionPermitV2 below.
  if (SDK_PROTOCOL_API_MAJOR_VERSION === 0 && SDK_PROTOCOL_API_MINOR_VERSION <= 13) {
    return await signDecryptionPermitV1(context, parameters);
  }

  return await signDecryptionPermitV2(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////
// parseSignedDecryptionPermit
////////////////////////////////////////////////////////////////////////////////

/**
 * Restores the bigint `eip712.domain.chainId` on a serialized permit.
 *
 * `serializeSignedDecryptionPermitToJSON` emits `chainId` as a decimal string
 * so the permit survives `JSON.stringify()`/`JSON.parse()` (e.g. localStorage).
 * Internally the domain is validated and stored with a bigint `chainId`, so a
 * string (or number) is converted back here before validation. Returns a new
 * object — the caller's input is never mutated. Anything that does not look
 * like a permit-with-domain-chainId is returned as-is and left to the
 * downstream validation to report.
 */
function _normalizeSerializedPermitDomainChainId(permit: unknown): unknown {
  if (permit === null || typeof permit !== 'object') {
    return permit;
  }
  const eip712 = (permit as Record<string, unknown>).eip712;
  if (eip712 === null || typeof eip712 !== 'object') {
    return permit;
  }
  const domain = (eip712 as Record<string, unknown>).domain;
  if (domain === null || typeof domain !== 'object') {
    return permit;
  }
  const chainId = (domain as Record<string, unknown>).chainId;
  if (typeof chainId !== 'string' && typeof chainId !== 'number') {
    return permit;
  }

  let chainIdBigInt: bigint;
  try {
    chainIdBigInt = BigInt(chainId);
  } catch {
    // Not a valid uint string/number — leave the permit untouched so the
    // domain validation reports the malformed chainId with proper context.
    return permit;
  }

  return {
    ...(permit as Record<string, unknown>),
    eip712: {
      ...(eip712 as Record<string, unknown>),
      domain: {
        ...(domain as Record<string, unknown>),
        chainId: chainIdBigInt,
      },
    },
  };
}

/**
 * Reads the `extraData` version carried by a serialized permit, or `undefined` when
 * the permit exposes no decodable `extraData` (missing, wrong type, or malformed —
 * left to the downstream permit validation to report with proper context).
 *
 * Navigates `permit.eip712.message.extraData` defensively (the permit is untrusted
 * input). Used to enforce the SDK protocol-API cap at parse time — see the CRITICAL
 * RULE in `readKmsSignersContext-p.ts`.
 */
function _parsePermitExtraDataVersion(permit: unknown): number | undefined {
  const extraData = (
    permit as { readonly eip712?: { readonly message?: { readonly extraData?: unknown } } } | null | undefined
  )?.eip712?.message?.extraData;
  if (typeof extraData !== 'string') {
    return undefined;
  }
  try {
    return createKmsExtraDataFromBytesHex(extraData as BytesHex).version;
  } catch {
    return undefined;
  }
}

export async function parseSignedDecryptionPermit(
  context: KmsSignDecryptionPermitContext,
  parameters: {
    readonly transportKeyPair: TransportKeyPair;
    readonly permit: unknown;
    readonly fhevmContext: FhevmClientFrozenContext;
  },
): Promise<SignedDecryptionPermit> {
  const { transportKeyPair, permit, fhevmContext } = parameters;
  // Accept permits revived from a JSON string (chainId serialized as a string).
  const sanitizedPermit = _normalizeSerializedPermitDomainChainId(permit);

  const hasVersion = isRecordUintNumberProperty(sanitizedPermit, 'version');

  // if no version, interpret as permit v1
  const sanitizedVersion: number = hasVersion ? sanitizedPermit.version : 1;

  // check valid version number
  if (!isUintNumber(sanitizedVersion) || sanitizedVersion > 2 || sanitizedVersion === 0) {
    throw new Error(`Unsupported permit version: ${sanitizedVersion}. Supported versions are 1 and 2.`);
  }

  // version is 1 or 2
  const version: 1 | 2 = sanitizedVersion as 1 | 2;

  // Enforce the SDK protocol-API cap (see the CRITICAL RULE in readKmsSignersContext-p.ts):
  // a v13-capped SDK must never accept a v2 permit, because a v13 relayer rejects it.
  // A v2 can arrive two independent ways; each is rejected here with its own message,
  // up front, instead of an opaque relayer 400 later (mirrors decryptValuesFromPairs).
  if (SDK_PROTOCOL_API_MAJOR_VERSION === 0 && SDK_PROTOCOL_API_MINOR_VERSION <= 13) {
    // (a) A V2-format permit.
    if (version > 1) {
      throw new Error(
        `Refusing to parse a V2-format decryption permit (version ${version}): this SDK is capped at protocol API v0.${SDK_PROTOCOL_API_MINOR_VERSION} and only supports V1 permits. V2 permits require an SDK on protocol API v0.14.0 or later.`,
      );
    }
    const extraDataVersion = _parsePermitExtraDataVersion(sanitizedPermit);
    // (b) A v2 extraData — including one carried inside a V1-format permit.
    if (extraDataVersion !== undefined && extraDataVersion >= EXTRA_DATA_V2) {
      throw new Error(
        `Refusing to parse a permit carrying extraData v2: this SDK is capped at protocol API v0.${SDK_PROTOCOL_API_MINOR_VERSION} and only accepts extraData v0/v1 (a v13 relayer rejects extraData v2). Use an SDK on protocol API v0.14.0 or later.`,
      );
    }
  }

  if (version === 1) {
    return await parseSignedDecryptionPermitV1(context, { transportKeyPair, permit: sanitizedPermit, fhevmContext });
  }

  return await parseSignedDecryptionPermitV2(context, { transportKeyPair, permit: sanitizedPermit, fhevmContext });
}
