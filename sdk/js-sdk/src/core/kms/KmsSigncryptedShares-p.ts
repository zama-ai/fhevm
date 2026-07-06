import type { KmsSigncryptedSharesMetadata, KmsSigncryptedShare } from '../types/kms-p.js';
import type { KmsSigncryptedShares, KmsSigncryptedSharesBrand } from '../types/kms.js';
import type { BytesHex, BytesHexNo0x, ChecksummedAddress } from '../types/primitives.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { TkmsVersion } from '../../wasm/tkms/KmsLibApi.js';
import { assertIsKmsSignersContext } from '../host-contracts/KmsSignersContext-p.js';
import { ensure0x } from '../base/string.js';
import { reconcileKmsSignersContext } from '../host-contracts/readKmsSignersContext-p.js';
import { recoverSigners } from '../utils-p/runtime/recoverSigners.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * EIP-712 types for the KMS `UserDecryptResponseVerification` structure that each
 * KMS node signs per response share (see gateway `Decryption.sol`).
 *
 * CRITICAL: field order and types are authoritative — they define the EIP-712
 * type hash and must match the on-chain struct exactly.
 *
 * Note: `ctHandles` is declared `bytes32[]` to match `Decryption.sol` (the tkms
 * WASM doc comments it as `uint256[]`; both encode the same 32 bytes, but the
 * type string participates in the type hash — if verification unexpectedly
 * fails, `uint256[]` is the first thing to try).
 */
const kmsUserDecryptResponseEip712Types = {
  EIP712Domain: [
    { name: 'name', type: 'string' },
    { name: 'version', type: 'string' },
    { name: 'chainId', type: 'uint256' },
    { name: 'verifyingContract', type: 'address' },
  ],
  UserDecryptResponseVerification: [
    { name: 'publicKey', type: 'bytes' },
    { name: 'ctHandles', type: 'bytes32[]' },
    { name: 'userDecryptedShare', type: 'bytes' },
    { name: 'extraData', type: 'bytes' },
  ],
} as const;

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_KMS_SIGNCRYPTED_SHARES_TOKEN = Symbol('KmsSigncryptedShares.token');

const GET_METADATA_FUNC = Symbol('KmsSigncryptedShares.getMetadata');
const GET_SHARES_FUNC = Symbol('KmsSigncryptedShares.getShares');

////////////////////////////////////////////////////////////////////////////////

/**
 * Validated collection of KMS signcrypted shares.
 *
 * Construction guarantees:
 * - Contains at least one share.
 * - All shares have identical `extraData` values.
 * - The shared `extraData` matches the `extraData` derived from the associated {@link KmsSignersContext}.
 *
 * @internal
 */
class KmsSigncryptedSharesImpl implements KmsSigncryptedShares {
  declare readonly [KmsSigncryptedSharesBrand]: never;
  readonly #metadata: KmsSigncryptedSharesMetadata;
  readonly #shares: readonly KmsSigncryptedShare[];

  constructor(metadata: KmsSigncryptedSharesMetadata, shares: readonly KmsSigncryptedShare[]) {
    this.#metadata = {
      kmsSignersContext: metadata.kmsSignersContext,
      eip712Domain: metadata.eip712Domain,
      eip712Signature: metadata.eip712Signature,
      eip712SignerAddress: metadata.eip712SignerAddress,
      handles: [...metadata.handles],
      tkmsVersion: metadata.tkmsVersion,
    };
    Object.freeze(this.#metadata);
    Object.freeze(this.#metadata.handles);

    this.#shares = [...shares];
    Object.freeze(this.#shares);
    this.#shares.forEach((share) => Object.freeze(share));
    if (this.#shares.length === 0) {
      throw new Error('Expected at least one signcrypted share.');
    }
  }

  public get tkmsVersion(): TkmsVersion {
    return this.#metadata.tkmsVersion;
  }

  public [GET_SHARES_FUNC](token: symbol): readonly KmsSigncryptedShare[] {
    if (token !== PRIVATE_KMS_SIGNCRYPTED_SHARES_TOKEN) {
      throw new Error('Unauthorized');
    }
    return this.#shares;
  }

  public [GET_METADATA_FUNC](token: symbol): KmsSigncryptedSharesMetadata {
    if (token !== PRIVATE_KMS_SIGNCRYPTED_SHARES_TOKEN) {
      throw new Error('Unauthorized');
    }
    return this.#metadata;
  }
}

type Context = {
  readonly runtime: FhevmRuntime;
  readonly chain: FhevmChain;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

/**
 * Asserts that at least one share exists and that all shares carry the same
 * `extraData`. Returns the common `extraData` value.
 */
function assertUniformExtraData(shares: readonly KmsSigncryptedShare[]): BytesHexNo0x {
  const firstShare = shares[0];
  if (firstShare === undefined) {
    throw new Error('Expected at least one signcrypted share.');
  }

  const firstExtraData = firstShare.extraData;
  for (let i = 1; i < shares.length; i++) {
    const share = shares[i];
    if (share !== undefined && share.extraData !== firstExtraData) {
      throw new Error(
        `Mismatched extraData across shares: share[0]="${firstExtraData}" vs share[${i}]="${share.extraData}".`,
      );
    }
  }

  return firstExtraData;
}

/**
 * Helper:
 * Verifies the EIP-712 signature of a single KMS signcrypted share.
 *
 * Rebuilds the `UserDecryptResponseVerification` payload the KMS node signed,
 * recovers the signer, and asserts it belongs to the reconciled KMS signers
 * context. This is an independent, JS-only check — the same verification is also
 * performed inside the tkms WASM during reconstruction (`verify=true`), so this
 * is primarily a test/defense-in-depth pass and is not required for correctness.
 *
 * @throws If the signer cannot be recovered, or is not a known KMS signer.
 */
export async function verifyKmsSigncryptedShare(
  context: Context,
  parameters: {
    readonly share: KmsSigncryptedShare;
    readonly metadata: KmsSigncryptedSharesMetadata;
    readonly transportPublicKey: BytesHex;
  },
): Promise<void> {
  const { share, metadata, transportPublicKey } = parameters;

  // Rebuild the exact EIP-712 message each KMS node signs per response share
  // (gateway Decryption.sol `UserDecryptResponseVerification`). `userDecryptedShare`
  // is the serialized share bytes carried in `share.payload`.
  const message = {
    publicKey: transportPublicKey,
    ctHandles: metadata.handles.map((h) => h.bytes32Hex),
    userDecryptedShare: ensure0x(share.payload),
    // v0 extraData is signed as '0x' (never '0x00').
    extraData: share.extraData === '' || share.extraData === '00' ? '0x' : ensure0x(share.extraData),
  };

  // Recover the signer of this share. The KMS EIP-712 domain lives in `metadata`.
  const [recoveredAddress] = await recoverSigners(context, {
    domain: metadata.eip712Domain,
    types: kmsUserDecryptResponseEip712Types,
    primaryType: 'UserDecryptResponseVerification',
    signatures: [ensure0x(share.signature)],
    message,
  });

  if (recoveredAddress === undefined) {
    throw new Error('Failed to recover signer from KMS signcrypted share signature.');
  }

  // The recovered signer must be one of the reconciled KMS signers (case-insensitive).
  if (!metadata.kmsSignersContext.has(recoveredAddress)) {
    throw new Error(
      `KMS signcrypted share signature was not signed by a known KMS signer (recovered ${recoveredAddress}).`,
    );
  }
}

/**
 * Creates a validated {@link KmsSigncryptedShares} instance.
 *
 * Enforces all invariants documented on {@link KmsSigncryptedSharesImpl}:
 * at least one share, uniform `extraData`, and consistency with the
 * {@link KmsSignersContext}.
 *
 * @throws If validation fails.
 * @internal
 */
export async function createKmsSigncryptedShares(
  context: Context,
  parameters: {
    readonly metadata: KmsSigncryptedSharesMetadata;
    readonly shares: readonly KmsSigncryptedShare[];
  },
): Promise<KmsSigncryptedShares> {
  const { metadata, shares } = parameters;

  /*
    TODO: add an optional signature verification pass. 
    (13 signatures verifications. Could be CPU costly)

    See: in KMS (eip712Domain)
    json.response[i].signature is an eip712 sig potentially on this message:

    struct UserDecryptResponseVerification {
        bytes publicKey;
        bytes32[] ctHandles;
        bytes userDecryptedShare;
        bytes extraData;
    }
  */

  // Assert context first — extraData comparison depends on a valid KmsSignersContext.
  assertIsKmsSignersContext(metadata.kmsSignersContext, {});

  // Extract the common extraData from all shares and validate its format.
  const relayerKmsExtraDataBytesHex: BytesHex = ensure0x(assertUniformExtraData(shares));

  // Reconcile KMS signer context using 'loose' mode.
  const reconciledKmsSignersContext: KmsSignersContext = await reconcileKmsSignersContext(context, {
    kmsVerifierAddress: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    protocolConfigAddress: context.chain.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
    relayerKmsExtraDataBytesHex,
    requestedKmsSignersContext: metadata.kmsSignersContext,
    mode: 'loose',
  });

  return new KmsSigncryptedSharesImpl({ ...metadata, kmsSignersContext: reconciledKmsSignersContext }, shares);
}

/**
 * Returns the validated shares. No additional verification is needed —
 * all invariants are enforced at {@link KmsSigncryptedSharesImpl} construction time.
 *
 * @internal
 */
export function getShares(signcryptedShares: KmsSigncryptedShares): readonly KmsSigncryptedShare[] {
  if (!(signcryptedShares instanceof KmsSigncryptedSharesImpl)) {
    throw new Error('Invalid KmsSigncryptedShares');
  }
  return signcryptedShares[GET_SHARES_FUNC](PRIVATE_KMS_SIGNCRYPTED_SHARES_TOKEN);
}

/**
 * @internal
 */
export function getMetadata(signcryptedShares: KmsSigncryptedShares): KmsSigncryptedSharesMetadata {
  if (!(signcryptedShares instanceof KmsSigncryptedSharesImpl)) {
    throw new Error('Invalid KmsSigncryptedShares');
  }
  return signcryptedShares[GET_METADATA_FUNC](PRIVATE_KMS_SIGNCRYPTED_SHARES_TOKEN);
}
