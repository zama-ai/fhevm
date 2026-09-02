const assert = require('node:assert/strict');

const packageName = '@fhevm/host-contracts-cleartext';
const resolvedPackage = require.resolve(packageName);
assert.match(resolvedPackage, /\/ts\/_cjs\/index\.js$/, `require resolved the wrong build: ${resolvedPackage}`);
const { deploy, verify } = require(packageName);

async function main() {
  const { ANVIL_PORT, ANVIL_RPC_URL, isPortOpen, MNEMONIC, startAnvil, stopAnvil, waitForAnvil } =
    await import('@fhevm/sdk-common-dev/anvil.ts');
  const { createViemEthereumAdapters, createViemEthereumHistory } =
    await import('@fhevm/sdk-vendored-dev/viemEthereumLib.ts');
  const { createPublicClient, http, toHex } = await import('viem');
  const { mnemonicToAccount } = await import('viem/accounts');
  const { foundry } = await import('viem/chains');

  assert.equal(await isPortOpen({ port: ANVIL_PORT }), false, `Port ${String(ANVIL_PORT)} is already in use`);
  const anvil = startAnvil({ port: ANVIL_PORT, mnemonic: MNEMONIC });

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
    /** @type {Array<{ name: string, detail?: string }>} */
    const failures = report.failures;
    /** @type {Array<{ name: string }>} */
    const skipped = report.skipped;

    assert.equal(
      report.ok,
      true,
      failures.map((failure) => `${failure.name}: ${failure.detail ?? 'failed'}`).join('\n'),
    );
    assert.deepEqual(
      skipped.map((check) => check.name),
      [],
      'verify skipped checks that should run against a local Anvil deployment',
    );

    console.log(`✅ require resolved ${resolvedPackage}`);
    console.log(`✅ deployed and verified the default v13 stack through CommonJS on ${ANVIL_RPC_URL}`);
  } finally {
    await stopAnvil(anvil.process);
  }
}

void main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
