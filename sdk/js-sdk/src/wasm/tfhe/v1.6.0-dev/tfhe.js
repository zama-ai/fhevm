/* @ts-self-types="./tfhe.d.ts" */
import { startWorkers, getTfheWorkers, terminateWorkers, setWorkerUrlConfig } from './startWorkers.js';

export class CompactCiphertextList {
    static __wrap(ptr) {
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
     * @param {CompactCiphertextList} other
     * @returns {boolean}
     */
    eq(other) {
        _assertClass(other, CompactCiphertextList);
        const ret = wasm.compactciphertextlist_eq(this.__wbg_ptr, other.__wbg_ptr);
        return ret !== 0;
    }
}
if (Symbol.dispose) CompactCiphertextList.prototype[Symbol.dispose] = CompactCiphertextList.prototype.free;

export class CompactCiphertextListBuilder {
    static __wrap(ptr) {
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
     * @param {Uint8Array} seed
     * @returns {CompactCiphertextList}
     */
    build_packed_seeded(seed) {
        const ptr0 = passArray8ToWasm0(seed, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.compactciphertextlistbuilder_build_packed_seeded(this.__wbg_ptr, ptr0, len0);
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
        const ret = wasm.compactciphertextlistbuilder_build_with_proof_packed(this.__wbg_ptr, crs.__wbg_ptr, ptr0, len0, compute_load);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ProvenCompactCiphertextList.__wrap(ret[0]);
    }
    /**
     * @param {CompactPkeCrs} crs
     * @param {Uint8Array} metadata
     * @param {ZkComputeLoad} compute_load
     * @param {Uint8Array} seed
     * @returns {ProvenCompactCiphertextList}
     */
    build_with_proof_packed_seeded(crs, metadata, compute_load, seed) {
        _assertClass(crs, CompactPkeCrs);
        const ptr0 = passArray8ToWasm0(metadata, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(seed, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.compactciphertextlistbuilder_build_with_proof_packed_seeded(this.__wbg_ptr, crs.__wbg_ptr, ptr0, len0, compute_load, ptr1, len1);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ProvenCompactCiphertextList.__wrap(ret[0]);
    }
    /**
     * @param {boolean} value
     */
    push_boolean(value) {
        const ret = wasm.compactciphertextlistbuilder_push_boolean(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_i10(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i10(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} value
     */
    push_i1024(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i1024(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_i12(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i12(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} value
     */
    push_i128(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i128(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_i14(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i14(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_i16(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i16(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} value
     */
    push_i160(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i160(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_i2(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i2(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} value
     */
    push_i2048(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i2048(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} value
     */
    push_i256(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i256(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_i32(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i32(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_i4(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i4(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} value
     */
    push_i512(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i512(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_i6(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i6(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {bigint} value
     */
    push_i64(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i64(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_i8(value) {
        const ret = wasm.compactciphertextlistbuilder_push_i8(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_u10(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u10(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} value
     */
    push_u1024(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u1024(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_u12(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u12(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} value
     */
    push_u128(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u128(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_u14(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u14(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_u16(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u16(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} value
     */
    push_u160(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u160(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_u2(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u2(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} value
     */
    push_u2048(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u2048(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} value
     */
    push_u256(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u256(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_u32(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u32(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_u4(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u4(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} value
     */
    push_u512(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u512(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_u6(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u6(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {bigint} value
     */
    push_u64(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u64(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} value
     */
    push_u8(value) {
        const ret = wasm.compactciphertextlistbuilder_push_u8(this.__wbg_ptr, value);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) CompactCiphertextListBuilder.prototype[Symbol.dispose] = CompactCiphertextListBuilder.prototype.free;

export class CompactCiphertextListExpander {
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
}
if (Symbol.dispose) CompactCiphertextListExpander.prototype[Symbol.dispose] = CompactCiphertextListExpander.prototype.free;

export class CompactPkeCrs {
    static __wrap(ptr) {
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
        const ret = wasm.compactpkecrs_safe_deserialize(ptr0, len0, serialized_size_limit);
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
        const ret = wasm.compactpkecrs_safe_deserialize_from_public_params(ptr0, len0, serialized_size_limit);
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
        const ret = wasm.compactpkecrs_safe_serialize(this.__wbg_ptr, serialized_size_limit);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
}
if (Symbol.dispose) CompactPkeCrs.prototype[Symbol.dispose] = CompactPkeCrs.prototype.free;

/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83}
 */
export const FheTypes = Object.freeze({
    Bool: 0, "0": "Bool",
    Uint4: 1, "1": "Uint4",
    Uint8: 2, "2": "Uint8",
    Uint16: 3, "3": "Uint16",
    Uint32: 4, "4": "Uint32",
    Uint64: 5, "5": "Uint64",
    Uint128: 6, "6": "Uint128",
    Uint160: 7, "7": "Uint160",
    Uint256: 8, "8": "Uint256",
    Uint512: 9, "9": "Uint512",
    Uint1024: 10, "10": "Uint1024",
    Uint2048: 11, "11": "Uint2048",
    Uint2: 12, "12": "Uint2",
    Uint6: 13, "13": "Uint6",
    Uint10: 14, "14": "Uint10",
    Uint12: 15, "15": "Uint12",
    Uint14: 16, "16": "Uint14",
    Int2: 17, "17": "Int2",
    Int4: 18, "18": "Int4",
    Int6: 19, "19": "Int6",
    Int8: 20, "20": "Int8",
    Int10: 21, "21": "Int10",
    Int12: 22, "22": "Int12",
    Int14: 23, "23": "Int14",
    Int16: 24, "24": "Int16",
    Int32: 25, "25": "Int32",
    Int64: 26, "26": "Int64",
    Int128: 27, "27": "Int128",
    Int160: 28, "28": "Int160",
    Int256: 29, "29": "Int256",
    AsciiString: 30, "30": "AsciiString",
    Int512: 31, "31": "Int512",
    Int1024: 32, "32": "Int1024",
    Int2048: 33, "33": "Int2048",
    Uint24: 34, "34": "Uint24",
    Uint40: 35, "35": "Uint40",
    Uint48: 36, "36": "Uint48",
    Uint56: 37, "37": "Uint56",
    Uint72: 38, "38": "Uint72",
    Uint80: 39, "39": "Uint80",
    Uint88: 40, "40": "Uint88",
    Uint96: 41, "41": "Uint96",
    Uint104: 42, "42": "Uint104",
    Uint112: 43, "43": "Uint112",
    Uint120: 44, "44": "Uint120",
    Uint136: 45, "45": "Uint136",
    Uint144: 46, "46": "Uint144",
    Uint152: 47, "47": "Uint152",
    Uint168: 48, "48": "Uint168",
    Uint176: 49, "49": "Uint176",
    Uint184: 50, "50": "Uint184",
    Uint192: 51, "51": "Uint192",
    Uint200: 52, "52": "Uint200",
    Uint208: 53, "53": "Uint208",
    Uint216: 54, "54": "Uint216",
    Uint224: 55, "55": "Uint224",
    Uint232: 56, "56": "Uint232",
    Uint240: 57, "57": "Uint240",
    Uint248: 58, "58": "Uint248",
    Int24: 59, "59": "Int24",
    Int40: 60, "60": "Int40",
    Int48: 61, "61": "Int48",
    Int56: 62, "62": "Int56",
    Int72: 63, "63": "Int72",
    Int80: 64, "64": "Int80",
    Int88: 65, "65": "Int88",
    Int96: 66, "66": "Int96",
    Int104: 67, "67": "Int104",
    Int112: 68, "68": "Int112",
    Int120: 69, "69": "Int120",
    Int136: 70, "70": "Int136",
    Int144: 71, "71": "Int144",
    Int152: 72, "72": "Int152",
    Int168: 73, "73": "Int168",
    Int176: 74, "74": "Int176",
    Int184: 75, "75": "Int184",
    Int192: 76, "76": "Int192",
    Int200: 77, "77": "Int200",
    Int208: 78, "78": "Int208",
    Int216: 79, "79": "Int216",
    Int224: 80, "80": "Int224",
    Int232: 81, "81": "Int232",
    Int240: 82, "82": "Int240",
    Int248: 83, "83": "Int248",
});

export class ProvenCompactCiphertextList {
    static __wrap(ptr) {
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
     * @param {ProvenCompactCiphertextList} other
     * @returns {boolean}
     */
    eq(other) {
        _assertClass(other, ProvenCompactCiphertextList);
        const ret = wasm.provencompactciphertextlist_eq(this.__wbg_ptr, other.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {number} index
     * @returns {FheTypes | undefined}
     */
    get_kind_of(index) {
        const ret = wasm.provencompactciphertextlist_get_kind_of(this.__wbg_ptr, index);
        return ret === 84 ? undefined : ret;
    }
    /**
     * @returns {boolean}
     */
    is_empty() {
        const ret = wasm.provencompactciphertextlist_is_empty(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {number}
     */
    len() {
        const ret = wasm.provencompactciphertextlist_len(this.__wbg_ptr);
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
        const ret = wasm.provencompactciphertextlist_safe_deserialize(ptr0, len0, serialized_size_limit);
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
        const ret = wasm.provencompactciphertextlist_safe_serialize(this.__wbg_ptr, serialized_size_limit);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
}
if (Symbol.dispose) ProvenCompactCiphertextList.prototype[Symbol.dispose] = ProvenCompactCiphertextList.prototype.free;

export class TfheClientKey {
    static __wrap(ptr) {
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
        const ret = wasm.tfheclientkey_safe_deserialize(ptr0, len0, serialized_size_limit);
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
        const ret = wasm.tfheclientkey_safe_serialize(this.__wbg_ptr, serialized_size_limit);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
}
if (Symbol.dispose) TfheClientKey.prototype[Symbol.dispose] = TfheClientKey.prototype.free;

export class TfheCompactPublicKey {
    static __wrap(ptr) {
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
        const ret = wasm.tfhecompactpublickey_safe_deserialize(ptr0, len0, serialized_size_limit);
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
        const ret = wasm.tfhecompactpublickey_safe_serialize(this.__wbg_ptr, serialized_size_limit);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
}
if (Symbol.dispose) TfheCompactPublicKey.prototype[Symbol.dispose] = TfheCompactPublicKey.prototype.free;

export class TfheConfig {
    static __wrap(ptr) {
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
    /**
     * @returns {TfheConfig}
     */
    static default() {
        const ret = wasm.tfheconfig_default();
        return TfheConfig.__wrap(ret);
    }
}
if (Symbol.dispose) TfheConfig.prototype[Symbol.dispose] = TfheConfig.prototype.free;

/**
 * @enum {0 | 1}
 */
export const ZkComputeLoad = Object.freeze({
    Proof: 0, "0": "Proof",
    Verify: 1, "1": "Verify",
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

export class wbg_rayon_PoolBuilder {
    static __wrap(ptr) {
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
if (Symbol.dispose) wbg_rayon_PoolBuilder.prototype[Symbol.dispose] = wbg_rayon_PoolBuilder.prototype.free;

/**
 * @param {number} receiver
 */
export function wbg_rayon_start_worker(receiver) {
    wasm.wbg_rayon_start_worker(receiver);
}
function __wbg_get_imports(memory) {
    const import0 = {
        __proto__: null,
        __wbg_BigInt_52ff8391297da194: function() { return handleError(function (arg0) {
            const ret = BigInt(arg0);
            return ret;
        }, arguments); },
        __wbg_BigInt_ae200e93cacbd2b3: function(arg0) {
            const ret = BigInt(arg0);
            return ret;
        },
        __wbg_Error_3639a60ed15f87e7: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg___wbindgen_bigint_get_as_i64_3af6d4ca77193a4b: function(arg0, arg1) {
            const v = arg1;
            const ret = typeof(v) === 'bigint' ? v : undefined;
            getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_bit_and_bfedece2bb657f4b: function(arg0, arg1) {
            const ret = arg0 & arg1;
            return ret;
        },
        __wbg___wbindgen_debug_string_07cb72cfcc952e2b: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_is_function_2f0fd7ceb86e64c5: function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_object_5b22ff2418063a9c: function(arg0) {
            const val = arg0;
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        },
        __wbg___wbindgen_is_string_eddc07a3efad52e6: function(arg0) {
            const ret = typeof(arg0) === 'string';
            return ret;
        },
        __wbg___wbindgen_is_undefined_244a92c34d3b6ec0: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_jsval_eq_403eaa3610500a25: function(arg0, arg1) {
            const ret = arg0 === arg1;
            return ret;
        },
        __wbg___wbindgen_lt_c483cc694de67c3e: function(arg0, arg1) {
            const ret = arg0 < arg1;
            return ret;
        },
        __wbg___wbindgen_memory_c2356dd1a089dfbd: function() {
            const ret = wasm.memory;
            return ret;
        },
        __wbg___wbindgen_module_df704393dfd1853c: function() {
            const ret = wasmModule;
            return ret;
        },
        __wbg___wbindgen_neg_9b4d71823e3bc513: function(arg0) {
            const ret = -arg0;
            return ret;
        },
        __wbg___wbindgen_shr_d8f8268f18c7a1c3: function(arg0, arg1) {
            const ret = arg0 >> arg1;
            return ret;
        },
        __wbg___wbindgen_string_get_965592073e5d848c: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_9c75d47bf9e7731e: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_call_a41d6421b30a32c5: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_crypto_38df2bab126b63dc: function(arg0) {
            const ret = arg0.crypto;
            return ret;
        },
        __wbg_error_a6fa202b58aa1cd3: function(arg0, arg1) {
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
        __wbg_getRandomValues_c44a50d8cfdaebeb: function() { return handleError(function (arg0, arg1) {
            arg0.getRandomValues(arg1);
        }, arguments); },
        __wbg_instanceof_Window_4153c1818a1c0c0b: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_length_ba3c032602efe310: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_msCrypto_bd5a034af96bcba6: function(arg0) {
            const ret = arg0.msCrypto;
            return ret;
        },
        __wbg_new_227d7c05414eb861: function() {
            const ret = new Error();
            return ret;
        },
        __wbg_new_with_length_9011f5da794bf5d9: function(arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return ret;
        },
        __wbg_node_84ea875411254db1: function(arg0) {
            const ret = arg0.node;
            return ret;
        },
        __wbg_process_44c7a14e11e9f69e: function(arg0) {
            const ret = arg0.process;
            return ret;
        },
        __wbg_prototypesetcall_fd4050e806e1d519: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
        },
        __wbg_randomFillSync_6c25eac9869eb53c: function() { return handleError(function (arg0, arg1) {
            arg0.randomFillSync(arg1);
        }, arguments); },
        __wbg_require_b4edbdcf3e2a1ef0: function() { return handleError(function () {
            const ret = module.require;
            return ret;
        }, arguments); },
        __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_startWorkers_8b582d57e92bd2d4: function(arg0, arg1, arg2) {
            const ret = startWorkers(arg0, arg1, wbg_rayon_PoolBuilder.__wrap(arg2));
            return ret;
        },
        __wbg_static_accessor_GLOBAL_THIS_1c7f1bd6c6941fdb: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_GLOBAL_e039bc914f83e74e: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_SELF_8bf8c48c28420ad5: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_WINDOW_6aeee9b51652ee0f: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_subarray_fbe3cef290e1fa43: function(arg0, arg1, arg2) {
            const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
            return ret;
        },
        __wbg_toString_15656af8d8e71f16: function(arg0, arg1, arg2) {
            const ret = arg1.toString(arg2);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_toString_8d874489bad7e5a2: function(arg0) {
            const ret = arg0.toString();
            return ret;
        },
        __wbg_versions_276b2795b1c6a219: function(arg0) {
            const ret = arg0.versions;
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_0000000000000002: function(arg0) {
            // Cast intrinsic for `I64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_0000000000000003: function(arg0, arg1) {
            // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
            const ret = getArrayU8FromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_cast_0000000000000004: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_cast_0000000000000005: function(arg0) {
            // Cast intrinsic for `U64 -> Externref`.
            const ret = BigInt.asUintN(64, arg0);
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
        memory: memory || new WebAssembly.Memory({initial:19,maximum:16384,shared:true}),
    };
    return {
        __proto__: null,
        "./tfhe_bg.js": import0,
    };
}

const CompactCiphertextListFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_compactciphertextlist_free(ptr, 1));
const CompactCiphertextListBuilderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_compactciphertextlistbuilder_free(ptr, 1));
const CompactCiphertextListExpanderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_compactciphertextlistexpander_free(ptr, 1));
const CompactPkeCrsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_compactpkecrs_free(ptr, 1));
const ProvenCompactCiphertextListFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_provencompactciphertextlist_free(ptr, 1));
const TfheClientKeyFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tfheclientkey_free(ptr, 1));
const TfheCompactPublicKeyFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tfhecompactpublickey_free(ptr, 1));
const TfheConfigFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tfheconfig_free(ptr, 1));
const wbg_rayon_PoolBuilderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wbg_rayon_poolbuilder_free(ptr, 1));

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

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer !== wasm.memory.buffer) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.buffer !== wasm.memory.buffer) {
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

let cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : undefined);
if (cachedTextDecoder) cachedTextDecoder.decode();

const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().slice(ptr, ptr + len));
}

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder() : undefined);

if (cachedTextEncoder) {
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

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module, thread_stack_size) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    if (typeof thread_stack_size !== 'undefined' && (typeof thread_stack_size !== 'number' || thread_stack_size === 0 || thread_stack_size % 65536 !== 0)) {
        throw new Error('invalid stack size');
    }

    wasm.__wbindgen_start(thread_stack_size);
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

function initSync(module, memory) {
    if (wasm !== undefined) return wasm;

    let thread_stack_size
    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module, memory, thread_stack_size} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
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

    let thread_stack_size
    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path, memory, thread_stack_size} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    // if (module_or_path === undefined) {
    //     module_or_path = new URL('tfhe_bg.wasm', import.meta.url);
    // }
    const imports = __wbg_get_imports(memory);

    // if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
    //     module_or_path = fetch(module_or_path);
    // }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module, thread_stack_size);
}

function getWasmInfo() {
  const memory = wasm?.memory;
  return {
    name: 'tfhe',
    version: "1.6.0-dev",
    downloadFiles: [
      {
        filename: "tfhe_bg.wasm",
        sha256: "c75b046a99f7e96fd4a14ab44162a51c363685a2c6f6af4249ee0ed5467a4a37",
      },
      {
        filename: "tfhe-worker.mjs",
        sha256: "2c2ef02437e5b7a41f4c5c1defa0a24dcfae737037a59a145a972bb4191571b8",
      }
    ],
    memory:
      memory === undefined
        ? undefined
        : {
            byteLength: memory.buffer.byteLength,
            pages: memory.buffer.byteLength / 65536,
          },
  };
}

export { initSync, getTfheWorkers, terminateWorkers, setWorkerUrlConfig, getWasmInfo, __wbg_init as initAsync };
export default __wbg_init;
