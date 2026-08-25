// Deploys a full cleartext v12 stack onto an already-running anvil node, then prints the addresses.
//
// Driven by scripts/anvil.sh; not a test. It lives in test/ts because it consumes the package by its
// PUBLISHED name (via the tarball-consumer fixture) and reuses the viem adapters its neighbours use —
// the same reason those files are excluded from the harness tsconfig and lint config.
//
// Usage: node test/ts/deployStack.ts <rpcUrl> [mnemonic]
import { deploy, precomputeAddresses } from '@fhevm/host-contracts-cleartext/ts';

import { privateKeyFromMnemonic, privateKeyToAddress } from './ethUtils.ts';
import { createViemEthereumAdapters } from './viemEthereumLib.ts';

const DEFAULT_MNEMONIC =
  'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';

const DEPLOYER_INDEX = 5;

async function main(): Promise<void> {
  const rpcUrl = process.argv[2];
  if (rpcUrl === undefined || rpcUrl === '') {
    throw new Error('usage: node test/ts/deployStack.ts <rpcUrl> [mnemonic]');
  }
  const mnemonic = process.argv[3] ?? DEFAULT_MNEMONIC;

  const deployerKey = privateKeyFromMnemonic({ mnemonic, addressIndex: DEPLOYER_INDEX });
  const deployerAddress = privateKeyToAddress({ privateKey: deployerKey });
  const adapters = createViemEthereumAdapters({ rpcUrl, privateKey: deployerKey });

  const { fhevmAddresses, cleartextAddresses, pauserSetAddress } = precomputeAddresses({
    ethUtils: adapters.utils,
    from: deployerAddress,
    startNonce: 0n,
  });

  const deployed = await deploy({
    ethProvider: adapters.provider,
    ethUtils: adapters.utils,
    deployer: adapters.signer,
    admin: adapters.signer,
    precomputed: { fhevmAddresses, cleartextAddresses, pauserSetAddress },
    // No `config`, so `deploy()` applies DEFAULT_BOOTSTRAP_CONFIG — the Solidity mirror of which is
    // pkg/forge/src/_internal/LocalHostBootstrap.sol. That default registers the coprocessor and KMS
    // signers derived from FHEVM_MNEMONIC, which is what the js-sdk cleartext relayer holds keys for and
    // looks up by on-chain address. The hand-rolled config this replaced registered the deployer instead,
    // giving a stack the SDK could not sign as — every address correct, every signature unobtainable.
  });

  const rows: ReadonlyArray<readonly [string, string]> = [
    ['ACL', deployed.fhevmAddresses.aclAddress],
    ['FHEVMExecutor', deployed.fhevmAddresses.fhevmExecutorAddress],
    ['KMSVerifier', deployed.fhevmAddresses.kmsVerifierAddress],
    ['InputVerifier', deployed.fhevmAddresses.inputVerifierAddress],
    ['HCULimit', deployed.fhevmAddresses.hcuLimitAddress],
    ['CleartextArithmetic', deployed.cleartextAddresses.cleartextArithmeticAddress],
    ['CleartextDB', deployed.cleartextAddresses.cleartextDbAddress],
    ['PauserSet', deployed.pauserSetAddress],
    ['ACLOwner', deployed.aclOwnerAddress],
  ];

  console.log('');
  console.log('  cleartext v12 stack deployed');
  console.log(`  rpc      ${rpcUrl}`);
  console.log(`  deployer ${deployerAddress}  (mnemonic index ${String(DEPLOYER_INDEX)})`);
  console.log('');
  for (const [name, address] of rows) {
    console.log(`  ${name.padEnd(20)} ${address}`);
  }
  console.log('');
}

await main();
