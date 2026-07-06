// PURE-SDK Solana user-decrypt: the WHOLE round-trip in JS via a single `client.userDecrypt` call —
// ML-KEM keygen + the v3 ed25519 request AND in-SDK de-signcryption to cleartext (vendored Solana
// TKMS WASM), all owned by the action. No kms-core checkout.
//
// Replaces the full-vertical's `cargo test -p kms` user-decrypt leg.
//
// Env: UD_RELAYER_URL, UD_CONTRACTS_CHAIN_ID (dec u64), UD_HANDLE, UD_SECRET_KEY (32-byte ed25519
//   seed), UD_CONTEXT_ID, UD_ALLOWED_DOMAIN_KEYS (csv), UD_ACL_VALUE_KEY, UD_EXPECTED (dec u64);
//   optional UD_NONCE, UD_START_TIMESTAMP, UD_DURATION_SECONDS. UD_HISTORICAL=1 additionally
//   requires UD_MMR_* fields below.
import {
  createFhevmDecryptClient,
  solanaSignerFromSecretKey,
  setFhevmRuntimeConfig,
  type SolanaUserDecryptParameters,
} from '@fhevm/sdk/solana';
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
function hexCsvToBytes(csv: string): Uint8Array[] {
  return csv
    .split(',')
    .map((s) => s.trim())
    .filter((s) => s.length > 0)
    .map(hexToBytes);
}

type UserDecryptMmrProof = NonNullable<SolanaUserDecryptParameters['mmrProof']>;

function historicalProofFromEnv(): UserDecryptMmrProof | undefined {
  if (process.env.UD_HISTORICAL !== '1') return undefined;
  return {
    encryptedValueAccount: hexToBytes(reqEnv('UD_MMR_ENCRYPTED_VALUE_ACCOUNT')),
    aclValueKey: hexToBytes(reqEnv('UD_ACL_VALUE_KEY')),
    peaks: hexCsvToBytes(reqEnv('UD_MMR_PEAKS')),
    leafCount: BigInt(reqEnv('UD_MMR_LEAF_COUNT')),
    proofSlot: BigInt(reqEnv('UD_MMR_PROOF_SLOT')),
    proof: {
      leafIndex: BigInt(reqEnv('UD_MMR_LEAF_INDEX')),
      siblings: hexCsvToBytes(process.env.UD_MMR_SIBLINGS ?? ''),
    },
    mmrProofBytes: hexToBytes(reqEnv('UD_MMR_PROOF_BYTES')),
    mode: 'historical',
    subject: hexToBytes(reqEnv('UD_MMR_SUBJECT')),
  };
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
const aclValueKey = hexToBytes(reqEnv('UD_ACL_VALUE_KEY'));
const mmrProof = historicalProofFromEnv();

const client = createFhevmDecryptClient({ signer, chain });
const parameters: SolanaUserDecryptParameters = {
  handles: [handleHex as BytesHex],
  allowedAclDomainKeys,
  contextId: hexToBytes(reqEnv('UD_CONTEXT_ID')),
  aclValueKey,
  ...(mmrProof ? { mmrProof } : {}),
  ...(process.env.UD_NONCE ? { nonce: hexToBytes(process.env.UD_NONCE) } : {}),
  ...(process.env.UD_START_TIMESTAMP && process.env.UD_DURATION_SECONDS
    ? {
        validity: {
          startTimestamp: BigInt(process.env.UD_START_TIMESTAMP),
          durationSeconds: BigInt(process.env.UD_DURATION_SECONDS),
        },
      }
    : {}),
};
const clearValues = await client.userDecrypt(parameters);
if (!clearValues.length) throw new Error('no clear values returned');

const value = BigInt(clearValues[0].value as bigint | number | boolean);
const expected = BigInt(reqEnv('UD_EXPECTED'));
if (value !== expected) throw new Error(`user-decrypt cleartext ${value} != expected ${expected}`);
process.stdout.write(`PURE-SDK user-decrypt OK: cleartext=${value}\n`);
process.exit(0);
