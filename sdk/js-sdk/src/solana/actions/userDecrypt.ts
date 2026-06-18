import type { SolanaUserDecryptSigner } from '../signer.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { WithDecrypt } from '../../core/types/coreFhevmRuntime.js';
import type { FetchUserDecryptResult, RelayerUserDecryptOptions } from '../../core/types/relayer.js';
import type { EncryptedValueLike } from '../../core/types/encryptedTypes.js';
import type { Handle } from '../../core/types/encryptedTypes-p.js';
import type { Bytes32Hex, BytesHex } from '../../core/types/primitives.js';
import {
  buildSolanaUserDecryptContextExtraData,
  solanaUserDecryptClientId,
  solanaUserDecryptSigningPreimage,
  SOLANA_USER_DECRYPT_ATTESTATION_TYPE,
} from '../../core/coprocessor/SolanaUserDecrypt-p.js';
import { toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { bytesToHex, hexToBytes, hexToBytes32 } from '../../core/base/bytes.js';
import { removeSuffix } from '../../core/base/string.js';
import { RelayerAsyncRequest } from '../../core/modules/relayer/module/RelayerAsyncRequest.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Context the {@link userDecrypt} action runs against. The Solana host has no on-chain
 * ACL/KMSVerifier to read; the runtime is used only for the relayer request auth config.
 */
export type SolanaUserDecryptContext = {
  readonly chain: FhevmSolanaChain;
  readonly runtime: WithDecrypt;
  readonly options: { readonly batchRpcCalls: boolean };
};

export type SolanaUserDecryptParameters = {
  /** The ciphertext handles to decrypt (each a 32-byte handle). */
  readonly handles: readonly EncryptedValueLike[];
  /**
   * The ML-KEM re-encryption public key (0x-hex) the KMS seals each share to; it is bound into the
   * signed preimage. The caller owns the matching key pair and de-signcrypts the returned shares
   * (the SDK does not de-signcrypt Solana responses yet — see {@link SolanaUserDecryptResult}).
   */
  readonly transportPublicKey: BytesHex;
  /** Override the chain's ACL domain keys for this request (each a bytes32 0x-hex). */
  readonly allowedAclDomainKeys?: readonly Bytes32Hex[] | undefined;
  /** 32-byte big-endian context id. Defaults to all-zero (no explicit context). */
  readonly contextId?: Uint8Array | undefined;
  /** Per-request 32-byte anti-replay nonce. A random one is generated when omitted. */
  readonly nonce?: Uint8Array | undefined;
  /** Validity window. Defaults to `now` for 1 day. */
  readonly validity?:
    | {
        readonly startTimestamp: bigint;
        readonly durationSeconds: bigint;
      }
    | undefined;
  readonly options?: RelayerUserDecryptOptions | undefined;
};

/** One aggregated KMS signcrypted share, as returned by the relayer's v3 user-decrypt job. */
export type SolanaUserDecryptShare = {
  /** The KMS party's external signature over its share (0x-hex). */
  readonly signature: string;
  /** The bincode-serialized signcrypted payload (hex). */
  readonly payload: string;
  /** Per-share extra data (hex). */
  readonly extraData: string;
};

/**
 * The result of a Solana user-decrypt round-trip up to the aggregated shares.
 *
 * NOTE: this stops at the signcrypted shares. De-signcryption to cleartext binds against the opaque
 * keccak `compute_link_solana` digest (RFC-021), which the SDK's bundled TKMS WASM does not yet
 * expose — only kms-core (`process_user_decryption_resp_solana`) does. Completing the round-trip in
 * the SDK requires exposing that Solana link path in the TKMS WASM (a KMS/TKMS change, tracked
 * separately). Until then the caller de-signcrypts the returned {@link shares} with their own
 * transport key pair (whose public key was passed as `transportPublicKey`) via kms-core.
 */
export type SolanaUserDecryptResult = {
  readonly shares: readonly SolanaUserDecryptShare[];
};

////////////////////////////////////////////////////////////////////////////////

const DEFAULT_DURATION_SECONDS = 86_400n;
const ED25519_SIGNATURE_LEN = 64;

function randomNonce(): Uint8Array {
  const nonce = new Uint8Array(32);
  crypto.getRandomValues(nonce);
  return nonce;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Runs the Solana user-decrypt round-trip up to the aggregated signcrypted shares:
 *
 * 1. derive `identity` from the signer and assemble the ed25519 signing preimage,
 * 2. sign it via the {@link SolanaUserDecryptSigner} and build the v3 attested request,
 * 3. POST it to the relayer's `/v3/user-decrypt` Solana seam and poll for the signcrypted shares.
 *
 * It returns the {@link SolanaUserDecryptShare}s; de-signcryption to cleartext is intentionally out
 * of scope until the Solana keccak-link path is exposed in the TKMS WASM (see
 * {@link SolanaUserDecryptResult}).
 */
export async function userDecrypt(
  context: SolanaUserDecryptContext,
  signer: SolanaUserDecryptSigner,
  parameters: SolanaUserDecryptParameters,
): Promise<SolanaUserDecryptResult> {
  const { chain, runtime } = context;

  if (parameters.handles.length === 0) {
    throw new Error('At least one handle is required');
  }

  const identity = signer.publicKey;
  const handles: readonly Handle[] = parameters.handles.map((h) => toFhevmHandle(h));
  const handleBytes: readonly Uint8Array[] = handles.map((h) => h.bytes32);

  const publicKey = hexToBytes(parameters.transportPublicKey);

  const contextId = parameters.contextId ?? new Uint8Array(32);
  const nonce = parameters.nonce ?? randomNonce();

  const allowedAclDomainKeysHex = parameters.allowedAclDomainKeys ?? chain.fhevm.acl.domainKeys;
  const allowedAclDomainKeys = allowedAclDomainKeysHex.map((k) => hexToBytes32(k));

  const startTimestamp = parameters.validity?.startTimestamp ?? BigInt(Math.floor(Date.now() / 1000));
  const durationSeconds = parameters.validity?.durationSeconds ?? DEFAULT_DURATION_SECONDS;

  // 1. + 2. Build the canonical ed25519 preimage (the same bytes the KMS connector re-derives),
  // sign it via the abstract signer, then assemble the request. `buildSolanaUserDecryptRequest`
  // signs from a raw seed; an abstract signer is opaque, so we route through the same exported
  // preimage + client-id helpers and attach the externally-produced signature.
  const input = {
    contractsChainId: chain.id,
    publicKey,
    handles: handleBytes,
    identity,
    contextId,
    nonce,
    allowedAclDomainKeys,
    startTimestamp,
    durationSeconds,
  };

  const preimage = solanaUserDecryptSigningPreimage(input);
  const signature = await signer.sign(preimage);
  if (signature.length !== ED25519_SIGNATURE_LEN) {
    throw new Error(`unexpected ed25519 signature length: ${signature.length}`);
  }

  const userAddress = solanaUserDecryptClientId(identity);
  const signatureHex = bytesToHex(signature);
  const handlesHex = handleBytes.map((h) => bytesToHex(h));

  // 3. POST the v3 attested envelope to the relayer's Solana ed25519 seam and poll for the
  // signcrypted shares. The shape is the relayer's `AttestedUserDecryptRequestJson`: the ed25519
  // auth fields travel as the typed `solana*` fields, EVM `allowedContracts` is empty (the Solana
  // ACL scope is `solanaAllowedAclDomainKeys`), and every hex field stays 0x-prefixed. The v3
  // payload uses `deny_unknown_fields`, so it carries exactly these keys and no v2 extras. The
  // connector ignores the EVM-shaped address fields on the Solana arm (it derives the subject from
  // `solanaUserIdentity`), but they must still parse as 20-byte addresses, so we reuse the derived
  // client id for `userAddress` / `contractAddress` / `ownerAddress`.
  const relayerOptions = {
    auth: runtime.config.auth,
    ...parameters.options,
  };

  const relayerPayload = {
    attestationType: SOLANA_USER_DECRYPT_ATTESTATION_TYPE,
    attestedPayload: {
      version: '2.0',
      type: 'user_decryption',
      handles: handlesHex.map((ctHandle) => ({
        ctHandle,
        contractAddress: userAddress,
        ownerAddress: userAddress,
      })),
      userAddress,
      allowedContracts: [] as readonly string[],
      requestValidity: {
        startTimestamp: startTimestamp.toString(),
        durationSeconds: durationSeconds.toString(),
      },
      publicKey: bytesToHex(publicKey),
      extraData: bytesToHex(buildSolanaUserDecryptContextExtraData(contextId)),
      solanaUserIdentity: bytesToHex(identity),
      solanaNonce: bytesToHex(nonce),
      solanaAllowedAclDomainKeys: allowedAclDomainKeys.map((k) => bytesToHex(k)),
    },
    signature: signatureHex,
  };

  const asyncRequest = new RelayerAsyncRequest({
    relayerOperation: 'USER_DECRYPT',
    url: `${removeSuffix(chain.fhevm.relayerUrl, '/')}/v3/user-decrypt`,
    payload: relayerPayload,
    options: relayerOptions,
  });

  const result = (await asyncRequest.run()) as FetchUserDecryptResult;

  const shares: readonly SolanaUserDecryptShare[] = result.map((r) => ({
    signature: r.signature,
    payload: r.payload,
    extraData: r.extraData,
  }));

  return { shares };
}
