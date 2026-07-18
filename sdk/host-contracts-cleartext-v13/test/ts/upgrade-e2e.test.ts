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

// getCurrentKmsContextId + getKmsSigners: identical signatures on the v12 `KMSVerifier` and v13 `ProtocolConfig`.
const KMS_CONTEXT_ABI = [
  {
    type: 'function',
    name: 'getCurrentKmsContextId',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'uint256' }],
  },
  {
    type: 'function',
    name: 'getKmsSigners',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'address[]' }],
  },
] as const;

// v12 `KMSVerifier` stored a single KMS threshold.
const KMS_THRESHOLD_V12_ABI = [
  { type: 'function', name: 'getThreshold', stateMutability: 'view', inputs: [], outputs: [{ type: 'uint256' }] },
] as const;

// v13 `ProtocolConfig` splits the KMS threshold into four per-operation thresholds.
const uint256Getter = (name: string) =>
  ({ type: 'function', name, stateMutability: 'view', inputs: [], outputs: [{ type: 'uint256' }] }) as const;
const KMS_THRESHOLDS_V13_ABI = [
  uint256Getter('getPublicDecryptionThreshold'),
  uint256Getter('getUserDecryptionThreshold'),
  uint256Getter('getKmsGenThreshold'),
  uint256Getter('getMpcThreshold'),
] as const;
const KMS_THRESHOLD_GETTERS = [
  'getPublicDecryptionThreshold',
  'getUserDecryptionThreshold',
  'getKmsGenThreshold',
  'getMpcThreshold',
] as const;

// The `InputVerifier` holds the coprocessor signer set — untouched by the v12→v13 upgrade.
const INPUT_VERIFIER_ABI = [
  {
    type: 'function',
    name: 'getCoprocessorSigners',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'address[]' }],
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

// Every host + cleartext contract exposes ACL-style `getVersion()` → "<Name> vMAJOR.MINOR.PATCH".
const VERSION_ABI = [
  {
    type: 'function',
    name: 'getVersion',
    stateMutability: 'pure',
    inputs: [],
    outputs: [{ type: 'string' }],
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

    const version = (address: string): Promise<string> =>
      publicClient.readContract({ address: address as Address, abi: VERSION_ABI, functionName: 'getVersion' });

    // The freshly deployed stack reports v12 versions across every host + cleartext contract.
    expect({
      acl: await version(v12.fhevmAddresses.aclAddress),
      fhevmExecutor: await version(v12.fhevmAddresses.fhevmExecutorAddress),
      kmsVerifier: await version(v12.fhevmAddresses.kmsVerifierAddress),
      inputVerifier: await version(v12.fhevmAddresses.inputVerifierAddress),
      hcuLimit: await version(v12.fhevmAddresses.hcuLimitAddress),
      cleartextArithmetic: await version(v12.cleartextAddresses.cleartextArithmeticAddress),
    }).toEqual({
      acl: 'ACL v0.3.0',
      fhevmExecutor: 'FHEVMExecutor v0.3.0',
      kmsVerifier: 'KMSVerifier v0.2.0',
      inputVerifier: 'InputVerifier v0.2.0',
      hcuLimit: 'HCULimit v0.2.0',
      cleartextArithmetic: 'CleartextArithmetic v0.3.0',
    });

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

    // --- 4. The two new v13 proxies are materialized, and CleartextArithmetic was re-pointed at the
    //        v13 implementation (its reported version bumped v0.3.0 → v0.4.0). ---
    for (const proxy of [migrated.protocolConfigAddress, migrated.kmsGenerationAddress] as const) {
      const impl = await publicClient.getStorageAt({ address: proxy as Address, slot: IMPL_SLOT });
      expect(BigInt(impl ?? '0x0')).not.toBe(0n);
    }
    // Every re-pointed proxy now reports its v13 version; the two new proxies report their initial
    // version; InputVerifier is intentionally left at v0.2.0 (its v13 bytecode is unchanged).
    expect({
      acl: await version(v12.fhevmAddresses.aclAddress),
      fhevmExecutor: await version(v12.fhevmAddresses.fhevmExecutorAddress),
      kmsVerifier: await version(v12.fhevmAddresses.kmsVerifierAddress),
      inputVerifier: await version(v12.fhevmAddresses.inputVerifierAddress),
      hcuLimit: await version(v12.fhevmAddresses.hcuLimitAddress),
      cleartextArithmetic: await version(v12.cleartextAddresses.cleartextArithmeticAddress),
      protocolConfig: await version(migrated.protocolConfigAddress),
      kmsGeneration: await version(migrated.kmsGenerationAddress),
    }).toEqual({
      acl: 'ACL v0.4.0',
      fhevmExecutor: 'FHEVMExecutor v0.4.0',
      kmsVerifier: 'KMSVerifier v0.3.0',
      inputVerifier: 'InputVerifier v0.2.0',
      hcuLimit: 'HCULimit v0.3.0',
      cleartextArithmetic: 'CleartextArithmetic v0.4.0',
      protocolConfig: 'ProtocolConfig v0.1.0',
      kmsGeneration: 'KMSGeneration v0.1.0',
    });

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

test('e2e: updateV12ToV13 with no migration config — defaults resolved from the live v12 KMSVerifier', async () => {
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

    // --- 1. Deploy a fresh v12 stack whose KMS signer set is the package defaults. ---
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
      // No config → deployV12 uses DEFAUT_BOOTSTRAP_CONFIG_V12, whose KMS signer set is the package
      // defaults. That is exactly the stack the no-migration upgrade path assumes (and the reason it
      // works: v12's default KMS signers ARE v13's default KMS signers).
    });

    const executor = v12.fhevmAddresses.fhevmExecutorAddress as Address;
    const cleartextDb = v12.cleartextAddresses.cleartextDbAddress as Address;
    const kmsVerifier = v12.fhevmAddresses.kmsVerifierAddress as Address;
    const inputVerifier = v12.fhevmAddresses.inputVerifierAddress as Address;

    const readContextId = (address: Address): Promise<bigint> =>
      publicClient.readContract({ address, abi: KMS_CONTEXT_ABI, functionName: 'getCurrentKmsContextId' });
    const readSigners = async (address: Address): Promise<string[]> =>
      [...(await publicClient.readContract({ address, abi: KMS_CONTEXT_ABI, functionName: 'getKmsSigners' }))].map(
        (s) => s.toLowerCase(),
      );
    const readCoprocessorSigners = async (): Promise<string[]> =>
      [
        ...(await publicClient.readContract({
          address: inputVerifier,
          abi: INPUT_VERIFIER_ABI,
          functionName: 'getCoprocessorSigners',
        })),
      ].map((s) => s.toLowerCase());

    // Capture the live v12 KMS context (id + signers + threshold) and coprocessor signer set. These are
    // the values the no-migration path must preserve; the post-upgrade assertions compare against them.
    // A freshly deployed default stack reports the minimum valid context id and its 4 default KMS nodes.
    const v12ContextId = await readContextId(kmsVerifier);
    const v12Signers = await readSigners(kmsVerifier);
    const v12Threshold = await publicClient.readContract({
      address: kmsVerifier,
      abi: KMS_THRESHOLD_V12_ABI,
      functionName: 'getThreshold',
    });
    const v12CoprocessorSigners = await readCoprocessorSigners();
    expect(v12ContextId).toBe(MIGRATED_CONTEXT_ID);
    expect(v12Signers).toHaveLength(4);

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

    // --- 3. Upgrade WITHOUT a migration config — it is resolved from the live v12 KMSVerifier + defaults. ---
    const migrated = await updateV12ToV13({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
      aclOwnerAddress: v12.aclOwnerAddress,
      existing: { ...v12.fhevmAddresses, pauserSetAddress: v12.pauserSetAddress },
      cleartext: v12.cleartextAddresses,
      // migration intentionally omitted — resolveDefaultMigration fills it from chain + defaults.
    });

    // --- 4. ProtocolConfig was seeded from the resolved migration: the live v12 context id, KMS signer
    //        set, and threshold all carried over unchanged. ---
    const protocolConfig = migrated.protocolConfigAddress as Address;
    expect(await readContextId(protocolConfig)).toBe(v12ContextId);
    expect(await readSigners(protocolConfig)).toEqual(v12Signers);
    // v12's single threshold is carried into all four v13 per-operation thresholds.
    for (const getter of KMS_THRESHOLD_GETTERS) {
      expect(
        await publicClient.readContract({ address: protocolConfig, abi: KMS_THRESHOLDS_V13_ABI, functionName: getter }),
      ).toBe(v12Threshold);
    }
    // The coprocessor signer set lives in InputVerifier, which the upgrade leaves untouched — unchanged.
    expect(await readCoprocessorSigners()).toEqual(v12CoprocessorSigners);
    expect(v12CoprocessorSigners.length).toBeGreaterThan(0);

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
