import { readFileSync } from 'node:fs';
import { deploy, precomputeAddresses, type BootstrapConfigV14 } from '@fhevm/host-contracts-cleartext/ts';
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
const FHE_TYPE_UINT8 = 2; // FheType.Uint8

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

function bootstrapConfig(deployerAddress: string, kmsSigner: string): BootstrapConfigV14 {
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
      initialKmsNodeParams: [
        {
          txSenderAddress: deployerAddress,
          signerAddress: kmsSigner,
          ipAddress: '127.0.0.1',
          storageUrl: 'https://kms.example',
          partyId: 1,
          mpcIdentity: 'kms-node-1',
          caCert: '0x',
          storagePrefix: 'kms',
        },
      ],
      initialThresholds: { publicDecryption: 1n, userDecryption: 1n, kmsGen: 1n, mpc: 1n },
      softwareVersion: '0.14.0',
      pcrValues: [],
    },
  };
}

test('full deploy of a brand-new v14 stack: all proxies materialize and cleartext round-trips', async () => {
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

    // (d) v0.14's new op, fheMulDiv, mirrors into the cleartext layer. Driven through MulDivProbe
    //     because the executor grants operand access with `ACL.allowTransient` (transient storage):
    //     an EOA cannot encrypt in one tx and mul-div in the next, so both must happen in one call.
    const probeArtifact = JSON.parse(
      readFileSync(new URL('../../out/MulDivProbe.sol/MulDivProbe.json', import.meta.url), 'utf8'),
    ) as { abi: readonly unknown[]; bytecode: { object: Hex } };

    const probeDeployHash = await wallet.deployContract({
      abi: probeArtifact.abi,
      bytecode: probeArtifact.bytecode.object,
      args: [executor],
    });
    const probeReceipt = await publicClient.waitForTransactionReceipt({ hash: probeDeployHash });
    const probeAddress = probeReceipt.contractAddress;
    if (probeAddress === undefined || probeAddress === null) {
      throw new Error('MulDivProbe deployment produced no contract address');
    }

    const mulDivCases = [
      // (7 * 6) / 4 == 10. Run with factor2 encrypted and again as a scalar: `fheMulDiv` inverts the
      // usual scalar bitmask (0x01 = encrypted, 0x03 = scalar), so a flipped convention fails here.
      { a: 7n, b: 6n, divisor: 4n, fheType: FHE_TYPE_UINT64, factor2Scalar: false, expected: 10n },
      { a: 7n, b: 6n, divisor: 4n, fheType: FHE_TYPE_UINT64, factor2Scalar: true, expected: 10n },
      // Intermediate widening, on uint8: 200 * 3 == 600 overflows the type, but the product is taken
      // at full precision before dividing, so (200 * 3) / 4 == 150. Clamping the product first would
      // give (600 & 0xff) / 4 == 22.
      { a: 200n, b: 3n, divisor: 4n, fheType: FHE_TYPE_UINT8, factor2Scalar: false, expected: 150n },
    ];

    for (const { a, b, divisor, fheType, factor2Scalar, expected } of mulDivCases) {
      const label = `mulDiv(${a}, ${b}, ${divisor}) type=${fheType} scalar=${factor2Scalar}`;
      const runHash = await wallet.writeContract({
        address: probeAddress,
        abi: probeArtifact.abi,
        functionName: 'run',
        args: [a, b, divisor, fheType, factor2Scalar],
      });
      await publicClient.waitForTransactionReceipt({ hash: runHash });

      const resultHandle = (await publicClient.readContract({
        address: probeAddress,
        abi: probeArtifact.abi,
        functionName: 'lastResult',
      })) as Hex;

      const mulDivCleartext = await publicClient.readContract({
        address: cleartextDb,
        abi: CLEARTEXT_DB_ABI,
        functionName: 'get',
        args: [resultHandle],
      });
      expect(mulDivCleartext, label).toBe(expected);
    }
  } finally {
    await stopAnvil(anvil.process);
  }
}, 120_000);
