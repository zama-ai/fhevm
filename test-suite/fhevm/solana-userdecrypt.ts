// Solana V2 user-decrypt launcher — runs the request half through the PUBLIC `@fhevm/sdk/solana`
// client: builds the ed25519-signed v3 `AttestedUserDecryptRequest`, POSTs it to the relayer's
// `/v3/user-decrypt` seam, and returns the aggregated KMS signcrypted shares as JSON on stdout.
//
// De-signcryption to cleartext is done by the Rust kms-core caller (the SDK's TKMS WASM does not yet
// expose the Solana keccak-link path); that caller owns the ML-KEM transport key pair and passes its
// PUBLIC key in via UD_PUBLIC_KEY.
//
// Inputs via env (hex unless noted):
//   UD_RELAYER_URL          relayer base URL (e.g. http://127.0.0.1:3000)
//   UD_CONTRACTS_CHAIN_ID   decimal u64 — the Solana host chain id embedded in the handle
//   UD_PUBLIC_KEY           ML-KEM re-encryption public key (0x-hex), sealed-to + bound in the preimage
//   UD_HANDLE               single 32-byte ciphertext handle (0x-hex)
//   UD_SECRET_KEY           user's 32-byte ed25519 seed (0x-hex)
//   UD_CONTEXT_ID           32-byte big-endian context id (0x-hex)
//   UD_NONCE                32-byte anti-replay nonce (0x-hex)
//   UD_ALLOWED_DOMAIN_KEYS  comma-separated 32-byte ACL domain keys (0x-hex)
//   UD_START_TIMESTAMP      decimal u64 (unix seconds)
//   UD_DURATION_SECONDS     decimal u64
//
// Output: JSON `{ "shares": [{ signature, payload, extraData }, ...] }` on stdout.

import { createFhevmDecryptClient, solanaSignerFromSecretKey, setFhevmRuntimeConfig } from '@fhevm/sdk/solana';
import { defineFhevmSolanaChain } from '@fhevm/sdk/chains';
import type { Bytes32Hex, BytesHex } from '@fhevm/sdk/types';

function reqEnv(name: string): string {
  const v = process.env[name];
  if (v === undefined || v === '') throw new Error(`missing env ${name}`);
  return v;
}

function hexToBytes(hex: string): Uint8Array {
  const s = hex.startsWith('0x') ? hex.slice(2) : hex;
  if (s.length % 2 !== 0) throw new Error(`odd-length hex: ${hex}`);
  return Uint8Array.from(Buffer.from(s, 'hex'));
}

const allowedAclDomainKeys = reqEnv('UD_ALLOWED_DOMAIN_KEYS')
  .split(',')
  .map((s) => s.trim())
  .filter((s) => s.length > 0) as Bytes32Hex[];

const chain = defineFhevmSolanaChain({
  id: BigInt(reqEnv('UD_CONTRACTS_CHAIN_ID')),
  fhevm: {
    relayerUrl: reqEnv('UD_RELAYER_URL'),
    acl: { domainKeys: allowedAclDomainKeys },
  },
});

setFhevmRuntimeConfig({
  auth: { type: 'ApiKeyHeader', value: process.env.ZAMA_FHEVM_API_KEY ?? 'local' },
});

const signer = solanaSignerFromSecretKey(hexToBytes(reqEnv('UD_SECRET_KEY')));
const client = createFhevmDecryptClient({ signer, chain });

const result = await client.userDecrypt({
  handles: [reqEnv('UD_HANDLE') as BytesHex],
  transportPublicKey: reqEnv('UD_PUBLIC_KEY') as BytesHex,
  allowedAclDomainKeys,
  contextId: hexToBytes(reqEnv('UD_CONTEXT_ID')),
  nonce: hexToBytes(reqEnv('UD_NONCE')),
  validity: {
    startTimestamp: BigInt(reqEnv('UD_START_TIMESTAMP')),
    durationSeconds: BigInt(reqEnv('UD_DURATION_SECONDS')),
  },
});

process.stdout.write(JSON.stringify({ shares: result.shares }) + '\n');
process.exit(0);
