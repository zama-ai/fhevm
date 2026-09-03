// The ABI repository against a live connection's public client: every host contract resolves its ABI
// from @fhevm/host-contracts-cleartext, wrappers are found by name and (case-insensitively) by
// address, optional contracts stay unregistered when no address is known, and the cleartext repository
// is structurally distinct from the host one.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import { type PublicClient, getAbiItem } from 'viem';

import { developmentChain, developmentPublicClient } from '#esm/internal/clients.js';
import {
  FhevmCleartextContractsRepository,
  FhevmContractsRepository,
  type FhevmHostContractsAddresses,
  isCleartextContractsRepository,
} from '#esm/internal/contracts.js';

type Fixture = {
  readonly client: PublicClient;
  readonly host: FhevmHostContractsAddresses;
  /** Distinct, checksummed, unused by `host`: the repository never validates addresses, only keys on them. */
  readonly spare: readonly [string, string, string];
};

async function withFixture(run: (fixture: Fixture) => void): Promise<void> {
  // No plugin: the repository only needs a provider, and the deploy the plugin would run is not under test.
  const hre = await createHardhatRuntimeEnvironment({});
  const connection = await hre.network.create();
  try {
    const accounts = (await connection.provider.request({ method: 'eth_accounts' })) as string[];
    assert.ok(accounts.length >= 7, 'need 7 accounts');
    const [acl, executor, inputVerifier, kmsVerifier, spare1, spare2, spare3] = accounts as [
      string,
      string,
      string,
      string,
      string,
      string,
      string,
    ];
    const chain = await developmentChain(connection.provider);
    run({
      client: developmentPublicClient(connection.provider, chain),
      host: {
        aclAddress: acl,
        fhevmExecutorAddress: executor,
        inputVerifierAddress: inputVerifier,
        kmsVerifierAddress: kmsVerifier,
      },
      spare: [spare1, spare2, spare3],
    });
  } finally {
    await connection.close();
  }
}

void test('the host repository wraps the four core contracts with their ABIs', async () => {
  await withFixture(({ client, host }) => {
    const repository = new FhevmContractsRepository(client, host);

    for (const wrapper of [
      repository.acl,
      repository.fhevmExecutor,
      repository.inputVerifier,
      repository.kmsVerifier,
    ]) {
      assert.equal(wrapper.package, '@fhevm/host-contracts-cleartext');
      assert.ok(wrapper.abi.length > 0, `${wrapper.name} has an ABI`);
      assert.ok(getAbiItem({ abi: wrapper.abi, name: 'getVersion' }), `${wrapper.name} reports getVersion()`);
      assert.equal(wrapper.contract.address, wrapper.address);
      assert.equal(typeof wrapper.contract.read, 'object', 'read-only contract is bound');
    }
    assert.equal(repository.acl.name, 'ACL');
    assert.equal(repository.fhevmExecutor.name, 'FHEVMExecutor');
    assert.equal(repository.hcuLimit, undefined, 'unknown addresses stay unregistered');
    assert.equal(repository.pauserSet, undefined);
    assert.equal(isCleartextContractsRepository(repository), false);
  });
});

void test('lookups work by name and by address whatever the casing', async () => {
  await withFixture(({ client, host, spare: [hcuLimit] }) => {
    const repository = new FhevmContractsRepository(client, { ...host, hcuLimitAddress: hcuLimit });

    assert.equal(repository.getContractFromName('HCULimit'), repository.hcuLimit);
    assert.equal(repository.getContractFromName('PauserSet'), undefined);
    const shouting = `0x${host.aclAddress.slice(2).toUpperCase()}`;
    assert.equal(repository.getContractFromAddress(shouting), repository.acl);
    assert.equal(repository.getContractFromAddress(host.aclAddress.toLowerCase()), repository.acl);
    assert.equal(repository.getContractFromAddress('0x0000000000000000000000000000000000000001'), undefined);
    assert.equal(repository.addressToContractMap().size, 5);
  });
});

void test('the cleartext repository adds the two cleartext-only contracts', async () => {
  await withFixture(({ client, host, spare: [arithmetic, db] }) => {
    const repository = new FhevmCleartextContractsRepository(client, {
      ...host,
      cleartextArithmeticAddress: arithmetic,
      cleartextDbAddress: db,
    });

    assert.equal(isCleartextContractsRepository(repository), true);
    assert.equal(repository.cleartextDb.name, 'CleartextDB');
    assert.ok(repository.cleartextArithmetic.abi.length > 0);
    assert.equal(repository.getContractFromAddress(db), repository.cleartextDb);
    assert.equal(repository.addressToContractMap().size, 6);
  });
});
