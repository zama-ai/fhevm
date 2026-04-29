import type { TypedValue } from '@fhevm/sdk/types';
import { createTypedValue } from '@fhevm/sdk/base';
import { createFhevmDecryptClient, createFhevmEncryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { getBaseEnv } from './setupCommon.js';
import { getViemTestConfig, type FheTestViemConfig } from './setup-viem.js';
import { createWalletClient, http, type Hex } from 'viem';
import { FHETestABI } from '../../fheTest/abi-v2.js';

let config: FheTestViemConfig;
config = getViemTestConfig();
setFhevmRuntimeConfig({
  auth: {
    type: 'ApiKeyHeader',
    value: config.zamaApiKey,
  },
});

const tv: TypedValue = createTypedValue({ type: 'uint8', value: 123n });

console.log(`--- createFhevmEncryptClient() with chain ${config.chainName}`);

const client = createFhevmEncryptClient({
  chain: config.fhevmChain,
  publicClient: config.publicClient,
});
await client.ready;

console.log(`--- encryptValue() value 123 with client on chain ${config.chainName}...`);

const result = await client.encryptValue({
  contractAddress: config.fheTestAddress,
  userAddress: config.account.address,
  value: tv,
});

const inputHandle = result.encryptedValue;
const makePublic = true;
console.log(`--- Resulting handle: ${inputHandle}`);

console.log(`--- Setting value in FheTest`);

const walletClient = createWalletClient({
  account: config.account,
  chain: config.publicClient.chain,
  transport: http(getBaseEnv().rpcUrl),
});

const hash = await walletClient.writeContract({
  address: config.fheTestAddress as Hex,
  abi: FHETestABI,
  functionName: 'setEuint8',
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  args: [inputHandle, result.inputProof, tv.value, makePublic] as any,
});

const receipt = await config.publicClient.waitForTransactionReceipt({
  hash,
});

if (receipt.status !== 'success') {
  console.error('Transaction failed');
  process.exit(1);
}

console.log('--- Transaction succeeded');

const decryptClient = createFhevmDecryptClient({
  chain: config.fhevmChain,
  publicClient: config.publicClient,
});

await decryptClient.ready;

// public decryption test
console.log('--- readPublicValue()...');

const actual = await decryptClient.readPublicValue({
  encryptedValue: result.encryptedValue,
});

console.log(`--- ReadPublicValue ${tv.type}: ${actual.value}`);

const transportKeypair = await decryptClient.generateTransportKeypair();
const signedPermit = await decryptClient.signDecryptionPermit({
  transportKeypair: transportKeypair,
  contractAddresses: [config.fheTestAddress],
  durationDays: 1,
  startTimestamp: Math.floor(Date.now() / 1000),
  signerAddress: config.account.address,
  signer: config.account,
});

console.log('--- decrypt()...');

const decryptedValue = await decryptClient.decryptValue({
  encryptedValue: inputHandle,
  contractAddress: config.fheTestAddress,
  transportKeypair: transportKeypair,
  signedPermit: signedPermit,
});

console.log('--- Decrypted value:', decryptedValue?.value);

process.exit(0);
