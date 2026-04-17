/* @ts-self-types="./tfhe.d.ts" */
import {
  startWorkers,
  getTfheWorkers,
  terminateWorkers,
  setWorkerUrlConfig,
} from "./startWorkers.v1.5.3.js";

export class Boolean {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    BooleanFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_boolean_free(ptr, 0);
  }
  /**
   * @param {BooleanCompressedCiphertext} compressed_ciphertext
   * @returns {BooleanCiphertext}
   */
  static decompress_ciphertext(compressed_ciphertext) {
    _assertClass(compressed_ciphertext, BooleanCompressedCiphertext);
    const ret = wasm.boolean_decompress_ciphertext(
      compressed_ciphertext.__wbg_ptr,
    );
    return BooleanCiphertext.__wrap(ret);
  }
  /**
   * @param {BooleanClientKey} client_key
   * @param {BooleanCiphertext} ct
   * @returns {boolean}
   */
  static decrypt(client_key, ct) {
    _assertClass(client_key, BooleanClientKey);
    _assertClass(ct, BooleanCiphertext);
    const ret = wasm.boolean_decrypt(client_key.__wbg_ptr, ct.__wbg_ptr);
    return ret !== 0;
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {BooleanCiphertext}
   */
  static deserialize_ciphertext(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.boolean_deserialize_ciphertext(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return BooleanCiphertext.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {BooleanClientKey}
   */
  static deserialize_client_key(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.boolean_deserialize_client_key(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return BooleanClientKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {BooleanCompressedCiphertext}
   */
  static deserialize_compressed_ciphertext(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.boolean_deserialize_compressed_ciphertext(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return BooleanCompressedCiphertext.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {BooleanCompressedServerKey}
   */
  static deserialize_compressed_server_key(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.boolean_deserialize_compressed_server_key(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return BooleanCompressedServerKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {BooleanPublicKey}
   */
  static deserialize_public_key(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.boolean_deserialize_public_key(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return BooleanPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {BooleanClientKey} client_key
   * @param {boolean} message
   * @returns {BooleanCiphertext}
   */
  static encrypt(client_key, message) {
    _assertClass(client_key, BooleanClientKey);
    const ret = wasm.boolean_encrypt(client_key.__wbg_ptr, message);
    return BooleanCiphertext.__wrap(ret);
  }
  /**
   * @param {BooleanClientKey} client_key
   * @param {boolean} message
   * @returns {BooleanCompressedCiphertext}
   */
  static encrypt_compressed(client_key, message) {
    _assertClass(client_key, BooleanClientKey);
    const ret = wasm.boolean_encrypt_compressed(client_key.__wbg_ptr, message);
    return BooleanCompressedCiphertext.__wrap(ret);
  }
  /**
   * @param {BooleanPublicKey} public_key
   * @param {boolean} message
   * @returns {BooleanCiphertext}
   */
  static encrypt_with_public_key(public_key, message) {
    _assertClass(public_key, BooleanPublicKey);
    const ret = wasm.boolean_encrypt_with_public_key(
      public_key.__wbg_ptr,
      message,
    );
    return BooleanCiphertext.__wrap(ret);
  }
  /**
   * @param {number} parameter_choice
   * @returns {BooleanParameters}
   */
  static get_parameters(parameter_choice) {
    const ret = wasm.boolean_get_parameters(parameter_choice);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return BooleanParameters.__wrap(ret[0]);
  }
  /**
   * @param {BooleanParameters} parameters
   * @returns {BooleanClientKey}
   */
  static new_client_key(parameters) {
    _assertClass(parameters, BooleanParameters);
    const ret = wasm.boolean_new_client_key(parameters.__wbg_ptr);
    return BooleanClientKey.__wrap(ret);
  }
  /**
   * @param {bigint} seed_high_bytes
   * @param {bigint} seed_low_bytes
   * @param {BooleanParameters} parameters
   * @returns {BooleanClientKey}
   */
  static new_client_key_from_seed_and_parameters(
    seed_high_bytes,
    seed_low_bytes,
    parameters,
  ) {
    _assertClass(parameters, BooleanParameters);
    const ret = wasm.boolean_new_client_key_from_seed_and_parameters(
      seed_high_bytes,
      seed_low_bytes,
      parameters.__wbg_ptr,
    );
    return BooleanClientKey.__wrap(ret);
  }
  /**
   * @param {BooleanClientKey} client_key
   * @returns {BooleanCompressedServerKey}
   */
  static new_compressed_server_key(client_key) {
    _assertClass(client_key, BooleanClientKey);
    const ret = wasm.boolean_new_compressed_server_key(client_key.__wbg_ptr);
    return BooleanCompressedServerKey.__wrap(ret);
  }
  /**
   * @param {number} std_dev
   * @returns {BooleanNoiseDistribution}
   */
  static new_gaussian_from_std_dev(std_dev) {
    const ret = wasm.boolean_new_gaussian_from_std_dev(std_dev);
    return BooleanNoiseDistribution.__wrap(ret);
  }
  /**
   * @param {number} lwe_dimension
   * @param {number} glwe_dimension
   * @param {number} polynomial_size
   * @param {BooleanNoiseDistribution} lwe_noise_distribution
   * @param {BooleanNoiseDistribution} glwe_noise_distribution
   * @param {number} pbs_base_log
   * @param {number} pbs_level
   * @param {number} ks_base_log
   * @param {number} ks_level
   * @param {BooleanEncryptionKeyChoice} encryption_key_choice
   * @returns {BooleanParameters}
   */
  static new_parameters(
    lwe_dimension,
    glwe_dimension,
    polynomial_size,
    lwe_noise_distribution,
    glwe_noise_distribution,
    pbs_base_log,
    pbs_level,
    ks_base_log,
    ks_level,
    encryption_key_choice,
  ) {
    _assertClass(lwe_noise_distribution, BooleanNoiseDistribution);
    _assertClass(glwe_noise_distribution, BooleanNoiseDistribution);
    const ret = wasm.boolean_new_parameters(
      lwe_dimension,
      glwe_dimension,
      polynomial_size,
      lwe_noise_distribution.__wbg_ptr,
      glwe_noise_distribution.__wbg_ptr,
      pbs_base_log,
      pbs_level,
      ks_base_log,
      ks_level,
      encryption_key_choice,
    );
    return BooleanParameters.__wrap(ret);
  }
  /**
   * @param {BooleanClientKey} client_key
   * @returns {BooleanPublicKey}
   */
  static new_public_key(client_key) {
    _assertClass(client_key, BooleanClientKey);
    const ret = wasm.boolean_new_public_key(client_key.__wbg_ptr);
    return BooleanPublicKey.__wrap(ret);
  }
  /**
   * @param {BooleanCiphertext} ciphertext
   * @returns {Uint8Array}
   */
  static serialize_ciphertext(ciphertext) {
    _assertClass(ciphertext, BooleanCiphertext);
    const ret = wasm.boolean_serialize_ciphertext(ciphertext.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @param {BooleanClientKey} client_key
   * @returns {Uint8Array}
   */
  static serialize_client_key(client_key) {
    _assertClass(client_key, BooleanClientKey);
    const ret = wasm.boolean_serialize_client_key(client_key.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @param {BooleanCompressedCiphertext} ciphertext
   * @returns {Uint8Array}
   */
  static serialize_compressed_ciphertext(ciphertext) {
    _assertClass(ciphertext, BooleanCompressedCiphertext);
    const ret = wasm.boolean_serialize_compressed_ciphertext(
      ciphertext.__wbg_ptr,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @param {BooleanCompressedServerKey} server_key
   * @returns {Uint8Array}
   */
  static serialize_compressed_server_key(server_key) {
    _assertClass(server_key, BooleanCompressedServerKey);
    const ret = wasm.boolean_serialize_compressed_server_key(
      server_key.__wbg_ptr,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @param {BooleanPublicKey} public_key
   * @returns {Uint8Array}
   */
  static serialize_public_key(public_key) {
    _assertClass(public_key, BooleanPublicKey);
    const ret = wasm.boolean_serialize_public_key(public_key.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @param {boolean} message
   * @returns {BooleanCiphertext}
   */
  static trivial_encrypt(message) {
    const ret = wasm.boolean_trivial_encrypt(message);
    return BooleanCiphertext.__wrap(ret);
  }
  /**
   * @param {number} bound_log2
   * @returns {BooleanNoiseDistribution}
   */
  static try_new_t_uniform(bound_log2) {
    const ret = wasm.boolean_try_new_t_uniform(bound_log2);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return BooleanNoiseDistribution.__wrap(ret[0]);
  }
}
if (Symbol.dispose) Boolean.prototype[Symbol.dispose] = Boolean.prototype.free;

export class BooleanCiphertext {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(BooleanCiphertext.prototype);
    obj.__wbg_ptr = ptr;
    BooleanCiphertextFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    BooleanCiphertextFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_booleanciphertext_free(ptr, 0);
  }
}
if (Symbol.dispose)
  BooleanCiphertext.prototype[Symbol.dispose] =
    BooleanCiphertext.prototype.free;

export class BooleanClientKey {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(BooleanClientKey.prototype);
    obj.__wbg_ptr = ptr;
    BooleanClientKeyFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    BooleanClientKeyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_booleanclientkey_free(ptr, 0);
  }
}
if (Symbol.dispose)
  BooleanClientKey.prototype[Symbol.dispose] = BooleanClientKey.prototype.free;

export class BooleanCompressedCiphertext {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(BooleanCompressedCiphertext.prototype);
    obj.__wbg_ptr = ptr;
    BooleanCompressedCiphertextFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    BooleanCompressedCiphertextFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_booleancompressedciphertext_free(ptr, 0);
  }
}
if (Symbol.dispose)
  BooleanCompressedCiphertext.prototype[Symbol.dispose] =
    BooleanCompressedCiphertext.prototype.free;

export class BooleanCompressedServerKey {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(BooleanCompressedServerKey.prototype);
    obj.__wbg_ptr = ptr;
    BooleanCompressedServerKeyFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    BooleanCompressedServerKeyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_booleancompressedserverkey_free(ptr, 0);
  }
}
if (Symbol.dispose)
  BooleanCompressedServerKey.prototype[Symbol.dispose] =
    BooleanCompressedServerKey.prototype.free;

/**
 * @enum {0 | 1}
 */
export const BooleanEncryptionKeyChoice = Object.freeze({
  Big: 0,
  0: "Big",
  Small: 1,
  1: "Small",
});

export class BooleanNoiseDistribution {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(BooleanNoiseDistribution.prototype);
    obj.__wbg_ptr = ptr;
    BooleanNoiseDistributionFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    BooleanNoiseDistributionFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_booleannoisedistribution_free(ptr, 0);
  }
}
if (Symbol.dispose)
  BooleanNoiseDistribution.prototype[Symbol.dispose] =
    BooleanNoiseDistribution.prototype.free;

/**
 * @enum {0 | 1 | 2 | 3}
 */
export const BooleanParameterSet = Object.freeze({
  Default: 0,
  0: "Default",
  TfheLib: 1,
  1: "TfheLib",
  DefaultKsPbs: 2,
  2: "DefaultKsPbs",
  TfheLibKsPbs: 3,
  3: "TfheLibKsPbs",
});

export class BooleanParameters {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(BooleanParameters.prototype);
    obj.__wbg_ptr = ptr;
    BooleanParametersFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    BooleanParametersFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_booleanparameters_free(ptr, 0);
  }
}
if (Symbol.dispose)
  BooleanParameters.prototype[Symbol.dispose] =
    BooleanParameters.prototype.free;

export class BooleanPublicKey {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(BooleanPublicKey.prototype);
    obj.__wbg_ptr = ptr;
    BooleanPublicKeyFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    BooleanPublicKeyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_booleanpublickey_free(ptr, 0);
  }
}
if (Symbol.dispose)
  BooleanPublicKey.prototype[Symbol.dispose] = BooleanPublicKey.prototype.free;

export class CompactCiphertextList {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompactCiphertextList.prototype);
    obj.__wbg_ptr = ptr;
    CompactCiphertextListFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompactCiphertextListFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compactciphertextlist_free(ptr, 0);
  }
  /**
   * @param {TfheCompactPublicKey} public_key
   * @returns {CompactCiphertextListBuilder}
   */
  static builder(public_key) {
    _assertClass(public_key, TfheCompactPublicKey);
    const ret = wasm.compactciphertextlist_builder(public_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompactCiphertextListBuilder.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompactCiphertextList}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compactciphertextlist_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompactCiphertextList.__wrap(ret[0]);
  }
  /**
   * @returns {CompactCiphertextListExpander}
   */
  expand() {
    const ret = wasm.compactciphertextlist_expand(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompactCiphertextListExpander.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheTypes | undefined}
   */
  get_kind_of(index) {
    const ret = wasm.compactciphertextlist_get_kind_of(this.__wbg_ptr, index);
    return ret === 84 ? undefined : ret;
  }
  /**
   * @returns {boolean}
   */
  is_empty() {
    const ret = wasm.compactciphertextlist_is_empty(this.__wbg_ptr);
    return ret !== 0;
  }
  /**
   * @returns {number}
   */
  len() {
    const ret = wasm.compactciphertextlist_len(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompactCiphertextList}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compactciphertextlist_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompactCiphertextList.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compactciphertextlist_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compactciphertextlist_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompactCiphertextList.prototype[Symbol.dispose] =
    CompactCiphertextList.prototype.free;

export class CompactCiphertextListBuilder {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompactCiphertextListBuilder.prototype);
    obj.__wbg_ptr = ptr;
    CompactCiphertextListBuilderFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompactCiphertextListBuilderFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compactciphertextlistbuilder_free(ptr, 0);
  }
  /**
   * @returns {CompactCiphertextList}
   */
  build() {
    const ret = wasm.compactciphertextlistbuilder_build(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompactCiphertextList.__wrap(ret[0]);
  }
  /**
   * @returns {CompactCiphertextList}
   */
  build_packed() {
    const ret = wasm.compactciphertextlistbuilder_build_packed(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompactCiphertextList.__wrap(ret[0]);
  }
  /**
   * @param {CompactPkeCrs} crs
   * @param {Uint8Array} metadata
   * @param {ZkComputeLoad} compute_load
   * @returns {ProvenCompactCiphertextList}
   */
  build_with_proof_packed(crs, metadata, compute_load) {
    _assertClass(crs, CompactPkeCrs);
    const ptr0 = passArray8ToWasm0(metadata, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compactciphertextlistbuilder_build_with_proof_packed(
      this.__wbg_ptr,
      crs.__wbg_ptr,
      ptr0,
      len0,
      compute_load,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ProvenCompactCiphertextList.__wrap(ret[0]);
  }
  /**
   * @param {boolean} value
   */
  push_boolean(value) {
    const ret = wasm.compactciphertextlistbuilder_push_boolean(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_i10(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i10(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i1024(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i1024(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i104(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i104(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i112(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i112(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_i12(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i12(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i120(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i120(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i128(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i128(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i136(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i136(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_i14(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i14(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i144(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i144(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i152(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i152(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_i16(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i16(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i160(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i160(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i168(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i168(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i176(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i176(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i184(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i184(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i192(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i192(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_i2(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i2(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i200(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i200(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i2048(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i2048(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i208(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i208(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i216(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i216(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i224(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i224(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i232(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i232(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_i24(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i24(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i240(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i240(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i248(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i248(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i256(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i256(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_i32(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i32(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_i4(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i4(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {bigint} value
   */
  push_i40(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i40(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {bigint} value
   */
  push_i48(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i48(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i512(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i512(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {bigint} value
   */
  push_i56(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i56(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_i6(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i6(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {bigint} value
   */
  push_i64(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i64(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i72(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i72(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_i8(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i8(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i80(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i80(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i88(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i88(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_i96(value) {
    const ret = wasm.compactciphertextlistbuilder_push_i96(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_u10(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u10(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u1024(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u1024(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u104(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u104(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u112(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u112(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_u12(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u12(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u120(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u120(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u128(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u128(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u136(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u136(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_u14(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u14(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u144(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u144(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u152(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u152(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_u16(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u16(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u160(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u160(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u168(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u168(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u176(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u176(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u184(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u184(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u192(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u192(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_u2(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u2(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u200(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u200(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u2048(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u2048(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u208(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u208(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u216(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u216(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u224(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u224(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u232(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u232(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_u24(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u24(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u240(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u240(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u248(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u248(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u256(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u256(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_u32(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u32(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_u4(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u4(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {bigint} value
   */
  push_u40(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u40(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {bigint} value
   */
  push_u48(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u48(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u512(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u512(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {bigint} value
   */
  push_u56(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u56(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_u6(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u6(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {bigint} value
   */
  push_u64(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u64(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u72(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u72(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {number} value
   */
  push_u8(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u8(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u80(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u80(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u88(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u88(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
  /**
   * @param {any} value
   */
  push_u96(value) {
    const ret = wasm.compactciphertextlistbuilder_push_u96(
      this.__wbg_ptr,
      value,
    );
    if (ret[1]) {
      throw takeFromExternrefTable0(ret[0]);
    }
  }
}
if (Symbol.dispose)
  CompactCiphertextListBuilder.prototype[Symbol.dispose] =
    CompactCiphertextListBuilder.prototype.free;

export class CompactCiphertextListExpander {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompactCiphertextListExpander.prototype);
    obj.__wbg_ptr = ptr;
    CompactCiphertextListExpanderFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompactCiphertextListExpanderFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compactciphertextlistexpander_free(ptr, 0);
  }
  /**
   * @param {number} index
   * @returns {FheBool}
   */
  get_bool(index) {
    const ret = wasm.compactciphertextlistexpander_get_bool(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheBool.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt10}
   */
  get_int10(index) {
    const ret = wasm.compactciphertextlistexpander_get_int10(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt10.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt1024}
   */
  get_int1024(index) {
    const ret = wasm.compactciphertextlistexpander_get_int1024(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt1024.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt104}
   */
  get_int104(index) {
    const ret = wasm.compactciphertextlistexpander_get_int104(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt104.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt112}
   */
  get_int112(index) {
    const ret = wasm.compactciphertextlistexpander_get_int112(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt112.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt12}
   */
  get_int12(index) {
    const ret = wasm.compactciphertextlistexpander_get_int12(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt12.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt120}
   */
  get_int120(index) {
    const ret = wasm.compactciphertextlistexpander_get_int120(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt120.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt128}
   */
  get_int128(index) {
    const ret = wasm.compactciphertextlistexpander_get_int128(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt128.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt136}
   */
  get_int136(index) {
    const ret = wasm.compactciphertextlistexpander_get_int136(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt136.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt14}
   */
  get_int14(index) {
    const ret = wasm.compactciphertextlistexpander_get_int14(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt14.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt144}
   */
  get_int144(index) {
    const ret = wasm.compactciphertextlistexpander_get_int144(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt144.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt152}
   */
  get_int152(index) {
    const ret = wasm.compactciphertextlistexpander_get_int152(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt152.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt16}
   */
  get_int16(index) {
    const ret = wasm.compactciphertextlistexpander_get_int16(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt16.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt160}
   */
  get_int160(index) {
    const ret = wasm.compactciphertextlistexpander_get_int160(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt160.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt168}
   */
  get_int168(index) {
    const ret = wasm.compactciphertextlistexpander_get_int168(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt168.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt176}
   */
  get_int176(index) {
    const ret = wasm.compactciphertextlistexpander_get_int176(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt176.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt184}
   */
  get_int184(index) {
    const ret = wasm.compactciphertextlistexpander_get_int184(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt184.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt192}
   */
  get_int192(index) {
    const ret = wasm.compactciphertextlistexpander_get_int192(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt192.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt2}
   */
  get_int2(index) {
    const ret = wasm.compactciphertextlistexpander_get_int2(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt2.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt200}
   */
  get_int200(index) {
    const ret = wasm.compactciphertextlistexpander_get_int200(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt200.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt2048}
   */
  get_int2048(index) {
    const ret = wasm.compactciphertextlistexpander_get_int2048(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt2048.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt208}
   */
  get_int208(index) {
    const ret = wasm.compactciphertextlistexpander_get_int208(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt208.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt216}
   */
  get_int216(index) {
    const ret = wasm.compactciphertextlistexpander_get_int216(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt216.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt224}
   */
  get_int224(index) {
    const ret = wasm.compactciphertextlistexpander_get_int224(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt224.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt232}
   */
  get_int232(index) {
    const ret = wasm.compactciphertextlistexpander_get_int232(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt232.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt24}
   */
  get_int24(index) {
    const ret = wasm.compactciphertextlistexpander_get_int24(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt24.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt240}
   */
  get_int240(index) {
    const ret = wasm.compactciphertextlistexpander_get_int240(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt240.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt248}
   */
  get_int248(index) {
    const ret = wasm.compactciphertextlistexpander_get_int248(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt248.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt256}
   */
  get_int256(index) {
    const ret = wasm.compactciphertextlistexpander_get_int256(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt256.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt32}
   */
  get_int32(index) {
    const ret = wasm.compactciphertextlistexpander_get_int32(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt32.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt4}
   */
  get_int4(index) {
    const ret = wasm.compactciphertextlistexpander_get_int4(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt4.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt40}
   */
  get_int40(index) {
    const ret = wasm.compactciphertextlistexpander_get_int40(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt40.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt48}
   */
  get_int48(index) {
    const ret = wasm.compactciphertextlistexpander_get_int48(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt48.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt512}
   */
  get_int512(index) {
    const ret = wasm.compactciphertextlistexpander_get_int512(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt512.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt56}
   */
  get_int56(index) {
    const ret = wasm.compactciphertextlistexpander_get_int56(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt56.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt6}
   */
  get_int6(index) {
    const ret = wasm.compactciphertextlistexpander_get_int6(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt6.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt64}
   */
  get_int64(index) {
    const ret = wasm.compactciphertextlistexpander_get_int64(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt64.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt72}
   */
  get_int72(index) {
    const ret = wasm.compactciphertextlistexpander_get_int72(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt72.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt8}
   */
  get_int8(index) {
    const ret = wasm.compactciphertextlistexpander_get_int8(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt8.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt80}
   */
  get_int80(index) {
    const ret = wasm.compactciphertextlistexpander_get_int80(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt80.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt88}
   */
  get_int88(index) {
    const ret = wasm.compactciphertextlistexpander_get_int88(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt88.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheInt96}
   */
  get_int96(index) {
    const ret = wasm.compactciphertextlistexpander_get_int96(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt96.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheTypes | undefined}
   */
  get_kind_of(index) {
    const ret = wasm.compactciphertextlistexpander_get_kind_of(
      this.__wbg_ptr,
      index,
    );
    return ret === 84 ? undefined : ret;
  }
  /**
   * @param {number} index
   * @returns {FheUint10}
   */
  get_uint10(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint10(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint10.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint1024}
   */
  get_uint1024(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint1024(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint1024.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint104}
   */
  get_uint104(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint104(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint104.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint112}
   */
  get_uint112(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint112(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint112.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint12}
   */
  get_uint12(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint12(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint12.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint120}
   */
  get_uint120(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint120(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint120.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint128}
   */
  get_uint128(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint128(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint128.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint136}
   */
  get_uint136(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint136(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint136.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint14}
   */
  get_uint14(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint14(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint14.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint144}
   */
  get_uint144(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint144(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint144.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint152}
   */
  get_uint152(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint152(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint152.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint16}
   */
  get_uint16(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint16(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint16.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint160}
   */
  get_uint160(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint160(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint160.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint168}
   */
  get_uint168(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint168(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint168.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint176}
   */
  get_uint176(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint176(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint176.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint184}
   */
  get_uint184(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint184(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint184.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint192}
   */
  get_uint192(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint192(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint192.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint2}
   */
  get_uint2(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint2(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint2.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint200}
   */
  get_uint200(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint200(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint200.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint2048}
   */
  get_uint2048(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint2048(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint2048.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint208}
   */
  get_uint208(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint208(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint208.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint216}
   */
  get_uint216(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint216(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint216.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint224}
   */
  get_uint224(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint224(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint224.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint232}
   */
  get_uint232(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint232(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint232.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint24}
   */
  get_uint24(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint24(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint24.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint240}
   */
  get_uint240(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint240(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint240.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint248}
   */
  get_uint248(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint248(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint248.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint256}
   */
  get_uint256(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint256(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint256.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint32}
   */
  get_uint32(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint32(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint32.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint4}
   */
  get_uint4(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint4(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint4.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint40}
   */
  get_uint40(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint40(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint40.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint48}
   */
  get_uint48(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint48(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint48.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint512}
   */
  get_uint512(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint512(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint512.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint56}
   */
  get_uint56(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint56(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint56.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint6}
   */
  get_uint6(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint6(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint6.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint64}
   */
  get_uint64(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint64(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint64.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint72}
   */
  get_uint72(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint72(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint72.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint8}
   */
  get_uint8(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint8(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint8.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint80}
   */
  get_uint80(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint80(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint80.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint88}
   */
  get_uint88(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint88(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint88.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheUint96}
   */
  get_uint96(index) {
    const ret = wasm.compactciphertextlistexpander_get_uint96(
      this.__wbg_ptr,
      index,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint96.__wrap(ret[0]);
  }
  /**
   * @returns {boolean}
   */
  is_empty() {
    const ret = wasm.compactciphertextlistexpander_is_empty(this.__wbg_ptr);
    return ret !== 0;
  }
  /**
   * @returns {number}
   */
  len() {
    const ret = wasm.compactciphertextlistexpander_len(this.__wbg_ptr);
    return ret >>> 0;
  }
}
if (Symbol.dispose)
  CompactCiphertextListExpander.prototype[Symbol.dispose] =
    CompactCiphertextListExpander.prototype.free;

export class CompactPkeCrs {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompactPkeCrs.prototype);
    obj.__wbg_ptr = ptr;
    CompactPkeCrsFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompactPkeCrsFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compactpkecrs_free(ptr, 0);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompactPkeCrs}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compactpkecrs_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompactPkeCrs.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompactPkeCrs}
   */
  static deserialize_from_public_params(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compactpkecrs_deserialize_from_public_params(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompactPkeCrs.__wrap(ret[0]);
  }
  /**
   * @param {TfheConfig} config
   * @param {number} max_num_bits
   * @returns {CompactPkeCrs}
   */
  static from_config(config, max_num_bits) {
    _assertClass(config, TfheConfig);
    const ret = wasm.compactpkecrs_from_config(config.__wbg_ptr, max_num_bits);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompactPkeCrs.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompactPkeCrs}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compactpkecrs_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompactPkeCrs.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompactPkeCrs}
   */
  static safe_deserialize_from_public_params(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compactpkecrs_safe_deserialize_from_public_params(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompactPkeCrs.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compactpkecrs_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @param {boolean} compress
   * @returns {Uint8Array}
   */
  serialize(compress) {
    const ret = wasm.compactpkecrs_serialize(this.__wbg_ptr, compress);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompactPkeCrs.prototype[Symbol.dispose] = CompactPkeCrs.prototype.free;

export class CompressedFheBool {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheBool.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheBoolFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheBoolFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfhebool_free(ptr, 0);
  }
  /**
   * @returns {FheBool}
   */
  decompress() {
    const ret = wasm.compressedfhebool_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheBool.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheBool}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfhebool_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheBool.__wrap(ret[0]);
  }
  /**
   * @param {boolean} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheBool}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfhebool_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheBool.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheBool}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfhebool_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheBool.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfhebool_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfhebool_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheBool.prototype[Symbol.dispose] =
    CompressedFheBool.prototype.free;

export class CompressedFheInt10 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt10.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt10Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt10Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint10_free(ptr, 0);
  }
  /**
   * @returns {FheInt10}
   */
  decompress() {
    const ret = wasm.compressedfheint10_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt10.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt10}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint10_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt10.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt10}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint10_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt10.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt10}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint10_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt10.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint10_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint10_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt10.prototype[Symbol.dispose] =
    CompressedFheInt10.prototype.free;

export class CompressedFheInt1024 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt1024.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt1024Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt1024Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint1024_free(ptr, 0);
  }
  /**
   * @returns {FheInt1024}
   */
  decompress() {
    const ret = wasm.compressedfheint1024_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt1024.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt1024}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint1024_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt1024.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt1024}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint1024_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt1024.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt1024}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint1024_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt1024.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint1024_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint1024_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt1024.prototype[Symbol.dispose] =
    CompressedFheInt1024.prototype.free;

export class CompressedFheInt104 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt104.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt104Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt104Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint104_free(ptr, 0);
  }
  /**
   * @returns {FheInt104}
   */
  decompress() {
    const ret = wasm.compressedfheint104_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt104.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt104}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint104_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt104.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt104}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint104_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt104.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt104}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint104_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt104.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint104_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint104_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt104.prototype[Symbol.dispose] =
    CompressedFheInt104.prototype.free;

export class CompressedFheInt112 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt112.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt112Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt112Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint112_free(ptr, 0);
  }
  /**
   * @returns {FheInt112}
   */
  decompress() {
    const ret = wasm.compressedfheint112_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt112.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt112}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint112_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt112.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt112}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint112_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt112.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt112}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint112_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt112.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint112_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint112_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt112.prototype[Symbol.dispose] =
    CompressedFheInt112.prototype.free;

export class CompressedFheInt12 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt12.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt12Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt12Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint12_free(ptr, 0);
  }
  /**
   * @returns {FheInt12}
   */
  decompress() {
    const ret = wasm.compressedfheint12_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt12.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt12}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint12_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt12.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt12}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint12_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt12.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt12}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint12_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt12.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint12_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint12_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt12.prototype[Symbol.dispose] =
    CompressedFheInt12.prototype.free;

export class CompressedFheInt120 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt120.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt120Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt120Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint120_free(ptr, 0);
  }
  /**
   * @returns {FheInt120}
   */
  decompress() {
    const ret = wasm.compressedfheint120_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt120.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt120}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint120_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt120.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt120}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint120_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt120.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt120}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint120_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt120.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint120_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint120_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt120.prototype[Symbol.dispose] =
    CompressedFheInt120.prototype.free;

export class CompressedFheInt128 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt128.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt128Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt128Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint128_free(ptr, 0);
  }
  /**
   * @returns {FheInt128}
   */
  decompress() {
    const ret = wasm.compressedfheint128_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt128.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt128}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint128_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt128.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt128}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint128_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt128.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt128}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint128_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt128.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint128_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint128_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt128.prototype[Symbol.dispose] =
    CompressedFheInt128.prototype.free;

export class CompressedFheInt136 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt136.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt136Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt136Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint136_free(ptr, 0);
  }
  /**
   * @returns {FheInt136}
   */
  decompress() {
    const ret = wasm.compressedfheint136_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt136.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt136}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint136_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt136.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt136}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint136_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt136.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt136}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint136_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt136.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint136_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint136_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt136.prototype[Symbol.dispose] =
    CompressedFheInt136.prototype.free;

export class CompressedFheInt14 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt14.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt14Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt14Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint14_free(ptr, 0);
  }
  /**
   * @returns {FheInt14}
   */
  decompress() {
    const ret = wasm.compressedfheint14_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt14.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt14}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint14_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt14.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt14}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint14_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt14.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt14}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint14_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt14.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint14_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint14_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt14.prototype[Symbol.dispose] =
    CompressedFheInt14.prototype.free;

export class CompressedFheInt144 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt144.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt144Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt144Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint144_free(ptr, 0);
  }
  /**
   * @returns {FheInt144}
   */
  decompress() {
    const ret = wasm.compressedfheint144_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt144.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt144}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint144_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt144.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt144}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint144_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt144.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt144}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint144_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt144.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint144_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint144_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt144.prototype[Symbol.dispose] =
    CompressedFheInt144.prototype.free;

export class CompressedFheInt152 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt152.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt152Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt152Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint152_free(ptr, 0);
  }
  /**
   * @returns {FheInt152}
   */
  decompress() {
    const ret = wasm.compressedfheint152_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt152.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt152}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint152_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt152.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt152}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint152_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt152.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt152}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint152_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt152.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint152_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint152_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt152.prototype[Symbol.dispose] =
    CompressedFheInt152.prototype.free;

export class CompressedFheInt16 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt16.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt16Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt16Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint16_free(ptr, 0);
  }
  /**
   * @returns {FheInt16}
   */
  decompress() {
    const ret = wasm.compressedfheint16_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt16.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt16}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint16_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt16.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt16}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint16_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt16.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt16}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint16_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt16.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint16_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint16_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt16.prototype[Symbol.dispose] =
    CompressedFheInt16.prototype.free;

export class CompressedFheInt160 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt160.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt160Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt160Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint160_free(ptr, 0);
  }
  /**
   * @returns {FheInt160}
   */
  decompress() {
    const ret = wasm.compressedfheint160_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt160.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt160}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint160_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt160.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt160}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint160_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt160.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt160}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint160_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt160.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint160_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint160_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt160.prototype[Symbol.dispose] =
    CompressedFheInt160.prototype.free;

export class CompressedFheInt168 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt168.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt168Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt168Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint168_free(ptr, 0);
  }
  /**
   * @returns {FheInt168}
   */
  decompress() {
    const ret = wasm.compressedfheint168_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt168.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt168}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint168_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt168.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt168}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint168_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt168.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt168}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint168_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt168.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint168_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint168_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt168.prototype[Symbol.dispose] =
    CompressedFheInt168.prototype.free;

export class CompressedFheInt176 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt176.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt176Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt176Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint176_free(ptr, 0);
  }
  /**
   * @returns {FheInt176}
   */
  decompress() {
    const ret = wasm.compressedfheint176_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt176.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt176}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint176_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt176.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt176}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint176_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt176.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt176}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint176_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt176.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint176_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint176_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt176.prototype[Symbol.dispose] =
    CompressedFheInt176.prototype.free;

export class CompressedFheInt184 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt184.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt184Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt184Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint184_free(ptr, 0);
  }
  /**
   * @returns {FheInt184}
   */
  decompress() {
    const ret = wasm.compressedfheint184_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt184.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt184}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint184_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt184.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt184}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint184_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt184.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt184}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint184_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt184.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint184_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint184_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt184.prototype[Symbol.dispose] =
    CompressedFheInt184.prototype.free;

export class CompressedFheInt192 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt192.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt192Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt192Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint192_free(ptr, 0);
  }
  /**
   * @returns {FheInt192}
   */
  decompress() {
    const ret = wasm.compressedfheint192_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt192.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt192}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint192_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt192.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt192}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint192_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt192.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt192}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint192_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt192.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint192_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint192_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt192.prototype[Symbol.dispose] =
    CompressedFheInt192.prototype.free;

export class CompressedFheInt2 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt2.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt2Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt2Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint2_free(ptr, 0);
  }
  /**
   * @returns {FheInt2}
   */
  decompress() {
    const ret = wasm.compressedfheint2_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt2.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt2}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint2_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt2.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt2}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint2_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt2.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt2}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint2_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt2.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint2_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint2_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt2.prototype[Symbol.dispose] =
    CompressedFheInt2.prototype.free;

export class CompressedFheInt200 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt200.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt200Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt200Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint200_free(ptr, 0);
  }
  /**
   * @returns {FheInt200}
   */
  decompress() {
    const ret = wasm.compressedfheint200_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt200.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt200}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint200_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt200.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt200}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint200_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt200.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt200}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint200_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt200.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint200_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint200_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt200.prototype[Symbol.dispose] =
    CompressedFheInt200.prototype.free;

export class CompressedFheInt2048 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt2048.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt2048Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt2048Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint2048_free(ptr, 0);
  }
  /**
   * @returns {FheInt2048}
   */
  decompress() {
    const ret = wasm.compressedfheint2048_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt2048.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt2048}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint2048_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt2048.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt2048}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint2048_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt2048.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt2048}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint2048_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt2048.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint2048_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint2048_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt2048.prototype[Symbol.dispose] =
    CompressedFheInt2048.prototype.free;

export class CompressedFheInt208 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt208.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt208Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt208Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint208_free(ptr, 0);
  }
  /**
   * @returns {FheInt208}
   */
  decompress() {
    const ret = wasm.compressedfheint208_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt208.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt208}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint208_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt208.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt208}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint208_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt208.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt208}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint208_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt208.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint208_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint208_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt208.prototype[Symbol.dispose] =
    CompressedFheInt208.prototype.free;

export class CompressedFheInt216 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt216.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt216Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt216Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint216_free(ptr, 0);
  }
  /**
   * @returns {FheInt216}
   */
  decompress() {
    const ret = wasm.compressedfheint216_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt216.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt216}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint216_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt216.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt216}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint216_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt216.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt216}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint216_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt216.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint216_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint216_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt216.prototype[Symbol.dispose] =
    CompressedFheInt216.prototype.free;

export class CompressedFheInt224 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt224.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt224Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt224Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint224_free(ptr, 0);
  }
  /**
   * @returns {FheInt224}
   */
  decompress() {
    const ret = wasm.compressedfheint224_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt224.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt224}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint224_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt224.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt224}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint224_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt224.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt224}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint224_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt224.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint224_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint224_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt224.prototype[Symbol.dispose] =
    CompressedFheInt224.prototype.free;

export class CompressedFheInt232 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt232.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt232Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt232Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint232_free(ptr, 0);
  }
  /**
   * @returns {FheInt232}
   */
  decompress() {
    const ret = wasm.compressedfheint232_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt232.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt232}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint232_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt232.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt232}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint232_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt232.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt232}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint232_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt232.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint232_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint232_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt232.prototype[Symbol.dispose] =
    CompressedFheInt232.prototype.free;

export class CompressedFheInt24 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt24.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt24Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt24Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint24_free(ptr, 0);
  }
  /**
   * @returns {FheInt24}
   */
  decompress() {
    const ret = wasm.compressedfheint24_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt24.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt24}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint24_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt24.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt24}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint24_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt24.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt24}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint24_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt24.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint24_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint24_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt24.prototype[Symbol.dispose] =
    CompressedFheInt24.prototype.free;

export class CompressedFheInt240 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt240.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt240Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt240Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint240_free(ptr, 0);
  }
  /**
   * @returns {FheInt240}
   */
  decompress() {
    const ret = wasm.compressedfheint240_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt240.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt240}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint240_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt240.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt240}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint240_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt240.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt240}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint240_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt240.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint240_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint240_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt240.prototype[Symbol.dispose] =
    CompressedFheInt240.prototype.free;

export class CompressedFheInt248 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt248.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt248Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt248Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint248_free(ptr, 0);
  }
  /**
   * @returns {FheInt248}
   */
  decompress() {
    const ret = wasm.compressedfheint248_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt248.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt248}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint248_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt248.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt248}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint248_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt248.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt248}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint248_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt248.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint248_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint248_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt248.prototype[Symbol.dispose] =
    CompressedFheInt248.prototype.free;

export class CompressedFheInt256 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt256.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt256Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt256Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint256_free(ptr, 0);
  }
  /**
   * @returns {FheInt256}
   */
  decompress() {
    const ret = wasm.compressedfheint256_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt256.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt256}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint256_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt256.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt256}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint256_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt256.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt256}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint256_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt256.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint256_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint256_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt256.prototype[Symbol.dispose] =
    CompressedFheInt256.prototype.free;

export class CompressedFheInt32 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt32.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt32Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt32Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint32_free(ptr, 0);
  }
  /**
   * @returns {FheInt32}
   */
  decompress() {
    const ret = wasm.compressedfheint32_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt32.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt32}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint32_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt32.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt32}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint32_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt32.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt32}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint32_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt32.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint32_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint32_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt32.prototype[Symbol.dispose] =
    CompressedFheInt32.prototype.free;

export class CompressedFheInt4 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt4.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt4Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt4Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint4_free(ptr, 0);
  }
  /**
   * @returns {FheInt4}
   */
  decompress() {
    const ret = wasm.compressedfheint4_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt4.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt4}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint4_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt4.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt4}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint4_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt4.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt4}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint4_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt4.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint4_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint4_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt4.prototype[Symbol.dispose] =
    CompressedFheInt4.prototype.free;

export class CompressedFheInt40 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt40.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt40Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt40Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint40_free(ptr, 0);
  }
  /**
   * @returns {FheInt40}
   */
  decompress() {
    const ret = wasm.compressedfheint40_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt40.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt40}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint40_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt40.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt40}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint40_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt40.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt40}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint40_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt40.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint40_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint40_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt40.prototype[Symbol.dispose] =
    CompressedFheInt40.prototype.free;

export class CompressedFheInt48 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt48.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt48Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt48Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint48_free(ptr, 0);
  }
  /**
   * @returns {FheInt48}
   */
  decompress() {
    const ret = wasm.compressedfheint48_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt48.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt48}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint48_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt48.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt48}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint48_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt48.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt48}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint48_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt48.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint48_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint48_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt48.prototype[Symbol.dispose] =
    CompressedFheInt48.prototype.free;

export class CompressedFheInt512 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt512.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt512Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt512Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint512_free(ptr, 0);
  }
  /**
   * @returns {FheInt512}
   */
  decompress() {
    const ret = wasm.compressedfheint512_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt512.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt512}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint512_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt512.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt512}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint512_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt512.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt512}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint512_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt512.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint512_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint512_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt512.prototype[Symbol.dispose] =
    CompressedFheInt512.prototype.free;

export class CompressedFheInt56 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt56.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt56Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt56Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint56_free(ptr, 0);
  }
  /**
   * @returns {FheInt56}
   */
  decompress() {
    const ret = wasm.compressedfheint56_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt56.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt56}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint56_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt56.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt56}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint56_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt56.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt56}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint56_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt56.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint56_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint56_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt56.prototype[Symbol.dispose] =
    CompressedFheInt56.prototype.free;

export class CompressedFheInt6 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt6.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt6Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt6Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint6_free(ptr, 0);
  }
  /**
   * @returns {FheInt6}
   */
  decompress() {
    const ret = wasm.compressedfheint6_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt6.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt6}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint6_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt6.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt6}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint6_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt6.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt6}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint6_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt6.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint6_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint6_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt6.prototype[Symbol.dispose] =
    CompressedFheInt6.prototype.free;

export class CompressedFheInt64 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt64.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt64Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt64Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint64_free(ptr, 0);
  }
  /**
   * @returns {FheInt64}
   */
  decompress() {
    const ret = wasm.compressedfheint64_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt64.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt64}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint64_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt64.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt64}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint64_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt64.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt64}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint64_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt64.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint64_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint64_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt64.prototype[Symbol.dispose] =
    CompressedFheInt64.prototype.free;

export class CompressedFheInt72 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt72.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt72Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt72Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint72_free(ptr, 0);
  }
  /**
   * @returns {FheInt72}
   */
  decompress() {
    const ret = wasm.compressedfheint72_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt72.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt72}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint72_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt72.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt72}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint72_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt72.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt72}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint72_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt72.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint72_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint72_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt72.prototype[Symbol.dispose] =
    CompressedFheInt72.prototype.free;

export class CompressedFheInt8 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt8.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt8Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt8Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint8_free(ptr, 0);
  }
  /**
   * @returns {FheInt8}
   */
  decompress() {
    const ret = wasm.compressedfheint8_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt8.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt8}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint8_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt8.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt8}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint8_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt8.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt8}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint8_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt8.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint8_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint8_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt8.prototype[Symbol.dispose] =
    CompressedFheInt8.prototype.free;

export class CompressedFheInt80 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt80.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt80Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt80Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint80_free(ptr, 0);
  }
  /**
   * @returns {FheInt80}
   */
  decompress() {
    const ret = wasm.compressedfheint80_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt80.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt80}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint80_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt80.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt80}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint80_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt80.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt80}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint80_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt80.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint80_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint80_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt80.prototype[Symbol.dispose] =
    CompressedFheInt80.prototype.free;

export class CompressedFheInt88 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt88.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt88Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt88Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint88_free(ptr, 0);
  }
  /**
   * @returns {FheInt88}
   */
  decompress() {
    const ret = wasm.compressedfheint88_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt88.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt88}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint88_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt88.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt88}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint88_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt88.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt88}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint88_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt88.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint88_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint88_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt88.prototype[Symbol.dispose] =
    CompressedFheInt88.prototype.free;

export class CompressedFheInt96 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheInt96.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheInt96Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheInt96Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheint96_free(ptr, 0);
  }
  /**
   * @returns {FheInt96}
   */
  decompress() {
    const ret = wasm.compressedfheint96_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt96.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheInt96}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint96_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt96.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheInt96}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheint96_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt96.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheInt96}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheint96_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheInt96.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheint96_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheint96_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheInt96.prototype[Symbol.dispose] =
    CompressedFheInt96.prototype.free;

export class CompressedFheUint10 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint10.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint10Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint10Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint10_free(ptr, 0);
  }
  /**
   * @returns {FheUint10}
   */
  decompress() {
    const ret = wasm.compressedfheuint10_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint10.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint10}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint10_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint10.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint10}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint10_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint10.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint10}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint10_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint10.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint10_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint10_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint10.prototype[Symbol.dispose] =
    CompressedFheUint10.prototype.free;

export class CompressedFheUint1024 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint1024.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint1024Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint1024Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint1024_free(ptr, 0);
  }
  /**
   * @returns {FheUint1024}
   */
  decompress() {
    const ret = wasm.compressedfheuint1024_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint1024.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint1024}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint1024_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint1024.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint1024}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint1024_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint1024.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint1024}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint1024_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint1024.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint1024_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint1024_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint1024.prototype[Symbol.dispose] =
    CompressedFheUint1024.prototype.free;

export class CompressedFheUint104 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint104.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint104Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint104Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint104_free(ptr, 0);
  }
  /**
   * @returns {FheUint104}
   */
  decompress() {
    const ret = wasm.compressedfheuint104_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint104.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint104}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint104_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint104.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint104}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint104_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint104.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint104}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint104_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint104.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint104_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint104_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint104.prototype[Symbol.dispose] =
    CompressedFheUint104.prototype.free;

export class CompressedFheUint112 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint112.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint112Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint112Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint112_free(ptr, 0);
  }
  /**
   * @returns {FheUint112}
   */
  decompress() {
    const ret = wasm.compressedfheuint112_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint112.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint112}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint112_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint112.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint112}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint112_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint112.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint112}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint112_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint112.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint112_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint112_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint112.prototype[Symbol.dispose] =
    CompressedFheUint112.prototype.free;

export class CompressedFheUint12 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint12.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint12Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint12Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint12_free(ptr, 0);
  }
  /**
   * @returns {FheUint12}
   */
  decompress() {
    const ret = wasm.compressedfheuint12_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint12.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint12}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint12_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint12.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint12}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint12_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint12.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint12}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint12_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint12.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint12_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint12_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint12.prototype[Symbol.dispose] =
    CompressedFheUint12.prototype.free;

export class CompressedFheUint120 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint120.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint120Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint120Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint120_free(ptr, 0);
  }
  /**
   * @returns {FheUint120}
   */
  decompress() {
    const ret = wasm.compressedfheuint120_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint120.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint120}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint120_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint120.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint120}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint120_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint120.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint120}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint120_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint120.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint120_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint120_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint120.prototype[Symbol.dispose] =
    CompressedFheUint120.prototype.free;

export class CompressedFheUint128 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint128.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint128Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint128Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint128_free(ptr, 0);
  }
  /**
   * @returns {FheUint128}
   */
  decompress() {
    const ret = wasm.compressedfheuint128_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint128.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint128}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint128_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint128.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint128}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint128_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint128.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint128}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint128_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint128.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint128_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint128_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint128.prototype[Symbol.dispose] =
    CompressedFheUint128.prototype.free;

export class CompressedFheUint136 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint136.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint136Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint136Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint136_free(ptr, 0);
  }
  /**
   * @returns {FheUint136}
   */
  decompress() {
    const ret = wasm.compressedfheuint136_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint136.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint136}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint136_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint136.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint136}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint136_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint136.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint136}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint136_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint136.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint136_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint136_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint136.prototype[Symbol.dispose] =
    CompressedFheUint136.prototype.free;

export class CompressedFheUint14 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint14.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint14Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint14Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint14_free(ptr, 0);
  }
  /**
   * @returns {FheUint14}
   */
  decompress() {
    const ret = wasm.compressedfheuint14_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint14.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint14}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint14_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint14.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint14}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint14_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint14.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint14}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint14_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint14.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint14_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint14_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint14.prototype[Symbol.dispose] =
    CompressedFheUint14.prototype.free;

export class CompressedFheUint144 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint144.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint144Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint144Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint144_free(ptr, 0);
  }
  /**
   * @returns {FheUint144}
   */
  decompress() {
    const ret = wasm.compressedfheuint144_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint144.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint144}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint144_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint144.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint144}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint144_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint144.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint144}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint144_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint144.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint144_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint144_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint144.prototype[Symbol.dispose] =
    CompressedFheUint144.prototype.free;

export class CompressedFheUint152 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint152.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint152Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint152Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint152_free(ptr, 0);
  }
  /**
   * @returns {FheUint152}
   */
  decompress() {
    const ret = wasm.compressedfheuint152_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint152.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint152}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint152_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint152.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint152}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint152_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint152.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint152}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint152_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint152.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint152_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint152_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint152.prototype[Symbol.dispose] =
    CompressedFheUint152.prototype.free;

export class CompressedFheUint16 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint16.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint16Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint16Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint16_free(ptr, 0);
  }
  /**
   * @returns {FheUint16}
   */
  decompress() {
    const ret = wasm.compressedfheuint16_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint16.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint16}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint16_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint16.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint16}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint16_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint16.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint16}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint16_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint16.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint16_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint16_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint16.prototype[Symbol.dispose] =
    CompressedFheUint16.prototype.free;

export class CompressedFheUint160 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint160.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint160Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint160Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint160_free(ptr, 0);
  }
  /**
   * @returns {FheUint160}
   */
  decompress() {
    const ret = wasm.compressedfheuint160_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint160.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint160}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint160_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint160.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint160}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint160_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint160.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint160}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint160_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint160.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint160_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint160_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint160.prototype[Symbol.dispose] =
    CompressedFheUint160.prototype.free;

export class CompressedFheUint168 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint168.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint168Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint168Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint168_free(ptr, 0);
  }
  /**
   * @returns {FheUint168}
   */
  decompress() {
    const ret = wasm.compressedfheuint168_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint168.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint168}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint168_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint168.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint168}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint168_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint168.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint168}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint168_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint168.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint168_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint168_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint168.prototype[Symbol.dispose] =
    CompressedFheUint168.prototype.free;

export class CompressedFheUint176 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint176.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint176Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint176Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint176_free(ptr, 0);
  }
  /**
   * @returns {FheUint176}
   */
  decompress() {
    const ret = wasm.compressedfheuint176_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint176.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint176}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint176_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint176.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint176}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint176_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint176.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint176}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint176_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint176.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint176_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint176_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint176.prototype[Symbol.dispose] =
    CompressedFheUint176.prototype.free;

export class CompressedFheUint184 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint184.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint184Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint184Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint184_free(ptr, 0);
  }
  /**
   * @returns {FheUint184}
   */
  decompress() {
    const ret = wasm.compressedfheuint184_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint184.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint184}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint184_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint184.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint184}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint184_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint184.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint184}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint184_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint184.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint184_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint184_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint184.prototype[Symbol.dispose] =
    CompressedFheUint184.prototype.free;

export class CompressedFheUint192 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint192.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint192Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint192Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint192_free(ptr, 0);
  }
  /**
   * @returns {FheUint192}
   */
  decompress() {
    const ret = wasm.compressedfheuint192_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint192.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint192}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint192_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint192.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint192}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint192_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint192.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint192}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint192_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint192.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint192_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint192_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint192.prototype[Symbol.dispose] =
    CompressedFheUint192.prototype.free;

export class CompressedFheUint2 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint2.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint2Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint2Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint2_free(ptr, 0);
  }
  /**
   * @returns {FheUint2}
   */
  decompress() {
    const ret = wasm.compressedfheuint2_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint2.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint2}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint2_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint2.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint2}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint2_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint2.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint2}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint2_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint2.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint2_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint2_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint2.prototype[Symbol.dispose] =
    CompressedFheUint2.prototype.free;

export class CompressedFheUint200 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint200.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint200Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint200Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint200_free(ptr, 0);
  }
  /**
   * @returns {FheUint200}
   */
  decompress() {
    const ret = wasm.compressedfheuint200_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint200.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint200}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint200_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint200.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint200}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint200_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint200.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint200}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint200_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint200.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint200_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint200_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint200.prototype[Symbol.dispose] =
    CompressedFheUint200.prototype.free;

export class CompressedFheUint2048 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint2048.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint2048Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint2048Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint2048_free(ptr, 0);
  }
  /**
   * @returns {FheUint2048}
   */
  decompress() {
    const ret = wasm.compressedfheuint2048_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint2048.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint2048}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint2048_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint2048.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint2048}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint2048_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint2048.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint2048}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint2048_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint2048.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint2048_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint2048_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint2048.prototype[Symbol.dispose] =
    CompressedFheUint2048.prototype.free;

export class CompressedFheUint208 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint208.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint208Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint208Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint208_free(ptr, 0);
  }
  /**
   * @returns {FheUint208}
   */
  decompress() {
    const ret = wasm.compressedfheuint208_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint208.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint208}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint208_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint208.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint208}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint208_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint208.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint208}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint208_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint208.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint208_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint208_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint208.prototype[Symbol.dispose] =
    CompressedFheUint208.prototype.free;

export class CompressedFheUint216 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint216.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint216Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint216Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint216_free(ptr, 0);
  }
  /**
   * @returns {FheUint216}
   */
  decompress() {
    const ret = wasm.compressedfheuint216_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint216.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint216}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint216_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint216.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint216}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint216_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint216.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint216}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint216_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint216.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint216_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint216_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint216.prototype[Symbol.dispose] =
    CompressedFheUint216.prototype.free;

export class CompressedFheUint224 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint224.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint224Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint224Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint224_free(ptr, 0);
  }
  /**
   * @returns {FheUint224}
   */
  decompress() {
    const ret = wasm.compressedfheuint224_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint224.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint224}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint224_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint224.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint224}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint224_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint224.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint224}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint224_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint224.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint224_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint224_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint224.prototype[Symbol.dispose] =
    CompressedFheUint224.prototype.free;

export class CompressedFheUint232 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint232.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint232Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint232Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint232_free(ptr, 0);
  }
  /**
   * @returns {FheUint232}
   */
  decompress() {
    const ret = wasm.compressedfheuint232_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint232.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint232}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint232_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint232.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint232}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint232_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint232.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint232}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint232_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint232.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint232_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint232_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint232.prototype[Symbol.dispose] =
    CompressedFheUint232.prototype.free;

export class CompressedFheUint24 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint24.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint24Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint24Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint24_free(ptr, 0);
  }
  /**
   * @returns {FheUint24}
   */
  decompress() {
    const ret = wasm.compressedfheuint24_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint24.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint24}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint24_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint24.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint24}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint24_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint24.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint24}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint24_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint24.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint24_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint24_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint24.prototype[Symbol.dispose] =
    CompressedFheUint24.prototype.free;

export class CompressedFheUint240 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint240.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint240Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint240Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint240_free(ptr, 0);
  }
  /**
   * @returns {FheUint240}
   */
  decompress() {
    const ret = wasm.compressedfheuint240_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint240.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint240}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint240_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint240.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint240}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint240_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint240.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint240}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint240_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint240.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint240_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint240_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint240.prototype[Symbol.dispose] =
    CompressedFheUint240.prototype.free;

export class CompressedFheUint248 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint248.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint248Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint248Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint248_free(ptr, 0);
  }
  /**
   * @returns {FheUint248}
   */
  decompress() {
    const ret = wasm.compressedfheuint248_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint248.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint248}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint248_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint248.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint248}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint248_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint248.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint248}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint248_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint248.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint248_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint248_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint248.prototype[Symbol.dispose] =
    CompressedFheUint248.prototype.free;

export class CompressedFheUint256 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint256.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint256Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint256Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint256_free(ptr, 0);
  }
  /**
   * @returns {FheUint256}
   */
  decompress() {
    const ret = wasm.compressedfheuint256_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint256.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint256}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint256_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint256.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint256}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint256_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint256.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint256}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint256_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint256.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint256_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint256_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint256.prototype[Symbol.dispose] =
    CompressedFheUint256.prototype.free;

export class CompressedFheUint32 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint32.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint32Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint32Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint32_free(ptr, 0);
  }
  /**
   * @returns {FheUint32}
   */
  decompress() {
    const ret = wasm.compressedfheuint32_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint32.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint32}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint32_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint32.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint32}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint32_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint32.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint32}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint32_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint32.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint32_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint32_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint32.prototype[Symbol.dispose] =
    CompressedFheUint32.prototype.free;

export class CompressedFheUint4 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint4.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint4Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint4Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint4_free(ptr, 0);
  }
  /**
   * @returns {FheUint4}
   */
  decompress() {
    const ret = wasm.compressedfheuint4_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint4.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint4}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint4_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint4.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint4}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint4_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint4.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint4}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint4_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint4.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint4_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint4_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint4.prototype[Symbol.dispose] =
    CompressedFheUint4.prototype.free;

export class CompressedFheUint40 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint40.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint40Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint40Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint40_free(ptr, 0);
  }
  /**
   * @returns {FheUint40}
   */
  decompress() {
    const ret = wasm.compressedfheuint40_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint40.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint40}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint40_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint40.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint40}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint40_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint40.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint40}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint40_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint40.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint40_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint40_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint40.prototype[Symbol.dispose] =
    CompressedFheUint40.prototype.free;

export class CompressedFheUint48 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint48.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint48Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint48Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint48_free(ptr, 0);
  }
  /**
   * @returns {FheUint48}
   */
  decompress() {
    const ret = wasm.compressedfheuint48_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint48.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint48}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint48_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint48.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint48}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint48_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint48.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint48}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint48_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint48.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint48_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint48_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint48.prototype[Symbol.dispose] =
    CompressedFheUint48.prototype.free;

export class CompressedFheUint512 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint512.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint512Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint512Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint512_free(ptr, 0);
  }
  /**
   * @returns {FheUint512}
   */
  decompress() {
    const ret = wasm.compressedfheuint512_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint512.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint512}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint512_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint512.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint512}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint512_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint512.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint512}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint512_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint512.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint512_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint512_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint512.prototype[Symbol.dispose] =
    CompressedFheUint512.prototype.free;

export class CompressedFheUint56 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint56.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint56Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint56Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint56_free(ptr, 0);
  }
  /**
   * @returns {FheUint56}
   */
  decompress() {
    const ret = wasm.compressedfheuint56_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint56.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint56}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint56_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint56.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint56}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint56_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint56.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint56}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint56_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint56.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint56_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint56_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint56.prototype[Symbol.dispose] =
    CompressedFheUint56.prototype.free;

export class CompressedFheUint6 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint6.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint6Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint6Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint6_free(ptr, 0);
  }
  /**
   * @returns {FheUint6}
   */
  decompress() {
    const ret = wasm.compressedfheuint6_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint6.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint6}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint6_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint6.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint6}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint6_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint6.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint6}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint6_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint6.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint6_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint6_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint6.prototype[Symbol.dispose] =
    CompressedFheUint6.prototype.free;

export class CompressedFheUint64 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint64.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint64Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint64Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint64_free(ptr, 0);
  }
  /**
   * @returns {FheUint64}
   */
  decompress() {
    const ret = wasm.compressedfheuint64_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint64.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint64}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint64_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint64.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint64}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint64_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint64.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint64}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint64_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint64.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint64_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint64_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint64.prototype[Symbol.dispose] =
    CompressedFheUint64.prototype.free;

export class CompressedFheUint72 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint72.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint72Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint72Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint72_free(ptr, 0);
  }
  /**
   * @returns {FheUint72}
   */
  decompress() {
    const ret = wasm.compressedfheuint72_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint72.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint72}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint72_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint72.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint72}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint72_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint72.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint72}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint72_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint72.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint72_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint72_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint72.prototype[Symbol.dispose] =
    CompressedFheUint72.prototype.free;

export class CompressedFheUint8 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint8.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint8Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint8Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint8_free(ptr, 0);
  }
  /**
   * @returns {FheUint8}
   */
  decompress() {
    const ret = wasm.compressedfheuint8_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint8.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint8}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint8_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint8.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint8}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint8_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint8.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint8}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint8_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint8.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint8_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint8_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint8.prototype[Symbol.dispose] =
    CompressedFheUint8.prototype.free;

export class CompressedFheUint80 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint80.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint80Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint80Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint80_free(ptr, 0);
  }
  /**
   * @returns {FheUint80}
   */
  decompress() {
    const ret = wasm.compressedfheuint80_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint80.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint80}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint80_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint80.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint80}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint80_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint80.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint80}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint80_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint80.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint80_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint80_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint80.prototype[Symbol.dispose] =
    CompressedFheUint80.prototype.free;

export class CompressedFheUint88 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint88.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint88Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint88Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint88_free(ptr, 0);
  }
  /**
   * @returns {FheUint88}
   */
  decompress() {
    const ret = wasm.compressedfheuint88_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint88.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint88}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint88_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint88.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint88}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint88_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint88.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint88}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint88_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint88.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint88_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint88_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint88.prototype[Symbol.dispose] =
    CompressedFheUint88.prototype.free;

export class CompressedFheUint96 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(CompressedFheUint96.prototype);
    obj.__wbg_ptr = ptr;
    CompressedFheUint96Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CompressedFheUint96Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_compressedfheuint96_free(ptr, 0);
  }
  /**
   * @returns {FheUint96}
   */
  decompress() {
    const ret = wasm.compressedfheuint96_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint96.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {CompressedFheUint96}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint96_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint96.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {CompressedFheUint96}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.compressedfheuint96_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint96.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {CompressedFheUint96}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compressedfheuint96_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompressedFheUint96.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.compressedfheuint96_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.compressedfheuint96_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  CompressedFheUint96.prototype[Symbol.dispose] =
    CompressedFheUint96.prototype.free;

export class FheBool {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheBool.prototype);
    obj.__wbg_ptr = ptr;
    FheBoolFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheBoolFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fhebool_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {boolean}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fhebool_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheBool}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fhebool_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheBool.__wrap(ret[0]);
  }
  /**
   * @param {boolean} value
   * @param {TfheClientKey} client_key
   * @returns {FheBool}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fhebool_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheBool.__wrap(ret[0]);
  }
  /**
   * @param {boolean} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheBool}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fhebool_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheBool.__wrap(ret[0]);
  }
  /**
   * @param {boolean} value
   * @param {TfhePublicKey} public_key
   * @returns {FheBool}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fhebool_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheBool.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheBool}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fhebool_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheBool.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fhebool_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fhebool_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose) FheBool.prototype[Symbol.dispose] = FheBool.prototype.free;

export class FheInt10 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt10.prototype);
    obj.__wbg_ptr = ptr;
    FheInt10Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt10Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint10_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint10_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt10}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint10_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt10.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt10}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint10_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt10.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt10}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint10_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt10.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt10}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint10_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt10.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt10}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint10_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt10.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint10_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint10_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt10.prototype[Symbol.dispose] = FheInt10.prototype.free;

export class FheInt1024 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt1024.prototype);
    obj.__wbg_ptr = ptr;
    FheInt1024Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt1024Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint1024_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint1024_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt1024}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint1024_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt1024.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt1024}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint1024_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt1024.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt1024}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint1024_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt1024.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt1024}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint1024_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt1024.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt1024}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint1024_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt1024.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint1024_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint1024_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt1024.prototype[Symbol.dispose] = FheInt1024.prototype.free;

export class FheInt104 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt104.prototype);
    obj.__wbg_ptr = ptr;
    FheInt104Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt104Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint104_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint104_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt104}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint104_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt104.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt104}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint104_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt104.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt104}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint104_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt104.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt104}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint104_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt104.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt104}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint104_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt104.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint104_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint104_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt104.prototype[Symbol.dispose] = FheInt104.prototype.free;

export class FheInt112 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt112.prototype);
    obj.__wbg_ptr = ptr;
    FheInt112Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt112Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint112_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint112_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt112}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint112_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt112.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt112}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint112_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt112.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt112}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint112_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt112.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt112}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint112_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt112.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt112}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint112_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt112.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint112_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint112_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt112.prototype[Symbol.dispose] = FheInt112.prototype.free;

export class FheInt12 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt12.prototype);
    obj.__wbg_ptr = ptr;
    FheInt12Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt12Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint12_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint12_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt12}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint12_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt12.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt12}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint12_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt12.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt12}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint12_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt12.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt12}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint12_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt12.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt12}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint12_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt12.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint12_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint12_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt12.prototype[Symbol.dispose] = FheInt12.prototype.free;

export class FheInt120 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt120.prototype);
    obj.__wbg_ptr = ptr;
    FheInt120Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt120Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint120_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint120_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt120}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint120_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt120.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt120}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint120_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt120.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt120}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint120_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt120.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt120}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint120_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt120.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt120}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint120_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt120.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint120_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint120_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt120.prototype[Symbol.dispose] = FheInt120.prototype.free;

export class FheInt128 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt128.prototype);
    obj.__wbg_ptr = ptr;
    FheInt128Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt128Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint128_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint128_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt128}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint128_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt128.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt128}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint128_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt128.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt128}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint128_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt128.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt128}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint128_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt128.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt128}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint128_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt128.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint128_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint128_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt128.prototype[Symbol.dispose] = FheInt128.prototype.free;

export class FheInt136 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt136.prototype);
    obj.__wbg_ptr = ptr;
    FheInt136Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt136Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint136_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint136_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt136}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint136_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt136.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt136}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint136_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt136.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt136}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint136_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt136.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt136}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint136_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt136.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt136}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint136_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt136.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint136_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint136_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt136.prototype[Symbol.dispose] = FheInt136.prototype.free;

export class FheInt14 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt14.prototype);
    obj.__wbg_ptr = ptr;
    FheInt14Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt14Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint14_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint14_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt14}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint14_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt14.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt14}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint14_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt14.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt14}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint14_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt14.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt14}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint14_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt14.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt14}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint14_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt14.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint14_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint14_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt14.prototype[Symbol.dispose] = FheInt14.prototype.free;

export class FheInt144 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt144.prototype);
    obj.__wbg_ptr = ptr;
    FheInt144Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt144Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint144_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint144_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt144}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint144_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt144.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt144}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint144_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt144.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt144}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint144_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt144.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt144}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint144_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt144.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt144}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint144_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt144.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint144_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint144_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt144.prototype[Symbol.dispose] = FheInt144.prototype.free;

export class FheInt152 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt152.prototype);
    obj.__wbg_ptr = ptr;
    FheInt152Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt152Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint152_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint152_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt152}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint152_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt152.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt152}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint152_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt152.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt152}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint152_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt152.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt152}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint152_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt152.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt152}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint152_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt152.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint152_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint152_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt152.prototype[Symbol.dispose] = FheInt152.prototype.free;

export class FheInt16 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt16.prototype);
    obj.__wbg_ptr = ptr;
    FheInt16Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt16Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint16_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint16_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt16}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint16_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt16.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt16}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint16_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt16.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt16}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint16_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt16.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt16}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint16_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt16.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt16}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint16_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt16.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint16_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint16_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt16.prototype[Symbol.dispose] = FheInt16.prototype.free;

export class FheInt160 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt160.prototype);
    obj.__wbg_ptr = ptr;
    FheInt160Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt160Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint160_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint160_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt160}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint160_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt160.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt160}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint160_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt160.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt160}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint160_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt160.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt160}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint160_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt160.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt160}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint160_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt160.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint160_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint160_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt160.prototype[Symbol.dispose] = FheInt160.prototype.free;

export class FheInt168 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt168.prototype);
    obj.__wbg_ptr = ptr;
    FheInt168Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt168Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint168_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint168_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt168}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint168_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt168.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt168}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint168_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt168.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt168}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint168_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt168.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt168}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint168_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt168.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt168}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint168_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt168.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint168_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint168_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt168.prototype[Symbol.dispose] = FheInt168.prototype.free;

export class FheInt176 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt176.prototype);
    obj.__wbg_ptr = ptr;
    FheInt176Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt176Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint176_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint176_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt176}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint176_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt176.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt176}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint176_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt176.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt176}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint176_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt176.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt176}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint176_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt176.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt176}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint176_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt176.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint176_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint176_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt176.prototype[Symbol.dispose] = FheInt176.prototype.free;

export class FheInt184 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt184.prototype);
    obj.__wbg_ptr = ptr;
    FheInt184Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt184Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint184_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint184_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt184}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint184_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt184.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt184}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint184_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt184.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt184}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint184_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt184.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt184}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint184_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt184.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt184}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint184_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt184.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint184_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint184_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt184.prototype[Symbol.dispose] = FheInt184.prototype.free;

export class FheInt192 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt192.prototype);
    obj.__wbg_ptr = ptr;
    FheInt192Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt192Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint192_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint192_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt192}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint192_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt192.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt192}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint192_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt192.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt192}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint192_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt192.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt192}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint192_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt192.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt192}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint192_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt192.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint192_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint192_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt192.prototype[Symbol.dispose] = FheInt192.prototype.free;

export class FheInt2 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt2.prototype);
    obj.__wbg_ptr = ptr;
    FheInt2Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt2Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint2_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint2_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt2}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint2_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt2.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt2}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint2_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt2.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt2}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint2_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt2.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt2}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint2_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt2.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt2}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint2_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt2.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint2_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint2_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose) FheInt2.prototype[Symbol.dispose] = FheInt2.prototype.free;

export class FheInt200 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt200.prototype);
    obj.__wbg_ptr = ptr;
    FheInt200Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt200Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint200_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint200_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt200}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint200_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt200.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt200}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint200_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt200.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt200}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint200_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt200.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt200}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint200_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt200.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt200}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint200_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt200.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint200_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint200_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt200.prototype[Symbol.dispose] = FheInt200.prototype.free;

export class FheInt2048 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt2048.prototype);
    obj.__wbg_ptr = ptr;
    FheInt2048Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt2048Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint2048_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint2048_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt2048}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint2048_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt2048.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt2048}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint2048_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt2048.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt2048}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint2048_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt2048.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt2048}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint2048_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt2048.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt2048}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint2048_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt2048.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint2048_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint2048_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt2048.prototype[Symbol.dispose] = FheInt2048.prototype.free;

export class FheInt208 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt208.prototype);
    obj.__wbg_ptr = ptr;
    FheInt208Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt208Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint208_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint208_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt208}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint208_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt208.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt208}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint208_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt208.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt208}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint208_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt208.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt208}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint208_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt208.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt208}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint208_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt208.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint208_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint208_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt208.prototype[Symbol.dispose] = FheInt208.prototype.free;

export class FheInt216 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt216.prototype);
    obj.__wbg_ptr = ptr;
    FheInt216Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt216Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint216_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint216_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt216}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint216_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt216.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt216}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint216_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt216.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt216}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint216_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt216.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt216}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint216_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt216.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt216}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint216_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt216.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint216_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint216_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt216.prototype[Symbol.dispose] = FheInt216.prototype.free;

export class FheInt224 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt224.prototype);
    obj.__wbg_ptr = ptr;
    FheInt224Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt224Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint224_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint224_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt224}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint224_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt224.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt224}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint224_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt224.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt224}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint224_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt224.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt224}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint224_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt224.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt224}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint224_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt224.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint224_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint224_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt224.prototype[Symbol.dispose] = FheInt224.prototype.free;

export class FheInt232 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt232.prototype);
    obj.__wbg_ptr = ptr;
    FheInt232Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt232Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint232_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint232_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt232}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint232_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt232.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt232}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint232_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt232.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt232}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint232_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt232.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt232}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint232_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt232.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt232}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint232_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt232.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint232_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint232_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt232.prototype[Symbol.dispose] = FheInt232.prototype.free;

export class FheInt24 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt24.prototype);
    obj.__wbg_ptr = ptr;
    FheInt24Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt24Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint24_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint24_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt24}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint24_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt24.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt24}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint24_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt24.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt24}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint24_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt24.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt24}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint24_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt24.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt24}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint24_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt24.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint24_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint24_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt24.prototype[Symbol.dispose] = FheInt24.prototype.free;

export class FheInt240 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt240.prototype);
    obj.__wbg_ptr = ptr;
    FheInt240Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt240Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint240_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint240_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt240}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint240_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt240.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt240}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint240_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt240.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt240}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint240_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt240.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt240}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint240_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt240.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt240}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint240_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt240.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint240_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint240_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt240.prototype[Symbol.dispose] = FheInt240.prototype.free;

export class FheInt248 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt248.prototype);
    obj.__wbg_ptr = ptr;
    FheInt248Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt248Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint248_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint248_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt248}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint248_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt248.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt248}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint248_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt248.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt248}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint248_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt248.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt248}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint248_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt248.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt248}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint248_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt248.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint248_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint248_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt248.prototype[Symbol.dispose] = FheInt248.prototype.free;

export class FheInt256 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt256.prototype);
    obj.__wbg_ptr = ptr;
    FheInt256Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt256Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint256_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint256_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt256}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint256_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt256.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt256}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint256_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt256.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt256}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint256_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt256.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt256}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint256_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt256.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt256}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint256_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt256.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint256_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint256_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt256.prototype[Symbol.dispose] = FheInt256.prototype.free;

export class FheInt32 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt32.prototype);
    obj.__wbg_ptr = ptr;
    FheInt32Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt32Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint32_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint32_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt32}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint32_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt32.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt32}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint32_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt32.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt32}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint32_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt32.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt32}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint32_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt32.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt32}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint32_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt32.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint32_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint32_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt32.prototype[Symbol.dispose] = FheInt32.prototype.free;

export class FheInt4 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt4.prototype);
    obj.__wbg_ptr = ptr;
    FheInt4Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt4Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint4_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint4_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt4}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint4_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt4.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt4}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint4_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt4.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt4}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint4_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt4.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt4}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint4_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt4.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt4}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint4_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt4.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint4_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint4_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose) FheInt4.prototype[Symbol.dispose] = FheInt4.prototype.free;

export class FheInt40 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt40.prototype);
    obj.__wbg_ptr = ptr;
    FheInt40Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt40Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint40_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {bigint}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint40_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt40}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint40_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt40.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt40}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint40_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt40.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt40}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint40_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt40.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt40}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint40_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt40.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt40}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint40_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt40.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint40_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint40_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt40.prototype[Symbol.dispose] = FheInt40.prototype.free;

export class FheInt48 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt48.prototype);
    obj.__wbg_ptr = ptr;
    FheInt48Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt48Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint48_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {bigint}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint48_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt48}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint48_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt48.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt48}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint48_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt48.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt48}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint48_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt48.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt48}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint48_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt48.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt48}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint48_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt48.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint48_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint48_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt48.prototype[Symbol.dispose] = FheInt48.prototype.free;

export class FheInt512 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt512.prototype);
    obj.__wbg_ptr = ptr;
    FheInt512Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt512Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint512_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint512_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt512}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint512_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt512.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt512}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint512_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt512.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt512}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint512_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt512.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt512}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint512_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt512.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt512}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint512_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt512.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint512_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint512_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt512.prototype[Symbol.dispose] = FheInt512.prototype.free;

export class FheInt56 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt56.prototype);
    obj.__wbg_ptr = ptr;
    FheInt56Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt56Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint56_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {bigint}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint56_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt56}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint56_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt56.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt56}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint56_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt56.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt56}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint56_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt56.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt56}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint56_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt56.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt56}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint56_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt56.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint56_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint56_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt56.prototype[Symbol.dispose] = FheInt56.prototype.free;

export class FheInt6 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt6.prototype);
    obj.__wbg_ptr = ptr;
    FheInt6Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt6Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint6_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint6_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt6}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint6_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt6.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt6}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint6_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt6.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt6}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint6_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt6.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt6}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint6_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt6.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt6}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint6_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt6.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint6_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint6_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose) FheInt6.prototype[Symbol.dispose] = FheInt6.prototype.free;

export class FheInt64 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt64.prototype);
    obj.__wbg_ptr = ptr;
    FheInt64Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt64Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint64_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {bigint}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint64_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt64}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint64_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt64.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt64}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint64_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt64.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt64}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint64_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt64.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt64}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint64_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt64.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt64}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint64_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt64.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint64_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint64_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt64.prototype[Symbol.dispose] = FheInt64.prototype.free;

export class FheInt72 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt72.prototype);
    obj.__wbg_ptr = ptr;
    FheInt72Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt72Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint72_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint72_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt72}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint72_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt72.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt72}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint72_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt72.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt72}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint72_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt72.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt72}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint72_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt72.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt72}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint72_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt72.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint72_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint72_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt72.prototype[Symbol.dispose] = FheInt72.prototype.free;

export class FheInt8 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt8.prototype);
    obj.__wbg_ptr = ptr;
    FheInt8Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt8Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint8_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint8_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt8}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint8_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt8.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt8}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint8_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt8.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt8}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint8_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt8.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt8}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint8_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt8.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt8}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint8_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt8.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint8_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint8_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose) FheInt8.prototype[Symbol.dispose] = FheInt8.prototype.free;

export class FheInt80 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt80.prototype);
    obj.__wbg_ptr = ptr;
    FheInt80Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt80Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint80_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint80_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt80}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint80_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt80.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt80}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint80_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt80.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt80}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint80_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt80.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt80}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint80_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt80.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt80}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint80_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt80.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint80_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint80_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt80.prototype[Symbol.dispose] = FheInt80.prototype.free;

export class FheInt88 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt88.prototype);
    obj.__wbg_ptr = ptr;
    FheInt88Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt88Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint88_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint88_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt88}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint88_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt88.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt88}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint88_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt88.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt88}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint88_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt88.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt88}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint88_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt88.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt88}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint88_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt88.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint88_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint88_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt88.prototype[Symbol.dispose] = FheInt88.prototype.free;

export class FheInt96 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheInt96.prototype);
    obj.__wbg_ptr = ptr;
    FheInt96Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheInt96Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheint96_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint96_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheInt96}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint96_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt96.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheInt96}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheint96_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt96.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheInt96}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheint96_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt96.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheInt96}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheint96_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt96.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheInt96}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheint96_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheInt96.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheint96_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheint96_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheInt96.prototype[Symbol.dispose] = FheInt96.prototype.free;

/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83}
 */
export const FheTypes = Object.freeze({
  Bool: 0,
  0: "Bool",
  Uint4: 1,
  1: "Uint4",
  Uint8: 2,
  2: "Uint8",
  Uint16: 3,
  3: "Uint16",
  Uint32: 4,
  4: "Uint32",
  Uint64: 5,
  5: "Uint64",
  Uint128: 6,
  6: "Uint128",
  Uint160: 7,
  7: "Uint160",
  Uint256: 8,
  8: "Uint256",
  Uint512: 9,
  9: "Uint512",
  Uint1024: 10,
  10: "Uint1024",
  Uint2048: 11,
  11: "Uint2048",
  Uint2: 12,
  12: "Uint2",
  Uint6: 13,
  13: "Uint6",
  Uint10: 14,
  14: "Uint10",
  Uint12: 15,
  15: "Uint12",
  Uint14: 16,
  16: "Uint14",
  Int2: 17,
  17: "Int2",
  Int4: 18,
  18: "Int4",
  Int6: 19,
  19: "Int6",
  Int8: 20,
  20: "Int8",
  Int10: 21,
  21: "Int10",
  Int12: 22,
  22: "Int12",
  Int14: 23,
  23: "Int14",
  Int16: 24,
  24: "Int16",
  Int32: 25,
  25: "Int32",
  Int64: 26,
  26: "Int64",
  Int128: 27,
  27: "Int128",
  Int160: 28,
  28: "Int160",
  Int256: 29,
  29: "Int256",
  AsciiString: 30,
  30: "AsciiString",
  Int512: 31,
  31: "Int512",
  Int1024: 32,
  32: "Int1024",
  Int2048: 33,
  33: "Int2048",
  Uint24: 34,
  34: "Uint24",
  Uint40: 35,
  35: "Uint40",
  Uint48: 36,
  36: "Uint48",
  Uint56: 37,
  37: "Uint56",
  Uint72: 38,
  38: "Uint72",
  Uint80: 39,
  39: "Uint80",
  Uint88: 40,
  40: "Uint88",
  Uint96: 41,
  41: "Uint96",
  Uint104: 42,
  42: "Uint104",
  Uint112: 43,
  43: "Uint112",
  Uint120: 44,
  44: "Uint120",
  Uint136: 45,
  45: "Uint136",
  Uint144: 46,
  46: "Uint144",
  Uint152: 47,
  47: "Uint152",
  Uint168: 48,
  48: "Uint168",
  Uint176: 49,
  49: "Uint176",
  Uint184: 50,
  50: "Uint184",
  Uint192: 51,
  51: "Uint192",
  Uint200: 52,
  52: "Uint200",
  Uint208: 53,
  53: "Uint208",
  Uint216: 54,
  54: "Uint216",
  Uint224: 55,
  55: "Uint224",
  Uint232: 56,
  56: "Uint232",
  Uint240: 57,
  57: "Uint240",
  Uint248: 58,
  58: "Uint248",
  Int24: 59,
  59: "Int24",
  Int40: 60,
  60: "Int40",
  Int48: 61,
  61: "Int48",
  Int56: 62,
  62: "Int56",
  Int72: 63,
  63: "Int72",
  Int80: 64,
  64: "Int80",
  Int88: 65,
  65: "Int88",
  Int96: 66,
  66: "Int96",
  Int104: 67,
  67: "Int104",
  Int112: 68,
  68: "Int112",
  Int120: 69,
  69: "Int120",
  Int136: 70,
  70: "Int136",
  Int144: 71,
  71: "Int144",
  Int152: 72,
  72: "Int152",
  Int168: 73,
  73: "Int168",
  Int176: 74,
  74: "Int176",
  Int184: 75,
  75: "Int184",
  Int192: 76,
  76: "Int192",
  Int200: 77,
  77: "Int200",
  Int208: 78,
  78: "Int208",
  Int216: 79,
  79: "Int216",
  Int224: 80,
  80: "Int224",
  Int232: 81,
  81: "Int232",
  Int240: 82,
  82: "Int240",
  Int248: 83,
  83: "Int248",
});

export class FheUint10 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint10.prototype);
    obj.__wbg_ptr = ptr;
    FheUint10Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint10Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint10_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint10_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint10}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint10_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint10.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint10}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint10_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint10.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint10}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint10_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint10.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint10}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint10_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint10.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint10}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint10_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint10.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint10_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint10_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint10.prototype[Symbol.dispose] = FheUint10.prototype.free;

export class FheUint1024 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint1024.prototype);
    obj.__wbg_ptr = ptr;
    FheUint1024Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint1024Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint1024_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint1024_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint1024}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint1024_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint1024.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint1024}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint1024_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint1024.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint1024}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint1024_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint1024.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint1024}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint1024_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint1024.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint1024}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint1024_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint1024.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint1024_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint1024_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint1024.prototype[Symbol.dispose] = FheUint1024.prototype.free;

export class FheUint104 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint104.prototype);
    obj.__wbg_ptr = ptr;
    FheUint104Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint104Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint104_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint104_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint104}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint104_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint104.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint104}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint104_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint104.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint104}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint104_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint104.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint104}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint104_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint104.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint104}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint104_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint104.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint104_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint104_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint104.prototype[Symbol.dispose] = FheUint104.prototype.free;

export class FheUint112 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint112.prototype);
    obj.__wbg_ptr = ptr;
    FheUint112Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint112Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint112_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint112_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint112}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint112_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint112.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint112}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint112_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint112.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint112}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint112_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint112.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint112}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint112_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint112.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint112}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint112_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint112.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint112_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint112_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint112.prototype[Symbol.dispose] = FheUint112.prototype.free;

export class FheUint12 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint12.prototype);
    obj.__wbg_ptr = ptr;
    FheUint12Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint12Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint12_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint12_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint12}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint12_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint12.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint12}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint12_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint12.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint12}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint12_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint12.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint12}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint12_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint12.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint12}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint12_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint12.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint12_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint12_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint12.prototype[Symbol.dispose] = FheUint12.prototype.free;

export class FheUint120 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint120.prototype);
    obj.__wbg_ptr = ptr;
    FheUint120Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint120Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint120_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint120_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint120}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint120_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint120.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint120}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint120_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint120.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint120}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint120_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint120.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint120}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint120_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint120.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint120}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint120_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint120.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint120_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint120_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint120.prototype[Symbol.dispose] = FheUint120.prototype.free;

export class FheUint128 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint128.prototype);
    obj.__wbg_ptr = ptr;
    FheUint128Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint128Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint128_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint128_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint128}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint128_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint128.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint128}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint128_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint128.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint128}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint128_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint128.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint128}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint128_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint128.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint128}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint128_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint128.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint128_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint128_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint128.prototype[Symbol.dispose] = FheUint128.prototype.free;

export class FheUint136 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint136.prototype);
    obj.__wbg_ptr = ptr;
    FheUint136Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint136Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint136_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint136_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint136}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint136_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint136.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint136}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint136_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint136.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint136}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint136_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint136.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint136}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint136_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint136.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint136}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint136_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint136.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint136_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint136_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint136.prototype[Symbol.dispose] = FheUint136.prototype.free;

export class FheUint14 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint14.prototype);
    obj.__wbg_ptr = ptr;
    FheUint14Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint14Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint14_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint14_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint14}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint14_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint14.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint14}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint14_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint14.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint14}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint14_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint14.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint14}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint14_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint14.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint14}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint14_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint14.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint14_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint14_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint14.prototype[Symbol.dispose] = FheUint14.prototype.free;

export class FheUint144 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint144.prototype);
    obj.__wbg_ptr = ptr;
    FheUint144Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint144Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint144_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint144_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint144}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint144_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint144.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint144}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint144_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint144.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint144}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint144_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint144.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint144}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint144_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint144.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint144}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint144_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint144.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint144_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint144_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint144.prototype[Symbol.dispose] = FheUint144.prototype.free;

export class FheUint152 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint152.prototype);
    obj.__wbg_ptr = ptr;
    FheUint152Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint152Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint152_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint152_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint152}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint152_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint152.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint152}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint152_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint152.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint152}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint152_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint152.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint152}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint152_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint152.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint152}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint152_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint152.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint152_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint152_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint152.prototype[Symbol.dispose] = FheUint152.prototype.free;

export class FheUint16 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint16.prototype);
    obj.__wbg_ptr = ptr;
    FheUint16Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint16Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint16_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint16_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint16}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint16_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint16.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint16}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint16_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint16.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint16}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint16_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint16.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint16}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint16_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint16.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint16}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint16_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint16.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint16_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint16_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint16.prototype[Symbol.dispose] = FheUint16.prototype.free;

export class FheUint160 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint160.prototype);
    obj.__wbg_ptr = ptr;
    FheUint160Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint160Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint160_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint160_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint160}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint160_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint160.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint160}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint160_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint160.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint160}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint160_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint160.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint160}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint160_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint160.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint160}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint160_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint160.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint160_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint160_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint160.prototype[Symbol.dispose] = FheUint160.prototype.free;

export class FheUint168 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint168.prototype);
    obj.__wbg_ptr = ptr;
    FheUint168Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint168Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint168_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint168_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint168}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint168_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint168.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint168}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint168_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint168.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint168}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint168_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint168.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint168}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint168_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint168.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint168}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint168_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint168.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint168_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint168_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint168.prototype[Symbol.dispose] = FheUint168.prototype.free;

export class FheUint176 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint176.prototype);
    obj.__wbg_ptr = ptr;
    FheUint176Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint176Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint176_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint176_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint176}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint176_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint176.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint176}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint176_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint176.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint176}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint176_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint176.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint176}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint176_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint176.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint176}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint176_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint176.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint176_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint176_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint176.prototype[Symbol.dispose] = FheUint176.prototype.free;

export class FheUint184 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint184.prototype);
    obj.__wbg_ptr = ptr;
    FheUint184Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint184Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint184_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint184_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint184}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint184_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint184.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint184}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint184_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint184.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint184}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint184_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint184.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint184}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint184_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint184.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint184}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint184_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint184.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint184_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint184_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint184.prototype[Symbol.dispose] = FheUint184.prototype.free;

export class FheUint192 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint192.prototype);
    obj.__wbg_ptr = ptr;
    FheUint192Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint192Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint192_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint192_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint192}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint192_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint192.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint192}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint192_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint192.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint192}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint192_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint192.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint192}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint192_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint192.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint192}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint192_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint192.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint192_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint192_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint192.prototype[Symbol.dispose] = FheUint192.prototype.free;

export class FheUint2 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint2.prototype);
    obj.__wbg_ptr = ptr;
    FheUint2Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint2Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint2_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint2_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint2}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint2_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint2.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint2}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint2_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint2.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint2}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint2_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint2.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint2}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint2_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint2.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint2}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint2_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint2.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint2_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint2_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint2.prototype[Symbol.dispose] = FheUint2.prototype.free;

export class FheUint200 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint200.prototype);
    obj.__wbg_ptr = ptr;
    FheUint200Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint200Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint200_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint200_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint200}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint200_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint200.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint200}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint200_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint200.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint200}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint200_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint200.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint200}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint200_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint200.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint200}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint200_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint200.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint200_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint200_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint200.prototype[Symbol.dispose] = FheUint200.prototype.free;

export class FheUint2048 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint2048.prototype);
    obj.__wbg_ptr = ptr;
    FheUint2048Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint2048Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint2048_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint2048_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint2048}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint2048_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint2048.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint2048}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint2048_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint2048.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint2048}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint2048_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint2048.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint2048}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint2048_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint2048.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint2048}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint2048_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint2048.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint2048_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint2048_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint2048.prototype[Symbol.dispose] = FheUint2048.prototype.free;

export class FheUint208 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint208.prototype);
    obj.__wbg_ptr = ptr;
    FheUint208Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint208Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint208_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint208_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint208}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint208_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint208.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint208}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint208_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint208.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint208}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint208_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint208.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint208}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint208_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint208.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint208}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint208_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint208.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint208_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint208_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint208.prototype[Symbol.dispose] = FheUint208.prototype.free;

export class FheUint216 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint216.prototype);
    obj.__wbg_ptr = ptr;
    FheUint216Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint216Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint216_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint216_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint216}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint216_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint216.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint216}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint216_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint216.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint216}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint216_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint216.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint216}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint216_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint216.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint216}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint216_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint216.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint216_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint216_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint216.prototype[Symbol.dispose] = FheUint216.prototype.free;

export class FheUint224 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint224.prototype);
    obj.__wbg_ptr = ptr;
    FheUint224Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint224Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint224_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint224_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint224}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint224_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint224.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint224}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint224_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint224.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint224}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint224_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint224.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint224}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint224_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint224.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint224}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint224_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint224.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint224_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint224_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint224.prototype[Symbol.dispose] = FheUint224.prototype.free;

export class FheUint232 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint232.prototype);
    obj.__wbg_ptr = ptr;
    FheUint232Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint232Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint232_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint232_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint232}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint232_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint232.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint232}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint232_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint232.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint232}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint232_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint232.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint232}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint232_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint232.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint232}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint232_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint232.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint232_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint232_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint232.prototype[Symbol.dispose] = FheUint232.prototype.free;

export class FheUint24 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint24.prototype);
    obj.__wbg_ptr = ptr;
    FheUint24Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint24Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint24_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint24_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint24}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint24_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint24.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint24}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint24_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint24.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint24}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint24_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint24.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint24}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint24_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint24.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint24}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint24_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint24.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint24_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint24_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint24.prototype[Symbol.dispose] = FheUint24.prototype.free;

export class FheUint240 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint240.prototype);
    obj.__wbg_ptr = ptr;
    FheUint240Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint240Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint240_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint240_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint240}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint240_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint240.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint240}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint240_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint240.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint240}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint240_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint240.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint240}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint240_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint240.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint240}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint240_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint240.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint240_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint240_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint240.prototype[Symbol.dispose] = FheUint240.prototype.free;

export class FheUint248 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint248.prototype);
    obj.__wbg_ptr = ptr;
    FheUint248Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint248Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint248_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint248_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint248}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint248_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint248.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint248}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint248_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint248.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint248}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint248_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint248.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint248}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint248_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint248.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint248}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint248_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint248.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint248_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint248_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint248.prototype[Symbol.dispose] = FheUint248.prototype.free;

export class FheUint256 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint256.prototype);
    obj.__wbg_ptr = ptr;
    FheUint256Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint256Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint256_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint256_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint256}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint256_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint256.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint256}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint256_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint256.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint256}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint256_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint256.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint256}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint256_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint256.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint256}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint256_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint256.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint256_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint256_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint256.prototype[Symbol.dispose] = FheUint256.prototype.free;

export class FheUint32 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint32.prototype);
    obj.__wbg_ptr = ptr;
    FheUint32Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint32Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint32_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint32_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint32}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint32_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint32.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint32}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint32_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint32.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint32}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint32_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint32.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint32}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint32_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint32.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint32}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint32_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint32.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint32_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint32_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint32.prototype[Symbol.dispose] = FheUint32.prototype.free;

export class FheUint4 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint4.prototype);
    obj.__wbg_ptr = ptr;
    FheUint4Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint4Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint4_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint4_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint4}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint4_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint4.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint4}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint4_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint4.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint4}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint4_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint4.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint4}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint4_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint4.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint4}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint4_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint4.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint4_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint4_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint4.prototype[Symbol.dispose] = FheUint4.prototype.free;

export class FheUint40 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint40.prototype);
    obj.__wbg_ptr = ptr;
    FheUint40Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint40Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint40_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {bigint}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint40_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return BigInt.asUintN(64, ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint40}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint40_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint40.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint40}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint40_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint40.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint40}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint40_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint40.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint40}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint40_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint40.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint40}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint40_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint40.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint40_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint40_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint40.prototype[Symbol.dispose] = FheUint40.prototype.free;

export class FheUint48 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint48.prototype);
    obj.__wbg_ptr = ptr;
    FheUint48Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint48Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint48_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {bigint}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint48_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return BigInt.asUintN(64, ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint48}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint48_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint48.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint48}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint48_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint48.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint48}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint48_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint48.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint48}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint48_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint48.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint48}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint48_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint48.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint48_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint48_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint48.prototype[Symbol.dispose] = FheUint48.prototype.free;

export class FheUint512 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint512.prototype);
    obj.__wbg_ptr = ptr;
    FheUint512Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint512Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint512_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint512_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint512}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint512_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint512.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint512}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint512_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint512.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint512}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint512_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint512.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint512}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint512_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint512.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint512}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint512_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint512.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint512_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint512_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint512.prototype[Symbol.dispose] = FheUint512.prototype.free;

export class FheUint56 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint56.prototype);
    obj.__wbg_ptr = ptr;
    FheUint56Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint56Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint56_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {bigint}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint56_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return BigInt.asUintN(64, ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint56}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint56_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint56.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint56}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint56_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint56.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint56}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint56_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint56.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint56}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint56_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint56.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint56}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint56_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint56.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint56_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint56_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint56.prototype[Symbol.dispose] = FheUint56.prototype.free;

export class FheUint6 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint6.prototype);
    obj.__wbg_ptr = ptr;
    FheUint6Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint6Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint6_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint6_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint6}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint6_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint6.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint6}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint6_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint6.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint6}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint6_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint6.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint6}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint6_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint6.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint6}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint6_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint6.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint6_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint6_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint6.prototype[Symbol.dispose] = FheUint6.prototype.free;

export class FheUint64 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint64.prototype);
    obj.__wbg_ptr = ptr;
    FheUint64Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint64Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint64_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {bigint}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint64_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return BigInt.asUintN(64, ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint64}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint64_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint64.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint64}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint64_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint64.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint64}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint64_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint64.__wrap(ret[0]);
  }
  /**
   * @param {bigint} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint64}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint64_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint64.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint64}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint64_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint64.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint64_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint64_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint64.prototype[Symbol.dispose] = FheUint64.prototype.free;

export class FheUint72 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint72.prototype);
    obj.__wbg_ptr = ptr;
    FheUint72Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint72Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint72_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint72_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint72}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint72_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint72.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint72}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint72_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint72.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint72}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint72_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint72.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint72}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint72_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint72.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint72}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint72_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint72.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint72_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint72_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint72.prototype[Symbol.dispose] = FheUint72.prototype.free;

export class FheUint8 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint8.prototype);
    obj.__wbg_ptr = ptr;
    FheUint8Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint8Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint8_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {number}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint8_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint8}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint8_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint8.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint8}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint8_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint8.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint8}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint8_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint8.__wrap(ret[0]);
  }
  /**
   * @param {number} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint8}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint8_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint8.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint8}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint8_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint8.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint8_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint8_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint8.prototype[Symbol.dispose] = FheUint8.prototype.free;

export class FheUint80 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint80.prototype);
    obj.__wbg_ptr = ptr;
    FheUint80Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint80Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint80_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint80_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint80}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint80_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint80.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint80}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint80_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint80.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint80}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint80_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint80.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint80}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint80_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint80.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint80}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint80_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint80.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint80_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint80_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint80.prototype[Symbol.dispose] = FheUint80.prototype.free;

export class FheUint88 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint88.prototype);
    obj.__wbg_ptr = ptr;
    FheUint88Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint88Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint88_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint88_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint88}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint88_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint88.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint88}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint88_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint88.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint88}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint88_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint88.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint88}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint88_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint88.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint88}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint88_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint88.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint88_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint88_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint88.prototype[Symbol.dispose] = FheUint88.prototype.free;

export class FheUint96 {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FheUint96.prototype);
    obj.__wbg_ptr = ptr;
    FheUint96Finalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FheUint96Finalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fheuint96_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {any}
   */
  decrypt(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint96_decrypt(this.__wbg_ptr, client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {FheUint96}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint96_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint96.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheClientKey} client_key
   * @returns {FheUint96}
   */
  static encrypt_with_client_key(value, client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.fheuint96_encrypt_with_client_key(
      value,
      client_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint96.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfheCompressedPublicKey} compressed_public_key
   * @returns {FheUint96}
   */
  static encrypt_with_compressed_public_key(value, compressed_public_key) {
    _assertClass(compressed_public_key, TfheCompressedPublicKey);
    const ret = wasm.fheuint96_encrypt_with_compressed_public_key(
      value,
      compressed_public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint96.__wrap(ret[0]);
  }
  /**
   * @param {any} value
   * @param {TfhePublicKey} public_key
   * @returns {FheUint96}
   */
  static encrypt_with_public_key(value, public_key) {
    _assertClass(public_key, TfhePublicKey);
    const ret = wasm.fheuint96_encrypt_with_public_key(
      value,
      public_key.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint96.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {FheUint96}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.fheuint96_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return FheUint96.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.fheuint96_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.fheuint96_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  FheUint96.prototype[Symbol.dispose] = FheUint96.prototype.free;

export class ProvenCompactCiphertextList {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(ProvenCompactCiphertextList.prototype);
    obj.__wbg_ptr = ptr;
    ProvenCompactCiphertextListFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    ProvenCompactCiphertextListFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_provencompactciphertextlist_free(ptr, 0);
  }
  /**
   * @param {TfheCompactPublicKey} public_key
   * @returns {CompactCiphertextListBuilder}
   */
  static builder(public_key) {
    _assertClass(public_key, TfheCompactPublicKey);
    const ret = wasm.provencompactciphertextlist_builder(public_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompactCiphertextListBuilder.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {ProvenCompactCiphertextList}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.provencompactciphertextlist_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ProvenCompactCiphertextList.__wrap(ret[0]);
  }
  /**
   * @returns {CompactCiphertextListExpander}
   */
  expand_without_verification() {
    const ret = wasm.provencompactciphertextlist_expand_without_verification(
      this.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompactCiphertextListExpander.__wrap(ret[0]);
  }
  /**
   * @param {number} index
   * @returns {FheTypes | undefined}
   */
  get_kind_of(index) {
    const ret = wasm.provencompactciphertextlist_get_kind_of(
      this.__wbg_ptr,
      index,
    );
    return ret === 84 ? undefined : ret;
  }
  /**
   * @returns {boolean}
   */
  is_empty() {
    const ret = wasm.compactciphertextlistexpander_is_empty(this.__wbg_ptr);
    return ret !== 0;
  }
  /**
   * @returns {number}
   */
  len() {
    const ret = wasm.compactciphertextlistexpander_len(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {ProvenCompactCiphertextList}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.provencompactciphertextlist_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ProvenCompactCiphertextList.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.provencompactciphertextlist_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.provencompactciphertextlist_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @param {CompactPkeCrs} crs
   * @param {TfheCompactPublicKey} public_key
   * @param {Uint8Array} metadata
   * @returns {CompactCiphertextListExpander}
   */
  verify_and_expand(crs, public_key, metadata) {
    _assertClass(crs, CompactPkeCrs);
    _assertClass(public_key, TfheCompactPublicKey);
    const ptr0 = passArray8ToWasm0(metadata, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.provencompactciphertextlist_verify_and_expand(
      this.__wbg_ptr,
      crs.__wbg_ptr,
      public_key.__wbg_ptr,
      ptr0,
      len0,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return CompactCiphertextListExpander.__wrap(ret[0]);
  }
}
if (Symbol.dispose)
  ProvenCompactCiphertextList.prototype[Symbol.dispose] =
    ProvenCompactCiphertextList.prototype.free;

export class Shortint {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    ShortintFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_shortint_free(ptr, 0);
  }
  /**
   * @param {ShortintCompressedCiphertext} compressed_ciphertext
   * @returns {ShortintCiphertext}
   */
  static decompress_ciphertext(compressed_ciphertext) {
    _assertClass(compressed_ciphertext, ShortintCompressedCiphertext);
    const ret = wasm.shortint_decompress_ciphertext(
      compressed_ciphertext.__wbg_ptr,
    );
    return ShortintCiphertext.__wrap(ret);
  }
  /**
   * @param {ShortintClientKey} client_key
   * @param {ShortintCiphertext} ct
   * @returns {bigint}
   */
  static decrypt(client_key, ct) {
    _assertClass(client_key, ShortintClientKey);
    _assertClass(ct, ShortintCiphertext);
    const ret = wasm.shortint_decrypt(client_key.__wbg_ptr, ct.__wbg_ptr);
    return BigInt.asUintN(64, ret);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {ShortintCiphertext}
   */
  static deserialize_ciphertext(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.shortint_deserialize_ciphertext(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ShortintCiphertext.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {ShortintClientKey}
   */
  static deserialize_client_key(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.shortint_deserialize_client_key(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ShortintClientKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {ShortintCompressedCiphertext}
   */
  static deserialize_compressed_ciphertext(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.shortint_deserialize_compressed_ciphertext(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ShortintCompressedCiphertext.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {ShortintCompressedPublicKey}
   */
  static deserialize_compressed_public_key(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.shortint_deserialize_compressed_public_key(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ShortintCompressedPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {ShortintCompressedServerKey}
   */
  static deserialize_compressed_server_key(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.shortint_deserialize_compressed_server_key(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ShortintCompressedServerKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {ShortintPublicKey}
   */
  static deserialize_public_key(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.shortint_deserialize_public_key(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ShortintPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {ShortintClientKey} client_key
   * @param {bigint} message
   * @returns {ShortintCiphertext}
   */
  static encrypt(client_key, message) {
    _assertClass(client_key, ShortintClientKey);
    const ret = wasm.shortint_encrypt(client_key.__wbg_ptr, message);
    return ShortintCiphertext.__wrap(ret);
  }
  /**
   * @param {ShortintClientKey} client_key
   * @param {bigint} message
   * @returns {ShortintCompressedCiphertext}
   */
  static encrypt_compressed(client_key, message) {
    _assertClass(client_key, ShortintClientKey);
    const ret = wasm.shortint_encrypt_compressed(client_key.__wbg_ptr, message);
    return ShortintCompressedCiphertext.__wrap(ret);
  }
  /**
   * @param {ShortintCompressedPublicKey} public_key
   * @param {bigint} message
   * @returns {ShortintCiphertext}
   */
  static encrypt_with_compressed_public_key(public_key, message) {
    _assertClass(public_key, ShortintCompressedPublicKey);
    const ret = wasm.shortint_encrypt_with_compressed_public_key(
      public_key.__wbg_ptr,
      message,
    );
    return ShortintCiphertext.__wrap(ret);
  }
  /**
   * @param {ShortintPublicKey} public_key
   * @param {bigint} message
   * @returns {ShortintCiphertext}
   */
  static encrypt_with_public_key(public_key, message) {
    _assertClass(public_key, ShortintPublicKey);
    const ret = wasm.shortint_encrypt_with_public_key(
      public_key.__wbg_ptr,
      message,
    );
    return ShortintCiphertext.__wrap(ret);
  }
  /**
   * @param {ShortintParameters} parameters
   * @returns {ShortintClientKey}
   */
  static new_client_key(parameters) {
    _assertClass(parameters, ShortintParameters);
    const ret = wasm.shortint_new_client_key(parameters.__wbg_ptr);
    return ShortintClientKey.__wrap(ret);
  }
  /**
   * @param {bigint} seed_high_bytes
   * @param {bigint} seed_low_bytes
   * @param {ShortintParameters} parameters
   * @returns {ShortintClientKey}
   */
  static new_client_key_from_seed_and_parameters(
    seed_high_bytes,
    seed_low_bytes,
    parameters,
  ) {
    _assertClass(parameters, ShortintParameters);
    const ret = wasm.shortint_new_client_key_from_seed_and_parameters(
      seed_high_bytes,
      seed_low_bytes,
      parameters.__wbg_ptr,
    );
    return ShortintClientKey.__wrap(ret);
  }
  /**
   * @param {ShortintClientKey} client_key
   * @returns {ShortintCompressedPublicKey}
   */
  static new_compressed_public_key(client_key) {
    _assertClass(client_key, ShortintClientKey);
    const ret = wasm.shortint_new_compressed_public_key(client_key.__wbg_ptr);
    return ShortintCompressedPublicKey.__wrap(ret);
  }
  /**
   * @param {ShortintClientKey} client_key
   * @returns {ShortintCompressedServerKey}
   */
  static new_compressed_server_key(client_key) {
    _assertClass(client_key, ShortintClientKey);
    const ret = wasm.shortint_new_compressed_server_key(client_key.__wbg_ptr);
    return ShortintCompressedServerKey.__wrap(ret);
  }
  /**
   * @param {number} std_dev
   * @returns {ShortintNoiseDistribution}
   */
  static new_gaussian_from_std_dev(std_dev) {
    const ret = wasm.shortint_new_gaussian_from_std_dev(std_dev);
    return ShortintNoiseDistribution.__wrap(ret);
  }
  /**
   * @param {number} lwe_dimension
   * @param {number} glwe_dimension
   * @param {number} polynomial_size
   * @param {ShortintNoiseDistribution} lwe_noise_distribution
   * @param {ShortintNoiseDistribution} glwe_noise_distribution
   * @param {number} pbs_base_log
   * @param {number} pbs_level
   * @param {number} ks_base_log
   * @param {number} ks_level
   * @param {bigint} message_modulus
   * @param {bigint} carry_modulus
   * @param {bigint} max_noise_level
   * @param {number} log2_p_fail
   * @param {number} modulus_power_of_2_exponent
   * @param {ShortintEncryptionKeyChoice} encryption_key_choice
   * @returns {ShortintParameters}
   */
  static new_parameters(
    lwe_dimension,
    glwe_dimension,
    polynomial_size,
    lwe_noise_distribution,
    glwe_noise_distribution,
    pbs_base_log,
    pbs_level,
    ks_base_log,
    ks_level,
    message_modulus,
    carry_modulus,
    max_noise_level,
    log2_p_fail,
    modulus_power_of_2_exponent,
    encryption_key_choice,
  ) {
    _assertClass(lwe_noise_distribution, ShortintNoiseDistribution);
    _assertClass(glwe_noise_distribution, ShortintNoiseDistribution);
    const ret = wasm.shortint_new_parameters(
      lwe_dimension,
      glwe_dimension,
      polynomial_size,
      lwe_noise_distribution.__wbg_ptr,
      glwe_noise_distribution.__wbg_ptr,
      pbs_base_log,
      pbs_level,
      ks_base_log,
      ks_level,
      message_modulus,
      carry_modulus,
      max_noise_level,
      log2_p_fail,
      modulus_power_of_2_exponent,
      encryption_key_choice,
    );
    return ShortintParameters.__wrap(ret);
  }
  /**
   * @param {ShortintClientKey} client_key
   * @returns {ShortintPublicKey}
   */
  static new_public_key(client_key) {
    _assertClass(client_key, ShortintClientKey);
    const ret = wasm.shortint_new_public_key(client_key.__wbg_ptr);
    return ShortintPublicKey.__wrap(ret);
  }
  /**
   * @param {ShortintCiphertext} ciphertext
   * @returns {Uint8Array}
   */
  static serialize_ciphertext(ciphertext) {
    _assertClass(ciphertext, ShortintCiphertext);
    const ret = wasm.shortint_serialize_ciphertext(ciphertext.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @param {ShortintClientKey} client_key
   * @returns {Uint8Array}
   */
  static serialize_client_key(client_key) {
    _assertClass(client_key, ShortintClientKey);
    const ret = wasm.shortint_serialize_client_key(client_key.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @param {ShortintCompressedCiphertext} ciphertext
   * @returns {Uint8Array}
   */
  static serialize_compressed_ciphertext(ciphertext) {
    _assertClass(ciphertext, ShortintCompressedCiphertext);
    const ret = wasm.shortint_serialize_compressed_ciphertext(
      ciphertext.__wbg_ptr,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @param {ShortintCompressedPublicKey} public_key
   * @returns {Uint8Array}
   */
  static serialize_compressed_public_key(public_key) {
    _assertClass(public_key, ShortintCompressedPublicKey);
    const ret = wasm.shortint_serialize_compressed_public_key(
      public_key.__wbg_ptr,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @param {ShortintCompressedServerKey} server_key
   * @returns {Uint8Array}
   */
  static serialize_compressed_server_key(server_key) {
    _assertClass(server_key, ShortintCompressedServerKey);
    const ret = wasm.shortint_serialize_compressed_server_key(
      server_key.__wbg_ptr,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @param {ShortintPublicKey} public_key
   * @returns {Uint8Array}
   */
  static serialize_public_key(public_key) {
    _assertClass(public_key, ShortintPublicKey);
    const ret = wasm.shortint_serialize_public_key(public_key.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @param {number} bound_log2
   * @returns {ShortintNoiseDistribution}
   */
  static try_new_t_uniform(bound_log2) {
    const ret = wasm.shortint_try_new_t_uniform(bound_log2);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ShortintNoiseDistribution.__wrap(ret[0]);
  }
}
if (Symbol.dispose)
  Shortint.prototype[Symbol.dispose] = Shortint.prototype.free;

export class ShortintCiphertext {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(ShortintCiphertext.prototype);
    obj.__wbg_ptr = ptr;
    ShortintCiphertextFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    ShortintCiphertextFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_shortintciphertext_free(ptr, 0);
  }
}
if (Symbol.dispose)
  ShortintCiphertext.prototype[Symbol.dispose] =
    ShortintCiphertext.prototype.free;

export class ShortintClientKey {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(ShortintClientKey.prototype);
    obj.__wbg_ptr = ptr;
    ShortintClientKeyFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    ShortintClientKeyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_shortintclientkey_free(ptr, 0);
  }
}
if (Symbol.dispose)
  ShortintClientKey.prototype[Symbol.dispose] =
    ShortintClientKey.prototype.free;

export class ShortintCompactPublicKeyEncryptionParameters {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(
      ShortintCompactPublicKeyEncryptionParameters.prototype,
    );
    obj.__wbg_ptr = ptr;
    ShortintCompactPublicKeyEncryptionParametersFinalization.register(
      obj,
      obj.__wbg_ptr,
      obj,
    );
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    ShortintCompactPublicKeyEncryptionParametersFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_shortintcompactpublickeyencryptionparameters_free(ptr, 0);
  }
  /**
   * @param {ShortintCompactPublicKeyEncryptionParametersName} name
   */
  constructor(name) {
    const ret = wasm.shortintcompactpublickeyencryptionparameters_new(name);
    this.__wbg_ptr = ret >>> 0;
    ShortintCompactPublicKeyEncryptionParametersFinalization.register(
      this,
      this.__wbg_ptr,
      this,
    );
    return this;
  }
  /**
   * @param {number} encryption_lwe_dimension
   * @param {ShortintNoiseDistribution} encryption_noise_distribution
   * @param {bigint} message_modulus
   * @param {bigint} carry_modulus
   * @param {number} modulus_power_of_2_exponent
   * @param {number} ks_base_log
   * @param {number} ks_level
   * @param {ShortintEncryptionKeyChoice} encryption_key_choice
   * @returns {ShortintCompactPublicKeyEncryptionParameters}
   */
  static new_parameters(
    encryption_lwe_dimension,
    encryption_noise_distribution,
    message_modulus,
    carry_modulus,
    modulus_power_of_2_exponent,
    ks_base_log,
    ks_level,
    encryption_key_choice,
  ) {
    _assertClass(encryption_noise_distribution, ShortintNoiseDistribution);
    const ret =
      wasm.shortintcompactpublickeyencryptionparameters_new_parameters(
        encryption_lwe_dimension,
        encryption_noise_distribution.__wbg_ptr,
        message_modulus,
        carry_modulus,
        modulus_power_of_2_exponent,
        ks_base_log,
        ks_level,
        encryption_key_choice,
      );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return ShortintCompactPublicKeyEncryptionParameters.__wrap(ret[0]);
  }
}
if (Symbol.dispose)
  ShortintCompactPublicKeyEncryptionParameters.prototype[Symbol.dispose] =
    ShortintCompactPublicKeyEncryptionParameters.prototype.free;

/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14}
 */
export const ShortintCompactPublicKeyEncryptionParametersName = Object.freeze({
  PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128: 0,
  0: "PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128",
  V1_1_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128: 1,
  1: "V1_1_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128",
  V1_1_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1: 2,
  2: "V1_1_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1",
  V1_0_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128: 3,
  3: "V1_0_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128",
  V1_0_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1: 4,
  4: "V1_0_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1",
  V0_11_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64: 5,
  5: "V0_11_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64",
  V0_11_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64_ZKV1: 6,
  6: "V0_11_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64_ZKV1",
  V1_2_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128: 7,
  7: "V1_2_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128",
  V1_2_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1: 8,
  8: "V1_2_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1",
  V1_3_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128: 9,
  9: "V1_3_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128",
  V1_3_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1: 10,
  10: "V1_3_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1",
  V1_4_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128: 11,
  11: "V1_4_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128",
  V1_4_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1: 12,
  12: "V1_4_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1",
  V1_5_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128: 13,
  13: "V1_5_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128",
  V1_5_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1: 14,
  14: "V1_5_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1",
});

export class ShortintCompressedCiphertext {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(ShortintCompressedCiphertext.prototype);
    obj.__wbg_ptr = ptr;
    ShortintCompressedCiphertextFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    ShortintCompressedCiphertextFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_shortintcompressedciphertext_free(ptr, 0);
  }
}
if (Symbol.dispose)
  ShortintCompressedCiphertext.prototype[Symbol.dispose] =
    ShortintCompressedCiphertext.prototype.free;

export class ShortintCompressedPublicKey {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(ShortintCompressedPublicKey.prototype);
    obj.__wbg_ptr = ptr;
    ShortintCompressedPublicKeyFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    ShortintCompressedPublicKeyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_shortintcompressedpublickey_free(ptr, 0);
  }
}
if (Symbol.dispose)
  ShortintCompressedPublicKey.prototype[Symbol.dispose] =
    ShortintCompressedPublicKey.prototype.free;

export class ShortintCompressedServerKey {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(ShortintCompressedServerKey.prototype);
    obj.__wbg_ptr = ptr;
    ShortintCompressedServerKeyFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    ShortintCompressedServerKeyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_shortintcompressedserverkey_free(ptr, 0);
  }
}
if (Symbol.dispose)
  ShortintCompressedServerKey.prototype[Symbol.dispose] =
    ShortintCompressedServerKey.prototype.free;

/**
 * @enum {0 | 1}
 */
export const ShortintEncryptionKeyChoice = Object.freeze({
  Big: 0,
  0: "Big",
  Small: 1,
  1: "Small",
});

export class ShortintNoiseDistribution {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(ShortintNoiseDistribution.prototype);
    obj.__wbg_ptr = ptr;
    ShortintNoiseDistributionFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    ShortintNoiseDistributionFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_shortintnoisedistribution_free(ptr, 0);
  }
}
if (Symbol.dispose)
  ShortintNoiseDistribution.prototype[Symbol.dispose] =
    ShortintNoiseDistribution.prototype.free;

/**
 * @enum {0 | 1}
 */
export const ShortintPBSOrder = Object.freeze({
  KeyswitchBootstrap: 0,
  0: "KeyswitchBootstrap",
  BootstrapKeyswitch: 1,
  1: "BootstrapKeyswitch",
});

export class ShortintParameters {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(ShortintParameters.prototype);
    obj.__wbg_ptr = ptr;
    ShortintParametersFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    ShortintParametersFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_shortintparameters_free(ptr, 0);
  }
  /**
   * @returns {bigint}
   */
  carry_modulus() {
    const ret = wasm.shortintparameters_carry_modulus(this.__wbg_ptr);
    return BigInt.asUintN(64, ret);
  }
  /**
   * @returns {ShortintEncryptionKeyChoice}
   */
  encryption_key_choice() {
    const ret = wasm.shortintparameters_encryption_key_choice(this.__wbg_ptr);
    return ret;
  }
  /**
   * @returns {number}
   */
  glwe_dimension() {
    const ret = wasm.shortintparameters_glwe_dimension(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * @returns {ShortintNoiseDistribution}
   */
  glwe_noise_distribution() {
    const ret = wasm.shortintparameters_glwe_noise_distribution(this.__wbg_ptr);
    return ShortintNoiseDistribution.__wrap(ret);
  }
  /**
   * @returns {number}
   */
  ks_base_log() {
    const ret = wasm.shortintparameters_ks_base_log(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * @returns {number}
   */
  ks_level() {
    const ret = wasm.shortintparameters_ks_level(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * @returns {number}
   */
  lwe_dimension() {
    const ret = wasm.shortintparameters_lwe_dimension(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * @returns {ShortintNoiseDistribution}
   */
  lwe_noise_distribution() {
    const ret = wasm.shortintparameters_glwe_noise_distribution(this.__wbg_ptr);
    return ShortintNoiseDistribution.__wrap(ret);
  }
  /**
   * @returns {bigint}
   */
  message_modulus() {
    const ret = wasm.shortintparameters_message_modulus(this.__wbg_ptr);
    return BigInt.asUintN(64, ret);
  }
  /**
   * @param {ShortintParametersName | null} [name]
   */
  constructor(name) {
    const ret = wasm.shortintparameters_new(isLikeNone(name) ? 498 : name);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    this.__wbg_ptr = ret[0] >>> 0;
    ShortintParametersFinalization.register(this, this.__wbg_ptr, this);
    return this;
  }
  /**
   * @returns {number}
   */
  pbs_base_log() {
    const ret = wasm.shortintparameters_pbs_base_log(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * @returns {number}
   */
  pbs_level() {
    const ret = wasm.shortintparameters_pbs_level(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * @returns {number}
   */
  polynomial_size() {
    const ret = wasm.shortintparameters_polynomial_size(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * @param {bigint} new_value
   */
  set_carry_modulus(new_value) {
    wasm.shortintparameters_set_carry_modulus(this.__wbg_ptr, new_value);
  }
  /**
   * @param {ShortintEncryptionKeyChoice} new_value
   */
  set_encryption_key_choice(new_value) {
    wasm.shortintparameters_set_encryption_key_choice(
      this.__wbg_ptr,
      new_value,
    );
  }
  /**
   * @param {number} new_value
   */
  set_glwe_dimension(new_value) {
    wasm.shortintparameters_set_glwe_dimension(this.__wbg_ptr, new_value);
  }
  /**
   * @param {ShortintNoiseDistribution} new_value
   */
  set_glwe_noise_distribution(new_value) {
    _assertClass(new_value, ShortintNoiseDistribution);
    wasm.shortintparameters_set_glwe_noise_distribution(
      this.__wbg_ptr,
      new_value.__wbg_ptr,
    );
  }
  /**
   * @param {number} new_value
   */
  set_ks_base_log(new_value) {
    wasm.shortintparameters_set_ks_base_log(this.__wbg_ptr, new_value);
  }
  /**
   * @param {number} new_value
   */
  set_ks_level(new_value) {
    wasm.shortintparameters_set_ks_level(this.__wbg_ptr, new_value);
  }
  /**
   * @param {number} new_value
   */
  set_lwe_dimension(new_value) {
    wasm.shortintparameters_set_lwe_dimension(this.__wbg_ptr, new_value);
  }
  /**
   * @param {ShortintNoiseDistribution} new_value
   */
  set_lwe_noise_distribution(new_value) {
    _assertClass(new_value, ShortintNoiseDistribution);
    wasm.shortintparameters_set_lwe_noise_distribution(
      this.__wbg_ptr,
      new_value.__wbg_ptr,
    );
  }
  /**
   * @param {bigint} new_value
   */
  set_message_modulus(new_value) {
    wasm.shortintparameters_set_message_modulus(this.__wbg_ptr, new_value);
  }
  /**
   * @param {number} new_value
   */
  set_pbs_base_log(new_value) {
    wasm.shortintparameters_set_pbs_base_log(this.__wbg_ptr, new_value);
  }
  /**
   * @param {number} new_value
   */
  set_pbs_level(new_value) {
    wasm.shortintparameters_set_pbs_level(this.__wbg_ptr, new_value);
  }
  /**
   * @param {number} new_value
   */
  set_polynomial_size(new_value) {
    wasm.shortintparameters_set_polynomial_size(this.__wbg_ptr, new_value);
  }
}
if (Symbol.dispose)
  ShortintParameters.prototype[Symbol.dispose] =
    ShortintParameters.prototype.free;

/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 123 | 124 | 125 | 126 | 127 | 128 | 129 | 130 | 131 | 132 | 133 | 134 | 135 | 136 | 137 | 138 | 139 | 140 | 141 | 142 | 143 | 144 | 145 | 146 | 147 | 148 | 149 | 150 | 151 | 152 | 153 | 154 | 155 | 156 | 157 | 158 | 159 | 160 | 161 | 162 | 163 | 164 | 165 | 166 | 167 | 168 | 169 | 170 | 171 | 172 | 173 | 174 | 175 | 176 | 177 | 178 | 179 | 180 | 181 | 182 | 183 | 184 | 185 | 186 | 187 | 188 | 189 | 190 | 191 | 192 | 193 | 194 | 195 | 196 | 197 | 198 | 199 | 200 | 201 | 202 | 203 | 204 | 205 | 206 | 207 | 208 | 209 | 210 | 211 | 212 | 213 | 214 | 215 | 216 | 217 | 218 | 219 | 220 | 221 | 222 | 223 | 224 | 225 | 226 | 227 | 228 | 229 | 230 | 231 | 232 | 233 | 234 | 235 | 236 | 237 | 238 | 239 | 240 | 241 | 242 | 243 | 244 | 245 | 246 | 247 | 248 | 249 | 250 | 251 | 252 | 253 | 254 | 255 | 256 | 257 | 258 | 259 | 260 | 261 | 262 | 263 | 264 | 265 | 266 | 267 | 268 | 269 | 270 | 271 | 272 | 273 | 274 | 275 | 276 | 277 | 278 | 279 | 280 | 281 | 282 | 283 | 284 | 285 | 286 | 287 | 288 | 289 | 290 | 291 | 292 | 293 | 294 | 295 | 296 | 297 | 298 | 299 | 300 | 301 | 302 | 303 | 304 | 305 | 306 | 307 | 308 | 309 | 310 | 311 | 312 | 313 | 314 | 315 | 316 | 317 | 318 | 319 | 320 | 321 | 322 | 323 | 324 | 325 | 326 | 327 | 328 | 329 | 330 | 331 | 332 | 333 | 334 | 335 | 336 | 337 | 338 | 339 | 340 | 341 | 342 | 343 | 344 | 345 | 346 | 347 | 348 | 349 | 350 | 351 | 352 | 353 | 354 | 355 | 356 | 357 | 358 | 359 | 360 | 361 | 362 | 363 | 364 | 365 | 366 | 367 | 368 | 369 | 370 | 371 | 372 | 373 | 374 | 375 | 376 | 377 | 378 | 379 | 380 | 381 | 382 | 383 | 384 | 385 | 386 | 387 | 388 | 389 | 390 | 391 | 392 | 393 | 394 | 395 | 396 | 397 | 398 | 399 | 400 | 401 | 402 | 403 | 404 | 405 | 406 | 407 | 408 | 409 | 410 | 411 | 412 | 413 | 414 | 415 | 416 | 417 | 418 | 419 | 420 | 421 | 422 | 423 | 424 | 425 | 426 | 427 | 428 | 429 | 430 | 431 | 432 | 433 | 434 | 435 | 436 | 437 | 438 | 439 | 440 | 441 | 442 | 443 | 444 | 445 | 446 | 447 | 448 | 449 | 450 | 451 | 452 | 453 | 454 | 455 | 456 | 457 | 458 | 459 | 460 | 461 | 462 | 463 | 464 | 465 | 466 | 467 | 468 | 469 | 470 | 471 | 472 | 473 | 474 | 475 | 476 | 477 | 478 | 479 | 480 | 481 | 482 | 483 | 484 | 485 | 486 | 487 | 488 | 489 | 490 | 491 | 492 | 493 | 494 | 495 | 496 | 497}
 */
export const ShortintParametersName = Object.freeze({
  PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128: 0,
  0: "PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128: 1,
  1: "V1_1_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128: 2,
  2: "V1_1_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128: 3,
  3: "V1_1_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128: 4,
  4: "V1_1_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128: 5,
  5: "V1_1_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128: 6,
  6: "V1_1_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128: 7,
  7: "V1_1_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128: 8,
  8: "V1_1_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128: 9,
  9: "V1_1_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128: 10,
  10: "V1_1_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128: 11,
  11: "V1_1_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128: 12,
  12: "V1_1_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128: 13,
  13: "V1_1_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128: 14,
  14: "V1_1_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128: 15,
  15: "V1_1_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128: 16,
  16: "V1_1_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128: 17,
  17: "V1_1_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128: 18,
  18: "V1_1_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128: 19,
  19: "V1_1_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128: 20,
  20: "V1_1_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128: 21,
  21: "V1_1_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128: 22,
  22: "V1_1_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128: 23,
  23: "V1_1_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128: 24,
  24: "V1_1_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128: 25,
  25: "V1_1_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128: 26,
  26: "V1_1_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128: 27,
  27: "V1_1_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128: 28,
  28: "V1_1_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128: 29,
  29: "V1_1_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128: 30,
  30: "V1_1_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128: 31,
  31: "V1_1_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128: 32,
  32: "V1_1_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128: 33,
  33: "V1_1_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128: 34,
  34: "V1_1_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128: 35,
  35: "V1_1_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128: 36,
  36: "V1_1_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128: 37,
  37: "V1_1_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128: 38,
  38: "V1_1_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128: 39,
  39: "V1_1_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128: 40,
  40: "V1_1_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 41,
  41: "V1_1_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 42,
  42: "V1_1_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 43,
  43: "V1_1_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 44,
  44: "V1_1_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 45,
  45: "V1_1_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 46,
  46: "V1_1_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 47,
  47: "V1_1_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 48,
  48: "V1_1_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 49,
  49: "V1_1_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 50,
  50: "V1_1_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 51,
  51: "V1_1_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 52,
  52: "V1_1_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 53,
  53: "V1_1_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 54,
  54: "V1_1_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 55,
  55: "V1_1_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 56,
  56: "V1_1_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 57,
  57: "V1_1_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 58,
  58: "V1_1_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 59,
  59: "V1_1_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 60,
  60: "V1_1_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 61,
  61: "V1_1_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 62,
  62: "V1_1_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 63,
  63: "V1_1_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 64,
  64: "V1_1_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 65,
  65: "V1_1_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 66,
  66: "V1_1_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 67,
  67: "V1_1_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 68,
  68: "V1_1_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 69,
  69: "V1_1_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 70,
  70: "V1_1_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_1_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 71,
  71: "V1_1_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128: 72,
  72: "V1_0_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128: 73,
  73: "V1_0_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128: 74,
  74: "V1_0_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128: 75,
  75: "V1_0_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128: 76,
  76: "V1_0_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128: 77,
  77: "V1_0_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128: 78,
  78: "V1_0_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128: 79,
  79: "V1_0_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128: 80,
  80: "V1_0_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128: 81,
  81: "V1_0_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128: 82,
  82: "V1_0_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128: 83,
  83: "V1_0_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128: 84,
  84: "V1_0_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128: 85,
  85: "V1_0_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128: 86,
  86: "V1_0_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128: 87,
  87: "V1_0_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128: 88,
  88: "V1_0_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128: 89,
  89: "V1_0_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128: 90,
  90: "V1_0_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128: 91,
  91: "V1_0_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128: 92,
  92: "V1_0_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128: 93,
  93: "V1_0_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128: 94,
  94: "V1_0_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128: 95,
  95: "V1_0_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128: 96,
  96: "V1_0_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128: 97,
  97: "V1_0_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128: 98,
  98: "V1_0_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128: 99,
  99: "V1_0_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128: 100,
  100: "V1_0_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128: 101,
  101: "V1_0_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128: 102,
  102: "V1_0_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128: 103,
  103: "V1_0_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128: 104,
  104: "V1_0_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128: 105,
  105: "V1_0_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128: 106,
  106: "V1_0_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128: 107,
  107: "V1_0_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128: 108,
  108: "V1_0_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128: 109,
  109: "V1_0_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128: 110,
  110: "V1_0_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128: 111,
  111: "V1_0_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 112,
  112: "V1_0_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 113,
  113: "V1_0_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 114,
  114: "V1_0_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 115,
  115: "V1_0_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 116,
  116: "V1_0_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 117,
  117: "V1_0_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 118,
  118: "V1_0_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 119,
  119: "V1_0_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 120,
  120: "V1_0_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 121,
  121: "V1_0_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 122,
  122: "V1_0_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 123,
  123: "V1_0_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 124,
  124: "V1_0_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 125,
  125: "V1_0_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 126,
  126: "V1_0_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 127,
  127: "V1_0_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 128,
  128: "V1_0_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 129,
  129: "V1_0_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 130,
  130: "V1_0_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 131,
  131: "V1_0_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 132,
  132: "V1_0_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 133,
  133: "V1_0_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 134,
  134: "V1_0_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 135,
  135: "V1_0_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 136,
  136: "V1_0_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 137,
  137: "V1_0_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 138,
  138: "V1_0_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 139,
  139: "V1_0_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 140,
  140: "V1_0_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 141,
  141: "V1_0_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_0_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 142,
  142: "V1_0_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V0_11_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M64: 143,
  143: "V0_11_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M64: 144,
  144: "V0_11_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M64: 145,
  145: "V0_11_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M64: 146,
  146: "V0_11_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M64: 147,
  147: "V0_11_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M64: 148,
  148: "V0_11_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M64: 149,
  149: "V0_11_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M64: 150,
  150: "V0_11_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M64: 151,
  151: "V0_11_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M64: 152,
  152: "V0_11_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M64: 153,
  153: "V0_11_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M64: 154,
  154: "V0_11_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M64: 155,
  155: "V0_11_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M64: 156,
  156: "V0_11_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M64: 157,
  157: "V0_11_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M64: 158,
  158: "V0_11_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M64: 159,
  159: "V0_11_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M64: 160,
  160: "V0_11_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M64: 161,
  161: "V0_11_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M64: 162,
  162: "V0_11_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M64: 163,
  163: "V0_11_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M64: 164,
  164: "V0_11_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M64: 165,
  165: "V0_11_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M64: 166,
  166: "V0_11_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M64: 167,
  167: "V0_11_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M64: 168,
  168: "V0_11_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M64: 169,
  169: "V0_11_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M64: 170,
  170: "V0_11_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M64: 171,
  171: "V0_11_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M64: 172,
  172: "V0_11_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M64: 173,
  173: "V0_11_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M64: 174,
  174: "V0_11_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M64: 175,
  175: "V0_11_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M64: 176,
  176: "V0_11_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M64: 177,
  177: "V0_11_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M64: 178,
  178: "V0_11_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M64: 179,
  179: "V0_11_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M64: 180,
  180: "V0_11_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M64: 181,
  181: "V0_11_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M64: 182,
  182: "V0_11_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 183,
  183: "V0_11_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 184,
  184: "V0_11_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 185,
  185: "V0_11_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 186,
  186: "V0_11_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 187,
  187: "V0_11_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 188,
  188: "V0_11_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 189,
  189: "V0_11_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 190,
  190: "V0_11_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 191,
  191: "V0_11_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 192,
  192: "V0_11_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 193,
  193: "V0_11_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 194,
  194: "V0_11_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 195,
  195: "V0_11_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 196,
  196: "V0_11_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 197,
  197: "V0_11_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 198,
  198: "V0_11_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 199,
  199: "V0_11_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 200,
  200: "V0_11_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 201,
  201: "V0_11_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 202,
  202: "V0_11_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 203,
  203: "V0_11_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 204,
  204: "V0_11_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 205,
  205: "V0_11_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 206,
  206: "V0_11_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 207,
  207: "V0_11_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 208,
  208: "V0_11_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64: 209,
  209: "V0_11_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M64: 210,
  210: "V0_11_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M64: 211,
  211: "V0_11_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M64: 212,
  212: "V0_11_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M64",
  V0_11_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M64: 213,
  213: "V0_11_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M64",
  V1_2_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128: 214,
  214: "V1_2_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128: 215,
  215: "V1_2_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128: 216,
  216: "V1_2_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128: 217,
  217: "V1_2_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128: 218,
  218: "V1_2_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128: 219,
  219: "V1_2_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128: 220,
  220: "V1_2_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128: 221,
  221: "V1_2_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128: 222,
  222: "V1_2_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128: 223,
  223: "V1_2_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128: 224,
  224: "V1_2_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128: 225,
  225: "V1_2_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128: 226,
  226: "V1_2_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128: 227,
  227: "V1_2_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128: 228,
  228: "V1_2_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128: 229,
  229: "V1_2_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128: 230,
  230: "V1_2_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128: 231,
  231: "V1_2_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128: 232,
  232: "V1_2_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128: 233,
  233: "V1_2_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128: 234,
  234: "V1_2_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128: 235,
  235: "V1_2_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128: 236,
  236: "V1_2_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128: 237,
  237: "V1_2_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128: 238,
  238: "V1_2_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128: 239,
  239: "V1_2_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128: 240,
  240: "V1_2_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128: 241,
  241: "V1_2_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128: 242,
  242: "V1_2_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128: 243,
  243: "V1_2_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128: 244,
  244: "V1_2_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128: 245,
  245: "V1_2_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128: 246,
  246: "V1_2_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128: 247,
  247: "V1_2_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128: 248,
  248: "V1_2_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128: 249,
  249: "V1_2_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128: 250,
  250: "V1_2_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128: 251,
  251: "V1_2_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128: 252,
  252: "V1_2_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128: 253,
  253: "V1_2_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 254,
  254: "V1_2_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 255,
  255: "V1_2_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 256,
  256: "V1_2_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 257,
  257: "V1_2_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 258,
  258: "V1_2_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 259,
  259: "V1_2_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 260,
  260: "V1_2_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 261,
  261: "V1_2_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 262,
  262: "V1_2_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 263,
  263: "V1_2_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 264,
  264: "V1_2_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 265,
  265: "V1_2_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 266,
  266: "V1_2_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 267,
  267: "V1_2_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 268,
  268: "V1_2_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 269,
  269: "V1_2_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 270,
  270: "V1_2_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 271,
  271: "V1_2_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 272,
  272: "V1_2_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 273,
  273: "V1_2_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 274,
  274: "V1_2_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 275,
  275: "V1_2_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 276,
  276: "V1_2_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 277,
  277: "V1_2_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 278,
  278: "V1_2_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 279,
  279: "V1_2_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 280,
  280: "V1_2_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 281,
  281: "V1_2_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 282,
  282: "V1_2_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 283,
  283: "V1_2_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_2_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 284,
  284: "V1_2_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128: 285,
  285: "V1_3_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128: 286,
  286: "V1_3_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128: 287,
  287: "V1_3_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128: 288,
  288: "V1_3_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128: 289,
  289: "V1_3_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128: 290,
  290: "V1_3_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128: 291,
  291: "V1_3_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128: 292,
  292: "V1_3_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128: 293,
  293: "V1_3_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128: 294,
  294: "V1_3_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128: 295,
  295: "V1_3_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128: 296,
  296: "V1_3_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128: 297,
  297: "V1_3_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128: 298,
  298: "V1_3_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128: 299,
  299: "V1_3_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128: 300,
  300: "V1_3_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128: 301,
  301: "V1_3_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128: 302,
  302: "V1_3_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128: 303,
  303: "V1_3_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128: 304,
  304: "V1_3_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128: 305,
  305: "V1_3_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128: 306,
  306: "V1_3_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128: 307,
  307: "V1_3_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128: 308,
  308: "V1_3_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128: 309,
  309: "V1_3_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128: 310,
  310: "V1_3_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128: 311,
  311: "V1_3_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128: 312,
  312: "V1_3_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128: 313,
  313: "V1_3_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128: 314,
  314: "V1_3_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128: 315,
  315: "V1_3_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128: 316,
  316: "V1_3_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128: 317,
  317: "V1_3_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128: 318,
  318: "V1_3_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128: 319,
  319: "V1_3_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128: 320,
  320: "V1_3_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128: 321,
  321: "V1_3_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128: 322,
  322: "V1_3_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128: 323,
  323: "V1_3_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128: 324,
  324: "V1_3_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 325,
  325: "V1_3_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 326,
  326: "V1_3_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 327,
  327: "V1_3_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 328,
  328: "V1_3_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 329,
  329: "V1_3_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 330,
  330: "V1_3_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 331,
  331: "V1_3_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 332,
  332: "V1_3_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 333,
  333: "V1_3_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 334,
  334: "V1_3_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 335,
  335: "V1_3_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 336,
  336: "V1_3_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 337,
  337: "V1_3_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 338,
  338: "V1_3_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 339,
  339: "V1_3_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 340,
  340: "V1_3_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 341,
  341: "V1_3_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 342,
  342: "V1_3_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 343,
  343: "V1_3_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 344,
  344: "V1_3_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 345,
  345: "V1_3_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 346,
  346: "V1_3_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 347,
  347: "V1_3_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 348,
  348: "V1_3_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 349,
  349: "V1_3_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 350,
  350: "V1_3_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 351,
  351: "V1_3_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 352,
  352: "V1_3_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 353,
  353: "V1_3_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 354,
  354: "V1_3_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_3_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 355,
  355: "V1_3_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128: 356,
  356: "V1_4_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128: 357,
  357: "V1_4_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128: 358,
  358: "V1_4_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128: 359,
  359: "V1_4_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128: 360,
  360: "V1_4_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128: 361,
  361: "V1_4_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128: 362,
  362: "V1_4_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128: 363,
  363: "V1_4_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128: 364,
  364: "V1_4_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128: 365,
  365: "V1_4_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128: 366,
  366: "V1_4_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128: 367,
  367: "V1_4_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128: 368,
  368: "V1_4_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128: 369,
  369: "V1_4_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128: 370,
  370: "V1_4_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128: 371,
  371: "V1_4_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128: 372,
  372: "V1_4_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128: 373,
  373: "V1_4_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128: 374,
  374: "V1_4_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128: 375,
  375: "V1_4_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128: 376,
  376: "V1_4_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128: 377,
  377: "V1_4_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128: 378,
  378: "V1_4_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128: 379,
  379: "V1_4_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128: 380,
  380: "V1_4_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128: 381,
  381: "V1_4_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128: 382,
  382: "V1_4_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128: 383,
  383: "V1_4_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128: 384,
  384: "V1_4_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128: 385,
  385: "V1_4_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128: 386,
  386: "V1_4_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128: 387,
  387: "V1_4_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128: 388,
  388: "V1_4_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128: 389,
  389: "V1_4_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128: 390,
  390: "V1_4_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128: 391,
  391: "V1_4_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128: 392,
  392: "V1_4_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128: 393,
  393: "V1_4_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128: 394,
  394: "V1_4_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128: 395,
  395: "V1_4_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 396,
  396: "V1_4_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 397,
  397: "V1_4_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 398,
  398: "V1_4_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 399,
  399: "V1_4_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 400,
  400: "V1_4_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 401,
  401: "V1_4_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 402,
  402: "V1_4_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 403,
  403: "V1_4_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 404,
  404: "V1_4_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 405,
  405: "V1_4_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 406,
  406: "V1_4_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 407,
  407: "V1_4_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 408,
  408: "V1_4_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 409,
  409: "V1_4_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 410,
  410: "V1_4_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 411,
  411: "V1_4_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 412,
  412: "V1_4_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 413,
  413: "V1_4_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 414,
  414: "V1_4_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 415,
  415: "V1_4_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 416,
  416: "V1_4_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 417,
  417: "V1_4_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 418,
  418: "V1_4_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 419,
  419: "V1_4_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 420,
  420: "V1_4_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 421,
  421: "V1_4_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 422,
  422: "V1_4_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 423,
  423: "V1_4_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 424,
  424: "V1_4_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 425,
  425: "V1_4_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_4_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 426,
  426: "V1_4_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128: 427,
  427: "V1_5_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128: 428,
  428: "V1_5_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128: 429,
  429: "V1_5_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128: 430,
  430: "V1_5_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128: 431,
  431: "V1_5_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128: 432,
  432: "V1_5_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128: 433,
  433: "V1_5_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128: 434,
  434: "V1_5_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128: 435,
  435: "V1_5_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128: 436,
  436: "V1_5_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128: 437,
  437: "V1_5_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128: 438,
  438: "V1_5_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128: 439,
  439: "V1_5_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128: 440,
  440: "V1_5_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128: 441,
  441: "V1_5_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128: 442,
  442: "V1_5_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128: 443,
  443: "V1_5_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128: 444,
  444: "V1_5_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128: 445,
  445: "V1_5_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128: 446,
  446: "V1_5_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128: 447,
  447: "V1_5_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128: 448,
  448: "V1_5_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128: 449,
  449: "V1_5_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128: 450,
  450: "V1_5_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128: 451,
  451: "V1_5_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128: 452,
  452: "V1_5_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128: 453,
  453: "V1_5_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128: 454,
  454: "V1_5_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128: 455,
  455: "V1_5_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128: 456,
  456: "V1_5_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128: 457,
  457: "V1_5_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128: 458,
  458: "V1_5_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128: 459,
  459: "V1_5_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128: 460,
  460: "V1_5_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128: 461,
  461: "V1_5_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128: 462,
  462: "V1_5_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128: 463,
  463: "V1_5_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128: 464,
  464: "V1_5_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128: 465,
  465: "V1_5_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128: 466,
  466: "V1_5_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 467,
  467: "V1_5_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 468,
  468: "V1_5_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 469,
  469: "V1_5_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 470,
  470: "V1_5_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 471,
  471: "V1_5_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 472,
  472: "V1_5_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 473,
  473: "V1_5_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 474,
  474: "V1_5_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 475,
  475: "V1_5_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 476,
  476: "V1_5_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 477,
  477: "V1_5_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 478,
  478: "V1_5_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 479,
  479: "V1_5_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 480,
  480: "V1_5_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 481,
  481: "V1_5_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 482,
  482: "V1_5_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 483,
  483: "V1_5_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 484,
  484: "V1_5_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 485,
  485: "V1_5_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 486,
  486: "V1_5_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 487,
  487: "V1_5_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 488,
  488: "V1_5_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 489,
  489: "V1_5_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 490,
  490: "V1_5_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 491,
  491: "V1_5_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 492,
  492: "V1_5_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: 493,
  493: "V1_5_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 494,
  494: "V1_5_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 495,
  495: "V1_5_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 496,
  496: "V1_5_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
  V1_5_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: 497,
  497: "V1_5_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128",
});

export class ShortintPublicKey {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(ShortintPublicKey.prototype);
    obj.__wbg_ptr = ptr;
    ShortintPublicKeyFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    ShortintPublicKeyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_shortintpublickey_free(ptr, 0);
  }
}
if (Symbol.dispose)
  ShortintPublicKey.prototype[Symbol.dispose] =
    ShortintPublicKey.prototype.free;

export class TfheClientKey {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(TfheClientKey.prototype);
    obj.__wbg_ptr = ptr;
    TfheClientKeyFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    TfheClientKeyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_tfheclientkey_free(ptr, 0);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {TfheClientKey}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.tfheclientkey_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheClientKey.__wrap(ret[0]);
  }
  /**
   * @param {TfheConfig} config
   * @returns {TfheClientKey}
   */
  static generate(config) {
    _assertClass(config, TfheConfig);
    const ret = wasm.tfheclientkey_generate(config.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheClientKey.__wrap(ret[0]);
  }
  /**
   * @param {TfheConfig} config
   * @param {any} seed
   * @returns {TfheClientKey}
   */
  static generate_with_seed(config, seed) {
    _assertClass(config, TfheConfig);
    const ret = wasm.tfheclientkey_generate_with_seed(config.__wbg_ptr, seed);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheClientKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {TfheClientKey}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.tfheclientkey_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheClientKey.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.tfheclientkey_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.tfheclientkey_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  TfheClientKey.prototype[Symbol.dispose] = TfheClientKey.prototype.free;

export class TfheCompactPublicKey {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(TfheCompactPublicKey.prototype);
    obj.__wbg_ptr = ptr;
    TfheCompactPublicKeyFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    TfheCompactPublicKeyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_tfhecompactpublickey_free(ptr, 0);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {TfheCompactPublicKey}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.tfhecompactpublickey_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompactPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {TfheCompactPublicKey}
   */
  static new(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.tfhecompactpublickey_new(client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompactPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {TfheCompactPublicKey}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.tfhecompactpublickey_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompactPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @param {ShortintCompactPublicKeyEncryptionParameters} conformance_params
   * @returns {TfheCompactPublicKey}
   */
  static safe_deserialize_conformant(
    buffer,
    serialized_size_limit,
    conformance_params,
  ) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(
      conformance_params,
      ShortintCompactPublicKeyEncryptionParameters,
    );
    const ret = wasm.tfhecompactpublickey_safe_deserialize_conformant(
      ptr0,
      len0,
      serialized_size_limit,
      conformance_params.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompactPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.tfhecompactpublickey_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.tfhecompactpublickey_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  TfheCompactPublicKey.prototype[Symbol.dispose] =
    TfheCompactPublicKey.prototype.free;

export class TfheCompressedCompactPublicKey {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(TfheCompressedCompactPublicKey.prototype);
    obj.__wbg_ptr = ptr;
    TfheCompressedCompactPublicKeyFinalization.register(
      obj,
      obj.__wbg_ptr,
      obj,
    );
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    TfheCompressedCompactPublicKeyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_tfhecompressedcompactpublickey_free(ptr, 0);
  }
  /**
   * @returns {TfheCompactPublicKey}
   */
  decompress() {
    const ret = wasm.tfhecompressedcompactpublickey_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompactPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {TfheCompressedCompactPublicKey}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.tfhecompressedcompactpublickey_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompressedCompactPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {TfheCompressedCompactPublicKey}
   */
  static new(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.tfhecompressedcompactpublickey_new(client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompressedCompactPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {TfheCompressedCompactPublicKey}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.tfhecompressedcompactpublickey_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompressedCompactPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @param {ShortintCompactPublicKeyEncryptionParameters} conformance_params
   * @returns {TfheCompressedCompactPublicKey}
   */
  static safe_deserialize_conformant(
    buffer,
    serialized_size_limit,
    conformance_params,
  ) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(
      conformance_params,
      ShortintCompactPublicKeyEncryptionParameters,
    );
    const ret = wasm.tfhecompressedcompactpublickey_safe_deserialize_conformant(
      ptr0,
      len0,
      serialized_size_limit,
      conformance_params.__wbg_ptr,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompressedCompactPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.tfhecompressedcompactpublickey_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.tfhecompressedcompactpublickey_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  TfheCompressedCompactPublicKey.prototype[Symbol.dispose] =
    TfheCompressedCompactPublicKey.prototype.free;

export class TfheCompressedPublicKey {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(TfheCompressedPublicKey.prototype);
    obj.__wbg_ptr = ptr;
    TfheCompressedPublicKeyFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    TfheCompressedPublicKeyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_tfhecompressedpublickey_free(ptr, 0);
  }
  /**
   * @returns {TfhePublicKey}
   */
  decompress() {
    const ret = wasm.tfhecompressedpublickey_decompress(this.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfhePublicKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {TfheCompressedPublicKey}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.tfhecompressedpublickey_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompressedPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {TfheCompressedPublicKey}
   */
  static new(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.tfhecompressedpublickey_new(client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompressedPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {TfheCompressedPublicKey}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.tfhecompressedpublickey_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompressedPublicKey.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.tfhecompressedpublickey_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.tfhecompressedpublickey_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  TfheCompressedPublicKey.prototype[Symbol.dispose] =
    TfheCompressedPublicKey.prototype.free;

export class TfheCompressedServerKey {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(TfheCompressedServerKey.prototype);
    obj.__wbg_ptr = ptr;
    TfheCompressedServerKeyFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    TfheCompressedServerKeyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_tfhecompressedserverkey_free(ptr, 0);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {TfheCompressedServerKey}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.tfhecompressedserverkey_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompressedServerKey.__wrap(ret[0]);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {TfheCompressedServerKey}
   */
  static new(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.tfhecompressedserverkey_new(client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompressedServerKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {TfheCompressedServerKey}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.tfhecompressedserverkey_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheCompressedServerKey.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.tfhecompressedserverkey_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.tfhecompressedserverkey_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  TfheCompressedServerKey.prototype[Symbol.dispose] =
    TfheCompressedServerKey.prototype.free;

export class TfheConfig {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(TfheConfig.prototype);
    obj.__wbg_ptr = ptr;
    TfheConfigFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    TfheConfigFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_tfheconfig_free(ptr, 0);
  }
}
if (Symbol.dispose)
  TfheConfig.prototype[Symbol.dispose] = TfheConfig.prototype.free;

export class TfheConfigBuilder {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(TfheConfigBuilder.prototype);
    obj.__wbg_ptr = ptr;
    TfheConfigBuilderFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    TfheConfigBuilderFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_tfheconfigbuilder_free(ptr, 0);
  }
  /**
   * @returns {TfheConfig}
   */
  build() {
    const ptr = this.__destroy_into_raw();
    const ret = wasm.tfheconfigbuilder_build(ptr);
    return TfheConfig.__wrap(ret);
  }
  /**
   * @returns {TfheConfigBuilder}
   */
  static default() {
    const ret = wasm.tfheconfigbuilder_default();
    return TfheConfigBuilder.__wrap(ret);
  }
  /**
   * @param {ShortintParameters} block_parameters
   * @returns {TfheConfigBuilder}
   */
  use_custom_parameters(block_parameters) {
    const ptr = this.__destroy_into_raw();
    _assertClass(block_parameters, ShortintParameters);
    const ret = wasm.tfheconfigbuilder_use_custom_parameters(
      ptr,
      block_parameters.__wbg_ptr,
    );
    return TfheConfigBuilder.__wrap(ret);
  }
  /**
   * @param {ShortintCompactPublicKeyEncryptionParameters} compact_public_key_parameters
   * @returns {TfheConfigBuilder}
   */
  use_dedicated_compact_public_key_parameters(compact_public_key_parameters) {
    const ptr = this.__destroy_into_raw();
    _assertClass(
      compact_public_key_parameters,
      ShortintCompactPublicKeyEncryptionParameters,
    );
    const ret =
      wasm.tfheconfigbuilder_use_dedicated_compact_public_key_parameters(
        ptr,
        compact_public_key_parameters.__wbg_ptr,
      );
    return TfheConfigBuilder.__wrap(ret);
  }
  /**
   * @param {ShortintParameters} block_parameters
   * @returns {TfheConfigBuilder}
   */
  static with_custom_parameters(block_parameters) {
    _assertClass(block_parameters, ShortintParameters);
    const ret = wasm.tfheconfigbuilder_with_custom_parameters(
      block_parameters.__wbg_ptr,
    );
    return TfheConfigBuilder.__wrap(ret);
  }
}
if (Symbol.dispose)
  TfheConfigBuilder.prototype[Symbol.dispose] =
    TfheConfigBuilder.prototype.free;

export class TfhePublicKey {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(TfhePublicKey.prototype);
    obj.__wbg_ptr = ptr;
    TfhePublicKeyFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    TfhePublicKeyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_tfhepublickey_free(ptr, 0);
  }
  /**
   * @param {Uint8Array} buffer
   * @returns {TfhePublicKey}
   */
  static deserialize(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.tfhepublickey_deserialize(ptr0, len0);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfhePublicKey.__wrap(ret[0]);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {TfhePublicKey}
   */
  static new(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.tfhepublickey_new(client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfhePublicKey.__wrap(ret[0]);
  }
  /**
   * @param {Uint8Array} buffer
   * @param {bigint} serialized_size_limit
   * @returns {TfhePublicKey}
   */
  static safe_deserialize(buffer, serialized_size_limit) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.tfhepublickey_safe_deserialize(
      ptr0,
      len0,
      serialized_size_limit,
    );
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfhePublicKey.__wrap(ret[0]);
  }
  /**
   * @param {bigint} serialized_size_limit
   * @returns {Uint8Array}
   */
  safe_serialize(serialized_size_limit) {
    const ret = wasm.tfhepublickey_safe_serialize(
      this.__wbg_ptr,
      serialized_size_limit,
    );
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * @returns {Uint8Array}
   */
  serialize() {
    const ret = wasm.tfhepublickey_serialize(this.__wbg_ptr);
    if (ret[3]) {
      throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
}
if (Symbol.dispose)
  TfhePublicKey.prototype[Symbol.dispose] = TfhePublicKey.prototype.free;

export class TfheServerKey {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(TfheServerKey.prototype);
    obj.__wbg_ptr = ptr;
    TfheServerKeyFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    TfheServerKeyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_tfheserverkey_free(ptr, 0);
  }
  /**
   * @param {TfheClientKey} client_key
   * @returns {TfheServerKey}
   */
  static new(client_key) {
    _assertClass(client_key, TfheClientKey);
    const ret = wasm.tfheserverkey_new(client_key.__wbg_ptr);
    if (ret[2]) {
      throw takeFromExternrefTable0(ret[1]);
    }
    return TfheServerKey.__wrap(ret[0]);
  }
}
if (Symbol.dispose)
  TfheServerKey.prototype[Symbol.dispose] = TfheServerKey.prototype.free;

/**
 * @enum {0 | 1}
 */
export const ZkComputeLoad = Object.freeze({
  Proof: 0,
  0: "Proof",
  Verify: 1,
  1: "Verify",
});

/**
 * @param {number} num_threads
 * @returns {Promise<any>}
 */
export function initThreadPool(num_threads) {
  const ret = wasm.initThreadPool(num_threads);
  return ret;
}

export function init_panic_hook() {
  wasm.init_panic_hook();
}

/**
 * @param {TfheServerKey} server_key
 */
export function set_server_key(server_key) {
  _assertClass(server_key, TfheServerKey);
  const ret = wasm.set_server_key(server_key.__wbg_ptr);
  if (ret[1]) {
    throw takeFromExternrefTable0(ret[0]);
  }
}

/**
 * @param {ShortintParametersName | null} [param]
 * @returns {string}
 */
export function shortint_params_name(param) {
  let deferred2_0;
  let deferred2_1;
  try {
    const ret = wasm.shortint_params_name(isLikeNone(param) ? 498 : param);
    var ptr1 = ret[0];
    var len1 = ret[1];
    if (ret[3]) {
      ptr1 = 0;
      len1 = 0;
      throw takeFromExternrefTable0(ret[2]);
    }
    deferred2_0 = ptr1;
    deferred2_1 = len1;
    return getStringFromWasm0(ptr1, len1);
  } finally {
    wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
  }
}

/**
 * @param {ShortintCompactPublicKeyEncryptionParametersName} param
 * @returns {string}
 */
export function shortint_pke_params_name(param) {
  let deferred1_0;
  let deferred1_1;
  try {
    const ret = wasm.shortint_pke_params_name(param);
    deferred1_0 = ret[0];
    deferred1_1 = ret[1];
    return getStringFromWasm0(ret[0], ret[1]);
  } finally {
    wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
  }
}

export class tfhe {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    tfheFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_tfhe_free(ptr, 0);
  }
}
if (Symbol.dispose) tfhe.prototype[Symbol.dispose] = tfhe.prototype.free;

export class wbg_rayon_PoolBuilder {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(wbg_rayon_PoolBuilder.prototype);
    obj.__wbg_ptr = ptr;
    wbg_rayon_PoolBuilderFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    wbg_rayon_PoolBuilderFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_wbg_rayon_poolbuilder_free(ptr, 0);
  }
  build() {
    wasm.wbg_rayon_poolbuilder_build(this.__wbg_ptr);
  }
  /**
   * @returns {number}
   */
  numThreads() {
    const ret = wasm.wbg_rayon_poolbuilder_numThreads(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * @returns {number}
   */
  receiver() {
    const ret = wasm.wbg_rayon_poolbuilder_receiver(this.__wbg_ptr);
    return ret >>> 0;
  }
}
if (Symbol.dispose)
  wbg_rayon_PoolBuilder.prototype[Symbol.dispose] =
    wbg_rayon_PoolBuilder.prototype.free;

/**
 * @param {number} receiver
 */
export function wbg_rayon_start_worker(receiver) {
  wasm.wbg_rayon_start_worker(receiver);
}

function __wbg_get_imports(memory) {
  const import0 = {
    __proto__: null,
    __wbg_BigInt_7ea1e749ce6b92fd: function () {
      return handleError(function (arg0) {
        const ret = BigInt(arg0);
        return ret;
      }, arguments);
    },
    __wbg_BigInt_b7bbccdff258c9f2: function (arg0) {
      const ret = BigInt(arg0);
      return ret;
    },
    __wbg_Error_8c4e43fe74559d73: function (arg0, arg1) {
      const ret = Error(getStringFromWasm0(arg0, arg1));
      return ret;
    },
    __wbg___wbindgen_bigint_get_as_i64_8fcf4ce7f1ca72a2: function (arg0, arg1) {
      const v = arg1;
      const ret = typeof v === "bigint" ? v : undefined;
      getDataViewMemory0().setBigInt64(
        arg0 + 8 * 1,
        isLikeNone(ret) ? BigInt(0) : ret,
        true,
      );
      getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
    },
    __wbg___wbindgen_bit_and_4362b1176950d42a: function (arg0, arg1) {
      const ret = arg0 & arg1;
      return ret;
    },
    __wbg___wbindgen_bit_or_71a02d39796ea13d: function (arg0, arg1) {
      const ret = arg0 | arg1;
      return ret;
    },
    __wbg___wbindgen_debug_string_0bc8482c6e3508ae: function (arg0, arg1) {
      const ret = debugString(arg1);
      const ptr1 = passStringToWasm0(
        ret,
        wasm.__wbindgen_malloc,
        wasm.__wbindgen_realloc,
      );
      const len1 = WASM_VECTOR_LEN;
      getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
      getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    },
    __wbg___wbindgen_is_function_0095a73b8b156f76: function (arg0) {
      const ret = typeof arg0 === "function";
      return ret;
    },
    __wbg___wbindgen_is_object_5ae8e5880f2c1fbd: function (arg0) {
      const val = arg0;
      const ret = typeof val === "object" && val !== null;
      return ret;
    },
    __wbg___wbindgen_is_string_cd444516edc5b180: function (arg0) {
      const ret = typeof arg0 === "string";
      return ret;
    },
    __wbg___wbindgen_is_undefined_9e4d92534c42d778: function (arg0) {
      const ret = arg0 === undefined;
      return ret;
    },
    __wbg___wbindgen_jsval_eq_11888390b0186270: function (arg0, arg1) {
      const ret = arg0 === arg1;
      return ret;
    },
    __wbg___wbindgen_lt_bb59cc3d23526e0d: function (arg0, arg1) {
      const ret = arg0 < arg1;
      return ret;
    },
    __wbg___wbindgen_memory_bd1fbcf21fbef3c8: function () {
      const ret = wasm.memory;
      return ret;
    },
    __wbg___wbindgen_module_f6b8052d79c1cc16: function () {
      const ret = wasmModule;
      return ret;
    },
    __wbg___wbindgen_neg_6b4d356dff49dcc6: function (arg0) {
      const ret = -arg0;
      return ret;
    },
    __wbg___wbindgen_shl_8d64d06761f9ea4e: function (arg0, arg1) {
      const ret = arg0 << arg1;
      return ret;
    },
    __wbg___wbindgen_shr_ef8e07cce709eb54: function (arg0, arg1) {
      const ret = arg0 >> arg1;
      return ret;
    },
    __wbg___wbindgen_string_get_72fb696202c56729: function (arg0, arg1) {
      const obj = arg1;
      const ret = typeof obj === "string" ? obj : undefined;
      var ptr1 = isLikeNone(ret)
        ? 0
        : passStringToWasm0(
            ret,
            wasm.__wbindgen_malloc,
            wasm.__wbindgen_realloc,
          );
      var len1 = WASM_VECTOR_LEN;
      getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
      getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    },
    __wbg___wbindgen_throw_be289d5034ed271b: function (arg0, arg1) {
      throw new Error(getStringFromWasm0(arg0, arg1));
    },
    __wbg_call_389efe28435a9388: function () {
      return handleError(function (arg0, arg1) {
        const ret = arg0.call(arg1);
        return ret;
      }, arguments);
    },
    __wbg_call_4708e0c13bdc8e95: function () {
      return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.call(arg1, arg2);
        return ret;
      }, arguments);
    },
    __wbg_crypto_86f2631e91b51511: function (arg0) {
      const ret = arg0.crypto;
      return ret;
    },
    __wbg_error_7534b8e9a36f1ab4: function (arg0, arg1) {
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
    __wbg_getRandomValues_b3f15fcbfabb0f8b: function () {
      return handleError(function (arg0, arg1) {
        arg0.getRandomValues(arg1);
      }, arguments);
    },
    __wbg_getTime_1e3cd1391c5c3995: function (arg0) {
      const ret = arg0.getTime();
      return ret;
    },
    __wbg_instanceof_Window_ed49b2db8df90359: function (arg0) {
      let result;
      try {
        result = arg0 instanceof Window;
      } catch (_) {
        result = false;
      }
      const ret = result;
      return ret;
    },
    __wbg_length_32ed9a279acd054c: function (arg0) {
      const ret = arg0.length;
      return ret;
    },
    __wbg_msCrypto_d562bbe83e0d4b91: function (arg0) {
      const ret = arg0.msCrypto;
      return ret;
    },
    __wbg_new_0_73afc35eb544e539: function () {
      const ret = new Date();
      return ret;
    },
    __wbg_new_8a6f238a6ece86ea: function () {
      const ret = new Error();
      return ret;
    },
    __wbg_new_no_args_1c7c842f08d00ebb: function (arg0, arg1) {
      const ret = new Function(getStringFromWasm0(arg0, arg1));
      return ret;
    },
    __wbg_new_with_length_a2c39cbe88fd8ff1: function (arg0) {
      const ret = new Uint8Array(arg0 >>> 0);
      return ret;
    },
    __wbg_node_e1f24f89a7336c2e: function (arg0) {
      const ret = arg0.node;
      return ret;
    },
    __wbg_process_3975fd6c72f520aa: function (arg0) {
      const ret = arg0.process;
      return ret;
    },
    __wbg_prototypesetcall_bdcdcc5842e4d77d: function (arg0, arg1, arg2) {
      Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
    },
    __wbg_randomFillSync_f8c153b79f285817: function () {
      return handleError(function (arg0, arg1) {
        arg0.randomFillSync(arg1);
      }, arguments);
    },
    __wbg_require_b74f47fc2d022fd6: function () {
      return handleError(function () {
        const ret = module.require;
        return ret;
      }, arguments);
    },
    __wbg_stack_0ed75d68575b0f3c: function (arg0, arg1) {
      const ret = arg1.stack;
      const ptr1 = passStringToWasm0(
        ret,
        wasm.__wbindgen_malloc,
        wasm.__wbindgen_realloc,
      );
      const len1 = WASM_VECTOR_LEN;
      getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
      getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    },
    __wbg_startWorkers_2ca11761e08ff5d5: function (arg0, arg1, arg2) {
      const ret = startWorkers(arg0, arg1, wbg_rayon_PoolBuilder.__wrap(arg2));
      return ret;
    },
    __wbg_static_accessor_GLOBAL_12837167ad935116: function () {
      const ret = typeof global === "undefined" ? null : global;
      return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    },
    __wbg_static_accessor_GLOBAL_THIS_e628e89ab3b1c95f: function () {
      const ret = typeof globalThis === "undefined" ? null : globalThis;
      return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    },
    __wbg_static_accessor_SELF_a621d3dfbb60d0ce: function () {
      const ret = typeof self === "undefined" ? null : self;
      return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    },
    __wbg_static_accessor_WINDOW_f8727f0cf888e0bd: function () {
      const ret = typeof window === "undefined" ? null : window;
      return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    },
    __wbg_subarray_a96e1fef17ed23cb: function (arg0, arg1, arg2) {
      const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
      return ret;
    },
    __wbg_toString_029ac24421fd7a24: function (arg0) {
      const ret = arg0.toString();
      return ret;
    },
    __wbg_toString_56d946daff83867b: function (arg0, arg1, arg2) {
      const ret = arg1.toString(arg2);
      const ptr1 = passStringToWasm0(
        ret,
        wasm.__wbindgen_malloc,
        wasm.__wbindgen_realloc,
      );
      const len1 = WASM_VECTOR_LEN;
      getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
      getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    },
    __wbg_versions_4e31226f5e8dc909: function (arg0) {
      const ret = arg0.versions;
      return ret;
    },
    __wbindgen_cast_0000000000000001: function (arg0) {
      // Cast intrinsic for `F64 -> Externref`.
      const ret = arg0;
      return ret;
    },
    __wbindgen_cast_0000000000000002: function (arg0, arg1) {
      // Cast intrinsic for `I128 -> Externref`.
      const ret = BigInt.asUintN(64, arg0) | (arg1 << BigInt(64));
      return ret;
    },
    __wbindgen_cast_0000000000000003: function (arg0) {
      // Cast intrinsic for `I64 -> Externref`.
      const ret = arg0;
      return ret;
    },
    __wbindgen_cast_0000000000000004: function (arg0, arg1) {
      // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
      const ret = getArrayU8FromWasm0(arg0, arg1);
      return ret;
    },
    __wbindgen_cast_0000000000000005: function (arg0, arg1) {
      // Cast intrinsic for `Ref(String) -> Externref`.
      const ret = getStringFromWasm0(arg0, arg1);
      return ret;
    },
    __wbindgen_cast_0000000000000006: function (arg0, arg1) {
      // Cast intrinsic for `U128 -> Externref`.
      const ret =
        BigInt.asUintN(64, arg0) | (BigInt.asUintN(64, arg1) << BigInt(64));
      return ret;
    },
    __wbindgen_cast_0000000000000007: function (arg0) {
      // Cast intrinsic for `U64 -> Externref`.
      const ret = BigInt.asUintN(64, arg0);
      return ret;
    },
    __wbindgen_init_externref_table: function () {
      const table = wasm.__wbindgen_externrefs;
      const offset = table.grow(4);
      table.set(0, undefined);
      table.set(offset + 0, undefined);
      table.set(offset + 1, null);
      table.set(offset + 2, true);
      table.set(offset + 3, false);
    },
    memory:
      memory ||
      new WebAssembly.Memory({ initial: 21, maximum: 16384, shared: true }),
  };
  return {
    __proto__: null,
    "./tfhe_bg.js": import0,
  };
}

const BooleanFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_boolean_free(ptr >>> 0, 1));
const BooleanCiphertextFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_booleanciphertext_free(ptr >>> 0, 1),
      );
const BooleanClientKeyFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_booleanclientkey_free(ptr >>> 0, 1),
      );
const BooleanCompressedCiphertextFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_booleancompressedciphertext_free(ptr >>> 0, 1),
      );
const BooleanCompressedServerKeyFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_booleancompressedserverkey_free(ptr >>> 0, 1),
      );
const BooleanNoiseDistributionFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_booleannoisedistribution_free(ptr >>> 0, 1),
      );
const BooleanParametersFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_booleanparameters_free(ptr >>> 0, 1),
      );
const BooleanPublicKeyFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_booleanpublickey_free(ptr >>> 0, 1),
      );
const CompactCiphertextListFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compactciphertextlist_free(ptr >>> 0, 1),
      );
const CompactCiphertextListBuilderFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compactciphertextlistbuilder_free(ptr >>> 0, 1),
      );
const CompactCiphertextListExpanderFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compactciphertextlistexpander_free(ptr >>> 0, 1),
      );
const CompactPkeCrsFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compactpkecrs_free(ptr >>> 0, 1),
      );
const CompressedFheBoolFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfhebool_free(ptr >>> 0, 1),
      );
const CompressedFheInt10Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint10_free(ptr >>> 0, 1),
      );
const CompressedFheInt1024Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint1024_free(ptr >>> 0, 1),
      );
const CompressedFheInt104Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint104_free(ptr >>> 0, 1),
      );
const CompressedFheInt112Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint112_free(ptr >>> 0, 1),
      );
const CompressedFheInt12Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint12_free(ptr >>> 0, 1),
      );
const CompressedFheInt120Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint120_free(ptr >>> 0, 1),
      );
const CompressedFheInt128Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint128_free(ptr >>> 0, 1),
      );
const CompressedFheInt136Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint136_free(ptr >>> 0, 1),
      );
const CompressedFheInt14Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint14_free(ptr >>> 0, 1),
      );
const CompressedFheInt144Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint144_free(ptr >>> 0, 1),
      );
const CompressedFheInt152Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint152_free(ptr >>> 0, 1),
      );
const CompressedFheInt16Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint16_free(ptr >>> 0, 1),
      );
const CompressedFheInt160Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint160_free(ptr >>> 0, 1),
      );
const CompressedFheInt168Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint168_free(ptr >>> 0, 1),
      );
const CompressedFheInt176Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint176_free(ptr >>> 0, 1),
      );
const CompressedFheInt184Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint184_free(ptr >>> 0, 1),
      );
const CompressedFheInt192Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint192_free(ptr >>> 0, 1),
      );
const CompressedFheInt2Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint2_free(ptr >>> 0, 1),
      );
const CompressedFheInt200Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint200_free(ptr >>> 0, 1),
      );
const CompressedFheInt2048Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint2048_free(ptr >>> 0, 1),
      );
const CompressedFheInt208Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint208_free(ptr >>> 0, 1),
      );
const CompressedFheInt216Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint216_free(ptr >>> 0, 1),
      );
const CompressedFheInt224Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint224_free(ptr >>> 0, 1),
      );
const CompressedFheInt232Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint232_free(ptr >>> 0, 1),
      );
const CompressedFheInt24Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint24_free(ptr >>> 0, 1),
      );
const CompressedFheInt240Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint240_free(ptr >>> 0, 1),
      );
const CompressedFheInt248Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint248_free(ptr >>> 0, 1),
      );
const CompressedFheInt256Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint256_free(ptr >>> 0, 1),
      );
const CompressedFheInt32Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint32_free(ptr >>> 0, 1),
      );
const CompressedFheInt4Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint4_free(ptr >>> 0, 1),
      );
const CompressedFheInt40Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint40_free(ptr >>> 0, 1),
      );
const CompressedFheInt48Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint48_free(ptr >>> 0, 1),
      );
const CompressedFheInt512Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint512_free(ptr >>> 0, 1),
      );
const CompressedFheInt56Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint56_free(ptr >>> 0, 1),
      );
const CompressedFheInt6Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint6_free(ptr >>> 0, 1),
      );
const CompressedFheInt64Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint64_free(ptr >>> 0, 1),
      );
const CompressedFheInt72Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint72_free(ptr >>> 0, 1),
      );
const CompressedFheInt8Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint8_free(ptr >>> 0, 1),
      );
const CompressedFheInt80Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint80_free(ptr >>> 0, 1),
      );
const CompressedFheInt88Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint88_free(ptr >>> 0, 1),
      );
const CompressedFheInt96Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheint96_free(ptr >>> 0, 1),
      );
const CompressedFheUint10Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint10_free(ptr >>> 0, 1),
      );
const CompressedFheUint1024Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint1024_free(ptr >>> 0, 1),
      );
const CompressedFheUint104Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint104_free(ptr >>> 0, 1),
      );
const CompressedFheUint112Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint112_free(ptr >>> 0, 1),
      );
const CompressedFheUint12Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint12_free(ptr >>> 0, 1),
      );
const CompressedFheUint120Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint120_free(ptr >>> 0, 1),
      );
const CompressedFheUint128Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint128_free(ptr >>> 0, 1),
      );
const CompressedFheUint136Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint136_free(ptr >>> 0, 1),
      );
const CompressedFheUint14Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint14_free(ptr >>> 0, 1),
      );
const CompressedFheUint144Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint144_free(ptr >>> 0, 1),
      );
const CompressedFheUint152Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint152_free(ptr >>> 0, 1),
      );
const CompressedFheUint16Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint16_free(ptr >>> 0, 1),
      );
const CompressedFheUint160Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint160_free(ptr >>> 0, 1),
      );
const CompressedFheUint168Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint168_free(ptr >>> 0, 1),
      );
const CompressedFheUint176Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint176_free(ptr >>> 0, 1),
      );
const CompressedFheUint184Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint184_free(ptr >>> 0, 1),
      );
const CompressedFheUint192Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint192_free(ptr >>> 0, 1),
      );
const CompressedFheUint2Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint2_free(ptr >>> 0, 1),
      );
const CompressedFheUint200Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint200_free(ptr >>> 0, 1),
      );
const CompressedFheUint2048Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint2048_free(ptr >>> 0, 1),
      );
const CompressedFheUint208Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint208_free(ptr >>> 0, 1),
      );
const CompressedFheUint216Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint216_free(ptr >>> 0, 1),
      );
const CompressedFheUint224Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint224_free(ptr >>> 0, 1),
      );
const CompressedFheUint232Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint232_free(ptr >>> 0, 1),
      );
const CompressedFheUint24Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint24_free(ptr >>> 0, 1),
      );
const CompressedFheUint240Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint240_free(ptr >>> 0, 1),
      );
const CompressedFheUint248Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint248_free(ptr >>> 0, 1),
      );
const CompressedFheUint256Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint256_free(ptr >>> 0, 1),
      );
const CompressedFheUint32Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint32_free(ptr >>> 0, 1),
      );
const CompressedFheUint4Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint4_free(ptr >>> 0, 1),
      );
const CompressedFheUint40Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint40_free(ptr >>> 0, 1),
      );
const CompressedFheUint48Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint48_free(ptr >>> 0, 1),
      );
const CompressedFheUint512Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint512_free(ptr >>> 0, 1),
      );
const CompressedFheUint56Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint56_free(ptr >>> 0, 1),
      );
const CompressedFheUint6Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint6_free(ptr >>> 0, 1),
      );
const CompressedFheUint64Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint64_free(ptr >>> 0, 1),
      );
const CompressedFheUint72Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint72_free(ptr >>> 0, 1),
      );
const CompressedFheUint8Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint8_free(ptr >>> 0, 1),
      );
const CompressedFheUint80Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint80_free(ptr >>> 0, 1),
      );
const CompressedFheUint88Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint88_free(ptr >>> 0, 1),
      );
const CompressedFheUint96Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_compressedfheuint96_free(ptr >>> 0, 1),
      );
const FheBoolFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fhebool_free(ptr >>> 0, 1));
const FheInt10Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint10_free(ptr >>> 0, 1));
const FheInt1024Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint1024_free(ptr >>> 0, 1),
      );
const FheInt104Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint104_free(ptr >>> 0, 1),
      );
const FheInt112Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint112_free(ptr >>> 0, 1),
      );
const FheInt12Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint12_free(ptr >>> 0, 1));
const FheInt120Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint120_free(ptr >>> 0, 1),
      );
const FheInt128Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint128_free(ptr >>> 0, 1),
      );
const FheInt136Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint136_free(ptr >>> 0, 1),
      );
const FheInt14Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint14_free(ptr >>> 0, 1));
const FheInt144Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint144_free(ptr >>> 0, 1),
      );
const FheInt152Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint152_free(ptr >>> 0, 1),
      );
const FheInt16Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint16_free(ptr >>> 0, 1));
const FheInt160Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint160_free(ptr >>> 0, 1),
      );
const FheInt168Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint168_free(ptr >>> 0, 1),
      );
const FheInt176Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint176_free(ptr >>> 0, 1),
      );
const FheInt184Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint184_free(ptr >>> 0, 1),
      );
const FheInt192Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint192_free(ptr >>> 0, 1),
      );
const FheInt2Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint2_free(ptr >>> 0, 1));
const FheInt200Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint200_free(ptr >>> 0, 1),
      );
const FheInt2048Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint2048_free(ptr >>> 0, 1),
      );
const FheInt208Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint208_free(ptr >>> 0, 1),
      );
const FheInt216Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint216_free(ptr >>> 0, 1),
      );
const FheInt224Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint224_free(ptr >>> 0, 1),
      );
const FheInt232Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint232_free(ptr >>> 0, 1),
      );
const FheInt24Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint24_free(ptr >>> 0, 1));
const FheInt240Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint240_free(ptr >>> 0, 1),
      );
const FheInt248Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint248_free(ptr >>> 0, 1),
      );
const FheInt256Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint256_free(ptr >>> 0, 1),
      );
const FheInt32Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint32_free(ptr >>> 0, 1));
const FheInt4Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint4_free(ptr >>> 0, 1));
const FheInt40Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint40_free(ptr >>> 0, 1));
const FheInt48Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint48_free(ptr >>> 0, 1));
const FheInt512Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheint512_free(ptr >>> 0, 1),
      );
const FheInt56Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint56_free(ptr >>> 0, 1));
const FheInt6Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint6_free(ptr >>> 0, 1));
const FheInt64Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint64_free(ptr >>> 0, 1));
const FheInt72Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint72_free(ptr >>> 0, 1));
const FheInt8Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint8_free(ptr >>> 0, 1));
const FheInt80Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint80_free(ptr >>> 0, 1));
const FheInt88Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint88_free(ptr >>> 0, 1));
const FheInt96Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheint96_free(ptr >>> 0, 1));
const FheUint10Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint10_free(ptr >>> 0, 1),
      );
const FheUint1024Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint1024_free(ptr >>> 0, 1),
      );
const FheUint104Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint104_free(ptr >>> 0, 1),
      );
const FheUint112Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint112_free(ptr >>> 0, 1),
      );
const FheUint12Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint12_free(ptr >>> 0, 1),
      );
const FheUint120Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint120_free(ptr >>> 0, 1),
      );
const FheUint128Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint128_free(ptr >>> 0, 1),
      );
const FheUint136Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint136_free(ptr >>> 0, 1),
      );
const FheUint14Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint14_free(ptr >>> 0, 1),
      );
const FheUint144Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint144_free(ptr >>> 0, 1),
      );
const FheUint152Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint152_free(ptr >>> 0, 1),
      );
const FheUint16Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint16_free(ptr >>> 0, 1),
      );
const FheUint160Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint160_free(ptr >>> 0, 1),
      );
const FheUint168Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint168_free(ptr >>> 0, 1),
      );
const FheUint176Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint176_free(ptr >>> 0, 1),
      );
const FheUint184Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint184_free(ptr >>> 0, 1),
      );
const FheUint192Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint192_free(ptr >>> 0, 1),
      );
const FheUint2Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheuint2_free(ptr >>> 0, 1));
const FheUint200Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint200_free(ptr >>> 0, 1),
      );
const FheUint2048Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint2048_free(ptr >>> 0, 1),
      );
const FheUint208Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint208_free(ptr >>> 0, 1),
      );
const FheUint216Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint216_free(ptr >>> 0, 1),
      );
const FheUint224Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint224_free(ptr >>> 0, 1),
      );
const FheUint232Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint232_free(ptr >>> 0, 1),
      );
const FheUint24Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint24_free(ptr >>> 0, 1),
      );
const FheUint240Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint240_free(ptr >>> 0, 1),
      );
const FheUint248Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint248_free(ptr >>> 0, 1),
      );
const FheUint256Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint256_free(ptr >>> 0, 1),
      );
const FheUint32Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint32_free(ptr >>> 0, 1),
      );
const FheUint4Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheuint4_free(ptr >>> 0, 1));
const FheUint40Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint40_free(ptr >>> 0, 1),
      );
const FheUint48Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint48_free(ptr >>> 0, 1),
      );
const FheUint512Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint512_free(ptr >>> 0, 1),
      );
const FheUint56Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint56_free(ptr >>> 0, 1),
      );
const FheUint6Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheuint6_free(ptr >>> 0, 1));
const FheUint64Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint64_free(ptr >>> 0, 1),
      );
const FheUint72Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint72_free(ptr >>> 0, 1),
      );
const FheUint8Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_fheuint8_free(ptr >>> 0, 1));
const FheUint80Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint80_free(ptr >>> 0, 1),
      );
const FheUint88Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint88_free(ptr >>> 0, 1),
      );
const FheUint96Finalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_fheuint96_free(ptr >>> 0, 1),
      );
const ProvenCompactCiphertextListFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_provencompactciphertextlist_free(ptr >>> 0, 1),
      );
const ShortintFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_shortint_free(ptr >>> 0, 1));
const ShortintCiphertextFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_shortintciphertext_free(ptr >>> 0, 1),
      );
const ShortintClientKeyFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_shortintclientkey_free(ptr >>> 0, 1),
      );
const ShortintCompactPublicKeyEncryptionParametersFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_shortintcompactpublickeyencryptionparameters_free(
          ptr >>> 0,
          1,
        ),
      );
const ShortintCompressedCiphertextFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_shortintcompressedciphertext_free(ptr >>> 0, 1),
      );
const ShortintCompressedPublicKeyFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_shortintcompressedpublickey_free(ptr >>> 0, 1),
      );
const ShortintCompressedServerKeyFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_shortintcompressedserverkey_free(ptr >>> 0, 1),
      );
const ShortintNoiseDistributionFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_shortintnoisedistribution_free(ptr >>> 0, 1),
      );
const ShortintParametersFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_shortintparameters_free(ptr >>> 0, 1),
      );
const ShortintPublicKeyFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_shortintpublickey_free(ptr >>> 0, 1),
      );
const TfheClientKeyFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_tfheclientkey_free(ptr >>> 0, 1),
      );
const TfheCompactPublicKeyFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_tfhecompactpublickey_free(ptr >>> 0, 1),
      );
const TfheCompressedCompactPublicKeyFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_tfhecompressedcompactpublickey_free(ptr >>> 0, 1),
      );
const TfheCompressedPublicKeyFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_tfhecompressedpublickey_free(ptr >>> 0, 1),
      );
const TfheCompressedServerKeyFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_tfhecompressedserverkey_free(ptr >>> 0, 1),
      );
const TfheConfigFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_tfheconfig_free(ptr >>> 0, 1),
      );
const TfheConfigBuilderFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_tfheconfigbuilder_free(ptr >>> 0, 1),
      );
const TfhePublicKeyFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_tfhepublickey_free(ptr >>> 0, 1),
      );
const TfheServerKeyFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_tfheserverkey_free(ptr >>> 0, 1),
      );
const tfheFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) => wasm.__wbg_tfhe_free(ptr >>> 0, 1));
const wbg_rayon_PoolBuilderFinalization =
  typeof FinalizationRegistry === "undefined"
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((ptr) =>
        wasm.__wbg_wbg_rayon_poolbuilder_free(ptr >>> 0, 1),
      );

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
  if (type == "number" || type == "boolean" || val == null) {
    return `${val}`;
  }
  if (type == "string") {
    return `"${val}"`;
  }
  if (type == "symbol") {
    const description = val.description;
    if (description == null) {
      return "Symbol";
    } else {
      return `Symbol(${description})`;
    }
  }
  if (type == "function") {
    const name = val.name;
    if (typeof name == "string" && name.length > 0) {
      return `Function(${name})`;
    } else {
      return "Function";
    }
  }
  // objects
  if (Array.isArray(val)) {
    const length = val.length;
    let debug = "[";
    if (length > 0) {
      debug += debugString(val[0]);
    }
    for (let i = 1; i < length; i++) {
      debug += ", " + debugString(val[i]);
    }
    debug += "]";
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
  if (className == "Object") {
    // we're a user defined class or Object
    // JSON.stringify avoids problems with cycles, and is generally much
    // easier than looping through ownProperties of `val`.
    try {
      return "Object(" + JSON.stringify(val) + ")";
    } catch (_) {
      return "Object";
    }
  }
  // errors
  if (val instanceof Error) {
    return `${val.name}: ${val.message}\n${val.stack}`;
  }
  // TODO we could test for more things here, like `Set`s and `Map`s.
  return className;
}

function getArrayU8FromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
  if (
    cachedDataViewMemory0 === null ||
    cachedDataViewMemory0.buffer !== wasm.memory.buffer
  ) {
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
  if (
    cachedUint8ArrayMemory0 === null ||
    cachedUint8ArrayMemory0.buffer !== wasm.memory.buffer
  ) {
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

function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === undefined) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr = malloc(buf.length, 1) >>> 0;
    getUint8ArrayMemory0()
      .subarray(ptr, ptr + buf.length)
      .set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
  }

  let len = arg.length;
  let ptr = malloc(len, 1) >>> 0;

  const mem = getUint8ArrayMemory0();

  let offset = 0;

  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 0x7f) break;
    mem[ptr + offset] = code;
  }
  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, (len = offset + arg.length * 3), 1) >>> 0;
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

let cachedTextDecoder =
  typeof TextDecoder !== "undefined"
    ? new TextDecoder("utf-8", { ignoreBOM: true, fatal: true })
    : undefined;
if (cachedTextDecoder) cachedTextDecoder.decode();

const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
  numBytesDecoded += len;
  if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
    cachedTextDecoder = new TextDecoder("utf-8", {
      ignoreBOM: true,
      fatal: true,
    });
    cachedTextDecoder.decode();
    numBytesDecoded = len;
  }
  return cachedTextDecoder.decode(getUint8ArrayMemory0().slice(ptr, ptr + len));
}

const cachedTextEncoder =
  typeof TextEncoder !== "undefined" ? new TextEncoder() : undefined;

if (cachedTextEncoder) {
  cachedTextEncoder.encodeInto = function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
      read: arg.length,
      written: buf.length,
    };
  };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module, thread_stack_size) {
  wasm = instance.exports;
  wasmModule = module;
  cachedDataViewMemory0 = null;
  cachedUint8ArrayMemory0 = null;
  if (
    typeof thread_stack_size !== "undefined" &&
    (typeof thread_stack_size !== "number" ||
      thread_stack_size === 0 ||
      thread_stack_size % 65536 !== 0)
  ) {
    throw "invalid stack size";
  }
  wasm.__wbindgen_start(thread_stack_size);
  return wasm;
}

async function __wbg_load(module, imports) {
  if (typeof Response === "function" && module instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming === "function") {
      try {
        return await WebAssembly.instantiateStreaming(module, imports);
      } catch (e) {
        const validResponse = module.ok && expectedResponseType(module.type);

        if (
          validResponse &&
          module.headers.get("Content-Type") !== "application/wasm"
        ) {
          console.warn(
            "`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",
            e,
          );
        } else {
          throw e;
        }
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
      case "basic":
      case "cors":
      case "default":
        return true;
    }
    return false;
  }
}

function initSync(module, memory) {
  if (wasm !== undefined) return wasm;

  let thread_stack_size;
  if (module !== undefined) {
    if (Object.getPrototypeOf(module) === Object.prototype) {
      ({ module, memory, thread_stack_size } = module);
    } else {
      console.warn(
        "using deprecated parameters for `initSync()`; pass a single object instead",
      );
    }
  }

  const imports = __wbg_get_imports(memory);
  if (!(module instanceof WebAssembly.Module)) {
    module = new WebAssembly.Module(module);
  }
  const instance = new WebAssembly.Instance(module, imports);
  return __wbg_finalize_init(instance, module, thread_stack_size);
}

async function __wbg_init(module_or_path, memory) {
  if (wasm !== undefined) return wasm;

  let thread_stack_size;
  if (module_or_path !== undefined) {
    if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
      ({ module_or_path, memory, thread_stack_size } = module_or_path);
    } else {
      console.warn(
        "using deprecated parameters for the initialization function; pass a single object instead",
      );
    }
  }

  //   if (module_or_path === undefined) {
  //     module_or_path = new URL('tfhe_bg.wasm', import.meta.url);
  //   }
  const imports = __wbg_get_imports(memory);

  //   if (
  //     typeof module_or_path === 'string' ||
  //     (typeof Request === 'function' && module_or_path instanceof Request) ||
  //     (typeof URL === 'function' && module_or_path instanceof URL)
  //   ) {
  //     module_or_path = fetch(module_or_path);
  //   }

  const { instance, module } = await __wbg_load(await module_or_path, imports);

  return __wbg_finalize_init(instance, module, thread_stack_size);
}

//export { initSync, __wbg_init as default };

function getWasmInfo() {
  return {
    name: "tfhe",
    version: "1.5.3"
  }
}

export { initSync, getTfheWorkers, terminateWorkers, setWorkerUrlConfig, getWasmInfo };
export default __wbg_init;
