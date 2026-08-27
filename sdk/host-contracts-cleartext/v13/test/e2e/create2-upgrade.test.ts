// The CREATE2 path, end to end, across a generation boundary: a fresh anvil, a v12 stack deployed by
// v12's own `create2-deploy` coordinator, then v13's coordinator upgrading it in place.
//
// Run: npm run test:create2-e2e
//
// ## Why this exists separately from test/ts/upgrade-e2e.test.ts
//
// That test drives the same migration through the TypeScript library — `deploy()` then `updateV12ToV13()`
// — from inside one Node process. This drives it through the OPERATOR-FACING path: two CLI coordinators,
// each spawning `forge script`, each broadcasting real transactions and sealing a manifest. The two share
// no code, which is the point. The library path can be correct while the CREATE2 path is broken, and the
// CREATE2 path is the one an operator actually runs against a testnet.
//
// It also exercises the thing neither unit tests nor the library e2e can: that the addresses v13's
// `compute` predicts for the two NEW proxies do not collide with the eight live ones it was handed, and
// that a `materialize` built from a sealed manifest lands on a stack that was deployed by a different
// package, at a different version, in a different process.
//
// ## Deliberately NOT in the fast lane
//
// `test:templates:run` globs `test/*.test.ts`, one level deep, so living in `test/e2e/` keeps this out of
// it. Two forge builds and a full deploy do not belong in a lane meant to run in seconds.
//
// ## Skips rather than fails when a prerequisite is missing
//
// Same policy as `internal/runUpgradeE2e.ts`, for the same reason: `anvil`/`forge` may be absent, the
// sibling v12 package may not be checked out, and neither is a defect in this package. Every skip names
// what is missing and how to get it. The one thing a skip must never do is look like a pass — each is
// reported by the test runner as a skip, with its reason.

import assert from 'node:assert/strict';
import { spawn, spawnSync, type ChildProcess } from 'node:child_process';
import { existsSync, readFileSync, rmSync } from 'node:fs';
import { join } from 'node:path';
import test from 'node:test';
import { Contract, JsonRpcProvider } from 'ethers';
import { PACKAGE_ROOT_ABS_PATH, PREVIOUS_GENERATION_DIR_ABS_PATH } from '../../internal/constants.ts';

////////////////////////////////////////////////////////////////////////////////
// Configuration
////////////////////////////////////////////////////////////////////////////////

/**
 * A port well away from 8545.
 *
 * Not a detail: a developer very often has a stack on the default port, and a test that quietly deployed
 * onto it would both corrupt that stack and pass. The port is also checked for occupancy before anvil is
 * started, so a collision is a named skip rather than a confusing deploy failure.
 */
const PORT = 8557;
const RPC_URL = `http://127.0.0.1:${PORT}`;

/**
 * A dedicated out-dir per generation, so a run never touches the `.out-anvil` a developer's manual
 * rehearsal uses. `.out-*` is gitignored in both generations.
 *
 * Passed to `--out-dir` as a BARE name, because the coordinator resolves that flag against `FS_ROOT` —
 * the `create2-deploy/` directory — not against the package root. It has to: `foundry.toml`'s
 * `fs_permissions` grants forge write access to exactly `./create2-deploy`, so a manifest outside it
 * cannot be sealed at all. Passing `create2-deploy/.out-…` here nests it one level too deep, which is
 * silent — the coordinator still exits 0, and only the missing manifest gives it away.
 */
const OUT_DIR_ARG = '.out-create2-e2e';

/** Where that lands on disk, for reading the manifest and for cleanup. */
function outDirAbs(packageRoot: string): string {
  return join(packageRoot, 'create2-deploy', OUT_DIR_ARG);
}

/** Distinct from the GUIDE's `anvil-rehearsal`, so a developer's manual rehearsal state is never reused. */
const DEPLOYMENT_ID = 'create2-e2e';

/** The four Solidity scripts v13's upgrade coordinator needs. Absent today. */
const UPGRADE_SCRIPTS = [
  'FhevmUpgradeBase.s.sol',
  'FhevmComputeUpgradeAddresses.s.sol',
  'FhevmUpgradeCreates.s.sol',
  'FhevmMaterializeUpgrade.s.sol',
] as const;

////////////////////////////////////////////////////////////////////////////////
// Prerequisites
////////////////////////////////////////////////////////////////////////////////

function haveBinary(name: string): boolean {
  return spawnSync(name, ['--version'], { stdio: 'ignore' }).status === 0;
}

/** Why this whole test cannot run, or undefined. `undefined` rather than `null` is what `test`'s `skip`
 *  option accepts, and passing the reason straight through is what makes the runner print it. */
function blockedReason(): string | undefined {
  for (const bin of ['anvil', 'forge'] as const) {
    if (!haveBinary(bin)) return `${bin} not found — install foundry (https://getfoundry.sh)`;
  }
  if (!existsSync(join(PREVIOUS_GENERATION_DIR_ABS_PATH, 'create2-deploy', 'deploy-testnet.ts'))) {
    return `the sibling v12 package has no create2-deploy coordinator at ${PREVIOUS_GENERATION_DIR_ABS_PATH}`;
  }
  // The v12 coordinator shells out to forge, which resolves its Solidity deps from v12's node_modules.
  if (!existsSync(join(PREVIOUS_GENERATION_DIR_ABS_PATH, 'node_modules'))) {
    return 'v12 dependencies are not installed — run: cd ../v12 && npm ci';
  }
  return undefined;
}

/** Why the UPGRADE half cannot run, or undefined. Separate from the above: the v12 deploy still can. */
function upgradeBlockedReason(): string | undefined {
  const dir = join(PACKAGE_ROOT_ABS_PATH, 'create2-deploy', 'script');
  const missing = UPGRADE_SCRIPTS.filter((s) => !existsSync(join(dir, s)));
  if (missing.length === 0) return undefined;
  return (
    `v13's CREATE2 upgrade is not implemented yet — missing ${missing.join(', ')}. ` +
    'This subtest activates by itself once they exist.'
  );
}

////////////////////////////////////////////////////////////////////////////////
// Harness
////////////////////////////////////////////////////////////////////////////////

async function portIsOpen(): Promise<boolean> {
  try {
    await fetch(RPC_URL, {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'eth_chainId', params: [] }),
      signal: AbortSignal.timeout(500),
    });
    return true;
  } catch {
    return false;
  }
}

async function waitForNode(deadlineMs: number): Promise<void> {
  const until = Date.now() + deadlineMs;
  while (Date.now() < until) {
    if (await portIsOpen()) return;
    await new Promise((r) => setTimeout(r, 200));
  }
  throw new Error(`anvil did not answer on ${RPC_URL} within ${deadlineMs}ms`);
}

/**
 * Runs a coordinator and returns its combined output.
 *
 * Output is captured rather than inherited so a failure can be quoted INTO the assertion message. A
 * coordinator prints a long preflight; "exit code 1" on its own would send the reader hunting through
 * scrollback for the line that mattered.
 */
function runCoordinator(cwd: string, args: readonly string[]): { ok: boolean; output: string } {
  const r = spawnSync('node', args, {
    cwd,
    encoding: 'utf8',
    // The coordinators use anvil's public mnemonic when they detect anvil (via anvil_nodeInfo), so no
    // keystore and no password prompt. FHEVM_* env is left untouched: the config file carries everything.
    env: process.env,
    maxBuffer: 64 * 1024 * 1024,
  });
  // `encoding: 'utf8'` makes both streams strings, so neither needs a nullish guard.
  return { ok: r.status === 0, output: `${r.stdout}${r.stderr}` };
}

/** `.address.<ROLE>` out of a sealed manifest. */
function manifestAddresses(packageRoot: string): Readonly<Record<string, string>> {
  const path = join(outDirAbs(packageRoot), 'manifest.json');
  assert.ok(existsSync(path), `no manifest sealed at ${path}`);
  const parsed = JSON.parse(readFileSync(path, 'utf8')) as { address?: Record<string, string> };
  const addresses = parsed.address;
  assert.ok(addresses, `manifest at ${path} has no "address" object`);
  return addresses;
}

/**
 * `CONTRACT_VERSIONS` read as TEXT from a generation's generated `pkg/ts/versions.ts`.
 *
 * Text rather than an import for two reasons. `test/tsconfig.json` sets `rootDir: ".."`, so nothing here
 * may import from the sibling generation at all — and reading the generated file is the stronger check
 * anyway: it asserts against what the build actually produced rather than against whatever a compiled
 * copy in this process happens to hold.
 */
function contractVersions(packageRoot: string): Readonly<Record<string, string>> {
  const path = join(packageRoot, 'pkg', 'ts', 'versions.ts');
  assert.ok(existsSync(path), `no generated versions table at ${path} — run: npm run build:templates`);
  const src = readFileSync(path, 'utf8');
  const out: Record<string, string> = {};
  for (const [, key, value] of src.matchAll(/^\s{2}([a-zA-Z][a-zA-Z0-9]*): '([^']+)',$/gm)) {
    if (key === undefined || value === undefined) continue;
    out[key] = value;
  }
  assert.ok(Object.keys(out).length >= 5, `parsed only ${Object.keys(out).length} versions from ${path}`);
  return out;
}

/**
 * A contract as a plain typed object.
 *
 * ethers' `Contract` exposes methods through a proxy, so every call site would otherwise need a non-null
 * assertion that the linter forbids — and rightly: `contract.owner!()` asserts something the type system
 * genuinely does not know. Casting once, here, to the shape the ABI describes puts the claim in one place
 * where the ABI sits beside it.
 */
function view<T>(address: string, abi: readonly string[], provider: JsonRpcProvider): T {
  return new Contract(address, abi, provider) as unknown as T;
}

type VersionedView = { getVersion(): Promise<string> };
type OwnableView = { owner(): Promise<string>; pendingOwner(): Promise<string> };
type AclView = OwnableView & { getPauserSetAddress(): Promise<string> };
type PauserSetView = { isPauser(account: string): Promise<boolean> };

const VERSIONED_ABI = ['function getVersion() view returns (string)'];
const OWNABLE_ABI = ['function owner() view returns (address)', 'function pendingOwner() view returns (address)'];
const ACL_ABI = [...OWNABLE_ABI, 'function getPauserSetAddress() view returns (address)'];
const PAUSER_SET_ABI = ['function isPauser(address) view returns (bool)'];

const ZERO = `0x${'0'.repeat(40)}`;

/**
 * One role out of a manifest, validated.
 *
 * Folds the "is it there" and "does it look like an address" checks into the read itself. A role that is
 * absent would otherwise flow onward as `undefined` and be reported by whatever call happened to use it,
 * which is a long way from the manifest that failed to seal it.
 */
function addressOf(manifest: Readonly<Record<string, string>>, role: string): string {
  const a = manifest[role];
  assert.ok(a !== undefined && /^0x[0-9a-fA-F]{40}$/.test(a), `manifest has no usable ${role}`);
  return a;
}

async function versionOf(provider: JsonRpcProvider, address: string): Promise<string> {
  return await view<VersionedView>(address, VERSIONED_ABI, provider).getVersion();
}

/** A generation's version table, keyed the way `CONTRACT_VERSIONS` keys it. */
function expectVersion(table: Readonly<Record<string, string>>, key: string): string {
  const want = table[key];
  assert.ok(want !== undefined, `the generated versions table has no "${key}"`);
  return want;
}

////////////////////////////////////////////////////////////////////////////////
// The test
////////////////////////////////////////////////////////////////////////////////

/** The nine live roles v13's upgrade is handed as arguments. */
const LIVE_ROLES = [
  'ACL_ADDRESS',
  'FHEVM_EXECUTOR_ADDRESS',
  'KMS_VERIFIER_ADDRESS',
  'INPUT_VERIFIER_ADDRESS',
  'HCU_LIMIT_ADDRESS',
  'CLEARTEXT_ARITHMETIC_ADDRESS',
  'CLEARTEXT_DB_ADDRESS',
  'PAUSER_SET_ADDRESS',
  'ACL_OWNER',
] as const;

/**
 * `--acl 0x… --fhevm-executor 0x… …` — the nine live addresses as the upgrade's arguments.
 *
 * The flag is DERIVED from the role name rather than copied from `common.ts`'s `EXISTING_FLAGS`: drop a
 * trailing `_ADDRESS`, lowercase, and turn `_` into `-`. That rule reproduces all nine exactly, including
 * `ACL_OWNER` -> `--acl-owner`, which has no `_ADDRESS` to drop.
 *
 * Derived rather than imported because `test/tsconfig.json` does not include `create2-deploy/` — and
 * derived rather than listed because a second list of flags is a second thing to update. If the
 * coordinator ever renames one, it rejects the unknown flag and this test fails loudly, which is the
 * behaviour a hand-copied list would not have.
 */
function existingAddressArgs(manifest: Readonly<Record<string, string>>): string[] {
  return LIVE_ROLES.flatMap((role) => [
    `--${role
      .replace(/_ADDRESS$/, '')
      .toLowerCase()
      .replaceAll('_', '-')}`,
    addressOf(manifest, role),
  ]);
}

void test('create2-deploy: fresh anvil, v12 stack, then the v13 upgrade', { skip: blockedReason() }, async (t) => {
  if (await portIsOpen()) {
    t.skip(`something is already listening on ${RPC_URL} — refusing to deploy onto it`);
    return;
  }

  const v12Root = PREVIOUS_GENERATION_DIR_ABS_PATH;
  const v13Root = PACKAGE_ROOT_ABS_PATH;

  // anvil's DEFAULT mnemonic on purpose: it is `test test … junk`, whose accounts 0 and 1 are what both
  // coordinators fall back to when they detect anvil. Passing our own would bypass that fallback, which is
  // the path an operator rehearsing without a keystore actually uses.
  let node: ChildProcess | undefined;
  let provider: JsonRpcProvider | undefined;

  /**
   * Why anvil is gone, or undefined while it is running.
   *
   * A coordinator that outlives its node reports the loss as `Connection refused` from deep inside forge's
   * output, which reads as a deploy bug. Recording the exit here lets the assertion name the real cause.
   */
  let nodeGone: string | undefined;

  const killNode = (): void => {
    if (node?.exitCode === null && node.signalCode === null) {
      node.kill('SIGTERM');
    }
  };

  try {
    node = spawn('anvil', ['--silent', '--host', '127.0.0.1', '--port', String(PORT)], { stdio: 'ignore' });
    node.on('exit', (code, signal) => {
      nodeGone = `the anvil on ${RPC_URL} exited mid-run (code ${String(code)}, signal ${String(signal)})`;
    });
    // Killed on the way out even if this process dies abnormally, so a crashed run cannot leave an
    // orphaned node holding the port for the next one.
    process.on('exit', killNode);
    await waitForNode(60_000);
    provider = new JsonRpcProvider(RPC_URL, undefined, { staticNetwork: true });

    let v12: Readonly<Record<string, string>> = {};
    /** Whether the deploy sealed a manifest — the subtests below all read one. */
    let v12Deployed = false;

    await t.test('v12 deploys from scratch through its own CREATE2 coordinator', async () => {
      rmSync(outDirAbs(v12Root), { recursive: true, force: true });
      const { ok, output } = runCoordinator(v12Root, [
        'create2-deploy/deploy-testnet.ts',
        '--config',
        'create2-deploy/anvil-config.json',
        // --rpc-url beats the config file (common.ts resolveOptions), which is what lets this run on a
        // private port while reusing the checked-in rehearsal config for everything else.
        '--rpc-url',
        RPC_URL,
        '--out-dir',
        OUT_DIR_ARG,
        '--deployment-id',
        DEPLOYMENT_ID,
        '--stage',
        'all',
      ]);
      if (!ok) {
        // The node is PROBED rather than read off the child's `exit` event: `runCoordinator` uses
        // spawnSync, so that event is still queued here and would report a live node as dead.
        const detail = (await portIsOpen()) ? '' : ` — ${nodeGone ?? `the anvil on ${RPC_URL} stopped answering`}`;
        assert.fail(`v12 create2 deploy failed${detail}:\n${output.slice(-4000)}`);
      }
      v12 = manifestAddresses(v12Root);
      // Asserted here so a role the deploy failed to seal is reported against the deploy, rather than
      // later as an unhelpful "bad argument" from the upgrade.
      for (const role of LIVE_ROLES) addressOf(v12, role);
      v12Deployed = true;
    });

    /**
     * Why the subtests below cannot run, or undefined. They all read the manifest the deploy sealed, so
     * without one they report `manifest has no usable ACL_ADDRESS` — three failures for a single cause,
     * none of them naming it. Skipping leaves exactly one failure to read.
     */
    const noStackReason = (): string | undefined =>
      v12Deployed ? undefined : 'the v12 deploy above failed, so there is no stack to inspect';

    /** Skips `st` when there is no stack, and reports whether it did. */
    const needsV12 = (st: { skip: (reason: string) => void }): boolean => {
      const reason = noStackReason();
      if (reason === undefined) return false;
      st.skip(reason);
      return true;
    };

    // Captured BEFORE the upgrade: the invariants are about what did NOT change, and the
    // pre-upgrade values are gone by the time the upgrade has run.
    let ownerBefore = '';
    let adminBefore = '';
    let pauserSetBefore = '';

    await t.test('every deployed contract reports its v12 version', async (st) => {
      if (needsV12(st)) return;
      assert.ok(provider);
      const table = contractVersions(v12Root);
      for (const [role, key] of [
        ['ACL_ADDRESS', 'acl'],
        ['FHEVM_EXECUTOR_ADDRESS', 'fhevmExecutor'],
        ['KMS_VERIFIER_ADDRESS', 'kmsVerifier'],
        ['INPUT_VERIFIER_ADDRESS', 'inputVerifier'],
        ['HCU_LIMIT_ADDRESS', 'hcuLimit'],
        ['CLEARTEXT_ARITHMETIC_ADDRESS', 'cleartextArithmetic'],
        ['PAUSER_SET_ADDRESS', 'pauserSet'],
      ] as const) {
        assert.equal(await versionOf(provider, addressOf(v12, role)), expectVersion(table, key), role);
      }
    });

    await t.test('the v12 stack is owned through ACLOwner, with nothing dangling', async (st) => {
      if (needsV12(st)) return;
      assert.ok(provider);
      const acl = view<AclView>(addressOf(v12, 'ACL_ADDRESS'), ACL_ABI, provider);
      const aclOwner = view<OwnableView>(addressOf(v12, 'ACL_OWNER'), OWNABLE_ABI, provider);

      ownerBefore = await acl.owner();
      adminBefore = await aclOwner.owner();
      pauserSetBefore = await acl.getPauserSetAddress();

      assert.equal(ownerBefore.toLowerCase(), addressOf(v12, 'ACL_OWNER').toLowerCase(), 'ACL.owner() is ACLOwner');
      // A dangling pending owner on either contract is a latent takeover: whoever holds that key can
      // accept at any later moment. It is a completion condition, not tidiness.
      assert.equal(await acl.pendingOwner(), ZERO, 'ACL.pendingOwner() == 0');
      assert.equal(await aclOwner.pendingOwner(), ZERO, 'ACLOwner.pendingOwner() == 0');
      assert.equal(
        pauserSetBefore.toLowerCase(),
        addressOf(v12, 'PAUSER_SET_ADDRESS').toLowerCase(),
        'the ACL points at the sealed PauserSet',
      );
      const pauserSet = view<PauserSetView>(pauserSetBefore, PAUSER_SET_ABI, provider);
      assert.equal(await pauserSet.isPauser(addressOf(v12, 'ACL_OWNER')), true, 'ACLOwner is a pauser');
    });

    // A missing upgrade script is one reason to skip; a v12 stack that never came up is the other, and it
    // has to be checked here rather than in each subtest because there is nothing to upgrade either way.
    const upgradeSkip = upgradeBlockedReason() ?? noStackReason();

    await t.test('v13 upgrades the live v12 stack through its own CREATE2 coordinator', { skip: upgradeSkip }, () => {
      rmSync(outDirAbs(v13Root), { recursive: true, force: true });
      const { ok, output } = runCoordinator(v13Root, [
        'create2-deploy/upgrade-testnet.ts',
        '--config',
        'create2-deploy/anvil-config.json',
        '--rpc-url',
        RPC_URL,
        '--out-dir',
        OUT_DIR_ARG,
        // The SAME deployment id as the deploy, deliberately: `_salt` mixes `cfg.version`, so "0.13"
        // against the v12 deploy's "0.12" already yields a disjoint salt namespace for the same role
        // names. Reusing the id is therefore correct and preferred.
        '--deployment-id',
        DEPLOYMENT_ID,
        '--stage',
        'all',
        ...existingAddressArgs(v12),
      ]);
      assert.ok(ok, `v13 create2 upgrade failed:\n${output.slice(-4000)}`);
    });

    await t.test('the five re-pointed contracts now report v13 versions', { skip: upgradeSkip }, async () => {
      assert.ok(provider);
      const table = contractVersions(v13Root);
      for (const [role, key] of [
        ['ACL_ADDRESS', 'acl'],
        ['FHEVM_EXECUTOR_ADDRESS', 'fhevmExecutor'],
        ['KMS_VERIFIER_ADDRESS', 'kmsVerifier'],
        ['HCU_LIMIT_ADDRESS', 'hcuLimit'],
        ['CLEARTEXT_ARITHMETIC_ADDRESS', 'cleartextArithmetic'],
      ] as const) {
        assert.equal(await versionOf(provider, addressOf(v12, role)), expectVersion(table, key), `${role} -> v13`);
      }
      // The negative half, which a list of positive expectations cannot give: InputVerifier is
      // deliberately absent from the op list, so a moved version there means something re-pointed a proxy
      // nobody intended to touch.
      assert.equal(
        await versionOf(provider, addressOf(v12, 'INPUT_VERIFIER_ADDRESS')),
        expectVersion(table, 'inputVerifier'),
        'InputVerifier is untouched by the upgrade',
      );
    });

    await t.test('the two new v13 proxies exist at their predicted addresses', { skip: upgradeSkip }, async () => {
      assert.ok(provider);
      const v13 = manifestAddresses(v13Root);
      const table = contractVersions(v13Root);
      for (const [role, key] of [
        ['PROTOCOL_CONFIG_ADDRESS', 'protocolConfig'],
        ['KMS_GENERATION_ADDRESS', 'kmsGeneration'],
      ] as const) {
        const address = addressOf(v13, role);
        // Not just "has code": the two new proxies are the only addresses in this flow that were PREDICTED
        // rather than supplied, so this is where a colliding or mis-derived salt would show up.
        assert.notEqual(await provider.getCode(address), '0x', `no code at ${role}`);
        assert.equal(await versionOf(provider, address), expectVersion(table, key), role);
        for (const live of LIVE_ROLES) {
          assert.notEqual(address.toLowerCase(), addressOf(v12, live).toLowerCase(), `${role} collides with ${live}`);
        }
      }
    });

    await t.test('ownership and the pauser set survived the upgrade', { skip: upgradeSkip }, async () => {
      assert.ok(provider);
      // The invariant that makes the whole operation safe, and the one every version check above would
      // pass without noticing: an upgrade may move code, never authority.
      const acl = view<AclView>(addressOf(v12, 'ACL_ADDRESS'), ACL_ABI, provider);
      const aclOwner = view<OwnableView>(addressOf(v12, 'ACL_OWNER'), OWNABLE_ABI, provider);

      assert.equal(await acl.owner(), ownerBefore, 'ACL.owner() unchanged');
      assert.equal(await aclOwner.owner(), adminBefore, 'ACLOwner.owner() unchanged');
      assert.equal(await acl.pendingOwner(), ZERO, 'no ACL transfer was even started');
      assert.equal(await aclOwner.pendingOwner(), ZERO, 'no ACLOwner transfer was even started');
      assert.equal(
        (await acl.getPauserSetAddress()).toLowerCase(),
        pauserSetBefore.toLowerCase(),
        'the ACL still points at the same PauserSet',
      );
      const pauserSet = view<PauserSetView>(pauserSetBefore, PAUSER_SET_ABI, provider);
      assert.equal(await pauserSet.isPauser(addressOf(v12, 'ACL_OWNER')), true, 'ACLOwner is still a pauser');
    });
  } finally {
    provider?.destroy();
    killNode();
    process.off('exit', killNode);
    rmSync(outDirAbs(PREVIOUS_GENERATION_DIR_ABS_PATH), { recursive: true, force: true });
    rmSync(outDirAbs(PACKAGE_ROOT_ABS_PATH), { recursive: true, force: true });
  }
});
