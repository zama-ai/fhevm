// In-SDK Solana user-decrypt de-signcryption. Uses the isolated, Solana-only TKMS WASM
// (kms_lib.v0.15.0-0-solana.*, exporting process_user_decryption_resp_solana_from_js) — kept separate
// from the EVM decrypt module's v0.13.10 blob because kms feature/solana is a newer, EVM-incompatible
// kms version. See FI#1546. DESIGNED TO BE DELETED once the EVM TKMS bindings are upgraded and the
// two blobs converge.
//
// This is the client half the kms-core live test used to own: it builds the ML-KEM transport
// keypair, and given the relayer's aggregated signcrypted shares + request inputs, de-signcrypts to
// cleartext. No kms checkout in the path.
import initSolanaTkms, {
  ml_kem_pke_keygen,
  ml_kem_pke_get_pk,
  ml_kem_pke_pk_to_u8vec,
  new_server_id_addr,
  new_solana_client,
  process_user_decryption_resp_solana_from_js,
  type PublicEncKeyMlKem512,
  type PrivateEncKeyMlKem512,
} from '../wasm/tkms/kms_lib.v0.15.0-0-solana.79de4926.js';
import { tkmsWasmBase64 } from '../wasm/tkms/kms_lib_bg.v0.15.0-0-solana.79de4926.wasm.base64.js';
import { base58 } from '@scure/base';
import type { FhevmSolanaKms } from '../core/types/fhevmSolanaChain.js';
import { bytesToHexNo0x } from '../core/base/bytes.js';
import { uint32ToBytes32 } from '../core/base/uint.js';
import { remove0x } from '../core/base/string.js';

let initialized: Promise<void> | undefined;
async function ensureInit(): Promise<void> {
  if (initialized === undefined) {
    const wasm = Uint8Array.from(atob(tkmsWasmBase64), (c) => c.charCodeAt(0));
    initialized = initSolanaTkms({ module_or_path: wasm }).then(() => undefined);
  }
  return initialized;
}

/** An ML-KEM transport keypair for Solana user-decrypt (the public key is bound into the request). */
export type SolanaTransportKeyPair = {
  readonly secretKey: PrivateEncKeyMlKem512;
  readonly publicKey: PublicEncKeyMlKem512;
  /** Serialized public key bytes — pass as `transportPublicKey` to the userDecrypt request. */
  readonly publicKeyBytes: Uint8Array;
};

/** Generate a fresh ML-KEM transport keypair (lazily initializes the Solana TKMS WASM). */
export async function generateSolanaTransportKeyPair(): Promise<SolanaTransportKeyPair> {
  await ensureInit();
  const secretKey = ml_kem_pke_keygen();
  const publicKey = ml_kem_pke_get_pk(secretKey);
  return { secretKey, publicKey, publicKeyBytes: ml_kem_pke_pk_to_u8vec(publicKey) };
}

export type SolanaSigncryptedShare = {
  readonly signature: string;
  readonly payload: string;
  readonly extraData: string;
};

/**
 * De-signcrypt the aggregated KMS shares to cleartext, entirely in-SDK. Verification is
 * fail-closed: each share must carry a valid node signature — from one of `kms.signers`, under
 * `kms.responseDomain` — over the Solana user-decryption link, recomputed here from the
 * (`kms.verifyingProgramId`, `hostChainId`) deployment pair, `solanaUserPubkey`, the handles, the
 * transport key and `extraData`. Returns one big-endian byte array per handle.
 */
export async function deSigncryptSolanaUserDecrypt(params: {
  readonly keyPair: SolanaTransportKeyPair;
  readonly shares: readonly SolanaSigncryptedShare[];
  readonly handles: readonly string[];
  readonly solanaUserPubkey: Uint8Array;
  readonly hostChainId: bigint;
  /** The request's `extra_data` bytes, exactly as sent to the relayer (hex, 0x optional). */
  readonly extraData: string;
  readonly kms: FhevmSolanaKms;
}): Promise<ReadonlyArray<{ bytes: Uint8Array; fheType: number }>> {
  await ensureInit();
  const serverAddrs = params.kms.signers.map((signer, index) => new_server_id_addr(index + 1, signer));
  const client = new_solana_client(serverAddrs, params.kms.fheParameter ?? 'default');
  // The de-signcryption only reads `enc_key`, `ciphertext_handles` and `extra_data`; the EVM-shaped
  // address fields exist solely because the WASM reuses the EVM-shaped request struct, and the
  // Solana path ignores them.
  const request = {
    signature: undefined,
    client_address: '0x0000000000000000000000000000000000000000',
    enc_key: bytesToHexNo0x(params.keyPair.publicKeyBytes),
    ciphertext_handles: params.handles.map(remove0x),
    eip712_verifying_contract: '0x0000000000000000000000000000000000000000',
    extra_data: remove0x(params.extraData),
  };
  const solanaRequest = {
    user_pubkey: bytesToHexNo0x(params.solanaUserPubkey),
    // A Solana chain id sets bit 63 and does not fit a JS number, so it travels as a decimal string.
    host_chain_id: params.hostChainId.toString(),
    verifying_program_id: bytesToHexNo0x(base58.decode(params.kms.verifyingProgramId)),
  };
  const responseDomain = {
    name: params.kms.responseDomain.name,
    version: params.kms.responseDomain.version,
    chain_id: uint32ToBytes32(params.kms.responseDomain.chainId),
    verifying_contract: params.kms.responseDomain.verifyingContract,
    salt: null,
  };
  const aggResp = params.shares.map((s) => ({
    signature: remove0x(s.signature),
    payload: remove0x(s.payload),
    extra_data: remove0x(s.extraData),
  }));
  const plaintexts = process_user_decryption_resp_solana_from_js(
    client,
    request,
    solanaRequest,
    aggResp,
    params.keyPair.publicKey,
    params.keyPair.secretKey,
    responseDomain,
  );
  // The wrapper already converts LE -> BE, so `bytes` is the big-endian plaintext.
  return plaintexts.map((p) => ({ bytes: p.bytes, fheType: p.fhe_type }));
}
