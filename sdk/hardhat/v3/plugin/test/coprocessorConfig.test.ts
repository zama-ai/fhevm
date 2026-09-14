// D5b: the consumer-contract config reader and the init assertion. The plugin tests carry no consumer
// contract, so `hardhat_setStorageAt` writes the three ERC-7201 slots of an arbitrary address: an empty
// config, the deployed stack's config, and a foreign one exercise all three outcomes.
//
// Tests import the BUILT payload (pkg/_esm); see connection.test.ts.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import { HardhatPluginError } from 'hardhat/plugins';
import { pad, toHex } from 'viem';

import plugin from '#esm/index.js';
import { computeStorageLocation } from '#esm/internal/coprocessorConfig.js';
import { precomputeLocalhostAddresses } from '#esm/internal/deploy.js';

const CONSUMER = '0x1111111111111111111111111111111111111111';
const FOREIGN = '0x9999999999999999999999999999999999999999';
const LOCATION = computeStorageLocation('confidential.storage.config');

const pluginError = (fragment: string) => (e: unknown) =>
  e instanceof HardhatPluginError && e.message.includes(fragment);

void test('the ERC-7201 location of the config namespace matches @fhevm/solidity', () => {
  assert.equal(toHex(LOCATION, { size: 32 }), '0x9e7b61f58c47dc699ac88507c4f5bb9f121c03808c5676a8078fe583e4649700');
});

void test('getCoprocessorConfig and assertCoprocessorInitialized read what a consumer contract stored', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const { fhevm } = connection;
    const write = async (addresses: readonly string[]): Promise<void> => {
      for (const [i, address] of addresses.entries()) {
        await connection.provider.request({
          method: 'hardhat_setStorageAt',
          params: [CONSUMER, toHex(LOCATION + BigInt(i), { size: 32 }), pad(address as `0x${string}`, { size: 32 })],
        });
      }
    };

    // Nothing stored: an all-zero config, which the assertion names as "not initialized".
    assert.deepEqual(Object.values(await fhevm.getCoprocessorConfig(CONSUMER)), [
      '0x0000000000000000000000000000000000000000',
      '0x0000000000000000000000000000000000000000',
      '0x0000000000000000000000000000000000000000',
    ]);
    await assert.rejects(
      fhevm.assertCoprocessorInitialized(CONSUMER),
      pluginError(`Contract at ${CONSUMER} is not initialized`),
    );
    await assert.rejects(
      fhevm.assertCoprocessorInitialized({ address: CONSUMER }, 'Consumer'),
      pluginError(
        `Contract Consumer at ${CONSUMER} is not initialized for FHE operations. Make sure it either inherits from @fhevm/solidity/config/ZamaConfig.sol:ZamaEthereumConfig`,
      ),
    );

    // The deployed stack's trio: the assertion passes, the reader hands the addresses back checksummed.
    const { aclAddress, fhevmExecutorAddress, kmsVerifierAddress } = precomputeLocalhostAddresses().fhevmAddresses;
    await write([aclAddress, fhevmExecutorAddress, kmsVerifierAddress]);
    const config = await fhevm.getCoprocessorConfig({ getAddress: () => Promise.resolve(CONSUMER) });
    assert.deepEqual(config, {
      ACLAddress: aclAddress,
      CoprocessorAddress: fhevmExecutorAddress,
      KMSVerifierAddress: kmsVerifierAddress,
    });
    await fhevm.assertCoprocessorInitialized(CONSUMER, 'Consumer');

    // A foreign KMSVerifier: the mismatch names the field and both addresses.
    await write([aclAddress, fhevmExecutorAddress, FOREIGN]);
    await assert.rejects(
      fhevm.assertCoprocessorInitialized(CONSUMER),
      pluginError(`Coprocessor KMSVerifierAddress mismatch. Contract at ${CONSUMER} was initialized`),
    );

    await assert.rejects(fhevm.getCoprocessorConfig('0xnope'), pluginError('not a valid contract address'));
  } finally {
    await connection.close();
  }
});
