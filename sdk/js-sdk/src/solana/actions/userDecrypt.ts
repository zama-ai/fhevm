import type { SolanaUserDecryptSigner } from '../signer.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { WithDecrypt } from '../../core/types/coreFhevmRuntime.js';
import type { FetchUserDecryptResult, RelayerUserDecryptOptions } from '../../core/types/relayer.js';
import type { EncryptedValueLike } from '../../core/types/encryptedTypes.js';
import type { ClearValue, Handle } from '../../core/types/encryptedTypes-p.js';
import type { Bytes32Hex } from '../../core/types/primitives.js';
import {
  buildSolanaUserDecryptContextExtraData,
  buildSolanaUserDecryptMmrProofExtraData,
  solanaUserDecryptClientId,
  solanaUserDecryptSigningPreimage,
  SOLANA_USER_DECRYPT_ATTESTATION_TYPE,
} from '../../core/coprocessor/SolanaUserDecrypt-p.js';
import { toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { bytesToHex, hexToBytes32 } from '../../core/base/bytes.js';
import { removeSuffix } from '../../core/base/string.js';
import { RelayerAsyncRequest } from '../../core/modules/relayer/module/RelayerAsyncRequest.js';
import { createClearValue } from '../../core/handle/ClearValue.js';
import { bytesToClearValueType } from '../../core/handle/FheType.js';
import { generateSolanaTransportKeyPair, deSigncryptSolanaUserDecrypt } from '../deSigncrypt.js';
import {
  verifyHistoricalAccessProof,
  verifyPublicDecryptProof,
  type MmrProof,
} from '../proof.js';

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
  /** Override the chain's ACL domain keys for this request (each a bytes32 0x-hex). */
  readonly allowedAclDomainKeys?: readonly Bytes32Hex[] | undefined;
  /** 32-byte big-endian context id. Defaults to all-zero (no explicit context). */
  readonly contextId?: Uint8Array | undefined;
  /**
   * Per-request 32-byte nonce, bound into the ed25519 signing preimage (a random one is generated
   * when omitted). It binds the nonce to the signed request but is NOT dedup-enforced on-chain or in
   * the connector; replay is bounded by the validity window, matching the EVM user-decrypt path.
   */
  readonly nonce?: Uint8Array | undefined;
  /** Validity window. Defaults to `now` for 1 day. */
  readonly validity?:
    | {
        readonly startTimestamp: bigint;
        readonly durationSeconds: bigint;
      }
    | undefined;
  readonly options?: RelayerUserDecryptOptions | undefined;
  /**
   * A historical/public MMR inclusion proof (RFC-024) authorizing this decrypt against the
   * `EncryptedValue` lineage, instead of the live current-handle ACL. Single-handle only (a
   * proof authorizes exactly one handle), so `handles` must have length 1 when this is set.
   *
   * DESIGN NOTE — "historical vs current" signal: at the time this was wired, the SDK had no
   * existing notion of a historical/current distinction (no `isHistorical`/`blockNumber`-style
   * flag anywhere in the request-building path). Rather than invent one, presence of this field
   * IS the signal: callers who supply `mmrProof` get the proof-gated path (verified client-side
   * below, then attached to the request); callers who omit it get the plain current-ACL path,
   * unchanged. If a first-class historical/current signal is added to the SDK later, this should
   * be reconciled with it rather than kept as a second, parallel signal.
   */
  readonly mmrProof?: SolanaUserDecryptMmrProofParameter | undefined;
};

/** The MMR proof inputs needed to both verify (client-side, pre-sign) and attach a proof-gated
 * Solana user-decrypt request. Field names mirror `proof.ts` / `solana_extra_data.rs`. */
export type SolanaUserDecryptMmrProofParameter = {
  /** The lineage's canonical PDA account (`encrypted_value_account` in `proof.ts`). */
  readonly encryptedValueAccount: Uint8Array;
  /** The lineage value key naming the `EncryptedValue` account (`acl_value_key` on the wire). */
  readonly aclValueKey: Uint8Array;
  /** The live MMR peaks fetched from the lineage account, used for client-side verification. */
  readonly peaks: readonly Uint8Array[];
  /** The live `leaf_count` fetched from the lineage account, used for client-side verification. */
  readonly leafCount: bigint;
  /** The lineage `leaf_count` the proof was built against (`proof_slot` on the wire). */
  readonly proofSlot: bigint;
  /** The decoded inclusion proof (leaf index + sibling path), used for client-side verification. */
  readonly proof: MmrProof;
  /** The full MMR-proof transport blob (mode prefix ‖ Borsh proof) attached verbatim to the wire
   * request; committed into the signed preimage byte-for-byte. */
  readonly mmrProofBytes: Uint8Array;
  /** Whether this proof authorizes a historical access grant or an exact public-decrypt leaf. A
   * historical proof additionally requires `subject`. */
  readonly mode: 'historical' | 'public';
  /** The subject the historical-access proof was issued to. Required when `mode` is `'historical'`. */
  readonly subject?: Uint8Array | undefined;
};

/** One aggregated KMS signcrypted share, as returned by the relayer's v3 user-decrypt job. */
type SolanaUserDecryptShare = {
  /** The KMS party's external signature over its share (0x-hex). */
  readonly signature: string;
  /** The bincode-serialized signcrypted payload (hex). */
  readonly payload: string;
  /** Per-share extra data (hex). */
  readonly extraData: string;
};

/**
 * The decrypted clear values, one per requested handle, in request order. Mirrors the EVM
 * user-decrypt return: de-signcryption runs entirely in-SDK against the vendored Solana TKMS WASM
 * (no kms-core), differing from EVM only in the link digest (`compute_link_solana`).
 */
export type SolanaUserDecryptResult = readonly ClearValue[];

////////////////////////////////////////////////////////////////////////////////

const DEFAULT_DURATION_SECONDS = 86_400n;
const ED25519_SIGNATURE_LEN = 64;

// Per-flow provenance token for the clear values this action produces (see {@link createClearValue}).
const SOLANA_USER_DECRYPT_TOKEN = Symbol('SolanaUserDecrypt.clearValue');

function randomNonce(): Uint8Array {
  const nonce = new Uint8Array(32);
  crypto.getRandomValues(nonce);
  return nonce;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Runs the full Solana user-decrypt round-trip and returns the decrypted clear values:
 *
 * 1. derive `identity` from the signer and assemble the ed25519 signing preimage,
 * 2. sign it via the {@link SolanaUserDecryptSigner} and build the v3 attested request,
 * 3. POST it to the relayer's `/v3/user-decrypt` Solana seam and poll for the signcrypted shares,
 * 4. de-signcrypt the shares to cleartext in-SDK (vendored Solana TKMS WASM, no kms-core).
 *
 * The action owns the ephemeral ML-KEM transport keypair end to end, so the caller never handles
 * transport keys — matching the EVM user-decrypt flow, which differs only in the link digest.
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

  // The action owns the ephemeral ML-KEM transport keypair: its public key is bound into the signed
  // request and its secret key de-signcrypts the response (step 4), so it never leaves this scope.
  const keyPair = await generateSolanaTransportKeyPair();
  const publicKey = keyPair.publicKeyBytes;

  const contextId = parameters.contextId ?? new Uint8Array(32);
  const nonce = parameters.nonce ?? randomNonce();

  const allowedAclDomainKeysHex = parameters.allowedAclDomainKeys ?? chain.fhevm.acl.domainKeys;
  const allowedAclDomainKeys = allowedAclDomainKeysHex.map((k) => hexToBytes32(k));

  const startTimestamp = parameters.validity?.startTimestamp ?? BigInt(Math.floor(Date.now() / 1000));
  const durationSeconds = parameters.validity?.durationSeconds ?? DEFAULT_DURATION_SECONDS;

  const mmrProof = parameters.mmrProof;
  if (mmrProof !== undefined && handleBytes.length !== 1) {
    // An MMR proof authorizes exactly one handle (see `proof.ts` / the connector's
    // `require_single_handle`); a multi-handle request has no single handle for the proof to
    // name, so refuse rather than silently ignoring the proof.
    throw new Error('an MMR proof (`mmrProof`) can only be attached to a single-handle request');
  }
  if (mmrProof !== undefined) {
    // Refuse to sign over a proof that would fail on-chain/in-connector verification: verifying
    // client-side, before signing, catches a malformed or stale proof here instead of burning a
    // relayer round trip (or worse, silently trusting an unverified proof).
    const handle = handleBytes[0];
    if (!handle) {
      throw new Error('missing handle for MMR proof verification');
    }
    const verified =
      mmrProof.mode === 'historical'
        ? (() => {
            if (!mmrProof.subject) {
              throw new Error('a historical MMR proof requires `subject`');
            }
            return verifyHistoricalAccessProof(
              mmrProof.encryptedValueAccount,
              mmrProof.peaks,
              mmrProof.leafCount,
              handle,
              mmrProof.subject,
              mmrProof.proof,
            );
          })()
        : verifyPublicDecryptProof(
            mmrProof.encryptedValueAccount,
            mmrProof.peaks,
            mmrProof.leafCount,
            handle,
            mmrProof.proof,
          );
    if (!verified) {
      throw new Error(
        `MMR proof failed client-side verification (mode=${mmrProof.mode}); refusing to sign the decrypt request`,
      );
    }
  }

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
    aclValueKey: mmrProof?.aclValueKey,
    mmrProofBytes: mmrProof?.mmrProofBytes,
    proofSlot: mmrProof?.proofSlot,
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
      extraData: bytesToHex(
        mmrProof
          ? buildSolanaUserDecryptMmrProofExtraData(
              contextId,
              mmrProof.aclValueKey,
              mmrProof.proofSlot,
              mmrProof.mmrProofBytes,
            )
          : buildSolanaUserDecryptContextExtraData(contextId),
      ),
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

  // 4. De-signcrypt the aggregated shares to cleartext in-SDK, then reconstruct typed clear values
  // with the same decoder as the EVM path (`bytesToClearValueType`), one per requested handle.
  const plaintexts = await deSigncryptSolanaUserDecrypt({
    keyPair,
    shares,
    handles: handlesHex,
    solanaUserPubkey: identity,
    hostChainId: BigInt(chain.id),
  });

  return plaintexts.map((plaintext, i) => {
    const handle = handles[i];
    if (!handle) {
      throw new Error(`missing handle at index ${i}`);
    }
    if (plaintext.fheType !== handle.fheTypeId) {
      throw new Error(
        `unexpected FHE type at index ${i}: got ${plaintext.fheType}, expected ${handle.fheTypeId}`,
      );
    }
    return createClearValue({
      value: bytesToClearValueType(handle.fheType, plaintext.bytes),
      handle,
      originToken: SOLANA_USER_DECRYPT_TOKEN,
    });
  });
}
