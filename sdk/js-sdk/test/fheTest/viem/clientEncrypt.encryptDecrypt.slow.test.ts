import type { TypedValue } from '../../../src/core/types/primitives.js';
import type { EncryptedValue } from '../../../src/core/types/encryptedTypes.js';
import { describe, it, expect, beforeAll } from 'vitest';
import { createFhevmDecryptClient, createFhevmEncryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { getViemTestConfig, type FheTestViemConfig } from './setup.js';
import { isV2, getBaseEnv } from '../setupCommon.js';
import { FHETestABI } from '../abi-v2.js';
import { createTypedValueArray } from '../../../src/core/base/typedValue.js';
import { createWalletClient, http, type Hex } from 'viem';
import { isBytes32Hex } from '../../../src/core/base/bytes.js';
import { toFhevmHandle } from '../../../src/core/handle/FhevmHandle.js';

////////////////////////////////////////////////////////////////////////////////
//
// Sepolia Testnet:
// ----------------
// npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encryptDecrypt.slow.test.ts
//
// Devnet:
// -------
// CHAIN=devnet npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encryptDecrypt.slow.test.ts
//
// localhost fhevm:
// ----------------
// CHAIN=localhostFhevm npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encryptDecrypt.slow.test.ts
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

describe.runIf(isV2(getViemTestConfig().chainName))(
  'Encrypt-Decrypt',
  () => {
    let config: FheTestViemConfig;

    beforeAll(() => {
      config = getViemTestConfig();
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
        publicClient: config.publicClient,
      });
      await client.ready;

      const result = await client.encryptValues({
        contractAddress: config.fheTestAddress,
        userAddress: config.account.address,
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
      const walletClient = createWalletClient({
        account: config.account,
        chain: config.publicClient.chain,
        transport: http(getBaseEnv().rpcUrl),
      });

      for (let i = 0; i < encryptTestCases.length; i++) {
        const enc: EncryptedValue = result.encryptedValues[i]!;
        const fheType = toFhevmHandle(enc).fheType;
        const ct = encryptTestCases[i]!.value;

        const inputHandle = enc;
        const inputProof = result.inputProof;
        const makePublic = true;

        // Compute function name from fheType: ebool → setEbool, euint8 → setEuint8, etc.
        const functionName = `set${fheType.charAt(0).toUpperCase()}${fheType.slice(1)}`;
        console.log(`${functionName}(${inputHandle})...`);

        const hash = await walletClient.writeContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: functionName as
            | 'setEbool'
            | 'setEuint8'
            | 'setEuint16'
            | 'setEuint32'
            | 'setEuint64'
            | 'setEuint128'
            | 'setEuint256'
            | 'setEaddress',
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          args: [inputHandle, inputProof, ct, makePublic] as any,
        });

        const receipt = await config.publicClient.waitForTransactionReceipt({
          hash,
        });
        expect(receipt.status).toBe('success');
      }

      // ┌─────────────────────────────────────────────────────────────────────┐
      // │  Phase 3: PRIVATE DECRYPT                                           │
      // │  Decrypt via signed permit + e2e transport keypair                  │
      // └─────────────────────────────────────────────────────────────────────┘
      const decryptClient = createFhevmDecryptClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });

      await decryptClient.ready;

      const transportKeypair = await decryptClient.generateTransportKeypair();
      const signedPermit = await decryptClient.signDecryptionPermit({
        transportKeypair,
        contractAddresses: [config.fheTestAddress],
        durationDays: 1,
        startTimestamp: Math.floor(Date.now() / 1000),
        signerAddress: config.account.address,
        signer: config.account,
      });

      const encryptedValues = result.encryptedValues;

      console.log('decrypt...');

      const typedValues = await decryptClient.decryptValues({
        encryptedValues,
        contractAddress: config.fheTestAddress,
        signedPermit,
        transportKeypair,
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
