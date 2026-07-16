import type { SignedDecryptionPermit } from '../types/signedDecryptionPermit.js';
import type { Eip712Like } from '../types/kms.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { NativeSigner } from '../modules/ethereum/types.js';
import type { TransportKeyPair } from './TransportKeyPair-p.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import { getResolvedProtocolVersion } from '../runtime/CoreFhevm-p.js';
import { shouldUseUserDecryptV2 } from '../runtime/userDecryptFlowVersion-p.js';
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
import { isRecordUintNumberProperty } from '../base/uint.js';

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

export type SignDecryptionPermitContext = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

export type SignDecryptionPermitParameters = {
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationSeconds: number;
  readonly signerAddress: string;
  readonly signer: NativeSigner;
  readonly delegatorAddress?: string | undefined;
  readonly transportKeyPair: TransportKeyPair;
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
  context: SignDecryptionPermitContext,
  parameters: SignDecryptionPermitParameters,
): Promise<SignedDecryptionPermit> {
  const protocolVersion = getResolvedProtocolVersion(context);
  if (protocolVersion === undefined) {
    throw new Error(
      'Unable to resolve protocol version from context, ensure proper initialization of the FhevmRuntime and FhevmChain.',
    );
  }

  if (!shouldUseUserDecryptV2(protocolVersion)) {
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

export async function parseSignedDecryptionPermit(
  context: SignDecryptionPermitContext,
  transportKeyPair: TransportKeyPair,
  permit: unknown,
): Promise<SignedDecryptionPermit> {
  // Accept permits revived from a JSON string (chainId serialized as a string).
  const sanitizedPermit = _normalizeSerializedPermitDomainChainId(permit);

  const hasVersion = isRecordUintNumberProperty(sanitizedPermit, 'version');

  // if no version, interpret as permit v1
  const version: number = hasVersion ? sanitizedPermit.version : 1;

  if (version === 1) {
    return await parseSignedDecryptionPermitV1(context, transportKeyPair, sanitizedPermit);
  }

  if (version === 2) {
    return await parseSignedDecryptionPermitV2(context, transportKeyPair, sanitizedPermit);
  }

  throw new Error(`Unsupported permit version: ${version}. Supported versions are 1 and 2.`);
}
