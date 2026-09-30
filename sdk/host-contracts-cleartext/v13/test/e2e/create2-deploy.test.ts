// The CREATE2 path, fresh: a new anvil, a v13 stack deployed by this package's own `create2-deploy`
// coordinator, and the terminal conditions read back independently of it.
//
// Run: npm run test:create2-deploy-e2e
//
// The sibling `create2-upgrade.test.ts` starts from a v12 stack and upgrades it; this one has no previous
// generation at all. The two are separate on purpose: a broken fresh deploy and a broken upgrade are
// different defects, and one failing must not hide the other behind a skip.
//
// Same policies as the upgrade test: a private port checked for occupancy, a dedicated out-dir, skips
// that name what is missing, and a cleanup that leaves nothing behind.

import assert from 'node:assert/strict';
import { spawn, spawnSync, type ChildProcess } from 'node:child_process';
import { existsSync, readFileSync, rmSync } from 'node:fs';
import { join } from 'node:path';
import test from 'node:test';
import { Contract, JsonRpcProvider } from 'ethers';
import { PACKAGE_ROOT_ABS_PATH } from '../../internal/constants.ts';

////////////////////////////////////////////////////////////////////////////////
// Configuration
////////////////////////////////////////////////////////////////////////////////

/** Away from 8545 and from the upgrade test's 8557, so the two can never share a node. */
const PORT = 8558;
const RPC_URL = `http://127.0.0.1:${PORT}`;

/** Bare name: the coordinator resolves `--out-dir` against `create2-deploy/`, where forge may write. */
const OUT_DIR_ARG = '.out-test-create2-deploy';
const DEPLOYMENT_ID = 'create2-deploy-e2e';

function outDirAbs(): string {
  return join(PACKAGE_ROOT_ABS_PATH, 'create2-deploy', OUT_DIR_ARG);
}

////////////////////////////////////////////////////////////////////////////////
// Prerequisites and harness
////////////////////////////////////////////////////////////////////////////////

function haveBinary(name: string): boolean {
  return spawnSync(name, ['--version'], { stdio: 'ignore' }).status === 0;
}

function blockedReason(): string | undefined {
  for (const bin of ['anvil', 'forge'] as const) {
    if (!haveBinary(bin)) return `${bin} not found — install foundry (https://getfoundry.sh)`;
  }
  return undefined;
}

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

////////////////////////////////////////////////////////////////////////////////
// Progress
//
// The coordinator runs for a minute or more with two forge builds inside it. Its output is streamed as it
// happens, so the run is never silent, and each phase is announced first — the reporter's own tick only
// appears when a subtest ends. CREATE2_E2E_QUIET=1 keeps the streaming out of a CI log.
////////////////////////////////////////////////////////////////////////////////

const QUIET = process.env.CREATE2_E2E_QUIET === '1';
const STARTED_AT = Date.now();

function announce(n: number, total: number, what: string): void {
  const elapsed = ((Date.now() - STARTED_AT) / 1000).toFixed(0);
  process.stderr.write(`\n[${String(n)}/${String(total)}] ${what}  (+${elapsed}s)\n`);
}

function note(what: string): void {
  process.stderr.write(`    ${what}\n`);
}

/**
 * Runs the coordinator, streaming its output live AND capturing it, so the operator sees progress and a
 * failure can still be quoted into the assertion.
 */
function runCoordinator(args: readonly string[]): Promise<{ ok: boolean; output: string }> {
  return new Promise((resolve) => {
    const child = spawn('node', ['create2-deploy/deploy-testnet.ts', ...args], {
      cwd: PACKAGE_ROOT_ABS_PATH,
      env: process.env,
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    let output = '';
    const onData = (chunk: Buffer): void => {
      const text = chunk.toString();
      output += text;
      if (!QUIET) process.stderr.write(text.replace(/\n(?!$)/g, '\n    │ ').replace(/^/, '    │ '));
    };
    child.stdout.on('data', onData);
    child.stderr.on('data', onData);
    child.on('close', (code) => {
      resolve({ ok: code === 0, output });
    });
  });
}

const STEPS = 5;

const COMMON_ARGS = [
  '--config',
  'create2-deploy/anvil-config.json',
  '--rpc-url',
  RPC_URL,
  '--out-dir',
  OUT_DIR_ARG,
  '--deployment-id',
  DEPLOYMENT_ID,
] as const;

function manifestAddresses(): Readonly<Record<string, string>> {
  const path = join(outDirAbs(), 'manifest.json');
  assert.ok(existsSync(path), `no manifest sealed at ${path}`);
  const parsed = JSON.parse(readFileSync(path, 'utf8')) as { address?: Record<string, string> };
  assert.ok(parsed.address, `manifest at ${path} has no "address" object`);
  return parsed.address;
}

/** `CONTRACT_VERSIONS` read as text from the generated `pkg/ts/versions.ts` — what the build produced. */
function contractVersions(): Readonly<Record<string, string>> {
  const path = join(PACKAGE_ROOT_ABS_PATH, 'pkg', 'ts', 'versions.ts');
  assert.ok(existsSync(path), `no generated versions table at ${path} — run: make generate`);
  const out: Record<string, string> = {};
  for (const [, key, value] of readFileSync(path, 'utf8').matchAll(/^\s{2}([a-zA-Z][a-zA-Z0-9]*): '([^']+)',$/gm)) {
    if (key !== undefined && value !== undefined) out[key] = value;
  }
  assert.ok(Object.keys(out).length >= 5, `parsed only ${Object.keys(out).length} versions from ${path}`);
  return out;
}

function addressOf(manifest: Readonly<Record<string, string>>, role: string): string {
  const a = manifest[role];
  assert.ok(a !== undefined && /^0x[0-9a-fA-F]{40}$/.test(a), `manifest has no usable ${role}`);
  return a;
}

function expectVersion(table: Readonly<Record<string, string>>, key: string): string {
  const want = table[key];
  assert.ok(want !== undefined, `the generated versions table has no "${key}"`);
  return want;
}

type VersionedView = { getVersion(): Promise<string> };
type OwnableView = { owner(): Promise<string>; pendingOwner(): Promise<string> };
type AclView = OwnableView & { getPauserSetAddress(): Promise<string>; getFHEVMExecutorAddress(): Promise<string> };
type PauserSetView = { isPauser(account: string): Promise<boolean> };

const VERSIONED_ABI = ['function getVersion() view returns (string)'];
const OWNABLE_ABI = ['function owner() view returns (address)', 'function pendingOwner() view returns (address)'];
const ACL_ABI = [
  ...OWNABLE_ABI,
  'function getPauserSetAddress() view returns (address)',
  'function getFHEVMExecutorAddress() view returns (address)',
];
const PAUSER_SET_ABI = ['function isPauser(address) view returns (bool)'];
const ZERO = `0x${'0'.repeat(40)}`;

/** anvil account 1 — `anvil-config.json`'s `admin`, the account that takes root in step F. */
const ANVIL_ADMIN = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8';

function view<T>(address: string, abi: readonly string[], provider: JsonRpcProvider): T {
  return new Contract(address, abi, provider) as unknown as T;
}

////////////////////////////////////////////////////////////////////////////////
// The test
////////////////////////////////////////////////////////////////////////////////

/** Every proxy role the deploy materializes, with its key in the generated versions table. */
const VERSIONED_ROLES = [
  ['ACL_ADDRESS', 'acl'],
  ['FHEVM_EXECUTOR_ADDRESS', 'fhevmExecutor'],
  ['KMS_VERIFIER_ADDRESS', 'kmsVerifier'],
  ['INPUT_VERIFIER_ADDRESS', 'inputVerifier'],
  ['HCU_LIMIT_ADDRESS', 'hcuLimit'],
  ['PROTOCOL_CONFIG_ADDRESS', 'protocolConfig'],
  ['KMS_GENERATION_ADDRESS', 'kmsGeneration'],
  ['CLEARTEXT_ARITHMETIC_ADDRESS', 'cleartextArithmetic'],
  ['CLEARTEXT_DB_ADDRESS', 'cleartextDB'],
  ['PAUSER_SET_ADDRESS', 'pauserSet'],
] as const;

void test(
  'create2-deploy: fresh anvil, a v13 stack through the CREATE2 coordinator',
  { skip: blockedReason() },
  async (t) => {
    if (await portIsOpen()) {
      t.skip(`something is already listening on ${RPC_URL} — refusing to deploy onto it`);
      return;
    }

    let node: ChildProcess | undefined;
    let provider: JsonRpcProvider | undefined;
    let nodeGone: string | undefined;
    const killNode = (): void => {
      if (node?.exitCode === null && node.signalCode === null) node.kill('SIGTERM');
    };

    try {
      announce(1, STEPS, `start a fresh anvil on ${RPC_URL}`);
      node = spawn('anvil', ['--silent', '--host', '127.0.0.1', '--port', String(PORT)], { stdio: 'ignore' });
      node.on('exit', (code, signal) => {
        nodeGone = `the anvil on ${RPC_URL} exited mid-run (code ${String(code)}, signal ${String(signal)})`;
      });
      process.on('exit', killNode);
      await waitForNode(60_000);
      provider = new JsonRpcProvider(RPC_URL, undefined, { staticNetwork: true });

      let manifest: Readonly<Record<string, string>> = {};
      let deployed = false;

      await t.test('the coordinator deploys and verifies the whole stack in one run', async () => {
        announce(
          2,
          STEPS,
          'deploy v13 with deploy-testnet.ts --stage all (3 forge builds, 22 creates, steps A-F, verify)',
        );
        rmSync(outDirAbs(), { recursive: true, force: true });
        const { ok, output } = await runCoordinator([...COMMON_ARGS, '--stage', 'all']);
        if (!ok) {
          const detail = (await portIsOpen()) ? '' : ` — ${nodeGone ?? `the anvil on ${RPC_URL} stopped answering`}`;
          assert.fail(`v13 create2 deploy failed${detail}:\n${output.slice(-4000)}`);
        }
        assert.match(output, /OK - every terminal condition/, 'the deploy ran its own verify');
        manifest = manifestAddresses();
        for (const [role] of VERSIONED_ROLES) addressOf(manifest, role);
        addressOf(manifest, 'ACL_OWNER');
        deployed = true;
      });

      const needsStack = (st: { skip: (reason: string) => void }): boolean => {
        if (deployed) return false;
        st.skip('the deploy above failed, so there is no stack to inspect');
        return true;
      };

      await t.test('every contract reports its v13 version at its sealed address', async (st) => {
        if (needsStack(st)) return;
        announce(3, STEPS, 'read every getVersion() back against the generated table');
        assert.ok(provider);
        const table = contractVersions();
        for (const [role, key] of VERSIONED_ROLES) {
          const got = await view<VersionedView>(addressOf(manifest, role), VERSIONED_ABI, provider).getVersion();
          assert.equal(got, expectVersion(table, key), role);
        }
      });

      await t.test('the admin holds root through ACLOwner, with nothing dangling', async (st) => {
        if (needsStack(st)) return;
        announce(4, STEPS, 'read ownership, pending owners, pauser and baked addresses back');
        assert.ok(provider);
        const acl = view<AclView>(addressOf(manifest, 'ACL_ADDRESS'), ACL_ABI, provider);
        const aclOwner = view<OwnableView>(addressOf(manifest, 'ACL_OWNER'), OWNABLE_ABI, provider);
        const pauserSet = view<PauserSetView>(addressOf(manifest, 'PAUSER_SET_ADDRESS'), PAUSER_SET_ABI, provider);

        assert.equal((await acl.owner()).toLowerCase(), addressOf(manifest, 'ACL_OWNER').toLowerCase(), 'ACL.owner()');
        assert.equal(
          (await aclOwner.owner()).toLowerCase(),
          ANVIL_ADMIN.toLowerCase(),
          'ACLOwner.owner() is the admin',
        );
        // The deployer must not be root any more, and nobody may be able to become root later.
        assert.equal(await acl.pendingOwner(), ZERO, 'ACL.pendingOwner() == 0');
        assert.equal(await aclOwner.pendingOwner(), ZERO, 'ACLOwner.pendingOwner() == 0');
        assert.equal(await pauserSet.isPauser(addressOf(manifest, 'ACL_OWNER')), true, 'ACLOwner is a pauser');
        assert.equal(
          (await acl.getPauserSetAddress()).toLowerCase(),
          addressOf(manifest, 'PAUSER_SET_ADDRESS').toLowerCase(),
          'ACL bakes the sealed PauserSet',
        );
        assert.equal(
          (await acl.getFHEVMExecutorAddress()).toLowerCase(),
          addressOf(manifest, 'FHEVM_EXECUTOR_ADDRESS').toLowerCase(),
          'ACL bakes the sealed FHEVMExecutor',
        );
      });

      await t.test('re-running every stage is a no-op, and verify still holds', async (st) => {
        if (needsStack(st)) return;
        announce(5, STEPS, 're-run --stage all (must send nothing), then --stage status');
        const again = await runCoordinator([...COMMON_ARGS, '--stage', 'all']);
        assert.ok(again.ok, `second --stage all failed:\n${again.output.slice(-4000)}`);
        assert.match(again.output, /OK - every terminal condition/);
        note('status');
        const status = await runCoordinator([...COMMON_ARGS, '--stage', 'status']);
        assert.ok(status.ok, `status failed:\n${status.output.slice(-2000)}`);
        // A create verdict other than `done` names a role after it; the summary line `todo 0` does not.
        assert.doesNotMatch(status.output, /^\s+(todo|DRIFT|NO CODE|TOO BIG)\s+[A-Z_]/m, status.output.slice(-3000));
        assert.doesNotMatch(status.output, /BLOCKED|FATAL|WAITING/, status.output.slice(-3000));
      });
    } finally {
      provider?.destroy();
      killNode();
      process.off('exit', killNode);
      rmSync(outDirAbs(), { recursive: true, force: true });
    }
  },
);
