import {
  deploy as deployV12,
  precomputeAddresses as precomputeV12,
  type BootstrapConfigV12,
} from '@fhevm/host-contracts-cleartext-v12/ts';
import { updateV12ToV13 } from '@fhevm/host-contracts-cleartext/ts';
import { createPublicClient, createWalletClient, http, parseEventLogs, type Address, type Hex } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { foundry } from 'viem/chains';
import { expect, test } from 'vitest';
import { startAnvil, stopAnvil, waitForAnvil } from './anvil.ts';
import { privateKeyFromMnemonic, privateKeyToAddress } from './ethUtils.ts';
import { createViemEthereumAdapters } from './viemEthereumLib.ts';

const IMPL_SLOT = '0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc' as const;
const MNEMONIC = 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';
const FHE_TYPE_UINT64 = 5;
// KMS_CONTEXT_COUNTER_BASE + 1 = (0x07 << 248) + 1 — the minimum valid migrated KMS context id.
const MIGRATED_CONTEXT_ID = (7n << 248n) + 1n;

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
] as const;

function v12BootstrapConfig(deployerAddress: string): BootstrapConfigV12 {
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

test('e2e: deploy a v12 cleartext stack, then upgrade it to v13 — cleartext survives the migration', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });
  const deployerAddress = privateKeyToAddress({ privateKey: deployerKey });
  const kmsSigner = privateKeyToAddress({
    privateKey: privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 8 }),
  });

  const anvil = startAnvil({ port: 8620, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
    const publicClient = createPublicClient({ chain: foundry, transport: http(anvil.rpcUrl) });
    const wallet = createWalletClient({
      account: privateKeyToAccount(deployerKey),
      chain: foundry,
      transport: http(anvil.rpcUrl),
    });

    // --- 1. Deploy a fresh v12 stack (installs a standing ACLOwner owned by the deployer). ---
    const precomputed = precomputeV12({ ethUtils: adapters.utils, from: deployerAddress, startNonce: 0n });
    const v12 = await deployV12({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
      precomputed: {
        fhevmAddresses: precomputed.fhevmAddresses,
        cleartextAddresses: precomputed.cleartextAddresses,
        pauserSetAddress: precomputed.pauserSetAddress,
      },
      config: v12BootstrapConfig(deployerAddress),
    });

    const executor = v12.fhevmAddresses.fhevmExecutorAddress as Address;
    const cleartextDb = v12.cleartextAddresses.cleartextDbAddress as Address;

    // trivialEncrypt(pt) on the executor, returning the resulting handle after mining.
    const trivialEncrypt = async (pt: bigint): Promise<Hex> => {
      const hash = await wallet.writeContract({
        address: executor,
        abi: EXECUTOR_ABI,
        functionName: 'trivialEncrypt',
        args: [pt, FHE_TYPE_UINT64],
      });
      const receipt = await publicClient.waitForTransactionReceipt({ hash });
      const events = parseEventLogs({ abi: EXECUTOR_ABI, eventName: 'TrivialEncrypt', logs: receipt.logs });
      const event = events[0];
      if (event === undefined) {
        throw new Error('TrivialEncrypt event not found');
      }
      return event.args.result;
    };

    // --- 2. Pre-upgrade round-trip: record a cleartext under the v12 executor. ---
    const handleBefore = await trivialEncrypt(42n);
    expect(
      await publicClient.readContract({
        address: cleartextDb,
        abi: CLEARTEXT_DB_ABI,
        functionName: 'get',
        args: [handleBefore],
      }),
    ).toBe(42n);

    // --- 3. Upgrade the live v12 stack to v13 (single atomic ACLOwner.upgrade). ---
    const migrated = await updateV12ToV13({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
      aclOwnerAddress: v12.aclOwnerAddress,
      existing: { ...v12.fhevmAddresses, pauserSetAddress: v12.pauserSetAddress },
      cleartext: v12.cleartextAddresses,
      migration: {
        existingContextId: MIGRATED_CONTEXT_ID,
        existingKmsNodes: [
          {
            txSenderAddress: deployerAddress,
            signerAddress: kmsSigner,
            ipAddress: '127.0.0.1',
            storageUrl: 'https://kms.example',
          },
        ],
        existingThresholds: { publicDecryption: 1n, userDecryption: 1n, kmsGen: 1n, mpc: 1n },
      },
    });

    // --- 4. The two new v13 proxies are materialized. ---
    for (const proxy of [migrated.protocolConfigAddress, migrated.kmsGenerationAddress] as const) {
      const impl = await publicClient.getStorageAt({ address: proxy as Address, slot: IMPL_SLOT });
      expect(BigInt(impl ?? '0x0')).not.toBe(0n);
    }

    // --- 5. Cleartext still works after the upgrade (new v13 executor impl → live CleartextArithmetic
    //        → CleartextDB), and the pre-upgrade value persisted through the migration. ---
    const handleAfter = await trivialEncrypt(99n);
    expect(
      await publicClient.readContract({
        address: cleartextDb,
        abi: CLEARTEXT_DB_ABI,
        functionName: 'get',
        args: [handleAfter],
      }),
    ).toBe(99n);
    expect(
      await publicClient.readContract({
        address: cleartextDb,
        abi: CLEARTEXT_DB_ABI,
        functionName: 'get',
        args: [handleBefore],
      }),
    ).toBe(42n);
  } finally {
    await stopAnvil(anvil.process);
  }
}, 180_000);
