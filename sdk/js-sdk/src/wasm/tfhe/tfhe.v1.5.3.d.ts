/* tslint:disable */
/* eslint-disable */

export class Boolean {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static decompress_ciphertext(
    compressed_ciphertext: BooleanCompressedCiphertext,
  ): BooleanCiphertext;
  static decrypt(client_key: BooleanClientKey, ct: BooleanCiphertext): boolean;
  static deserialize_ciphertext(buffer: Uint8Array): BooleanCiphertext;
  static deserialize_client_key(buffer: Uint8Array): BooleanClientKey;
  static deserialize_compressed_ciphertext(
    buffer: Uint8Array,
  ): BooleanCompressedCiphertext;
  static deserialize_compressed_server_key(
    buffer: Uint8Array,
  ): BooleanCompressedServerKey;
  static deserialize_public_key(buffer: Uint8Array): BooleanPublicKey;
  static encrypt(
    client_key: BooleanClientKey,
    message: boolean,
  ): BooleanCiphertext;
  static encrypt_compressed(
    client_key: BooleanClientKey,
    message: boolean,
  ): BooleanCompressedCiphertext;
  static encrypt_with_public_key(
    public_key: BooleanPublicKey,
    message: boolean,
  ): BooleanCiphertext;
  static get_parameters(parameter_choice: number): BooleanParameters;
  static new_client_key(parameters: BooleanParameters): BooleanClientKey;
  static new_client_key_from_seed_and_parameters(
    seed_high_bytes: bigint,
    seed_low_bytes: bigint,
    parameters: BooleanParameters,
  ): BooleanClientKey;
  static new_compressed_server_key(
    client_key: BooleanClientKey,
  ): BooleanCompressedServerKey;
  static new_gaussian_from_std_dev(std_dev: number): BooleanNoiseDistribution;
  static new_parameters(
    lwe_dimension: number,
    glwe_dimension: number,
    polynomial_size: number,
    lwe_noise_distribution: BooleanNoiseDistribution,
    glwe_noise_distribution: BooleanNoiseDistribution,
    pbs_base_log: number,
    pbs_level: number,
    ks_base_log: number,
    ks_level: number,
    encryption_key_choice: BooleanEncryptionKeyChoice,
  ): BooleanParameters;
  static new_public_key(client_key: BooleanClientKey): BooleanPublicKey;
  static serialize_ciphertext(ciphertext: BooleanCiphertext): Uint8Array;
  static serialize_client_key(client_key: BooleanClientKey): Uint8Array;
  static serialize_compressed_ciphertext(
    ciphertext: BooleanCompressedCiphertext,
  ): Uint8Array;
  static serialize_compressed_server_key(
    server_key: BooleanCompressedServerKey,
  ): Uint8Array;
  static serialize_public_key(public_key: BooleanPublicKey): Uint8Array;
  static trivial_encrypt(message: boolean): BooleanCiphertext;
  static try_new_t_uniform(bound_log2: number): BooleanNoiseDistribution;
}

export class BooleanCiphertext {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export class BooleanClientKey {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export class BooleanCompressedCiphertext {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export class BooleanCompressedServerKey {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export enum BooleanEncryptionKeyChoice {
  Big = 0,
  Small = 1,
}

export class BooleanNoiseDistribution {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export enum BooleanParameterSet {
  Default = 0,
  TfheLib = 1,
  DefaultKsPbs = 2,
  TfheLibKsPbs = 3,
}

export class BooleanParameters {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export class BooleanPublicKey {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export class CompactCiphertextList {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static builder(
    public_key: TfheCompactPublicKey,
  ): CompactCiphertextListBuilder;
  static deserialize(buffer: Uint8Array): CompactCiphertextList;
  expand(): CompactCiphertextListExpander;
  get_kind_of(index: number): FheTypes | undefined;
  is_empty(): boolean;
  len(): number;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompactCiphertextList;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompactCiphertextListBuilder {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  build(): CompactCiphertextList;
  build_packed(): CompactCiphertextList;
  build_with_proof_packed(
    crs: CompactPkeCrs,
    metadata: Uint8Array,
    compute_load: ZkComputeLoad,
  ): ProvenCompactCiphertextList;
  push_boolean(value: boolean): void;
  push_i10(value: number): void;
  push_i1024(value: any): void;
  push_i104(value: any): void;
  push_i112(value: any): void;
  push_i12(value: number): void;
  push_i120(value: any): void;
  push_i128(value: any): void;
  push_i136(value: any): void;
  push_i14(value: number): void;
  push_i144(value: any): void;
  push_i152(value: any): void;
  push_i16(value: number): void;
  push_i160(value: any): void;
  push_i168(value: any): void;
  push_i176(value: any): void;
  push_i184(value: any): void;
  push_i192(value: any): void;
  push_i2(value: number): void;
  push_i200(value: any): void;
  push_i2048(value: any): void;
  push_i208(value: any): void;
  push_i216(value: any): void;
  push_i224(value: any): void;
  push_i232(value: any): void;
  push_i24(value: number): void;
  push_i240(value: any): void;
  push_i248(value: any): void;
  push_i256(value: any): void;
  push_i32(value: number): void;
  push_i4(value: number): void;
  push_i40(value: bigint): void;
  push_i48(value: bigint): void;
  push_i512(value: any): void;
  push_i56(value: bigint): void;
  push_i6(value: number): void;
  push_i64(value: bigint): void;
  push_i72(value: any): void;
  push_i8(value: number): void;
  push_i80(value: any): void;
  push_i88(value: any): void;
  push_i96(value: any): void;
  push_u10(value: number): void;
  push_u1024(value: any): void;
  push_u104(value: any): void;
  push_u112(value: any): void;
  push_u12(value: number): void;
  push_u120(value: any): void;
  push_u128(value: any): void;
  push_u136(value: any): void;
  push_u14(value: number): void;
  push_u144(value: any): void;
  push_u152(value: any): void;
  push_u16(value: number): void;
  push_u160(value: any): void;
  push_u168(value: any): void;
  push_u176(value: any): void;
  push_u184(value: any): void;
  push_u192(value: any): void;
  push_u2(value: number): void;
  push_u200(value: any): void;
  push_u2048(value: any): void;
  push_u208(value: any): void;
  push_u216(value: any): void;
  push_u224(value: any): void;
  push_u232(value: any): void;
  push_u24(value: number): void;
  push_u240(value: any): void;
  push_u248(value: any): void;
  push_u256(value: any): void;
  push_u32(value: number): void;
  push_u4(value: number): void;
  push_u40(value: bigint): void;
  push_u48(value: bigint): void;
  push_u512(value: any): void;
  push_u56(value: bigint): void;
  push_u6(value: number): void;
  push_u64(value: bigint): void;
  push_u72(value: any): void;
  push_u8(value: number): void;
  push_u80(value: any): void;
  push_u88(value: any): void;
  push_u96(value: any): void;
}

export class CompactCiphertextListExpander {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  get_bool(index: number): FheBool;
  get_int10(index: number): FheInt10;
  get_int1024(index: number): FheInt1024;
  get_int104(index: number): FheInt104;
  get_int112(index: number): FheInt112;
  get_int12(index: number): FheInt12;
  get_int120(index: number): FheInt120;
  get_int128(index: number): FheInt128;
  get_int136(index: number): FheInt136;
  get_int14(index: number): FheInt14;
  get_int144(index: number): FheInt144;
  get_int152(index: number): FheInt152;
  get_int16(index: number): FheInt16;
  get_int160(index: number): FheInt160;
  get_int168(index: number): FheInt168;
  get_int176(index: number): FheInt176;
  get_int184(index: number): FheInt184;
  get_int192(index: number): FheInt192;
  get_int2(index: number): FheInt2;
  get_int200(index: number): FheInt200;
  get_int2048(index: number): FheInt2048;
  get_int208(index: number): FheInt208;
  get_int216(index: number): FheInt216;
  get_int224(index: number): FheInt224;
  get_int232(index: number): FheInt232;
  get_int24(index: number): FheInt24;
  get_int240(index: number): FheInt240;
  get_int248(index: number): FheInt248;
  get_int256(index: number): FheInt256;
  get_int32(index: number): FheInt32;
  get_int4(index: number): FheInt4;
  get_int40(index: number): FheInt40;
  get_int48(index: number): FheInt48;
  get_int512(index: number): FheInt512;
  get_int56(index: number): FheInt56;
  get_int6(index: number): FheInt6;
  get_int64(index: number): FheInt64;
  get_int72(index: number): FheInt72;
  get_int8(index: number): FheInt8;
  get_int80(index: number): FheInt80;
  get_int88(index: number): FheInt88;
  get_int96(index: number): FheInt96;
  get_kind_of(index: number): FheTypes | undefined;
  get_uint10(index: number): FheUint10;
  get_uint1024(index: number): FheUint1024;
  get_uint104(index: number): FheUint104;
  get_uint112(index: number): FheUint112;
  get_uint12(index: number): FheUint12;
  get_uint120(index: number): FheUint120;
  get_uint128(index: number): FheUint128;
  get_uint136(index: number): FheUint136;
  get_uint14(index: number): FheUint14;
  get_uint144(index: number): FheUint144;
  get_uint152(index: number): FheUint152;
  get_uint16(index: number): FheUint16;
  get_uint160(index: number): FheUint160;
  get_uint168(index: number): FheUint168;
  get_uint176(index: number): FheUint176;
  get_uint184(index: number): FheUint184;
  get_uint192(index: number): FheUint192;
  get_uint2(index: number): FheUint2;
  get_uint200(index: number): FheUint200;
  get_uint2048(index: number): FheUint2048;
  get_uint208(index: number): FheUint208;
  get_uint216(index: number): FheUint216;
  get_uint224(index: number): FheUint224;
  get_uint232(index: number): FheUint232;
  get_uint24(index: number): FheUint24;
  get_uint240(index: number): FheUint240;
  get_uint248(index: number): FheUint248;
  get_uint256(index: number): FheUint256;
  get_uint32(index: number): FheUint32;
  get_uint4(index: number): FheUint4;
  get_uint40(index: number): FheUint40;
  get_uint48(index: number): FheUint48;
  get_uint512(index: number): FheUint512;
  get_uint56(index: number): FheUint56;
  get_uint6(index: number): FheUint6;
  get_uint64(index: number): FheUint64;
  get_uint72(index: number): FheUint72;
  get_uint8(index: number): FheUint8;
  get_uint80(index: number): FheUint80;
  get_uint88(index: number): FheUint88;
  get_uint96(index: number): FheUint96;
  is_empty(): boolean;
  len(): number;
}

export class CompactPkeCrs {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static deserialize(buffer: Uint8Array): CompactPkeCrs;
  static deserialize_from_public_params(buffer: Uint8Array): CompactPkeCrs;
  static from_config(config: TfheConfig, max_num_bits: number): CompactPkeCrs;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompactPkeCrs;
  static safe_deserialize_from_public_params(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompactPkeCrs;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(compress: boolean): Uint8Array;
}

export class CompressedFheBool {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheBool;
  static deserialize(buffer: Uint8Array): CompressedFheBool;
  static encrypt_with_client_key(
    value: boolean,
    client_key: TfheClientKey,
  ): CompressedFheBool;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheBool;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt10 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt10;
  static deserialize(buffer: Uint8Array): CompressedFheInt10;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheInt10;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt10;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt1024 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt1024;
  static deserialize(buffer: Uint8Array): CompressedFheInt1024;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt1024;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt1024;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt104 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt104;
  static deserialize(buffer: Uint8Array): CompressedFheInt104;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt104;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt104;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt112 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt112;
  static deserialize(buffer: Uint8Array): CompressedFheInt112;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt112;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt112;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt12 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt12;
  static deserialize(buffer: Uint8Array): CompressedFheInt12;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheInt12;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt12;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt120 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt120;
  static deserialize(buffer: Uint8Array): CompressedFheInt120;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt120;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt120;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt128 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt128;
  static deserialize(buffer: Uint8Array): CompressedFheInt128;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt128;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt128;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt136 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt136;
  static deserialize(buffer: Uint8Array): CompressedFheInt136;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt136;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt136;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt14 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt14;
  static deserialize(buffer: Uint8Array): CompressedFheInt14;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheInt14;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt14;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt144 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt144;
  static deserialize(buffer: Uint8Array): CompressedFheInt144;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt144;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt144;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt152 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt152;
  static deserialize(buffer: Uint8Array): CompressedFheInt152;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt152;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt152;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt16 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt16;
  static deserialize(buffer: Uint8Array): CompressedFheInt16;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheInt16;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt16;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt160 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt160;
  static deserialize(buffer: Uint8Array): CompressedFheInt160;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt160;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt160;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt168 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt168;
  static deserialize(buffer: Uint8Array): CompressedFheInt168;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt168;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt168;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt176 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt176;
  static deserialize(buffer: Uint8Array): CompressedFheInt176;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt176;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt176;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt184 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt184;
  static deserialize(buffer: Uint8Array): CompressedFheInt184;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt184;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt184;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt192 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt192;
  static deserialize(buffer: Uint8Array): CompressedFheInt192;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt192;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt192;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt2 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt2;
  static deserialize(buffer: Uint8Array): CompressedFheInt2;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheInt2;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt2;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt200 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt200;
  static deserialize(buffer: Uint8Array): CompressedFheInt200;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt200;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt200;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt2048 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt2048;
  static deserialize(buffer: Uint8Array): CompressedFheInt2048;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt2048;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt2048;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt208 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt208;
  static deserialize(buffer: Uint8Array): CompressedFheInt208;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt208;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt208;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt216 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt216;
  static deserialize(buffer: Uint8Array): CompressedFheInt216;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt216;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt216;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt224 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt224;
  static deserialize(buffer: Uint8Array): CompressedFheInt224;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt224;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt224;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt232 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt232;
  static deserialize(buffer: Uint8Array): CompressedFheInt232;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt232;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt232;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt24 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt24;
  static deserialize(buffer: Uint8Array): CompressedFheInt24;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheInt24;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt24;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt240 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt240;
  static deserialize(buffer: Uint8Array): CompressedFheInt240;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt240;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt240;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt248 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt248;
  static deserialize(buffer: Uint8Array): CompressedFheInt248;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt248;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt248;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt256 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt256;
  static deserialize(buffer: Uint8Array): CompressedFheInt256;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt256;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt256;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt32 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt32;
  static deserialize(buffer: Uint8Array): CompressedFheInt32;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheInt32;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt32;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt4 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt4;
  static deserialize(buffer: Uint8Array): CompressedFheInt4;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheInt4;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt4;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt40 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt40;
  static deserialize(buffer: Uint8Array): CompressedFheInt40;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): CompressedFheInt40;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt40;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt48 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt48;
  static deserialize(buffer: Uint8Array): CompressedFheInt48;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): CompressedFheInt48;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt48;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt512 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt512;
  static deserialize(buffer: Uint8Array): CompressedFheInt512;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt512;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt512;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt56 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt56;
  static deserialize(buffer: Uint8Array): CompressedFheInt56;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): CompressedFheInt56;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt56;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt6 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt6;
  static deserialize(buffer: Uint8Array): CompressedFheInt6;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheInt6;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt6;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt64 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt64;
  static deserialize(buffer: Uint8Array): CompressedFheInt64;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): CompressedFheInt64;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt64;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt72 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt72;
  static deserialize(buffer: Uint8Array): CompressedFheInt72;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt72;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt72;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt8 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt8;
  static deserialize(buffer: Uint8Array): CompressedFheInt8;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheInt8;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt8;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt80 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt80;
  static deserialize(buffer: Uint8Array): CompressedFheInt80;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt80;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt80;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt88 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt88;
  static deserialize(buffer: Uint8Array): CompressedFheInt88;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt88;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt88;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheInt96 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheInt96;
  static deserialize(buffer: Uint8Array): CompressedFheInt96;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheInt96;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheInt96;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint10 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint10;
  static deserialize(buffer: Uint8Array): CompressedFheUint10;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheUint10;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint10;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint1024 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint1024;
  static deserialize(buffer: Uint8Array): CompressedFheUint1024;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint1024;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint1024;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint104 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint104;
  static deserialize(buffer: Uint8Array): CompressedFheUint104;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint104;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint104;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint112 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint112;
  static deserialize(buffer: Uint8Array): CompressedFheUint112;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint112;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint112;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint12 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint12;
  static deserialize(buffer: Uint8Array): CompressedFheUint12;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheUint12;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint12;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint120 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint120;
  static deserialize(buffer: Uint8Array): CompressedFheUint120;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint120;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint120;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint128 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint128;
  static deserialize(buffer: Uint8Array): CompressedFheUint128;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint128;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint128;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint136 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint136;
  static deserialize(buffer: Uint8Array): CompressedFheUint136;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint136;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint136;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint14 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint14;
  static deserialize(buffer: Uint8Array): CompressedFheUint14;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheUint14;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint14;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint144 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint144;
  static deserialize(buffer: Uint8Array): CompressedFheUint144;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint144;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint144;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint152 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint152;
  static deserialize(buffer: Uint8Array): CompressedFheUint152;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint152;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint152;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint16 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint16;
  static deserialize(buffer: Uint8Array): CompressedFheUint16;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheUint16;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint16;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint160 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint160;
  static deserialize(buffer: Uint8Array): CompressedFheUint160;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint160;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint160;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint168 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint168;
  static deserialize(buffer: Uint8Array): CompressedFheUint168;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint168;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint168;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint176 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint176;
  static deserialize(buffer: Uint8Array): CompressedFheUint176;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint176;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint176;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint184 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint184;
  static deserialize(buffer: Uint8Array): CompressedFheUint184;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint184;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint184;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint192 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint192;
  static deserialize(buffer: Uint8Array): CompressedFheUint192;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint192;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint192;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint2 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint2;
  static deserialize(buffer: Uint8Array): CompressedFheUint2;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheUint2;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint2;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint200 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint200;
  static deserialize(buffer: Uint8Array): CompressedFheUint200;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint200;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint200;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint2048 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint2048;
  static deserialize(buffer: Uint8Array): CompressedFheUint2048;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint2048;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint2048;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint208 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint208;
  static deserialize(buffer: Uint8Array): CompressedFheUint208;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint208;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint208;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint216 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint216;
  static deserialize(buffer: Uint8Array): CompressedFheUint216;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint216;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint216;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint224 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint224;
  static deserialize(buffer: Uint8Array): CompressedFheUint224;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint224;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint224;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint232 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint232;
  static deserialize(buffer: Uint8Array): CompressedFheUint232;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint232;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint232;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint24 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint24;
  static deserialize(buffer: Uint8Array): CompressedFheUint24;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheUint24;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint24;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint240 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint240;
  static deserialize(buffer: Uint8Array): CompressedFheUint240;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint240;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint240;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint248 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint248;
  static deserialize(buffer: Uint8Array): CompressedFheUint248;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint248;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint248;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint256 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint256;
  static deserialize(buffer: Uint8Array): CompressedFheUint256;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint256;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint256;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint32 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint32;
  static deserialize(buffer: Uint8Array): CompressedFheUint32;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheUint32;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint32;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint4 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint4;
  static deserialize(buffer: Uint8Array): CompressedFheUint4;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheUint4;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint4;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint40 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint40;
  static deserialize(buffer: Uint8Array): CompressedFheUint40;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): CompressedFheUint40;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint40;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint48 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint48;
  static deserialize(buffer: Uint8Array): CompressedFheUint48;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): CompressedFheUint48;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint48;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint512 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint512;
  static deserialize(buffer: Uint8Array): CompressedFheUint512;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint512;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint512;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint56 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint56;
  static deserialize(buffer: Uint8Array): CompressedFheUint56;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): CompressedFheUint56;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint56;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint6 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint6;
  static deserialize(buffer: Uint8Array): CompressedFheUint6;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheUint6;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint6;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint64 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint64;
  static deserialize(buffer: Uint8Array): CompressedFheUint64;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): CompressedFheUint64;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint64;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint72 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint72;
  static deserialize(buffer: Uint8Array): CompressedFheUint72;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint72;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint72;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint8 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint8;
  static deserialize(buffer: Uint8Array): CompressedFheUint8;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): CompressedFheUint8;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint8;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint80 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint80;
  static deserialize(buffer: Uint8Array): CompressedFheUint80;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint80;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint80;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint88 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint88;
  static deserialize(buffer: Uint8Array): CompressedFheUint88;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint88;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint88;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class CompressedFheUint96 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): FheUint96;
  static deserialize(buffer: Uint8Array): CompressedFheUint96;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): CompressedFheUint96;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): CompressedFheUint96;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheBool {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): boolean;
  static deserialize(buffer: Uint8Array): FheBool;
  static encrypt_with_client_key(
    value: boolean,
    client_key: TfheClientKey,
  ): FheBool;
  static encrypt_with_compressed_public_key(
    value: boolean,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheBool;
  static encrypt_with_public_key(
    value: boolean,
    public_key: TfhePublicKey,
  ): FheBool;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheBool;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt10 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheInt10;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheInt10;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt10;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheInt10;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt10;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt1024 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt1024;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt1024;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt1024;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt1024;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt1024;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt104 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt104;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt104;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt104;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt104;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt104;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt112 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt112;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt112;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt112;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt112;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt112;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt12 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheInt12;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheInt12;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt12;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheInt12;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt12;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt120 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt120;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt120;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt120;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt120;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt120;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt128 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt128;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt128;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt128;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt128;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt128;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt136 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt136;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt136;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt136;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt136;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt136;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt14 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheInt14;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheInt14;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt14;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheInt14;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt14;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt144 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt144;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt144;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt144;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt144;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt144;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt152 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt152;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt152;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt152;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt152;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt152;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt16 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheInt16;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheInt16;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt16;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheInt16;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt16;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt160 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt160;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt160;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt160;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt160;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt160;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt168 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt168;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt168;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt168;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt168;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt168;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt176 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt176;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt176;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt176;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt176;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt176;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt184 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt184;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt184;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt184;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt184;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt184;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt192 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt192;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt192;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt192;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt192;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt192;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt2 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheInt2;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheInt2;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt2;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheInt2;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt2;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt200 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt200;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt200;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt200;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt200;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt200;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt2048 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt2048;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt2048;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt2048;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt2048;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt2048;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt208 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt208;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt208;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt208;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt208;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt208;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt216 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt216;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt216;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt216;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt216;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt216;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt224 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt224;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt224;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt224;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt224;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt224;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt232 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt232;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt232;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt232;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt232;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt232;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt24 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheInt24;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheInt24;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt24;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheInt24;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt24;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt240 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt240;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt240;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt240;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt240;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt240;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt248 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt248;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt248;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt248;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt248;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt248;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt256 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt256;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt256;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt256;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt256;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt256;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt32 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheInt32;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheInt32;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt32;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheInt32;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt32;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt4 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheInt4;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheInt4;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt4;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheInt4;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt4;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt40 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): bigint;
  static deserialize(buffer: Uint8Array): FheInt40;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): FheInt40;
  static encrypt_with_compressed_public_key(
    value: bigint,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt40;
  static encrypt_with_public_key(
    value: bigint,
    public_key: TfhePublicKey,
  ): FheInt40;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt40;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt48 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): bigint;
  static deserialize(buffer: Uint8Array): FheInt48;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): FheInt48;
  static encrypt_with_compressed_public_key(
    value: bigint,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt48;
  static encrypt_with_public_key(
    value: bigint,
    public_key: TfhePublicKey,
  ): FheInt48;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt48;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt512 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt512;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt512;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt512;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt512;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt512;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt56 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): bigint;
  static deserialize(buffer: Uint8Array): FheInt56;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): FheInt56;
  static encrypt_with_compressed_public_key(
    value: bigint,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt56;
  static encrypt_with_public_key(
    value: bigint,
    public_key: TfhePublicKey,
  ): FheInt56;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt56;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt6 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheInt6;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheInt6;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt6;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheInt6;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt6;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt64 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): bigint;
  static deserialize(buffer: Uint8Array): FheInt64;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): FheInt64;
  static encrypt_with_compressed_public_key(
    value: bigint,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt64;
  static encrypt_with_public_key(
    value: bigint,
    public_key: TfhePublicKey,
  ): FheInt64;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt64;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt72 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt72;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt72;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt72;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt72;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt72;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt8 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheInt8;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheInt8;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt8;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheInt8;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt8;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt80 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt80;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt80;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt80;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt80;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt80;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt88 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt88;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt88;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt88;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt88;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt88;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheInt96 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheInt96;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheInt96;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheInt96;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheInt96;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheInt96;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
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

export class FheUint10 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheUint10;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheUint10;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint10;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheUint10;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint10;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint1024 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint1024;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint1024;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint1024;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint1024;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint1024;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint104 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint104;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint104;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint104;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint104;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint104;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint112 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint112;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint112;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint112;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint112;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint112;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint12 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheUint12;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheUint12;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint12;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheUint12;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint12;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint120 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint120;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint120;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint120;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint120;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint120;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint128 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint128;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint128;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint128;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint128;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint128;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint136 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint136;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint136;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint136;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint136;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint136;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint14 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheUint14;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheUint14;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint14;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheUint14;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint14;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint144 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint144;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint144;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint144;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint144;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint144;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint152 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint152;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint152;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint152;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint152;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint152;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint16 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheUint16;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheUint16;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint16;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheUint16;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint16;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint160 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint160;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint160;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint160;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint160;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint160;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint168 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint168;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint168;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint168;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint168;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint168;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint176 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint176;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint176;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint176;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint176;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint176;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint184 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint184;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint184;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint184;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint184;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint184;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint192 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint192;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint192;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint192;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint192;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint192;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint2 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheUint2;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheUint2;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint2;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheUint2;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint2;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint200 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint200;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint200;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint200;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint200;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint200;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint2048 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint2048;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint2048;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint2048;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint2048;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint2048;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint208 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint208;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint208;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint208;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint208;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint208;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint216 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint216;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint216;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint216;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint216;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint216;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint224 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint224;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint224;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint224;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint224;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint224;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint232 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint232;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint232;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint232;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint232;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint232;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint24 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheUint24;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheUint24;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint24;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheUint24;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint24;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint240 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint240;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint240;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint240;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint240;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint240;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint248 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint248;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint248;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint248;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint248;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint248;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint256 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint256;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint256;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint256;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint256;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint256;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint32 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheUint32;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheUint32;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint32;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheUint32;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint32;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint4 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheUint4;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheUint4;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint4;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheUint4;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint4;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint40 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): bigint;
  static deserialize(buffer: Uint8Array): FheUint40;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): FheUint40;
  static encrypt_with_compressed_public_key(
    value: bigint,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint40;
  static encrypt_with_public_key(
    value: bigint,
    public_key: TfhePublicKey,
  ): FheUint40;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint40;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint48 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): bigint;
  static deserialize(buffer: Uint8Array): FheUint48;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): FheUint48;
  static encrypt_with_compressed_public_key(
    value: bigint,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint48;
  static encrypt_with_public_key(
    value: bigint,
    public_key: TfhePublicKey,
  ): FheUint48;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint48;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint512 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint512;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint512;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint512;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint512;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint512;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint56 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): bigint;
  static deserialize(buffer: Uint8Array): FheUint56;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): FheUint56;
  static encrypt_with_compressed_public_key(
    value: bigint,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint56;
  static encrypt_with_public_key(
    value: bigint,
    public_key: TfhePublicKey,
  ): FheUint56;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint56;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint6 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheUint6;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheUint6;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint6;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheUint6;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint6;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint64 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): bigint;
  static deserialize(buffer: Uint8Array): FheUint64;
  static encrypt_with_client_key(
    value: bigint,
    client_key: TfheClientKey,
  ): FheUint64;
  static encrypt_with_compressed_public_key(
    value: bigint,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint64;
  static encrypt_with_public_key(
    value: bigint,
    public_key: TfhePublicKey,
  ): FheUint64;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint64;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint72 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint72;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint72;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint72;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint72;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint72;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint8 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): number;
  static deserialize(buffer: Uint8Array): FheUint8;
  static encrypt_with_client_key(
    value: number,
    client_key: TfheClientKey,
  ): FheUint8;
  static encrypt_with_compressed_public_key(
    value: number,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint8;
  static encrypt_with_public_key(
    value: number,
    public_key: TfhePublicKey,
  ): FheUint8;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint8;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint80 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint80;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint80;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint80;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint80;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint80;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint88 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint88;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint88;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint88;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint88;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint88;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class FheUint96 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decrypt(client_key: TfheClientKey): any;
  static deserialize(buffer: Uint8Array): FheUint96;
  static encrypt_with_client_key(
    value: any,
    client_key: TfheClientKey,
  ): FheUint96;
  static encrypt_with_compressed_public_key(
    value: any,
    compressed_public_key: TfheCompressedPublicKey,
  ): FheUint96;
  static encrypt_with_public_key(
    value: any,
    public_key: TfhePublicKey,
  ): FheUint96;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): FheUint96;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class ProvenCompactCiphertextList {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static builder(
    public_key: TfheCompactPublicKey,
  ): CompactCiphertextListBuilder;
  static deserialize(buffer: Uint8Array): ProvenCompactCiphertextList;
  expand_without_verification(): CompactCiphertextListExpander;
  get_kind_of(index: number): FheTypes | undefined;
  is_empty(): boolean;
  len(): number;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): ProvenCompactCiphertextList;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
  verify_and_expand(
    crs: CompactPkeCrs,
    public_key: TfheCompactPublicKey,
    metadata: Uint8Array,
  ): CompactCiphertextListExpander;
}

export class Shortint {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static decompress_ciphertext(
    compressed_ciphertext: ShortintCompressedCiphertext,
  ): ShortintCiphertext;
  static decrypt(client_key: ShortintClientKey, ct: ShortintCiphertext): bigint;
  static deserialize_ciphertext(buffer: Uint8Array): ShortintCiphertext;
  static deserialize_client_key(buffer: Uint8Array): ShortintClientKey;
  static deserialize_compressed_ciphertext(
    buffer: Uint8Array,
  ): ShortintCompressedCiphertext;
  static deserialize_compressed_public_key(
    buffer: Uint8Array,
  ): ShortintCompressedPublicKey;
  static deserialize_compressed_server_key(
    buffer: Uint8Array,
  ): ShortintCompressedServerKey;
  static deserialize_public_key(buffer: Uint8Array): ShortintPublicKey;
  static encrypt(
    client_key: ShortintClientKey,
    message: bigint,
  ): ShortintCiphertext;
  static encrypt_compressed(
    client_key: ShortintClientKey,
    message: bigint,
  ): ShortintCompressedCiphertext;
  static encrypt_with_compressed_public_key(
    public_key: ShortintCompressedPublicKey,
    message: bigint,
  ): ShortintCiphertext;
  static encrypt_with_public_key(
    public_key: ShortintPublicKey,
    message: bigint,
  ): ShortintCiphertext;
  static new_client_key(parameters: ShortintParameters): ShortintClientKey;
  static new_client_key_from_seed_and_parameters(
    seed_high_bytes: bigint,
    seed_low_bytes: bigint,
    parameters: ShortintParameters,
  ): ShortintClientKey;
  static new_compressed_public_key(
    client_key: ShortintClientKey,
  ): ShortintCompressedPublicKey;
  static new_compressed_server_key(
    client_key: ShortintClientKey,
  ): ShortintCompressedServerKey;
  static new_gaussian_from_std_dev(std_dev: number): ShortintNoiseDistribution;
  static new_parameters(
    lwe_dimension: number,
    glwe_dimension: number,
    polynomial_size: number,
    lwe_noise_distribution: ShortintNoiseDistribution,
    glwe_noise_distribution: ShortintNoiseDistribution,
    pbs_base_log: number,
    pbs_level: number,
    ks_base_log: number,
    ks_level: number,
    message_modulus: bigint,
    carry_modulus: bigint,
    max_noise_level: bigint,
    log2_p_fail: number,
    modulus_power_of_2_exponent: number,
    encryption_key_choice: ShortintEncryptionKeyChoice,
  ): ShortintParameters;
  static new_public_key(client_key: ShortintClientKey): ShortintPublicKey;
  static serialize_ciphertext(ciphertext: ShortintCiphertext): Uint8Array;
  static serialize_client_key(client_key: ShortintClientKey): Uint8Array;
  static serialize_compressed_ciphertext(
    ciphertext: ShortintCompressedCiphertext,
  ): Uint8Array;
  static serialize_compressed_public_key(
    public_key: ShortintCompressedPublicKey,
  ): Uint8Array;
  static serialize_compressed_server_key(
    server_key: ShortintCompressedServerKey,
  ): Uint8Array;
  static serialize_public_key(public_key: ShortintPublicKey): Uint8Array;
  static try_new_t_uniform(bound_log2: number): ShortintNoiseDistribution;
}

export class ShortintCiphertext {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export class ShortintClientKey {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export class ShortintCompactPublicKeyEncryptionParameters {
  free(): void;
  [Symbol.dispose](): void;
  constructor(name: ShortintCompactPublicKeyEncryptionParametersName);
  static new_parameters(
    encryption_lwe_dimension: number,
    encryption_noise_distribution: ShortintNoiseDistribution,
    message_modulus: bigint,
    carry_modulus: bigint,
    modulus_power_of_2_exponent: number,
    ks_base_log: number,
    ks_level: number,
    encryption_key_choice: ShortintEncryptionKeyChoice,
  ): ShortintCompactPublicKeyEncryptionParameters;
}

export enum ShortintCompactPublicKeyEncryptionParametersName {
  PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128 = 0,
  V1_1_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128 = 1,
  V1_1_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1 = 2,
  V1_0_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128 = 3,
  V1_0_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1 = 4,
  V0_11_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64 = 5,
  V0_11_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64_ZKV1 = 6,
  V1_2_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128 = 7,
  V1_2_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1 = 8,
  V1_3_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128 = 9,
  V1_3_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1 = 10,
  V1_4_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128 = 11,
  V1_4_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1 = 12,
  V1_5_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128 = 13,
  V1_5_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1 = 14,
}

export class ShortintCompressedCiphertext {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export class ShortintCompressedPublicKey {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export class ShortintCompressedServerKey {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export enum ShortintEncryptionKeyChoice {
  Big = 0,
  Small = 1,
}

export class ShortintNoiseDistribution {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export enum ShortintPBSOrder {
  KeyswitchBootstrap = 0,
  BootstrapKeyswitch = 1,
}

export class ShortintParameters {
  free(): void;
  [Symbol.dispose](): void;
  carry_modulus(): bigint;
  encryption_key_choice(): ShortintEncryptionKeyChoice;
  glwe_dimension(): number;
  glwe_noise_distribution(): ShortintNoiseDistribution;
  ks_base_log(): number;
  ks_level(): number;
  lwe_dimension(): number;
  lwe_noise_distribution(): ShortintNoiseDistribution;
  message_modulus(): bigint;
  constructor(name?: ShortintParametersName | null);
  pbs_base_log(): number;
  pbs_level(): number;
  polynomial_size(): number;
  set_carry_modulus(new_value: bigint): void;
  set_encryption_key_choice(new_value: ShortintEncryptionKeyChoice): void;
  set_glwe_dimension(new_value: number): void;
  set_glwe_noise_distribution(new_value: ShortintNoiseDistribution): void;
  set_ks_base_log(new_value: number): void;
  set_ks_level(new_value: number): void;
  set_lwe_dimension(new_value: number): void;
  set_lwe_noise_distribution(new_value: ShortintNoiseDistribution): void;
  set_message_modulus(new_value: bigint): void;
  set_pbs_base_log(new_value: number): void;
  set_pbs_level(new_value: number): void;
  set_polynomial_size(new_value: number): void;
}

export enum ShortintParametersName {
  PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128 = 0,
  V1_1_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 1,
  V1_1_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 2,
  V1_1_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 3,
  V1_1_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 4,
  V1_1_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 5,
  V1_1_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 6,
  V1_1_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 7,
  V1_1_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 8,
  V1_1_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 9,
  V1_1_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 10,
  V1_1_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 11,
  V1_1_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 12,
  V1_1_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 13,
  V1_1_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 14,
  V1_1_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 15,
  V1_1_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 16,
  V1_1_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 17,
  V1_1_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 18,
  V1_1_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 19,
  V1_1_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 20,
  V1_1_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 21,
  V1_1_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128 = 22,
  V1_1_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 23,
  V1_1_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 24,
  V1_1_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 25,
  V1_1_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 26,
  V1_1_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 27,
  V1_1_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 28,
  V1_1_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128 = 29,
  V1_1_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128 = 30,
  V1_1_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 31,
  V1_1_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 32,
  V1_1_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 33,
  V1_1_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 34,
  V1_1_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 35,
  V1_1_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 36,
  V1_1_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128 = 37,
  V1_1_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128 = 38,
  V1_1_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128 = 39,
  V1_1_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128 = 40,
  V1_1_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 41,
  V1_1_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 42,
  V1_1_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 43,
  V1_1_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 44,
  V1_1_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 45,
  V1_1_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 46,
  V1_1_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 47,
  V1_1_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 48,
  V1_1_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 49,
  V1_1_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 50,
  V1_1_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 51,
  V1_1_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 52,
  V1_1_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 53,
  V1_1_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 54,
  V1_1_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 55,
  V1_1_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 56,
  V1_1_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 57,
  V1_1_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 58,
  V1_1_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 59,
  V1_1_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 60,
  V1_1_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 61,
  V1_1_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 62,
  V1_1_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 63,
  V1_1_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 64,
  V1_1_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 65,
  V1_1_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 66,
  V1_1_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 67,
  V1_1_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 68,
  V1_1_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 69,
  V1_1_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 70,
  V1_1_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 71,
  V1_0_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 72,
  V1_0_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 73,
  V1_0_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 74,
  V1_0_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 75,
  V1_0_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 76,
  V1_0_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 77,
  V1_0_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 78,
  V1_0_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 79,
  V1_0_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 80,
  V1_0_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 81,
  V1_0_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 82,
  V1_0_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 83,
  V1_0_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 84,
  V1_0_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 85,
  V1_0_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 86,
  V1_0_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 87,
  V1_0_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 88,
  V1_0_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 89,
  V1_0_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 90,
  V1_0_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 91,
  V1_0_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 92,
  V1_0_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128 = 93,
  V1_0_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 94,
  V1_0_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 95,
  V1_0_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 96,
  V1_0_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 97,
  V1_0_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 98,
  V1_0_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 99,
  V1_0_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128 = 100,
  V1_0_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128 = 101,
  V1_0_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 102,
  V1_0_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 103,
  V1_0_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 104,
  V1_0_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 105,
  V1_0_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 106,
  V1_0_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 107,
  V1_0_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128 = 108,
  V1_0_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128 = 109,
  V1_0_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128 = 110,
  V1_0_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128 = 111,
  V1_0_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 112,
  V1_0_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 113,
  V1_0_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 114,
  V1_0_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 115,
  V1_0_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 116,
  V1_0_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 117,
  V1_0_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 118,
  V1_0_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 119,
  V1_0_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 120,
  V1_0_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 121,
  V1_0_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 122,
  V1_0_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 123,
  V1_0_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 124,
  V1_0_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 125,
  V1_0_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 126,
  V1_0_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 127,
  V1_0_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 128,
  V1_0_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 129,
  V1_0_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 130,
  V1_0_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 131,
  V1_0_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 132,
  V1_0_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 133,
  V1_0_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 134,
  V1_0_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 135,
  V1_0_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 136,
  V1_0_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 137,
  V1_0_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 138,
  V1_0_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 139,
  V1_0_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 140,
  V1_0_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 141,
  V1_0_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 142,
  V0_11_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M64 = 143,
  V0_11_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M64 = 144,
  V0_11_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M64 = 145,
  V0_11_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M64 = 146,
  V0_11_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M64 = 147,
  V0_11_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M64 = 148,
  V0_11_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M64 = 149,
  V0_11_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M64 = 150,
  V0_11_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M64 = 151,
  V0_11_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M64 = 152,
  V0_11_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M64 = 153,
  V0_11_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M64 = 154,
  V0_11_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M64 = 155,
  V0_11_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M64 = 156,
  V0_11_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M64 = 157,
  V0_11_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M64 = 158,
  V0_11_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M64 = 159,
  V0_11_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M64 = 160,
  V0_11_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M64 = 161,
  V0_11_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M64 = 162,
  V0_11_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M64 = 163,
  V0_11_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M64 = 164,
  V0_11_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M64 = 165,
  V0_11_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M64 = 166,
  V0_11_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M64 = 167,
  V0_11_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M64 = 168,
  V0_11_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M64 = 169,
  V0_11_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M64 = 170,
  V0_11_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M64 = 171,
  V0_11_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M64 = 172,
  V0_11_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M64 = 173,
  V0_11_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M64 = 174,
  V0_11_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M64 = 175,
  V0_11_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M64 = 176,
  V0_11_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M64 = 177,
  V0_11_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M64 = 178,
  V0_11_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M64 = 179,
  V0_11_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M64 = 180,
  V0_11_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M64 = 181,
  V0_11_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M64 = 182,
  V0_11_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 183,
  V0_11_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 184,
  V0_11_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 185,
  V0_11_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 186,
  V0_11_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 187,
  V0_11_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 188,
  V0_11_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 189,
  V0_11_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 190,
  V0_11_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 191,
  V0_11_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 192,
  V0_11_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 193,
  V0_11_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 194,
  V0_11_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 195,
  V0_11_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 196,
  V0_11_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 197,
  V0_11_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 198,
  V0_11_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 199,
  V0_11_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 200,
  V0_11_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 201,
  V0_11_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 202,
  V0_11_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 203,
  V0_11_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 204,
  V0_11_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 205,
  V0_11_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 206,
  V0_11_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 207,
  V0_11_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 208,
  V0_11_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M64 = 209,
  V0_11_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M64 = 210,
  V0_11_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M64 = 211,
  V0_11_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M64 = 212,
  V0_11_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M64 = 213,
  V1_2_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 214,
  V1_2_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 215,
  V1_2_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 216,
  V1_2_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 217,
  V1_2_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 218,
  V1_2_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 219,
  V1_2_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 220,
  V1_2_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 221,
  V1_2_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 222,
  V1_2_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 223,
  V1_2_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 224,
  V1_2_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 225,
  V1_2_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 226,
  V1_2_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 227,
  V1_2_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 228,
  V1_2_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 229,
  V1_2_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 230,
  V1_2_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 231,
  V1_2_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 232,
  V1_2_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 233,
  V1_2_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 234,
  V1_2_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128 = 235,
  V1_2_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 236,
  V1_2_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 237,
  V1_2_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 238,
  V1_2_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 239,
  V1_2_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 240,
  V1_2_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 241,
  V1_2_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128 = 242,
  V1_2_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128 = 243,
  V1_2_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 244,
  V1_2_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 245,
  V1_2_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 246,
  V1_2_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 247,
  V1_2_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 248,
  V1_2_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 249,
  V1_2_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128 = 250,
  V1_2_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128 = 251,
  V1_2_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128 = 252,
  V1_2_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128 = 253,
  V1_2_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 254,
  V1_2_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 255,
  V1_2_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 256,
  V1_2_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 257,
  V1_2_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 258,
  V1_2_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 259,
  V1_2_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 260,
  V1_2_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 261,
  V1_2_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 262,
  V1_2_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 263,
  V1_2_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 264,
  V1_2_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 265,
  V1_2_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 266,
  V1_2_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 267,
  V1_2_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 268,
  V1_2_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 269,
  V1_2_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 270,
  V1_2_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 271,
  V1_2_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 272,
  V1_2_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 273,
  V1_2_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 274,
  V1_2_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 275,
  V1_2_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 276,
  V1_2_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 277,
  V1_2_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 278,
  V1_2_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 279,
  V1_2_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 280,
  V1_2_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 281,
  V1_2_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 282,
  V1_2_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 283,
  V1_2_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 284,
  V1_3_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 285,
  V1_3_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 286,
  V1_3_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 287,
  V1_3_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 288,
  V1_3_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 289,
  V1_3_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 290,
  V1_3_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 291,
  V1_3_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 292,
  V1_3_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 293,
  V1_3_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 294,
  V1_3_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 295,
  V1_3_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 296,
  V1_3_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 297,
  V1_3_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 298,
  V1_3_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 299,
  V1_3_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 300,
  V1_3_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 301,
  V1_3_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 302,
  V1_3_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 303,
  V1_3_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 304,
  V1_3_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 305,
  V1_3_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128 = 306,
  V1_3_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 307,
  V1_3_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 308,
  V1_3_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 309,
  V1_3_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 310,
  V1_3_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 311,
  V1_3_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 312,
  V1_3_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128 = 313,
  V1_3_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128 = 314,
  V1_3_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 315,
  V1_3_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 316,
  V1_3_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 317,
  V1_3_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 318,
  V1_3_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 319,
  V1_3_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 320,
  V1_3_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128 = 321,
  V1_3_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128 = 322,
  V1_3_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128 = 323,
  V1_3_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128 = 324,
  V1_3_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 325,
  V1_3_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 326,
  V1_3_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 327,
  V1_3_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 328,
  V1_3_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 329,
  V1_3_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 330,
  V1_3_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 331,
  V1_3_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 332,
  V1_3_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 333,
  V1_3_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 334,
  V1_3_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 335,
  V1_3_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 336,
  V1_3_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 337,
  V1_3_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 338,
  V1_3_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 339,
  V1_3_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 340,
  V1_3_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 341,
  V1_3_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 342,
  V1_3_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 343,
  V1_3_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 344,
  V1_3_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 345,
  V1_3_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 346,
  V1_3_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 347,
  V1_3_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 348,
  V1_3_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 349,
  V1_3_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 350,
  V1_3_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 351,
  V1_3_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 352,
  V1_3_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 353,
  V1_3_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 354,
  V1_3_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 355,
  V1_4_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 356,
  V1_4_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 357,
  V1_4_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 358,
  V1_4_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 359,
  V1_4_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 360,
  V1_4_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 361,
  V1_4_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 362,
  V1_4_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 363,
  V1_4_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 364,
  V1_4_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 365,
  V1_4_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 366,
  V1_4_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 367,
  V1_4_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 368,
  V1_4_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 369,
  V1_4_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 370,
  V1_4_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 371,
  V1_4_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 372,
  V1_4_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 373,
  V1_4_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 374,
  V1_4_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 375,
  V1_4_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 376,
  V1_4_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128 = 377,
  V1_4_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 378,
  V1_4_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 379,
  V1_4_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 380,
  V1_4_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 381,
  V1_4_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 382,
  V1_4_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 383,
  V1_4_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128 = 384,
  V1_4_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128 = 385,
  V1_4_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 386,
  V1_4_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 387,
  V1_4_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 388,
  V1_4_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 389,
  V1_4_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 390,
  V1_4_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 391,
  V1_4_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128 = 392,
  V1_4_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128 = 393,
  V1_4_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128 = 394,
  V1_4_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128 = 395,
  V1_4_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 396,
  V1_4_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 397,
  V1_4_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 398,
  V1_4_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 399,
  V1_4_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 400,
  V1_4_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 401,
  V1_4_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 402,
  V1_4_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 403,
  V1_4_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 404,
  V1_4_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 405,
  V1_4_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 406,
  V1_4_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 407,
  V1_4_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 408,
  V1_4_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 409,
  V1_4_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 410,
  V1_4_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 411,
  V1_4_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 412,
  V1_4_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 413,
  V1_4_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 414,
  V1_4_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 415,
  V1_4_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 416,
  V1_4_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 417,
  V1_4_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 418,
  V1_4_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 419,
  V1_4_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 420,
  V1_4_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 421,
  V1_4_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 422,
  V1_4_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 423,
  V1_4_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 424,
  V1_4_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 425,
  V1_4_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 426,
  V1_5_PARAM_MESSAGE_1_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 427,
  V1_5_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 428,
  V1_5_PARAM_MESSAGE_2_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 429,
  V1_5_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 430,
  V1_5_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 431,
  V1_5_PARAM_MESSAGE_3_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 432,
  V1_5_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 433,
  V1_5_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 434,
  V1_5_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 435,
  V1_5_PARAM_MESSAGE_4_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 436,
  V1_5_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 437,
  V1_5_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 438,
  V1_5_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 439,
  V1_5_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 440,
  V1_5_PARAM_MESSAGE_5_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 441,
  V1_5_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 442,
  V1_5_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 443,
  V1_5_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 444,
  V1_5_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 445,
  V1_5_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 446,
  V1_5_PARAM_MESSAGE_6_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 447,
  V1_5_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128 = 448,
  V1_5_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 449,
  V1_5_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 450,
  V1_5_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 451,
  V1_5_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 452,
  V1_5_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 453,
  V1_5_PARAM_MESSAGE_7_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 454,
  V1_5_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128 = 455,
  V1_5_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128 = 456,
  V1_5_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128 = 457,
  V1_5_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M128 = 458,
  V1_5_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128 = 459,
  V1_5_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128 = 460,
  V1_5_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128 = 461,
  V1_5_PARAM_MESSAGE_8_CARRY_0_KS_PBS_GAUSSIAN_2M128 = 462,
  V1_5_PARAM_MESSAGE_1_CARRY_1_PBS_KS_GAUSSIAN_2M128 = 463,
  V1_5_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128 = 464,
  V1_5_PARAM_MESSAGE_3_CARRY_3_PBS_KS_GAUSSIAN_2M128 = 465,
  V1_5_PARAM_MESSAGE_4_CARRY_4_PBS_KS_GAUSSIAN_2M128 = 466,
  V1_5_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 467,
  V1_5_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 468,
  V1_5_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 469,
  V1_5_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 470,
  V1_5_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 471,
  V1_5_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 472,
  V1_5_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 473,
  V1_5_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 474,
  V1_5_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 475,
  V1_5_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 476,
  V1_5_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 477,
  V1_5_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 478,
  V1_5_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 479,
  V1_5_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 480,
  V1_5_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 481,
  V1_5_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 482,
  V1_5_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 483,
  V1_5_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 484,
  V1_5_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 485,
  V1_5_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 486,
  V1_5_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 487,
  V1_5_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 488,
  V1_5_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 489,
  V1_5_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 490,
  V1_5_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 491,
  V1_5_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 492,
  V1_5_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128 = 493,
  V1_5_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 494,
  V1_5_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 495,
  V1_5_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 496,
  V1_5_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128 = 497,
}

export class ShortintPublicKey {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export class TfheClientKey {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static deserialize(buffer: Uint8Array): TfheClientKey;
  static generate(config: TfheConfig): TfheClientKey;
  static generate_with_seed(config: TfheConfig, seed: any): TfheClientKey;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): TfheClientKey;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class TfheCompactPublicKey {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static deserialize(buffer: Uint8Array): TfheCompactPublicKey;
  static new(client_key: TfheClientKey): TfheCompactPublicKey;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): TfheCompactPublicKey;
  static safe_deserialize_conformant(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
    conformance_params: ShortintCompactPublicKeyEncryptionParameters,
  ): TfheCompactPublicKey;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class TfheCompressedCompactPublicKey {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): TfheCompactPublicKey;
  static deserialize(buffer: Uint8Array): TfheCompressedCompactPublicKey;
  static new(client_key: TfheClientKey): TfheCompressedCompactPublicKey;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): TfheCompressedCompactPublicKey;
  static safe_deserialize_conformant(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
    conformance_params: ShortintCompactPublicKeyEncryptionParameters,
  ): TfheCompressedCompactPublicKey;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class TfheCompressedPublicKey {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  decompress(): TfhePublicKey;
  static deserialize(buffer: Uint8Array): TfheCompressedPublicKey;
  static new(client_key: TfheClientKey): TfheCompressedPublicKey;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): TfheCompressedPublicKey;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class TfheCompressedServerKey {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static deserialize(buffer: Uint8Array): TfheCompressedServerKey;
  static new(client_key: TfheClientKey): TfheCompressedServerKey;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): TfheCompressedServerKey;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class TfheConfig {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export class TfheConfigBuilder {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  build(): TfheConfig;
  static default(): TfheConfigBuilder;
  use_custom_parameters(
    block_parameters: ShortintParameters,
  ): TfheConfigBuilder;
  use_dedicated_compact_public_key_parameters(
    compact_public_key_parameters: ShortintCompactPublicKeyEncryptionParameters,
  ): TfheConfigBuilder;
  static with_custom_parameters(
    block_parameters: ShortintParameters,
  ): TfheConfigBuilder;
}

export class TfhePublicKey {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static deserialize(buffer: Uint8Array): TfhePublicKey;
  static new(client_key: TfheClientKey): TfhePublicKey;
  static safe_deserialize(
    buffer: Uint8Array,
    serialized_size_limit: bigint,
  ): TfhePublicKey;
  safe_serialize(serialized_size_limit: bigint): Uint8Array;
  serialize(): Uint8Array;
}

export class TfheServerKey {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static new(client_key: TfheClientKey): TfheServerKey;
}

export enum ZkComputeLoad {
  Proof = 0,
  Verify = 1,
}

export function initThreadPool(num_threads: number): Promise<any>;

export function init_panic_hook(): void;

export function set_server_key(server_key: TfheServerKey): void;

export function shortint_params_name(
  param?: ShortintParametersName | null,
): string;

export function shortint_pke_params_name(
  param: ShortintCompactPublicKeyEncryptionParametersName,
): string;

export class tfhe {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export class wbg_rayon_PoolBuilder {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  build(): void;
  numThreads(): number;
  receiver(): number;
}

export function wbg_rayon_start_worker(receiver: number): void;

export type InitInput =
  | RequestInfo
  | URL
  | Response
  | BufferSource
  | WebAssembly.Module;

export interface InitOutput {
  readonly __wbg_boolean_free: (a: number, b: number) => void;
  readonly __wbg_booleanciphertext_free: (a: number, b: number) => void;
  readonly __wbg_booleanclientkey_free: (a: number, b: number) => void;
  readonly __wbg_booleancompressedciphertext_free: (
    a: number,
    b: number,
  ) => void;
  readonly __wbg_booleancompressedserverkey_free: (
    a: number,
    b: number,
  ) => void;
  readonly __wbg_booleannoisedistribution_free: (a: number, b: number) => void;
  readonly __wbg_booleanparameters_free: (a: number, b: number) => void;
  readonly __wbg_booleanpublickey_free: (a: number, b: number) => void;
  readonly __wbg_tfhe_free: (a: number, b: number) => void;
  readonly boolean_decompress_ciphertext: (a: number) => number;
  readonly boolean_decrypt: (a: number, b: number) => number;
  readonly boolean_deserialize_ciphertext: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly boolean_deserialize_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly boolean_deserialize_compressed_ciphertext: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly boolean_deserialize_compressed_server_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly boolean_deserialize_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly boolean_encrypt: (a: number, b: number) => number;
  readonly boolean_encrypt_compressed: (a: number, b: number) => number;
  readonly boolean_encrypt_with_public_key: (a: number, b: number) => number;
  readonly boolean_get_parameters: (a: number) => [number, number, number];
  readonly boolean_new_client_key: (a: number) => number;
  readonly boolean_new_client_key_from_seed_and_parameters: (
    a: bigint,
    b: bigint,
    c: number,
  ) => number;
  readonly boolean_new_compressed_server_key: (a: number) => number;
  readonly boolean_new_gaussian_from_std_dev: (a: number) => number;
  readonly boolean_new_parameters: (
    a: number,
    b: number,
    c: number,
    d: number,
    e: number,
    f: number,
    g: number,
    h: number,
    i: number,
    j: number,
  ) => number;
  readonly boolean_new_public_key: (a: number) => number;
  readonly boolean_serialize_ciphertext: (
    a: number,
  ) => [number, number, number, number];
  readonly boolean_serialize_client_key: (
    a: number,
  ) => [number, number, number, number];
  readonly boolean_serialize_compressed_ciphertext: (
    a: number,
  ) => [number, number, number, number];
  readonly boolean_serialize_compressed_server_key: (
    a: number,
  ) => [number, number, number, number];
  readonly boolean_serialize_public_key: (
    a: number,
  ) => [number, number, number, number];
  readonly boolean_trivial_encrypt: (a: number) => number;
  readonly boolean_try_new_t_uniform: (a: number) => [number, number, number];
  readonly __wbg_shortint_free: (a: number, b: number) => void;
  readonly __wbg_shortintciphertext_free: (a: number, b: number) => void;
  readonly __wbg_shortintclientkey_free: (a: number, b: number) => void;
  readonly __wbg_shortintcompactpublickeyencryptionparameters_free: (
    a: number,
    b: number,
  ) => void;
  readonly __wbg_shortintcompressedciphertext_free: (
    a: number,
    b: number,
  ) => void;
  readonly __wbg_shortintcompressedpublickey_free: (
    a: number,
    b: number,
  ) => void;
  readonly __wbg_shortintcompressedserverkey_free: (
    a: number,
    b: number,
  ) => void;
  readonly __wbg_shortintnoisedistribution_free: (a: number, b: number) => void;
  readonly __wbg_shortintparameters_free: (a: number, b: number) => void;
  readonly __wbg_shortintpublickey_free: (a: number, b: number) => void;
  readonly shortint_decompress_ciphertext: (a: number) => number;
  readonly shortint_decrypt: (a: number, b: number) => bigint;
  readonly shortint_deserialize_ciphertext: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly shortint_deserialize_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly shortint_deserialize_compressed_ciphertext: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly shortint_deserialize_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly shortint_deserialize_compressed_server_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly shortint_deserialize_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly shortint_encrypt: (a: number, b: bigint) => number;
  readonly shortint_encrypt_compressed: (a: number, b: bigint) => number;
  readonly shortint_encrypt_with_compressed_public_key: (
    a: number,
    b: bigint,
  ) => number;
  readonly shortint_encrypt_with_public_key: (a: number, b: bigint) => number;
  readonly shortint_new_client_key: (a: number) => number;
  readonly shortint_new_client_key_from_seed_and_parameters: (
    a: bigint,
    b: bigint,
    c: number,
  ) => number;
  readonly shortint_new_compressed_public_key: (a: number) => number;
  readonly shortint_new_compressed_server_key: (a: number) => number;
  readonly shortint_new_gaussian_from_std_dev: (a: number) => number;
  readonly shortint_new_parameters: (
    a: number,
    b: number,
    c: number,
    d: number,
    e: number,
    f: number,
    g: number,
    h: number,
    i: number,
    j: bigint,
    k: bigint,
    l: bigint,
    m: number,
    n: number,
    o: number,
  ) => number;
  readonly shortint_new_public_key: (a: number) => number;
  readonly shortint_params_name: (
    a: number,
  ) => [number, number, number, number];
  readonly shortint_pke_params_name: (a: number) => [number, number];
  readonly shortint_serialize_ciphertext: (
    a: number,
  ) => [number, number, number, number];
  readonly shortint_serialize_client_key: (
    a: number,
  ) => [number, number, number, number];
  readonly shortint_serialize_compressed_ciphertext: (
    a: number,
  ) => [number, number, number, number];
  readonly shortint_serialize_compressed_public_key: (
    a: number,
  ) => [number, number, number, number];
  readonly shortint_serialize_compressed_server_key: (
    a: number,
  ) => [number, number, number, number];
  readonly shortint_serialize_public_key: (
    a: number,
  ) => [number, number, number, number];
  readonly shortint_try_new_t_uniform: (a: number) => [number, number, number];
  readonly shortintcompactpublickeyencryptionparameters_new: (
    a: number,
  ) => number;
  readonly shortintcompactpublickeyencryptionparameters_new_parameters: (
    a: number,
    b: number,
    c: bigint,
    d: bigint,
    e: number,
    f: number,
    g: number,
    h: number,
  ) => [number, number, number];
  readonly shortintparameters_carry_modulus: (a: number) => bigint;
  readonly shortintparameters_encryption_key_choice: (a: number) => number;
  readonly shortintparameters_glwe_dimension: (a: number) => number;
  readonly shortintparameters_glwe_noise_distribution: (a: number) => number;
  readonly shortintparameters_ks_base_log: (a: number) => number;
  readonly shortintparameters_ks_level: (a: number) => number;
  readonly shortintparameters_lwe_dimension: (a: number) => number;
  readonly shortintparameters_message_modulus: (a: number) => bigint;
  readonly shortintparameters_new: (a: number) => [number, number, number];
  readonly shortintparameters_pbs_base_log: (a: number) => number;
  readonly shortintparameters_pbs_level: (a: number) => number;
  readonly shortintparameters_polynomial_size: (a: number) => number;
  readonly shortintparameters_set_carry_modulus: (a: number, b: bigint) => void;
  readonly shortintparameters_set_encryption_key_choice: (
    a: number,
    b: number,
  ) => void;
  readonly shortintparameters_set_glwe_dimension: (
    a: number,
    b: number,
  ) => void;
  readonly shortintparameters_set_glwe_noise_distribution: (
    a: number,
    b: number,
  ) => void;
  readonly shortintparameters_set_ks_base_log: (a: number, b: number) => void;
  readonly shortintparameters_set_ks_level: (a: number, b: number) => void;
  readonly shortintparameters_set_lwe_dimension: (a: number, b: number) => void;
  readonly shortintparameters_set_lwe_noise_distribution: (
    a: number,
    b: number,
  ) => void;
  readonly shortintparameters_set_message_modulus: (
    a: number,
    b: bigint,
  ) => void;
  readonly shortintparameters_set_pbs_base_log: (a: number, b: number) => void;
  readonly shortintparameters_set_pbs_level: (a: number, b: number) => void;
  readonly shortintparameters_set_polynomial_size: (
    a: number,
    b: number,
  ) => void;
  readonly shortintparameters_lwe_noise_distribution: (a: number) => number;
  readonly __wbg_compactciphertextlist_free: (a: number, b: number) => void;
  readonly __wbg_compactciphertextlistbuilder_free: (
    a: number,
    b: number,
  ) => void;
  readonly __wbg_compactciphertextlistexpander_free: (
    a: number,
    b: number,
  ) => void;
  readonly __wbg_compactpkecrs_free: (a: number, b: number) => void;
  readonly __wbg_compressedfhebool_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint1024_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint104_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint10_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint112_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint120_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint128_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint12_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint136_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint144_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint14_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint152_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint160_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint168_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint16_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint176_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint184_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint192_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint200_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint2048_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint208_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint216_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint224_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint232_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint240_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint248_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint24_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint256_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint2_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint32_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint40_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint48_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint4_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint512_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint56_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint64_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint6_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint72_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint80_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint88_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint8_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheint96_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint1024_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint104_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint10_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint112_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint120_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint128_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint12_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint136_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint144_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint14_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint152_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint160_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint168_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint16_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint176_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint184_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint192_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint200_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint2048_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint208_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint216_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint224_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint232_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint240_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint248_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint24_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint256_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint2_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint32_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint40_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint48_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint4_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint512_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint56_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint64_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint6_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint72_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint80_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint88_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint8_free: (a: number, b: number) => void;
  readonly __wbg_compressedfheuint96_free: (a: number, b: number) => void;
  readonly __wbg_fhebool_free: (a: number, b: number) => void;
  readonly __wbg_fheint1024_free: (a: number, b: number) => void;
  readonly __wbg_fheint104_free: (a: number, b: number) => void;
  readonly __wbg_fheint10_free: (a: number, b: number) => void;
  readonly __wbg_fheint112_free: (a: number, b: number) => void;
  readonly __wbg_fheint120_free: (a: number, b: number) => void;
  readonly __wbg_fheint128_free: (a: number, b: number) => void;
  readonly __wbg_fheint12_free: (a: number, b: number) => void;
  readonly __wbg_fheint136_free: (a: number, b: number) => void;
  readonly __wbg_fheint144_free: (a: number, b: number) => void;
  readonly __wbg_fheint14_free: (a: number, b: number) => void;
  readonly __wbg_fheint152_free: (a: number, b: number) => void;
  readonly __wbg_fheint160_free: (a: number, b: number) => void;
  readonly __wbg_fheint168_free: (a: number, b: number) => void;
  readonly __wbg_fheint16_free: (a: number, b: number) => void;
  readonly __wbg_fheint176_free: (a: number, b: number) => void;
  readonly __wbg_fheint184_free: (a: number, b: number) => void;
  readonly __wbg_fheint192_free: (a: number, b: number) => void;
  readonly __wbg_fheint200_free: (a: number, b: number) => void;
  readonly __wbg_fheint2048_free: (a: number, b: number) => void;
  readonly __wbg_fheint208_free: (a: number, b: number) => void;
  readonly __wbg_fheint216_free: (a: number, b: number) => void;
  readonly __wbg_fheint224_free: (a: number, b: number) => void;
  readonly __wbg_fheint232_free: (a: number, b: number) => void;
  readonly __wbg_fheint240_free: (a: number, b: number) => void;
  readonly __wbg_fheint248_free: (a: number, b: number) => void;
  readonly __wbg_fheint24_free: (a: number, b: number) => void;
  readonly __wbg_fheint256_free: (a: number, b: number) => void;
  readonly __wbg_fheint2_free: (a: number, b: number) => void;
  readonly __wbg_fheint32_free: (a: number, b: number) => void;
  readonly __wbg_fheint40_free: (a: number, b: number) => void;
  readonly __wbg_fheint48_free: (a: number, b: number) => void;
  readonly __wbg_fheint4_free: (a: number, b: number) => void;
  readonly __wbg_fheint512_free: (a: number, b: number) => void;
  readonly __wbg_fheint56_free: (a: number, b: number) => void;
  readonly __wbg_fheint64_free: (a: number, b: number) => void;
  readonly __wbg_fheint6_free: (a: number, b: number) => void;
  readonly __wbg_fheint72_free: (a: number, b: number) => void;
  readonly __wbg_fheint80_free: (a: number, b: number) => void;
  readonly __wbg_fheint88_free: (a: number, b: number) => void;
  readonly __wbg_fheint8_free: (a: number, b: number) => void;
  readonly __wbg_fheint96_free: (a: number, b: number) => void;
  readonly __wbg_fheuint1024_free: (a: number, b: number) => void;
  readonly __wbg_fheuint104_free: (a: number, b: number) => void;
  readonly __wbg_fheuint10_free: (a: number, b: number) => void;
  readonly __wbg_fheuint112_free: (a: number, b: number) => void;
  readonly __wbg_fheuint120_free: (a: number, b: number) => void;
  readonly __wbg_fheuint128_free: (a: number, b: number) => void;
  readonly __wbg_fheuint12_free: (a: number, b: number) => void;
  readonly __wbg_fheuint136_free: (a: number, b: number) => void;
  readonly __wbg_fheuint144_free: (a: number, b: number) => void;
  readonly __wbg_fheuint14_free: (a: number, b: number) => void;
  readonly __wbg_fheuint152_free: (a: number, b: number) => void;
  readonly __wbg_fheuint160_free: (a: number, b: number) => void;
  readonly __wbg_fheuint168_free: (a: number, b: number) => void;
  readonly __wbg_fheuint16_free: (a: number, b: number) => void;
  readonly __wbg_fheuint176_free: (a: number, b: number) => void;
  readonly __wbg_fheuint184_free: (a: number, b: number) => void;
  readonly __wbg_fheuint192_free: (a: number, b: number) => void;
  readonly __wbg_fheuint200_free: (a: number, b: number) => void;
  readonly __wbg_fheuint2048_free: (a: number, b: number) => void;
  readonly __wbg_fheuint208_free: (a: number, b: number) => void;
  readonly __wbg_fheuint216_free: (a: number, b: number) => void;
  readonly __wbg_fheuint224_free: (a: number, b: number) => void;
  readonly __wbg_fheuint232_free: (a: number, b: number) => void;
  readonly __wbg_fheuint240_free: (a: number, b: number) => void;
  readonly __wbg_fheuint248_free: (a: number, b: number) => void;
  readonly __wbg_fheuint24_free: (a: number, b: number) => void;
  readonly __wbg_fheuint256_free: (a: number, b: number) => void;
  readonly __wbg_fheuint2_free: (a: number, b: number) => void;
  readonly __wbg_fheuint32_free: (a: number, b: number) => void;
  readonly __wbg_fheuint40_free: (a: number, b: number) => void;
  readonly __wbg_fheuint48_free: (a: number, b: number) => void;
  readonly __wbg_fheuint4_free: (a: number, b: number) => void;
  readonly __wbg_fheuint512_free: (a: number, b: number) => void;
  readonly __wbg_fheuint56_free: (a: number, b: number) => void;
  readonly __wbg_fheuint64_free: (a: number, b: number) => void;
  readonly __wbg_fheuint6_free: (a: number, b: number) => void;
  readonly __wbg_fheuint72_free: (a: number, b: number) => void;
  readonly __wbg_fheuint80_free: (a: number, b: number) => void;
  readonly __wbg_fheuint88_free: (a: number, b: number) => void;
  readonly __wbg_fheuint8_free: (a: number, b: number) => void;
  readonly __wbg_fheuint96_free: (a: number, b: number) => void;
  readonly __wbg_provencompactciphertextlist_free: (
    a: number,
    b: number,
  ) => void;
  readonly __wbg_tfheclientkey_free: (a: number, b: number) => void;
  readonly __wbg_tfhecompactpublickey_free: (a: number, b: number) => void;
  readonly __wbg_tfhecompressedcompactpublickey_free: (
    a: number,
    b: number,
  ) => void;
  readonly __wbg_tfhecompressedpublickey_free: (a: number, b: number) => void;
  readonly __wbg_tfhecompressedserverkey_free: (a: number, b: number) => void;
  readonly __wbg_tfheconfig_free: (a: number, b: number) => void;
  readonly __wbg_tfheconfigbuilder_free: (a: number, b: number) => void;
  readonly __wbg_tfhepublickey_free: (a: number, b: number) => void;
  readonly __wbg_tfheserverkey_free: (a: number, b: number) => void;
  readonly compactciphertextlist_builder: (
    a: number,
  ) => [number, number, number];
  readonly compactciphertextlist_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlist_expand: (
    a: number,
  ) => [number, number, number];
  readonly compactciphertextlist_get_kind_of: (a: number, b: number) => number;
  readonly compactciphertextlist_is_empty: (a: number) => number;
  readonly compactciphertextlist_len: (a: number) => number;
  readonly compactciphertextlist_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compactciphertextlist_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compactciphertextlist_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compactciphertextlistbuilder_build: (
    a: number,
  ) => [number, number, number];
  readonly compactciphertextlistbuilder_build_packed: (
    a: number,
  ) => [number, number, number];
  readonly compactciphertextlistbuilder_build_with_proof_packed: (
    a: number,
    b: number,
    c: number,
    d: number,
    e: number,
  ) => [number, number, number];
  readonly compactciphertextlistbuilder_push_boolean: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i10: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i1024: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i104: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i112: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i12: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i120: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i128: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i136: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i14: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i144: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i152: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i16: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i160: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i168: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i176: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i184: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i192: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i2: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i200: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i2048: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i208: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i216: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i224: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i232: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i24: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i240: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i248: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i256: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i32: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i4: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i40: (
    a: number,
    b: bigint,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i48: (
    a: number,
    b: bigint,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i512: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i56: (
    a: number,
    b: bigint,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i6: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i64: (
    a: number,
    b: bigint,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i72: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i8: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i80: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i88: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_i96: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u10: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u1024: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u104: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u112: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u12: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u120: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u128: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u136: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u14: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u144: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u152: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u16: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u160: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u168: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u176: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u184: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u192: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u2: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u200: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u2048: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u208: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u216: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u224: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u232: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u24: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u240: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u248: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u256: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u32: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u4: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u40: (
    a: number,
    b: bigint,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u48: (
    a: number,
    b: bigint,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u512: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u56: (
    a: number,
    b: bigint,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u6: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u64: (
    a: number,
    b: bigint,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u72: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u8: (
    a: number,
    b: number,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u80: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u88: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistbuilder_push_u96: (
    a: number,
    b: any,
  ) => [number, number];
  readonly compactciphertextlistexpander_get_bool: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int10: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int1024: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int104: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int112: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int12: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int120: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int128: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int136: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int14: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int144: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int152: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int16: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int160: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int168: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int176: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int184: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int192: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int2: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int200: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int2048: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int208: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int216: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int224: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int232: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int24: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int240: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int248: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int256: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int32: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int4: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int40: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int48: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int512: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int56: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int6: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int64: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int72: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int8: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int80: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int88: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_int96: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_kind_of: (
    a: number,
    b: number,
  ) => number;
  readonly compactciphertextlistexpander_get_uint10: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint1024: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint104: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint112: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint12: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint120: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint128: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint136: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint14: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint144: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint152: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint16: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint160: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint168: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint176: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint184: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint192: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint2: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint200: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint2048: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint208: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint216: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint224: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint232: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint24: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint240: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint248: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint256: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint32: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint4: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint40: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint48: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint512: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint56: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint6: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint64: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint72: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint8: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint80: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint88: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_get_uint96: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactciphertextlistexpander_is_empty: (a: number) => number;
  readonly compactciphertextlistexpander_len: (a: number) => number;
  readonly compactpkecrs_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactpkecrs_deserialize_from_public_params: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactpkecrs_from_config: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compactpkecrs_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compactpkecrs_safe_deserialize_from_public_params: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compactpkecrs_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compactpkecrs_serialize: (
    a: number,
    b: number,
  ) => [number, number, number, number];
  readonly compressedfhebool_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfhebool_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfhebool_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfhebool_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfhebool_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfhebool_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint1024_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint1024_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint1024_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint1024_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint1024_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint1024_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint104_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint104_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint104_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint104_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint104_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint104_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint10_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint10_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint10_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint10_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint10_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint10_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint112_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint112_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint112_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint112_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint112_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint112_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint120_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint120_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint120_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint120_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint120_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint120_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint128_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint128_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint128_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint128_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint128_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint128_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint12_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint12_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint12_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint12_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint12_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint12_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint136_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint136_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint136_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint136_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint136_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint136_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint144_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint144_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint144_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint144_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint144_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint144_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint14_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint14_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint14_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint14_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint14_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint14_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint152_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint152_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint152_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint152_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint152_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint152_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint160_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint160_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint160_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint160_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint160_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint160_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint168_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint168_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint168_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint168_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint168_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint168_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint16_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint16_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint16_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint16_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint16_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint16_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint176_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint176_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint176_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint176_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint176_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint176_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint184_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint184_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint184_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint184_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint184_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint184_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint192_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint192_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint192_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint192_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint192_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint192_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint200_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint200_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint200_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint200_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint200_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint200_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint2048_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint2048_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint2048_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint2048_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint2048_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint2048_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint208_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint208_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint208_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint208_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint208_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint208_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint216_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint216_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint216_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint216_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint216_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint216_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint224_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint224_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint224_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint224_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint224_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint224_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint232_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint232_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint232_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint232_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint232_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint232_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint240_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint240_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint240_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint240_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint240_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint240_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint248_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint248_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint248_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint248_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint248_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint248_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint24_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint24_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint24_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint24_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint24_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint24_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint256_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint256_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint256_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint256_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint256_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint256_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint2_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint2_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint2_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint2_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint2_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint2_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint32_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint32_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint32_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint32_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint32_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint32_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint40_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint40_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint40_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint40_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint40_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint40_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint48_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint48_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint48_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint48_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint48_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint48_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint4_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint4_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint4_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint4_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint4_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint4_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint512_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint512_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint512_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint512_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint512_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint512_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint56_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint56_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint56_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint56_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint56_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint56_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint64_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint64_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint64_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint64_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint64_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint64_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint6_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint6_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint6_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint6_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint6_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint6_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint72_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint72_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint72_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint72_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint72_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint72_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint80_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint80_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint80_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint80_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint80_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint80_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint88_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint88_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint88_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint88_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint88_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint88_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint8_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint8_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint8_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint8_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint8_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint8_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheint96_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheint96_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint96_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheint96_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheint96_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheint96_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint1024_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint1024_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint1024_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint1024_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint1024_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint1024_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint104_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint104_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint104_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint104_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint104_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint104_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint10_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint10_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint10_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint10_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint10_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint10_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint112_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint112_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint112_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint112_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint112_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint112_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint120_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint120_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint120_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint120_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint120_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint120_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint128_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint128_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint128_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint128_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint128_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint128_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint12_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint12_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint12_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint12_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint12_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint12_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint136_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint136_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint136_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint136_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint136_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint136_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint144_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint144_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint144_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint144_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint144_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint144_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint14_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint14_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint14_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint14_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint14_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint14_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint152_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint152_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint152_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint152_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint152_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint152_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint160_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint160_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint160_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint160_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint160_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint160_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint168_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint168_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint168_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint168_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint168_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint168_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint16_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint16_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint16_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint16_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint16_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint16_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint176_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint176_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint176_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint176_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint176_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint176_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint184_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint184_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint184_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint184_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint184_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint184_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint192_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint192_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint192_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint192_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint192_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint192_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint200_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint200_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint200_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint200_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint200_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint200_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint2048_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint2048_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint2048_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint2048_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint2048_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint2048_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint208_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint208_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint208_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint208_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint208_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint208_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint216_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint216_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint216_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint216_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint216_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint216_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint224_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint224_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint224_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint224_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint224_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint224_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint232_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint232_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint232_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint232_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint232_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint232_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint240_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint240_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint240_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint240_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint240_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint240_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint248_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint248_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint248_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint248_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint248_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint248_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint24_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint24_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint24_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint24_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint24_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint24_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint256_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint256_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint256_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint256_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint256_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint256_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint2_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint2_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint2_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint2_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint2_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint2_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint32_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint32_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint32_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint32_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint32_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint32_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint40_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint40_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint40_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint40_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint40_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint40_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint48_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint48_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint48_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint48_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint48_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint48_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint4_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint4_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint4_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint4_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint4_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint4_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint512_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint512_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint512_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint512_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint512_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint512_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint56_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint56_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint56_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint56_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint56_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint56_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint64_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint64_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint64_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint64_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint64_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint64_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint6_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint6_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint6_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint6_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint6_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint6_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint72_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint72_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint72_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint72_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint72_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint72_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint80_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint80_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint80_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint80_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint80_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint80_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint88_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint88_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint88_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint88_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint88_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint88_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint8_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint8_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint8_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint8_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint8_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint8_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly compressedfheuint96_decompress: (
    a: number,
  ) => [number, number, number];
  readonly compressedfheuint96_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint96_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly compressedfheuint96_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly compressedfheuint96_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly compressedfheuint96_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fhebool_decrypt: (a: number, b: number) => [number, number, number];
  readonly fhebool_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fhebool_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fhebool_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fhebool_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fhebool_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fhebool_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fhebool_serialize: (a: number) => [number, number, number, number];
  readonly fheint1024_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint1024_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint1024_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint1024_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint1024_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint1024_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint1024_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint1024_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheint104_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint104_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint104_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint104_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint104_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint104_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint104_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint104_serialize: (a: number) => [number, number, number, number];
  readonly fheint10_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheint10_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint10_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint10_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint10_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint10_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint10_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint10_serialize: (a: number) => [number, number, number, number];
  readonly fheint112_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint112_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint112_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint112_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint112_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint112_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint112_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint112_serialize: (a: number) => [number, number, number, number];
  readonly fheint120_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint120_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint120_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint120_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint120_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint120_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint120_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint120_serialize: (a: number) => [number, number, number, number];
  readonly fheint128_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint128_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint128_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint128_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint128_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint128_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint128_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint128_serialize: (a: number) => [number, number, number, number];
  readonly fheint12_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheint12_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint12_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint12_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint12_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint12_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint12_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint12_serialize: (a: number) => [number, number, number, number];
  readonly fheint136_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint136_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint136_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint136_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint136_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint136_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint136_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint136_serialize: (a: number) => [number, number, number, number];
  readonly fheint144_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint144_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint144_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint144_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint144_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint144_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint144_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint144_serialize: (a: number) => [number, number, number, number];
  readonly fheint14_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheint14_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint14_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint14_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint14_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint14_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint14_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint14_serialize: (a: number) => [number, number, number, number];
  readonly fheint152_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint152_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint152_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint152_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint152_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint152_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint152_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint152_serialize: (a: number) => [number, number, number, number];
  readonly fheint160_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint160_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint160_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint160_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint160_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint160_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint160_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint160_serialize: (a: number) => [number, number, number, number];
  readonly fheint168_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint168_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint168_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint168_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint168_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint168_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint168_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint168_serialize: (a: number) => [number, number, number, number];
  readonly fheint16_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheint16_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint16_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint16_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint16_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint16_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint16_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint16_serialize: (a: number) => [number, number, number, number];
  readonly fheint176_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint176_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint176_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint176_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint176_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint176_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint176_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint176_serialize: (a: number) => [number, number, number, number];
  readonly fheint184_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint184_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint184_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint184_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint184_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint184_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint184_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint184_serialize: (a: number) => [number, number, number, number];
  readonly fheint192_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint192_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint192_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint192_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint192_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint192_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint192_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint192_serialize: (a: number) => [number, number, number, number];
  readonly fheint200_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint200_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint200_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint200_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint200_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint200_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint200_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint200_serialize: (a: number) => [number, number, number, number];
  readonly fheint2048_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint2048_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint2048_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint2048_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint2048_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint2048_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint2048_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint2048_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheint208_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint208_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint208_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint208_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint208_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint208_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint208_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint208_serialize: (a: number) => [number, number, number, number];
  readonly fheint216_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint216_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint216_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint216_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint216_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint216_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint216_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint216_serialize: (a: number) => [number, number, number, number];
  readonly fheint224_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint224_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint224_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint224_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint224_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint224_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint224_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint224_serialize: (a: number) => [number, number, number, number];
  readonly fheint232_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint232_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint232_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint232_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint232_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint232_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint232_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint232_serialize: (a: number) => [number, number, number, number];
  readonly fheint240_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint240_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint240_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint240_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint240_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint240_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint240_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint240_serialize: (a: number) => [number, number, number, number];
  readonly fheint248_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint248_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint248_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint248_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint248_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint248_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint248_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint248_serialize: (a: number) => [number, number, number, number];
  readonly fheint24_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheint24_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint24_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint24_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint24_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint24_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint24_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint24_serialize: (a: number) => [number, number, number, number];
  readonly fheint256_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint256_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint256_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint256_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint256_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint256_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint256_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint256_serialize: (a: number) => [number, number, number, number];
  readonly fheint2_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheint2_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint2_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint2_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint2_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint2_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint2_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint2_serialize: (a: number) => [number, number, number, number];
  readonly fheint32_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheint32_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint32_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint32_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint32_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint32_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint32_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint32_serialize: (a: number) => [number, number, number, number];
  readonly fheint40_decrypt: (a: number, b: number) => [bigint, number, number];
  readonly fheint40_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint40_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheint40_encrypt_with_compressed_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheint40_encrypt_with_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheint40_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint40_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint40_serialize: (a: number) => [number, number, number, number];
  readonly fheint48_decrypt: (a: number, b: number) => [bigint, number, number];
  readonly fheint48_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint48_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheint48_encrypt_with_compressed_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheint48_encrypt_with_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheint48_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint48_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint48_serialize: (a: number) => [number, number, number, number];
  readonly fheint4_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheint4_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint4_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint4_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint4_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint4_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint4_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint4_serialize: (a: number) => [number, number, number, number];
  readonly fheint512_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint512_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint512_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint512_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint512_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint512_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint512_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint512_serialize: (a: number) => [number, number, number, number];
  readonly fheint56_decrypt: (a: number, b: number) => [bigint, number, number];
  readonly fheint56_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint56_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheint56_encrypt_with_compressed_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheint56_encrypt_with_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheint56_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint56_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint56_serialize: (a: number) => [number, number, number, number];
  readonly fheint64_decrypt: (a: number, b: number) => [bigint, number, number];
  readonly fheint64_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint64_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheint64_encrypt_with_compressed_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheint64_encrypt_with_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheint64_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint64_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint64_serialize: (a: number) => [number, number, number, number];
  readonly fheint6_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheint6_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint6_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint6_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint6_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint6_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint6_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint6_serialize: (a: number) => [number, number, number, number];
  readonly fheint72_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheint72_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint72_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint72_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint72_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint72_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint72_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint72_serialize: (a: number) => [number, number, number, number];
  readonly fheint80_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheint80_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint80_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint80_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint80_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint80_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint80_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint80_serialize: (a: number) => [number, number, number, number];
  readonly fheint88_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheint88_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint88_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint88_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint88_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint88_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint88_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint88_serialize: (a: number) => [number, number, number, number];
  readonly fheint8_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheint8_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint8_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint8_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint8_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint8_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint8_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint8_serialize: (a: number) => [number, number, number, number];
  readonly fheint96_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheint96_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheint96_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint96_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint96_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheint96_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheint96_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheint96_serialize: (a: number) => [number, number, number, number];
  readonly fheuint1024_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint1024_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint1024_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint1024_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint1024_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint1024_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint1024_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint1024_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint104_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint104_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint104_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint104_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint104_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint104_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint104_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint104_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint10_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint10_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint10_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint10_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint10_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint10_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint10_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint10_serialize: (a: number) => [number, number, number, number];
  readonly fheuint112_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint112_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint112_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint112_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint112_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint112_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint112_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint112_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint120_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint120_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint120_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint120_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint120_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint120_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint120_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint120_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint128_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint128_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint128_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint128_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint128_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint128_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint128_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint128_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint12_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint12_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint12_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint12_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint12_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint12_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint12_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint12_serialize: (a: number) => [number, number, number, number];
  readonly fheuint136_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint136_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint136_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint136_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint136_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint136_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint136_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint136_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint144_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint144_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint144_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint144_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint144_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint144_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint144_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint144_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint14_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint14_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint14_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint14_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint14_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint14_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint14_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint14_serialize: (a: number) => [number, number, number, number];
  readonly fheuint152_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint152_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint152_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint152_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint152_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint152_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint152_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint152_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint160_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint160_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint160_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint160_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint160_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint160_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint160_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint160_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint168_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint168_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint168_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint168_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint168_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint168_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint168_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint168_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint16_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint16_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint16_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint16_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint16_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint16_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint16_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint16_serialize: (a: number) => [number, number, number, number];
  readonly fheuint176_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint176_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint176_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint176_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint176_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint176_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint176_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint176_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint184_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint184_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint184_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint184_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint184_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint184_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint184_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint184_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint192_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint192_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint192_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint192_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint192_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint192_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint192_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint192_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint200_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint200_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint200_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint200_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint200_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint200_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint200_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint200_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint2048_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint2048_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint2048_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint2048_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint2048_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint2048_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint2048_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint2048_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint208_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint208_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint208_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint208_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint208_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint208_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint208_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint208_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint216_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint216_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint216_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint216_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint216_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint216_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint216_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint216_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint224_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint224_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint224_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint224_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint224_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint224_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint224_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint224_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint232_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint232_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint232_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint232_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint232_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint232_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint232_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint232_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint240_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint240_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint240_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint240_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint240_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint240_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint240_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint240_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint248_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint248_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint248_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint248_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint248_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint248_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint248_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint248_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint24_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint24_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint24_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint24_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint24_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint24_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint24_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint24_serialize: (a: number) => [number, number, number, number];
  readonly fheuint256_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint256_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint256_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint256_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint256_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint256_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint256_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint256_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint2_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheuint2_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint2_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint2_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint2_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint2_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint2_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint2_serialize: (a: number) => [number, number, number, number];
  readonly fheuint32_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint32_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint32_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint32_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint32_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint32_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint32_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint32_serialize: (a: number) => [number, number, number, number];
  readonly fheuint40_decrypt: (
    a: number,
    b: number,
  ) => [bigint, number, number];
  readonly fheuint40_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint40_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheuint40_encrypt_with_compressed_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheuint40_encrypt_with_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheuint40_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint40_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint40_serialize: (a: number) => [number, number, number, number];
  readonly fheuint48_decrypt: (
    a: number,
    b: number,
  ) => [bigint, number, number];
  readonly fheuint48_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint48_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheuint48_encrypt_with_compressed_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheuint48_encrypt_with_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheuint48_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint48_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint48_serialize: (a: number) => [number, number, number, number];
  readonly fheuint4_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheuint4_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint4_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint4_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint4_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint4_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint4_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint4_serialize: (a: number) => [number, number, number, number];
  readonly fheuint512_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint512_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint512_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint512_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint512_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint512_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint512_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint512_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly fheuint56_decrypt: (
    a: number,
    b: number,
  ) => [bigint, number, number];
  readonly fheuint56_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint56_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheuint56_encrypt_with_compressed_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheuint56_encrypt_with_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheuint56_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint56_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint56_serialize: (a: number) => [number, number, number, number];
  readonly fheuint64_decrypt: (
    a: number,
    b: number,
  ) => [bigint, number, number];
  readonly fheuint64_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint64_encrypt_with_client_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheuint64_encrypt_with_compressed_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheuint64_encrypt_with_public_key: (
    a: bigint,
    b: number,
  ) => [number, number, number];
  readonly fheuint64_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint64_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint64_serialize: (a: number) => [number, number, number, number];
  readonly fheuint6_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheuint6_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint6_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint6_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint6_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint6_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint6_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint6_serialize: (a: number) => [number, number, number, number];
  readonly fheuint72_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint72_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint72_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint72_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint72_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint72_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint72_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint72_serialize: (a: number) => [number, number, number, number];
  readonly fheuint80_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint80_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint80_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint80_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint80_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint80_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint80_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint80_serialize: (a: number) => [number, number, number, number];
  readonly fheuint88_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint88_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint88_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint88_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint88_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint88_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint88_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint88_serialize: (a: number) => [number, number, number, number];
  readonly fheuint8_decrypt: (a: number, b: number) => [number, number, number];
  readonly fheuint8_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint8_encrypt_with_client_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint8_encrypt_with_compressed_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint8_encrypt_with_public_key: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint8_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint8_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint8_serialize: (a: number) => [number, number, number, number];
  readonly fheuint96_decrypt: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint96_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly fheuint96_encrypt_with_client_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint96_encrypt_with_compressed_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint96_encrypt_with_public_key: (
    a: any,
    b: number,
  ) => [number, number, number];
  readonly fheuint96_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly fheuint96_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly fheuint96_serialize: (a: number) => [number, number, number, number];
  readonly init_panic_hook: () => void;
  readonly provencompactciphertextlist_builder: (
    a: number,
  ) => [number, number, number];
  readonly provencompactciphertextlist_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly provencompactciphertextlist_expand_without_verification: (
    a: number,
  ) => [number, number, number];
  readonly provencompactciphertextlist_get_kind_of: (
    a: number,
    b: number,
  ) => number;
  readonly provencompactciphertextlist_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly provencompactciphertextlist_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly provencompactciphertextlist_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly provencompactciphertextlist_verify_and_expand: (
    a: number,
    b: number,
    c: number,
    d: number,
    e: number,
  ) => [number, number, number];
  readonly set_server_key: (a: number) => [number, number];
  readonly tfheclientkey_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly tfheclientkey_generate: (a: number) => [number, number, number];
  readonly tfheclientkey_generate_with_seed: (
    a: number,
    b: any,
  ) => [number, number, number];
  readonly tfheclientkey_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly tfheclientkey_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly tfheclientkey_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly tfhecompactpublickey_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly tfhecompactpublickey_new: (a: number) => [number, number, number];
  readonly tfhecompactpublickey_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly tfhecompactpublickey_safe_deserialize_conformant: (
    a: number,
    b: number,
    c: bigint,
    d: number,
  ) => [number, number, number];
  readonly tfhecompactpublickey_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly tfhecompactpublickey_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly tfhecompressedcompactpublickey_decompress: (
    a: number,
  ) => [number, number, number];
  readonly tfhecompressedcompactpublickey_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly tfhecompressedcompactpublickey_new: (
    a: number,
  ) => [number, number, number];
  readonly tfhecompressedcompactpublickey_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly tfhecompressedcompactpublickey_safe_deserialize_conformant: (
    a: number,
    b: number,
    c: bigint,
    d: number,
  ) => [number, number, number];
  readonly tfhecompressedcompactpublickey_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly tfhecompressedcompactpublickey_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly tfhecompressedpublickey_decompress: (
    a: number,
  ) => [number, number, number];
  readonly tfhecompressedpublickey_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly tfhecompressedpublickey_new: (a: number) => [number, number, number];
  readonly tfhecompressedpublickey_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly tfhecompressedpublickey_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly tfhecompressedpublickey_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly tfhecompressedserverkey_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly tfhecompressedserverkey_new: (a: number) => [number, number, number];
  readonly tfhecompressedserverkey_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly tfhecompressedserverkey_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly tfhecompressedserverkey_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly tfheconfigbuilder_build: (a: number) => number;
  readonly tfheconfigbuilder_default: () => number;
  readonly tfheconfigbuilder_use_custom_parameters: (
    a: number,
    b: number,
  ) => number;
  readonly tfheconfigbuilder_use_dedicated_compact_public_key_parameters: (
    a: number,
    b: number,
  ) => number;
  readonly tfheconfigbuilder_with_custom_parameters: (a: number) => number;
  readonly tfhepublickey_deserialize: (
    a: number,
    b: number,
  ) => [number, number, number];
  readonly tfhepublickey_new: (a: number) => [number, number, number];
  readonly tfhepublickey_safe_deserialize: (
    a: number,
    b: number,
    c: bigint,
  ) => [number, number, number];
  readonly tfhepublickey_safe_serialize: (
    a: number,
    b: bigint,
  ) => [number, number, number, number];
  readonly tfhepublickey_serialize: (
    a: number,
  ) => [number, number, number, number];
  readonly tfheserverkey_new: (a: number) => [number, number, number];
  readonly provencompactciphertextlist_len: (a: number) => number;
  readonly provencompactciphertextlist_is_empty: (a: number) => number;
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
  readonly __wbindgen_realloc: (
    a: number,
    b: number,
    c: number,
    d: number,
  ) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_thread_destroy: (
    a?: number,
    b?: number,
    c?: number,
  ) => void;
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
export function initSync(
  module:
    | {
        module: SyncInitInput;
        memory?: WebAssembly.Memory;
        thread_stack_size?: number;
      }
    | SyncInitInput,
  memory?: WebAssembly.Memory,
): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory, thread_stack_size?: number }} module_or_path - Passing `InitInput` directly is deprecated.
 * @param {WebAssembly.Memory} memory - Deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init(
  module_or_path?:
    | {
        module_or_path: InitInput | Promise<InitInput>;
        memory?: WebAssembly.Memory;
        thread_stack_size?: number;
      }
    | InitInput
    | Promise<InitInput>,
  memory?: WebAssembly.Memory,
): Promise<InitOutput>;

////////////////////////////////////////////////////////////////////////////////

export function getWasmInfo(): { name: string, version: string };
export function getTfheWorkers(): object[];
export function terminateWorkers(): Promise<unknown>;
export function setWorkerUrlConfig(parameters?: {
  readonly workerUrl?: URL | undefined;
  readonly logger?:
    | {
        debug: (message: string) => void;
        error: (message: string, cause: unknown) => void;
      }
    | undefined;
}): void;
