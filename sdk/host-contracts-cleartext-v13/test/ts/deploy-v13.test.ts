import {
  deploy,
  pauseACL,
  precomputeAddresses,
  unpauseACL,
  type BootstrapConfigV13,
} from '@fhevm/host-contracts-cleartext/ts';
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

const ACL_ABI = [
  { type: 'function', name: 'paused', stateMutability: 'view', inputs: [], outputs: [{ type: 'bool' }] },
] as const;

const PAUSER_SET_ABI = [
  {
    type: 'function',
    name: 'isPauser',
    stateMutability: 'view',
    inputs: [{ name: 'account', type: 'address' }],
    outputs: [{ type: 'bool' }],
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

function bootstrapConfig(deployerAddress: string, kmsSigner: string): BootstrapConfigV13 {
  return {
    kmsVerifier: { verifyingContractSource: deployerAddress, chainIDSource: 1n },
    inputVerifier: {
      verifyingContractSource: deployerAddress,
      chainIDSource: 1n,
      initialSigners: [deployerAddress],
      initialThreshold: 1n,
    },
    hcuLimit: { hcuCapPerBlock: 281474976710655n, maxHCUDepthPerTx: 5000000n, maxHCUPerTx: 20000000n },
    protocolConfig: {
      initialKmsNodes: [
        {
          txSenderAddress: deployerAddress,
          signerAddress: kmsSigner,
          ipAddress: '127.0.0.1',
          storageUrl: 'https://kms.example',
        },
      ],
      initialThresholds: { publicDecryption: 1n, userDecryption: 1n, kmsGen: 1n, mpc: 1n },
    },
  };
}

test('full deploy of a brand-new v13 stack: all proxies materialize and cleartext round-trips', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });
  const deployerAddress = privateKeyToAddress({ privateKey: deployerKey });
  const kmsSigner = privateKeyToAddress({
    privateKey: privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 8 }),
  });

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

    // Deploy a fresh v13 stack from scratch (deployer = admin).
    const deployed = await deploy({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
      precomputed: { fhevmAddresses, cleartextAddresses, pauserSetAddress },
      config: bootstrapConfig(deployerAddress, kmsSigner),
    });

    // (a) All 9 proxies are materialized (non-zero ERC-1967 implementation slot).
    const proxies: readonly Address[] = [
      deployed.fhevmAddresses.aclAddress,
      deployed.fhevmAddresses.fhevmExecutorAddress,
      deployed.fhevmAddresses.kmsVerifierAddress,
      deployed.fhevmAddresses.inputVerifierAddress,
      deployed.fhevmAddresses.hcuLimitAddress,
      deployed.fhevmAddresses.protocolConfigAddress,
      deployed.fhevmAddresses.kmsGenerationAddress,
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

    // (d) ACLOwner is a registered pauser and can pause/unpause the ACL through the admin.
    const acl = deployed.fhevmAddresses.aclAddress as Address;
    const paused = (): Promise<boolean> =>
      publicClient.readContract({ address: acl, abi: ACL_ABI, functionName: 'paused' });

    expect(
      await publicClient.readContract({
        address: deployed.pauserSetAddress as Address,
        abi: PAUSER_SET_ABI,
        functionName: 'isPauser',
        args: [deployed.aclOwnerAddress as Address],
      }),
    ).toBe(true);

    expect(await paused()).toBe(false);
    await pauseACL({ admin: adapters.signer, aclOwnerAddress: deployed.aclOwnerAddress });
    expect(await paused()).toBe(true);
    await unpauseACL({ admin: adapters.signer, aclOwnerAddress: deployed.aclOwnerAddress });
    expect(await paused()).toBe(false);
  } finally {
    await stopAnvil(anvil.process);
  }
}, 120_000);

test('deploy without precomputed derives addresses from the deployer live nonce', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });
  const deployerAddress = privateKeyToAddress({ privateKey: deployerKey });

  const anvil = startAnvil({ port: 8611, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
    const publicClient = createPublicClient({ chain: foundry, transport: http(anvil.rpcUrl) });
    const wallet = createWalletClient({
      account: privateKeyToAccount(deployerKey),
      chain: foundry,
      transport: http(anvil.rpcUrl),
    });

    // Advance the deployer's nonce past 0 so the derivation is exercised at a non-trivial offset — a
    // nonce-0 deploy would pass even if `getTransactionCount` were ignored.
    for (let i = 0; i < 3; i++) {
      const hash = await wallet.sendTransaction({ to: deployerAddress, value: 0n });
      await publicClient.waitForTransactionReceipt({ hash });
    }
    const liveNonce = BigInt(await publicClient.getTransactionCount({ address: deployerAddress }));
    expect(liveNonce).toBe(3n);

    // Addresses the deploy SHOULD derive, computed independently from the live nonce.
    const expected = precomputeAddresses({ ethUtils: adapters.utils, from: deployerAddress, startNonce: liveNonce });

    // Deploy with NO precomputed (and no config): deploy reads the nonce and precomputes internally.
    const deployed = await deploy({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
    });

    // (a) The internally-derived addresses match a nonce-based precompute exactly.
    expect(deployed.fhevmAddresses).toEqual(expected.fhevmAddresses);
    expect(deployed.cleartextAddresses).toEqual(expected.cleartextAddresses);
    expect(deployed.pauserSetAddress).toBe(expected.pauserSetAddress);

    // (b) The stack actually materialized at those addresses (wrong addresses would have reverted).
    const impl = await publicClient.getStorageAt({
      address: deployed.fhevmAddresses.aclAddress as Address,
      slot: IMPL_SLOT,
    });
    expect(BigInt(impl ?? '0x0')).not.toBe(0n);
  } finally {
    await stopAnvil(anvil.process);
  }
}, 120_000);
