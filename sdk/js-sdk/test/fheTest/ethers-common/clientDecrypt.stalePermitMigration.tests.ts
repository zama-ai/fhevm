import type { ethers } from 'ethers';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { getEthersTestConfig, type CreateEthersDecryptClientFn, type FheTestEthersConfig } from '../setup-ethers.js';
import { decryptTestCases, fheTypeIdFromName, createLogger } from '../setupCommon.js';
import { asEncryptedValue, type EncryptedValue } from '@fhevm/sdk/types';

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

// A permit signed while the chain was on protocol v0.11 embeds extraData v0
// (`0x00`) in its EIP-712 message. Permits are cached client-side (e.g.
// localStorage) for their whole validity window (up to 365 days), so this
// exact payload legitimately comes back after the chain has migrated
// (v11 -> v12 -> v13 -> v14). The signature covers extraData, so the SDK
// cannot rewrite it — the decrypt path has to accept the old encoding.
const STALE_V11_EXTRA_DATA = '0x00';

export function defineClientDecryptStalePermitMigrationTests(parameters: {
  readonly runIf: boolean;
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
     * Signs a fresh permit, then rebuilds it as a v11-era cached permit: same
     * EIP-712 message but with extraData v0 (`0x00`), re-signed by the same
     * account, reloaded through `parseSignedDecryptionPermit` — byte-identical
     * to what a dApp restores from cache after the chain migrated away from
     * v0.11.
     */
    async function signFreshAndForgeStalePermit(client: Awaited<ReturnType<typeof createReadyClient>>) {
      const transportKeyPair = await client.generateTransportKeyPair();
      const signedPermit = await client.signDecryptionPermit({
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.wallet.address,
        signer: config.signer,
      });

      const { domain, types, message } = signedPermit.eip712;
      const msg = message as {
        readonly publicKey: string;
        readonly contractAddresses: readonly string[];
        readonly startTimestamp: string;
        readonly durationDays: string;
        readonly extraData: string;
      };

      const staleMessage = { ...msg, extraData: STALE_V11_EXTRA_DATA };

      // ethers signTypedData wants the types WITHOUT the EIP712Domain entry.
      const staleSignature = await config.wallet.signTypedData(
        domain as ethers.TypedDataDomain,
        {
          UserDecryptRequestVerification: [
            { name: 'publicKey', type: 'bytes' },
            { name: 'contractAddresses', type: 'address[]' },
            { name: 'startTimestamp', type: 'uint256' },
            { name: 'durationDays', type: 'uint256' },
            { name: 'extraData', type: 'bytes' },
          ],
        },
        staleMessage,
      );

      // Reload through the public API, exactly like a dApp restoring a cached
      // permit. Parse validates structure + signature but not the extraData
      // era, so a stale permit parses successfully — the migration gap only
      // surfaces at decrypt time.
      const stalePermit = await client.parseSignedDecryptionPermit({
        serializedPermit: {
          version: 1,
          eip712: {
            domain: domain as Record<string, unknown>,
            types: types as Record<string, ReadonlyArray<{ readonly name: string; readonly type: string }>>,
            primaryType: 'UserDecryptRequestVerification',
            message: staleMessage,
          },
          signature: staleSignature,
          signerAddress: config.wallet.address,
        },
        transportKeyPair,
      });

      return { transportKeyPair, signedPermit, stalePermit };
    }

    it('premise: fresh permits on this chain embed a post-v11 extraData (not 0x00)', async () => {
      const client = await createReadyClient();
      const { signedPermit } = await signFreshAndForgeStalePermit(client);
      const freshExtraData = (signedPermit.eip712.message as { extraData: string }).extraData;
      console.log(`  current-context extraData: ${freshExtraData}`);
      // If this fails, the chain still derives extraData v0 (v11 semantics)
      // and this suite should be excluded for it (see the wrapper's runIf).
      expect(freshExtraData).not.toBe(STALE_V11_EXTRA_DATA);
    });

    it('parses a cached v11-era permit (extraData v0 = 0x00) signed by the same account', async () => {
      const client = await createReadyClient();
      const { stalePermit } = await signFreshAndForgeStalePermit(client);
      expect(stalePermit).toBeDefined();
      expect((stalePermit.eip712.message as { extraData: string }).extraData).toBe(STALE_V11_EXTRA_DATA);
    });

    // ------------------------------------------------------------------------
    // KNOWN GAP — expected to fail until the extraData migration patch lands.
    //
    // Today the SDK strict-compares the permit's extraData bytes against the
    // CURRENT on-chain KMS signers context (fetchKmsSigncryptedSharesV1/V2),
    // so the stale permit is rejected with
    //   'extraData "0x00" does not match KmsSignersContext extraData ...'
    //
    // `it.fails` asserts the body currently fails. Once the migration patch
    // lands, vitest reports these as failing — that is the signal to remove
    // the `.fails` markers and keep them as permanent regression tests.
    // ------------------------------------------------------------------------
    it.fails(
      'decrypts with a cached v11-era permit (extraData v0 = 0x00) — KNOWN GAP, remove .fails once the migration patch lands',
      async () => {
        const fheType = decryptTestCases[0]!;
        const fheTypeId = fheTypeIdFromName(fheType);

        const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

        const encryptedValue: EncryptedValue = asEncryptedValue(
          await fheTest.getHandleOf!(config.wallet.address, fheTypeId),
        );
        expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');

        const expectedRaw = (await fheTest.getClearText!(encryptedValue)) as bigint;

        const client = await createReadyClient();
        const { transportKeyPair, stalePermit } = await signFreshAndForgeStalePermit(client);

        const typedValue = await client.decryptValue({
          contractAddress: config.fheTestAddress,
          encryptedValue,
          signedPermit: stalePermit,
          transportKeyPair,
        });

        console.log(`  ${fheType}: decrypted=${typedValue.value} expected=${expectedRaw}`);
        if (fheType === 'ebool') {
          expect(typedValue.value).toBe(expectedRaw !== 0n);
        } else if (fheType === 'eaddress') {
          const expectedAddr = '0x' + expectedRaw.toString(16).padStart(40, '0');
          expect(String(typedValue.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
        } else {
          expect(BigInt(typedValue.value as number | bigint)).toBe(expectedRaw);
        }
      },
    );
  });
}
