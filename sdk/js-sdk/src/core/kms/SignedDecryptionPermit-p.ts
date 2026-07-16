import type { SignedDecryptionPermit } from '../types/signedDecryptionPermit.js';
import type { KmsDelegatedUserDecryptEip712V1, KmsUserDecryptEip712V1, KmsUserDecryptEip712V2 } from '../types/kms.js';
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
 * Serializes a signed decryption permit to a plain object suitable for
 * JSON serialization. Uses the public getters — does not access private fields.
 *
 * `toJSON()` is intentionally not on the class to prevent accidental
 * serialization of sensitive data via `JSON.stringify(permit)`.
 */
export function serializeSignedDecryptionPermitToJSON(permit: SignedDecryptionPermit):
  | {
      version: 1;
      eip712: KmsUserDecryptEip712V1 | KmsDelegatedUserDecryptEip712V1;
      signature: string;
      signerAddress: string;
    }
  | {
      version: 2;
      eip712: KmsUserDecryptEip712V2;
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
      eip712: permit.eip712,
      signature: permit.signature,
      signerAddress: permit.signerAddress,
    };
  }

  return {
    version: 2,
    eip712: permit.eip712,
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

export async function parseSignedDecryptionPermit(
  context: SignDecryptionPermitContext,
  transportKeyPair: TransportKeyPair,
  permit: unknown,
): Promise<SignedDecryptionPermit> {
  const hasVersion = isRecordUintNumberProperty(permit, 'version');

  // if no version, interpret as permit v1
  const version: number = hasVersion ? permit.version : 1;

  if (version === 1) {
    return await parseSignedDecryptionPermitV1(context, transportKeyPair, permit);
  }

  if (version === 2) {
    return await parseSignedDecryptionPermitV2(context, transportKeyPair, permit);
  }

  throw new Error(`Unsupported permit version: ${version}. Supported versions are 1 and 2.`);
}
