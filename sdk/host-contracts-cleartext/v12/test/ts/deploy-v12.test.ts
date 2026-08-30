import {
  deploy,
  pauseACL,
  precomputeAddresses,
  unpauseACL,
  type BootstrapConfig,
  type FhevmAddresses,
  CONTRACT_VERSIONS,
  snapshotStack,
  verify,
} from '../../pkg/ts/index.ts';
import { createPublicClient, createWalletClient, http, parseEventLogs, type Address, type Hex } from 'viem';
import { mnemonicToAccount, privateKeyToAccount } from 'viem/accounts';
import { foundry } from 'viem/chains';
import { expect, test } from 'vitest';
import { startAnvil, stopAnvil, waitForAnvil } from '@fhevm/sdk-common-dev';
import { privateKeyFromMnemonic, privateKeyToAddress } from '@fhevm/sdk-common-dev';
import { expectedHcuLimit } from './utils/expectedBootstrap.ts';
import { createViemEthereumAdapters, createViemEthereumHistory } from '@fhevm/sdk-vendored-dev/viemEthereumLib.ts';

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
  { type: 'function', name: 'getACLAddress', stateMutability: 'view', inputs: [], outputs: [{ type: 'address' }] },
  { type: 'function', name: 'getHCULimitAddress', stateMutability: 'view', inputs: [], outputs: [{ type: 'address' }] },
  {
    type: 'function',
    name: 'getInputVerifierAddress',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'address' }],
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
  {
    type: 'function',
    name: 'getFHEVMExecutorAddress',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'address' }],
  },
  {
    type: 'function',
    name: 'getPauserSetAddress',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'address' }],
  },
] as const;

const HCU_LIMIT_ABI = [
  {
    type: 'function',
    name: 'getFHEVMExecutorAddress',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'address' }],
  },
] as const;

const GET_VERSION_ABI = [
  { type: 'function', name: 'getVersion', stateMutability: 'pure', inputs: [], outputs: [{ type: 'string' }] },
] as const;

/**
 * `getVersion()` for every deployed contract that exposes it, as `"<CONTRACT_NAME> v<MAJOR>.<MINOR>.<PATCH>"`.
 *
 * This is the strongest identity check available from outside: it proves the implementation sitting
 * behind each proxy is the contract we believe it is, where a non-zero implementation slot only
 * proves *something* is there.
 *
 * Two things look wrong here and are not. The cleartext variants report their **base** contract's
 * name — `CONTRACT_NAME` is a `private constant` read by the base's `getVersion`, and the subclasses
 * override neither — so `CleartextFHEVMExecutor` identifies itself as `FHEVMExecutor`.
 * `CleartextArithmetic` is the lone exception, declaring its own. And `CleartextDB` is absent
 * because it exposes no `getVersion` at all.
 *
 * These strings track upstream's `MINOR_VERSION` constants, so they move when a contract is bumped
 * during a generation sync — see README step 7, which is also where `reinitializeV<n>` is checked
 * (the reinitializer number tracks the same minor version).
 */
const EXPECTED_VERSIONS = CONTRACT_VERSIONS;

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

/**
 * Reads back, from the live chain, the addresses that were patched into the deployed bytecode.
 *
 * This is the only assertion that closes the loop on placeholder patching. Comparing the deploy's
 * returned addresses against `precomputeAddresses` proves nothing about the patch — both sides come
 * from the same derivation — and a non-zero implementation slot only proves *something* is deployed
 * there. Mis-patching (the right address written into the wrong constant's offsets) leaves every
 * proxy correct and every impl slot non-zero, and would go entirely unnoticed without these calls.
 *
 * Nothing during deploy exercises the wiring: the initializers do not call across contracts, so a
 * mis-patched address does not revert. It only surfaces here, or in production.
 *
 * The six getters below cover five distinct patched constants across three contracts — the same
 * probes `scripts/anvil.sh` runs, plus `getHCULimitAddress` and `getPauserSetAddress`.
 */
async function expectPatchedWiring(parameters: {
  readonly publicClient: ReturnType<typeof createPublicClient>;
  readonly fhevmAddresses: FhevmAddresses;
  readonly pauserSetAddress: string;
}): Promise<void> {
  const { publicClient, fhevmAddresses, pauserSetAddress } = parameters;
  const acl = fhevmAddresses.aclAddress as Address;
  const executor = fhevmAddresses.fhevmExecutorAddress as Address;
  const hcuLimit = fhevmAddresses.hcuLimitAddress as Address;
  // Getters return checksummed addresses; the derived ones are not necessarily checksummed.
  const lower = (value: string): string => value.toLowerCase();

  expect(
    lower(await publicClient.readContract({ address: acl, abi: ACL_ABI, functionName: 'getFHEVMExecutorAddress' })),
  ).toBe(lower(executor));
  expect(
    lower(await publicClient.readContract({ address: acl, abi: ACL_ABI, functionName: 'getPauserSetAddress' })),
  ).toBe(lower(pauserSetAddress));

  expect(
    lower(await publicClient.readContract({ address: executor, abi: EXECUTOR_ABI, functionName: 'getACLAddress' })),
  ).toBe(lower(acl));
  expect(
    lower(
      await publicClient.readContract({ address: executor, abi: EXECUTOR_ABI, functionName: 'getHCULimitAddress' }),
    ),
  ).toBe(lower(hcuLimit));
  expect(
    lower(
      await publicClient.readContract({
        address: executor,
        abi: EXECUTOR_ABI,
        functionName: 'getInputVerifierAddress',
      }),
    ),
  ).toBe(lower(fhevmAddresses.inputVerifierAddress));

  expect(
    lower(
      await publicClient.readContract({
        address: hcuLimit,
        abi: HCU_LIMIT_ABI,
        functionName: 'getFHEVMExecutorAddress',
      }),
    ),
  ).toBe(lower(executor));
}

function bootstrapConfig(deployerAddress: string, kmsSignerAddress: string): BootstrapConfig {
  return {
    kmsVerifier: {
      verifyingContractSource: deployerAddress,
      chainIDSource: 1n,
      // This generation registers the KMS signer set on the verifier itself.
      initialSigners: [kmsSignerAddress],
      initialThreshold: 1n,
    },
    inputVerifier: {
      verifyingContractSource: deployerAddress,
      chainIDSource: 1n,
      initialSigners: [deployerAddress],
      initialThreshold: 1n,
    },
    hcuLimit: expectedHcuLimit(),
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
      deployed.cleartextAddresses.cleartextArithmeticAddress,
      deployed.cleartextAddresses.cleartextDbAddress,
    ].map((address) => address as Address);
    for (const proxy of proxies) {
      const impl = await publicClient.getStorageAt({ address: proxy, slot: IMPL_SLOT });
      expect(BigInt(impl ?? '0x0'), `impl slot for proxy ${proxy}`).not.toBe(0n);
    }

    // (a2) The addresses patched into the deployed bytecode match the stack that was deployed.
    await expectPatchedWiring({
      publicClient,
      fhevmAddresses: deployed.fhevmAddresses,
      pauserSetAddress: deployed.pauserSetAddress,
    });

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
    //     The DB — proves executor → CleartextArithmetic → CleartextDB wiring end to end.
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

    // (b) A contract really exists at the derived ACL address — this is what catches a bad derivation,
    //     since a miscomputed address would hold nothing. It says nothing about the wiring: the
    //     initializers make no cross-contract calls, so mis-patched addresses do not revert on deploy.
    const impl = await publicClient.getStorageAt({
      address: deployed.fhevmAddresses.aclAddress as Address,
      slot: IMPL_SLOT,
    });
    expect(BigInt(impl ?? '0x0')).not.toBe(0n);

    // (c) The addresses patched into the deployed bytecode are the ones we derived.
    await expectPatchedWiring({
      publicClient,
      fhevmAddresses: deployed.fhevmAddresses,
      pauserSetAddress: deployed.pauserSetAddress,
    });
  } finally {
    await stopAnvil(anvil.process);
  }
}, 120_000);

test('every deployed contract reports its expected getVersion()', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });

  const anvil = startAnvil({ port: 8612, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
    const publicClient = createPublicClient({ chain: foundry, transport: http(anvil.rpcUrl) });

    const deployed = await deploy({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
    });

    const cases: ReadonlyArray<readonly [keyof typeof EXPECTED_VERSIONS, string]> = [
      ['acl', deployed.fhevmAddresses.aclAddress],
      ['fhevmExecutor', deployed.fhevmAddresses.fhevmExecutorAddress],
      ['inputVerifier', deployed.fhevmAddresses.inputVerifierAddress],
      ['kmsVerifier', deployed.fhevmAddresses.kmsVerifierAddress],
      ['hcuLimit', deployed.fhevmAddresses.hcuLimitAddress],
      ['pauserSet', deployed.pauserSetAddress],
      ['cleartextArithmetic', deployed.cleartextAddresses.cleartextArithmeticAddress],
    ];

    // Guard against a contract being added to the stack and silently escaping this check.
    expect(cases.length, 'every getVersion-bearing contract must be covered').toBe(
      Object.keys(EXPECTED_VERSIONS).length,
    );

    for (const [key, address] of cases) {
      const version = await publicClient.readContract({
        address: address as Address,
        abi: GET_VERSION_ABI,
        functionName: 'getVersion',
      });
      expect(version, `${key} at ${address}`).toBe(EXPECTED_VERSIONS[key]);
    }
  } finally {
    await stopAnvil(anvil.process);
  }
}, 120_000);

// The signer mnemonic, distinct from MNEMONIC above (which funds the deploy). The js-sdk cleartext
// relayer derives its signing keys from this one, at these HD change indices.
const FHEVM_MNEMONIC = 'test test test test test test test future home engine virtual motion';
const COPROCESSOR_CHANGE_INDEX = 2;
const KMS_CHANGE_INDEX = 3;
const DEFAULT_SIGNER_COUNT = 4; // DEFAULT_NUM_COPROCESSORS / DEFAULT_NUM_KMS_NODES

const INPUT_VERIFIER_ABI = [
  {
    type: 'function',
    name: 'getCoprocessorSigners',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ type: 'address[]' }],
  },
  { type: 'function', name: 'getThreshold', stateMutability: 'view', inputs: [], outputs: [{ type: 'uint256' }] },
] as const;

const KMS_VERIFIER_ABI = [
  { type: 'function', name: 'getKmsSigners', stateMutability: 'view', inputs: [], outputs: [{ type: 'address[]' }] },
  { type: 'function', name: 'getThreshold', stateMutability: 'view', inputs: [], outputs: [{ type: 'uint256' }] },
] as const;

/** Derived here rather than read from `ts/signers/`, so this checks the SDK's derivation, not our copy. */
function derivedSigners(changeIndex: number): readonly string[] {
  return Array.from({ length: DEFAULT_SIGNER_COUNT }, (_unused, index) =>
    mnemonicToAccount(FHEVM_MNEMONIC, { changeIndex, addressIndex: index }).address.toLowerCase(),
  );
}

/**
 * `deploy()` with no `config` must apply DEFAULT_BOOTSTRAP_CONFIG — the stack the js-sdk cleartext
 * client can actually use.
 *
 * This is the gap that let `deployStack.ts` ship a hand-rolled config registering the deployer as the sole
 * coprocessor signer: every address was right, every contract was live, and no test noticed, because the
 * relayer only fails later when it is asked to sign as a signer it has no key for.
 */
test('deploy with no config registers the signers the SDK derives', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });

  const anvil = startAnvil({ port: 8613, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
    const publicClient = createPublicClient({ chain: foundry, transport: http(anvil.rpcUrl) });

    // No `config` and no `precomputed`: exactly what a caller gets by default.
    const deployed = await deploy({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
    });

    const coprocessors = await publicClient.readContract({
      address: deployed.fhevmAddresses.inputVerifierAddress as Address,
      abi: INPUT_VERIFIER_ABI,
      functionName: 'getCoprocessorSigners',
    });
    expect(coprocessors.map((address) => address.toLowerCase())).toEqual(derivedSigners(COPROCESSOR_CHANGE_INDEX));

    expect(
      await publicClient.readContract({
        address: deployed.fhevmAddresses.inputVerifierAddress as Address,
        abi: INPUT_VERIFIER_ABI,
        functionName: 'getThreshold',
      }),
    ).toBe(BigInt(DEFAULT_SIGNER_COUNT));

    // This generation keeps the KMS signer set and threshold on KMSVerifier itself.
    const kmsSigners = await publicClient.readContract({
      address: deployed.fhevmAddresses.kmsVerifierAddress as Address,
      abi: KMS_VERIFIER_ABI,
      functionName: 'getKmsSigners',
    });
    expect(kmsSigners.map((address) => address.toLowerCase())).toEqual(derivedSigners(KMS_CHANGE_INDEX));

    expect(
      await publicClient.readContract({
        address: deployed.fhevmAddresses.kmsVerifierAddress as Address,
        abi: KMS_VERIFIER_ABI,
        functionName: 'getThreshold',
      }),
    ).toBe(BigInt(DEFAULT_SIGNER_COUNT));
  } finally {
    await stopAnvil(anvil.process);
  }
}, 120_000);

////////////////////////////////////////////////////////////////////////////////
// The public `verify` entry point
////////////////////////////////////////////////////////////////////////////////

/**
 * The `history` capability `verify` needs beyond `AbstractEthereumProvider`, over viem.
 *
 * Written out here rather than shipped as a helper on purpose: it is the reference implementation a
 * consumer copies, so if it is longer than a few lines the interface is wrong. It is not.
 */
test('verify reports a freshly deployed stack as sound', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });
  const adminKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 6 });

  const anvil = startAnvil({ port: 8616, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
    const adminAdapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: adminKey });
    const history = createViemEthereumHistory(anvil.rpcUrl);

    const deployed = await deploy({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adminAdapters.signer,
    });

    const report = await verify({
      mode: 'deploy',
      ethProvider: adapters.provider,
      history,
      deployed,
      // Supplied, not derived: `verify` cannot know who SHOULD own a stack, and reports a skip rather
      // than assuming. Passing it is what turns "somebody owns this" into "the right account does".
      expected: { admin: privateKeyToAddress({ privateKey: adminKey }) },
    });

    expect(
      report.failures.map((f) => `${f.name}: ${f.detail ?? ''}`),
      'a stack deploy() just built must verify clean',
    ).toEqual([]);
    expect(report.ok).toBe(true);

    // A skip is not a pass, so the count is pinned. With `history` supplied and an admin expectation
    // given, everything except the unstated bootstrap expectations should have RUN.
    expect(
      report.skipped.map((s) => s.name),
      'only checks with no stated expectation may be skipped here',
    ).toEqual([]);

    // Non-vacuity: the report must actually contain the checks it claims to make. A verify() that ran
    // nothing would satisfy every assertion above.
    const names = report.checks.map((c) => c.name);
    for (const prefix of ['code.', 'materialized.', 'version.', 'wiring.', 'ownership.', 'pausers.']) {
      expect(names.filter((n) => n.startsWith(prefix)).length, `verify ran no ${prefix} checks`).toBeGreaterThan(0);
    }
    expect(names.length, 'verify should run dozens of checks, not a handful').toBeGreaterThan(25);
  } finally {
    await stopAnvil(anvil.process);
  }
}, 180_000);

test('verify catches a stack whose admin never accepted ownership', async () => {
  // The one failure mode that leaves everything else looking perfect: the deployer is still root over the
  // stack, every version and every wired address is correct, and only `ACLOwner.owner()` gives it away.
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });

  const anvil = startAnvil({ port: 8617, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
    const deployed = await deploy({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
    });

    const report = await verify({
      mode: 'deploy',
      ethProvider: adapters.provider,
      history: createViemEthereumHistory(anvil.rpcUrl),
      deployed,
      // Deliberately wrong: claim an admin that never took ownership.
      expected: {
        admin: privateKeyToAddress({ privateKey: privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 7 }) }),
      },
    });

    expect(report.ok).toBe(false);
    expect(
      report.failures.map((f) => f.name),
      'the wrong-admin claim must be the only thing that fails',
    ).toEqual(['ownership.ACLOwner.owner']);
  } finally {
    await stopAnvil(anvil.process);
  }
}, 180_000);

test('snapshotStack captures every readable value, and verify needs one for upgrade mode', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: 5 });

  const anvil = startAnvil({ port: 8618, mnemonic: MNEMONIC });
  try {
    await waitForAnvil(anvil.rpcUrl);

    const adapters = createViemEthereumAdapters({ rpcUrl: anvil.rpcUrl, privateKey: deployerKey });
    const history = createViemEthereumHistory(anvil.rpcUrl);
    const deployed = await deploy({
      ethProvider: adapters.provider,
      ethUtils: adapters.utils,
      deployer: adapters.signer,
      admin: adapters.signer,
    });

    const before = await snapshotStack({ ethProvider: adapters.provider, history, deployed });
    // The survey's whole value is breadth — a handful of readings would mean the ABI enumeration silently
    // matched almost nothing, and every survival comparison built on it would be vacuous.
    expect(Object.keys(before.readings).length, 'the survey must cover the stack broadly').toBeGreaterThan(30);
    expect(before.blockNumber, 'a snapshot taken with history must carry a height').not.toBeNull();

    // Comparing a stack against a snapshot of ITSELF is the degenerate upgrade: nothing moved, so every
    // exempt reading is an exemption that did not fire — which verify is required to report, because that
    // is what stops the allow-list decaying into a way to ignore regressions.
    const report = await verify({ mode: 'upgrade', ethProvider: adapters.provider, history, deployed, before });
    expect(report.failures.map((f) => f.name)).toContain('survival.exemptionsWereUsed');
    expect(
      report.failures.map((f) => f.name),
      'nothing actually changed, so no value may be reported as having changed',
    ).not.toContain('survival.everythingElseUnchanged');

    // With nothing exempt, a stack compared against itself must show no drift at all.
    const strict = await verify({
      mode: 'upgrade',
      ethProvider: adapters.provider,
      history,
      deployed,
      before,
      mayChange: [],
    });
    expect(
      strict.failures.map((f) => `${f.name}: ${f.detail ?? ''}`),
      'a stack must survive being compared against itself',
    ).toEqual([]);
  } finally {
    await stopAnvil(anvil.process);
  }
}, 180_000);
