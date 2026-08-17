import type { Account, Chain, Hex, PublicClient, Transport } from 'viem';
import { describe, it, expect, beforeAll, beforeEach } from 'vitest';
import { createWalletClient, http } from 'viem';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { canUseUnifiedDecryptionPermit, createUnsignedUnifiedDecryptionPermitEip712 } from '@fhevm/sdk/actions/base';
import { asEncryptedValue, type EncryptedValue, type TypedValue } from '@fhevm/sdk/types';
import { getViemTestConfig, type CreateViemDecryptClientFn, type FheTestViemConfig } from '../setup-viem.js';
import { FHETestABI } from '../FheTest-abi-v2.js';
import {
  decryptTestCases,
  fheTypeIdFromName,
  clearTypeFromHandle,
  createLogger,
  prepareSingleChain,
} from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// Requires a protocol v14+ chain (KMSVerifier >= 0.4.0 + ProtocolConfig deployed):
//
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.unifiedPermit.test.ts
//
////////////////////////////////////////////////////////////////////////////////

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

export function defineClientDecryptUnifiedPermitTests(parameters: {
  readonly runIf: boolean;
  // Real deployed environments (testnet/devnet) can lag behind the chain's protocol version — the
  // relayer feature may not be rolled out yet even though the chain itself is on protocol v14+. Set
  // this to have the suite probe `canUseUnifiedDecryptionPermit()` once and skip all its tests if the
  // relayer doesn't actually support the unified route. Not needed for localstack chains, where
  // `protocolEraOf` already statically guarantees relayer support.
  readonly checkRelayerSupportsUnifiedPermit?: boolean;
  readonly createFhevmDecryptClient: CreateViemDecryptClientFn;
}): void {
  describe.runIf(parameters.runIf)('Decrypt client — unified (V2) permit', () => {
    let config: FheTestViemConfig;
    let relayerSupportsUnifiedPermit = true;

    async function createReadyClient() {
      const client = parameters.createFhevmDecryptClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });
      await client.ready;
      return client;
    }

    beforeAll(async () => {
      config = getViemTestConfig();
      setFhevmRuntimeConfig({
        auth: {
          type: 'ApiKeyHeader',
          value: config.zamaApiKey,
        },
        logger: createLogger(console.log),
      });

      if (parameters.checkRelayerSupportsUnifiedPermit) {
        const client = await createReadyClient();
        relayerSupportsUnifiedPermit = await canUseUnifiedDecryptionPermit(client, {
          options: { auth: { type: 'ApiKeyHeader', value: config.zamaApiKey } },
        });
      }
    });

    beforeEach((ctx) => {
      ctx.skip(!relayerSupportsUnifiedPermit, 'relayer does not support the unified (V2) user-decrypt route yet');
    });

    it('should sign a self unified decryption permit', async () => {
      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();

      const signedPermit = await client.signUnifiedDecryptionPermit({
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.account.address,
        signer: config.account,
      });

      expect(signedPermit).toBeDefined();
      expect(signedPermit.version).toBe(2);
      expect(signedPermit.eip712.primaryType).toBe('UserDecryptRequestVerification');
      expect(signedPermit.isDelegated).toBe(false);
      expect(signedPermit.signerAddress.toLowerCase()).toBe(config.account.address.toLowerCase());
      expect(signedPermit.encryptedDataOwnerAddress.toLowerCase()).toBe(config.account.address.toLowerCase());
    });

    it('should serialize, parse and verify a unified decryption permit', async () => {
      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();

      const signedPermit = await client.signUnifiedDecryptionPermit({
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.account.address,
        signer: config.account,
      });
      expect(signedPermit.version).toBe(2);

      const serialized = await client.serializeSignedDecryptionPermit({ signedPermit });
      expect(serialized.version).toBe(2);
      expect(serialized.eip712).toBeDefined();

      // parseSignedDecryptionPermit re-validates the permit (EIP-712 signature +
      // transport key pair binding) — parsing IS the verification step.
      const parsed = await client.parseSignedDecryptionPermit({
        serializedPermit: serialized,
        transportKeyPair,
      });
      expect(parsed.version).toBe(2);
      expect(parsed.eip712.primaryType).toBe('UserDecryptRequestVerification');
      expect(parsed.signerAddress.toLowerCase()).toBe(config.account.address.toLowerCase());
    });

    it('signs and parses a manually-built unified permit (createUnsignedUnifiedDecryptionPermitEip712)', async () => {
      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();

      const eip712 = await createUnsignedUnifiedDecryptionPermitEip712(client, {
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.account.address,
      });

      const signature = await config.account.signTypedData({
        domain: eip712.domain,
        types: eip712.types,
        primaryType: 'UserDecryptRequestVerification',
        message: eip712.message,
      } as Parameters<typeof config.account.signTypedData>[0]);

      const permit = await client.parseSignedDecryptionPermit({
        serializedPermit: {
          version: 2,
          eip712,
          signature,
          signerAddress: config.account.address,
        },
        transportKeyPair,
      });

      expect(permit.version).toBe(2);
      expect(permit.eip712.primaryType).toBe('UserDecryptRequestVerification');
      expect(permit.signerAddress.toLowerCase()).toBe(config.account.address.toLowerCase());
    });

    // ┌─────────────────────────────────────────────────────────────────────┐
    // │  Per-type decrypt tests (V2 permit, routed through the v3 relayer   │
    // │  user-decrypt endpoint via fetchKmsSigncryptedSharesV2)             │
    // └─────────────────────────────────────────────────────────────────────┘

    for (const fheType of decryptTestCases) {
      it(`should decrypt ${fheType} with a unified permit and match on-chain clear text`, async () => {
        const fheTypeId = fheTypeIdFromName(fheType);

        const encryptedValue: EncryptedValue = asEncryptedValue(
          await config.publicClient.readContract({
            address: config.fheTestAddress as Hex,
            abi: FHETestABI,
            functionName: 'getHandleOf',
            args: [config.account.address, fheTypeId],
          }),
        );
        expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');

        const expectedRaw = await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getClearText',
          args: [encryptedValue],
        });

        const client = await createReadyClient();
        const transportKeyPair = await client.generateTransportKeyPair();
        const signedPermit = await client.signUnifiedDecryptionPermit({
          transportKeyPair,
          contractAddresses: [config.fheTestAddress],
          durationSeconds: 24 * 3600,
          startTimestamp: Math.floor(Date.now() / 1000) - 5,
          signerAddress: config.account.address,
          signer: config.account,
        });

        const typedValue = await client.decryptValue({
          contractAddress: config.fheTestAddress,
          encryptedValue,
          signedPermit,
          transportKeyPair,
        });

        expect(typedValue.type).toBe(clearTypeFromHandle(encryptedValue));

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
    // │  All-at-once decrypt, and decryptValuesFromPairs                    │
    // └─────────────────────────────────────────────────────────────────────┘

    it('should decrypt all types in a single call with a unified permit', async () => {
      const entries: { fheType: string; handle: EncryptedValue; expectedRaw: bigint }[] = [];

      for (const fheType of decryptTestCases) {
        const fheTypeId = fheTypeIdFromName(fheType);
        const encryptedValue: EncryptedValue = asEncryptedValue(
          await config.publicClient.readContract({
            address: config.fheTestAddress as Hex,
            abi: FHETestABI,
            functionName: 'getHandleOf',
            args: [config.account.address, fheTypeId],
          }),
        );
        expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');

        const expectedRaw = await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getClearText',
          args: [encryptedValue],
        });
        entries.push({ fheType, handle: encryptedValue, expectedRaw });
      }

      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();
      const signedPermit = await client.signUnifiedDecryptionPermit({
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.account.address,
        signer: config.account,
      });

      const encryptedValues = entries.map((e) => asEncryptedValue(e.handle));

      const typedValues: readonly TypedValue[] = await client.decryptValues({
        encryptedValues,
        contractAddress: config.fheTestAddress,
        signedPermit,
        transportKeyPair,
      });

      expect(typedValues).toHaveLength(entries.length);

      for (let i = 0; i < entries.length; i++) {
        const { fheType, expectedRaw } = entries[i]!;
        const decrypted = typedValues[i]!;

        if (fheType === 'ebool') {
          expect(decrypted.value).toBe(expectedRaw !== 0n);
        } else if (fheType === 'eaddress') {
          const expectedAddr = '0x' + expectedRaw.toString(16).padStart(40, '0');
          expect(String(decrypted.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
        } else {
          expect(BigInt(decrypted.value as number | bigint)).toBe(expectedRaw);
        }
      }
    });

    it('should decrypt all types via decryptValuesFromPairs with a unified permit', async () => {
      const entries: { fheType: string; handle: EncryptedValue; expectedRaw: bigint }[] = [];

      for (const fheType of decryptTestCases) {
        const fheTypeId = fheTypeIdFromName(fheType);
        const encryptedValue: EncryptedValue = asEncryptedValue(
          await config.publicClient.readContract({
            address: config.fheTestAddress as Hex,
            abi: FHETestABI,
            functionName: 'getHandleOf',
            args: [config.account.address, fheTypeId],
          }),
        );
        expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');

        const expectedRaw = await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getClearText',
          args: [encryptedValue],
        });
        entries.push({ fheType, handle: encryptedValue, expectedRaw });
      }

      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();
      const signedPermit = await client.signUnifiedDecryptionPermit({
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.account.address,
        signer: config.account,
      });

      const pairs = entries.map((e) => ({
        encryptedValue: asEncryptedValue(e.handle),
        contractAddress: config.fheTestAddress,
      }));

      const typedValues: readonly TypedValue[] = await client.decryptValuesFromPairs({
        pairs,
        signedPermit,
        transportKeyPair,
      });

      expect(typedValues).toHaveLength(entries.length);

      for (let i = 0; i < entries.length; i++) {
        const { fheType, expectedRaw } = entries[i]!;
        const decrypted = typedValues[i]!;

        if (fheType === 'ebool') {
          expect(decrypted.value).toBe(expectedRaw !== 0n);
        } else if (fheType === 'eaddress') {
          const expectedAddr = '0x' + expectedRaw.toString(16).padStart(40, '0');
          expect(String(decrypted.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
        } else {
          expect(BigInt(decrypted.value as number | bigint)).toBe(expectedRaw);
        }
      }
    });

    // ┌─────────────────────────────────────────────────────────────────────┐
    // │  Delegated unified decrypt (Alice delegates to Bob)                 │
    // │  V2 delegation is post-sign metadata: the signed message always     │
    // │  embeds the signer's own address as `userAddress`, and             │
    // │  `encryptedDataOwnerAddress` (delegatorAddress) is what             │
    // │  decryptValue/decryptValues forward as `ownerAddress` to the        │
    // │  relayer — see decryptValuesFromPairs.ts.                          │
    // └─────────────────────────────────────────────────────────────────────┘

    describe(
      'delegated unified decrypt (alice delegates to bob)',
      () => {
        beforeAll(async () => {
          // Skip the on-chain delegation setup too — its tests are about to be skipped by the
          // `beforeEach` above, and there's no point spending a transaction for nothing.
          if (!relayerSupportsUnifiedPermit) {
            return;
          }

          const aclAddress = config.fhevmChain.fhevm.contracts.acl.address as Hex;
          const existingExpiration = (await config.publicClient.readContract({
            address: aclAddress,
            abi: ACL_DELEGATE_ABI,
            functionName: 'getUserDecryptionDelegationExpirationDate',
            args: [config.alice.account.address, config.bob.account.address, config.fheTestAddress as Hex],
          })) as bigint;

          const block = await config.publicClient.getBlock();
          if (existingExpiration > block.timestamp) {
            return;
          }

          const walletClient = createWalletClient({
            account: config.alice.account as Account,
            chain: (config.publicClient as PublicClient<Transport, Chain>).chain,
            transport: http(prepareSingleChain().rpcUrl),
          });
          const hash = await walletClient.writeContract({
            address: aclAddress,
            abi: ACL_DELEGATE_ABI,
            functionName: 'delegateForUserDecryption',
            args: [
              config.bob.account.address,
              config.fheTestAddress as Hex,
              BigInt(Math.floor(Date.now() / 1000) + 86400 * 360),
            ],
          });
          const receipt = await config.publicClient.waitForTransactionReceipt({ hash });
          if (receipt.status !== 'success') {
            throw new Error(`Delegation tx failed: ${receipt.transactionHash}`);
          }
          // Wait for the delegation to propagate to the gateway's MultichainACL.
          await new Promise((r) => setTimeout(r, 15000));
        }, 120_000);

        it('signs a delegated unified permit (userAddress is the signer, not the delegator)', async () => {
          const client = await createReadyClient();
          const transportKeyPair = await client.generateTransportKeyPair();

          const signedPermit = await client.signUnifiedDecryptionPermit({
            transportKeyPair,
            contractAddresses: [config.fheTestAddress],
            durationSeconds: 24 * 3600,
            startTimestamp: Math.floor(Date.now() / 1000) - 5,
            signerAddress: config.bob.account.address,
            signer: config.bob.account,
            delegatorAddress: config.alice.account.address,
          });

          expect(signedPermit.version).toBe(2);
          expect(signedPermit.isDelegated).toBe(true);
          expect(signedPermit.signerAddress.toLowerCase()).toBe(config.bob.account.address.toLowerCase());
          expect(signedPermit.encryptedDataOwnerAddress.toLowerCase()).toBe(config.alice.account.address.toLowerCase());
        });

        it('decrypts Alice handles via a Bob-signed delegated unified permit', async () => {
          const fheType = decryptTestCases[0]!;
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

          const expectedRaw = await config.publicClient.readContract({
            address: config.fheTestAddress as Hex,
            abi: FHETestABI,
            functionName: 'getClearText',
            args: [aliceHandle],
          });

          const client = await createReadyClient();
          const transportKeyPair = await client.generateTransportKeyPair();
          const bobSignedPermit = await client.signUnifiedDecryptionPermit({
            transportKeyPair,
            contractAddresses: [config.fheTestAddress],
            durationSeconds: 24 * 3600,
            startTimestamp: Math.floor(Date.now() / 1000) - 5,
            signerAddress: config.bob.account.address,
            signer: config.bob.account,
            delegatorAddress: config.alice.account.address,
          });

          const typedValue = await client.decryptValue({
            encryptedValue: aliceHandle,
            contractAddress: config.fheTestAddress,
            signedPermit: bobSignedPermit,
            transportKeyPair,
          });

          expect(typedValue.type).toBe(clearTypeFromHandle(aliceHandle));
          if (fheType === 'ebool') {
            expect(typedValue.value).toBe(expectedRaw !== 0n);
          } else if (fheType === 'eaddress') {
            const expectedAddr = '0x' + expectedRaw.toString(16).padStart(40, '0');
            expect(String(typedValue.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
          } else {
            expect(BigInt(typedValue.value as number | bigint)).toBe(expectedRaw);
          }
        });
      },
      5 * 60_000,
    );
  });
}
