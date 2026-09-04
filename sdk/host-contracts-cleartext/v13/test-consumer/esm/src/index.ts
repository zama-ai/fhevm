import assert from 'node:assert/strict';

import { deploy, verify } from '@fhevm/host-contracts-cleartext';
import {
  ANVIL_PORT,
  ANVIL_RPC_URL,
  isPortOpen,
  MNEMONIC,
  startAnvil,
  stopAnvil,
  waitForAnvil,
} from '@fhevm/sdk-common-dev/anvil.ts';
import { createViemEthereumAdapters, createViemEthereumHistory } from '@fhevm/sdk-vendored-dev/viemEthereumLib.ts';
import { createPublicClient, http, toHex } from 'viem';
import { mnemonicToAccount } from 'viem/accounts';
import { foundry } from 'viem/chains';

async function main(): Promise<void> {
  assert.equal(await isPortOpen({ port: ANVIL_PORT }), false, `Port ${String(ANVIL_PORT)} is already in use`);
  const anvil = startAnvil({ port: ANVIL_PORT, mnemonic: MNEMONIC, printBanner: true });

  try {
    await waitForAnvil(anvil.rpcUrl);
    const publicClient = createPublicClient({ chain: foundry, transport: http(ANVIL_RPC_URL) });
    assert.equal(await publicClient.getChainId(), foundry.id, 'Anvil is not running');

    const account = mnemonicToAccount(MNEMONIC, { addressIndex: 5 });
    const privateKeyBytes = account.getHdKey().privateKey;
    assert(privateKeyBytes !== null, 'The deployer account has no private key');
    const adapters = createViemEthereumAdapters({ rpcUrl: ANVIL_RPC_URL, privateKey: toHex(privateKeyBytes) });

    const deployed = await deploy({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
    });

    const report = await verify({
      mode: 'deploy',
      ethProvider: adapters.provider,
      history: createViemEthereumHistory(ANVIL_RPC_URL),
      deployed,
      expected: { admin: account.address },
    });

    assert.equal(
      report.ok,
      true,
      report.failures.map((failure) => `${failure.name}: ${failure.detail ?? 'failed'}`).join('\n'),
    );
    assert.deepEqual(
      report.skipped.map((check) => check.name),
      [],
      'verify skipped checks that should run against a local Anvil deployment',
    );

    console.log(`✅ deployed and verified the default v13 stack on ${ANVIL_RPC_URL}`);
  } finally {
    await stopAnvil(anvil.process);
  }
}

await main();
