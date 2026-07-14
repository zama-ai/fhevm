import { deploy, precomputeAddresses, type BootstrapConfigV12 } from '@fhevm/host-contracts-cleartext-v12/ts';
import { createPublicClient, createWalletClient, http, parseEventLogs, type Address, type Hex } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { foundry } from 'viem/chains';
import { expect, test } from 'vitest';
import { startAnvil, stopAnvil, waitForAnvil } from './anvil.ts';
import { privateKeyFromMnemonic, privateKeyToAddress } from './ethUtils.ts';
import { createViemEthereumAdapters } from './viemEthereumLib.ts';

// ERC-1967 implementation slot: keccak256("eip1967.proxy.implementation") - 1.
const IMPL_SLOT = '0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc' as const;
const MNEMONIC = 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';
const FHE_TYPE_UINT64 = 5; // FheType.Uint64

// Executor: trivialEncrypt + its result event.
const EXECUTOR_ABI = [
  {
    type: 'function',
    name: 'trivialEncrypt',
    stateMutability: 'nonpayable',
    inputs: [
      { name: 'pt', type: 'uint256' },
      { name: 'toType', type: 'uint8' },
    ],
    outputs: [{ name: 'result', type: 'bytes32' }],
  },
  {
    type: 'function',
    name: 'plaintexts',
    stateMutability: 'view',
    inputs: [{ name: 'handle', type: 'bytes32' }],
    outputs: [{ type: 'uint256' }],
  },
  {
    type: 'event',
    name: 'TrivialEncrypt',
    inputs: [
      { name: 'caller', type: 'address', indexed: true },
      { name: 'pt', type: 'uint256', indexed: false },
      { name: 'toType', type: 'uint8', indexed: false },
      { name: 'result', type: 'bytes32', indexed: false },
    ],
  },
] as const;

const CLEARTEXT_DB_ABI = [
  {
    type: 'function',
    name: 'get',
    stateMutability: 'view',
    inputs: [{ name: 'handle', type: 'bytes32' }],
    outputs: [{ type: 'uint256' }],
  },
  {
    type: 'function',
    name: 'isWriter',
    stateMutability: 'view',
    inputs: [{ name: 'account', type: 'address' }],
    outputs: [{ type: 'bool' }],
  },
] as const;

function bootstrapConfig(deployerAddress: string): BootstrapConfigV12 {
  // In v12 both KMSVerifier and InputVerifier take (verifyingContractSource, chainIDSource,
  // initialSigners, initialThreshold) — the KMS signer set had not yet moved to ProtocolConfig.
  const verifier = {
    verifyingContractSource: deployerAddress,
    chainIDSource: 1n,
    initialSigners: [deployerAddress],
    initialThreshold: 1n,
  };
  return {
    kmsVerifier: verifier,
    inputVerifier: verifier,
    hcuLimit: { hcuCapPerBlock: 281474976710655n, maxHCUDepthPerTx: 5000000n, maxHCUPerTx: 20000000n },
  };
}

test('full deploy of a brand-new v12 stack: all proxies materialize and cleartext round-trips', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });
  const deployerAddress = privateKeyToAddress({ privateKey: deployerKey });

  const anvil = startAnvil({ port: 8610, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
    const publicClient = createPublicClient({ chain: foundry, transport: http(anvil.rpcUrl) });
    const wallet = createWalletClient({
      account: privateKeyToAccount(deployerKey),
      chain: foundry,
      transport: http(anvil.rpcUrl),
    });

    const { fhevmAddresses, cleartextAddresses, pauserSetAddress } = precomputeAddresses({
      ethUtils: adapters.utils,
      from: deployerAddress,
      startNonce: 0n,
    });

    // Deploy a fresh v12 stack from scratch (deployer = admin).
    const deployed = await deploy({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
      precomputed: { fhevmAddresses, cleartextAddresses, pauserSetAddress },
      config: bootstrapConfig(deployerAddress),
    });

    // (a) All 7 proxies are materialized (non-zero ERC-1967 implementation slot).
    const proxies: readonly Address[] = [
      deployed.fhevmAddresses.aclAddress,
      deployed.fhevmAddresses.fhevmExecutorAddress,
      deployed.fhevmAddresses.kmsVerifierAddress,
      deployed.fhevmAddresses.inputVerifierAddress,
      deployed.fhevmAddresses.hcuLimitAddress,
      deployed.cleartextAddresses.cleartextArithmeticAddress,
      deployed.cleartextAddresses.cleartextDbAddress,
    ].map((address) => address as Address);
    for (const proxy of proxies) {
      const impl = await publicClient.getStorageAt({ address: proxy, slot: IMPL_SLOT });
      expect(BigInt(impl ?? '0x0'), `impl slot for proxy ${proxy}`).not.toBe(0n);
    }

    // (b) The DB writer is CleartextArithmetic; the executor is not (executor never touches the DB).
    const cleartextDb = deployed.cleartextAddresses.cleartextDbAddress as Address;
    expect(
      await publicClient.readContract({
        address: cleartextDb,
        abi: CLEARTEXT_DB_ABI,
        functionName: 'isWriter',
        args: [deployed.cleartextAddresses.cleartextArithmeticAddress as Address],
      }),
    ).toBe(true);
    expect(
      await publicClient.readContract({
        address: cleartextDb,
        abi: CLEARTEXT_DB_ABI,
        functionName: 'isWriter',
        args: [deployed.fhevmAddresses.fhevmExecutorAddress as Address],
      }),
    ).toBe(false);

    // (c) Functional round-trip: trivialEncrypt(42) via the executor, then read the cleartext from
    //     the DB — proves executor → CleartextArithmetic → CleartextDB wiring end to end.
    const executor = deployed.fhevmAddresses.fhevmExecutorAddress as Address;
    const hash = await wallet.writeContract({
      address: executor,
      abi: EXECUTOR_ABI,
      functionName: 'trivialEncrypt',
      args: [42n, FHE_TYPE_UINT64],
    });
    const receipt = await publicClient.waitForTransactionReceipt({ hash });

    const events = parseEventLogs({ abi: EXECUTOR_ABI, eventName: 'TrivialEncrypt', logs: receipt.logs });
    const trivialEncryptEvent = events[0];
    if (trivialEncryptEvent === undefined) {
      throw new Error('TrivialEncrypt event not found in receipt');
    }
    const handle: Hex = trivialEncryptEvent.args.result;

    const stored = await publicClient.readContract({
      address: cleartextDb,
      abi: CLEARTEXT_DB_ABI,
      functionName: 'get',
      args: [handle],
    });
    expect(stored).toBe(42n);

    // Compat accessor: executor.plaintexts(handle) forwards to the DB via CleartextArithmetic.
    const viaExecutor = await publicClient.readContract({
      address: executor,
      abi: EXECUTOR_ABI,
      functionName: 'plaintexts',
      args: [handle],
    });
    expect(viaExecutor).toBe(42n);
  } finally {
    await stopAnvil(anvil.process);
  }
}, 120_000);
