import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

export type EnvRecord = Record<string, string>;

export interface SolanaTestContext {
  addresses: EnvRecord;
  localnetEnv: EnvRecord;
  testSuiteEnv: EnvRecord;
  addressesEnvPath: string;
  chainId: number;
}

// Pre-generated Solana keypairs committed to the repo for deterministic test accounts.
//
// anchor-authority: the default payer/signer for all local-cli commands and the Anchor
// wallet used during deployment. It controls the state PDAs for the host program.
//
// confidential-token-recipient: the recipient account in token transfer tests. Uses a
// separate identity from the deployer to exercise ACL separation (approve + transferFrom).
//
// Both are 64-byte Solana keypairs: first 32 bytes = private key, last 32 bytes = public key.
// Written to temp files at runtime so local-cli can read them via --payer-keypair / --recipient-keypair.

const ANCHOR_AUTHORITY_KEYPAIR_BYTES = Uint8Array.from([
  16, 103, 179, 215, 168, 177, 170, 220, 224, 48, 117, 60, 156, 248, 135, 184,
  115, 82, 135, 107, 13, 189, 182, 195, 42, 157, 189, 162, 191, 192, 31, 77,
  201, 45, 229, 75, 250, 3, 199, 45, 29, 142, 207, 194, 127, 223, 116, 140,
  76, 197, 84, 135, 105, 118, 102, 169, 219, 68, 42, 221, 96, 98, 252, 1,
]);

const CONFIDENTIAL_TOKEN_RECIPIENT_KEYPAIR_BYTES = Uint8Array.from([
  194, 73, 56, 51, 31, 45, 82, 184, 78, 183, 96, 241, 245, 139, 102, 9,
  241, 89, 162, 92, 142, 69, 203, 2, 34, 10, 253, 16, 113, 39, 59, 255,
  70, 244, 217, 44, 130, 168, 209, 122, 92, 158, 118, 30, 37, 214, 174, 226,
  110, 168, 203, 52, 19, 115, 57, 171, 186, 60, 71, 142, 206, 2, 191, 125,
]);

/** Writes a keypair byte array to a temp file and returns the path. */
function writeTempKeypairFile(name: string, bytes: Uint8Array): string {
  const filePath = path.join(os.tmpdir(), `solana-e2e-${name}-${process.pid}.json`);
  fs.writeFileSync(filePath, JSON.stringify(Array.from(bytes)));
  return filePath;
}

/** Paths to temp keypair files written at context setup time. */
export let anchorAuthorityKeypairPath: string;
export let tokenRecipientKeypairPath: string;

const envSnapshot = (): EnvRecord =>
  Object.fromEntries(
    Object.entries(process.env).filter((entry): entry is [string, string] => typeof entry[1] === 'string'),
  );

export async function setupSolanaContext(): Promise<SolanaTestContext> {
  const addressesEnvPath = process.env.FHEVM_SOLANA_ADDRESSES_ENV;
  if (!addressesEnvPath) {
    throw new Error('FHEVM_SOLANA_ADDRESSES_ENV env var is required');
  }
  if (!fs.existsSync(addressesEnvPath)) {
    throw new Error(`Solana addresses env file not found: ${addressesEnvPath}`);
  }

  anchorAuthorityKeypairPath = writeTempKeypairFile('anchor-authority', ANCHOR_AUTHORITY_KEYPAIR_BYTES);
  tokenRecipientKeypairPath = writeTempKeypairFile('token-recipient', CONFIDENTIAL_TOKEN_RECIPIENT_KEYPAIR_BYTES);

  const mergedEnv = envSnapshot();
  const chainId = Number(mergedEnv.SOLANA_HOST_CHAIN_ID);
  if (!Number.isFinite(chainId)) {
    throw new Error('invalid SOLANA_HOST_CHAIN_ID in process.env');
  }

  return {
    addresses: mergedEnv,
    localnetEnv: mergedEnv,
    testSuiteEnv: mergedEnv,
    addressesEnvPath,
    chainId,
  };
}
