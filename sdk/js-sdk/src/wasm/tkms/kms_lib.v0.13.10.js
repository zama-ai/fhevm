/* @ts-self-types="./kms_lib.v0.13.10.d.ts" */

export class CiphertextHandle {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CiphertextHandleFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ciphertexthandle_free(ptr, 0);
    }
}
if (Symbol.dispose) CiphertextHandle.prototype[Symbol.dispose] = CiphertextHandle.prototype.free;

/**
 * Core Client
 *
 * Simple client to interact with the KMS servers. This can be seen as a proof-of-concept
 * and reference code for validating the KMS. The logic supplied by the client will be
 * distributed across the aggregator/proxy and smart contracts.
 */
export class Client {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Client.prototype);
        obj.__wbg_ptr = ptr;
        ClientFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ClientFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_client_free(ptr, 0);
    }
}
if (Symbol.dispose) Client.prototype[Symbol.dispose] = Client.prototype.free;

/**
 * Eip712 domain information.
 * Any constraints specified in the [standard](<https://eips.ethereum.org/EIPS/eip-712#definition-of-domainseparator>) _must_ be fulfilled.
 * Furthermore, be aware that all parameters will eventually be parsed into Solidity types.
 */
export class Eip712DomainMsg {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Eip712DomainMsg.prototype);
        obj.__wbg_ptr = ptr;
        Eip712DomainMsgFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Eip712DomainMsgFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_eip712domainmsg_free(ptr, 0);
    }
    /**
     * @returns {Uint8Array}
     */
    get chain_id() {
        const ret = wasm.__wbg_get_eip712domainmsg_chain_id(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_eip712domainmsg_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {Uint8Array | undefined}
     */
    get salt() {
        const ret = wasm.__wbg_get_eip712domainmsg_salt(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {string}
     */
    get verifying_contract() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_eip712domainmsg_verifying_contract(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    get version() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_eip712domainmsg_version(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {Uint8Array} arg0
     */
    set chain_id(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_chain_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {Uint8Array | null} [arg0]
     */
    set salt(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_salt(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {string} arg0
     */
    set verifying_contract(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_verifying_contract(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {string} arg0
     */
    set version(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_version(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) Eip712DomainMsg.prototype[Symbol.dispose] = Eip712DomainMsg.prototype.free;

/**
 * Validity of this struct is not checked.
 */
export class ParsedUserDecryptionRequest {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ParsedUserDecryptionRequestFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_parseduserdecryptionrequest_free(ptr, 0);
    }
}
if (Symbol.dispose) ParsedUserDecryptionRequest.prototype[Symbol.dispose] = ParsedUserDecryptionRequest.prototype.free;

export class PrivateEncKeyMlKem512 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PrivateEncKeyMlKem512.prototype);
        obj.__wbg_ptr = ptr;
        PrivateEncKeyMlKem512Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PrivateEncKeyMlKem512Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_privateenckeymlkem512_free(ptr, 0);
    }
}
if (Symbol.dispose) PrivateEncKeyMlKem512.prototype[Symbol.dispose] = PrivateEncKeyMlKem512.prototype.free;

export class PrivateSigKey {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PrivateSigKey.prototype);
        obj.__wbg_ptr = ptr;
        PrivateSigKeyFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PrivateSigKeyFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_privatesigkey_free(ptr, 0);
    }
}
if (Symbol.dispose) PrivateSigKey.prototype[Symbol.dispose] = PrivateSigKey.prototype.free;

export class PublicEncKeyMlKem512 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PublicEncKeyMlKem512.prototype);
        obj.__wbg_ptr = ptr;
        PublicEncKeyMlKem512Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PublicEncKeyMlKem512Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_publicenckeymlkem512_free(ptr, 0);
    }
}
if (Symbol.dispose) PublicEncKeyMlKem512.prototype[Symbol.dispose] = PublicEncKeyMlKem512.prototype.free;

export class PublicSigKey {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PublicSigKey.prototype);
        obj.__wbg_ptr = ptr;
        PublicSigKeyFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PublicSigKeyFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_publicsigkey_free(ptr, 0);
    }
}
if (Symbol.dispose) PublicSigKey.prototype[Symbol.dispose] = PublicSigKey.prototype.free;

/**
 * / A unique 32 Byte / 256 Bit ID, to be used to identify a request and
 * / for retrieving the computed result later on.
 * / Must be encoded in lower-case hex. The string must NOT contain a `0x` prefix.
 */
export class RequestId {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(RequestId.prototype);
        obj.__wbg_ptr = ptr;
        RequestIdFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RequestIdFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_requestid_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get request_id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_requestid_request_id(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set request_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_name(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) RequestId.prototype[Symbol.dispose] = RequestId.prototype.free;

export class ServerIdAddr {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ServerIdAddr.prototype);
        obj.__wbg_ptr = ptr;
        ServerIdAddrFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    static __unwrap(jsValue) {
        if (!(jsValue instanceof ServerIdAddr)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ServerIdAddrFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_serveridaddr_free(ptr, 0);
    }
}
if (Symbol.dispose) ServerIdAddr.prototype[Symbol.dispose] = ServerIdAddr.prototype.free;

export class TypedCiphertext {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(TypedCiphertext.prototype);
        obj.__wbg_ptr = ptr;
        TypedCiphertextFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    static __unwrap(jsValue) {
        if (!(jsValue instanceof TypedCiphertext)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TypedCiphertextFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_typedciphertext_free(ptr, 0);
    }
    /**
     * The ciphertext format, see CiphertextFormat documentation for details.
     * CiphertextFormat::default() is used if unspecified.
     * @returns {number}
     */
    get ciphertext_format() {
        const ret = wasm.__wbg_get_typedciphertext_ciphertext_format(this.__wbg_ptr);
        return ret;
    }
    /**
     * The actual ciphertext to decrypt, taken directly from fhevm.
     * @returns {Uint8Array}
     */
    get ciphertext() {
        const ret = wasm.__wbg_get_typedciphertext_ciphertext(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * The external handle of the ciphertext (the handle used in the copro).
     * @returns {Uint8Array}
     */
    get external_handle() {
        const ret = wasm.__wbg_get_typedciphertext_external_handle(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * The type of plaintext encrypted. The type should match FheType from tfhe-rs:
     * <https://github.com/zama-ai/tfhe-rs/blob/main/tfhe/src/high_level_api/mod.rs>
     * @returns {number}
     */
    get fhe_type() {
        const ret = wasm.__wbg_get_typedciphertext_fhe_type(this.__wbg_ptr);
        return ret;
    }
    /**
     * The ciphertext format, see CiphertextFormat documentation for details.
     * CiphertextFormat::default() is used if unspecified.
     * @param {number} arg0
     */
    set ciphertext_format(arg0) {
        wasm.__wbg_set_typedciphertext_ciphertext_format(this.__wbg_ptr, arg0);
    }
    /**
     * The actual ciphertext to decrypt, taken directly from fhevm.
     * @param {Uint8Array} arg0
     */
    set ciphertext(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * The external handle of the ciphertext (the handle used in the copro).
     * @param {Uint8Array} arg0
     */
    set external_handle(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_version(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * The type of plaintext encrypted. The type should match FheType from tfhe-rs:
     * <https://github.com/zama-ai/tfhe-rs/blob/main/tfhe/src/high_level_api/mod.rs>
     * @param {number} arg0
     */
    set fhe_type(arg0) {
        wasm.__wbg_set_typedciphertext_fhe_type(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) TypedCiphertext.prototype[Symbol.dispose] = TypedCiphertext.prototype.free;

export class TypedPlaintext {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(TypedPlaintext.prototype);
        obj.__wbg_ptr = ptr;
        TypedPlaintextFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TypedPlaintextFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_typedplaintext_free(ptr, 0);
    }
    /**
     * The actual plaintext in bytes.
     * @returns {Uint8Array}
     */
    get bytes() {
        const ret = wasm.__wbg_get_typedplaintext_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * The type of plaintext encrypted. The type should match FheType from tfhe-rs:
     * <https://github.com/zama-ai/tfhe-rs/blob/main/tfhe/src/high_level_api/mod.rs>
     * @returns {number}
     */
    get fhe_type() {
        const ret = wasm.__wbg_get_typedplaintext_fhe_type(this.__wbg_ptr);
        return ret;
    }
    /**
     * The actual plaintext in bytes.
     * @param {Uint8Array} arg0
     */
    set bytes(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * The type of plaintext encrypted. The type should match FheType from tfhe-rs:
     * <https://github.com/zama-ai/tfhe-rs/blob/main/tfhe/src/high_level_api/mod.rs>
     * @param {number} arg0
     */
    set fhe_type(arg0) {
        wasm.__wbg_set_typedplaintext_fhe_type(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) TypedPlaintext.prototype[Symbol.dispose] = TypedPlaintext.prototype.free;

export class TypedSigncryptedCiphertext {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(TypedSigncryptedCiphertext.prototype);
        obj.__wbg_ptr = ptr;
        TypedSigncryptedCiphertextFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    static __unwrap(jsValue) {
        if (!(jsValue instanceof TypedSigncryptedCiphertext)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TypedSigncryptedCiphertextFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_typedsigncryptedciphertext_free(ptr, 0);
    }
    /**
     * The external handles that were originally in the request.
     * @returns {Uint8Array}
     */
    get external_handle() {
        const ret = wasm.__wbg_get_typedsigncryptedciphertext_external_handle(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * The type of plaintext encrypted. The type should match FheType from tfhe-rs:
     * <https://github.com/zama-ai/tfhe-rs/blob/main/tfhe/src/high_level_api/mod.rs>
     * @returns {number}
     */
    get fhe_type() {
        const ret = wasm.__wbg_get_typedciphertext_fhe_type(this.__wbg_ptr);
        return ret;
    }
    /**
     * The packing factor determines whether the decrypted plaintext
     * has a different way of packing compared to what is specified in the plaintext modulus.
     * @returns {number}
     */
    get packing_factor() {
        const ret = wasm.__wbg_get_typedciphertext_ciphertext_format(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * The signcrypted payload, using a hybrid encryption approach in
     * sign-then-encrypt.
     * @returns {Uint8Array}
     */
    get signcrypted_ciphertext() {
        const ret = wasm.__wbg_get_typedsigncryptedciphertext_signcrypted_ciphertext(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * The external handles that were originally in the request.
     * @param {Uint8Array} arg0
     */
    set external_handle(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_version(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * The type of plaintext encrypted. The type should match FheType from tfhe-rs:
     * <https://github.com/zama-ai/tfhe-rs/blob/main/tfhe/src/high_level_api/mod.rs>
     * @param {number} arg0
     */
    set fhe_type(arg0) {
        wasm.__wbg_set_typedciphertext_fhe_type(this.__wbg_ptr, arg0);
    }
    /**
     * The packing factor determines whether the decrypted plaintext
     * has a different way of packing compared to what is specified in the plaintext modulus.
     * @param {number} arg0
     */
    set packing_factor(arg0) {
        wasm.__wbg_set_typedciphertext_ciphertext_format(this.__wbg_ptr, arg0);
    }
    /**
     * The signcrypted payload, using a hybrid encryption approach in
     * sign-then-encrypt.
     * @param {Uint8Array} arg0
     */
    set signcrypted_ciphertext(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_name(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) TypedSigncryptedCiphertext.prototype[Symbol.dispose] = TypedSigncryptedCiphertext.prototype.free;

export class UserDecryptionRequest {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        UserDecryptionRequestFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_userdecryptionrequest_free(ptr, 0);
    }
    /**
     * The client's (blockchain wallet) address, encoded using EIP-55. I.e. including `0x`.
     * @returns {string}
     */
    get client_address() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_userdecryptionrequest_client_address(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * MPC context ID which is used to identify the context to use for this request.
     *
     * NOTE: at the moment this can be None since we do not fully support multiple contexts.
     * See <https://github.com/zama-ai/kms-internal/issues/2530>
     * @returns {RequestId | undefined}
     */
    get context_id() {
        const ret = wasm.__wbg_get_userdecryptionrequest_context_id(this.__wbg_ptr);
        return ret === 0 ? undefined : RequestId.__wrap(ret);
    }
    /**
     * The user's EIP712 domain. This MUST be present. Furthermore, the `verifying_contract` MUST be set and be distinct from `client_address`.
     * @returns {Eip712DomainMsg | undefined}
     */
    get domain() {
        const ret = wasm.__wbg_get_userdecryptionrequest_domain(this.__wbg_ptr);
        return ret === 0 ? undefined : Eip712DomainMsg.__wrap(ret);
    }
    /**
     * Encoding of the user's public encryption key for this request.
     * This must be a bincode (v.1) encoded ML-KEM 512 key.
     * @returns {Uint8Array}
     */
    get enc_key() {
        const ret = wasm.__wbg_get_userdecryptionrequest_enc_key(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * The epoch number placeholder (zama-ai/kms-internal#2743).
     * @returns {RequestId | undefined}
     */
    get epoch_id() {
        const ret = wasm.__wbg_get_userdecryptionrequest_epoch_id(this.__wbg_ptr);
        return ret === 0 ? undefined : RequestId.__wrap(ret);
    }
    /**
     * Extra data from the gateway.
     * @returns {Uint8Array}
     */
    get extra_data() {
        const ret = wasm.__wbg_get_userdecryptionrequest_extra_data(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * The 32 Byte / 256 Bit key id to use for decryption. This is the request_id
     * used for key generation
     * @returns {RequestId | undefined}
     */
    get key_id() {
        const ret = wasm.__wbg_get_userdecryptionrequest_key_id(this.__wbg_ptr);
        return ret === 0 ? undefined : RequestId.__wrap(ret);
    }
    /**
     * The 32 Byte / 256 Bit ID of the user decryption request, without `0x`
     * prefix. Future queries for the result must use this request ID.
     * @returns {RequestId | undefined}
     */
    get request_id() {
        const ret = wasm.__wbg_get_userdecryptionrequest_request_id(this.__wbg_ptr);
        return ret === 0 ? undefined : RequestId.__wrap(ret);
    }
    /**
     * The list of ciphertexts to decrypt for the user.
     * @returns {TypedCiphertext[]}
     */
    get typed_ciphertexts() {
        const ret = wasm.__wbg_get_userdecryptionrequest_typed_ciphertexts(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * The client's (blockchain wallet) address, encoded using EIP-55. I.e. including `0x`.
     * @param {string} arg0
     */
    set client_address(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_version(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * MPC context ID which is used to identify the context to use for this request.
     *
     * NOTE: at the moment this can be None since we do not fully support multiple contexts.
     * See <https://github.com/zama-ai/kms-internal/issues/2530>
     * @param {RequestId | null} [arg0]
     */
    set context_id(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, RequestId);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_userdecryptionrequest_context_id(this.__wbg_ptr, ptr0);
    }
    /**
     * The user's EIP712 domain. This MUST be present. Furthermore, the `verifying_contract` MUST be set and be distinct from `client_address`.
     * @param {Eip712DomainMsg | null} [arg0]
     */
    set domain(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, Eip712DomainMsg);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_userdecryptionrequest_domain(this.__wbg_ptr, ptr0);
    }
    /**
     * Encoding of the user's public encryption key for this request.
     * This must be a bincode (v.1) encoded ML-KEM 512 key.
     * @param {Uint8Array} arg0
     */
    set enc_key(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_chain_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * The epoch number placeholder (zama-ai/kms-internal#2743).
     * @param {RequestId | null} [arg0]
     */
    set epoch_id(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, RequestId);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_userdecryptionrequest_epoch_id(this.__wbg_ptr, ptr0);
    }
    /**
     * Extra data from the gateway.
     * @param {Uint8Array} arg0
     */
    set extra_data(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_verifying_contract(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * The 32 Byte / 256 Bit key id to use for decryption. This is the request_id
     * used for key generation
     * @param {RequestId | null} [arg0]
     */
    set key_id(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, RequestId);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_userdecryptionrequest_key_id(this.__wbg_ptr, ptr0);
    }
    /**
     * The 32 Byte / 256 Bit ID of the user decryption request, without `0x`
     * prefix. Future queries for the result must use this request ID.
     * @param {RequestId | null} [arg0]
     */
    set request_id(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, RequestId);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_userdecryptionrequest_request_id(this.__wbg_ptr, ptr0);
    }
    /**
     * The list of ciphertexts to decrypt for the user.
     * @param {TypedCiphertext[]} arg0
     */
    set typed_ciphertexts(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_userdecryptionrequest_typed_ciphertexts(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) UserDecryptionRequest.prototype[Symbol.dispose] = UserDecryptionRequest.prototype.free;

export class UserDecryptionResponse {
    static __unwrap(jsValue) {
        if (!(jsValue instanceof UserDecryptionResponse)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        UserDecryptionResponseFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_userdecryptionresponse_free(ptr, 0);
    }
    /**
     * This is the external signature created from the Eip712 domain
     * on the structure, where userDecryptedShare is bc2wrap::serialize(&payload)
     * struct UserDecryptResponseVerification {
     *      bytes publicKey;
     *      uint256\[\] ctHandles;
     *      bytes userDecryptedShare; // serialization of payload
     *      bytes extraData;
     * }
     * @returns {Uint8Array}
     */
    get external_signature() {
        const ret = wasm.__wbg_get_userdecryptionresponse_external_signature(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * Extra data used in the EIP712 signature - external_signature.
     * @returns {Uint8Array}
     */
    get extra_data() {
        const ret = wasm.__wbg_get_userdecryptionresponse_extra_data(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * The actual \[UserDecryptionResponsePayload\].
     * @returns {UserDecryptionResponsePayload | undefined}
     */
    get payload() {
        const ret = wasm.__wbg_get_userdecryptionresponse_payload(this.__wbg_ptr);
        return ret === 0 ? undefined : UserDecryptionResponsePayload.__wrap(ret);
    }
    /**
     * @returns {Uint8Array}
     */
    get signature() {
        const ret = wasm.__wbg_get_userdecryptionresponse_signature(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * This is the external signature created from the Eip712 domain
     * on the structure, where userDecryptedShare is bc2wrap::serialize(&payload)
     * struct UserDecryptResponseVerification {
     *      bytes publicKey;
     *      uint256\[\] ctHandles;
     *      bytes userDecryptedShare; // serialization of payload
     *      bytes extraData;
     * }
     * @param {Uint8Array} arg0
     */
    set external_signature(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_version(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * Extra data used in the EIP712 signature - external_signature.
     * @param {Uint8Array} arg0
     */
    set extra_data(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_chain_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * The actual \[UserDecryptionResponsePayload\].
     * @param {UserDecryptionResponsePayload | null} [arg0]
     */
    set payload(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, UserDecryptionResponsePayload);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_userdecryptionresponse_payload(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {Uint8Array} arg0
     */
    set signature(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_name(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) UserDecryptionResponse.prototype[Symbol.dispose] = UserDecryptionResponse.prototype.free;

export class UserDecryptionResponsePayload {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(UserDecryptionResponsePayload.prototype);
        obj.__wbg_ptr = ptr;
        UserDecryptionResponsePayloadFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        UserDecryptionResponsePayloadFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_userdecryptionresponsepayload_free(ptr, 0);
    }
    /**
     * The degree of the sharing scheme used.
     * @returns {number}
     */
    get degree() {
        const ret = wasm.__wbg_get_userdecryptionresponsepayload_degree(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * This is needed to ensure the response corresponds to the request.
     * It is the digest of UserDecryptionLinker hashed using EIP712
     * under the given domain in the request.
     * @returns {Uint8Array}
     */
    get digest() {
        const ret = wasm.__wbg_get_userdecryptionresponsepayload_digest(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * The ID of the MPC party doing the user decryption. Used for polynomial
     * reconstruction.
     * @returns {number}
     */
    get party_id() {
        const ret = wasm.__wbg_get_userdecryptionresponsepayload_party_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * The resulting signcrypted ciphertexts, each ciphertext
     * must be decrypted and then reconstructed with the other shares
     * to produce the final plaintext.
     * @returns {TypedSigncryptedCiphertext[]}
     */
    get signcrypted_ciphertexts() {
        const ret = wasm.__wbg_get_userdecryptionresponsepayload_signcrypted_ciphertexts(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * The server's signature verification key, Encoded using SEC1.
     * Needed to validate the response, but MUST also be linked to a list of
     * trusted keys.
     * @returns {Uint8Array}
     */
    get verification_key() {
        const ret = wasm.__wbg_get_userdecryptionresponsepayload_verification_key(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * The degree of the sharing scheme used.
     * @param {number} arg0
     */
    set degree(arg0) {
        wasm.__wbg_set_userdecryptionresponsepayload_degree(this.__wbg_ptr, arg0);
    }
    /**
     * This is needed to ensure the response corresponds to the request.
     * It is the digest of UserDecryptionLinker hashed using EIP712
     * under the given domain in the request.
     * @param {Uint8Array} arg0
     */
    set digest(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_version(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * The ID of the MPC party doing the user decryption. Used for polynomial
     * reconstruction.
     * @param {number} arg0
     */
    set party_id(arg0) {
        wasm.__wbg_set_userdecryptionresponsepayload_party_id(this.__wbg_ptr, arg0);
    }
    /**
     * The resulting signcrypted ciphertexts, each ciphertext
     * must be decrypted and then reconstructed with the other shares
     * to produce the final plaintext.
     * @param {TypedSigncryptedCiphertext[]} arg0
     */
    set signcrypted_ciphertexts(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_userdecryptionresponsepayload_signcrypted_ciphertexts(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * The server's signature verification key, Encoded using SEC1.
     * Needed to validate the response, but MUST also be linked to a list of
     * trusted keys.
     * @param {Uint8Array} arg0
     */
    set verification_key(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_eip712domainmsg_name(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) UserDecryptionResponsePayload.prototype[Symbol.dispose] = UserDecryptionResponsePayload.prototype.free;

/**
 * @param {Client} client
 * @returns {string}
 */
export function get_client_address(client) {
    let deferred1_0;
    let deferred1_1;
    try {
        _assertClass(client, Client);
        const ret = wasm.get_client_address(client.__wbg_ptr);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {Client} client
 * @returns {PrivateSigKey | undefined}
 */
export function get_client_secret_key(client) {
    _assertClass(client, Client);
    const ret = wasm.get_client_secret_key(client.__wbg_ptr);
    return ret === 0 ? undefined : PrivateSigKey.__wrap(ret);
}

/**
 * @param {Client} client
 * @returns {ServerIdAddr[]}
 */
export function get_server_addrs(client) {
    _assertClass(client, Client);
    const ret = wasm.get_server_addrs(client.__wbg_ptr);
    var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}

/**
 * This function is *not* used by relayer-sdk because the decryption
 * is handled by [process_user_decryption_resp].
 * It's just here for completeness and tests.
 * @param {Uint8Array} ct
 * @param {PrivateEncKeyMlKem512} my_sk
 * @returns {Uint8Array}
 */
export function ml_kem_pke_decrypt(ct, my_sk) {
    const ptr0 = passArray8ToWasm0(ct, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(my_sk, PrivateEncKeyMlKem512);
    const ret = wasm.ml_kem_pke_decrypt(ptr0, len0, my_sk.__wbg_ptr);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * This function is *not* used by relayer-sdk because the encryption
 * happens on the KMS side. It's just here for completeness and tests.
 * @param {Uint8Array} msg
 * @param {PublicEncKeyMlKem512} their_pk
 * @returns {Uint8Array}
 */
export function ml_kem_pke_encrypt(msg, their_pk) {
    const ptr0 = passArray8ToWasm0(msg, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(their_pk, PublicEncKeyMlKem512);
    const ret = wasm.ml_kem_pke_encrypt(ptr0, len0, their_pk.__wbg_ptr);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {PrivateEncKeyMlKem512} sk
 * @returns {PublicEncKeyMlKem512}
 */
export function ml_kem_pke_get_pk(sk) {
    _assertClass(sk, PrivateEncKeyMlKem512);
    const ret = wasm.ml_kem_pke_get_pk(sk.__wbg_ptr);
    return PublicEncKeyMlKem512.__wrap(ret);
}

/**
 * @returns {PrivateEncKeyMlKem512}
 */
export function ml_kem_pke_keygen() {
    const ret = wasm.ml_kem_pke_keygen();
    return PrivateEncKeyMlKem512.__wrap(ret);
}

/**
 * @returns {number}
 */
export function ml_kem_pke_pk_len() {
    const ret = wasm.ml_kem_pke_pk_len();
    return ret >>> 0;
}

/**
 * @param {PublicEncKeyMlKem512} pk
 * @returns {Uint8Array}
 */
export function ml_kem_pke_pk_to_u8vec(pk) {
    _assertClass(pk, PublicEncKeyMlKem512);
    const ret = wasm.ml_kem_pke_pk_to_u8vec(pk.__wbg_ptr);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @returns {number}
 */
export function ml_kem_pke_sk_len() {
    const ret = wasm.ml_kem_pke_sk_len();
    return ret >>> 0;
}

/**
 * @param {PrivateEncKeyMlKem512} sk
 * @returns {Uint8Array}
 */
export function ml_kem_pke_sk_to_u8vec(sk) {
    _assertClass(sk, PrivateEncKeyMlKem512);
    const ret = wasm.ml_kem_pke_sk_to_u8vec(sk.__wbg_ptr);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

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
 * @param {ServerIdAddr[]} server_addrs
 * @param {string} client_address_hex
 * @param {string} fhe_parameter
 * @returns {Client}
 */
export function new_client(server_addrs, client_address_hex, fhe_parameter) {
    const ptr0 = passArrayJsValueToWasm0(server_addrs, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(client_address_hex, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(fhe_parameter, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.new_client(ptr0, len0, ptr1, len1, ptr2, len2);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return Client.__wrap(ret[0]);
}

/**
 * Create a new [ServerIdAddr] structure that holds an ID and an address
 * which must be a valid EIP-55 address, notably prefixed with "0x".
 * @param {number} id
 * @param {string} addr
 * @returns {ServerIdAddr}
 */
export function new_server_id_addr(id, addr) {
    const ptr0 = passStringToWasm0(addr, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.new_server_id_addr(id, ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ServerIdAddr.__wrap(ret[0]);
}

/**
 * @param {PrivateSigKey} sk
 * @returns {Uint8Array}
 */
export function private_sig_key_to_u8vec(sk) {
    _assertClass(sk, PrivateSigKey);
    const ret = wasm.private_sig_key_to_u8vec(sk.__wbg_ptr);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * Process the user_decryption response from Rust objects.
 * Consider using [process_user_decryption_resp_from_js]
 * when using the JS API.
 * The result is a byte array representing a plaintext of any length.
 *
 * * `client` - client that wants to perform user_decryption.
 *
 * * `request` - the initial user_decryption request.
 * Must be given if `verify` is true.
 *
 * * `eip712_domain` - the EIP-712 domain.
 * Must be given if `verify` is true.
 *
 * * `agg_resp` - the vector of user_decryption responses.
 *
 * * `enc_pk` - The ephemeral public key.
 *
 * * `enc_sk` - The ephemeral secret key.
 *
 * * `verify` - Whether to perform signature verification for the response.
 * It is insecure if `verify = false`!
 * @param {Client} client
 * @param {ParsedUserDecryptionRequest | null | undefined} request
 * @param {Eip712DomainMsg | null | undefined} eip712_domain
 * @param {UserDecryptionResponse[]} agg_resp
 * @param {PublicEncKeyMlKem512} enc_pk
 * @param {PrivateEncKeyMlKem512} enc_sk
 * @param {boolean} verify
 * @returns {TypedPlaintext[]}
 */
export function process_user_decryption_resp(client, request, eip712_domain, agg_resp, enc_pk, enc_sk, verify) {
    _assertClass(client, Client);
    let ptr0 = 0;
    if (!isLikeNone(request)) {
        _assertClass(request, ParsedUserDecryptionRequest);
        ptr0 = request.__destroy_into_raw();
    }
    let ptr1 = 0;
    if (!isLikeNone(eip712_domain)) {
        _assertClass(eip712_domain, Eip712DomainMsg);
        ptr1 = eip712_domain.__destroy_into_raw();
    }
    const ptr2 = passArrayJsValueToWasm0(agg_resp, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    _assertClass(enc_pk, PublicEncKeyMlKem512);
    _assertClass(enc_sk, PrivateEncKeyMlKem512);
    const ret = wasm.process_user_decryption_resp(client.__wbg_ptr, ptr0, ptr1, ptr2, len2, enc_pk.__wbg_ptr, enc_sk.__wbg_ptr, verify);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v4 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v4;
}

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
 * * `verify` - Whether to perform signature verification for the response.
 * It is insecure if `verify = false`!
 * @param {Client} client
 * @param {any} request
 * @param {any} eip712_domain
 * @param {any} agg_resp
 * @param {PublicEncKeyMlKem512} enc_pk
 * @param {PrivateEncKeyMlKem512} enc_sk
 * @param {boolean} verify
 * @returns {TypedPlaintext[]}
 */
export function process_user_decryption_resp_from_js(client, request, eip712_domain, agg_resp, enc_pk, enc_sk, verify) {
    _assertClass(client, Client);
    _assertClass(enc_pk, PublicEncKeyMlKem512);
    _assertClass(enc_sk, PrivateEncKeyMlKem512);
    const ret = wasm.process_user_decryption_resp_from_js(client.__wbg_ptr, request, eip712_domain, agg_resp, enc_pk.__wbg_ptr, enc_sk.__wbg_ptr, verify);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}

/**
 * @param {PublicSigKey} pk
 * @returns {Uint8Array}
 */
export function public_sig_key_to_u8vec(pk) {
    _assertClass(pk, PublicSigKey);
    const ret = wasm.public_sig_key_to_u8vec(pk.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {Uint8Array} v
 * @returns {PublicEncKeyMlKem512}
 */
export function u8vec_to_ml_kem_pke_pk(v) {
    const ptr0 = passArray8ToWasm0(v, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.u8vec_to_ml_kem_pke_pk(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return PublicEncKeyMlKem512.__wrap(ret[0]);
}

/**
 * @param {Uint8Array} v
 * @returns {PrivateEncKeyMlKem512}
 */
export function u8vec_to_ml_kem_pke_sk(v) {
    const ptr0 = passArray8ToWasm0(v, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.u8vec_to_ml_kem_pke_sk(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return PrivateEncKeyMlKem512.__wrap(ret[0]);
}

/**
 * @param {Uint8Array} v
 * @returns {PrivateSigKey}
 */
export function u8vec_to_private_sig_key(v) {
    const ptr0 = passArray8ToWasm0(v, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.u8vec_to_private_sig_key(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return PrivateSigKey.__wrap(ret[0]);
}

/**
 * @param {Uint8Array} v
 * @returns {PublicSigKey}
 */
export function u8vec_to_public_sig_key(v) {
    const ptr0 = passArray8ToWasm0(v, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.u8vec_to_public_sig_key(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return PublicSigKey.__wrap(ret[0]);
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_8c4e43fe74559d73: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_Number_04624de7d0e8332d: function(arg0) {
            const ret = Number(arg0);
            return ret;
        },
        __wbg_String_8f0eb39a4a4c2f66: function(arg0, arg1) {
            const ret = String(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_boolean_get_bbbb1c18aa2f5e25: function(arg0) {
            const v = arg0;
            const ret = typeof(v) === 'boolean' ? v : undefined;
            return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
        },
        __wbg___wbindgen_debug_string_0bc8482c6e3508ae: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_in_47fa6863be6f2f25: function(arg0, arg1) {
            const ret = arg0 in arg1;
            return ret;
        },
        __wbg___wbindgen_is_function_0095a73b8b156f76: function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_null_ac34f5003991759a: function(arg0) {
            const ret = arg0 === null;
            return ret;
        },
        __wbg___wbindgen_is_object_5ae8e5880f2c1fbd: function(arg0) {
            const val = arg0;
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        },
        __wbg___wbindgen_is_string_cd444516edc5b180: function(arg0) {
            const ret = typeof(arg0) === 'string';
            return ret;
        },
        __wbg___wbindgen_is_undefined_9e4d92534c42d778: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_jsval_loose_eq_9dd77d8cd6671811: function(arg0, arg1) {
            const ret = arg0 == arg1;
            return ret;
        },
        __wbg___wbindgen_number_get_8ff4255516ccad3e: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_string_get_72fb696202c56729: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_be289d5034ed271b: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_call_389efe28435a9388: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments); },
        __wbg_call_4708e0c13bdc8e95: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_crypto_ed58b8e10a292839: function(arg0) {
            const ret = arg0.crypto;
            return ret;
        },
        __wbg_done_57b39ecd9addfe81: function(arg0) {
            const ret = arg0.done;
            return ret;
        },
        __wbg_error_7534b8e9a36f1ab4: function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        },
        __wbg_getRandomValues_bcb4912f16000dc4: function() { return handleError(function (arg0, arg1) {
            arg0.getRandomValues(arg1);
        }, arguments); },
        __wbg_get_9b94d73e6221f75c: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        },
        __wbg_get_b3ed3ad4be2bc8ac: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_get_with_ref_key_1dc361bd10053bfe: function(arg0, arg1) {
            const ret = arg0[arg1];
            return ret;
        },
        __wbg_instanceof_ArrayBuffer_c367199e2fa2aa04: function(arg0) {
            let result;
            try {
                result = arg0 instanceof ArrayBuffer;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Uint8Array_9b9075935c74707c: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Uint8Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_isArray_d314bb98fcf08331: function(arg0) {
            const ret = Array.isArray(arg0);
            return ret;
        },
        __wbg_isSafeInteger_bfbc7332a9768d2a: function(arg0) {
            const ret = Number.isSafeInteger(arg0);
            return ret;
        },
        __wbg_iterator_6ff6560ca1568e55: function() {
            const ret = Symbol.iterator;
            return ret;
        },
        __wbg_length_32ed9a279acd054c: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_length_35a7bace40f36eac: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_msCrypto_0a36e2ec3a343d26: function(arg0) {
            const ret = arg0.msCrypto;
            return ret;
        },
        __wbg_new_8a6f238a6ece86ea: function() {
            const ret = new Error();
            return ret;
        },
        __wbg_new_dd2b680c8bf6ae29: function(arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        },
        __wbg_new_no_args_1c7c842f08d00ebb: function(arg0, arg1) {
            const ret = new Function(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_new_with_length_a2c39cbe88fd8ff1: function(arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return ret;
        },
        __wbg_next_3482f54c49e8af19: function() { return handleError(function (arg0) {
            const ret = arg0.next();
            return ret;
        }, arguments); },
        __wbg_next_418f80d8f5303233: function(arg0) {
            const ret = arg0.next;
            return ret;
        },
        __wbg_node_02999533c4ea02e3: function(arg0) {
            const ret = arg0.node;
            return ret;
        },
        __wbg_process_5c1d670bc53614b8: function(arg0) {
            const ret = arg0.process;
            return ret;
        },
        __wbg_prototypesetcall_bdcdcc5842e4d77d: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
        },
        __wbg_randomFillSync_ab2cfe79ebbf2740: function() { return handleError(function (arg0, arg1) {
            arg0.randomFillSync(arg1);
        }, arguments); },
        __wbg_require_79b1e9274cde3c87: function() { return handleError(function () {
            const ret = module.require;
            return ret;
        }, arguments); },
        __wbg_serveridaddr_new: function(arg0) {
            const ret = ServerIdAddr.__wrap(arg0);
            return ret;
        },
        __wbg_serveridaddr_unwrap: function(arg0) {
            const ret = ServerIdAddr.__unwrap(arg0);
            return ret;
        },
        __wbg_stack_0ed75d68575b0f3c: function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_static_accessor_GLOBAL_12837167ad935116: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_GLOBAL_THIS_e628e89ab3b1c95f: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_SELF_a621d3dfbb60d0ce: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_WINDOW_f8727f0cf888e0bd: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_subarray_a96e1fef17ed23cb: function(arg0, arg1, arg2) {
            const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
            return ret;
        },
        __wbg_typedciphertext_new: function(arg0) {
            const ret = TypedCiphertext.__wrap(arg0);
            return ret;
        },
        __wbg_typedciphertext_unwrap: function(arg0) {
            const ret = TypedCiphertext.__unwrap(arg0);
            return ret;
        },
        __wbg_typedplaintext_new: function(arg0) {
            const ret = TypedPlaintext.__wrap(arg0);
            return ret;
        },
        __wbg_typedsigncryptedciphertext_new: function(arg0) {
            const ret = TypedSigncryptedCiphertext.__wrap(arg0);
            return ret;
        },
        __wbg_typedsigncryptedciphertext_unwrap: function(arg0) {
            const ret = TypedSigncryptedCiphertext.__unwrap(arg0);
            return ret;
        },
        __wbg_userdecryptionresponse_unwrap: function(arg0) {
            const ret = UserDecryptionResponse.__unwrap(arg0);
            return ret;
        },
        __wbg_value_0546255b415e96c1: function(arg0) {
            const ret = arg0.value;
            return ret;
        },
        __wbg_versions_c71aa1626a93e0a1: function(arg0) {
            const ret = arg0.versions;
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
            const ret = getArrayU8FromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./kms_lib_bg.js": import0,
    };
}

const CiphertextHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_ciphertexthandle_free(ptr >>> 0, 1));
const ClientFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_client_free(ptr >>> 0, 1));
const Eip712DomainMsgFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_eip712domainmsg_free(ptr >>> 0, 1));
const ParsedUserDecryptionRequestFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_parseduserdecryptionrequest_free(ptr >>> 0, 1));
const PrivateEncKeyMlKem512Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_privateenckeymlkem512_free(ptr >>> 0, 1));
const PrivateSigKeyFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_privatesigkey_free(ptr >>> 0, 1));
const PublicEncKeyMlKem512Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_publicenckeymlkem512_free(ptr >>> 0, 1));
const PublicSigKeyFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_publicsigkey_free(ptr >>> 0, 1));
const RequestIdFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_requestid_free(ptr >>> 0, 1));
const ServerIdAddrFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_serveridaddr_free(ptr >>> 0, 1));
const TypedCiphertextFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_typedciphertext_free(ptr >>> 0, 1));
const TypedPlaintextFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_typedplaintext_free(ptr >>> 0, 1));
const TypedSigncryptedCiphertextFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_typedsigncryptedciphertext_free(ptr >>> 0, 1));
const UserDecryptionRequestFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_userdecryptionrequest_free(ptr >>> 0, 1));
const UserDecryptionResponseFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_userdecryptionresponse_free(ptr >>> 0, 1));
const UserDecryptionResponsePayloadFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_userdecryptionresponsepayload_free(ptr >>> 0, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    // if (module_or_path === undefined) {
    //     module_or_path = new URL('kms_lib_bg.wasm', import.meta.url);
    // }
    const imports = __wbg_get_imports();

    // if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
    //     module_or_path = fetch(module_or_path);
    // }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

function getWasmInfo() {
  return {
    name: "tkms",
    version: "0.13.10"
  }
}

export { initSync, getWasmInfo, __wbg_init as default };
