import type { Bytes, UintNumber } from './primitives.js';
import type { Prettify } from './utils.js';

/**
 * Configuration for fetching a TFHE Public Key Encryption (PKE) Common Reference
 * String (CRS) from a remote URL.
 *
 * Typically obtained from the <relayer-url>/keyurl response, which provides
 * the URLs for fetching the data.
 */
export type FheEncryptionCrsSource = {
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
export type FheEncryptionPublicKeySource = {
  /** Unique identifier for the public key provided by the relayer */
  readonly id: string;
  /** URL from which to fetch the public key bytes */
  readonly url: string;
};

/**
 * URL configuration for fetching TFHE PKE (Public Key Encryption) parameters.
 */
export type FheEncryptionKeySource = {
  /** URL configuration for the TFHE compact public key */
  readonly publicKeySource: FheEncryptionPublicKeySource;
  /** URL configuration for the PKE CRS (Common Reference String) */
  readonly crsSource: FheEncryptionCrsSource;
  /** Metadata about the key origin (relayer URL, chain ID) */
  readonly metadata: FheEncryptionKeyMetadata;
};

////////////////////////////////////////////////////////////////////////////////

export declare const FheEncryptionPublicKeyBrand: unique symbol;
export declare const FheEncryptionCrsBrand: unique symbol;

////////////////////////////////////////////////////////////////////////////////

export type FheEncryptionPublicKey = {
  readonly [FheEncryptionPublicKeyBrand]: never;
  readonly id: string;
};

export type FheEncryptionPublicKeyBytes = Prettify<
  Omit<FheEncryptionPublicKey, typeof FheEncryptionPublicKeyBrand> & {
    readonly bytes: Bytes;
  }
>;

////////////////////////////////////////////////////////////////////////////////

// Generated via a KMS ceremony
// Defined by TFHE-rs
export type FheEncryptionCrs = {
  readonly [FheEncryptionCrsBrand]: never;
  readonly id: string;
  readonly capacity: UintNumber;
};

export type FheEncryptionCrsBytes = Prettify<
  Omit<FheEncryptionCrs, typeof FheEncryptionCrsBrand> & {
    readonly bytes: Bytes;
  }
>;

////////////////////////////////////////////////////////////////////////////////

export type FheEncryptionKeyMetadata = {
  readonly relayerUrl: string;
  readonly chainId: number;
};

export type FheEncryptionKeyWasm = {
  readonly publicKey: FheEncryptionPublicKey;
  readonly crs: FheEncryptionCrs;
  readonly metadata: FheEncryptionKeyMetadata;
};

////////////////////////////////////////////////////////////////////////////////

export type FheEncryptionKeyBytes = {
  readonly publicKeyBytes: FheEncryptionPublicKeyBytes;
  readonly crsBytes: FheEncryptionCrsBytes;
  readonly metadata: FheEncryptionKeyMetadata;
};

////////////////////////////////////////////////////////////////////////////////

export type FheEncryptionKeyUrls = {
  readonly publicKeyUrl: string;
  readonly crsUrl: string;
  readonly metadata: FheEncryptionKeyMetadata;
};
