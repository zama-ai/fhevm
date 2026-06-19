// PURE-SDK Solana user-decrypt: the WHOLE round-trip in JS — ML-KEM keygen + the v3 ed25519 request
// (shares) via @fhevm/sdk/solana, AND de-signcryption to cleartext via the SDK's in-SDK
// deSigncryptSolanaUserDecrypt (vendored Solana TKMS WASM). No kms-core checkout.
//
// Replaces the full-vertical's `cargo test -p kms` user-decrypt leg.
//
// Env: UD_RELAYER_URL, UD_CONTRACTS_CHAIN_ID (dec u64), UD_HANDLE, UD_SECRET_KEY (32-byte ed25519
//   seed), UD_CONTEXT_ID, UD_ALLOWED_DOMAIN_KEYS (csv), UD_EXPECTED (dec u64); optional UD_NONCE,
//   UD_START_TIMESTAMP, UD_DURATION_SECONDS.
import { createFhevmDecryptClient, solanaSignerFromSecretKey, setFhevmRuntimeConfig } from '@fhevm/sdk/solana';
import { defineFhevmSolanaChain } from '@fhevm/sdk/chains';
import type { Bytes32Hex, BytesHex } from '@fhevm/sdk/types';
import {
  generateSolanaTransportKeyPair,
  deSigncryptSolanaUserDecrypt,
} from '../../sdk/js-sdk/src/solana/deSigncrypt.js';

function reqEnv(name: string): string {
  const v = process.env[name];
  if (v === undefined || v === '') throw new Error(`missing env ${name}`);
  return v;
}
function hexToBytes(hex: string): Uint8Array {
  return Uint8Array.from(Buffer.from(hex.startsWith('0x') ? hex.slice(2) : hex, 'hex'));
}

const keyPair = await generateSolanaTransportKeyPair();
const transportPublicKey = ('0x' + Buffer.from(keyPair.publicKeyBytes).toString('hex')) as BytesHex;

const allowedAclDomainKeys = reqEnv('UD_ALLOWED_DOMAIN_KEYS')
  .split(',')
  .map((s) => s.trim())
  .filter((s) => s.length > 0) as Bytes32Hex[];

const chainId = BigInt(reqEnv('UD_CONTRACTS_CHAIN_ID'));
const chain = defineFhevmSolanaChain({
  id: chainId,
  fhevm: { relayerUrl: reqEnv('UD_RELAYER_URL'), acl: { domainKeys: allowedAclDomainKeys } },
});
setFhevmRuntimeConfig({ auth: { type: 'ApiKeyHeader', value: process.env.ZAMA_FHEVM_API_KEY ?? 'local' } });

const signer = solanaSignerFromSecretKey(hexToBytes(reqEnv('UD_SECRET_KEY')));
const handleHex = reqEnv('UD_HANDLE');

const client = createFhevmDecryptClient({ signer, chain });
const { shares } = await client.userDecrypt({
  handles: [handleHex as BytesHex],
  transportPublicKey,
  allowedAclDomainKeys,
  contextId: hexToBytes(reqEnv('UD_CONTEXT_ID')),
  ...(process.env.UD_NONCE ? { nonce: hexToBytes(process.env.UD_NONCE) } : {}),
  ...(process.env.UD_START_TIMESTAMP && process.env.UD_DURATION_SECONDS
    ? {
        validity: {
          startTimestamp: BigInt(process.env.UD_START_TIMESTAMP),
          durationSeconds: BigInt(process.env.UD_DURATION_SECONDS),
        },
      }
    : {}),
});

const plaintexts = await deSigncryptSolanaUserDecrypt({
  keyPair,
  shares,
  handles: [handleHex],
  solanaUserPubkey: signer.publicKey,
  hostChainId: chainId,
});
if (!plaintexts.length) throw new Error('no plaintexts returned');

let value = 0n;
for (const b of plaintexts[0].bytes) value = (value << 8n) | BigInt(b);

const expected = BigInt(reqEnv('UD_EXPECTED'));
if (value !== expected) throw new Error(`user-decrypt cleartext ${value} != expected ${expected}`);
process.stdout.write(`PURE-SDK user-decrypt OK: cleartext=${value}\n`);
process.exit(0);
