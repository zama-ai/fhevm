import type { Bytes, BytesHex, UintNumber } from "./primitives.js";

/**
 * Configuration for fetching a TFHE Public Key Encryption (PKE) Common Reference
 * String (CRS) from a remote URL.
 *
 * Typically obtained from the <relayer-url>/keyurl response, which provides
 * the URLs for fetching the data.
 */
export type GlobalFheCrsSource = {
  /** Unique identifier for the CRS provided by the relayer */
  readonly id: string;
  /** URL from which to fetch the CRS bytes */
  readonly url: string;
  /** The CRS capacity (always 2048 in the current configuration). */
  readonly capacity: number;
};

/**
 * Configuration for fetching a TFHE public key from a remote URL.
 *
 * Typically obtained from the <relayer-url>/keyurl response, which provides
 * the URLs for fetching the data.
 */
export type GlobalFhePublicKeySource = {
  /** Unique identifier for the public key provided by the relayer */
  readonly id: string;
  /** URL from which to fetch the public key bytes */
  readonly url: string;
};

/**
 * URL configuration for fetching TFHE PKE (Public Key Encryption) parameters.
 */
export type GlobalFhePkeParamsSource = {
  /** URL configuration for the TFHE compact public key */
  readonly publicKeySource: GlobalFhePublicKeySource;
  /** URL configuration for the PKE CRS (Common Reference String) */
  readonly crsSource: GlobalFheCrsSource;
};

////////////////////////////////////////////////////////////////////////////////

export declare const GlobalFhePublicKeyBrand: unique symbol;
export declare const GlobalFheCrsBrand: unique symbol;

////////////////////////////////////////////////////////////////////////////////

export type GlobalFhePublicKey = {
  readonly [GlobalFhePublicKeyBrand]: never;
  readonly id: string;
};

export type GlobalFhePublicKeyBytes = GlobalFhePublicKey & {
  readonly bytes: Bytes;
};

export type GlobalFhePublicKeyBytesHex = GlobalFhePublicKey & {
  readonly bytesHex: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////

// Generated via a KMS ceremony
// Defined by TFHE-rs
export type GlobalFheCrs = {
  readonly [GlobalFheCrsBrand]: never;
  readonly id: string;
  readonly capacity: UintNumber;
};

export type GlobalFheCrsBytes = GlobalFheCrs & {
  readonly bytes: Bytes;
};

export type GlobalFheCrsBytesHex = GlobalFheCrs & {
  readonly bytesHex: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////

export type GlobalFhePkeParams = {
  readonly publicKey: GlobalFhePublicKey;
  readonly crs: GlobalFheCrs;
};

////////////////////////////////////////////////////////////////////////////////

export type GlobalFhePkeParamsBytes = {
  readonly publicKeyBytes: GlobalFhePublicKeyBytes;
  readonly crsBytes: GlobalFheCrsBytes;
};

////////////////////////////////////////////////////////////////////////////////

export type GlobalFhePkeParamsBytesHex = {
  readonly publicKeyBytesHex: GlobalFhePublicKeyBytesHex;
  readonly crsBytesHex: GlobalFheCrsBytesHex;
};

////////////////////////////////////////////////////////////////////////////////

export type GlobalFhePkeParamsUrls = {
  readonly publicKeyUrl: string;
  readonly crsUrl: string;
};
