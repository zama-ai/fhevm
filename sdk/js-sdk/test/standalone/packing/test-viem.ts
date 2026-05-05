import type { Account, Chain, Transport, WalletClient, WriteContractReturnType } from 'viem';
import { createFhevmDecryptClient, createFhevmEncryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { getBaseEnv } from './setupCommon.js';
import { getViemTestConfig, type FheTestViemConfig } from './setup-viem.js';
import { createWalletClient, http, type Hex } from 'viem';
import { FHETestABI } from '../../fheTest/abi-v2.js';
import type { EncryptedValue, FhevmDecryptClient, FhevmEncryptClient, TypedValue } from '@fhevm/sdk/types';
import type { EncryptValueReturnType } from '@fhevm/sdk/actions/encrypt';
import type { SignDecryptionPermitReturnType, TransportKeyPair } from '@fhevm/sdk/actions/decrypt';

let config: FheTestViemConfig = getViemTestConfig();
setFhevmRuntimeConfig({
  auth: {
    type: 'ApiKeyHeader',
    value: config.zamaApiKey,
  },
});

const tv = { type: 'uint8', value: 123n };

console.log(`--- createFhevmEncryptClient() with chain ${config.chainName}`);

const client: FhevmEncryptClient = createFhevmEncryptClient({
  chain: config.fhevmChain,
  publicClient: config.publicClient,
});
await client.ready;

console.log(`--- encryptValue() value 123 with client on chain ${config.chainName}...`);

const result: EncryptValueReturnType = await client.encryptValue({
  contractAddress: config.fheTestAddress,
  userAddress: config.account.address,
  value: tv,
});

const inputHandle: EncryptedValue = result.encryptedValue;
const makePublic = true;
console.log(`--- Resulting handle: ${inputHandle}`);

console.log(`--- Setting value in FheTest`);

const walletClient: WalletClient<Transport, Chain, Account> = createWalletClient({
  account: config.account,
  chain: config.publicClient.chain,
  transport: http(getBaseEnv().rpcUrl),
});

const hash: WriteContractReturnType = await walletClient.writeContract({
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

const decryptClient: FhevmDecryptClient = createFhevmDecryptClient({
  chain: config.fhevmChain,
  publicClient: config.publicClient,
});

await decryptClient.ready;

// public decryption test
console.log('--- readPublicValue()...');

const actual: TypedValue = await decryptClient.readPublicValue({
  encryptedValue: result.encryptedValue,
});

console.log(`--- ReadPublicValue ${tv.type}: ${actual.value}`);

const transportKeyPair: TransportKeyPair = await decryptClient.generateTransportKeyPair();
const signedPermit: SignDecryptionPermitReturnType = await decryptClient.signDecryptionPermit({
  transportKeyPair: transportKeyPair,
  contractAddresses: [config.fheTestAddress],
  durationDays: 1,
  startTimestamp: Math.floor(Date.now() / 1000),
  signerAddress: config.account.address,
  signer: config.account,
});

console.log('--- decrypt()...');

const decryptedValue: TypedValue = await decryptClient.decryptValue({
  encryptedValue: inputHandle,
  contractAddress: config.fheTestAddress,
  transportKeyPair: transportKeyPair,
  signedPermit: signedPermit,
});

console.log('--- Decrypted value:', decryptedValue?.value);

process.exit(0);
