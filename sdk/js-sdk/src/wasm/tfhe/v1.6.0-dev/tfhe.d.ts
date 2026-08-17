/* tslint:disable */
/* eslint-disable */

export class CompactCiphertextList {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    static builder(public_key: TfheCompactPublicKey): CompactCiphertextListBuilder;
    eq(other: CompactCiphertextList): boolean;
}

export class CompactCiphertextListBuilder {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    build(): CompactCiphertextList;
    build_packed(): CompactCiphertextList;
    build_packed_seeded(seed: Uint8Array): CompactCiphertextList;
    build_with_proof_packed(crs: CompactPkeCrs, metadata: Uint8Array, compute_load: ZkComputeLoad): ProvenCompactCiphertextList;
    build_with_proof_packed_seeded(crs: CompactPkeCrs, metadata: Uint8Array, compute_load: ZkComputeLoad, seed: Uint8Array): ProvenCompactCiphertextList;
    push_boolean(value: boolean): void;
    push_i10(value: number): void;
    push_i1024(value: any): void;
    push_i12(value: number): void;
    push_i128(value: any): void;
    push_i14(value: number): void;
    push_i16(value: number): void;
    push_i160(value: any): void;
    push_i2(value: number): void;
    push_i2048(value: any): void;
    push_i256(value: any): void;
    push_i32(value: number): void;
    push_i4(value: number): void;
    push_i512(value: any): void;
    push_i6(value: number): void;
    push_i64(value: bigint): void;
    push_i8(value: number): void;
    push_u10(value: number): void;
    push_u1024(value: any): void;
    push_u12(value: number): void;
    push_u128(value: any): void;
    push_u14(value: number): void;
    push_u16(value: number): void;
    push_u160(value: any): void;
    push_u2(value: number): void;
    push_u2048(value: any): void;
    push_u256(value: any): void;
    push_u32(value: number): void;
    push_u4(value: number): void;
    push_u512(value: any): void;
    push_u6(value: number): void;
    push_u64(value: bigint): void;
    push_u8(value: number): void;
}

export class CompactCiphertextListExpander {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

export class CompactPkeCrs {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    static from_config(config: TfheConfig, max_num_bits: number): CompactPkeCrs;
    static safe_deserialize(buffer: Uint8Array, serialized_size_limit: bigint): CompactPkeCrs;
    static safe_deserialize_from_public_params(buffer: Uint8Array, serialized_size_limit: bigint): CompactPkeCrs;
    safe_serialize(serialized_size_limit: bigint): Uint8Array;
}

export enum FheTypes {
    Bool = 0,
    Uint4 = 1,
    Uint8 = 2,
    Uint16 = 3,
    Uint32 = 4,
    Uint64 = 5,
    Uint128 = 6,
    Uint160 = 7,
    Uint256 = 8,
    Uint512 = 9,
    Uint1024 = 10,
    Uint2048 = 11,
    Uint2 = 12,
    Uint6 = 13,
    Uint10 = 14,
    Uint12 = 15,
    Uint14 = 16,
    Int2 = 17,
    Int4 = 18,
    Int6 = 19,
    Int8 = 20,
    Int10 = 21,
    Int12 = 22,
    Int14 = 23,
    Int16 = 24,
    Int32 = 25,
    Int64 = 26,
    Int128 = 27,
    Int160 = 28,
    Int256 = 29,
    AsciiString = 30,
    Int512 = 31,
    Int1024 = 32,
    Int2048 = 33,
    Uint24 = 34,
    Uint40 = 35,
    Uint48 = 36,
    Uint56 = 37,
    Uint72 = 38,
    Uint80 = 39,
    Uint88 = 40,
    Uint96 = 41,
    Uint104 = 42,
    Uint112 = 43,
    Uint120 = 44,
    Uint136 = 45,
    Uint144 = 46,
    Uint152 = 47,
    Uint168 = 48,
    Uint176 = 49,
    Uint184 = 50,
    Uint192 = 51,
    Uint200 = 52,
    Uint208 = 53,
    Uint216 = 54,
    Uint224 = 55,
    Uint232 = 56,
    Uint240 = 57,
    Uint248 = 58,
    Int24 = 59,
    Int40 = 60,
    Int48 = 61,
    Int56 = 62,
    Int72 = 63,
    Int80 = 64,
    Int88 = 65,
    Int96 = 66,
    Int104 = 67,
    Int112 = 68,
    Int120 = 69,
    Int136 = 70,
    Int144 = 71,
    Int152 = 72,
    Int168 = 73,
    Int176 = 74,
    Int184 = 75,
    Int192 = 76,
    Int200 = 77,
    Int208 = 78,
    Int216 = 79,
    Int224 = 80,
    Int232 = 81,
    Int240 = 82,
    Int248 = 83,
}

export class ProvenCompactCiphertextList {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    static builder(public_key: TfheCompactPublicKey): CompactCiphertextListBuilder;
    eq(other: ProvenCompactCiphertextList): boolean;
    get_kind_of(index: number): FheTypes | undefined;
    is_empty(): boolean;
    len(): number;
    static safe_deserialize(buffer: Uint8Array, serialized_size_limit: bigint): ProvenCompactCiphertextList;
    safe_serialize(serialized_size_limit: bigint): Uint8Array;
}

export class TfheClientKey {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    static generate(config: TfheConfig): TfheClientKey;
    static generate_with_seed(config: TfheConfig, seed: any): TfheClientKey;
    static safe_deserialize(buffer: Uint8Array, serialized_size_limit: bigint): TfheClientKey;
    safe_serialize(serialized_size_limit: bigint): Uint8Array;
}

export class TfheCompactPublicKey {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    static new(client_key: TfheClientKey): TfheCompactPublicKey;
    static safe_deserialize(buffer: Uint8Array, serialized_size_limit: bigint): TfheCompactPublicKey;
    safe_serialize(serialized_size_limit: bigint): Uint8Array;
}

export class TfheConfig {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    static default(): TfheConfig;
}

export enum ZkComputeLoad {
    Proof = 0,
    Verify = 1,
}

export function initThreadPool(num_threads: number): Promise<any>;

export function init_panic_hook(): void;

export class wbg_rayon_PoolBuilder {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    build(): void;
    numThreads(): number;
    receiver(): number;
}

export function wbg_rayon_start_worker(receiver: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly __wbg_compactciphertextlist_free: (a: number, b: number) => void;
    readonly __wbg_compactciphertextlistbuilder_free: (a: number, b: number) => void;
    readonly __wbg_compactciphertextlistexpander_free: (a: number, b: number) => void;
    readonly __wbg_compactpkecrs_free: (a: number, b: number) => void;
    readonly __wbg_provencompactciphertextlist_free: (a: number, b: number) => void;
    readonly __wbg_tfheclientkey_free: (a: number, b: number) => void;
    readonly __wbg_tfhecompactpublickey_free: (a: number, b: number) => void;
    readonly __wbg_tfheconfig_free: (a: number, b: number) => void;
    readonly compactciphertextlist_builder: (a: number) => [number, number, number];
    readonly compactciphertextlist_eq: (a: number, b: number) => number;
    readonly compactciphertextlistbuilder_build: (a: number) => [number, number, number];
    readonly compactciphertextlistbuilder_build_packed: (a: number) => [number, number, number];
    readonly compactciphertextlistbuilder_build_packed_seeded: (a: number, b: number, c: number) => [number, number, number];
    readonly compactciphertextlistbuilder_build_with_proof_packed: (a: number, b: number, c: number, d: number, e: number) => [number, number, number];
    readonly compactciphertextlistbuilder_build_with_proof_packed_seeded: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number];
    readonly compactciphertextlistbuilder_push_boolean: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_i10: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_i1024: (a: number, b: any) => [number, number];
    readonly compactciphertextlistbuilder_push_i12: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_i128: (a: number, b: any) => [number, number];
    readonly compactciphertextlistbuilder_push_i14: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_i16: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_i160: (a: number, b: any) => [number, number];
    readonly compactciphertextlistbuilder_push_i2: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_i2048: (a: number, b: any) => [number, number];
    readonly compactciphertextlistbuilder_push_i256: (a: number, b: any) => [number, number];
    readonly compactciphertextlistbuilder_push_i32: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_i4: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_i512: (a: number, b: any) => [number, number];
    readonly compactciphertextlistbuilder_push_i6: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_i64: (a: number, b: bigint) => [number, number];
    readonly compactciphertextlistbuilder_push_i8: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_u10: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_u1024: (a: number, b: any) => [number, number];
    readonly compactciphertextlistbuilder_push_u12: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_u128: (a: number, b: any) => [number, number];
    readonly compactciphertextlistbuilder_push_u14: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_u16: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_u160: (a: number, b: any) => [number, number];
    readonly compactciphertextlistbuilder_push_u2: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_u2048: (a: number, b: any) => [number, number];
    readonly compactciphertextlistbuilder_push_u256: (a: number, b: any) => [number, number];
    readonly compactciphertextlistbuilder_push_u32: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_u4: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_u512: (a: number, b: any) => [number, number];
    readonly compactciphertextlistbuilder_push_u6: (a: number, b: number) => [number, number];
    readonly compactciphertextlistbuilder_push_u64: (a: number, b: bigint) => [number, number];
    readonly compactciphertextlistbuilder_push_u8: (a: number, b: number) => [number, number];
    readonly compactpkecrs_from_config: (a: number, b: number) => [number, number, number];
    readonly compactpkecrs_safe_deserialize: (a: number, b: number, c: bigint) => [number, number, number];
    readonly compactpkecrs_safe_deserialize_from_public_params: (a: number, b: number, c: bigint) => [number, number, number];
    readonly compactpkecrs_safe_serialize: (a: number, b: bigint) => [number, number, number, number];
    readonly init_panic_hook: () => void;
    readonly provencompactciphertextlist_builder: (a: number) => [number, number, number];
    readonly provencompactciphertextlist_eq: (a: number, b: number) => number;
    readonly provencompactciphertextlist_get_kind_of: (a: number, b: number) => number;
    readonly provencompactciphertextlist_is_empty: (a: number) => number;
    readonly provencompactciphertextlist_len: (a: number) => number;
    readonly provencompactciphertextlist_safe_deserialize: (a: number, b: number, c: bigint) => [number, number, number];
    readonly provencompactciphertextlist_safe_serialize: (a: number, b: bigint) => [number, number, number, number];
    readonly tfheclientkey_generate: (a: number) => [number, number, number];
    readonly tfheclientkey_generate_with_seed: (a: number, b: any) => [number, number, number];
    readonly tfheclientkey_safe_deserialize: (a: number, b: number, c: bigint) => [number, number, number];
    readonly tfheclientkey_safe_serialize: (a: number, b: bigint) => [number, number, number, number];
    readonly tfhecompactpublickey_new: (a: number) => [number, number, number];
    readonly tfhecompactpublickey_safe_deserialize: (a: number, b: number, c: bigint) => [number, number, number];
    readonly tfhecompactpublickey_safe_serialize: (a: number, b: bigint) => [number, number, number, number];
    readonly tfheconfig_default: () => number;
    readonly __wbg_wbg_rayon_poolbuilder_free: (a: number, b: number) => void;
    readonly initThreadPool: (a: number) => any;
    readonly wbg_rayon_poolbuilder_build: (a: number) => void;
    readonly wbg_rayon_poolbuilder_numThreads: (a: number) => number;
    readonly wbg_rayon_poolbuilder_receiver: (a: number) => number;
    readonly wbg_rayon_start_worker: (a: number) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly memory: WebAssembly.Memory;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_thread_destroy: (a?: number, b?: number, c?: number) => void;
    readonly __wbindgen_start: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput, memory?: WebAssembly.Memory, thread_stack_size?: number }} module - Passing `SyncInitInput` directly is deprecated.
 * @param {WebAssembly.Memory} memory - Deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput, memory?: WebAssembly.Memory, thread_stack_size?: number } | SyncInitInput, memory?: WebAssembly.Memory): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory, thread_stack_size?: number }} module_or_path - Passing `InitInput` directly is deprecated.
 * @param {WebAssembly.Memory} memory - Deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory, thread_stack_size?: number } | InitInput | Promise<InitInput>, memory?: WebAssembly.Memory): Promise<InitOutput>;

////////////////////////////////////////////////////////////////////////////////

export function initAsync(...args: Parameters<typeof __wbg_init>): ReturnType<typeof __wbg_init>;

export function getWasmInfo(): {
  name: string;
  version: string;
  downloadFiles: readonly {
    filename: string;
    sha256: string;
  }[];
  memory?: {
    byteLength: number;
    pages: number;
  };
};
export function getTfheWorkers(): object[];
export function terminateWorkers(): Promise<unknown>;
export type WasmAssetLoadMode =
  | 'embedded-base64'
  | 'verified-blob'
  | 'precheck-direct-url'
  | 'trusted-direct-url'
  | 'auto';
export function setWorkerUrlConfig(parameters?: {
  readonly workerUrl?: URL | undefined;
  readonly wasmAssetLoadMode?: WasmAssetLoadMode | undefined;
  // Required: the SDK injects the resolved runtime kind (browser vs Node); the
  // worker bootstrap no longer detects it itself.
  readonly isBrowserLike: boolean;
  readonly logger?:
    | {
        debug: (message: string) => void;
        error: (message: string, cause: unknown) => void;
      }
    | undefined;
}): void;
