// Turning the KMS answer into plaintext, and refusing every answer that is not this request's.
//
// A Solana user decryption runs the KMS client's user-decryption path, through the same WASM module
// and entry points as the EVM decrypt module. The user address is base58 of the permit's 32-byte
// user key, and that format is what makes the client hold the response to the Solana link: the
// EIP-712 `SolanaUserDecryptionLinker(publicKey, handles, userAddress)` struct hashed under the
// gateway's `Decryption` domain. The client checks each share's node signature, the link, and the
// 2t+1 agreement; a set that fails any of them yields nothing rather than a partial answer.
//
// Every input passed to the client is the client's own: the permit's fields, the requested handles
// and the trust configuration. None comes from the response, which is why this module's parameters
// have no field a response could supply. The request's `extra_data`, the KMS route the permit
// signed, is not a link input; the node signature on each share covers it.
//
// One rule does live here: the typed answer. The link binds the handles' bytes, not the payload's
// type field, so the verified plaintexts are checked against the request's handles — one per handle,
// in order, each of the type its handle embeds — before anything is returned.

import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { PrivateEncKeyMlKem512, PublicEncKeyMlKem512 } from '../../wasm/tkms/KmsLibApi.js';
import { getAddressDecoder } from '@solana/kit';
import { initTkmsModule } from '../../core/modules/decrypt/module/init-p.js';
import { bytes32ToHandle } from '../../core/handle/FhevmHandle.js';
import { bytesToHexNo0x, isBytes32 } from '../../core/base/bytes.js';
import { remove0x } from '../../core/base/string.js';
import { toChecksummedAddress } from '../../core/base/address.js';

/**
 * The EIP-55 form of a configured EVM address, for the client's checksum-validating parser.
 *
 * The client refuses anything but EIP-55 mixed case ("Bad address checksum"), while configuration
 * produced from on-chain bytes is naturally all-lowercase — a valid address the parser would
 * reject. Only the case-insensitive spellings are re-encoded here: all-lowercase and all-uppercase
 * carry no checksum to preserve. A mixed-case spelling claims a checksum and passes through
 * untouched, so a wrong claim is still refused by the client — re-encoding it would erase the typo
 * protection EIP-55 exists for. A string that is not a 20-byte 0x-hex also passes through, to be
 * refused by the client's parser rather than mapped to a second error shape here.
 */
function eip55Normalized(address: string): string {
  const hexNo0x = remove0x(address);
  const caseless = hexNo0x === hexNo0x.toLowerCase() || hexNo0x === hexNo0x.toUpperCase();
  if (!caseless) {
    return address;
  }
  return toChecksummedAddress(`0x${hexNo0x.toLowerCase()}`) ?? address;
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
 * The gateway `Decryption` contract's EIP-712 domain: the domain the link is hashed under, and the
 * domain a KMS node signed the response's external signature under.
 *
 * Trusted configuration, like the signer set: a domain taken from the response would let the
 * response choose both the link it is held to and the key it is verified against.
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
 * The request fields the response is verified against. Every one of them is the client's own.
 *
 * There is no `link` parameter and no room for one: a caller who could pass a link would be able to
 * pass the one the response carries, which is the substitution the link exists to stop.
 */
export interface SolanaUserDecryptRequestInputs {
  /** The recipient: the permit's 32-byte user address. */
  readonly userAddress: Uint8Array;
  /** The requested handles, in the order the request carried them — position is part of the link. */
  readonly handles: readonly Uint8Array[];
  /** The serialized transport key, in full: the link commits to the key, not to its fingerprint. */
  readonly transportKey: Uint8Array;
  /** The gateway domain the link is hashed under; a different domain is a different link. */
  readonly gatewayEip712Domain: SolanaGatewayEip712Domain;
  /**
   * The request's `extra_data`, verbatim: the KMS routing the permit signed, in its wire form. Each
   * node signature covers it, so it must be the exact bytes the request carried — rebuilt from the
   * signed permit, never copied from a response.
   */
  readonly extraData: Uint8Array;
}

/**
 * Generates a transport keypair for one permit session.
 *
 * A fresh pair per permit: the permit commits to this key's fingerprint, so reusing a key across
 * permits would let one permit's response be de-signcrypted under another's.
 *
 * @param runtime - The client runtime that owns the KMS WASM module.
 */
export async function generateSolanaTransportKeyPair(runtime: FhevmRuntime): Promise<SolanaTransportKeyPair> {
  const kmsLib = await initTkmsModule(runtime);
  const secretKey = kmsLib.ml_kem_pke_keygen();
  const publicKey = kmsLib.ml_kem_pke_get_pk(secretKey);
  return { secretKey, publicKey, publicKeyBytes: kmsLib.ml_kem_pke_pk_to_u8vec(publicKey) };
}

/**
 * Verifies the KMS response and returns the plaintexts, or refuses the whole response.
 *
 * All or nothing: a share that does not authenticate or does not carry the recomputed link is
 * discarded, and if what remains cannot reconstruct, the call fails rather than returning the values
 * it could recover. A partial answer would be indistinguishable from a complete one to the caller.
 *
 * @param response.runtime - The client runtime that owns the KMS WASM module.
 * @param response.request - The client's own request fields.
 * @param response.shares - The signcrypted shares as returned, in any order.
 * @param response.keyPair - The session's transport keypair; the secret key stays here.
 * @param response.signers - The registered KMS signer set, from trusted configuration.
 * @param response.fheParameter - The FHE parameter choice the client was built for.
 * @throws If no set of authenticated shares agrees on the recomputed link and reaches the threshold.
 */
export async function verifySolanaUserDecryptResponse(response: {
  readonly runtime: FhevmRuntime;
  readonly request: SolanaUserDecryptRequestInputs;
  readonly shares: readonly SolanaSigncryptedShare[];
  readonly keyPair: SolanaTransportKeyPair;
  readonly signers: readonly SolanaKmsSigner[];
  readonly fheParameter: string;
}): Promise<readonly SolanaUserDecryptPlaintext[]> {
  // No shares is not an empty answer: it is a response that reaches no threshold, said here in the
  // request's own terms rather than left to a reconstruction error about a missing pivot.
  if (response.shares.length === 0) {
    throw new Error('the response carries no shares, and a response with no shares reaches no threshold');
  }
  const kmsLib = await initTkmsModule(response.runtime);
  const { request } = response;
  const userAddress = getAddressDecoder().decode(request.userAddress);
  const domain = request.gatewayEip712Domain;

  // The trust anchor: the registered signer set, from configuration the caller read on chain. A
  // key carried inside the response acts only under its binding to one of these addresses.
  const client = kmsLib.new_client(
    response.signers.map((signer) => kmsLib.new_server_id_addr(signer.partyId, eip55Normalized(signer.address))),
    userAddress,
    response.fheParameter,
  );
  try {
    const plaintexts = kmsLib.process_user_decryption_resp_from_js(
      client,
      {
        signature: undefined,
        client_address: userAddress,
        enc_key: bytesToHexNo0x(request.transportKey),
        ciphertext_handles: request.handles.map((handle) => bytesToHexNo0x(handle)),
        eip712_verifying_contract: eip55Normalized(domain.verifyingContract),
        extra_data: bytesToHexNo0x(request.extraData),
      },
      gatewayDomainWasmArg(domain),
      response.shares.map((share) => ({
        signature: remove0x(share.signature),
        payload: remove0x(share.payload),
        extra_data: remove0x(share.extraData),
      })),
      response.keyPair.publicKey,
      response.keyPair.secretKey,
      // Left to the client, which derives t from n = 3t + 1, as the EVM decrypt module does.
      undefined,
      true,
    );
    // The client already converts little-endian to big-endian, so `bytes` is the plaintext as-is.
    const typed = plaintexts.map((plaintext) => {
      const value = { bytes: plaintext.bytes, fheTypeId: plaintext.fhe_type };
      plaintext.free();
      return value;
    });
    verifySolanaUserDecryptPlaintexts(typed, request.handles);
    return typed;
  } finally {
    client.free();
  }
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
 * The domain in the client's JS shape: the chain id as 32 big-endian bytes, no salt.
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
    // EIP-55-normalized at this crossing, like the signer set: the client's domain parser refuses a
    // lowercase spelling of a valid contract address as a bad checksum.
    verifying_contract: eip55Normalized(domain.verifyingContract),
    salt: null,
  };
}
