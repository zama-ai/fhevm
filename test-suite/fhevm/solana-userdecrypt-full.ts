// PURE-SDK Solana user-decrypt: the WHOLE round-trip in JS via a single `client.userDecrypt` call —
// ML-KEM keygen + the v3 ed25519 request AND in-SDK de-signcryption to cleartext (vendored Solana
// TKMS WASM), all owned by the action. No kms-core checkout.
//
// Replaces the full-vertical's `cargo test -p kms` user-decrypt leg.
//
// Env: UD_RELAYER_URL, UD_CONTRACTS_CHAIN_ID (dec u64), UD_HANDLE, UD_SECRET_KEY (32-byte ed25519
//   seed), UD_CONTEXT_ID, UD_ALLOWED_DOMAIN_KEYS (csv), UD_EXPECTED (dec u64); optional UD_NONCE,
//   UD_START_TIMESTAMP, UD_DURATION_SECONDS.
import { createFhevmDecryptClient, solanaSignerFromSecretKey, setFhevmRuntimeConfig } from '@fhevm/sdk/solana';
import { defineFhevmSolanaChain } from '@fhevm/sdk/chains';
import type { Bytes32Hex, BytesHex } from '@fhevm/sdk/types';

function reqEnv(name: string): string {
  const v = process.env[name];
  if (v === undefined || v === '') throw new Error(`missing env ${name}`);
  return v;
}
function hexToBytes(hex: string): Uint8Array {
  return Uint8Array.from(Buffer.from(hex.startsWith('0x') ? hex.slice(2) : hex, 'hex'));
}

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
const clearValues = await client.userDecrypt({
  handles: [handleHex as BytesHex],
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
if (!clearValues.length) throw new Error('no clear values returned');

const value = BigInt(clearValues[0].value as bigint | number | boolean);
const expected = BigInt(reqEnv('UD_EXPECTED'));
if (value !== expected) throw new Error(`user-decrypt cleartext ${value} != expected ${expected}`);
process.stdout.write(`PURE-SDK user-decrypt OK: cleartext=${value}\n`);
process.exit(0);
