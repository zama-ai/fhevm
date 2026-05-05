import type { ethers } from 'ethers';
import type { FheTestEthersConfig } from './setup-ethers.js';
import type {
  TransportKeyPair,
  SignDecryptionPermitReturnType,
  DecryptValueReturnType,
} from '@fhevm/sdk/actions/decrypt';
import type { FhevmEncryptClient, FhevmDecryptClient, TypedValue, EncryptedValue } from '@fhevm/sdk/types';
import { createFhevmDecryptClient, createFhevmEncryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from './setup-ethers.js';
import type { EncryptValueReturnType } from '@fhevm/sdk/actions/encrypt';

let config: FheTestEthersConfig = getEthersTestConfig();
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
  provider: config.provider,
});
await client.ready;

console.log(`--- encryptValue() value 123 with client on chain ${config.chainName}...`);

const result: EncryptValueReturnType = await client.encryptValue({
  contractAddress: config.fheTestAddress,
  userAddress: config.wallet.address,
  value: tv,
});

const inputHandle: EncryptedValue = result.encryptedValue;
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

const decryptClient: FhevmDecryptClient = createFhevmDecryptClient({
  chain: config.fhevmChain,
  provider: config.provider,
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
  signerAddress: config.wallet.address,
  signer: config.signer,
});

console.log('--- decryptValue()...');

const decryptedValue: DecryptValueReturnType = await decryptClient.decryptValue({
  encryptedValue: inputHandle,
  contractAddress: config.fheTestAddress,
  transportKeyPair: transportKeyPair,
  signedPermit: signedPermit,
});

console.log('--- Decrypted value:', decryptedValue?.value);

process.exit(0);
