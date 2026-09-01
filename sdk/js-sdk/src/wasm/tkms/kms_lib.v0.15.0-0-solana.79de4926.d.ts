/* tslint:disable */
/* eslint-disable */

export class CiphertextHandle {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

/**
 * Core Client
 *
 * Simple client to interact with the KMS servers. This can be seen as a proof-of-concept
 * and reference code for validating the KMS. The logic supplied by the client will be
 * distributed across the aggregator/proxy and smart contracts.
 */
export class Client {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

/**
 * Eip712 domain information.
 * Any constraints specified in the [standard](<https://eips.ethereum.org/EIPS/eip-712#definition-of-domainseparator>) *must* be fulfilled.
 * Furthermore, be aware that all parameters will eventually be parsed into Solidity types.
 */
export class Eip712DomainMsg {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    chain_id: Uint8Array;
    name: string;
    get salt(): Uint8Array | undefined;
    set salt(value: Uint8Array | null | undefined);
    verifying_contract: string;
    version: string;
}

/**
 * Validity of this struct is not checked.
 */
export class ParsedUserDecryptionRequest {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

export class PrivateEncKeyMlKem512 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

export class PrivateSigKey {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

export class PublicEncKeyMlKem512 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

export class PublicSigKey {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

/**
 * / A unique 32 Byte / 256 Bit ID, to be used to identify a request and
 * / for retrieving the computed result later on.
 * / Must be encoded in lower-case hex. The string must NOT contain a `0x` prefix.
 */
export class RequestId {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    request_id: string;
}

export class ServerIdAddr {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

export class TypedCiphertext {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * The ciphertext format, see CiphertextFormat documentation for details.
     * CiphertextFormat::default() is used if unspecified.
     */
    ciphertext_format: number;
    /**
     * The actual ciphertext to decrypt, taken directly from fhevm.
     */
    ciphertext: Uint8Array;
    /**
     * The external handle of the ciphertext (the handle used in the copro).
     */
    external_handle: Uint8Array;
    /**
     * The type of plaintext encrypted. The type should match FheType from tfhe-rs:
     * <https://github.com/zama-ai/tfhe-rs/blob/main/tfhe/src/high_level_api/mod.rs>
     */
    fhe_type: number;
}

export class TypedPlaintext {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * The actual plaintext in bytes.
     */
    bytes: Uint8Array;
    /**
     * The type of plaintext encrypted. The type should match FheType from tfhe-rs:
     * <https://github.com/zama-ai/tfhe-rs/blob/main/tfhe/src/high_level_api/mod.rs>
     */
    fhe_type: number;
}

/**
 * A single signature together with the scheme that produced it.
 *
 * Used to carry a chain-native signature for one signing scheme when a response
 * may be signed under several schemes at once (e.g. ECDSA/secp256k1 EIP-712 for
 * an EVM, ed25519 for Solana, ML-DSA for post-quantum).
 */
export class TypedSignature {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * The signature scheme that produced `signature`.
     */
    scheme: number;
    /**
     * The raw, scheme-specific signature bytes.
     */
    signature: Uint8Array;
}

export class TypedSigncryptedCiphertext {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * The external handles that were originally in the request.
     */
    external_handle: Uint8Array;
    /**
     * The type of plaintext encrypted. The type should match FheType from tfhe-rs:
     * <https://github.com/zama-ai/tfhe-rs/blob/main/tfhe/src/high_level_api/mod.rs>
     */
    fhe_type: number;
    /**
     * The packing factor determines whether the decrypted plaintext
     * has a different way of packing compared to what is specified in the plaintext modulus.
     */
    packing_factor: number;
    /**
     * The signcrypted payload, using a hybrid encryption approach in
     * sign-then-encrypt.
     */
    signcrypted_ciphertext: Uint8Array;
}

export class UserDecryptionRequest {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * The EVM client's (blockchain wallet) address, encoded using EIP-55, including `0x`.
     * Solana requests MUST leave this empty and carry their identity in `signing_metadata`.
     */
    client_address: string;
    /**
     * MPC context ID which is used to identify the context to use for this request.
     * If unset, the server's default context is used.
     */
    get context_id(): RequestId | undefined;
    /**
     * MPC context ID which is used to identify the context to use for this request.
     * If unset, the server's default context is used.
     */
    set context_id(value: RequestId | null | undefined);
    /**
     * The user's EIP712 domain. This MUST be present. Furthermore, the `verifying_contract` MUST be set and, for EVM requests, be distinct from `client_address`.
     */
    get domain(): Eip712DomainMsg | undefined;
    /**
     * The user's EIP712 domain. This MUST be present. Furthermore, the `verifying_contract` MUST be set and, for EVM requests, be distinct from `client_address`.
     */
    set domain(value: Eip712DomainMsg | null | undefined);
    /**
     * Encoding of the user's public encryption key for this request.
     * This must be a bincode (v.1) encoded ML-KEM 512 key.
     */
    enc_key: Uint8Array;
    /**
     * The MPC epoch ID identifying which epoch's key material and session to
     * use for this request.
     */
    get epoch_id(): RequestId | undefined;
    /**
     * The MPC epoch ID identifying which epoch's key material and session to
     * use for this request.
     */
    set epoch_id(value: RequestId | null | undefined);
    /**
     * Extra data used in the EIP712 signature - external_signature.
     */
    extra_data: Uint8Array;
    /**
     * The 32 Byte / 256 Bit key id to use for decryption. This is the request_id
     * used for key generation
     */
    get key_id(): RequestId | undefined;
    /**
     * The 32 Byte / 256 Bit key id to use for decryption. This is the request_id
     * used for key generation
     */
    set key_id(value: RequestId | null | undefined);
    /**
     * The 32 Byte / 256 Bit ID of the user decryption request, without `0x`
     * prefix. Future queries for the result must use this request ID.
     */
    get request_id(): RequestId | undefined;
    /**
     * The 32 Byte / 256 Bit ID of the user decryption request, without `0x`
     * prefix. Future queries for the result must use this request ID.
     */
    set request_id(value: RequestId | null | undefined);
    /**
     * The signature schemes to include in the response `signatures` field.
     * If empty, the response `signatures` list is empty; legacy ECDSA signatures
     * remain available via the deprecated scalar fields for backward compatibility.
     */
    signing_schemes: Int32Array;
    /**
     * The list of ciphertexts to decrypt for the user.
     */
    typed_ciphertexts: TypedCiphertext[];
}

export class UserDecryptionResponse {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * This is the external ECDSA signature created from the Eip712 domain
     * on the structure, where userDecryptedShare is bc2wrap::serialize(&payload)
     * struct UserDecryptResponseVerification {
     * bytes publicKey;
     * uint256\[\] ctHandles;
     * bytes userDecryptedShare; // serialization of payload
     * bytes extraData;
     * }
     * DEPRECATED: To be removed in 0.16 TODO(0.16)
     */
    external_signature: Uint8Array;
    /**
     * Extra data used in the EIP712 signature - external_signature.
     */
    extra_data: Uint8Array;
    /**
     * The actual \[UserDecryptionResponsePayload\].
     */
    get payload(): UserDecryptionResponsePayload | undefined;
    /**
     * The actual \[UserDecryptionResponsePayload\].
     */
    set payload(value: UserDecryptionResponsePayload | null | undefined);
    /**
     * DEPRECATED to be removed in 0.16.0 TODO(0.16)
     * The KMS-internal ECDSA/secp256k1 authenticity signature over
     * the serialization of \[UserDecryptionResponsePayload\]. Kept at field 1 (and
     * kept populated) for wire backward-compatibility with clients that predate
     * the multi-scheme `signatures` list.
     */
    signature: Uint8Array;
    /**
     * Per-scheme KMS signatures, one per scheme requested in the request's
     * `signing_schemes`.
     */
    signatures: TypedSignature[];
}

export class UserDecryptionResponsePayload {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * The degree of the sharing scheme used.
     */
    degree: number;
    /**
     * This is needed to ensure the response corresponds to the request.
     * It is the digest of UserDecryptionLinker hashed using EIP712
     * under the given domain in the request.
     */
    digest: Uint8Array;
    /**
     * The ID of the MPC party doing the user decryption. Used for polynomial
     * reconstruction.
     */
    party_id: number;
    /**
     * The resulting signcrypted ciphertexts, each ciphertext
     * must be decrypted and then reconstructed with the other shares
     * to produce the final plaintext.
     */
    signcrypted_ciphertexts: TypedSigncryptedCiphertext[];
    /**
     * The server's signature verification key, Encoded using SEC1.
     * Needed to validate the response, but MUST also be linked to a list of
     * trusted keys.
     */
    verification_key: Uint8Array;
}

export function get_client_address(client: Client): string;

export function get_client_secret_key(client: Client): PrivateSigKey | undefined;

export function get_server_addrs(client: Client): ServerIdAddr[];

/**
 * This function is *not* used by relayer-sdk because the decryption
 * is handled by [process_user_decryption_resp].
 * It's just here for completeness and tests.
 */
export function ml_kem_pke_decrypt(ct: Uint8Array, my_sk: PrivateEncKeyMlKem512): Uint8Array;

/**
 * This function is *not* used by relayer-sdk because the encryption
 * happens on the KMS side. It's just here for completeness and tests.
 */
export function ml_kem_pke_encrypt(msg: Uint8Array, their_pk: PublicEncKeyMlKem512): Uint8Array;

export function ml_kem_pke_get_pk(sk: PrivateEncKeyMlKem512): PublicEncKeyMlKem512;

export function ml_kem_pke_keygen(): PrivateEncKeyMlKem512;

export function ml_kem_pke_pk_len(): number;

export function ml_kem_pke_pk_to_u8vec(pk: PublicEncKeyMlKem512): Uint8Array;

export function ml_kem_pke_sk_len(): number;

export function ml_kem_pke_sk_to_u8vec(sk: PrivateEncKeyMlKem512): Uint8Array;

/**
 * Instantiate a new client.
 *
 * * `server_addrs` - a list of KMS server ID with EIP-55 addresses,
 * the elements in the list can be created using [new_server_id_addr].
 *
 * * `client_address_hex` - the client (wallet) address in hex,
 * must be prefixed with "0x".
 *
 * * `fhe_parameter` - the parameter choice, which can be either `"test"` or `"default"`.
 * The "default" parameter choice is selected if no matching string is found.
 */
export function new_client(server_addrs: ServerIdAddr[], client_address_hex: string, fhe_parameter: string): Client;

/**
 * Create a new [ServerIdAddr] structure that holds an ID and an address
 * which must be a valid EIP-55 address, notably prefixed with "0x".
 */
export function new_server_id_addr(id: number, addr: string): ServerIdAddr;

/**
 * Instantiate a new client for the Solana user-decryption path.
 *
 * The same trusted configuration [new_client] takes — the registered KMS signer set and the FHE
 * parameter choice — without the wallet address, which the Solana path does not have: the
 * recipient there is the raw 32-byte ed25519 key carried by every request, so the client's
 * address field plays no part and is fixed to zero.
 *
 * * `server_addrs` - the registered KMS node signer set, as `(party id, EIP-55 address)` pairs
 * built with [new_server_id_addr] — on Solana, the host program's KMS-context signer set, which
 * the caller reads on chain. This is the trust anchor of response verification: keys carried
 * inside a response act only under their binding to one of these addresses, and an empty set
 * authenticates nothing.
 *
 * * `fhe_parameter` - the parameter choice, which can be either `"test"` or `"default"`.
 * The "default" parameter choice is selected if no matching string is found.
 */
export function new_solana_client(server_addrs: ServerIdAddr[], fhe_parameter: string): Client;

export function private_sig_key_to_u8vec(sk: PrivateSigKey): Uint8Array;

/**
 * Process the user_decryption response from JavaScript objects.
 * The returned result is a byte array representing a plaintext of any length,
 * postprocessing is returned to turn it into an integer.
 *
 * * `client` - client that wants to perform user_decryption.
 *
 * * `request` - the initial user_decryption request JS object.
 * It can be set to null if `verify` is false.
 * Otherwise the caller needs to give the following JS object.
 * Note that `client_address` and `eip712_verifying_contract` follow EIP-55.
 * The signature field is not needed.
 * ```
 * {
 *   signature: undefined,
 *   client_address: '0x17853A630aAe15AED549B2B874de08B73C0F59c5',
 *   enc_key: '2000000000000000df2fcacb774f03187f3802a27259f45c06d33cefa68d9c53426b15ad531aa822',
 *   ciphertext_handles: [ '0748b542afe2353c86cb707e3d21044b0be1fd18efc7cbaa6a415af055bfb358' ]
 *   eip712_verifying_contract: '0x66f9664f97F2b50F62D13eA064982f936dE76657'
 * }
 * ```
 *
 * * `eip712_domain` - the EIP-712 domain JS object.
 * It can be set to null if `verify` is false.
 * Otherwise the caller needs to give the following JS object.
 * Note that `salt` is optional and `verifying_contract` follows EIP-55,
 * additionally, `chain_id` is an array of u8.
 * ```
 * {
 *   name: 'Authorization token',
 *   version: '1',
 *   chain_id: [
 *     70, 31, 0, 0, 0, 0, 0, 0, 0,
 *      0,  0, 0, 0, 0, 0, 0, 0, 0,
 *      0,  0, 0, 0, 0, 0, 0, 0, 0,
 *      0,  0, 0, 0, 0
 *   ],
 *   verifying_contract: '0x66f9664f97F2b50F62D13eA064982f936dE76657',
 *   salt: []
 * }
 * ```
 *
 * * `agg_resp` - the response JS object from the gateway.
 * It has two fields like so, both are hex encoded byte arrays.
 * ```
 * [
 *   {
 *     signature: '69e7e040cab157aa819015b321c012dccb1545ffefd325b359b492653f0347517e28e66c572cdc299e259024329859ff9fcb0096e1ce072af0b6e1ca1fe25ec6',
 *     payload: '0100000029...',
 *     extra_data: '01234...',
 *   }
 * ]
 * ```
 *
 * * `enc_pk` - The ephemeral public key.
 *
 * * `enc_sk` - The ephemeral secret key.
 *
 * * `threshold` - Optional expected threshold/degree used during response validation.
 * Validation requires at least `threshold + 1` matching responses, and the selected pivot
 * response must have `degree == threshold`. If not provided, it is computed from the number
 * of server addresses as `(n - 1) / 3`.
 *
 * * `verify` - Whether to perform signature verification for the response.
 * It is insecure if `verify = false`!
 */
export function process_user_decryption_resp_from_js(client: Client, request: any, eip712_domain: any, agg_resp: any, enc_pk: PublicEncKeyMlKem512, enc_sk: PrivateEncKeyMlKem512, threshold: number | null | undefined, verify: boolean): TypedPlaintext[];

/**
 * Solana variant of [process_user_decryption_resp_from_js]. The signed link is the Solana
 * user-decryption binding over the deployment pair (`verifying_program_id`, host chain id), the
 * recipient, the handles, the transport key and the request's `extra_data`, not the EVM EIP-712
 * `UserDecryptionLinker`; de-signcryption is otherwise identical to the EVM path.
 *
 * * `client` - the client built with [new_solana_client] from trusted configuration: the
 * registered KMS signer set — on Solana, the host program's KMS-context signer set, which the
 * caller reads on chain — and the FHE parameter choice a threshold release reconstructs under.
 * The signer set is the trust anchor of the node-signature rule: keys carried inside a response
 * act only under their binding to one of the registered addresses, and a client holding an empty
 * set authenticates nothing. The client's wallet address plays no part on this path — the
 * recipient is the 32-byte ed25519 key below.
 *
 * * `solana_request` - the Solana-owned request fields, as one named object:
 * `{ user_pubkey, host_chain_id, verifying_program_id }`.
 * Identities are 32-byte hex strings; `host_chain_id` is a decimal string, the vector-set
 * convention, because a Solana chain id sets bit 63 and does not fit a JS number.
 *
 * * `eip712_domain` - the EIP-712 domain a KMS node produced the response's `external_signature`
 * under, in the same JS shape [process_user_decryption_resp_from_js] takes it. A wasm response
 * never carries an internal ECDSA signature (see [js_to_resp]), so this is the domain every share
 * is authenticated against. It is a trailing argument, and omitting it is treated as the empty
 * domain — under which no real external signature verifies, so such a response is rejected by the
 * signature rule rather than accepted unchecked.
 */
export function process_user_decryption_resp_solana_from_js(client: Client, request: any, solana_request: any, agg_resp: any, enc_pk: PublicEncKeyMlKem512, enc_sk: PrivateEncKeyMlKem512, eip712_domain: any): TypedPlaintext[];

export function public_sig_key_to_u8vec(pk: PublicSigKey): Uint8Array;

export function u8vec_to_ml_kem_pke_pk(v: Uint8Array): PublicEncKeyMlKem512;

export function u8vec_to_ml_kem_pke_sk(v: Uint8Array): PrivateEncKeyMlKem512;

export function u8vec_to_private_sig_key(v: Uint8Array): PrivateSigKey;

export function u8vec_to_public_sig_key(v: Uint8Array): PublicSigKey;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_ciphertexthandle_free: (a: number, b: number) => void;
    readonly __wbg_parseduserdecryptionrequest_free: (a: number, b: number) => void;
    readonly __wbg_privatesigkey_free: (a: number, b: number) => void;
    readonly __wbg_publicsigkey_free: (a: number, b: number) => void;
    readonly __wbg_privateenckeymlkem512_free: (a: number, b: number) => void;
    readonly __wbg_publicenckeymlkem512_free: (a: number, b: number) => void;
    readonly __wbg_serveridaddr_free: (a: number, b: number) => void;
    readonly get_client_address: (a: number) => [number, number];
    readonly get_client_secret_key: (a: number) => number;
    readonly get_server_addrs: (a: number) => [number, number];
    readonly ml_kem_pke_decrypt: (a: number, b: number, c: number) => [number, number];
    readonly ml_kem_pke_encrypt: (a: number, b: number, c: number) => [number, number];
    readonly ml_kem_pke_get_pk: (a: number) => number;
    readonly ml_kem_pke_keygen: () => number;
    readonly ml_kem_pke_pk_len: () => number;
    readonly ml_kem_pke_pk_to_u8vec: (a: number) => [number, number, number, number];
    readonly ml_kem_pke_sk_len: () => number;
    readonly ml_kem_pke_sk_to_u8vec: (a: number) => [number, number, number, number];
    readonly new_client: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number];
    readonly new_server_id_addr: (a: number, b: number, c: number) => [number, number, number];
    readonly new_solana_client: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly private_sig_key_to_u8vec: (a: number) => [number, number, number, number];
    readonly process_user_decryption_resp_from_js: (a: number, b: any, c: any, d: any, e: number, f: number, g: number, h: number) => [number, number, number, number];
    readonly process_user_decryption_resp_solana_from_js: (a: number, b: any, c: any, d: any, e: number, f: number, g: any) => [number, number, number, number];
    readonly public_sig_key_to_u8vec: (a: number) => [number, number];
    readonly u8vec_to_ml_kem_pke_pk: (a: number, b: number) => [number, number, number];
    readonly u8vec_to_ml_kem_pke_sk: (a: number, b: number) => [number, number, number];
    readonly u8vec_to_private_sig_key: (a: number, b: number) => [number, number, number];
    readonly u8vec_to_public_sig_key: (a: number, b: number) => [number, number, number];
    readonly __wbg_client_free: (a: number, b: number) => void;
    readonly __wbg_eip712domainmsg_free: (a: number, b: number) => void;
    readonly __wbg_get_eip712domainmsg_chain_id: (a: number) => [number, number];
    readonly __wbg_get_eip712domainmsg_name: (a: number) => [number, number];
    readonly __wbg_get_eip712domainmsg_salt: (a: number) => [number, number];
    readonly __wbg_get_eip712domainmsg_verifying_contract: (a: number) => [number, number];
    readonly __wbg_get_eip712domainmsg_version: (a: number) => [number, number];
    readonly __wbg_get_typedciphertext_ciphertext: (a: number) => [number, number];
    readonly __wbg_get_typedciphertext_ciphertext_format: (a: number) => number;
    readonly __wbg_get_typedciphertext_external_handle: (a: number) => [number, number];
    readonly __wbg_get_typedciphertext_fhe_type: (a: number) => number;
    readonly __wbg_get_typedplaintext_fhe_type: (a: number) => number;
    readonly __wbg_get_userdecryptionrequest_context_id: (a: number) => number;
    readonly __wbg_get_userdecryptionrequest_domain: (a: number) => number;
    readonly __wbg_get_userdecryptionrequest_epoch_id: (a: number) => number;
    readonly __wbg_get_userdecryptionrequest_extra_data: (a: number) => [number, number];
    readonly __wbg_get_userdecryptionrequest_key_id: (a: number) => number;
    readonly __wbg_get_userdecryptionrequest_request_id: (a: number) => number;
    readonly __wbg_get_userdecryptionrequest_signing_schemes: (a: number) => [number, number];
    readonly __wbg_get_userdecryptionrequest_typed_ciphertexts: (a: number) => [number, number];
    readonly __wbg_get_userdecryptionresponse_payload: (a: number) => number;
    readonly __wbg_get_userdecryptionresponse_signatures: (a: number) => [number, number];
    readonly __wbg_get_userdecryptionresponsepayload_degree: (a: number) => number;
    readonly __wbg_get_userdecryptionresponsepayload_party_id: (a: number) => number;
    readonly __wbg_get_userdecryptionresponsepayload_signcrypted_ciphertexts: (a: number) => [number, number];
    readonly __wbg_requestid_free: (a: number, b: number) => void;
    readonly __wbg_set_eip712domainmsg_chain_id: (a: number, b: number, c: number) => void;
    readonly __wbg_set_eip712domainmsg_name: (a: number, b: number, c: number) => void;
    readonly __wbg_set_eip712domainmsg_salt: (a: number, b: number, c: number) => void;
    readonly __wbg_set_eip712domainmsg_verifying_contract: (a: number, b: number, c: number) => void;
    readonly __wbg_set_eip712domainmsg_version: (a: number, b: number, c: number) => void;
    readonly __wbg_set_typedciphertext_ciphertext_format: (a: number, b: number) => void;
    readonly __wbg_set_typedciphertext_fhe_type: (a: number, b: number) => void;
    readonly __wbg_set_typedplaintext_fhe_type: (a: number, b: number) => void;
    readonly __wbg_set_userdecryptionrequest_context_id: (a: number, b: number) => void;
    readonly __wbg_set_userdecryptionrequest_domain: (a: number, b: number) => void;
    readonly __wbg_set_userdecryptionrequest_epoch_id: (a: number, b: number) => void;
    readonly __wbg_set_userdecryptionrequest_key_id: (a: number, b: number) => void;
    readonly __wbg_set_userdecryptionrequest_request_id: (a: number, b: number) => void;
    readonly __wbg_set_userdecryptionrequest_signing_schemes: (a: number, b: number, c: number) => void;
    readonly __wbg_set_userdecryptionrequest_typed_ciphertexts: (a: number, b: number, c: number) => void;
    readonly __wbg_set_userdecryptionresponse_payload: (a: number, b: number) => void;
    readonly __wbg_set_userdecryptionresponse_signatures: (a: number, b: number, c: number) => void;
    readonly __wbg_set_userdecryptionresponsepayload_degree: (a: number, b: number) => void;
    readonly __wbg_set_userdecryptionresponsepayload_party_id: (a: number, b: number) => void;
    readonly __wbg_set_userdecryptionresponsepayload_signcrypted_ciphertexts: (a: number, b: number, c: number) => void;
    readonly __wbg_typedciphertext_free: (a: number, b: number) => void;
    readonly __wbg_typedplaintext_free: (a: number, b: number) => void;
    readonly __wbg_typedsignature_free: (a: number, b: number) => void;
    readonly __wbg_typedsigncryptedciphertext_free: (a: number, b: number) => void;
    readonly __wbg_userdecryptionrequest_free: (a: number, b: number) => void;
    readonly __wbg_userdecryptionresponse_free: (a: number, b: number) => void;
    readonly __wbg_userdecryptionresponsepayload_free: (a: number, b: number) => void;
    readonly __wbg_get_requestid_request_id: (a: number) => [number, number];
    readonly __wbg_get_userdecryptionrequest_client_address: (a: number) => [number, number];
    readonly __wbg_get_typedsignature_scheme: (a: number) => number;
    readonly __wbg_get_typedsigncryptedciphertext_fhe_type: (a: number) => number;
    readonly __wbg_get_typedsigncryptedciphertext_packing_factor: (a: number) => number;
    readonly __wbg_set_typedsignature_scheme: (a: number, b: number) => void;
    readonly __wbg_set_typedsigncryptedciphertext_fhe_type: (a: number, b: number) => void;
    readonly __wbg_set_typedsigncryptedciphertext_packing_factor: (a: number, b: number) => void;
    readonly __wbg_set_requestid_request_id: (a: number, b: number, c: number) => void;
    readonly __wbg_set_typedciphertext_ciphertext: (a: number, b: number, c: number) => void;
    readonly __wbg_set_typedciphertext_external_handle: (a: number, b: number, c: number) => void;
    readonly __wbg_set_typedplaintext_bytes: (a: number, b: number, c: number) => void;
    readonly __wbg_set_typedsignature_signature: (a: number, b: number, c: number) => void;
    readonly __wbg_set_typedsigncryptedciphertext_external_handle: (a: number, b: number, c: number) => void;
    readonly __wbg_set_typedsigncryptedciphertext_signcrypted_ciphertext: (a: number, b: number, c: number) => void;
    readonly __wbg_set_userdecryptionrequest_client_address: (a: number, b: number, c: number) => void;
    readonly __wbg_set_userdecryptionrequest_enc_key: (a: number, b: number, c: number) => void;
    readonly __wbg_set_userdecryptionrequest_extra_data: (a: number, b: number, c: number) => void;
    readonly __wbg_set_userdecryptionresponse_external_signature: (a: number, b: number, c: number) => void;
    readonly __wbg_set_userdecryptionresponse_extra_data: (a: number, b: number, c: number) => void;
    readonly __wbg_set_userdecryptionresponse_signature: (a: number, b: number, c: number) => void;
    readonly __wbg_set_userdecryptionresponsepayload_digest: (a: number, b: number, c: number) => void;
    readonly __wbg_set_userdecryptionresponsepayload_verification_key: (a: number, b: number, c: number) => void;
    readonly __wbg_get_typedplaintext_bytes: (a: number) => [number, number];
    readonly __wbg_get_typedsignature_signature: (a: number) => [number, number];
    readonly __wbg_get_typedsigncryptedciphertext_external_handle: (a: number) => [number, number];
    readonly __wbg_get_typedsigncryptedciphertext_signcrypted_ciphertext: (a: number) => [number, number];
    readonly __wbg_get_userdecryptionrequest_enc_key: (a: number) => [number, number];
    readonly __wbg_get_userdecryptionresponse_external_signature: (a: number) => [number, number];
    readonly __wbg_get_userdecryptionresponse_extra_data: (a: number) => [number, number];
    readonly __wbg_get_userdecryptionresponse_signature: (a: number) => [number, number];
    readonly __wbg_get_userdecryptionresponsepayload_digest: (a: number) => [number, number];
    readonly __wbg_get_userdecryptionresponsepayload_verification_key: (a: number) => [number, number];
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_drop_slice: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
