// Turning the KMS answer into plaintext, and refusing every answer that is not this request's.
//
// Four rules stand between the shares and a plaintext, and all four are about the same thing: the
// response must be bound to the request the client actually made. The link is recomputed here from the
// client's own fields — never from anything the response echoed back, which is why this module's
// parameters carry no field a response could supply. Each share's signature is checked before its link
// is compared, so an unauthenticated share cannot even be considered. The comparison is byte equality.
// And the shares that survive must agree on one link and reach the threshold: a set that disagrees
// yields nothing rather than a partial answer.
//
// The rules live inside the KMS client (the vendored TKMS blob), which is deliberate — the same code
// the KMS side runs, rather than a second implementation of the same arithmetic in TypeScript. What
// belongs here is passing it the client's own inputs, and the committed linker vectors are what proves
// the link this SDK computes is the link the KMS computes.
//
// One rule does live here: the typed answer. The link binds the handles' bytes, not the payload's
// type field, so the verified plaintexts are checked against the request's handles — one per handle,
// in order, each of the type its handle embeds — before anything is returned.

import initSolanaTkms, {
  type PrivateEncKeyMlKem512,
  type PublicEncKeyMlKem512,
  compute_solana_user_decrypt_link_from_js,
  ml_kem_pke_get_pk,
  ml_kem_pke_keygen,
  ml_kem_pke_pk_to_u8vec,
  new_server_id_addr,
  new_solana_client,
  process_user_decryption_resp_solana_from_js,
} from '../../wasm/tkms/kms_lib.v0.15.0-0-solana.024279f8.js';
import { tkmsWasmBase64 } from '../../wasm/tkms/kms_lib_bg.v0.15.0-0-solana.024279f8.wasm.base64.js';
import { bytes32ToHandle } from '../../core/handle/FhevmHandle.js';
import { bytesToHexNo0x, isBytes32 } from '../../core/base/bytes.js';
import { isomorphicCompileWasmFromBase64 } from '../../core/base/wasm.js';
import { remove0x } from '../../core/base/string.js';

/** One lazy initialization of the vendored blob, shared by every entry point of this module. */
let initialized: Promise<void> | undefined;

function ensureInit(): Promise<void> {
  initialized ??= isomorphicCompileWasmFromBase64(tkmsWasmBase64)
    .then((module) => initSolanaTkms({ module_or_path: module }))
    .then(() => undefined);
  return initialized;
}

/** The ML-KEM transport keypair of one permit session. The secret key never leaves the client. */
export interface SolanaTransportKeyPair {
  readonly secretKey: PrivateEncKeyMlKem512;
  readonly publicKey: PublicEncKeyMlKem512;
  /** The serialized container the permit commits to, and the request carries. */
  readonly publicKeyBytes: Uint8Array;
}

/**
 * One KMS party, as the response verification must be told about it.
 *
 * This is trusted configuration, not response data: the set comes from the host program's KMS-context
 * signer set. Taking it from the response would make the response its own authority.
 */
export interface SolanaKmsSigner {
  /** The party id the KMS uses for this signer; unique within the set. */
  readonly partyId: number;
  /** The signer's address, as the registry records it. */
  readonly address: string;
}

/** One signcrypted share, as the relayer returns it. */
export interface SolanaSigncryptedShare {
  readonly signature: string;
  readonly payload: string;
  readonly extraData: string;
}

/** One decrypted value: its big-endian bytes and the FHE type it was encrypted under. */
export interface SolanaUserDecryptPlaintext {
  readonly bytes: Uint8Array;
  readonly fheTypeId: number;
}

/**
 * The gateway EIP-712 domain a KMS node signed the response's external signature under.
 *
 * Trusted configuration, like the signer set: a domain taken from the response would let the
 * response choose what it is verified against. Without one, verification treats the domain as
 * empty — under which no real external signature verifies, so real responses are refused by the
 * signature rule rather than accepted unchecked.
 */
export interface SolanaGatewayEip712Domain {
  readonly name: string;
  readonly version: string;
  /** The gateway chain id — the EVM chain the KMS signs for, not the Solana host id. */
  readonly chainId: bigint;
  /** The gateway's verifying contract, as a 0x-hex EVM address. */
  readonly verifyingContract: string;
}

/**
 * The fields the link is computed over. Every one of them is the client's own.
 *
 * There is no `link` parameter and no room for one: a caller who could pass a link would be able to
 * pass the one the response carries, which is the substitution the whole construction exists to stop.
 */
export interface SolanaUserDecryptLinkInputs {
  /** The recipient: the permit's 32-byte user pubkey. */
  readonly userPubkey: Uint8Array;
  /** The host chain id the permit was signed for. */
  readonly hostChainId: bigint;
  /** The 32-byte program id the permit was signed for. */
  readonly verifyingProgramId: Uint8Array;
  /** The signed KMS context id. */
  readonly kmsContextId: Uint8Array;
  /** The signed KMS epoch id. */
  readonly kmsEpochId: Uint8Array;
  /** The requested handles, in the order the request carried them — position is part of the link. */
  readonly handles: readonly Uint8Array[];
  /** The serialized transport key, in full: the link commits to the key, not to its fingerprint. */
  readonly transportKey: Uint8Array;
}

/**
 * Generates a transport keypair for one permit session.
 *
 * A fresh pair per permit: the permit commits to this key's fingerprint, so reusing a key across
 * permits would let one permit's response be de-signcrypted under another's.
 */
export async function generateSolanaTransportKeyPair(): Promise<SolanaTransportKeyPair> {
  await ensureInit();
  const secretKey = ml_kem_pke_keygen();
  const publicKey = ml_kem_pke_get_pk(secretKey);
  return { secretKey, publicKey, publicKeyBytes: ml_kem_pke_pk_to_u8vec(publicKey) };
}

/**
 * Computes the link a response must carry, from the client's own request fields.
 *
 * Exported for the committed linker vectors, which are the only thing that makes "the SDK and the KMS
 * compute the same link" checkable rather than assumed.
 *
 * @param inputs - The client's own fields.
 * @throws If the inputs are not a well-formed request: an identity of the wrong width, a handle that
 *   is not a Solana handle, an empty handle list, handles disagreeing about their embedded chain id, or
 *   an embedded chain id that is not the declared one.
 */
export async function solanaUserDecryptLink(inputs: SolanaUserDecryptLinkInputs): Promise<Uint8Array> {
  await ensureInit();
  // Marshalling and nothing else: the one canonical link computation lives in the blob, and a
  // second copy here would be a link rule that agrees today and drifts tomorrow.
  return compute_solana_user_decrypt_link_from_js(
    inputs.userPubkey,
    inputs.hostChainId,
    inputs.verifyingProgramId,
    inputs.kmsContextId,
    inputs.kmsEpochId,
    inputs.handles.map((handle) => bytesToHexNo0x(handle)),
    inputs.transportKey,
  );
}

/**
 * Verifies the KMS response and returns the plaintexts, or refuses the whole response.
 *
 * All or nothing: a share whose link does not match the recomputed one is discarded, and if what
 * remains cannot reconstruct, the call fails rather than returning the values it could recover. A
 * partial answer would be indistinguishable from a complete one to the caller.
 *
 * @param response.link - The request fields the link is recomputed from.
 * @param response.shares - The signcrypted shares as returned, in any order.
 * @param response.keyPair - The session's transport keypair; the secret key stays here.
 * @param response.signers - The registered KMS signer set, from trusted configuration.
 * @param response.fheParameter - The FHE parameter choice the client was built for.
 * @param response.gatewayEip712Domain - The domain node signatures verify under; absent, no real
 * external signature verifies and every real response is refused fail-closed.
 * @throws If no set of authenticated shares agrees on the recomputed link and reaches the threshold.
 */
export async function verifySolanaUserDecryptResponse(response: {
  readonly link: SolanaUserDecryptLinkInputs;
  readonly shares: readonly SolanaSigncryptedShare[];
  readonly keyPair: SolanaTransportKeyPair;
  readonly signers: readonly SolanaKmsSigner[];
  readonly fheParameter: string;
  readonly gatewayEip712Domain?: SolanaGatewayEip712Domain | undefined;
}): Promise<readonly SolanaUserDecryptPlaintext[]> {
  // No shares is not an empty answer: it is a response that reaches no threshold, said here in the
  // request's own terms rather than left to a reconstruction error about a missing pivot.
  if (response.shares.length === 0) {
    throw new Error('the response carries no shares, and a response with no shares reaches no threshold');
  }
  await ensureInit();

  // The trust anchor: the registered signer set, from configuration the caller read on chain. A
  // key carried inside the response acts only under its binding to one of these addresses.
  const client = new_solana_client(
    response.signers.map((signer) => new_server_id_addr(signer.partyId, signer.address)),
    response.fheParameter,
  );

  // The request half of the link contract, in the blob's request shape. The EVM-shaped fields are
  // zeroed, not read on this path: the recipient is the raw ed25519 key below, and the link is the
  // Solana binding computed from the explicit arguments plus the handles and transport key here —
  // every one of them the client's own, none of them from the response.
  const request = {
    signature: undefined,
    client_address: '0x0000000000000000000000000000000000000000',
    enc_key: bytesToHexNo0x(response.link.transportKey),
    ciphertext_handles: response.link.handles.map((handle) => bytesToHexNo0x(handle)),
    eip712_verifying_contract: '0x0000000000000000000000000000000000000000',
    extra_data: '00',
  };

  const aggResp = response.shares.map((share) => ({
    signature: remove0x(share.signature),
    payload: remove0x(share.payload),
    extra_data: remove0x(share.extraData),
  }));

  // The trailing EIP-712 domain comes from trusted configuration; omitted, the blob treats it as
  // the empty domain — under which no external signature verifies, so a response is refused by the
  // signature rule rather than accepted unchecked.
  const plaintexts = process_user_decryption_resp_solana_from_js(
    client,
    request,
    response.link.userPubkey,
    response.link.hostChainId,
    response.link.verifyingProgramId,
    response.link.kmsContextId,
    response.link.kmsEpochId,
    aggResp,
    response.keyPair.publicKey,
    response.keyPair.secretKey,
    response.gatewayEip712Domain === undefined ? undefined : gatewayDomainWasmArg(response.gatewayEip712Domain),
  );

  // The wrapper already converts little-endian to big-endian, so `bytes` is the plaintext as-is.
  const typed = plaintexts.map((plaintext) => ({ bytes: plaintext.bytes, fheTypeId: plaintext.fhe_type }));
  verifySolanaUserDecryptPlaintexts(typed, response.link.handles);
  return typed;
}

/**
 * Verifies that the plaintexts are the typed answer to these handles: one per handle, in request
 * order, each of the FHE type its handle embeds.
 *
 * The link binds the handles' bytes, not the payload's type field, so every link rule passes when
 * the KMS answers under the right link with the wrong type — a euint64 released as an ebool. This
 * is the one rule that reads the type, and it lives here because the response layer is the only
 * layer holding both the plaintexts and the handles they answer.
 *
 * Exported so it can be pinned directly: the committed vectors carry no signcrypted shares, so no
 * test reaches this check through a full verification.
 *
 * @param plaintexts - The decrypted values, as verification produced them.
 * @param handles - The requested handles, in the order the request carried them.
 * @throws If the counts differ, or any plaintext is not of its handle's type.
 */
export function verifySolanaUserDecryptPlaintexts(
  plaintexts: readonly SolanaUserDecryptPlaintext[],
  handles: readonly Uint8Array[],
): void {
  if (plaintexts.length !== handles.length) {
    throw new Error(`the response carries ${plaintexts.length} plaintext(s) for ${handles.length} requested handle(s)`);
  }
  for (const [index, handle] of handles.entries()) {
    const plaintext = plaintexts[index];
    // The count check above proved this much; the narrowing re-states it for the type system.
    if (plaintext === undefined) {
      throw new Error(
        `the response carries ${plaintexts.length} plaintext(s) for ${handles.length} requested handle(s)`,
      );
    }
    if (!isBytes32(handle)) {
      throw new Error(`the handle at position ${index} is not a 32-byte handle, and embeds no type to check against`);
    }
    const expected = bytes32ToHandle(handle).fheTypeId;
    if (plaintext.fheTypeId !== expected) {
      throw new Error(
        `plaintext ${index} is of FHE type ${plaintext.fheTypeId}, and the handle at that position asks for type ${expected}`,
      );
    }
  }
}

/**
 * The domain in the blob's JS shape: the chain id as 32 big-endian bytes, no salt.
 *
 * @param domain - The configured gateway domain.
 */
function gatewayDomainWasmArg(domain: SolanaGatewayEip712Domain): {
  readonly name: string;
  readonly version: string;
  readonly chain_id: Uint8Array;
  readonly verifying_contract: string;
  readonly salt: null;
} {
  const chainId = new Uint8Array(32);
  let value = domain.chainId;
  for (let index = 31; index >= 0 && value > 0n; index -= 1) {
    chainId[index] = Number(value & 0xffn);
    value >>= 8n;
  }
  return {
    name: domain.name,
    version: domain.version,
    chain_id: chainId,
    verifying_contract: domain.verifyingContract,
    salt: null,
  };
}
