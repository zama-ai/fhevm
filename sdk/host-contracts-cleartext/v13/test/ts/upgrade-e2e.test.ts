// The previous generation, imported straight from the WORKSPACE — v12 is a member, so npm has already
// linked it into node_modules and there is nothing to build, pack or install first. The subpath works
// because a harness manifest declares no `exports` (ARCHITECTURE.md I2); this import is what keeps that
// mechanism exercised rather than merely documented.
//
// Deliberately NOT a tarball. A tarball buys publish fidelity — `files` omissions, undeclared deps,
// stale build output — and that is worth paying for when the package under test is the thing being
// published. Here v12 is just a library used to stand up the "before" stack, and its own publish
// contract is proven by its own `test:tarball:run`. v13, below, is still consumed by its PUBLISHED name,
// because that half IS the publish rehearsal.
//
// Its exports are UNSUFFIXED — a package is pinned to one generation by its own version, so the
// specifier is what says which generation this is, not the type name. Aliased on the way in, because
// this file is the one place where both generations are in scope at once.
import {
  deploy as deployV12,
  precomputeAddresses as precomputeV12,
  type BootstrapConfig as BootstrapConfigV12,
} from '@fhevm/host-contracts-cleartext-v12-dev/pkg/ts/index.ts';
import { updateV12ToV13 } from '../../pkg/ts/index.ts';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createPublicClient, createWalletClient, http, parseEventLogs, type Address, type Hex } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { foundry } from 'viem/chains';
import { expect, test } from 'vitest';
import {
  startAnvil,
  stopAnvil,
  waitForAnvil,
  MNEMONIC,
  DEPLOYER_ADDRESS_INDEX,
  ERC_1967_IMPL_SLOT,
} from '@fhevm/sdk-common-dev';
import { privateKeyFromMnemonic, privateKeyToAddress } from '@fhevm/sdk-common-dev';
import { expectedHcuLimit } from './utils/expectedBootstrap.ts';
import { createViemEthereumAdapters } from '@fhevm/sdk-vendored-dev/viemEthereumLib.ts';

const IMPL_SLOT = ERC_1967_IMPL_SLOT;
const FHE_TYPE_UINT64 = 5;
// KMS_CONTEXT_COUNTER_BASE + 1 = (0x07 << 248) + 1 — the minimum valid migrated KMS context id.
const MIGRATED_CONTEXT_ID = (7n << 248n) + 1n;

////////////////////////////////////////////////////////////////////////////////

/**
 * Every zero-argument view getter the LIVE v12 stack exposes, read from the v12 fixture's own ABIs.
 *
 * Enumerated rather than listed, deliberately. The question this answers is "does anything readable
 * change across the upgrade that should not", and a hand-written list can only answer it for the getters
 * whoever wrote the list thought of. Reading the previous generation's ABIs means a getter added upstream
 * is covered with no edit here — and a getter REMOVED by the new generation shows up as a revert, which is
 * itself a break worth failing on.
 *
 * Zero-argument only, because those are the ones whose value is a property of the stack rather than of an
 * argument. `ACL.isAllowed(handle, account)` matters just as much, but it is covered by the cleartext
 * round-trip instead, which exercises it with real handles.
 *
 * Located by asking the module system, not by walking a relative path: v12 is a workspace member, so
 * resolution already knows where it is and this cannot drift if the layout moves. Resolving a real file
 * inside the directory rather than the package root also makes it a live assertion that the subpath
 * reach-in of I2 still works — if v12's harness ever gained an `exports` map this throws
 * ERR_PACKAGE_PATH_NOT_EXPORTED right here, instead of surfacing as a puzzling missing-ABI much later.
 */
const V12_ABI_DIR = dirname(
  fileURLToPath(import.meta.resolve('@fhevm/host-contracts-cleartext-v12-dev/pkg/abi/ACL.json')),
);

type AbiEntry = {
  readonly type?: string;
  readonly name?: string;
  readonly stateMutability?: string;
  readonly inputs?: readonly unknown[];
};

/** The contracts a live v12 stack actually has, paired with the ABI file that describes each. */
function surveyTargets(deployed: {
  readonly fhevmAddresses: Record<string, string>;
  readonly cleartextAddresses: Record<string, string>;
  readonly pauserSetAddress: string;
  readonly aclOwnerAddress: string;
}): ReadonlyArray<{ readonly label: string; readonly abiFile: string; readonly address: Address }> {
  return [
    { label: 'ACL', abiFile: 'ACL.json', address: deployed.fhevmAddresses.aclAddress as Address },
    {
      label: 'FHEVMExecutor',
      abiFile: 'CleartextFHEVMExecutor.json',
      address: deployed.fhevmAddresses.fhevmExecutorAddress as Address,
    },
    {
      label: 'KMSVerifier',
      abiFile: 'CleartextKMSVerifier.json',
      address: deployed.fhevmAddresses.kmsVerifierAddress as Address,
    },
    {
      label: 'InputVerifier',
      abiFile: 'CleartextInputVerifier.json',
      address: deployed.fhevmAddresses.inputVerifierAddress as Address,
    },
    { label: 'HCULimit', abiFile: 'HCULimit.json', address: deployed.fhevmAddresses.hcuLimitAddress as Address },
    {
      label: 'CleartextArithmetic',
      abiFile: 'CleartextArithmetic.json',
      address: deployed.cleartextAddresses.cleartextArithmeticAddress as Address,
    },
    {
      label: 'CleartextDB',
      abiFile: 'CleartextDB.json',
      address: deployed.cleartextAddresses.cleartextDbAddress as Address,
    },
    { label: 'PauserSet', abiFile: 'PauserSet.json', address: deployed.pauserSetAddress as Address },
    { label: 'ACLOwner', abiFile: 'ACLOwner.json', address: deployed.aclOwnerAddress as Address },
  ];
}

/**
 * The only readings allowed to differ across the upgrade, and why. Anything not here must be identical.
 *
 * `getVersion` changes on exactly the contracts `updateV12ToV13` re-points — which is asserted positively
 * elsewhere in this test, so here it is only excused. `InputVerifier` is deliberately absent: its bytecode
 * is unchanged between generations, so its version must NOT move.
 */
const MAY_CHANGE = new Set([
  'ACL.getVersion',
  'FHEVMExecutor.getVersion',
  'KMSVerifier.getVersion',
  'HCULimit.getVersion',
  'CleartextArithmetic.getVersion',
  // Returns `block.number` by construction, so it differs between any two blocks.
  'HCULimit.getBlockMeter',
]);

/** Reads every zero-argument getter on every contract, as comparable strings. */
async function surveyStack(
  publicClient: ReturnType<typeof createPublicClient>,
  targets: ReturnType<typeof surveyTargets>,
): Promise<Map<string, string>> {
  const readings = new Map<string, string>();
  for (const target of targets) {
    const abi = JSON.parse(readFileSync(join(V12_ABI_DIR, target.abiFile), 'utf8')) as AbiEntry[];
    const getters = abi.filter(
      (entry) =>
        entry.type === 'function' &&
        (entry.stateMutability === 'view' || entry.stateMutability === 'pure') &&
        (entry.inputs ?? []).length === 0 &&
        entry.name !== undefined,
    );
    for (const getter of getters) {
      const key = `${target.label}.${getter.name ?? ''}`;
      // A revert is recorded as a value rather than skipped. Some getters revert by design — OpenZeppelin
      // gates `proxiableUUID` with `notDelegated`, so it always reverts when called through the proxy —
      // and pretending they do not exist would mean an exclusion list. Recording the revert instead makes
      // "reverted before, reverts now" a survival, and turns "worked before, reverts now" into the
      // failure it is, which is the more interesting direction.
      let reading: string;
      try {
        const value = await publicClient.readContract({
          address: target.address,
          abi: abi as never,
          functionName: getter.name ?? '',
        });
        // bigints and tuples both have to survive a comparison, so everything is stringified alike.
        reading = JSON.stringify(value, (_k, v: unknown) => (typeof v === 'bigint' ? `${v.toString()}n` : v));
      } catch {
        reading = '<reverted>';
      }
      readings.set(key, reading);
    }
  }
  return readings;
}

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
  // A second FHE operation, so the DB is filled by more than one code path before the upgrade.
  //
  // `fheRand` rather than a binary op like `fheAdd`, and that is forced rather than chosen: the executor
  // gates binary operands on `ACL.isAllowed`, and `trivialEncrypt` grants only `allowTransient` — which
  // is transient storage, cleared at the end of its transaction. `ACL.allow` could make it persist but
  // itself requires `isAllowed(handle, msg.sender)`, so it too has to be in that same transaction. An EOA
  // sending one call per transaction therefore cannot feed a handle from one call into the next; doing so
  // needs a dApp-style helper contract that does both in one frame. `fheRand` takes no handle operand, so
  // it needs no allowance, while still going through _generateRand -> recordRand -> CleartextDB.set.
  {
    type: 'function',
    name: 'fheRand',
    stateMutability: 'nonpayable',
    inputs: [{ name: 'randType', type: 'uint8' }],
    outputs: [{ name: 'result', type: 'bytes32' }],
  },
  {
    type: 'event',
    name: 'FheRand',
    inputs: [
      { name: 'caller', type: 'address', indexed: true },
      { name: 'randType', type: 'uint8', indexed: false },
      { name: 'seed', type: 'bytes16', indexed: false },
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

/**
 * The events that would betray a change the upgrade must never make.
 *
 * Asserting on EVENT ABSENCE rather than on before/after values, because the two prove different things.
 * `PauserSet` exposes only `isPauser(address)` and no enumeration, so comparing values can only show that
 * the accounts you thought to name are unchanged — it cannot show that nobody else was added. No
 * Add/Remove/Swap event across the whole upgrade proves the membership did not move at all, whoever is
 * in it. The same argument applies to ownership: no transfer event means it never even started.
 */
const OWNERSHIP_EVENTS_ABI = [
  {
    type: 'event',
    name: 'OwnershipTransferStarted',
    inputs: [
      { name: 'previousOwner', type: 'address', indexed: true },
      { name: 'newOwner', type: 'address', indexed: true },
    ],
  },
  {
    type: 'event',
    name: 'OwnershipTransferred',
    inputs: [
      { name: 'previousOwner', type: 'address', indexed: true },
      { name: 'newOwner', type: 'address', indexed: true },
    ],
  },
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

/** Separate from PAUSER_SET_ABI: viem's `getLogs({ events })` takes event items only, not a whole ABI. */
const PAUSER_EVENTS_ABI = [
  { type: 'event', name: 'AddPauser', inputs: [{ name: 'account', type: 'address', indexed: true }] },
  { type: 'event', name: 'RemovePauser', inputs: [{ name: 'account', type: 'address', indexed: true }] },
  {
    type: 'event',
    name: 'SwapPauser',
    inputs: [
      { name: 'oldAccount', type: 'address', indexed: true },
      { name: 'newAccount', type: 'address', indexed: true },
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
    hcuLimit: expectedHcuLimit(),
  };
}

test('e2e: deploy a v12 cleartext stack, then upgrade it to v13 — cleartext survives the migration', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: DEPLOYER_ADDRESS_INDEX });
  const deployerAddress = privateKeyToAddress({ privateKey: deployerKey });
  // No separate KMS signer: this stack is deployed with the deployer AS its KMS signer, and the migration
  // config below has to describe the stack it upgrades. A distinct signer here is what made the survey
  // fail — the migration would have replaced the signer set instead of carrying it over.

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

    // fheRand(randType), returning the result handle after mining.
    const fheRand = async (): Promise<Hex> => {
      const hash = await wallet.writeContract({
        address: executor,
        abi: EXECUTOR_ABI,
        functionName: 'fheRand',
        args: [FHE_TYPE_UINT64],
      });
      const receipt = await publicClient.waitForTransactionReceipt({ hash });
      const events = parseEventLogs({ abi: EXECUTOR_ABI, eventName: 'FheRand', logs: receipt.logs });
      const event = events[0];
      if (event === undefined) {
        throw new Error('FheRand event not found');
      }
      return event.args.result;
    };

    const dbGet = async (handle: Hex): Promise<bigint> =>
      (await publicClient.readContract({
        address: cleartextDb,
        abi: CLEARTEXT_DB_ABI,
        functionName: 'get',
        args: [handle],
      })) as bigint;

    // --- 2. Pre-upgrade round-trip: fill the DB under the v12 executor. ---
    //
    // Two different write paths through the v12 arithmetic contract, so the DB holds entries produced by
    // more than one operator before anything is upgraded. Both must still resolve afterwards, which is
    // what says the re-pointed v13 implementations kept addressing the SAME CleartextDB rather than the
    // placeholder address their bytecode ships with.
    const handleBefore = await trivialEncrypt(42n);
    expect(await dbGet(handleBefore)).toBe(42n);

    // Whatever fheRand produced, the DB has to agree with it after the upgrade — the value is unknown
    // here, so it is read now and compared to itself later.
    const randHandleBefore = await fheRand();
    const randValueBefore = await dbGet(randHandleBefore);

    // Snapshot every readable property of the live v12 stack, so the upgrade can be held to preserving
    // all of it rather than only the handful this test thought to name.
    const surveyed = surveyTargets(v12 as never);
    const before = await surveyStack(publicClient, surveyed);
    // Guard against a vacuous survey: if the ABIs could not be read this would silently compare nothing.
    expect(before.size).toBeGreaterThan(30);

    // The block the upgrade starts after. Everything it sends lands above this, so the log scan below
    // covers the WHOLE upgrade — the empty impl, both proxies, all seven implementations and the atomic
    // ACLOwner.upgrade — not merely the last transaction.
    const blockBeforeUpgrade = await publicClient.getBlockNumber();
    const pauserSet = v12.pauserSetAddress as Address;
    const aclOwner = v12.aclOwnerAddress as Address;
    const isPauser = (account: string): Promise<boolean> =>
      publicClient.readContract({
        address: pauserSet,
        abi: PAUSER_SET_ABI,
        functionName: 'isPauser',
        args: [account as Address],
      }) as Promise<boolean>;

    // The standing ACLOwner is a registered pauser after setupACLOwner; that is the state to preserve.
    expect(await isPauser(aclOwner)).toBe(true);

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
            // The signer the live v12 stack was actually deployed with (see v12BootstrapConfig), NOT an
            // arbitrary one. `existingKmsNodes` means "what this stack already has": v13 reads its KMS
            // signer set from ProtocolConfig, so a value that disagrees with the running stack silently
            // REPLACES the signer set during what is supposed to be a migration. The survey below is what
            // enforces that — it failed on exactly this when the two disagreed.
            signerAddress: deployerAddress,
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

    // --- 4b. Everything readable survives. Every zero-argument getter the v12 stack exposed returns
    //         exactly what it did before, except the versions the upgrade is supposed to move. ---
    const after = await surveyStack(publicClient, surveyed);

    const changed: string[] = [];
    const vanished: string[] = [];
    for (const [key, valueBefore] of before) {
      const valueAfter = after.get(key);
      if (valueAfter === undefined) {
        vanished.push(key);
      } else if (valueAfter !== valueBefore && !MAY_CHANGE.has(key)) {
        changed.push(`${key}: ${valueBefore} -> ${valueAfter}`);
      }
    }
    expect(vanished, `getters the v12 stack had and the upgraded stack does not:\n  ${vanished.join('\n  ')}`).toEqual(
      [],
    );
    expect(changed, `readable state changed across the upgrade:\n  ${changed.join('\n  ')}`).toEqual([]);

    // The exemptions are not a licence: each one must actually have changed, or it does not belong in
    // MAY_CHANGE. This is what stops the set from quietly growing into a way of ignoring regressions.
    const unusedExemptions = [...MAY_CHANGE].filter((key) => before.has(key) && before.get(key) === after.get(key));
    expect(
      unusedExemptions,
      `MAY_CHANGE lists readings that did not change — remove them:\n  ${unusedExemptions.join('\n  ')}`,
    ).toEqual([]);

    // --- 4c. Ownership never moved, and the pauser set never changed. ---
    //
    // The upgrade is supposed to run entirely THROUGH the standing ACLOwner without touching who owns
    // what. A coordinator that re-ran the ownership dance, or a migration that re-registered pausers,
    // would still leave a working stack — and would have handed root to a different account, or changed
    // who can pause it, with nothing else noticing.
    const upgradeBlocks = { fromBlock: blockBeforeUpgrade + 1n, toBlock: 'latest' } as const;

    for (const [label, address] of [
      ['ACL', v12.fhevmAddresses.aclAddress as Address],
      ['ACLOwner', aclOwner],
    ] as const) {
      const transfers = await publicClient.getLogs({ address, events: OWNERSHIP_EVENTS_ABI, ...upgradeBlocks });
      expect(
        transfers.map((log) => log.eventName),
        `${label} emitted an ownership event during the upgrade — ownership must never change`,
      ).toEqual([]);
    }

    const pauserChanges = await publicClient.getLogs({
      address: pauserSet,
      events: PAUSER_EVENTS_ABI,
      ...upgradeBlocks,
    });
    expect(
      pauserChanges.map((log) => log.eventName),
      'PauserSet membership changed during the upgrade — the pausers must stay the same',
    ).toEqual([]);

    // Positive counterpart to the event scan: the PauserSet is the same contract, and the ACLOwner is
    // still a pauser in it. Event absence proves nothing was added or removed; this proves the set the
    // stack points at is still the one that was checked.
    expect(v12.pauserSetAddress).toBe(pauserSet);
    expect(await isPauser(aclOwner)).toBe(true);

    // --- 5. Cleartext still works after the upgrade (new v13 executor impl → live CleartextArithmetic
    //        → CleartextDB), and the pre-upgrade value persisted through the migration. ---
    // The DB proxy is unchanged by the upgrade — asserted directly, so a re-pointed CleartextDB reports
    // itself as a wrong ADDRESS here rather than as a puzzling wrong value below.
    expect(v12.cleartextAddresses.cleartextDbAddress).toBe(cleartextDb);

    // Every pre-upgrade handle still resolves to the value it had, through the REPLACED arithmetic
    // implementation. This is the assertion the whole test exists for: a migration that left the new
    // executor or arithmetic pointing at their placeholder addresses passes every version check above and
    // fails here.
    expect(await dbGet(handleBefore)).toBe(42n);
    expect(await dbGet(randHandleBefore)).toBe(randValueBefore);

    // And the new v13 implementations can still write, by both paths.
    const handleAfter = await trivialEncrypt(99n);
    expect(await dbGet(handleAfter)).toBe(99n);

    const randHandleAfter = await fheRand();
    const randValueAfter = await dbGet(randHandleAfter);

    expect(randValueAfter).not.toBe(randValueBefore);
    expect(randHandleAfter).not.toBe(randHandleBefore);

    // The drawn value is unknowable, and 0 is a legitimate draw — so its PRESENCE is the assertion. The
    // read is what proves the v13 arithmetic wrote to the live DB rather than to a placeholder address.
    expect(typeof (await dbGet(randHandleAfter))).toBe('bigint');
  } finally {
    await stopAnvil(anvil.process);
  }
}, 180_000);

test('e2e: updateV12ToV13 with no migration config — defaults resolved from the live v12 KMSVerifier', async () => {
  const deployerKey = privateKeyFromMnemonic({ mnemonic: MNEMONIC, addressIndex: DEPLOYER_ADDRESS_INDEX });
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
      // No config → deployV12 uses DEFAULT_BOOTSTRAP_CONFIG, whose KMS signer set is the package
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
