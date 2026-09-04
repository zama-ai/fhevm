import { asEncryptedValue, type EncryptedValue, type TypedValue } from '@fhevm/sdk/types';
import { ethers } from 'ethers';
import { describe, it, expect, beforeAll, beforeEach } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { canUseUnifiedDecryptionPermit, createUnsignedUnifiedDecryptionPermitEip712 } from '@fhevm/sdk/actions/base';
import { getEthersTestConfig, type CreateEthersDecryptClientFn, type FheTestEthersConfig } from '../setup-ethers.js';
import { decryptTestCases, fheTypeIdFromName, clearTypeFromHandle, createLogger } from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// Requires a protocol v14+ chain (KMSVerifier >= 0.4.0 + ProtocolConfig deployed):
//
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.unifiedPermit.test.ts
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

async function delegateForUserDecryption(parameters: {
  readonly aclAddress: string;
  readonly delegatorSigner: ethers.Signer;
  readonly delegateAddress: string;
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

export function defineClientDecryptUnifiedPermitTests(parameters: {
  readonly runIf: boolean;
  // Real deployed environments (testnet/devnet) can lag behind the chain's protocol version — the
  // relayer feature may not be rolled out yet even though the chain itself is on protocol v14+. Set
  // this to have the suite probe `canUseUnifiedDecryptionPermit()` once and skip all its tests if the
  // relayer doesn't actually support the unified route. Not needed for localstack chains, where
  // `protocolEraOf` already statically guarantees relayer support.
  readonly checkRelayerSupportsUnifiedPermit?: boolean;
  readonly createFhevmDecryptClient: CreateEthersDecryptClientFn;
}): void {
  describe.runIf(parameters.runIf)('Decrypt client — unified (V2) permit', () => {
    let config: FheTestEthersConfig;
    let logger: ReturnType<typeof createLogger>;
    let relayerSupportsUnifiedPermit = true;

    async function createReadyClient() {
      const client = parameters.createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;
      return client;
    }

    beforeAll(async () => {
      config = getEthersTestConfig();
      logger = createLogger(console.log, `${config.fhevmChain.id}`);
      setFhevmRuntimeConfig({
        auth: {
          type: 'ApiKeyHeader',
          value: config.zamaApiKey,
        },
        logger,
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
        signerAddress: config.wallet.address,
        signer: config.signer,
      });

      expect(signedPermit).toBeDefined();
      expect(signedPermit.version).toBe(2);
      expect(signedPermit.eip712.primaryType).toBe('UserDecryptRequestVerification');
      expect(signedPermit.isDelegated).toBe(false);
      expect(signedPermit.signerAddress.toLowerCase()).toBe(config.wallet.address.toLowerCase());
      expect(signedPermit.encryptedDataOwnerAddress.toLowerCase()).toBe(config.wallet.address.toLowerCase());
    });

    it('should serialize, parse and verify a unified decryption permit', async () => {
      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();

      const signedPermit = await client.signUnifiedDecryptionPermit({
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.wallet.address,
        signer: config.signer,
      });
      expect(signedPermit.version).toBe(2);

      const serialized = await client.serializeSignedDecryptionPermit({ signedPermit });
      expect(serialized.version).toBe(2);
      expect(serialized.eip712).toBeDefined();

      const parsed = await client.parseSignedDecryptionPermit({
        serializedPermit: serialized,
        transportKeyPair,
      });
      expect(parsed.version).toBe(2);
      expect(parsed.eip712.primaryType).toBe('UserDecryptRequestVerification');
      expect(parsed.signerAddress.toLowerCase()).toBe(config.wallet.address.toLowerCase());
    });

    it('signs and parses a manually-built unified permit (createUnsignedUnifiedDecryptionPermitEip712)', async () => {
      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();

      const eip712 = await createUnsignedUnifiedDecryptionPermitEip712(client, {
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.wallet.address,
      });

      // Strip EIP712Domain — ethers derives it from `domain`.
      const { EIP712Domain: _domainType, ...requestTypes } = eip712.types;
      const signature = await config.wallet.signTypedData(
        eip712.domain as ethers.TypedDataDomain,
        requestTypes as Record<string, ethers.TypedDataField[]>,
        eip712.message,
      );

      const permit = await client.parseSignedDecryptionPermit({
        serializedPermit: {
          version: 2,
          eip712,
          signature,
          signerAddress: config.wallet.address,
        },
        transportKeyPair,
      });

      expect(permit.version).toBe(2);
      expect(permit.eip712.primaryType).toBe('UserDecryptRequestVerification');
      expect(permit.signerAddress.toLowerCase()).toBe(config.wallet.address.toLowerCase());
    });

    it('signs and parses a manually-built DELEGATED unified permit (createUnsignedUnifiedDecryptionPermitEip712)', async () => {
      // Delegation is post-sign metadata, so it's attached to the permit object
      // handed to parseSignedDecryptionPermit rather than derived from the eip712.
      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();

      const eip712 = await createUnsignedUnifiedDecryptionPermitEip712(client, {
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.bob.wallet.address,
      });

      // Strip EIP712Domain — ethers derives it from `domain`.
      const { EIP712Domain: _domainType, ...requestTypes } = eip712.types;
      const signature = await config.bob.wallet.signTypedData(
        eip712.domain as ethers.TypedDataDomain,
        requestTypes as Record<string, ethers.TypedDataField[]>,
        eip712.message,
      );

      const permit = await client.parseSignedDecryptionPermit({
        serializedPermit: {
          version: 2,
          eip712,
          signature,
          signerAddress: config.bob.wallet.address,
          delegatorAddress: config.alice.wallet.address,
        },
        transportKeyPair,
      });

      expect(permit.version).toBe(2);
      expect(permit.isDelegated).toBe(true);
      expect(permit.signerAddress.toLowerCase()).toBe(config.bob.wallet.address.toLowerCase());
      expect(permit.encryptedDataOwnerAddress.toLowerCase()).toBe(config.alice.wallet.address.toLowerCase());
    });

    it('should serialize, parse and verify a DELEGATED unified decryption permit', async () => {
      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();

      const signedPermit = await client.signUnifiedDecryptionPermit({
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.bob.wallet.address,
        signer: config.bob.signer,
        delegatorAddress: config.alice.wallet.address,
      });
      expect(signedPermit.version).toBe(2);
      expect(signedPermit.isDelegated).toBe(true);

      const serialized = await client.serializeSignedDecryptionPermit({ signedPermit });
      expect(serialized.version).toBe(2);
      expect(serialized.delegatorAddress?.toLowerCase()).toBe(config.alice.wallet.address.toLowerCase());

      const parsed = await client.parseSignedDecryptionPermit({
        serializedPermit: serialized,
        transportKeyPair,
      });
      expect(parsed.version).toBe(2);
      expect(parsed.isDelegated).toBe(true);
      expect(parsed.signerAddress.toLowerCase()).toBe(config.bob.wallet.address.toLowerCase());
      expect(parsed.encryptedDataOwnerAddress.toLowerCase()).toBe(config.alice.wallet.address.toLowerCase());
    });

    it('rejects signing a DELEGATED unified permit when signerAddress equals delegatorAddress', async () => {
      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();

      await expect(
        client.signUnifiedDecryptionPermit({
          transportKeyPair,
          contractAddresses: [config.fheTestAddress],
          durationSeconds: 24 * 3600,
          startTimestamp: Math.floor(Date.now() / 1000) - 5,
          signerAddress: config.bob.wallet.address,
          signer: config.bob.signer,
          delegatorAddress: config.bob.wallet.address,
        }),
      ).rejects.toThrow('signerAddress and delegatorAddress must be different');
    });

    it('rejects parsing a DELEGATED unified permit when signerAddress equals delegatorAddress', async () => {
      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();
      const address = config.bob.wallet.address;

      await expect(
        client.parseSignedDecryptionPermit({
          serializedPermit: {
            version: 2,
            eip712: { primaryType: 'UserDecryptRequestVerification', domain: {}, types: {}, message: {} },
            signature: `0x${'11'.repeat(65)}`,
            signerAddress: address,
            delegatorAddress: address,
          },
          transportKeyPair,
        }),
      ).rejects.toThrow('signerAddress and delegatorAddress must be different');
    });

    // ┌─────────────────────────────────────────────────────────────────────┐
    // │  Per-type decrypt tests (V2 permit, routed through the v3 relayer   │
    // │  user-decrypt endpoint via fetchKmsSigncryptedSharesV2)             │
    // └─────────────────────────────────────────────────────────────────────┘

    for (const fheType of decryptTestCases) {
      it(`should decrypt ${fheType} with a unified permit and match on-chain clear text`, async () => {
        const fheTypeId = fheTypeIdFromName(fheType);
        const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

        const encryptedValue: EncryptedValue = asEncryptedValue(
          await fheTest.getHandleOf!(config.wallet.address, fheTypeId),
        );
        expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');

        const expectedRaw: bigint = await fheTest.getClearText!(encryptedValue);

        const client = await createReadyClient();
        const transportKeyPair = await client.generateTransportKeyPair();
        const signedPermit = await client.signUnifiedDecryptionPermit({
          transportKeyPair,
          contractAddresses: [config.fheTestAddress],
          durationSeconds: 24 * 3600,
          startTimestamp: Math.floor(Date.now() / 1000) - 5,
          signerAddress: config.wallet.address,
          signer: config.signer,
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
      const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;
      const entries: { fheType: string; handle: EncryptedValue; expectedRaw: bigint }[] = [];

      for (const fheType of decryptTestCases) {
        const fheTypeId = fheTypeIdFromName(fheType);
        const encryptedValue: EncryptedValue = asEncryptedValue(
          await fheTest.getHandleOf!(config.wallet.address, fheTypeId),
        );
        expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        const expectedRaw: bigint = await fheTest.getClearText!(encryptedValue);
        entries.push({ fheType, handle: encryptedValue, expectedRaw });
      }

      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();
      const signedPermit = await client.signUnifiedDecryptionPermit({
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.wallet.address,
        signer: config.signer,
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
      const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;
      const entries: { fheType: string; handle: EncryptedValue; expectedRaw: bigint }[] = [];

      for (const fheType of decryptTestCases) {
        const fheTypeId = fheTypeIdFromName(fheType);
        const encryptedValue: EncryptedValue = asEncryptedValue(
          await fheTest.getHandleOf!(config.wallet.address, fheTypeId),
        );
        expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        const expectedRaw: bigint = await fheTest.getClearText!(encryptedValue);
        entries.push({ fheType, handle: encryptedValue, expectedRaw });
      }

      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();
      const signedPermit = await client.signUnifiedDecryptionPermit({
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.wallet.address,
        signer: config.signer,
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

          const existingExpiration = await getUserDecryptionDelegationExpirationDate({
            aclAddress: config.fhevmChain.fhevm.contracts.acl.address,
            provider: config.provider,
            delegatorAddress: config.alice.wallet.address,
            delegateAddress: config.bob.wallet.address,
            contractAddress: config.fheTestAddress,
          });

          const block = await config.provider.getBlock('latest');
          const blockTimestamp = BigInt(block!.timestamp);
          if (existingExpiration > blockTimestamp) {
            return;
          }

          const receipt = await delegateForUserDecryption({
            aclAddress: config.fhevmChain.fhevm.contracts.acl.address,
            delegatorSigner: config.alice.signer,
            delegateAddress: config.bob.wallet.address,
            contractAddress: config.fheTestAddress,
            durationSeconds: 86400 * 360,
          });
          if (receipt.status !== 1) {
            throw new Error(`Delegation tx failed: ${receipt.hash}`);
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
            signerAddress: config.bob.wallet.address,
            signer: config.bob.signer,
            delegatorAddress: config.alice.wallet.address,
          });

          expect(signedPermit.version).toBe(2);
          expect(signedPermit.isDelegated).toBe(true);
          expect(signedPermit.signerAddress.toLowerCase()).toBe(config.bob.wallet.address.toLowerCase());
          expect(signedPermit.encryptedDataOwnerAddress.toLowerCase()).toBe(config.alice.wallet.address.toLowerCase());
        });

        it('decrypts Alice handles via a Bob-signed delegated unified permit', async () => {
          const fheType = decryptTestCases[0]!;
          const fheTypeId = fheTypeIdFromName(fheType);
          const fheTest = config.fheTestContract.connect(config.alice.signer) as ethers.Contract;

          const aliceHandle: EncryptedValue = asEncryptedValue(
            await fheTest.getHandleOf!(config.alice.wallet.address, fheTypeId),
          );
          expect(aliceHandle).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');

          const expectedRaw: bigint = await fheTest.getClearText!(aliceHandle);

          const client = await createReadyClient();
          const transportKeyPair = await client.generateTransportKeyPair();
          const bobSignedPermit = await client.signUnifiedDecryptionPermit({
            transportKeyPair,
            contractAddresses: [config.fheTestAddress],
            durationSeconds: 24 * 3600,
            startTimestamp: Math.floor(Date.now() / 1000) - 5,
            signerAddress: config.bob.wallet.address,
            signer: config.bob.signer,
            delegatorAddress: config.alice.wallet.address,
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
