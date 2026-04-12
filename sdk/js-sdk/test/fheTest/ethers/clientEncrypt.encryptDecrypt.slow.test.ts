//
// Sepolia Testnet:
// ----------------
// npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encryptDecrypt.test.ts
//
// Devnet:
// -------
// CHAIN=devnet npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encryptDecrypt.test.ts
//
// localhost fhevm:
// ----------------
// CHAIN=localhostFhevm npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encryptDecrypt.test.ts
//
import { describe, it, expect, beforeAll } from 'vitest';
import {
  createFhevmDecryptClient,
  createFhevmEncryptClient,
  setFhevmRuntimeConfig,
} from '@fhevm/sdk/ethers';
import { getEthersTestConfig, type FheTestEthersConfig } from './setup.js';
import type { ethers } from 'ethers';
import type {
  ChecksummedAddress,
  TypedValue,
} from '../../../src/core/types/primitives.js';
import { createTypedValueArray } from '../../../src/core/base/typedValue.js';

////////////////////////////////////////////////////////////////////////////////

// Map FHE type to: contract function name, value type name, test value
const encryptTestCases: TypedValue[] = createTypedValueArray([
  {
    value: true,
    type: 'bool' as const,
  },
  {
    type: 'uint8' as const,
    value: 42,
  },
  {
    type: 'uint16' as const,
    value: 1234,
  },
  {
    type: 'uint32' as const,
    value: 123456,
  },
  {
    type: 'uint64' as const,
    value: 123456789n,
  },
  {
    type: 'uint128' as const,
    value: 123456789012345n,
  },
  {
    type: 'uint256' as const,
    value: 123456789012345678901234567890n,
  },
  {
    type: 'address' as const,
    value: '0x37AC010c1c566696326813b840319B58Bb5840E4',
  },
]);

////////////////////////////////////////////////////////////////////////////////

describe(
  'Encrypt-Decrypt',
  () => {
    let config: FheTestEthersConfig;

    beforeAll(() => {
      config = getEthersTestConfig();
      setFhevmRuntimeConfig({
        auth: {
          type: 'ApiKeyHeader',
          value: config.zamaApiKey,
        },
        logger: {
          debug: (message: string) => console.log(message),
          error: (message: string) => console.log(message),
        },
      });
    });

    it('should encrypt, submit on-chain, and decrypt all types', async () => {
      // ┌─────────────────────────────────────────────────────────────────────┐
      // │  Phase 1: ENCRYPT                                                   │
      // │  Client-side encryption of all FHE types into external handles      │
      // └─────────────────────────────────────────────────────────────────────┘
      const client = createFhevmEncryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;

      const result = await client.encrypt({
        contractAddress: config.fheTestAddress,
        userAddress: config.wallet.address,
        values: encryptTestCases,
      });

      expect(result.externalEncryptedValues).toHaveLength(
        encryptTestCases.length,
      );
      expect(result.inputProof).toBeDefined();
      expect(result.inputProof.startsWith('0x')).toBe(true);

      for (let i = 0; i < encryptTestCases.length; i++) {
        const tc = encryptTestCases[i]!;
        const ev = result.externalEncryptedValues[i]!;
        console.log(`  ${tc.type}: handle=${ev.bytes32Hex.slice(0, 20)}...`);
        expect(ev.bytes32Hex).toBeDefined();
      }

      // ┌─────────────────────────────────────────────────────────────────────┐
      // │  Phase 2: SUBMIT ON-CHAIN                                           │
      // │  Send encrypted handles + input proof to FHETest contract           │
      // └─────────────────────────────────────────────────────────────────────┘
      const fheTest = config.fheTestContract.connect(
        config.signer,
      ) as ethers.Contract;

      for (let i = 0; i < encryptTestCases.length; i++) {
        const enc = result.externalEncryptedValues[i]!;
        const ct = encryptTestCases[i]!.value;

        const inputHandle = enc.bytes32Hex;
        const inputProof = result.inputProof;
        const makePublic = true;

        let tx: ethers.TransactionResponse;

        console.log(`setE${enc.fheType.substring(1)}(${inputHandle})...`);

        switch (enc.fheType) {
          case 'ebool':
            tx = await fheTest.setEbool!(
              inputHandle,
              inputProof,
              ct,
              makePublic,
            );
            break;
          case 'euint8':
            tx = await fheTest.setEuint8!(
              inputHandle,
              inputProof,
              ct,
              makePublic,
            );
            break;
          case 'euint16':
            tx = await fheTest.setEuint16!(
              inputHandle,
              inputProof,
              ct,
              makePublic,
            );
            break;
          case 'euint32':
            tx = await fheTest.setEuint32!(
              inputHandle,
              inputProof,
              ct,
              makePublic,
            );
            break;
          case 'euint64':
            tx = await fheTest.setEuint64!(
              inputHandle,
              inputProof,
              ct,
              makePublic,
            );
            break;
          case 'euint128':
            tx = await fheTest.setEuint128!(
              inputHandle,
              inputProof,
              ct,
              makePublic,
            );
            break;
          case 'euint256':
            tx = await fheTest.setEuint256!(
              inputHandle,
              inputProof,
              ct,
              makePublic,
            );
            break;
          case 'eaddress':
            tx = await fheTest.setEaddress!(
              inputHandle,
              inputProof,
              ct,
              makePublic,
            );
            break;
          default:
            throw new Error(`Unsupported fheType`);
        }

        const receipt = await tx.wait();
        expect(receipt?.status).toBe(1);
      }

      // ┌─────────────────────────────────────────────────────────────────────┐
      // │  Phase 3: PRIVATE DECRYPT                                           │
      // │  Decrypt via signed permit + e2e transport keypair                  │
      // └─────────────────────────────────────────────────────────────────────┘
      const decryptClient = createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });

      await decryptClient.ready;

      const e2eTransportKeypair =
        await decryptClient.generateE2eTransportKeypair();
      const signedPermit = await decryptClient.signDecryptionPermit({
        e2eTransportKeypair,
        contractAddresses: [config.fheTestAddress],
        durationDays: 1,
        startTimestamp: Math.floor(Date.now() / 1000),
        signerAddress: config.wallet.address,
        signer: config.signer,
      });

      const encryptedValues = result.externalEncryptedValues.map((ev) => {
        return {
          encryptedValue: ev.bytes32Hex,
          contractAddress: config.fheTestAddress as ChecksummedAddress,
        };
      });

      console.log('decrypt...');

      const clearValues = await decryptClient.decrypt({
        encryptedValues,
        signedPermit,
        e2eTransportKeypair,
      });

      for (let i = 0; i < encryptTestCases.length; i++) {
        console.log(clearValues[i]?.value);
        expect(clearValues[i]?.type).toBe(encryptTestCases[i]?.type);
        expect(clearValues[i]?.value).toBe(encryptTestCases[i]?.value);
      }

      // ┌─────────────────────────────────────────────────────────────────────┐
      // │  Phase 4: PUBLIC DECRYPT                                            │
      // │  Verify the same clear values via readPublicValue (no permit)       │
      // └─────────────────────────────────────────────────────────────────────┘
      console.log('publicDecrypt...');

      const publicProof = await decryptClient.readPublicValue({
        encryptedValues: result.externalEncryptedValues,
      });

      expect(publicProof.orderedClearValues).toHaveLength(
        encryptTestCases.length,
      );

      for (let i = 0; i < encryptTestCases.length; i++) {
        const expected = encryptTestCases[i]!;
        const actual = publicProof.orderedClearValues[i]!;
        console.log(`  readPublicValue ${expected.type}: ${actual.value}`);
        expect(actual.value).toBe(expected.value);
      }
    });
  },
  5 * 60_000,
);
