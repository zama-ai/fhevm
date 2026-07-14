// PURE-SDK historical Solana user-decrypt. The live MMR proof is supplied explicitly; the public
// SDK owns request signing, relayer retries, proof verification, and de-signcryption.
//
// Env: UD_RELAYER_URL, UD_CONTRACTS_CHAIN_ID, UD_HANDLE, UD_SECRET_KEY, UD_CONTEXT_ID,
// UD_ALLOWED_DOMAIN_KEYS, UD_ACL_VALUE_KEY, UD_EXPECTED, and every UD_MMR_* field below.
import {
  createFhevmDecryptClient,
  defineFhevmSolanaChain,
  solanaSignerFromSecretKey,
  setFhevmRuntimeConfig,
  type SolanaUserDecryptParameters,
} from '@fhevm/sdk/solana';
import type { Bytes32Hex, BytesHex } from '@fhevm/sdk/types';

function reqEnv(name: string): string {
  const value = process.env[name];
  if (value === undefined || value === '') throw new Error(`missing env ${name}`);
  return value;
}

function hexToBytes(hex: string): Uint8Array {
  return Uint8Array.from(Buffer.from(hex.startsWith('0x') ? hex.slice(2) : hex, 'hex'));
}

function hexCsvToBytes(csv: string): Uint8Array[] {
  return csv
    .split(',')
    .map((value) => value.trim())
    .filter(Boolean)
    .map(hexToBytes);
}

const allowedAclDomainKeys = reqEnv('UD_ALLOWED_DOMAIN_KEYS')
  .split(',')
  .map((value) => value.trim())
  .filter(Boolean) as Bytes32Hex[];
const chain = defineFhevmSolanaChain({
  id: BigInt(reqEnv('UD_CONTRACTS_CHAIN_ID')),
  fhevm: { relayerUrl: reqEnv('UD_RELAYER_URL'), acl: { domainKeys: allowedAclDomainKeys } },
});
setFhevmRuntimeConfig({ auth: { type: 'ApiKeyHeader', value: process.env.ZAMA_FHEVM_API_KEY ?? 'local' } });

const parameters: SolanaUserDecryptParameters = {
  handles: [reqEnv('UD_HANDLE') as BytesHex],
  allowedAclDomainKeys,
  contextId: hexToBytes(reqEnv('UD_CONTEXT_ID')),
  aclValueKey: hexToBytes(reqEnv('UD_ACL_VALUE_KEY')),
  mmrProof: {
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
  },
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

const signer = solanaSignerFromSecretKey(hexToBytes(reqEnv('UD_SECRET_KEY')));
const clearValues = await createFhevmDecryptClient({ signer, chain }).userDecrypt(parameters);
if (clearValues.length !== 1) throw new Error(`expected one clear value, got ${clearValues.length}`);

const value = BigInt(clearValues[0].value as bigint | number | boolean);
const expected = BigInt(reqEnv('UD_EXPECTED'));
if (value !== expected) throw new Error(`user-decrypt cleartext ${value} != expected ${expected}`);
process.stdout.write(`PURE-SDK historical user-decrypt OK: cleartext=${value}\n`);
