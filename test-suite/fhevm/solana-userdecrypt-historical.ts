// PURE-SDK historical Solana user-decrypt through the permit path. The historical evidence — the
// account read and the verified access proof — is resolved by the SDK's own evidence source
// against the RPC and the proof service; nothing is supplied by hand anymore, which is the point:
// this worker exercises exactly the path a real client runs when its handle was replaced by an
// update mid-flight.
//
// Env: UD_RELAYER_URL, UD_RPC_URL, UD_PROOF_SERVICE_URL, UD_CONTRACTS_CHAIN_ID, UD_HANDLE,
// UD_SECRET_KEY, UD_CONTEXT_ID, UD_ALLOWED_DOMAIN_KEYS, UD_ACL_VALUE_KEY,
// UD_VERIFYING_PROGRAM_ID, UD_KMS_SIGNERS, UD_GATEWAY_CHAIN_ID, UD_GATEWAY_DECRYPTION_CONTRACT,
// UD_EXPECTED; optional: UD_SUBJECT (defaults to the signer), UD_EPOCH_ID (zero), UD_FHE_PARAMETER
// ("test"), UD_DURATION_SECONDS ("3600").
import {
  createFhevmDecryptClient,
  defineFhevmSolanaChain,
  setFhevmRuntimeConfig,
  solanaPermitWalletFromSecretKey,
} from '@fhevm/sdk/solana';
import type { Bytes32Hex } from '@fhevm/sdk/types';

function reqEnv(name: string): string {
  const value = process.env[name];
  if (value === undefined || value === '') throw new Error(`missing env ${name}`);
  return value;
}

function hexToBytes(hex: string): Uint8Array {
  return Uint8Array.from(Buffer.from(hex.startsWith('0x') ? hex.slice(2) : hex, 'hex'));
}

const allowedAclDomainKeys = reqEnv('UD_ALLOWED_DOMAIN_KEYS')
  .split(',')
  .map((value) => value.trim())
  .filter(Boolean) as Bytes32Hex[];
const chain = defineFhevmSolanaChain({
  id: BigInt(reqEnv('UD_CONTRACTS_CHAIN_ID')),
  fhevm: {
    relayerUrl: reqEnv('UD_RELAYER_URL'),
    acl: { domainKeys: allowedAclDomainKeys },
    rpcUrl: reqEnv('UD_RPC_URL'),
    proofServiceUrl: reqEnv('UD_PROOF_SERVICE_URL'),
    verifyingProgramId: reqEnv('UD_VERIFYING_PROGRAM_ID') as Bytes32Hex,
  },
});
setFhevmRuntimeConfig({ auth: { type: 'ApiKeyHeader', value: process.env.ZAMA_FHEVM_API_KEY ?? 'local' } });

// Party ids follow the registry order — the same assumption the EVM SDK path makes.
const kmsSigners = reqEnv('UD_KMS_SIGNERS')
  .split(',')
  .map((value) => value.trim())
  .filter(Boolean)
  .map((address, index) => ({ partyId: index + 1, address }));

const client = createFhevmDecryptClient({
  chain,
  trust: {
    kmsSigners,
    kmsContextId: reqEnv('UD_CONTEXT_ID') as Bytes32Hex,
    kmsEpochId: (process.env.UD_EPOCH_ID ?? `0x${'0'.repeat(64)}`) as Bytes32Hex,
    fheParameter: process.env.UD_FHE_PARAMETER ?? 'test',
    gatewayEip712Domain: {
      name: 'Decryption',
      version: '1',
      chainId: BigInt(reqEnv('UD_GATEWAY_CHAIN_ID')),
      verifyingContract: reqEnv('UD_GATEWAY_DECRYPTION_CONTRACT'),
    },
  },
});

const wallet = solanaPermitWalletFromSecretKey(hexToBytes(reqEnv('UD_SECRET_KEY')));
const session = await client.signPermit({
  wallet,
  durationSeconds: BigInt(process.env.UD_DURATION_SECONDS ?? '3600'),
});

const clearValues = await client.userDecrypt({
  session,
  entries: [
    {
      handle: hexToBytes(reqEnv('UD_HANDLE')),
      encryptedValueId: hexToBytes(reqEnv('UD_ACL_VALUE_KEY')),
      ...(process.env.UD_SUBJECT ? { subject: hexToBytes(process.env.UD_SUBJECT) } : {}),
    },
  ],
});
if (clearValues.length !== 1) throw new Error(`expected one clear value, got ${clearValues.length}`);

const value = BigInt(clearValues[0].value as bigint | number | boolean);
const expected = BigInt(reqEnv('UD_EXPECTED'));
if (value !== expected) throw new Error(`user-decrypt cleartext ${value} != expected ${expected}`);
process.stdout.write(`PURE-SDK historical user-decrypt OK: cleartext=${value}\n`);
