import { secp256k1 } from "@noble/curves/secp256k1";
import { keccak_256 } from "@noble/hashes/sha3";
import { bytesToHex } from "@noble/hashes/utils";
import { HDKey } from "@scure/bip32";
import { mnemonicToSeedSync, validateMnemonic } from "@scure/bip39";
import { wordlist } from "@scure/bip39/wordlists/english";
import {
  CUSTODIAN_ENCRYPTION_KEYS,
  HD_INDICES,
  MAX_COPROCESSORS,
  type CoprocessorKeySet,
  type CustodianKeySet,
  type DerivedKeys,
  type KeyPair,
  type KmsNodeKeySet,
} from "./model";

const DERIVATION_PATH_PREFIX = "m/44'/60'/0'/0";

export function mnemonicToSeed(mnemonic: string): Uint8Array {
  if (!validateMnemonic(mnemonic, wordlist)) {
    throw new Error("invalid mnemonic");
  }
  return mnemonicToSeedSync(mnemonic);
}

export function checksumAddress(address: string): string {
  const lowerHex = address.toLowerCase().replace(/^0x/, "");
  const hash = bytesToHex(keccak_256(new TextEncoder().encode(lowerHex)));
  let output = "0x";

  for (let i = 0; i < lowerHex.length; i += 1) {
    const nibble = Number.parseInt(hash[i] ?? "0", 16);
    output += nibble >= 8 ? lowerHex[i]?.toUpperCase() : lowerHex[i];
  }

  return output;
}

export function publicKeyToAddress(publicKey: Uint8Array): string {
  const normalized = publicKey.length === 65 ? publicKey.slice(1) : publicKey;
  const hash = keccak_256(normalized);
  const address = bytesToHex(hash.slice(-20));
  return checksumAddress(`0x${address}`);
}

export function deriveAccount(seed: Uint8Array, index: number): KeyPair {
  const hd = HDKey.fromMasterSeed(seed);
  const derived = hd.derive(`${DERIVATION_PATH_PREFIX}/${index}`);
  if (!derived.privateKey) {
    throw new Error(`unable to derive private key at index ${index}`);
  }

  const privateKey = `0x${bytesToHex(derived.privateKey)}`;
  const publicKey = secp256k1.getPublicKey(derived.privateKey, false);
  const address = publicKeyToAddress(publicKey);

  return { privateKey, address };
}

function deriveCoprocessorKeys(seed: Uint8Array, numCoprocessors: number): CoprocessorKeySet[] {
  if (numCoprocessors > MAX_COPROCESSORS) {
    throw new Error(`numCoprocessors must be <= ${MAX_COPROCESSORS}`);
  }

  return HD_INDICES.coprocessorTxSender.slice(0, numCoprocessors).map((index) => {
    const txSender = deriveAccount(seed, index);
    return {
      txSender,
      signer: txSender,
      s3BucketUrl: "s3://ct128",
    };
  });
}

function deriveKmsNodeKeys(seed: Uint8Array, numKmsNodes: number): KmsNodeKeySet[] {
  return Array.from({ length: numKmsNodes }, (_, i) => {
    const txIndex = HD_INDICES.kmsTxSender[0] + i * 2;
    const signerIndex = txIndex + 1;
    return {
      txSender: deriveAccount(seed, txIndex),
      signer: deriveAccount(seed, signerIndex),
      ipAddress: `127.0.0.${i + 1}`,
      storageUrl: "s3://kms-public",
    };
  });
}

function deriveCustodianKeys(seed: Uint8Array): CustodianKeySet[] {
  return HD_INDICES.custodians.map((index, i) => ({
    txSender: deriveAccount(seed, index),
    signer: deriveAccount(seed, index + 1),
    encryptionKey: CUSTODIAN_ENCRYPTION_KEYS[i] ?? "",
  }));
}

export function deriveAllKeys(
  mnemonic: string,
  numCoprocessors: number,
  numKmsNodes: number,
): DerivedKeys {
  const seed = mnemonicToSeed(mnemonic);
  const pausers = HD_INDICES.pausers.map((index) => deriveAccount(seed, index));

  return {
    deployer: deriveAccount(seed, HD_INDICES.deployer),
    newOwner: deriveAccount(seed, HD_INDICES.newOwner),
    txSender: deriveAccount(seed, HD_INDICES.txSender),
    coprocessors: deriveCoprocessorKeys(seed, numCoprocessors),
    kmsNodes: deriveKmsNodeKeys(seed, numKmsNodes),
    custodians: deriveCustodianKeys(seed),
    pausers,
  };
}
