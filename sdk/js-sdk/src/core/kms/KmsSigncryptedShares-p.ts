import type { KmsSigncryptedSharesMetadata, KmsSigncryptedShare } from '../types/kms-p.js';
import type { KmsSigncryptedShares, KmsSigncryptedSharesBrand } from '../types/kms.js';
import type { BytesHex, BytesHexNo0x, ChecksummedAddress } from '../types/primitives.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import { assertIsKmsSignersContext } from '../host-contracts/KmsSignersContext-p.js';
import { ensure0x } from '../base/string.js';
import { assertIsKmsExtraData } from './kmsExtraData.js';
import { reconcileKmsSignersContext } from '../host-contracts/readKmsSignersContext-p.js';

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
    };
    Object.freeze(this.#metadata);
    Object.freeze(this.#metadata.handles);

    this.#shares = [...shares];
    Object.freeze(this.#shares);
    this.#shares.forEach((share) => Object.freeze(share));
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
  const relayerExtraData: BytesHex = ensure0x(assertUniformExtraData(shares));
  assertIsKmsExtraData(relayerExtraData, {});

  // Reconcile KMS signer context using 'loose' mode.
  const reconciledKmsSignersContext: KmsSignersContext = await reconcileKmsSignersContext(context, {
    address: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    relayerExtraData,
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
