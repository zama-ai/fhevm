import {
  deploy as deployV13,
  precomputeAddresses as precomputeV13,
  type BootstrapConfigV13,
} from '@fhevm/host-contracts-cleartext-v13/ts';
import { updateV13ToV14 } from '@fhevm/host-contracts-cleartext/ts';
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

const KMS_VERIFIER_ABI = [
  {
    type: 'function',
    name: 'getKmsSigners',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'address[]' }],
  },
] as const;

// getKmsSigners / getCurrentKmsContextId share selectors across v13 and v14, and are exposed by both
// KMSVerifier (v13) and ProtocolConfig — one ABI serves every read in the no-migration test.
const KMS_CONTEXT_ABI = [
  {
    type: 'function',
    name: 'getKmsSigners',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'address[]' }],
  },
  {
    type: 'function',
    name: 'getCurrentKmsContextId',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'uint256' }],
  },
] as const;

function v13BootstrapConfig(deployerAddress: string, kmsSigner: string): BootstrapConfigV13 {
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

test('e2e: deploy a v13 cleartext stack, then upgrade it to v14 — cleartext survives the migration', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });
  const deployerAddress = privateKeyToAddress({ privateKey: deployerKey });
  const kmsSigner = privateKeyToAddress({
    privateKey: privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 8 }),
  });

  const anvil = startAnvil({ port: 8621, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
    const publicClient = createPublicClient({ chain: foundry, transport: http(anvil.rpcUrl) });
    const wallet = createWalletClient({
      account: privateKeyToAccount(deployerKey),
      chain: foundry,
      transport: http(anvil.rpcUrl),
    });

    // --- 1. Deploy a fresh v13 stack (installs a standing ACLOwner owned by the deployer). ---
    const precomputed = precomputeV13({ ethUtils: adapters.utils, from: deployerAddress, startNonce: 0n });
    const v13 = await deployV13({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
      precomputed: {
        fhevmAddresses: precomputed.fhevmAddresses,
        cleartextAddresses: precomputed.cleartextAddresses,
        pauserSetAddress: precomputed.pauserSetAddress,
      },
      config: v13BootstrapConfig(deployerAddress, kmsSigner),
    });

    const executor = v13.fhevmAddresses.fhevmExecutorAddress as Address;
    const cleartextDb = v13.cleartextAddresses.cleartextDbAddress as Address;

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

    // --- 2. Pre-upgrade round-trip: record a cleartext under the v13 executor. ---
    const handleBefore = await trivialEncrypt(42n);
    expect(
      await publicClient.readContract({
        address: cleartextDb,
        abi: CLEARTEXT_DB_ABI,
        functionName: 'get',
        args: [handleBefore],
      }),
    ).toBe(42n);

    const executorImplBefore = await publicClient.getStorageAt({ address: executor, slot: IMPL_SLOT });

    // --- 3. Upgrade the live v13 stack to v14 (single atomic ACLOwner.upgrade). No new proxies:
    //        v14 has the same contract set, so the migration only re-points + version-bumps. ---
    await updateV13ToV14({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
      aclOwnerAddress: v13.aclOwnerAddress,
      existing: { ...v13.fhevmAddresses, pauserSetAddress: v13.pauserSetAddress },
      cleartext: v13.cleartextAddresses,
      migration: {
        // The live context's node set, re-expressed in the v14 shape (the new fields are exactly what
        // v13 never stored on-chain).
        kmsNodeParams: [
          {
            txSenderAddress: deployerAddress,
            signerAddress: kmsSigner,
            ipAddress: '127.0.0.1',
            storageUrl: 'https://kms.example',
            partyId: 1,
            mpcIdentity: 'kms-1',
            caCert: '0x',
            storagePrefix: '',
          },
        ],
        softwareVersion: '0.0.0-e2e',
        pcrValues: [],
      },
    });

    // --- 4. The executor proxy really was re-pointed at a new (v14) implementation. ---
    const executorImplAfter = await publicClient.getStorageAt({ address: executor, slot: IMPL_SLOT });
    expect(executorImplAfter).not.toBe(executorImplBefore);
    expect(BigInt(executorImplAfter ?? '0x0')).not.toBe(0n);

    // --- 5. Cleartext still works after the upgrade (new v14 executor impl → re-pointed
    //        CleartextArithmetic → CleartextDB), and the pre-upgrade value persisted. ---
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

    // --- 6. ProtocolConfig state survived the reinitializeV2 migration: the KMS signer set installed
    //        by the v13 bootstrap is still readable through the upgraded KMSVerifier. ---
    const signers = await publicClient.readContract({
      address: v13.fhevmAddresses.kmsVerifierAddress as Address,
      abi: KMS_VERIFIER_ABI,
      functionName: 'getKmsSigners',
    });
    expect(signers.map((s) => s.toLowerCase())).toContain(kmsSigner.toLowerCase());
  } finally {
    await stopAnvil(anvil.process);
  }
}, 180_000);

test('e2e: updateV13ToV14 with no migration config — defaults resolved from the live v13 ProtocolConfig', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });
  const deployerAddress = privateKeyToAddress({ privateKey: deployerKey });

  const anvil = startAnvil({ port: 8621, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
    const publicClient = createPublicClient({ chain: foundry, transport: http(anvil.rpcUrl) });
    const wallet = createWalletClient({
      account: privateKeyToAccount(deployerKey),
      chain: foundry,
      transport: http(anvil.rpcUrl),
    });

    // --- 1. Deploy a fresh v13 stack whose KMS signer set is the package defaults. ---
    const precomputed = precomputeV13({ ethUtils: adapters.utils, from: deployerAddress, startNonce: 0n });
    const v13 = await deployV13({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
      precomputed: {
        fhevmAddresses: precomputed.fhevmAddresses,
        cleartextAddresses: precomputed.cleartextAddresses,
        pauserSetAddress: precomputed.pauserSetAddress,
      },
      // No config → deployV13 uses DEFAULT_BOOTSTRAP_CONFIG_V13, whose KMS signer set is the package
      // defaults. That is exactly the stack the no-migration upgrade path assumes (and the reason it
      // works: v13's default KMS signers ARE v14's default KMS signers).
    });

    const executor = v13.fhevmAddresses.fhevmExecutorAddress as Address;
    const cleartextDb = v13.cleartextAddresses.cleartextDbAddress as Address;
    const kmsVerifier = v13.fhevmAddresses.kmsVerifierAddress as Address;
    const protocolConfig = v13.fhevmAddresses.protocolConfigAddress as Address;

    const readContextId = (address: Address): Promise<bigint> =>
      publicClient.readContract({ address, abi: KMS_CONTEXT_ABI, functionName: 'getCurrentKmsContextId' });
    const readSigners = async (address: Address): Promise<string[]> =>
      [...(await publicClient.readContract({ address, abi: KMS_CONTEXT_ABI, functionName: 'getKmsSigners' }))].map(
        (s) => s.toLowerCase(),
      );

    // Capture the live v13 KMS context (id + signers). These are the values the no-migration path must
    // preserve; the post-upgrade assertions compare against them. A freshly deployed default stack
    // reports its 4 default KMS nodes.
    const v13ContextId = await readContextId(kmsVerifier);
    const v13Signers = await readSigners(kmsVerifier);
    expect(v13Signers).toHaveLength(4);

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

    // --- 2. Pre-upgrade round-trip: record a cleartext under the v13 executor. ---
    const handleBefore = await trivialEncrypt(42n);

    // --- 3. Upgrade WITHOUT a migration config — it is resolved from the live v13 ProtocolConfig +
    //        defaults (v14-shaped node params rebuilt from the default pool). ---
    await updateV13ToV14({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
      aclOwnerAddress: v13.aclOwnerAddress,
      existing: { ...v13.fhevmAddresses, pauserSetAddress: v13.pauserSetAddress },
      cleartext: v13.cleartextAddresses,
      // migration intentionally omitted — resolveDefaultMigration fills it from chain + defaults.
    });

    // --- 4. ProtocolConfig state survived the reinitializeV2 migration: the live v13 context id and
    //        KMS signer set carried over unchanged. ---
    expect(await readContextId(protocolConfig)).toBe(v13ContextId);
    expect(await readSigners(protocolConfig)).toEqual(v13Signers);

    // --- 5. Cleartext still works after the upgrade, and the pre-upgrade value persisted. ---
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
