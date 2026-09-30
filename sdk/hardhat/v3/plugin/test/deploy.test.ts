// The deploy sequence on a fresh in-process chain: the stack lands on the precomputed addresses (the
// ZamaConfig trio among them), the proxies report the versions the package ships, a second run is a
// no-op, and a deployer that has already sent a transaction is refused by name.

import assert from 'node:assert/strict';
import test from 'node:test';

import { CONTRACT_VERSIONS } from '@fhevm/host-contracts-cleartext/ts';
import { HardhatPluginError } from 'hardhat/plugins';
import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import type { NetworkConnection } from 'hardhat/types/network';
import { type Address, type Chain, type PublicClient, parseAbi, parseEther, toHex } from 'viem';
import { mnemonicToAccount } from 'viem/accounts';

import { developmentChain, developmentPublicClient, developmentWalletClient } from '#esm/internal/clients.js';
import { LOCALHOST_DEPLOYER } from '#esm/internal/constants.js';
import { deployCleartextStack, precomputeLocalhostAddresses } from '#esm/internal/deploy.js';

const VERSION_ABI = parseAbi(['function getVersion() view returns (string)']);

async function withConnection(
  run: (connection: NetworkConnection, client: PublicClient, chain: Chain) => Promise<void>,
): Promise<void> {
  // No plugin: the sequence under test must find a FRESH chain, and the plugin would already have run it.
  const hre = await createHardhatRuntimeEnvironment({});
  const connection = await hre.network.create();
  try {
    const chain = await developmentChain(connection.provider);
    await run(connection, developmentPublicClient(connection.provider, chain), chain);
  } finally {
    await connection.close();
  }
}

async function version(client: PublicClient, address: string): Promise<string> {
  return client.readContract({ address: address as Address, abi: VERSION_ABI, functionName: 'getVersion' });
}

async function code(client: PublicClient, address: string): Promise<string> {
  return (await client.getCode({ address: address as Address })) ?? '0x';
}

void test('deploys the stack onto the precomputed addresses, once', async () => {
  await withConnection(async (connection, client) => {
    const expected = precomputeLocalhostAddresses();
    assert.equal(await code(client, expected.fhevmAddresses.aclAddress), '0x', 'fresh chain');

    const deployed = await deployCleartextStack(connection.provider);
    const { aclOwnerAddress, ...landed } = deployed;
    assert.deepEqual(landed, expected);
    assert.notEqual(await code(client, aclOwnerAddress), '0x', 'the ACL owner exists');
    for (const address of Object.values(expected.fhevmAddresses)) {
      assert.notEqual(await code(client, address), '0x', `${address} holds code`);
    }
    assert.equal(await version(client, expected.fhevmAddresses.aclAddress), CONTRACT_VERSIONS.acl);
    assert.equal(await version(client, expected.fhevmAddresses.fhevmExecutorAddress), CONTRACT_VERSIONS.fhevmExecutor);
    assert.equal(await version(client, expected.cleartextAddresses.cleartextDbAddress), CONTRACT_VERSIONS.cleartextDB);

    const deployer = LOCALHOST_DEPLOYER.address as Address;
    const nonceAfter = await client.getTransactionCount({ address: deployer });
    const again = await deployCleartextStack(connection.provider);
    assert.deepEqual(again, deployed, 'a second run reports the same stack');
    assert.equal(await client.getTransactionCount({ address: deployer }), nonceAfter, 'and sends nothing');
  });
});

void test('refuses a deployer that already moved off its start nonce', async () => {
  await withConnection(async (connection, client, chain) => {
    const account = mnemonicToAccount(LOCALHOST_DEPLOYER.mnemonic, { path: LOCALHOST_DEPLOYER.path });
    await connection.provider.request({
      method: 'hardhat_setBalance',
      params: [account.address, toHex(parseEther('1'))],
    });
    const wallet = developmentWalletClient(connection.provider, chain, account);
    const hash = await wallet.sendTransaction({ to: account.address, value: 0n });
    await client.waitForTransactionReceipt({ hash });

    await assert.rejects(
      deployCleartextStack(connection.provider),
      (error: unknown) =>
        HardhatPluginError.isHardhatPluginError(error) && error.message.includes('is at nonce 1, expected 0'),
    );
  });
});
