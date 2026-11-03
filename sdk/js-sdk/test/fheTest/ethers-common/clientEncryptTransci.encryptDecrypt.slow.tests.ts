import type { ethers } from 'ethers';
import { asEncryptedValue, type EncryptedValue } from '@fhevm/sdk/types';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import {
  getEthersTestConfig,
  type CreateEthersDecryptClientFn,
  type CreateEthersEncryptTransciClientFn,
  type FheTestEthersConfig,
} from '../setup-ethers.js';
import {
  clearTypeFromHandle,
  createLogger,
  encryptTestCases,
  fheTypeIdFromName,
  isBytes32Hex,
} from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientEncrypt.encryptDecrypt.slow.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encryptDecrypt.slow.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encryptDecrypt.slow.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encryptDecrypt.slow.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientEncryptTransciDecryptSlowTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmEncryptTransciClient: CreateEthersEncryptTransciClientFn;
  readonly createFhevmDecryptClient: CreateEthersDecryptClientFn;
}): void {
  describe.runIf(parameters.runIf)(
    'EncryptTransci-Decrypt',
    () => {
      let config: FheTestEthersConfig;

      beforeAll(() => {
        config = getEthersTestConfig();
        setFhevmRuntimeConfig({
          auth: {
            type: 'ApiKeyHeader',
            value: config.zamaApiKey,
          },
          logger: createLogger(console.log),
        });
      });

      it('should encrypt, submit on-chain, and decrypt all types', async () => {
        // ┌─────────────────────────────────────────────────────────────────────┐
        // │  Phase 1: ENCRYPT                                                   │
        // │  Client-side encryption of all FHE types into external handles      │
        // └─────────────────────────────────────────────────────────────────────┘
        const client = parameters.createFhevmEncryptTransciClient({
          chain: config.fhevmChain,
          provider: config.provider,
        });
        await client.ready;

        // RFC-030: the one-time pad is fetched from the KMS through a pad
        // decryption request authenticated by a v2 (unified) permit.
        // The encrypt-transciphering client generates the transport key pair itself.
        const padTransportKeyPair = await client.generateTransportKeyPair();
        const padSignedPermit = await client.signUnifiedDecryptionPermit({
          transportKeyPair: padTransportKeyPair,
          contractAddresses: [config.fheTestAddress],
          durationSeconds: 24 * 3600,
          startTimestamp: Math.floor(Date.now() / 1000) - 5,
          signerAddress: config.wallet.address,
          signer: config.signer,
        });

        const result = await client.encryptValuesTransci({
          contractAddress: config.fheTestAddress,
          userAddress: config.wallet.address,
          values: encryptTestCases,
          signedPermit: padSignedPermit,
        });

        expect(result.encryptedValues).toHaveLength(encryptTestCases.length);
        expect(result.inputPackage).toBeDefined();
        expect(result.inputPackage.startsWith('0x')).toBe(true);

        for (let i = 0; i < encryptTestCases.length; i++) {
          const tc = encryptTestCases[i]!;
          const ev = result.encryptedValues[i]!;
          expect(ev).toBeDefined();
          expect(isBytes32Hex(ev)).toBe(true);
          console.log(`  ${tc.type}: handle=${ev.slice(0, 20)}...`);
        }

        // ┌─────────────────────────────────────────────────────────────────────┐
        // │  Phase 2: SUBMIT ON-CHAIN                                           │
        // │  Send encrypted handles + input package to FHETest contract         │
        // └─────────────────────────────────────────────────────────────────────┘
        const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

        // RFC-030: `fromExternal` mints a fresh compute handle in the transaction;
        // the external handles are only typed indices into the input package.
        // FHETest stores each compute handle, keyed by (account, FheType).
        const computeHandles: EncryptedValue[] = [];

        for (let i = 0; i < encryptTestCases.length; i++) {
          const enc: EncryptedValue = result.encryptedValues[i]!;
          const fheType = clearTypeFromHandle(enc);
          const ct = encryptTestCases[i]!.value;

          const externalHandle = enc;
          const inputPackage = result.inputPackage;
          const makePublic = true;

          let tx: ethers.TransactionResponse;

          console.log(`setE${fheType}(${externalHandle})...`);

          switch (fheType) {
            case 'bool':
              tx = await fheTest.setEbool!(externalHandle, inputPackage, ct, makePublic);
              break;
            case 'uint8':
              tx = await fheTest.setEuint8!(externalHandle, inputPackage, ct, makePublic);
              break;
            case 'uint16':
              tx = await fheTest.setEuint16!(externalHandle, inputPackage, ct, makePublic);
              break;
            case 'uint32':
              tx = await fheTest.setEuint32!(externalHandle, inputPackage, ct, makePublic);
              break;
            case 'uint64':
              tx = await fheTest.setEuint64!(externalHandle, inputPackage, ct, makePublic);
              break;
            case 'uint128':
              tx = await fheTest.setEuint128!(externalHandle, inputPackage, ct, makePublic);
              break;
            case 'uint256':
              tx = await fheTest.setEuint256!(externalHandle, inputPackage, ct, makePublic);
              break;
            case 'address':
              tx = await fheTest.setEaddress!(externalHandle, inputPackage, ct, makePublic);
              break;
            default:
              throw new Error(`Unsupported type: ${fheType}`);
          }

          const receipt = await tx.wait();
          expect(receipt?.status).toBe(1);

          const computeHandle: string = await fheTest.getHandleOf!(
            config.wallet.address,
            fheTypeIdFromName(`e${fheType}`),
          );
          expect(computeHandle.toLowerCase()).not.toBe(externalHandle.toLowerCase());
          computeHandles.push(asEncryptedValue(computeHandle));
        }

        // ┌─────────────────────────────────────────────────────────────────────┐
        // │  Phase 3: PRIVATE DECRYPT                                           │
        // │  Decrypt via signed permit + e2e transport key pair                 │
        // └─────────────────────────────────────────────────────────────────────┘
        const decryptClient = parameters.createFhevmDecryptClient({
          chain: config.fhevmChain,
          provider: config.provider,
        });

        await decryptClient.ready;

        // Reuse the key pair generated by the encrypt-transciphering client: key
        // pairs are interchangeable between clients sharing the same runtime.
        const transportKeyPair = padTransportKeyPair;
        const signedPermit = await decryptClient.signLegacyDecryptionPermit({
          transportKeyPair: transportKeyPair,
          contractAddresses: [config.fheTestAddress],
          durationSeconds: 24 * 3600,
          startTimestamp: Math.floor(Date.now() / 1000) - 5,
          signerAddress: config.wallet.address,
          signer: config.signer,
        });

        console.log('decrypt...');

        const typedValues = await decryptClient.decryptValues({
          encryptedValues: computeHandles,
          contractAddress: config.fheTestAddress,
          signedPermit,
          transportKeyPair: transportKeyPair,
        });

        for (let i = 0; i < encryptTestCases.length; i++) {
          console.log(typedValues[i]?.value);
          expect(typedValues[i]?.type).toBe(encryptTestCases[i]?.type);
          expect(typedValues[i]?.value).toBe(encryptTestCases[i]?.value);
        }

        // ┌─────────────────────────────────────────────────────────────────────┐
        // │  Phase 4: PUBLIC DECRYPT                                            │
        // │  Verify the same clear values via decryptPublicValue (no permit)    │
        // └─────────────────────────────────────────────────────────────────────┘
        console.log('publicDecrypt...');

        const publicTypedValues = await decryptClient.decryptPublicValues({
          encryptedValues: computeHandles,
        });

        expect(publicTypedValues).toHaveLength(encryptTestCases.length);

        for (let i = 0; i < encryptTestCases.length; i++) {
          const expected = encryptTestCases[i]!;
          const actual = publicTypedValues[i]!;
          console.log(`  decryptPublicValue ${expected.type}: ${actual.value}`);
          expect(actual.value).toBe(expected.value);
          expect(actual.type).toBe(expected.type);
        }
      });
    },
    5 * 60_000,
  );
}
