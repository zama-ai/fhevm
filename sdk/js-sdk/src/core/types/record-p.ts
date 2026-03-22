import type {
  Uint,
  ChecksummedAddress,
  Uint256,
  Address,
} from "./primitives.js";

/**
 * Generic helper type that creates a record with a specific property of type T.
 */
export type RecordWithPropertyType<K extends string, T> = Record<
  string,
  unknown
> &
  Record<K, NonNullable<T>>;

export type RecordNonNullablePropertyType<K extends string> =
  RecordWithPropertyType<K, unknown>;

export type RecordArrayPropertyType<K extends string> = RecordWithPropertyType<
  K,
  unknown[]
>;

export type RecordStringPropertyType<K extends string> = RecordWithPropertyType<
  K,
  string
>;

export type RecordStringArrayPropertyType<K extends string> =
  RecordWithPropertyType<K, string[]>;

export type RecordBooleanPropertyType<K extends string> =
  RecordWithPropertyType<K, boolean>;

export type RecordUintPropertyType<K extends string> = RecordWithPropertyType<
  K,
  Uint
>;
export type RecordUint256PropertyType<K extends string> =
  RecordWithPropertyType<K, Uint256>;

export type RecordUint8ArrayPropertyType<K extends string> =
  RecordWithPropertyType<K, Uint8Array>;

export type RecordChecksummedAddressPropertyType<K extends string> =
  RecordWithPropertyType<K, ChecksummedAddress>;

export type RecordChecksummedAddressArrayPropertyType<K extends string> =
  RecordWithPropertyType<K, ChecksummedAddress[]>;

export type RecordAddressPropertyType<K extends string> =
  RecordWithPropertyType<K, Address>;

export type RecordAddressArrayPropertyType<K extends string> =
  RecordWithPropertyType<K, Address[]>;
