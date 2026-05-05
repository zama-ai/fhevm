import type { Account, Hex, PublicClient, Transport, Chain } from 'viem';
import { describe, it, expect, beforeAll } from 'vitest';
import { asEncryptedValue, type EncryptedValue } from '@fhevm/sdk/types';
import { createFhevmDecryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { getViemTestConfig, type FheTestViemConfig } from './setup.js';
import { FHETestABI } from '../abi-v2.js';
import { createWalletClient, http } from 'viem';
import {
  decryptTestCases,
  isCleartext,
  fheTypeIdFromName,
  clearTypeFromHandle,
  getBaseEnv,
  isV2,
  fheTypeIdFromHandle,
} from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// Sepolia Testnet:
// ----------------
// npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.delegateDecrypt
//
// Devnet:
// -------
// CHAIN=devnet npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.delegateDecrypt
//
// localhost fhevm:
// ----------------
// CHAIN=localhostFhevm npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.delegateDecrypt
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
  readonly aclAddress: Hex;
  readonly delegatorAccount: Account; // Alice
  readonly delegateAddress: Hex; // Bob
  readonly contractAddress: Hex;
  readonly durationSeconds: number;
  readonly publicClient: PublicClient<Transport, Chain>;
}) {
  const walletClient = createWalletClient({
    account: parameters.delegatorAccount,
    chain: parameters.publicClient.chain,
    transport: http(getBaseEnv().rpcUrl),
  });

  const expirationDate = BigInt(Math.floor(Date.now() / 1000) + parameters.durationSeconds);

  const hash = await walletClient.writeContract({
    address: parameters.aclAddress,
    abi: ACL_DELEGATE_ABI,
    functionName: 'delegateForUserDecryption',
    args: [parameters.delegateAddress, parameters.contractAddress, expirationDate],
  });

  return parameters.publicClient.waitForTransactionReceipt({ hash });
}

/**
 * Reads the expiration date of a delegation from the ACL contract.
 * Returns `0n` if no delegation exists.
 */
async function getUserDecryptionDelegationExpirationDate(parameters: {
  readonly aclAddress: Hex;
  readonly publicClient: PublicClient<Transport, Chain>;
  readonly delegatorAddress: Hex;
  readonly delegateAddress: Hex;
  readonly contractAddress: Hex;
}): Promise<bigint> {
  return parameters.publicClient.readContract({
    address: parameters.aclAddress,
    abi: ACL_DELEGATE_ABI,
    functionName: 'getUserDecryptionDelegationExpirationDate',
    args: [parameters.delegatorAddress, parameters.delegateAddress, parameters.contractAddress],
  });
}

describe.runIf(isV2(getViemTestConfig().chainName) && !isCleartext(getViemTestConfig().chainName))(
  'Decrypt client — delegated decrypt',
  () => {
    let config: FheTestViemConfig;

    beforeAll(async () => {
      config = getViemTestConfig();
      setFhevmRuntimeConfig({
        auth: {
          type: 'ApiKeyHeader',
          value: config.zamaApiKey,
        },
      });
      console.log(`  Alice: ${config.alice.account.address}`);
      console.log(`  Bob:   ${config.bob.account.address}`);

      // Check if delegation already exists
      const aclAddress = config.fhevmChain.fhevm.contracts.acl.address as Hex;
      const existingExpiration = await getUserDecryptionDelegationExpirationDate({
        aclAddress,
        publicClient: config.publicClient,
        delegatorAddress: config.alice.account.address,
        delegateAddress: config.bob.account.address,
        contractAddress: config.fheTestAddress as Hex,
      });

      // Use block.timestamp instead of Date.now() — the expiration is based on
      // block.timestamp when the delegation tx was mined, not wall-clock time.
      const block = await config.publicClient.getBlock();
      const blockTimestamp = block.timestamp;
      if (existingExpiration > blockTimestamp) {
        console.log(`  Delegation already active (expires ${existingExpiration}), skipping tx`);
      } else {
        console.log(`  Delegation not yet active, calling delegateForUserDecryption()...`);
        // Alice delegates decryption to Bob
        const receipt = await delegateForUserDecryption({
          aclAddress,
          delegatorAccount: config.alice.account,
          delegateAddress: config.bob.account.address,
          contractAddress: config.fheTestAddress as Hex,
          durationSeconds: 86400 * 360, // a bit less than a year
          publicClient: config.publicClient,
        });
        if (receipt.status !== 'success') {
          throw new Error(`Delegation tx failed: ${receipt.transactionHash}`);
        }
        console.log(`  Delegation tx: ${receipt.transactionHash}`);
      }
    });

    it('should sign a delegated decryption permit', async () => {
      const client = createFhevmDecryptClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });
      await client.ready;

      const keyPair = await client.generateTransportKeyPair();

      // Bob signs a delegated permit to decrypt Alice's handles
      const signedPermit = await client.signDecryptionPermit({
        transportKeyPair: keyPair,
        contractAddresses: [config.fheTestAddress],
        durationDays: 1,
        startTimestamp: Math.floor(Date.now() / 1000),
        signerAddress: config.bob.account.address,
        signer: config.bob.account,
        delegatorAddress: config.alice.account.address,
      });

      expect(signedPermit).toBeDefined();
      expect(signedPermit.isDelegated).toBe(true);
      expect(signedPermit.signerAddress.toLowerCase()).toBe(config.bob.account.address.toLowerCase());
      expect(signedPermit.encryptedDataOwnerAddress.toLowerCase()).toBe(config.alice.account.address.toLowerCase());
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

        // Read Alice's handle from FHETest contract
        const aliceHandle: EncryptedValue = asEncryptedValue(
          await config.publicClient.readContract({
            address: config.fheTestAddress as Hex,
            abi: FHETestABI,
            functionName: 'getHandleOf',
            args: [config.alice.account.address, fheTypeId],
          }),
        );
        expect(aliceHandle).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');

        // Read expected clear value from FHETest._db
        const expectedRaw = await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getClearText',
          args: [aliceHandle],
        });
        console.log(`  ${fheType}: handle=${aliceHandle.slice(0, 20)}... expected=${expectedRaw}`);

        // Bob decrypts Alice's handle via delegated permit
        const client = createFhevmDecryptClient({
          chain: config.fhevmChain,
          publicClient: config.publicClient,
        });
        await client.ready;

        const transportKeyPair = await client.generateTransportKeyPair();
        const bobSignedPermit = await client.signDecryptionPermit({
          transportKeyPair: transportKeyPair,
          contractAddresses: [config.fheTestAddress],
          durationDays: 1,
          startTimestamp: Math.floor(Date.now() / 1000),
          signerAddress: config.bob.account.address,
          signer: config.bob.account,
          delegatorAddress: config.alice.account.address,
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
      // Read all of Alice's handles and their expected clear values
      const aliceEntries: {
        aliceHandle: EncryptedValue;
        aliceClearValue: bigint;
      }[] = [];

      for (const fheType of decryptTestCases) {
        const fheTypeId = fheTypeIdFromName(fheType);
        const aliceHandle: EncryptedValue = asEncryptedValue(
          await config.publicClient.readContract({
            address: config.fheTestAddress as Hex,
            abi: FHETestABI,
            functionName: 'getHandleOf',
            args: [config.alice.account.address, fheTypeId],
          }),
        );
        expect(aliceHandle).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        expect(fheTypeIdFromHandle(aliceHandle)).toBe(fheTypeIdFromName(fheType));
        const aliceClearValue = await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getClearText',
          args: [aliceHandle],
        });
        aliceEntries.push({ aliceHandle, aliceClearValue });
        console.log(`  ${fheType}: handle=${aliceHandle.slice(0, 20)}... expected=${aliceClearValue}`);
      }

      // Bob decrypts all of Alice's handles in a single call
      const bobClient = createFhevmDecryptClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });
      await bobClient.ready;

      const bobKeyPair = await bobClient.generateTransportKeyPair();
      const bobSignedPermit = await bobClient.signDecryptionPermit({
        transportKeyPair: bobKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationDays: 1,
        startTimestamp: Math.floor(Date.now() / 1000),
        signerAddress: config.bob.account.address,
        signer: config.bob.account,
        delegatorAddress: config.alice.account.address,
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
          `  ${fheTypeIdFromHandle(aliceHandle)}: bobDecrypted=${bobDecrypted.value} aliceExpected=${aliceClearValue}`,
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
