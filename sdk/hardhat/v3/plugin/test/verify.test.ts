// The post-deploy state the package's `deploy()` leaves behind, read back through the repository: the
// KMS and coprocessor signer sets are registered with consistent thresholds, the HCU caps are set,
// and the package's own `verify()` passes on it — while a chain with no stack fails it by name.

import assert from 'node:assert/strict';
import test from 'node:test';

import { HardhatPluginError } from 'hardhat/plugins';
import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import type { Address, PublicClient } from 'viem';

import plugin from '../pkg/_esm/index.js';
import { developmentChain, developmentPublicClient } from '../pkg/_esm/internal/clients.js';
import { FhevmCleartextContractsRepository, type FhevmContractWrapper } from '../pkg/_esm/internal/contracts.js';
import { deployCleartextStack, precomputeLocalhostAddresses } from '../pkg/_esm/internal/deploy.js';
import { verifyCleartextStack } from '../pkg/_esm/internal/verify.js';

const localhost = precomputeLocalhostAddresses();

async function read(client: PublicClient, wrapper: FhevmContractWrapper, functionName: string): Promise<unknown> {
  return client.readContract({ address: wrapper.address, abi: wrapper.abi, functionName });
}

void test('the fresh stack has its signers and HCU caps registered', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const chain = await developmentChain(connection.provider);
    const client = developmentPublicClient(connection.provider, chain);
    const { fhevmAddresses, cleartextAddresses } = localhost;
    const repository = new FhevmCleartextContractsRepository(client, {
      ...fhevmAddresses,
      ...cleartextAddresses,
      pauserSetAddress: localhost.pauserSetAddress,
    });

    const kmsSigners = (await read(client, repository.kmsVerifier, 'getKmsSigners')) as Address[];
    const kmsThreshold = (await read(client, repository.kmsVerifier, 'getThreshold')) as bigint;
    assert.ok(kmsSigners.length > 0, 'KMS signers registered');
    assert.ok(kmsThreshold > 0n && kmsThreshold <= BigInt(kmsSigners.length), 'KMS threshold within the set');

    const coprocessors = (await read(client, repository.inputVerifier, 'getCoprocessorSigners')) as Address[];
    const coprocessorThreshold = (await read(client, repository.inputVerifier, 'getThreshold')) as bigint;
    assert.ok(coprocessors.length > 0, 'coprocessor signers registered');
    assert.ok(coprocessorThreshold > 0n && coprocessorThreshold <= BigInt(coprocessors.length));

    assert.ok(repository.hcuLimit, 'the local stack registers HCULimit');
    assert.ok(((await read(client, repository.hcuLimit, 'getGlobalHCUCapPerBlock')) as bigint) > 0n);
    assert.ok(((await read(client, repository.hcuLimit, 'getMaxHCUPerTx')) as bigint) > 0n);
    assert.ok(((await read(client, repository.hcuLimit, 'getMaxHCUDepthPerTx')) as bigint) > 0n);
  } finally {
    await connection.close();
  }
});

void test("the package's verify() passes on a prepared chain and fails by name on a bare one", async () => {
  const prepared = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await prepared.network.create();
  try {
    const deployed = await deployCleartextStack(connection.provider); // already present: reports the stack
    await verifyCleartextStack(connection.provider, deployed);
  } finally {
    await connection.close();
  }

  const bare = await createHardhatRuntimeEnvironment({});
  const empty = await bare.network.create();
  try {
    await assert.rejects(
      verifyCleartextStack(empty.provider, { ...localhost, aclOwnerAddress: localhost.pauserSetAddress }),
      (error: unknown) => HardhatPluginError.isHardhatPluginError(error) && error.message.includes('did not verify'),
    );
  } finally {
    await empty.close();
  }
});
