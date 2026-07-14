// One-shot Node worker for the real Solana SDK transfer seam. The proof object stays in memory
// from build -> submit -> transfer; this process is never retried after submission ambiguity.
import fs from 'node:fs/promises';

import { defineFhevmSolanaChain } from '@fhevm/sdk/chains';
import { createFhevmEncryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/solana';
import type { Bytes32Hex } from '@fhevm/sdk/types';
import {
  address,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  getAddressEncoder,
  getProgramDerivedAddress,
} from '@solana/kit';

const HOST_PROGRAM = address('6AtbvED1rfX68aCT1tYgU1aeu4kFksPDxZG9gtB1Fgtu');
const HOST_CONFIG_SEED = new TextEncoder().encode('host-config');

function required(name: string): string {
  const value = process.env[name];
  if (!value) throw new Error(`missing env ${name}`);
  return value;
}

const bytes32 = (value: string): Bytes32Hex => {
  if (!/^0x[0-9a-f]{64}$/i.test(value)) throw new Error(`expected bytes32 hex, got ${value}`);
  return value as Bytes32Hex;
};

const addressHex = (value: string): Bytes32Hex =>
  `0x${Buffer.from(getAddressEncoder().encode(address(value))).toString('hex')}` as Bytes32Hex;

const _fetch = globalThis.fetch;
globalThis.fetch = ((url: string | URL | Request, options?: RequestInit) =>
  _fetch(typeof url === 'string' ? url.replace('//minio:9000', '//127.0.0.1:9000') : url, options)) as typeof fetch;

const keypairBytes = Uint8Array.from(JSON.parse(await fs.readFile(required('TRANSFER_OWNER_KEYPAIR'), 'utf8')) as number[]);
const owner = await createKeyPairSignerFromBytes(keypairBytes);
if (owner.address !== required('TRANSFER_OWNER')) throw new Error('transfer signer does not match the probed Alice owner');
if (owner.address === required('TRANSFER_RECIPIENT')) throw new Error('Alice and Bob must use distinct owners');

const chainId = BigInt(required('TRANSFER_CHAIN_ID'));
if ((chainId & (1n << 63n)) === 0n) throw new Error('transfer chain id is not a Solana high-bit chain id');
const aclProgramAddress = bytes32(required('TRANSFER_ACL_PROGRAM'));
if (addressHex(HOST_PROGRAM) !== aclProgramAddress) throw new Error('configured ACL program is not the fixed Zama host program');
const chain = defineFhevmSolanaChain({
  id: chainId,
  fhevm: { relayerUrl: required('TRANSFER_RELAYER_URL'), acl: { domainKeys: [addressHex(required('TRANSFER_MINT'))] } },
});
setFhevmRuntimeConfig({ auth: { type: 'ApiKeyHeader', value: process.env.ZAMA_FHEVM_API_KEY ?? 'local' } });
const client = createFhevmEncryptClient({ chain, aclProgramAddress });
const inputProof = await client.buildInputProof({
  contractAddress: addressHex(required('TRANSFER_COMPUTE_SIGNER')),
  userAddress: addressHex(owner.address),
  values: [{ type: 'uint64', value: 400n }],
});
const inputProofResult = await client.submitInputProof({ inputProof });
const [hostConfig] = await getProgramDerivedAddress({
  programAddress: HOST_PROGRAM,
  seeds: [HOST_CONFIG_SEED],
});
const rpc = createSolanaRpc(required('TRANSFER_RPC_URL'));
const rpcSubscriptions = createSolanaRpcSubscriptions(required('TRANSFER_WS_URL'));
const signature = await client.confidentialTransfer({
  rpc,
  rpcSubscriptions,
  inputProof,
  inputProofResult,
  inputIndex: 0,
  owner,
  feePayer: owner,
  mint: address(required('TRANSFER_MINT')),
  fromAccount: address(required('TRANSFER_FROM_ACCOUNT')),
  toAccount: address(required('TRANSFER_TO_ACCOUNT')),
  fromBalanceValue: address(required('TRANSFER_FROM_BALANCE')),
  toBalanceValue: address(required('TRANSFER_TO_BALANCE')),
  hostConfig,
});
const inputHandle = inputProof.getInputHandles()[0]?.bytes32Hex;
if (inputHandle === undefined) throw new Error('SDK transfer proof did not contain its euint64 handle');
process.stdout.write(
  `${JSON.stringify({ version: 1, signature, inputHandle })}\n`,
);
process.exit(0);
