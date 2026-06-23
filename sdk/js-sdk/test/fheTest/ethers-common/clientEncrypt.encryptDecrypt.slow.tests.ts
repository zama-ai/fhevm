import type { ethers } from 'ethers';
import type { EncryptedValue } from '@fhevm/sdk/types';
import type { FhevmModuleVersions } from '../../../src/core/types/moduleVersions.js';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import {
  getEthersDecryptClientOptions,
  getEthersEncryptClientOptions,
  getEthersTestConfig,
  type CreateEthersDecryptClientFn,
  type CreateEthersEncryptClientFn,
  type FheTestEthersConfig,
} from '../setup-ethers.js';
import { clearTypeFromHandle, encryptTestCases, isBytes32Hex } from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientEncrypt.encryptDecrypt.slow.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encryptDecrypt.slow.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encryptDecrypt.slow.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encryptDecrypt.slow.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientEncryptDecryptSlowTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmEncryptClient: CreateEthersEncryptClientFn;
  readonly createFhevmDecryptClient: CreateEthersDecryptClientFn;
  readonly moduleVersions?: FhevmModuleVersions | undefined;
  /**
   * When provided, encryption uses deterministic ("seeded") public encryption
   * with this seed, and the test additionally asserts verify-by-reproduction
   * (an independent verifier reproduces identical handles). When omitted, the
   * normal (non-deterministic) path is exercised, unchanged.
   */
  readonly seed?: Uint8Array | undefined;
}): void {
  const seed = parameters.seed;

  describe.runIf(parameters.runIf)(
    `Encrypt-Decrypt${seed !== undefined ? ' (seeded)' : ''}`,
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

      it('should encrypt, submit on-chain, and decrypt all types', async (ctx) => {
        // ┌─────────────────────────────────────────────────────────────────────┐
        // │  Phase 1: ENCRYPT                                                   │
        // │  Client-side encryption of all FHE types into external handles      │
        // └─────────────────────────────────────────────────────────────────────┘
        const client = parameters.createFhevmEncryptClient({
          chain: config.fhevmChain,
          provider: config.provider,
          options: getEthersEncryptClientOptions(config, parameters.moduleVersions),
        });
        await client.ready;

        // Seeded encryption is a TFHE 1.6.1 feature; on older chains (e.g.
        // localstack v11/v12 → tfhe 1.5.3) `encryptSeeded` is a hard error, so
        // skip the seeded variant there. The non-seeded variant always runs.
        if (seed !== undefined && client.tfheVersion !== '1.6.1') {
          ctx.skip();
          return;
        }

        // Seeded mode uses the distinct `encryptSeeded` API; the default path
        // (`encryptValues`) is unchanged and never sees a seed.
        const result =
          seed !== undefined
            ? await client.encryptSeeded({
                contractAddress: config.fheTestAddress,
                userAddress: config.wallet.address,
                values: encryptTestCases,
                seed,
              })
            : await client.encryptValues({
                contractAddress: config.fheTestAddress,
                userAddress: config.wallet.address,
                values: encryptTestCases,
              });

        expect(result.encryptedValues).toHaveLength(encryptTestCases.length);
        expect(result.inputProof).toBeDefined();
        expect(result.inputProof.startsWith('0x')).toBe(true);

        // ┌─────────────────────────────────────────────────────────────────────┐
        // │  Phase 1b: VERIFY-BY-REPRODUCTION (seeded only)                     │
        // │  An independent verifier re-runs the same seeded encryption and     │
        // │  must reproduce byte-identical handles. A different seed must not.   │
        // └─────────────────────────────────────────────────────────────────────┘
        if (seed !== undefined) {
          // encryptSeeded echoes back the seed it used.
          expect('seed' in result && result.seed).toBeDefined();

          const verification = await client.verifySeededEncryption({
            seed,
            values: encryptTestCases,
            contractAddress: config.fheTestAddress,
            userAddress: config.wallet.address,
            expectedEncryptedValues: result.encryptedValues,
          });
          expect(verification.mismatches).toEqual([]);
          expect(verification.verified).toBe(true);

          // Negative control: a different seed must NOT reproduce the same handles.
          const wrongSeed = new Uint8Array(seed.length).fill((seed[0]! + 1) & 0xff);
          const bad = await client.verifySeededEncryption({
            seed: wrongSeed,
            values: encryptTestCases,
            contractAddress: config.fheTestAddress,
            userAddress: config.wallet.address,
            expectedEncryptedValues: result.encryptedValues,
          });
          expect(bad.verified).toBe(false);
        }

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
          const fheType = clearTypeFromHandle(enc);
          const ct = encryptTestCases[i]!.value;

          const inputHandle = enc;
          const inputProof = result.inputProof;
          const makePublic = true;

          let tx: ethers.TransactionResponse;

          console.log(`setE${fheType}(${inputHandle})...`);

          switch (fheType) {
            case 'bool':
              tx = await fheTest.setEbool!(inputHandle, inputProof, ct, makePublic);
              break;
            case 'uint8':
              tx = await fheTest.setEuint8!(inputHandle, inputProof, ct, makePublic);
              break;
            case 'uint16':
              tx = await fheTest.setEuint16!(inputHandle, inputProof, ct, makePublic);
              break;
            case 'uint32':
              tx = await fheTest.setEuint32!(inputHandle, inputProof, ct, makePublic);
              break;
            case 'uint64':
              tx = await fheTest.setEuint64!(inputHandle, inputProof, ct, makePublic);
              break;
            case 'uint128':
              tx = await fheTest.setEuint128!(inputHandle, inputProof, ct, makePublic);
              break;
            case 'uint256':
              tx = await fheTest.setEuint256!(inputHandle, inputProof, ct, makePublic);
              break;
            case 'address':
              tx = await fheTest.setEaddress!(inputHandle, inputProof, ct, makePublic);
              break;
            default:
              throw new Error(`Unsupported type: ${fheType}`);
          }

          const receipt = await tx.wait();
          expect(receipt?.status).toBe(1);
        }

        // ┌─────────────────────────────────────────────────────────────────────┐
        // │  Phase 3: PRIVATE DECRYPT                                           │
        // │  Decrypt via signed permit + e2e transport key pair                  │
        // └─────────────────────────────────────────────────────────────────────┘
        const decryptClient = parameters.createFhevmDecryptClient({
          chain: config.fhevmChain,
          provider: config.provider,
          options: getEthersDecryptClientOptions(parameters.moduleVersions),
        });

        await decryptClient.ready;

        const transportKeyPair = await decryptClient.generateTransportKeyPair();
        const signedPermit = await decryptClient.signDecryptionPermit({
          transportKeyPair: transportKeyPair,
          contractAddresses: [config.fheTestAddress],
          durationDays: 1,
          startTimestamp: Math.floor(Date.now() / 1000) - 5,
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
