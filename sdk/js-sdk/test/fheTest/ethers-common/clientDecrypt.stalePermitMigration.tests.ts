import { ethers } from 'ethers';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { getEthersTestConfig, type CreateEthersDecryptClientFn, type FheTestEthersConfig } from '../setup-ethers.js';
import { decryptTestCases, fheTypeIdFromName, clearTypeFromHandle, createLogger } from '../setupCommon.js';
import { asEncryptedValue, type EncryptedValue, type TypedValue } from '@fhevm/sdk/types';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack_v12 npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.stalePermitMigration.test.ts
// CHAIN=localstack_v13 npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.stalePermitMigration.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.stalePermitMigration.test.ts
//
// Not applicable to localstack_v11: on protocol v0.11 the current KMS context
// already encodes to extraData v0 (0x00), so a "stale v11 permit" is
// indistinguishable from a fresh one.
//
////////////////////////////////////////////////////////////////////////////////

// A permit is cached client-side (e.g. localStorage) for its whole validity
// window (up to 365 days), so it legitimately comes back after the chain has
// migrated (v11 -> v12 -> v13 -> v14). Its EIP-712 message embeds the
// extraData encoding of the era it was signed on, and the signature covers it,
// so the SDK cannot rewrite it — the decrypt path must accept the old encoding
// and resolve the KMS signers context it refers to (extraData v0 = "current
// context" sentinel, mirroring KMSVerifier._extractKmsContextId and the
// gateway Decryption._extractContextId).
//
// v11-era permits embed extraData v0 (`0x00`); v12/v13-era permits embed
// extraData v1 (`0x01` + 32-byte context id).
const STALE_V11_EXTRA_DATA = '0x00';

const word = (value: bigint): string => value.toString(16).padStart(64, '0');
const v1ExtraData = (kmsContextId: bigint): string => `0x01${word(kmsContextId)}`;

// CRITICAL: field order mirrors the authoritative kmsUserDecryptEip712V1Types /
// kmsDelegatedUserDecryptEip712V1Types — it determines the EIP-712 type hash,
// and parseSignedDecryptionPermit deep-compares the whole types record.
const V1_SELF_TYPES = {
  EIP712Domain: [
    { name: 'name', type: 'string' },
    { name: 'version', type: 'string' },
    { name: 'chainId', type: 'uint256' },
    { name: 'verifyingContract', type: 'address' },
  ],
  UserDecryptRequestVerification: [
    { name: 'publicKey', type: 'bytes' },
    { name: 'contractAddresses', type: 'address[]' },
    { name: 'startTimestamp', type: 'uint256' },
    { name: 'durationDays', type: 'uint256' },
    { name: 'extraData', type: 'bytes' },
  ],
} as const;

const V1_DELEGATED_TYPES = {
  EIP712Domain: [
    { name: 'name', type: 'string' },
    { name: 'version', type: 'string' },
    { name: 'chainId', type: 'uint256' },
    { name: 'verifyingContract', type: 'address' },
  ],
  DelegatedUserDecryptRequestVerification: [
    { name: 'publicKey', type: 'bytes' },
    { name: 'contractAddresses', type: 'address[]' },
    { name: 'delegatorAddress', type: 'address' },
    { name: 'startTimestamp', type: 'uint256' },
    { name: 'durationDays', type: 'uint256' },
    { name: 'extraData', type: 'bytes' },
  ],
} as const;

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

export function defineClientDecryptStalePermitMigrationTests(parameters: {
  readonly runIf: boolean;
  /** Protocol era of the chain under test (12, 13, or 14 — see `protocolEraOf`). */
  readonly era: number;
  readonly createFhevmDecryptClient: CreateEthersDecryptClientFn;
}): void {
  describe.runIf(parameters.runIf)('Decrypt client — stale cached permit across protocol migration', () => {
    let config: FheTestEthersConfig;

    beforeAll(() => {
      config = getEthersTestConfig();
      setFhevmRuntimeConfig({
        auth: {
          type: 'ApiKeyHeader',
          value: config.zamaApiKey,
        },
        logger: createLogger(console.log),
      });
    });

    async function createReadyClient() {
      const client = parameters.createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;
      return client;
    }

    /**
     * Rebuilds a cached old-era V1 permit from scratch: the caller chooses the
     * embedded `extraData` (and optionally the validity window / delegator),
     * the message is signed by the given wallet and reloaded through the
     * public `parseSignedDecryptionPermit` API — byte-identical to what a dApp
     * restores from cache after a protocol migration. Built from chain config
     * (not from a freshly signed permit) so it stays a V1 artifact regardless
     * of what permit version the chain currently produces.
     */
    async function forgeV1Permit(
      client: Awaited<ReturnType<typeof createReadyClient>>,
      transportKeyPair: unknown,
      opts: {
        readonly extraData: string;
        readonly wallet?: ethers.HDNodeWallet;
        readonly delegatorAddress?: string;
        readonly startTimestamp?: number;
        readonly durationDays?: number;
      },
    ) {
      const wallet = opts.wallet ?? config.wallet;
      const startTimestamp = opts.startTimestamp ?? Math.floor(Date.now() / 1000) - 5;
      const durationDays = opts.durationDays ?? 1;

      const domain = {
        name: 'Decryption',
        version: '1',
        chainId: BigInt(config.fhevmChain.id),
        verifyingContract: ethers.getAddress(config.fhevmChain.fhevm.gateway.contracts.decryption.address),
      };

      const publicKey = (transportKeyPair as { readonly publicKey: string }).publicKey;
      const contractAddresses = [ethers.getAddress(config.fheTestAddress)];

      const message = {
        publicKey,
        contractAddresses,
        ...(opts.delegatorAddress !== undefined ? { delegatorAddress: ethers.getAddress(opts.delegatorAddress) } : {}),
        startTimestamp: String(startTimestamp),
        durationDays: String(durationDays),
        extraData: opts.extraData,
      };

      const isDelegated = opts.delegatorAddress !== undefined;
      const primaryType = isDelegated ? 'DelegatedUserDecryptRequestVerification' : 'UserDecryptRequestVerification';

      // ethers signTypedData wants the types WITHOUT the EIP712Domain entry.
      const signature = await wallet.signTypedData(
        domain,
        isDelegated
          ? { DelegatedUserDecryptRequestVerification: [...V1_DELEGATED_TYPES.DelegatedUserDecryptRequestVerification] }
          : { UserDecryptRequestVerification: [...V1_SELF_TYPES.UserDecryptRequestVerification] },
        message,
      );

      return await client.parseSignedDecryptionPermit({
        serializedPermit: {
          version: 1,
          eip712: {
            domain,
            types: (isDelegated ? V1_DELEGATED_TYPES : V1_SELF_TYPES) as unknown as Record<
              string,
              ReadonlyArray<{ readonly name: string; readonly type: string }>
            >,
            primaryType,
            message,
          },
          signature,
          signerAddress: wallet.address,
        },
        transportKeyPair,
      });
    }

    async function readHandleAndExpected(ownerAddress: string): Promise<{
      fheType: string;
      encryptedValue: EncryptedValue;
      expectedRaw: bigint;
    }> {
      const fheType = decryptTestCases[0]!;
      const fheTypeId = fheTypeIdFromName(fheType);
      const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;
      const encryptedValue: EncryptedValue = asEncryptedValue(await fheTest.getHandleOf!(ownerAddress, fheTypeId));
      expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
      const expectedRaw = (await fheTest.getClearText!(encryptedValue)) as bigint;
      return { fheType, encryptedValue, expectedRaw };
    }

    function expectDecryptedMatches(fheType: string, typedValue: TypedValue, expectedRaw: bigint): void {
      console.log(`  ${fheType}: decrypted=${typedValue.value} expected=${expectedRaw}`);
      if (fheType === 'ebool') {
        expect(typedValue.value).toBe(expectedRaw !== 0n);
      } else if (fheType === 'eaddress') {
        const expectedAddr = '0x' + expectedRaw.toString(16).padStart(40, '0');
        expect(String(typedValue.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
      } else {
        expect(BigInt(typedValue.value as number | bigint)).toBe(expectedRaw);
      }
    }

    /** Signs a fresh permit and returns the extraData the current chain embeds. */
    async function currentPermitExtraData(client: Awaited<ReturnType<typeof createReadyClient>>): Promise<string> {
      const transportKeyPair = await client.generateTransportKeyPair();
      const signedPermit = await client.signDecryptionPermit({
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.wallet.address,
        signer: config.signer,
      });
      return (signedPermit.eip712.message as { extraData: string }).extraData;
    }

    it('premise: fresh permits on this chain embed a post-v11 extraData (not 0x00)', async () => {
      const client = await createReadyClient();
      const freshExtraData = await currentPermitExtraData(client);
      console.log(`  current-context extraData: ${freshExtraData}`);
      // If this fails, the chain still derives extraData v0 (v11 semantics)
      // and this suite should be excluded for it (see the wrapper's runIf).
      expect(freshExtraData).not.toBe(STALE_V11_EXTRA_DATA);
    });

    it('parses a cached v11-era permit (extraData v0 = 0x00) signed by the same account', async () => {
      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();
      const stalePermit = await forgeV1Permit(client, transportKeyPair, { extraData: STALE_V11_EXTRA_DATA });
      expect(stalePermit).toBeDefined();
      expect((stalePermit.eip712.message as { extraData: string }).extraData).toBe(STALE_V11_EXTRA_DATA);
    });

    // v11 -> v12/v13/v14 migration: extraData v0 is the "current context"
    // sentinel on-chain, so a cached v11 permit must keep decrypting after the
    // chain migrated.
    it('decrypts with a cached v11-era permit (extraData v0 = 0x00) after the chain migrated', async () => {
      const { fheType, encryptedValue, expectedRaw } = await readHandleAndExpected(config.wallet.address);

      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();
      const stalePermit = await forgeV1Permit(client, transportKeyPair, { extraData: STALE_V11_EXTRA_DATA });

      const typedValue = await client.decryptValue({
        contractAddress: config.fheTestAddress,
        encryptedValue,
        signedPermit: stalePermit,
        transportKeyPair,
      });

      expect(typedValue.type).toBe(clearTypeFromHandle(encryptedValue));
      expectDecryptedMatches(fheType, typedValue, expectedRaw);
    });

    // v13 -> v14 migration: a v13-era cached permit embeds extraData v1
    // (0x01 + context id). On a v14+ chain the current context encodes to v2
    // (context id + epoch id), so the encodings differ even when the context
    // id is unchanged — the permit must still decrypt.
    describe.runIf(parameters.era >= 14)('v13-era cached permit on a v14+ chain', () => {
      it('decrypts with a cached v13-era permit (extraData v1, current context id)', async () => {
        const client = await createReadyClient();

        const freshExtraData = await currentPermitExtraData(client);
        expect(freshExtraData).not.toBe(STALE_V11_EXTRA_DATA);
        expect(freshExtraData.length).toBeGreaterThanOrEqual(68);
        // extraData layout (v1 and v2): version byte + 32-byte context id.
        const currentContextId = BigInt(`0x${freshExtraData.slice(4, 68)}`);
        expect(currentContextId).not.toBe(0n);

        const { fheType, encryptedValue, expectedRaw } = await readHandleAndExpected(config.wallet.address);

        const transportKeyPair = await client.generateTransportKeyPair();
        const stalePermit = await forgeV1Permit(client, transportKeyPair, {
          extraData: v1ExtraData(currentContextId),
        });

        const typedValue = await client.decryptValue({
          contractAddress: config.fheTestAddress,
          encryptedValue,
          signedPermit: stalePermit,
          transportKeyPair,
        });

        expectDecryptedMatches(fheType, typedValue, expectedRaw);
      });
    });

    // Negative boundary: migration tolerance must not become substitution
    // tolerance. A permit referencing a context id that does not exist
    // on-chain is genuinely unusable and must be rejected (on-chain context
    // lookup revert / context mismatch) — never decrypted with the wrong
    // signer set.
    it('rejects a permit referencing an unknown KMS context (extraData v1, bogus context id)', async () => {
      const { encryptedValue } = await readHandleAndExpected(config.wallet.address);

      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();
      const bogusPermit = await forgeV1Permit(client, transportKeyPair, {
        extraData: v1ExtraData(0xdeadbeefn),
      });

      await expect(
        client.decryptValue({
          contractAddress: config.fheTestAddress,
          encryptedValue,
          signedPermit: bogusPermit,
          transportKeyPair,
        }),
      ).rejects.toThrow();
    });

    // Order-of-checks pin: a permit that is BOTH stale and expired must fail
    // with the clear expiration error, not a confusing context error.
    it('rejects an expired stale permit with the expiration error', async () => {
      const { encryptedValue } = await readHandleAndExpected(config.wallet.address);

      const client = await createReadyClient();
      const transportKeyPair = await client.generateTransportKeyPair();
      const expiredPermit = await forgeV1Permit(client, transportKeyPair, {
        extraData: STALE_V11_EXTRA_DATA,
        startTimestamp: Math.floor(Date.now() / 1000) - 3 * 86400,
        durationDays: 1,
      });

      await expect(
        client.decryptValue({
          contractAddress: config.fheTestAddress,
          encryptedValue,
          signedPermit: expiredPermit,
          transportKeyPair,
        }),
      ).rejects.toThrow('request has expired');
    });

    // Delegated flow: same stale-cache scenario, but Bob decrypts Alice's
    // handles via an on-chain ACL delegation and a delegated V1 permit forged
    // with v11-era extraData.
    describe('delegated stale permit (alice delegates to bob)', () => {
      beforeAll(async () => {
        const aclAddress = config.fhevmChain.fhevm.contracts.acl.address;
        const aclContract = new ethers.Contract(aclAddress, ACL_DELEGATE_ABI, config.provider);
        const existingExpiration = (await aclContract.getFunction('getUserDecryptionDelegationExpirationDate')(
          config.alice.wallet.address,
          config.bob.wallet.address,
          config.fheTestAddress,
        )) as bigint;

        const block = await config.provider.getBlock('latest');
        const blockTimestamp = BigInt(block?.timestamp ?? 0);
        if (existingExpiration > blockTimestamp) {
          console.log(`  Delegation already active (expires ${existingExpiration}), skipping tx`);
          return;
        }

        console.log('  Delegation not yet active, calling delegateForUserDecryption()...');
        const aclWithAlice = new ethers.Contract(aclAddress, ACL_DELEGATE_ABI, config.alice.signer);
        const tx = await aclWithAlice.getFunction('delegateForUserDecryption')(
          config.bob.wallet.address,
          config.fheTestAddress,
          Math.floor(Date.now() / 1000) + 86400 * 360,
        );
        const receipt = (await tx.wait()) as ethers.TransactionReceipt;
        if (receipt.status !== 1) {
          throw new Error(`Delegation tx failed: ${receipt.hash}`);
        }
        // Wait for the delegation to propagate to the gateway's MultichainACL.
        await new Promise((r) => setTimeout(r, 15000));
      }, 120_000);

      it('decrypts with a delegated cached v11-era permit (extraData v0 = 0x00)', async () => {
        const { fheType, encryptedValue, expectedRaw } = await readHandleAndExpected(config.alice.wallet.address);

        const client = await createReadyClient();
        const transportKeyPair = await client.generateTransportKeyPair();

        // Bob signs the delegated permit (v11-era shape) to decrypt Alice's handles
        const stalePermit = await forgeV1Permit(client, transportKeyPair, {
          extraData: STALE_V11_EXTRA_DATA,
          wallet: config.bob.wallet,
          delegatorAddress: config.alice.wallet.address,
        });
        expect(stalePermit.isDelegated).toBe(true);

        const typedValue = await client.decryptValue({
          contractAddress: config.fheTestAddress,
          encryptedValue,
          signedPermit: stalePermit,
          transportKeyPair,
        });

        expectDecryptedMatches(fheType, typedValue, expectedRaw);
      });
    });
  });
}
