import type { ethers } from 'ethers';
import type { TypedValue } from '../../../src/core/types/primitives.js';
import type { EncryptedValue } from '../../../src/core/types/encryptedTypes.js';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { createFhevmCleartextDecryptClient, createFhevmCleartextEncryptClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig, type FheTestEthersConfig } from '../ethers/setup.js';
import { createTypedValueArray } from '../../../src/core/base/typedValue.js';
import { isBytes32Hex } from '../../../src/core/base/bytes.js';
import { toFhevmHandle } from '../../../src/core/handle/FhevmHandle.js';
import { isCleartext } from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// localhost:
// ----------
// CHAIN=localhost npx vitest run --config test/fheTest/vitest.config.ts cleartext-ethers/clientEncrypt.encryptDecrypt.slow.test.ts
//
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

describe.runIf(isCleartext(getEthersTestConfig().chainName))(
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
      const client = createFhevmCleartextEncryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;

      const result = await client.encryptValues({
        contractAddress: config.fheTestAddress,
        userAddress: config.wallet.address,
        values: encryptTestCases,
      });

      expect(result.encryptedValues).toHaveLength(encryptTestCases.length);
      expect(result.inputProof).toBeDefined();
      expect(result.inputProof.startsWith('0x')).toBe(true);

      for (let i = 0; i < encryptTestCases.length; i++) {
        const tc = encryptTestCases[i]!;
        const ev = result.encryptedValues[i]!;
        expect(ev).toBeDefined();
        expect(isBytes32Hex(ev)).toBe(true);
        console.log(`  ${tc.type}: handle=${ev.slice(0, 20)}...`);
      }

      // ┌─────────────────────────────────────────────────────────────────────┐
      // │  Phase 2: SUBMIT ON-CHAIN                                           │
      // │  Send encrypted handles + input proof to FHETest contract           │
      // └─────────────────────────────────────────────────────────────────────┘
      const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

      for (let i = 0; i < encryptTestCases.length; i++) {
        const enc: EncryptedValue = result.encryptedValues[i]!;
        const fheType = toFhevmHandle(enc).fheType;
        const ct = encryptTestCases[i]!.value;

        const inputHandle = enc;
        const inputProof = result.inputProof;
        const makePublic = true;

        let tx: ethers.TransactionResponse;

        console.log(`setE${fheType.substring(1)}(${inputHandle})...`);

        switch (fheType) {
          case 'ebool':
            tx = await fheTest.setEbool!(inputHandle, inputProof, ct, makePublic);
            break;
          case 'euint8':
            tx = await fheTest.setEuint8!(inputHandle, inputProof, ct, makePublic);
            break;
          case 'euint16':
            tx = await fheTest.setEuint16!(inputHandle, inputProof, ct, makePublic);
            break;
          case 'euint32':
            tx = await fheTest.setEuint32!(inputHandle, inputProof, ct, makePublic);
            break;
          case 'euint64':
            tx = await fheTest.setEuint64!(inputHandle, inputProof, ct, makePublic);
            break;
          case 'euint128':
            tx = await fheTest.setEuint128!(inputHandle, inputProof, ct, makePublic);
            break;
          case 'euint256':
            tx = await fheTest.setEuint256!(inputHandle, inputProof, ct, makePublic);
            break;
          case 'eaddress':
            tx = await fheTest.setEaddress!(inputHandle, inputProof, ct, makePublic);
            break;
          default:
            throw new Error(`Unsupported fheType`);
        }

        const receipt = await tx.wait();
        expect(receipt?.status).toBe(1);
      }

      // ┌─────────────────────────────────────────────────────────────────────┐
      // │  Phase 3: PRIVATE DECRYPT                                           │
      // │  Decrypt via signed permit + e2e transport key pair                  │
      // └─────────────────────────────────────────────────────────────────────┘
      const decryptClient = createFhevmCleartextDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });

      await decryptClient.ready;

      const transportKeyPair = await decryptClient.generateTransportKeyPair();
      const signedPermit = await decryptClient.signDecryptionPermit({
        transportKeyPair: transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationDays: 1,
        startTimestamp: Math.floor(Date.now() / 1000),
        signerAddress: config.wallet.address,
        signer: config.signer,
      });

      const encryptedValues = result.encryptedValues;

      console.log('decrypt...');

      const typedValues = await decryptClient.decryptValues({
        encryptedValues,
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
      // │  Verify the same clear values via readPublicValue (no permit)       │
      // └─────────────────────────────────────────────────────────────────────┘
      console.log('publicDecrypt...');

      const publicTypedValues = await decryptClient.readPublicValues({
        encryptedValues: result.encryptedValues,
      });

      expect(publicTypedValues).toHaveLength(encryptTestCases.length);

      for (let i = 0; i < encryptTestCases.length; i++) {
        const expected = encryptTestCases[i]!;
        const actual = publicTypedValues[i]!;
        console.log(`  readPublicValue ${expected.type}: ${actual.value}`);
        expect(actual.value).toBe(expected.value);
        expect(actual.type).toBe(expected.type);
      }
    });
  },
  5 * 60_000,
);
