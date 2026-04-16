import type { TypedValue } from '@fhevm/sdk/types';
import { createTypedValue } from '@fhevm/sdk/base';
import { createFhevmDecryptClient, createFhevmEncryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { getEthersTestConfig, type FheTestEthersConfig } from './setup.js';
import type { ethers } from 'ethers';

let config: FheTestEthersConfig;
config = getEthersTestConfig();
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
  provider: config.provider,
});
await client.ready;

console.log(`--- encryptValue() value 123 with client on chain ${config.chainName}...`);

const result = await client.encryptValue({
  contractAddress: config.fheTestAddress,
  userAddress: config.wallet.address,
  value: tv,
});

const inputHandle = result.encryptedValue;
const makePublic = true;
console.log(`--- Resulting handle: ${inputHandle}`);

console.log(`--- Setting value in FheTest`);

const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

let tx: ethers.TransactionResponse = await fheTest.setEuint8!(inputHandle, result.inputProof, tv.value, makePublic);
const receipt = await tx.wait();
if (receipt?.status !== 1) {
  console.error('Transaction failed');
  process.exit(1);
}

console.log('--- Transaction succeeded');

const decryptClient = createFhevmDecryptClient({
  chain: config.fhevmChain,
  provider: config.provider,
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
  signerAddress: config.wallet.address,
  signer: config.signer,
});

console.log('--- decryptValue()...');

const decryptedValue = await decryptClient.decryptValue({
  encryptedValue: inputHandle,
  contractAddress: config.fheTestAddress,
  transportKeypair: transportKeypair,
  signedPermit: signedPermit,
});

console.log('--- Decrypted value:', decryptedValue?.value);

process.exit(0);
