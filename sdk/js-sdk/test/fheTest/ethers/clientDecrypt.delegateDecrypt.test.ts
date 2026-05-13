import { asEncryptedValue, type EncryptedValue } from '@fhevm/sdk/types';
import { ethers } from 'ethers';
import { describe, it, expect, beforeAll } from 'vitest';
import { createFhevmDecryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { getEthersTestConfig, type FheTestEthersConfig } from './setup.js';
import {
  decryptTestCases,
  isCleartext,
  fheTypeIdFromName,
  clearTypeFromHandle,
  fheTypeIdFromHandle,
} from '../setupCommon.js';

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
// localstack:
// ----------------
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.delegateDecrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

// Alice (config.alice) — owns the handles, delegates to Bob
// Bob (config.bob) — signs the delegated permit and decrypts

const ACL_DELEGATE_ABI = [
  {
    inputs: [
      { internalType: 'address', name: 'delegate', type: 'address' },
      { internalType: 'address', name: 'contractAddress', type: 'address' },
      { internalType: 'uint64', name: 'expirationDate', type: 'uint64' },
    ],
    name: 'delegateForUserDecryption',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      { internalType: 'address', name: 'delegator', type: 'address' },
      { internalType: 'address', name: 'delegate', type: 'address' },
      { internalType: 'address', name: 'contractAddress', type: 'address' },
    ],
    name: 'getUserDecryptionDelegationExpirationDate',
    outputs: [{ internalType: 'uint64', name: '', type: 'uint64' }],
    stateMutability: 'view',
    type: 'function',
  },
] as const;

/**
 * Alice calls `ACL.delegateForUserDecryption` on-chain, granting `delegate`
 * permission to decrypt her handles on `contractAddress` until `expirationDate`.
 */
async function delegateForUserDecryption(parameters: {
  readonly aclAddress: string;
  readonly delegatorSigner: ethers.Signer; // Alice
  readonly delegateAddress: string; // Bob
  readonly contractAddress: string;
  readonly durationSeconds: number;
}): Promise<ethers.TransactionReceipt> {
  const aclContract = new ethers.Contract(parameters.aclAddress, ACL_DELEGATE_ABI, parameters.delegatorSigner);

  const expirationDate = Math.floor(Date.now() / 1000) + parameters.durationSeconds;

  const tx = await aclContract.getFunction('delegateForUserDecryption')(
    parameters.delegateAddress,
    parameters.contractAddress,
    expirationDate,
  );
  return tx.wait();
}

/**
 * Reads the expiration date of a delegation from the ACL contract.
 * Returns `0n` if no delegation exists.
 */
async function getUserDecryptionDelegationExpirationDate(parameters: {
  readonly aclAddress: string;
  readonly provider: ethers.Provider;
  readonly delegatorAddress: string;
  readonly delegateAddress: string;
  readonly contractAddress: string;
}): Promise<bigint> {
  const aclContract = new ethers.Contract(parameters.aclAddress, ACL_DELEGATE_ABI, parameters.provider);

  return aclContract.getFunction('getUserDecryptionDelegationExpirationDate')(
    parameters.delegatorAddress,
    parameters.delegateAddress,
    parameters.contractAddress,
  );
}

describe.runIf(!isCleartext(getEthersTestConfig().chainName))(
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

      // Check if delegation already exists
      const existingExpiration = await getUserDecryptionDelegationExpirationDate({
        aclAddress: config.fhevmChain.fhevm.contracts.acl.address,
        provider: config.provider,
        delegatorAddress: config.alice.wallet.address,
        delegateAddress: config.bob.wallet.address,
        contractAddress: config.fheTestAddress,
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
        const receipt = await delegateForUserDecryption({
          aclAddress: config.fhevmChain.fhevm.contracts.acl.address,
          delegatorSigner: config.alice.signer,
          delegateAddress: config.bob.wallet.address,
          contractAddress: config.fheTestAddress,
          durationSeconds: 86400 * 360, // a bit less than a year
        });
        if (receipt.status !== 1) {
          throw new Error(`Delegation tx failed: ${receipt.hash}`);
        }
        console.log(`  Delegation tx: ${receipt.hash}`);
      }
    });

    it('should sign a delegated decryption permit', async () => {
      const client = createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;

      const keyPair = await client.generateTransportKeyPair();

      // Bob signs a delegated permit to decrypt Alice's handles
      const signedPermit = await client.signDecryptionPermit({
        transportKeyPair: keyPair,
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
        const aliceHandle: EncryptedValue = asEncryptedValue(
          await fheTest.getHandleOf!(config.alice.wallet.address, fheTypeId),
        );
        expect(aliceHandle).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');

        // Read expected clear value from FHETest._db
        const expectedRaw: bigint = await fheTest.getClearText!(aliceHandle);
        console.log(`  ${fheType}: handle=${aliceHandle.slice(0, 20)}... expected=${expectedRaw}`);

        // Bob decrypts Alice's handle via delegated permit
        const client = createFhevmDecryptClient({
          chain: config.fhevmChain,
          provider: config.provider,
        });
        await client.ready;

        const transportKeyPair = await client.generateTransportKeyPair();
        const bobSignedPermit = await client.signDecryptionPermit({
          transportKeyPair: transportKeyPair,
          contractAddresses: [config.fheTestAddress],
          durationDays: 1,
          startTimestamp: Math.floor(Date.now() / 1000),
          signerAddress: config.bob.wallet.address,
          signer: config.bob.signer,
          delegatorAddress: config.alice.wallet.address,
        });

        const typedValue = await client.decryptValue({
          encryptedValue: aliceHandle,
          contractAddress: config.fheTestAddress,
          signedPermit: bobSignedPermit,
          transportKeyPair: transportKeyPair,
        });

        expect(typedValue.type).toBe(clearTypeFromHandle(aliceHandle));

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
        aliceHandle: EncryptedValue;
        aliceClearValue: bigint;
      }[] = [];

      for (const fheType of decryptTestCases) {
        const fheTypeId = fheTypeIdFromName(fheType);
        const aliceHandle: EncryptedValue = asEncryptedValue(
          await fheTest.getHandleOf!(config.alice.wallet.address, fheTypeId),
        );
        expect(aliceHandle).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        expect(fheTypeIdFromHandle(aliceHandle)).toBe(fheTypeIdFromName(fheType));
        const aliceClearValue: bigint = await fheTest.getClearText!(aliceHandle);
        aliceEntries.push({ aliceHandle, aliceClearValue });
        console.log(`  ${fheType}: handle=${aliceHandle.slice(0, 20)}... expected=${aliceClearValue}`);
      }

      // Bob decrypts all of Alice's handles in a single call
      const bobClient = createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await bobClient.ready;

      const bobKeyPair = await bobClient.generateTransportKeyPair();
      const bobSignedPermit = await bobClient.signDecryptionPermit({
        transportKeyPair: bobKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationDays: 1,
        startTimestamp: Math.floor(Date.now() / 1000),
        signerAddress: config.bob.wallet.address,
        signer: config.bob.signer,
        delegatorAddress: config.alice.wallet.address,
      });

      const aliceEncryptedValues = aliceEntries.map((e) => asEncryptedValue(e.aliceHandle));

      const bobDecryptedValues = await bobClient.decryptValues({
        contractAddress: config.fheTestAddress,
        encryptedValues: aliceEncryptedValues,
        signedPermit: bobSignedPermit,
        transportKeyPair: bobKeyPair,
      });

      expect(bobDecryptedValues).toHaveLength(aliceEntries.length);

      for (let i = 0; i < aliceEntries.length; i++) {
        const { aliceHandle, aliceClearValue } = aliceEntries[i]!;
        const bobDecrypted = bobDecryptedValues[i]!;
        console.log(
          `  ${clearTypeFromHandle(aliceHandle)}: bobDecrypted=${bobDecrypted.value} aliceExpected=${aliceClearValue}`,
        );

        expect(bobDecrypted.type).toBe(clearTypeFromHandle(aliceHandle));

        if (clearTypeFromHandle(aliceHandle) === 'ebool') {
          expect(bobDecrypted.value).toBe(aliceClearValue !== 0n);
        } else if (clearTypeFromHandle(aliceHandle) === 'eaddress') {
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
