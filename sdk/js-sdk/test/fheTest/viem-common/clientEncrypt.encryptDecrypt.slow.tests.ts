import type { EncryptedValue } from '@fhevm/sdk/types';
import type { FhevmModuleVersions } from '../../../src/core/types/moduleVersions.js';
import {
  getViemDecryptClientOptions,
  getViemEncryptClientOptions,
  getViemTestConfig,
  type CreateViemDecryptClientFn,
  type CreateViemEncryptClientFn,
  type FheTestViemConfig,
} from '../setup-viem.js';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { clearTypeFromHandle, encryptTestCases, prepareSingleChain, isBytes32Hex } from '../setupCommon.js';
import { FHETestABI } from '../FheTest-abi-v2.js';
import { createWalletClient, http, type Hex } from 'viem';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext/clientEncrypt.encryptDecrypt.slow.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encryptDecrypt.slow.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encryptDecrypt.slow.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encryptDecrypt.slow.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientEncryptDecryptSlowTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmEncryptClient: CreateViemEncryptClientFn;
  readonly createFhevmDecryptClient: CreateViemDecryptClientFn;
  readonly moduleVersions?: FhevmModuleVersions | undefined;
}): void {
  describe.runIf(parameters.runIf)(
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
        const client = parameters.createFhevmEncryptClient({
          chain: config.fhevmChain,
          publicClient: config.publicClient,
          options: getViemEncryptClientOptions(config, parameters.moduleVersions),
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
          transport: http(prepareSingleChain().rpcUrl),
        });

        for (let i = 0; i < encryptTestCases.length; i++) {
          const enc: EncryptedValue = result.encryptedValues[i]!;
          const clearType = clearTypeFromHandle(enc);
          const ct = encryptTestCases[i]!.value;

          const inputHandle = enc;
          const inputProof = result.inputProof;
          const makePublic = true;

          // Compute function name from clearType: ebool → setEbool, euint8 → setEuint8, etc.
          const functionName = `setE${clearType}`;
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
        // │  Decrypt via signed permit + e2e transport key pair                 │
        // └─────────────────────────────────────────────────────────────────────┘
        const decryptClient = parameters.createFhevmDecryptClient({
          chain: config.fhevmChain,
          publicClient: config.publicClient,
          options: getViemDecryptClientOptions(parameters.moduleVersions),
        });

        await decryptClient.ready;

        const transportKeyPair = await decryptClient.generateTransportKeyPair();
        const signedPermit = await decryptClient.signDecryptionPermit({
          transportKeyPair: transportKeyPair,
          contractAddresses: [config.fheTestAddress],
          durationSeconds: 24 * 3600,
          startTimestamp: Math.floor(Date.now() / 1000) - 5,
          signerAddress: config.account.address,
          signer: config.account,
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
        // │  Verify the same clear values via decryptPublicValue (no permit)    │
        // └─────────────────────────────────────────────────────────────────────┘
        console.log('publicDecrypt...');

        const publicTypedValues = await decryptClient.decryptPublicValues({
          encryptedValues: result.encryptedValues,
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
