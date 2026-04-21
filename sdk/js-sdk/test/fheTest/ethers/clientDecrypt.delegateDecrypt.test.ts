import type { ChecksummedAddress } from '../../../src/core/types/primitives.js';
import type { FheType } from '../../../src/core/types/fheType.js';
import type { Handle } from '../../../src/core/types/encryptedTypes-p.js';
import { describe, it, expect, beforeAll } from 'vitest';
import { createFhevmDecryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { delegateForUserDecryption, getUserDecryptionDelegationExpirationDate } from '@fhevm/sdk/actions/host';
import { getEthersTestConfig, type FheTestEthersConfig } from './setup.js';
import { fheTypeIdFromName } from '../../../src/core/handle/FheType.js';
import { ethers } from 'ethers';
import { toFhevmHandle } from '../../../src/core/handle/FhevmHandle.js';
import { asEncryptedValue } from '../../../src/core/handle/EncryptedValue.js';

////////////////////////////////////////////////////////////////////////////////
//
// Sepolia Testnet:
// ----------------
// npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.delegateDecrypt.test.ts
//
// Devnet:
// -------
// CHAIN=devnet npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.delegateDecrypt.test.ts
//
// localhost fhevm:
// ----------------
// CHAIN=localhostFhevm npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.delegateDecrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

// Each FHE type to decrypt
const decryptTestCases: readonly FheType[] = [
  'ebool',
  'euint8',
  'euint16',
  'euint32',
  'euint64',
  'euint128',
  'euint256',
  'eaddress',
] as const;

// Alice (config.alice) — owns the handles, delegates to Bob
// Bob (config.bob) — signs the delegated permit and decrypts

describe(
  'Decrypt client — delegated decrypt',
  () => {
    let config: FheTestEthersConfig;

    beforeAll(async () => {
      config = getEthersTestConfig();
      setFhevmRuntimeConfig({
        auth: {
          type: 'ApiKeyHeader',
          value: config.zamaApiKey,
        },
      });
      console.log(`  Alice: ${config.alice.wallet.address}`);
      console.log(`  Bob:   ${config.bob.wallet.address}`);

      const aclAddress = config.fhevmChain.fhevm.contracts.acl.address as ChecksummedAddress;

      // Check if delegation already exists
      const readClient = createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });

      const existingExpiration = await getUserDecryptionDelegationExpirationDate(readClient, {
        aclAddress,
        delegatorAddress: config.alice.wallet.address as ChecksummedAddress,
        delegateAddress: config.bob.wallet.address as ChecksummedAddress,
        contractAddress: config.fheTestAddress as ChecksummedAddress,
      });

      // Use block.timestamp instead of Date.now() — the expiration is based on
      // block.timestamp when the delegation tx was mined, not wall-clock time.
      const block = await config.provider.getBlock('latest');
      const blockTimestamp = BigInt(block!.timestamp);
      if (existingExpiration > blockTimestamp) {
        console.log(`  Delegation already active (expires ${existingExpiration}), skipping tx`);
      } else {
        console.log(`  Delegation not yet active, calling delegateForUserDecryption()...`);

        // Alice delegates decryption to Bob
        const callArgs = delegateForUserDecryption({
          aclAddress: config.fhevmChain.fhevm.contracts.acl.address,
          delegateAddress: config.bob.wallet.address,
          contractAddress: config.fheTestAddress,
          expirationDate: BigInt(Math.floor(Date.now() / 1000) + 86400 * 360),
        });

        const aclContract = new ethers.Contract(
          callArgs.address,
          callArgs.abi as ethers.InterfaceAbi,
          config.alice.signer,
        );
        const tx: ethers.TransactionResponse = await aclContract.getFunction(callArgs.functionName)(...callArgs.args);
        const receipt = await tx.wait();
        if (receipt!.status !== 1) {
          throw new Error(`Delegation tx failed: ${receipt!.hash}`);
        }
        console.log(`  Delegation tx: ${receipt!.hash}`);
      }
    });

    it('should sign a delegated decryption permit', async () => {
      const client = createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;

      const keypair = await client.generateTransportKeypair();

      // Bob signs a delegated permit to decrypt Alice's handles
      const signedPermit = await client.signDecryptionPermit({
        transportKeypair: keypair,
        contractAddresses: [config.fheTestAddress],
        durationDays: 1,
        startTimestamp: Math.floor(Date.now() / 1000),
        signerAddress: config.bob.wallet.address,
        signer: config.bob.signer,
        delegatorAddress: config.alice.wallet.address,
      });

      expect(signedPermit).toBeDefined();
      expect(signedPermit.isDelegated).toBe(true);
      expect(signedPermit.signerAddress.toLowerCase()).toBe(config.bob.wallet.address.toLowerCase());
      expect(signedPermit.encryptedDataOwnerAddress.toLowerCase()).toBe(config.alice.wallet.address.toLowerCase());
    });

    // ┌─────────────────────────────────────────────────────────────────────┐
    // │  Per-type delegated decrypt tests                                   │
    // │  For each FHE type:                                                 │
    // │  1. Read Alice's handle from FHETest.getHandleOf(alice, fheType)    │
    // │  2. Read the expected clear value from FHETest.getClearText(handle) │
    // │  3. Bob signs a delegated permit and decrypts                       │
    // │  4. Compare decrypted value with expected                           │
    // └─────────────────────────────────────────────────────────────────────┘

    for (const fheType of decryptTestCases) {
      it(`should decrypt ${fheType} via delegated decrypt`, async () => {
        const fheTypeId = fheTypeIdFromName(fheType);
        const fheTest = config.fheTestContract.connect(config.alice.signer) as ethers.Contract;

        // Read Alice's handle from FHETest contract
        const aliceHandle: Handle = toFhevmHandle(await fheTest.getHandleOf!(config.alice.wallet.address, fheTypeId));
        expect(aliceHandle.bytes32Hex).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');

        // Read expected clear value from FHETest._db
        const expectedRaw: bigint = await fheTest.getClearText!(aliceHandle.bytes32Hex);
        console.log(`  ${fheType}: handle=${aliceHandle.bytes32Hex.slice(0, 20)}... expected=${expectedRaw}`);

        // Bob decrypts Alice's handle via delegated permit
        const client = createFhevmDecryptClient({
          chain: config.fhevmChain,
          provider: config.provider,
        });
        await client.ready;

        const transportKeypair = await client.generateTransportKeypair();
        const bobSignedPermit = await client.signDecryptionPermit({
          transportKeypair,
          contractAddresses: [config.fheTestAddress],
          durationDays: 1,
          startTimestamp: Math.floor(Date.now() / 1000),
          signerAddress: config.bob.wallet.address,
          signer: config.bob.signer,
          delegatorAddress: config.alice.wallet.address,
        });

        const typedValue = await client.decryptValue({
          encryptedValue: aliceHandle.bytes32Hex,
          contractAddress: config.fheTestAddress as ChecksummedAddress,
          signedPermit: bobSignedPermit,
          transportKeypair,
        });

        expect(typedValue.type).toBe(aliceHandle.clearType);

        console.log(`  ${fheType}: decrypted=${typedValue.value} expected=${expectedRaw}`);

        if (fheType === 'ebool') {
          expect(typedValue.value).toBe(expectedRaw !== 0n);
        } else if (fheType === 'eaddress') {
          const expectedAddr = '0x' + expectedRaw.toString(16).padStart(40, '0');
          expect(String(typedValue.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
        } else {
          expect(BigInt(typedValue.value as number | bigint)).toBe(expectedRaw);
        }
      });
    }

    // ┌─────────────────────────────────────────────────────────────────────┐
    // │  All-at-once delegated decrypt test                                 │
    // │  Read all handles, decrypt in a single call, compare each result    │
    // └─────────────────────────────────────────────────────────────────────┘

    it('should decrypt all types in a single delegated call', async () => {
      const fheTest = config.fheTestContract.connect(config.alice.signer) as ethers.Contract;

      // Read all of Alice's handles and their expected clear values
      const aliceEntries: {
        aliceHandle: Handle;
        aliceClearValue: bigint;
      }[] = [];

      for (const fheType of decryptTestCases) {
        const fheTypeId = fheTypeIdFromName(fheType);
        const aliceHandle: Handle = toFhevmHandle(await fheTest.getHandleOf!(config.alice.wallet.address, fheTypeId));
        expect(aliceHandle.bytes32Hex).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        expect(aliceHandle.fheType).toBe(fheType);
        const aliceClearValue: bigint = await fheTest.getClearText!(aliceHandle.bytes32Hex);
        aliceEntries.push({ aliceHandle, aliceClearValue });
        console.log(`  ${fheType}: handle=${aliceHandle.bytes32Hex.slice(0, 20)}... expected=${aliceClearValue}`);
      }

      // Bob decrypts all of Alice's handles in a single call
      const bobClient = createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await bobClient.ready;

      const bobKeypair = await bobClient.generateTransportKeypair();
      const bobSignedPermit = await bobClient.signDecryptionPermit({
        transportKeypair: bobKeypair,
        contractAddresses: [config.fheTestAddress],
        durationDays: 1,
        startTimestamp: Math.floor(Date.now() / 1000),
        signerAddress: config.bob.wallet.address,
        signer: config.bob.signer,
        delegatorAddress: config.alice.wallet.address,
      });

      const aliceEncryptedValues = aliceEntries.map((e) => asEncryptedValue(e.aliceHandle));

      const bobDecryptedValues = await bobClient.decryptValues({
        contractAddress: config.fheTestAddress as ChecksummedAddress,
        encryptedValues: aliceEncryptedValues,
        signedPermit: bobSignedPermit,
        transportKeypair: bobKeypair,
      });

      expect(bobDecryptedValues).toHaveLength(aliceEntries.length);

      for (let i = 0; i < aliceEntries.length; i++) {
        const { aliceHandle, aliceClearValue } = aliceEntries[i]!;
        const bobDecrypted = bobDecryptedValues[i]!;
        console.log(`  ${aliceHandle.fheType}: bobDecrypted=${bobDecrypted.value} aliceExpected=${aliceClearValue}`);

        expect(bobDecrypted.type).toBe(aliceHandle.clearType);

        if (aliceHandle.fheType === 'ebool') {
          expect(bobDecrypted.value).toBe(aliceClearValue !== 0n);
        } else if (aliceHandle.fheType === 'eaddress') {
          const expectedAddr = '0x' + aliceClearValue.toString(16).padStart(40, '0');
          expect(String(bobDecrypted.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
        } else {
          expect(BigInt(bobDecrypted.value as number | bigint)).toBe(aliceClearValue);
        }
      }
    });
  },
  5 * 60_000,
);
