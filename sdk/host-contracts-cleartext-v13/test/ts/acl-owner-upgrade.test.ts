import { deploy, precomputeAddresses, type BootstrapConfigV13 } from '@fhevm/host-contracts-cleartext/ts';
import { createPublicClient, createWalletClient, getAddress, http, type Address, type Hex } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { foundry } from 'viem/chains';
import { expect, test } from 'vitest';
import { startAnvil, stopAnvil, waitForAnvil, type AnvilNode } from './anvil.ts';
import { privateKeyFromMnemonic, privateKeyToAddress } from './ethUtils.ts';
import { createViemEthereumAdapters } from './viemEthereumLib.ts';

// ERC-1967 implementation slot: keccak256("eip1967.proxy.implementation") - 1.
const IMPL_SLOT = '0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc' as const;
const MNEMONIC = 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';

// Minimal, fully-typed ABI fragments (avoids importing untyped JSON artifacts).
// ACL real-implementation read surface (`getPauserSetAddress` only exists post-materialization).
const ACL_ABI = [
  { type: 'function', name: 'owner', stateMutability: 'view', inputs: [], outputs: [{ type: 'address' }] },
  { type: 'function', name: 'pendingOwner', stateMutability: 'view', inputs: [], outputs: [{ type: 'address' }] },
  {
    type: 'function',
    name: 'getPauserSetAddress',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'address' }],
  },
] as const;

// ACLOwner write surface.
const ACL_OWNER_ABI = [
  {
    type: 'function',
    name: 'upgrade',
    stateMutability: 'nonpayable',
    inputs: [
      {
        name: 'ops',
        type: 'tuple[]',
        components: [
          { name: 'proxy', type: 'address' },
          { name: 'implementation', type: 'address' },
          { name: 'initData', type: 'bytes' },
        ],
      },
    ],
    outputs: [],
  },
  {
    type: 'function',
    name: 'transferACLOwnership',
    stateMutability: 'nonpayable',
    inputs: [{ name: 'newOwner', type: 'address' }],
    outputs: [],
  },
] as const;

/** A concretely-typed wallet client for a mnemonic account (so viem's writeContract needs no chain/account). */
function walletFor(rpcUrl: string, addressIndex: number) {
  const privateKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex });
  return createWalletClient({ account: privateKeyToAccount(privateKey), chain: foundry, transport: http(rpcUrl) });
}

/** Valid bootstrap config for a fresh v13 stack (one KMS node, all thresholds = 1). */
function bootstrapConfig(parameters: {
  readonly verifyingContractSource: string;
  readonly coprocessorSigner: string;
  readonly kmsTxSender: string;
  readonly kmsSigner: string;
}): BootstrapConfigV13 {
  return {
    kmsVerifier: { verifyingContractSource: parameters.verifyingContractSource, chainIDSource: 1n },
    inputVerifier: {
      verifyingContractSource: parameters.verifyingContractSource,
      chainIDSource: 1n,
      initialSigners: [parameters.coprocessorSigner],
      initialThreshold: 1n,
    },
    hcuLimit: { hcuCapPerBlock: 281474976710655n, maxHCUDepthPerTx: 5000000n, maxHCUPerTx: 20000000n },
    protocolConfig: {
      initialKmsNodes: [
        {
          txSenderAddress: parameters.kmsTxSender,
          signerAddress: parameters.kmsSigner,
          ipAddress: '127.0.0.1',
          storageUrl: 'https://kms.example',
        },
      ],
      initialThresholds: { publicDecryption: 1n, userDecryption: 1n, kmsGen: 1n, mpc: 1n },
    },
  };
}

type DeployedStack = {
  readonly anvil: AnvilNode;
  readonly rpcUrl: string;
  readonly publicClient: ReturnType<typeof createPublicClient>;
  readonly aclOwnerAddress: Address;
  readonly aclAddress: Address;
  readonly pauserSetAddress: Address;
  readonly proxies: readonly Address[];
};

/** Starts anvil and deploys a fresh v13 stack via the public `deploy(...)` (deployer = admin). */
async function deployStack(port: number): Promise<DeployedStack> {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });
  const deployerAddress = privateKeyToAddress({ privateKey: deployerKey });
  const kmsSigner = privateKeyToAddress({
    privateKey: privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 8 }),
  });

  const anvil = startAnvil({ port, mnemonic: MNEMONIC });
  await waitForAnvil(anvil.rpcUrl);

  const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
  const publicClient = createPublicClient({ chain: foundry, transport: http(anvil.rpcUrl) });

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
    config: bootstrapConfig({
      verifyingContractSource: deployerAddress,
      coprocessorSigner: deployerAddress,
      kmsTxSender: deployerAddress,
      kmsSigner,
    }),
  });

  const addr = deployed.fhevmAddresses;
  const proxies: readonly Address[] = [
    addr.aclAddress,
    addr.fhevmExecutorAddress,
    addr.kmsVerifierAddress,
    addr.inputVerifierAddress,
    addr.hcuLimitAddress,
    addr.protocolConfigAddress,
    addr.kmsGenerationAddress,
    deployed.cleartextAddresses.cleartextArithmeticAddress,
    deployed.cleartextAddresses.cleartextDbAddress,
  ].map((a) => a as Address);

  return {
    anvil,
    rpcUrl: anvil.rpcUrl,
    publicClient,
    aclOwnerAddress: deployed.aclOwnerAddress as Address,
    aclAddress: addr.aclAddress as Address,
    pauserSetAddress: deployed.pauserSetAddress as Address,
    proxies,
  };
}

function readImplSlot(publicClient: DeployedStack['publicClient'], address: Address): Promise<Hex | undefined> {
  return publicClient.getStorageAt({ address, slot: IMPL_SLOT });
}

test('deploy materializes a fresh v13 stack owned by ACLOwner', async () => {
  const stack = await deployStack(8600);
  try {
    // `deploy` completing means the single atomic ACLOwner.upgrade materialized all 7 proxies
    // (any failing initializeFromEmptyProxy would have reverted the whole transaction).
    for (const proxy of stack.proxies) {
      const impl = await readImplSlot(stack.publicClient, proxy);
      expect(impl, `impl slot for proxy ${proxy}`).toBeDefined();
      expect(BigInt(impl ?? '0x0')).not.toBe(0n);
    }

    // ACL is a real, materialized contract: owned by ACLOwner and pointing at the deployed PauserSet.
    const owner = await stack.publicClient.readContract({
      address: stack.aclAddress,
      abi: ACL_ABI,
      functionName: 'owner',
    });
    expect(getAddress(owner)).toBe(getAddress(stack.aclOwnerAddress));

    const pauserSet = await stack.publicClient.readContract({
      address: stack.aclAddress,
      abi: ACL_ABI,
      functionName: 'getPauserSetAddress',
    });
    expect(getAddress(pauserSet)).toBe(getAddress(stack.pauserSetAddress));
  } finally {
    await stopAnvil(stack.anvil.process);
  }
}, 120_000);

test('a non-owner cannot call ACLOwner.upgrade', async () => {
  const stack = await deployStack(8601);
  try {
    const stranger = walletFor(stack.rpcUrl, 6);
    await expect(
      stranger.writeContract({
        address: stack.aclOwnerAddress,
        abi: ACL_OWNER_ABI,
        functionName: 'upgrade',
        args: [[]],
      }),
    ).rejects.toThrow();
  } finally {
    await stopAnvil(stack.anvil.process);
  }
}, 120_000);

test('transferACLOwnership hands the ACL owner role onward', async () => {
  const stack = await deployStack(8602);
  try {
    const owner = walletFor(stack.rpcUrl, 5);
    const successor = privateKeyToAddress({
      privateKey: privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 7 }),
    });

    await owner.writeContract({
      address: stack.aclOwnerAddress,
      abi: ACL_OWNER_ABI,
      functionName: 'transferACLOwnership',
      args: [successor],
    });

    const pending = await stack.publicClient.readContract({
      address: stack.aclAddress,
      abi: ACL_ABI,
      functionName: 'pendingOwner',
    });
    expect(getAddress(pending)).toBe(getAddress(successor));
  } finally {
    await stopAnvil(stack.anvil.process);
  }
}, 120_000);
