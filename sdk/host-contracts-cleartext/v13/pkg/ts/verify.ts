import { abi as aclAbi } from './artifacts/ACL.js';
import { abi as aclOwnerAbi } from './artifacts/ACLOwner.js';
import { abi as cleartextArithmeticAbi } from './artifacts/CleartextArithmetic.js';
import { abi as cleartextDbAbi } from './artifacts/CleartextDB.js';
import { abi as fhevmExecutorAbi } from './artifacts/CleartextFHEVMExecutor.js';
import { abi as inputVerifierAbi } from './artifacts/CleartextInputVerifier.js';
import { abi as kmsVerifierAbi } from './artifacts/CleartextKMSVerifier.js';
import { abi as hcuLimitAbi } from './artifacts/HCULimit.js';
import { abi as kmsGenerationAbi } from './artifacts/KMSGeneration.js';
import { abi as pauserSetAbi } from './artifacts/PauserSet.js';
import { abi as protocolConfigAbi } from './artifacts/ProtocolConfig.js';
import type {
  AbstractEthereumProvider,
  Deployed,
  KmsThresholds,
  StackSnapshot,
  VerifyCheck,
  VerifyExpectations,
  VerifyParameters,
  VerifyReport,
  AbstractEthereumHistory,
  PartialStack,
  SnapshotParameters,
} from './types/public.js';
import { CONTRACT_VERSIONS } from './versions.js';

////////////////////////////////////////////////////////////////////////////////
// The stack, as a table
////////////////////////////////////////////////////////////////////////////////

/**
 * The ERC-1967 implementation slot: `keccak256("eip1967.proxy.implementation") - 1`.
 *
 * Read directly rather than inferred, because a proxy's CODE is identical before and after it is pointed
 * at a real implementation. Presence of code proves a proxy exists; only this slot says whether it has
 * been materialized.
 */
const IMPLEMENTATION_SLOT = '0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc';

const ZERO_ADDRESS = `0x${'0'.repeat(40)}`;

/** Which `CONTRACT_VERSIONS` key each label's `getVersion()` must return, where it has one. */
type Target = {
  readonly label: string;
  readonly address: string;
  readonly abi: readonly unknown[];
  /** Absent for the two contracts that expose no `getVersion()`: `CleartextDB` and `ACLOwner`. */
  readonly versionKey?: keyof typeof CONTRACT_VERSIONS;
  /** False for `PauserSet` and `ACLOwner`, which are plain contracts rather than ERC-1967 proxies. */
  readonly isProxy: boolean;
};

/**
 * Every contract a deployed stack consists of, paired with the ABI that describes it.
 *
 * One table, consumed by every check below, so "which contracts exist" is stated once. Keyed by role
 * rather than by a bare address list, so a caller cannot silently pass the wrong contract for a slot.
 * Roles with no address are dropped — see `PartialStack` for why that is a legitimate state.
 */
function targetsOf(deployed: PartialStack): readonly Target[] {
  const f = deployed.fhevmAddresses;
  const c = deployed.cleartextAddresses ?? {};
  // `undefined` means "this contract does not exist yet", which is a legitimate state for a snapshot taken
  // before an upgrade. Dropped rather than surveyed, so an absent proxy produces no readings at all —
  // which is what makes the upgrade filling it in read as NEW rather than as changed.
  const all: ReadonlyArray<Target | null> = [
    { label: 'ACL', address: f.aclAddress, abi: aclAbi, versionKey: 'acl', isProxy: true },
    {
      label: 'FHEVMExecutor',
      address: f.fhevmExecutorAddress,
      abi: fhevmExecutorAbi,
      versionKey: 'fhevmExecutor',
      isProxy: true,
    },
    {
      label: 'KMSVerifier',
      address: f.kmsVerifierAddress,
      abi: kmsVerifierAbi,
      versionKey: 'kmsVerifier',
      isProxy: true,
    },
    {
      label: 'InputVerifier',
      address: f.inputVerifierAddress,
      abi: inputVerifierAbi,
      versionKey: 'inputVerifier',
      isProxy: true,
    },
    { label: 'HCULimit', address: f.hcuLimitAddress, abi: hcuLimitAbi, versionKey: 'hcuLimit', isProxy: true },
    {
      label: 'ProtocolConfig',
      address: f.protocolConfigAddress,
      abi: protocolConfigAbi,
      versionKey: 'protocolConfig',
      isProxy: true,
    },
    {
      label: 'KMSGeneration',
      address: f.kmsGenerationAddress,
      abi: kmsGenerationAbi,
      versionKey: 'kmsGeneration',
      isProxy: true,
    },
    {
      label: 'CleartextArithmetic',
      address: c.cleartextArithmeticAddress,
      abi: cleartextArithmeticAbi,
      versionKey: 'cleartextArithmetic',
      isProxy: true,
    },
    { label: 'CleartextDB', address: c.cleartextDbAddress, abi: cleartextDbAbi, isProxy: true },
    {
      label: 'PauserSet',
      address: deployed.pauserSetAddress,
      abi: pauserSetAbi,
      versionKey: 'pauserSet',
      isProxy: false,
    },
    { label: 'ACLOwner', address: deployed.aclOwnerAddress, abi: aclOwnerAbi, isProxy: false },
  ].map((t) => (t.address === undefined ? null : ({ ...t, address: t.address } as Target)));
  return all.filter((t): t is Target => t !== null);
}

////////////////////////////////////////////////////////////////////////////////
// Reading
////////////////////////////////////////////////////////////////////////////////

function sameAddress(a: string, b: string): boolean {
  return a.toLowerCase() === b.toLowerCase();
}

/**
 * bigints and tuples both have to survive comparison, so every reading is stringified alike.
 *
 * `undefined` is spelled out rather than left to `JSON.stringify`, which returns the VALUE `undefined` for
 * it — a reading that would then compare equal to a missing one.
 */
function stringifyReading(value: unknown): string {
  if (value === undefined) return 'undefined';
  return JSON.stringify(value, (_key, v: unknown) => (typeof v === 'bigint' ? `${v.toString()}n` : v));
}

/**
 * Every zero-argument `view`/`pure` getter on a contract, by name.
 *
 * The whole point of enumerating the ABI rather than listing getters is coverage of what nobody thought
 * to name. A hand-written list can only check the values its author remembered.
 */
function zeroArgGetters(abi: readonly unknown[]): readonly string[] {
  const names: string[] = [];
  for (const entry of abi) {
    const e = entry as {
      type?: string;
      name?: string;
      stateMutability?: string;
      inputs?: readonly unknown[];
    };
    if (e.type !== 'function' || e.name === undefined) continue;
    if (e.stateMutability !== 'view' && e.stateMutability !== 'pure') continue;
    if ((e.inputs ?? []).length !== 0) continue;
    names.push(e.name);
  }
  return names;
}

/**
 * Reads every zero-argument getter on every contract in the stack.
 *
 * A revert is recorded as the value `<reverted>` rather than skipped. Some getters revert by design —
 * OpenZeppelin gates `proxiableUUID` with `notDelegated`, so it always reverts through a proxy — and
 * pretending they do not exist would require an exclusion list. Recording the revert instead makes
 * "reverted before, reverts now" a survival, and turns "worked before, reverts now" into the failure it
 * is, which is the more interesting direction.
 */
async function readAll(
  ethProvider: AbstractEthereumProvider,
  targets: readonly Target[],
): Promise<Record<string, string>> {
  const readings: Record<string, string> = {};
  for (const target of targets) {
    for (const functionName of zeroArgGetters(target.abi)) {
      let reading: string;
      try {
        reading = stringifyReading(
          await ethProvider.readContract({ address: target.address, abi: target.abi, functionName }),
        );
      } catch {
        reading = '<reverted>';
      }
      readings[`${target.label}.${functionName}`] = reading;
    }
  }
  return readings;
}

/**
 * Snapshots everything readable about a live stack, for comparison after an upgrade.
 *
 * Must be called BEFORE the upgrade: by the time `verify` runs the previous values are gone from the
 * chain, so this snapshot is the only witness that they were ever different.
 *
 * `blockNumber` is captured alongside the readings, and is what bounds the event scans `verify` runs over
 * the upgrade's blocks. Without it those scans would have to guess a range.
 *
 * ## A caveat when snapshotting the PREVIOUS generation
 *
 * The getters are enumerated from THIS package's ABIs. Pointed at a v12 stack, a getter that v13 added
 * simply reverts and is recorded as `<reverted>` — harmless, and it stays `<reverted>` or starts working,
 * either of which reads correctly. What this cannot see is a getter v13 REMOVED: it is absent from the
 * enumeration, so its disappearance is invisible. Supply `abis` with the previous generation's ABIs to
 * close that gap when the previous package is available.
 */
export async function snapshotStack(parameters: SnapshotParameters): Promise<StackSnapshot> {
  const targets = withAbiOverrides(targetsOf(parameters.deployed), parameters.abis);
  const blockNumber = parameters.history === undefined ? null : await parameters.history.getBlockNumber();
  return { blockNumber, readings: await readAll(parameters.ethProvider, targets) };
}

function withAbiOverrides(
  targets: readonly Target[],
  abis: Readonly<Record<string, readonly unknown[]>> | undefined,
): readonly Target[] {
  if (abis === undefined) return targets;
  return targets.map((t) => {
    const override = abis[t.label];
    return override === undefined ? t : { ...t, abi: override };
  });
}

////////////////////////////////////////////////////////////////////////////////
// The report
////////////////////////////////////////////////////////////////////////////////

/**
 * Collects checks instead of throwing on the first failure.
 *
 * One run should report everything wrong with a stack. An operator fixing a deployment wants the whole
 * list; throwing on the first problem turns that into one round trip per problem.
 */
class Report {
  private readonly checks: VerifyCheck[] = [];

  pass(name: string): void {
    this.checks.push({ name, status: 'pass' });
  }

  fail(name: string, detail: string): void {
    this.checks.push({ name, status: 'fail', detail });
  }

  /** A check that could not run — a missing adapter capability, never a value that looked wrong. */
  skip(name: string, detail: string): void {
    this.checks.push({ name, status: 'skip', detail });
  }

  expect(name: string, ok: boolean, detail: () => string): void {
    if (ok) this.pass(name);
    else this.fail(name, detail());
  }

  expectAddress(name: string, got: string, want: string): void {
    this.expect(name, sameAddress(got, want), () => `got ${got}, want ${want}`);
  }

  expectEqual(name: string, got: unknown, want: unknown): void {
    const g = stringifyReading(got);
    const w = stringifyReading(want);
    this.expect(name, g === w, () => `got ${g}, want ${w}`);
  }

  finish(): VerifyReport {
    const checks = [...this.checks];
    const failures = checks.filter((c) => c.status === 'fail');
    const skipped = checks.filter((c) => c.status === 'skip');
    return { ok: failures.length === 0, checks, failures, skipped };
  }
}

////////////////////////////////////////////////////////////////////////////////
// Checks
////////////////////////////////////////////////////////////////////////////////

async function checkCode(
  report: Report,
  ethProvider: AbstractEthereumProvider,
  targets: readonly Target[],
): Promise<void> {
  for (const target of targets) {
    // A throw and an empty result mean the same thing here — nothing is deployed there — so both collapse
    // into one answer rather than one being a failure and the other an error.
    let code: string;
    try {
      code = await ethProvider.getCodeAt({ address: target.address });
    } catch {
      code = '0x';
    }
    report.expect(
      `code.${target.label}`,
      code.length > 2,
      () => `no code at ${target.address} — nothing is deployed there`,
    );
  }
}

async function checkMaterialized(
  report: Report,
  history: AbstractEthereumHistory | undefined,
  targets: readonly Target[],
): Promise<void> {
  const proxies = targets.filter((t) => t.isProxy);
  if (history === undefined) {
    for (const target of proxies) {
      report.skip(
        `materialized.${target.label}`,
        "needs `history.getStorageAt` — a proxy's code is identical before and after it is pointed at a " +
          'real implementation, so this cannot be inferred from code alone',
      );
    }
    return;
  }
  for (const target of proxies) {
    const raw = await history.getStorageAt({ address: target.address, slot: IMPLEMENTATION_SLOT });
    report.expect(
      `materialized.${target.label}`,
      BigInt(raw === '0x' ? '0x0' : raw) !== 0n,
      () => `${target.label} still points at the zero implementation — it was never materialized`,
    );
  }
}

/**
 * The version each contract reports, against the generated `CONTRACT_VERSIONS`.
 *
 * Generated, never hand-written: the constants are read out of the contract sources this package
 * vendors, so bumping a contract cannot leave a stale expectation here.
 */
async function checkVersions(
  report: Report,
  ethProvider: AbstractEthereumProvider,
  targets: readonly Target[],
): Promise<void> {
  for (const target of targets) {
    if (target.versionKey === undefined) continue;
    const want = CONTRACT_VERSIONS[target.versionKey];
    try {
      const got = (await ethProvider.readContract({
        address: target.address,
        abi: target.abi,
        functionName: 'getVersion',
      })) as string;
      report.expect(`version.${target.label}`, got === want, () => `got "${got}", want "${want}"`);
    } catch (error) {
      report.fail(`version.${target.label}`, `getVersion() reverted: ${String(error)}`);
    }
  }
}

/**
 * Every baked-in address the host contracts expose, against the addresses actually deployed.
 *
 * Distinct in kind from every other check here. The others ask the CHAIN questions — is there code at
 * this address, who owns this. These ask the BYTECODE what it was COMPILED WITH, and that is the only way
 * to catch a stack assembled from a stale build, a remapping that silently did not apply, or placeholder
 * markers that survived into the implementations.
 */
async function checkWiring(report: Report, ethProvider: AbstractEthereumProvider, deployed: Deployed): Promise<void> {
  const { fhevmAddresses: f, cleartextAddresses: c } = deployed;
  const wiring: ReadonlyArray<{
    readonly name: string;
    readonly address: string;
    readonly abi: readonly unknown[];
    readonly functionName: string;
    readonly want: string;
  }> = [
    {
      name: 'ACL.getFHEVMExecutorAddress',
      address: f.aclAddress,
      abi: aclAbi,
      functionName: 'getFHEVMExecutorAddress',
      want: f.fhevmExecutorAddress,
    },
    {
      name: 'ACL.getPauserSetAddress',
      address: f.aclAddress,
      abi: aclAbi,
      functionName: 'getPauserSetAddress',
      want: deployed.pauserSetAddress,
    },
    {
      name: 'FHEVMExecutor.getACLAddress',
      address: f.fhevmExecutorAddress,
      abi: fhevmExecutorAbi,
      functionName: 'getACLAddress',
      want: f.aclAddress,
    },
    {
      name: 'FHEVMExecutor.getHCULimitAddress',
      address: f.fhevmExecutorAddress,
      abi: fhevmExecutorAbi,
      functionName: 'getHCULimitAddress',
      want: f.hcuLimitAddress,
    },
    {
      name: 'FHEVMExecutor.getInputVerifierAddress',
      address: f.fhevmExecutorAddress,
      abi: fhevmExecutorAbi,
      functionName: 'getInputVerifierAddress',
      want: f.inputVerifierAddress,
    },
    {
      name: 'FHEVMExecutor.getCleartextArithmeticAddress',
      address: f.fhevmExecutorAddress,
      abi: fhevmExecutorAbi,
      functionName: 'getCleartextArithmeticAddress',
      want: c.cleartextArithmeticAddress,
    },
    {
      name: 'HCULimit.getFHEVMExecutorAddress',
      address: f.hcuLimitAddress,
      abi: hcuLimitAbi,
      functionName: 'getFHEVMExecutorAddress',
      want: f.fhevmExecutorAddress,
    },
    {
      name: 'CleartextArithmetic.getCleartextDBAddress',
      address: c.cleartextArithmeticAddress,
      abi: cleartextArithmeticAbi,
      functionName: 'getCleartextDBAddress',
      want: c.cleartextDbAddress,
    },
    {
      name: 'CleartextDB.getACLAddress',
      address: c.cleartextDbAddress,
      abi: cleartextDbAbi,
      functionName: 'getACLAddress',
      want: f.aclAddress,
    },
  ];

  for (const w of wiring) {
    try {
      const got = (await ethProvider.readContract({
        address: w.address,
        abi: w.abi,
        functionName: w.functionName,
      })) as string;
      report.expectAddress(`wiring.${w.name}`, got, w.want);
    } catch (error) {
      report.fail(`wiring.${w.name}`, `reverted — is the proxy materialized? ${String(error)}`);
    }
  }
}

/**
 * Ownership, and the two `pendingOwner` checks that are not tidiness.
 *
 * `ACLOwner` is the single atomic upgrade root, so whoever owns it is root over the whole stack. A
 * dangling pending owner on either contract is a latent takeover: whoever holds that key can accept at
 * any future moment. Both are completion conditions, not cosmetics.
 */
async function checkOwnership(
  report: Report,
  ethProvider: AbstractEthereumProvider,
  deployed: Deployed,
  expected: VerifyExpectations | undefined,
): Promise<void> {
  const read = async (address: string, abi: readonly unknown[], functionName: string): Promise<string> =>
    (await ethProvider.readContract({ address, abi, functionName })) as string;

  const acl = deployed.fhevmAddresses.aclAddress;
  const aclOwner = deployed.aclOwnerAddress;

  report.expectAddress('ownership.ACL.owner', await read(acl, aclAbi, 'owner'), aclOwner);
  report.expectAddress('ownership.ACL.pendingOwner', await read(acl, aclAbi, 'pendingOwner'), ZERO_ADDRESS);
  report.expectAddress('ownership.ACLOwner.acl', await read(aclOwner, aclOwnerAbi, 'acl'), acl);
  report.expectAddress(
    'ownership.ACLOwner.pendingOwner',
    await read(aclOwner, aclOwnerAbi, 'pendingOwner'),
    ZERO_ADDRESS,
  );

  const admin = expected?.admin;
  if (admin === undefined) {
    report.skip(
      'ownership.ACLOwner.owner',
      'no `expected.admin` supplied — who SHOULD own the stack is not something this can derive',
    );
    return;
  }
  // Compared against the expectation rather than against the chain: `ACLOwner.owner() == admin` is what
  // makes the admin's own acceptOwnership() a gate rather than a suggestion. Until they have sent it, the
  // deployer key is still root.
  report.expectAddress('ownership.ACLOwner.owner', await read(aclOwner, aclOwnerAbi, 'owner'), admin);
}

async function checkPausers(
  report: Report,
  ethProvider: AbstractEthereumProvider,
  deployed: Deployed,
  expected: VerifyExpectations | undefined,
): Promise<void> {
  const isPauser = async (account: string): Promise<boolean> =>
    (await ethProvider.readContract({
      address: deployed.pauserSetAddress,
      abi: pauserSetAbi,
      functionName: 'isPauser',
      args: [account],
    })) as boolean;

  report.expect(
    'pausers.ACLOwner',
    await isPauser(deployed.aclOwnerAddress),
    () => 'the ACLOwner is not a pauser — the stack cannot be paused through its own upgrade root',
  );
  for (const account of expected?.pausers ?? []) {
    report.expect(`pausers.${account}`, await isPauser(account), () => `${account} is not a pauser`);
  }
}

/**
 * The bootstrap values, each checked only if the caller says what to expect.
 *
 * Deliberately not derived from a mnemonic here. This package cannot know whether a given stack was
 * bootstrapped from our defaults, and asserting that it was would make `verify` unusable against any
 * stack seeded with real keys. The caller knows; `expected` is where they say so.
 */
async function checkBootstrap(
  report: Report,
  ethProvider: AbstractEthereumProvider,
  deployed: Deployed,
  expected: VerifyExpectations | undefined,
): Promise<void> {
  if (expected === undefined) return;
  const f = deployed.fhevmAddresses;
  const read = (address: string, abi: readonly unknown[], functionName: string): Promise<unknown> =>
    ethProvider.readContract({ address, abi, functionName });

  if (expected.coprocessorSigners !== undefined) {
    const got = (await read(f.inputVerifierAddress, inputVerifierAbi, 'getCoprocessorSigners')) as readonly string[];
    report.expect(
      'bootstrap.coprocessorSigners',
      got.length === expected.coprocessorSigners.length &&
        got.every((a, i) => sameAddress(a, expected.coprocessorSigners?.[i] ?? '')),
      () => `got [${got.join(', ')}], want [${(expected.coprocessorSigners ?? []).join(', ')}]`,
    );
  }
  if (expected.coprocessorThreshold !== undefined) {
    report.expectEqual(
      'bootstrap.coprocessorThreshold',
      await read(f.inputVerifierAddress, inputVerifierAbi, 'getThreshold'),
      expected.coprocessorThreshold,
    );
  }
  if (expected.kmsSigners !== undefined) {
    const got = (await read(f.protocolConfigAddress, protocolConfigAbi, 'getKmsSigners')) as readonly string[];
    report.expect(
      'bootstrap.kmsSigners',
      got.length === expected.kmsSigners.length && got.every((a, i) => sameAddress(a, expected.kmsSigners?.[i] ?? '')),
      () => `got [${got.join(', ')}], want [${(expected.kmsSigners ?? []).join(', ')}]`,
    );
  }
  if (expected.kmsContextId !== undefined) {
    report.expectEqual(
      'bootstrap.kmsContextId',
      await read(f.protocolConfigAddress, protocolConfigAbi, 'getCurrentKmsContextId'),
      expected.kmsContextId,
    );
  }
  const thresholds = expected.kmsThresholds;
  if (thresholds !== undefined) {
    const getters: ReadonlyArray<readonly [keyof KmsThresholds, string]> = [
      ['publicDecryption', 'getPublicDecryptionThreshold'],
      ['userDecryption', 'getUserDecryptionThreshold'],
      ['kmsGen', 'getKmsGenThreshold'],
      ['mpc', 'getMpcThreshold'],
    ];
    for (const [key, functionName] of getters) {
      report.expectEqual(
        `bootstrap.kmsThresholds.${key}`,
        await read(f.protocolConfigAddress, protocolConfigAbi, functionName),
        thresholds[key],
      );
    }
  }
}

////////////////////////////////////////////////////////////////////////////////
// Survival — upgrade mode only
////////////////////////////////////////////////////////////////////////////////

/**
 * The readings allowed to differ across an upgrade, and why. Anything else must be identical.
 *
 * `getVersion` moves on exactly the contracts an upgrade re-points — which `checkVersions` asserts
 * positively, so here it is only excused. `InputVerifier` is deliberately absent: its bytecode is
 * unchanged between generations, so a moved version there means something re-pointed a proxy nobody
 * intended to touch.
 */
export const DEFAULT_MAY_CHANGE: readonly string[] = [
  'ACL.getVersion',
  'FHEVMExecutor.getVersion',
  'KMSVerifier.getVersion',
  'HCULimit.getVersion',
  'CleartextArithmetic.getVersion',
  // Returns `block.number` by construction, so it differs between any two blocks.
  'HCULimit.getBlockMeter',
];

/** Events that must not appear during an upgrade. Ownership must never move; pausers must not change. */
const OWNERSHIP_EVENTS: readonly string[] = ['OwnershipTransferStarted', 'OwnershipTransferred'];
const PAUSER_EVENTS: readonly string[] = ['AddPauser', 'RemovePauser', 'SwapPauser'];

function checkSurvival(
  report: Report,
  before: StackSnapshot,
  after: Record<string, string>,
  mayChange: readonly string[],
): void {
  const exempt = new Set(mayChange);

  // A getter the previous stack had and this one does not. Reported separately from a changed value
  // because the cause is different: a removed getter is an ABI regression, not a state change.
  const vanished = Object.keys(before.readings).filter((key) => !(key in after));
  report.expect('survival.noGetterVanished', vanished.length === 0, () => vanished.join(', '));

  const changed: string[] = [];
  for (const [key, was] of Object.entries(before.readings)) {
    if (exempt.has(key) || !(key in after)) continue;
    const now = after[key];
    if (now !== was) changed.push(`${key}: ${was} -> ${String(now)}`);
  }
  report.expect('survival.everythingElseUnchanged', changed.length === 0, () => changed.join('\n'));

  // The exemption list must be USED. Otherwise it quietly becomes a way to ignore regressions: an entry
  // that never changes is an entry nobody would notice going stale.
  const unused = mayChange.filter((key) => key in before.readings && before.readings[key] === after[key]);
  report.expect(
    'survival.exemptionsWereUsed',
    unused.length === 0,
    () => `these are exempt but did not change — remove them: ${unused.join(', ')}`,
  );
}

/**
 * That no ownership or pauser event was emitted across the upgrade's blocks.
 *
 * Not redundant with comparing the values, and the reason is worth stating: `PauserSet` exposes
 * `isPauser(address)` and no enumeration, so comparing values can only show that accounts SOMEONE THOUGHT
 * TO NAME are unchanged — it cannot show that nobody else was added. Log absence proves the membership
 * never moved, whoever is in it. The same argument applies to ownership: no event means no transfer was
 * even initiated.
 */
async function checkNoAuthorityEvents(
  report: Report,
  history: AbstractEthereumHistory | undefined,
  deployed: Deployed,
  before: StackSnapshot,
): Promise<void> {
  const names = ['events.ownership', 'events.pausers'];
  if (history === undefined || before.blockNumber === null) {
    const why =
      history === undefined
        ? 'needs `history.getLogs`'
        : 'the snapshot carries no block number — pass `history` to `snapshotStack` too';
    for (const name of names) {
      report.skip(
        name,
        `${why}. Without it, "nobody was ADDED to the pauser set" cannot be shown at all: PauserSet has ` +
          'no enumeration, so value comparison only covers accounts someone thought to name',
      );
    }
    return;
  }

  const fromBlock = before.blockNumber + 1n;
  const scans: ReadonlyArray<readonly [string, string, readonly unknown[], readonly string[]]> = [
    ['events.ownership', deployed.fhevmAddresses.aclAddress, aclAbi, OWNERSHIP_EVENTS],
    ['events.ownership', deployed.aclOwnerAddress, aclOwnerAbi, OWNERSHIP_EVENTS],
    ['events.pausers', deployed.pauserSetAddress, pauserSetAbi, PAUSER_EVENTS],
  ];
  for (const [name, address, abi, eventNames] of scans) {
    const logs = await history.getLogs({ address, abi, eventNames, fromBlock, toBlock: 'latest' });
    report.expect(
      name,
      logs.length === 0,
      () => `${address} emitted ${logs.map((l) => l.eventName).join(', ')} during the upgrade`,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////
// Entry point
////////////////////////////////////////////////////////////////////////////////

/**
 * Checks the full integrity of a deployed or upgraded stack.
 *
 * Returns a report rather than throwing, so one run reports everything wrong. `report.ok` is the verdict;
 * `report.failures` says what is broken and `report.skipped` what could not be checked at all — read both,
 * because a skip is not a pass.
 *
 * ## `mode: 'deploy'`
 *
 * Everything that must be true of a correct stack at rest: code at every address, every proxy pointing at
 * a real implementation, every version matching `CONTRACT_VERSIONS`, every baked-in address agreeing with
 * what was actually deployed, ownership resting in the `ACLOwner` with nothing pending, and the `ACLOwner`
 * being a pauser. Anything the package cannot derive — who the admin should be, which signers were seeded —
 * is checked only if `expected` says what to expect, and reported as a skip otherwise rather than assumed.
 *
 * ## `mode: 'upgrade'`
 *
 * All of the above, plus what a version check cannot see: that nothing ELSE changed. Requires a
 * `snapshotStack()` taken before the upgrade, and compares every readable value against it — every
 * zero-argument getter on every contract, not a list someone remembered — allowing only `mayChange`.
 * It also requires that each `mayChange` entry actually changed, so the exemption list cannot decay into
 * a way of ignoring regressions.
 *
 * ## Adapter capabilities
 *
 * Two checks need more than `AbstractEthereumProvider` offers, so they take an optional `history`
 * adapter: reading the ERC-1967 slot (a proxy's code is identical before and after materialization) and
 * scanning for ownership/pauser events. Both are reported as skips when it is absent — with the reason,
 * because the pauser event scan is the ONLY check that can show nobody was added to a set that exposes no
 * enumeration.
 *
 * @example
 * ```ts
 * const before = await snapshotStack({ ethProvider, history, deployed: v12Stack });
 * const upgraded = await updateV12ToV13({ ... });
 * const report = await verify({ ethProvider, history, deployed: upgraded, mode: 'upgrade', before });
 * if (!report.ok) throw new Error(report.failures.map((f) => `${f.name}: ${f.detail ?? ''}`).join('\n'));
 * ```
 */
export async function verify(parameters: VerifyParameters): Promise<VerifyReport> {
  const report = new Report();
  const { ethProvider, history, deployed, expected } = parameters;
  const targets = targetsOf(deployed);

  await checkCode(report, ethProvider, targets);
  await checkMaterialized(report, history, targets);
  await checkVersions(report, ethProvider, targets);
  await checkWiring(report, ethProvider, deployed);
  await checkOwnership(report, ethProvider, deployed, expected);
  await checkPausers(report, ethProvider, deployed, expected);
  await checkBootstrap(report, ethProvider, deployed, expected);

  if (parameters.mode === 'upgrade') {
    const after = await readAll(ethProvider, withAbiOverrides(targets, parameters.abis));
    checkSurvival(report, parameters.before, after, parameters.mayChange ?? DEFAULT_MAY_CHANGE);
    await checkNoAuthorityEvents(report, history, deployed, parameters.before);
  }

  return report.finish();
}
