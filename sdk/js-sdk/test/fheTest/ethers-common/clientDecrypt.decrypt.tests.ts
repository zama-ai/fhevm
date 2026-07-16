import type { ethers } from 'ethers';
import type { EncryptedValue, TypedValue } from '@fhevm/sdk/types';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { canDecryptValue, canDecryptValues, canDecryptValuesFromPairs } from '@fhevm/sdk/actions/decrypt';
import {
  decryptTestCases,
  fheTypeIdFromName,
  clearTypeFromHandle,
  fheTypeIdFromHandle,
  createLogger,
} from '../setupCommon.js';
import { asEncryptedValue } from '@fhevm/sdk/types';
import { getEthersTestConfig, type CreateEthersDecryptClientFn, type FheTestEthersConfig } from '../setup-ethers.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientDecrypt.decrypt.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.decrypt.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.decrypt.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.decrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientDecryptDecryptTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmDecryptClient: CreateEthersDecryptClientFn;
}): void {
  describe.runIf(parameters.runIf)('Decrypt client — user decrypt', () => {
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

    it('should create a decrypt client', () => {
      const client = parameters.createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      expect(client).toBeDefined();
      expect(typeof client.decryptValue).toBe('function');
      expect(typeof client.decryptValues).toBe('function');
      expect(typeof client.decryptValuesFromPairs).toBe('function');
      expect(typeof client.generateTransportKeyPair).toBe('function');
      expect(typeof client.signDecryptionPermit).toBe('function');
    });

    it('should generate an e2e transport key pair', async () => {
      const client = parameters.createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;

      const keyPair = await client.generateTransportKeyPair();
      expect(keyPair).toBeDefined();
    });

    it('should sign a self decryption permit', async () => {
      const client = parameters.createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;

      const keyPair = await client.generateTransportKeyPair();
      const signedPermit = await client.signDecryptionPermit({
        transportKeyPair: keyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.wallet.address,
        signer: config.signer,
      });

      expect(signedPermit).toBeDefined();
      // While the v3 relayer routes are disabled (RELAYER_V3_ROUTES_ENABLED =
      // false in src/core/runtime/userDecryptFlowVersion-p.ts), permits are
      // always V1 regardless of protocol version. Restore the version-based
      // expectation (protocol >= 0.14 -> V2) when the v3 routes are re-enabled.
      expect(signedPermit.version).toBe(1);
      expect(signedPermit.eip712.primaryType).toBe('UserDecryptRequestVerification');
      expect(signedPermit.isDelegated).toBe(false);
    });

    // ┌─────────────────────────────────────────────────────────────────────┐
    // │  Per-type decrypt tests                                             │
    // │  For each FHE type:                                                 │
    // │  1. Read the handle from FHETest.getHandleOf(deployer, fheType)     │
    // │  2. Read the expected clear value from FHETest.getClearText(handle) │
    // │  3. Decrypt via the SDK                                             │
    // │  4. Compare decrypted value with expected                           │
    // └─────────────────────────────────────────────────────────────────────┘

    for (const fheType of decryptTestCases) {
      it(`should decrypt ${fheType} and match on-chain clear text`, async () => {
        const fheTypeId = fheTypeIdFromName(fheType);
        const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

        // Read handle from FHETest contract
        const encryptedValue: EncryptedValue = asEncryptedValue(
          await fheTest.getHandleOf!(config.wallet.address, fheTypeId),
        );
        expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        expect(fheTypeIdFromHandle(encryptedValue)).toBe(fheTypeIdFromName(fheType));

        console.log(`  ${fheType}: handle=${encryptedValue.slice(0, 20)}...`);

        // Read expected clear value from FHETest._db
        const expectedRaw: bigint = await fheTest.getClearText!(encryptedValue);

        // Decrypt via SDK
        const client = parameters.createFhevmDecryptClient({
          chain: config.fhevmChain,
          provider: config.provider,
        });
        await client.ready;

        const transportKeyPair = await client.generateTransportKeyPair();
        const signedPermit = await client.signDecryptionPermit({
          transportKeyPair: transportKeyPair,
          contractAddresses: [config.fheTestAddress],
          durationSeconds: 24 * 3600,
          startTimestamp: Math.floor(Date.now() / 1000) - 5,
          signerAddress: config.wallet.address,
          signer: config.signer,
        });

        const typedValue = await client.decryptValue({
          contractAddress: config.fheTestAddress,
          encryptedValue: encryptedValue,
          signedPermit,
          transportKeyPair: transportKeyPair,
        });

        expect(typedValue.type).toBe(clearTypeFromHandle(encryptedValue));

        console.log(`  ${fheType}: decrypted=${typedValue.value} expected=${expectedRaw}`);

        // Compare based on type
        if (fheType === 'ebool') {
          expect(typedValue.value).toBe(expectedRaw !== 0n);
        } else if (fheType === 'eaddress') {
          // eaddress: compare as lowercase hex strings
          const expectedAddr = '0x' + expectedRaw.toString(16).padStart(40, '0');
          expect(String(typedValue.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
        } else {
          // uint types: compare as bigint
          expect(BigInt(typedValue.value as number | bigint)).toBe(expectedRaw);
        }
      });
    }

    // ┌─────────────────────────────────────────────────────────────────────┐
    // │  All-at-once decrypt test                                           │
    // │  Read all handles, decrypt in a single call, compare each result    │
    // └─────────────────────────────────────────────────────────────────────┘

    it('should decrypt all types in a single call', async () => {
      const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

      // Read all handles and their expected clear values from FHETest
      const entries: {
        fheType: string;
        encryptedValue: EncryptedValue;
        expectedRaw: bigint;
      }[] = [];

      for (const fheType of decryptTestCases) {
        const fheTypeId = fheTypeIdFromName(fheType);

        const encryptedValue: EncryptedValue = asEncryptedValue(
          await fheTest.getHandleOf!(config.wallet.address, fheTypeId),
        );
        expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');

        const expectedRaw: bigint = await fheTest.getClearText!(encryptedValue);
        entries.push({ fheType, encryptedValue, expectedRaw });
        console.log(`  ${fheType}: handle=${encryptedValue.slice(0, 20)}... expected=${expectedRaw}`);
      }

      // Decrypt all in a single call
      const client = parameters.createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;

      const transportKeyPair = await client.generateTransportKeyPair();
      const signedPermit = await client.signDecryptionPermit({
        transportKeyPair: transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.wallet.address,
        signer: config.signer,
      });

      const encryptedValues = entries.map((e) => asEncryptedValue(e.encryptedValue));

      const typedValues: readonly TypedValue[] = await client.decryptValues({
        encryptedValues,
        contractAddress: config.fheTestAddress,
        signedPermit,
        transportKeyPair: transportKeyPair,
      });

      expect(typedValues).toHaveLength(entries.length);

      // Compare each result
      for (let i = 0; i < entries.length; i++) {
        const { fheType, expectedRaw } = entries[i]!;
        const decrypted = typedValues[i]!;
        console.log(`  ${fheType}: decrypted=${decrypted.value} expected=${expectedRaw}`);

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
    // │  Pairs decrypt test                                                 │
    // │  Read all handles, decrypt via decryptValuesFromPairs, compare each │
    // └─────────────────────────────────────────────────────────────────────┘

    it('should decrypt all types via decryptValuesFromPairs', async () => {
      const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

      // Read all handles and their expected clear values from FHETest
      const entries: {
        fheType: string;
        encryptedValue: EncryptedValue;
        expectedRaw: bigint;
      }[] = [];

      for (const fheType of decryptTestCases) {
        const fheTypeId = fheTypeIdFromName(fheType);

        const encryptedValue: EncryptedValue = asEncryptedValue(
          await fheTest.getHandleOf!(config.wallet.address, fheTypeId),
        );
        expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');

        const expectedRaw: bigint = await fheTest.getClearText!(encryptedValue);
        entries.push({ fheType, encryptedValue, expectedRaw });
        console.log(`  ${fheType}: handle=${encryptedValue.slice(0, 20)}... expected=${expectedRaw}`);
      }

      // Decrypt all via decryptValuesFromPairs, each pair carrying its own contract address
      const client = parameters.createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;

      const transportKeyPair = await client.generateTransportKeyPair();
      const signedPermit = await client.signDecryptionPermit({
        transportKeyPair: transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.wallet.address,
        signer: config.signer,
      });

      const pairs = entries.map((e) => ({
        encryptedValue: asEncryptedValue(e.encryptedValue),
        contractAddress: config.fheTestAddress,
      }));

      const typedValues: readonly TypedValue[] = await client.decryptValuesFromPairs({
        pairs,
        signedPermit,
        transportKeyPair: transportKeyPair,
      });

      expect(typedValues).toHaveLength(entries.length);

      // Compare each result
      for (let i = 0; i < entries.length; i++) {
        const { fheType, expectedRaw } = entries[i]!;
        const decrypted = typedValues[i]!;
        console.log(`  ${fheType}: decrypted=${decrypted.value} expected=${expectedRaw}`);

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
    // │  canDecryptValue / canDecryptValues / canDecryptValuesFromPairs     │
    // │  ACL preflight checks — no actual decryption is performed           │
    // └─────────────────────────────────────────────────────────────────────┘

    it('should report canDecryptValue, canDecryptValues and canDecryptValuesFromPairs as allowed', async () => {
      const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

      // Read all handles from FHETest
      const encryptedValues: EncryptedValue[] = [];

      for (const fheType of decryptTestCases) {
        const fheTypeId = fheTypeIdFromName(fheType);
        const encryptedValue: EncryptedValue = asEncryptedValue(
          await fheTest.getHandleOf!(config.wallet.address, fheTypeId),
        );
        expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        encryptedValues.push(encryptedValue);
      }

      const client = parameters.createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;

      const transportKeyPair = await client.generateTransportKeyPair();
      const signedPermit = await client.signDecryptionPermit({
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.wallet.address,
        signer: config.signer,
      });

      // canDecryptValue — identifying the target user by a plain address
      const singleByUserAddress = await canDecryptValue(client, {
        encryptedValue: encryptedValues[0]!,
        contractAddress: config.fheTestAddress,
        userAddress: config.wallet.address,
      });
      expect(singleByUserAddress.allowed).toBe(true);
      expect(singleByUserAddress.details.contractAllowed).toBe(true);
      expect(singleByUserAddress.details.userAllowed).toBe(true);

      // canDecryptValue — identifying the target user by a signed decryption permit
      const singleByPermit = await canDecryptValue(client, {
        encryptedValue: encryptedValues[0]!,
        contractAddress: config.fheTestAddress,
        signedPermit,
        transportKeyPair,
      });
      expect(singleByPermit.allowed).toBe(true);
      expect(singleByPermit.details.contractAllowed).toBe(true);
      expect(singleByPermit.details.userAllowed).toBe(true);

      // canDecryptValues — batch of handles on a single contract
      const batchResult = await canDecryptValues(client, {
        encryptedValues,
        contractAddress: config.fheTestAddress,
        signedPermit,
        transportKeyPair,
      });
      expect(batchResult.allowed).toBe(true);
      expect(batchResult.details).toHaveLength(encryptedValues.length);
      for (const detail of batchResult.details) {
        expect(detail.contractAllowed).toBe(true);
        expect(detail.userAllowed).toBe(true);
      }

      // canDecryptValuesFromPairs — same batch, expressed as handle/contract-address pairs
      const pairs = encryptedValues.map((encryptedValue) => ({
        encryptedValue,
        contractAddress: config.fheTestAddress,
      }));

      const pairsResult = await canDecryptValuesFromPairs(client, {
        pairs,
        signedPermit,
        transportKeyPair,
      });
      expect(pairsResult.allowed).toBe(true);
      expect(pairsResult.details).toHaveLength(pairs.length);
      for (const detail of pairsResult.details) {
        expect(detail.contractAllowed).toBe(true);
        expect(detail.userAllowed).toBe(true);
      }
    });
  });
}
