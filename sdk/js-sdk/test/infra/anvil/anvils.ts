// Reusable anvil orchestration for the fresh test infra.
//
// Spawns N cleartext anvils (one per WASM/protocol version) by shelling out to
// the existing `test/scripts/fhevm-anvil.sh` with `--skip-fhetest`, so each anvil
// deploys only the FHEVM stack (no TFHETest.sol). Readiness is detected by
// scanning the script's stdout for its "stack deployed" marker, which prints
// only after anvil is up AND the deploy has finished.
//
// Pure orchestration: no Playwright/Next/Vite. Consumers (Playwright globalSetup,
// a node-server runner, a CLI) import `startAnvils` / `stopAnvils`.
//
// Requires `anvil`/`cast`/`forge` on PATH (inherited from the caller's env).

import { type ChildProcess, execFileSync, spawn } from 'node:child_process';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { type FoundryProfile, SLOT_INFO } from '../config.js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const FHEVM_ANVIL_SH = resolve(__dirname, '../../scripts/fhevm-anvil.sh');

/** Marker printed by `localcleartext-run-tests.sh --ethlib none` once deployed. */
const DEPLOY_MARKER = 'stack deployed';
const DEFAULT_CHAIN_ID = 31337;
const DEFAULT_READY_TIMEOUT_MS = 180_000;
const PORT_DOWN_TIMEOUT_MS = 10_000;
// Probes MUST be bounded: a wedged/half-open listener (accepts the socket but
// never responds) would otherwise hang `fetch` forever and stall startAnvils.
const PROBE_TIMEOUT_MS = 3_000;

// contracts/ lives at the SDK root; anvils.ts is test/infra/anvil/.
const CONTRACTS_DIR = resolve(__dirname, '../../../contracts');

export type { FoundryProfile };

export type AnvilSpec = {
  /** Gateway slot id this anvil backs (e.g. 'v12'). */
  readonly slot: string;
  /** TCP port for the anvil RPC. */
  readonly port: number;
  /** Foundry profile → protocol version → WASM version (v12→1.5.4, v13→1.6.1). */
  readonly foundryProfile: FoundryProfile;
  /** anvil chain id (default 31337). */
  readonly chainId?: number | undefined;
  /**
   * Deployer mnemonic. If set, passed to the deploy as `DEPLOYER_MNEMONIC` so the
   * stack lands at a distinct address set, and the regenerated FHEVMHostAddresses.sol
   * is restored afterwards. If omitted, the deploy's built-in default is used.
   */
  readonly deployerMnemonic?: string | undefined;
  /**
   * This slot's ACL proxy address (derived from its deployer). Used to confirm a
   * reused anvil actually has THIS slot's FHEVM stack deployed — not just an open
   * port, and not another slot's stack at a different address.
   */
  readonly aclAddress: string;
};

export type AnvilHandle = {
  readonly spec: AnvilSpec;
  readonly rpcUrl: string;
  /** True when an already-running anvil on `port` was reused (not spawned here). */
  readonly reused: boolean;
  readonly proc: ChildProcess | undefined;
};

export type StartAnvilsOptions = {
  readonly readyTimeoutMs?: number | undefined;
};

/**
 * Starts all anvils, resolving once every one has deployed.
 *
 * Deploys run **sequentially**, not concurrently: the cleartext deploy writes a
 * shared `test/.env.localcleartext` (its `RPC_URL`) and runs Foundry in the
 * shared `contracts/` working dir, so two parallel deploys clobber each other's
 * target anvil → "nonce too low / EOA nonce changed unexpectedly". (anvil itself
 * resolves fast; it's the deploy that must be serialized.)
 */
export async function startAnvils(specs: readonly AnvilSpec[], options?: StartAnvilsOptions): Promise<AnvilHandle[]> {
  const handles: AnvilHandle[] = [];
  for (const spec of specs) {
    handles.push(await _startAnvil(spec, options));
  }
  return handles;
}

/** Stops the anvils this module spawned (reused ones are left running). */
export async function stopAnvils(handles: readonly AnvilHandle[]): Promise<void> {
  await Promise.all(handles.map(async (handle) => _stopProc(handle.proc)));
}

async function _startAnvil(spec: AnvilSpec, options?: StartAnvilsOptions): Promise<AnvilHandle> {
  const rpcUrl = `http://127.0.0.1:${String(spec.port)}`;
  const chainId = spec.chainId ?? DEFAULT_CHAIN_ID;
  const prefix = `[anvil:${spec.slot}]`;

  // Verified reuse: only reuse an anvil that is BOTH the expected chain id AND
  // has the FHEVM stack deployed. "Port responds" alone is not enough — a stale
  // anvil (wrong chain id, or up-but-undeployed) would otherwise be silently
  // reused, surfacing as a confusing failure later (e.g. missing ACL at init).
  const existingChainId = await _probeChainId(rpcUrl);
  if (existingChainId !== undefined) {
    const deployed = await _hasCode(rpcUrl, spec.aclAddress);
    if (existingChainId === chainId && deployed) {
      return { spec, rpcUrl, reused: true, proc: undefined };
    }
    // Stale anvil on a dedicated test port — replace it so the run lands in a
    // known-correct state instead of inheriting garbage.
    process.stdout.write(
      `${prefix} replacing stale anvil on ${rpcUrl} ` +
        `(chainId=${String(existingChainId)}, deployed=${String(deployed)}; ` +
        `want chainId=${String(chainId)}, deployed=true)\n`,
    );
    _killPort(spec.port);
    await _waitForPortDown(rpcUrl, PORT_DOWN_TIMEOUT_MS);
  }

  // A custom deployer mnemonic lands the stack at distinct addresses; pass it as
  // DEPLOYER_MNEMONIC (inherited by fhevm-deploy.sh). Omitted → deploy uses default.
  const deployEnv: NodeJS.ProcessEnv = { ...process.env, PORT: String(spec.port), CHAIN_ID: String(chainId) };
  if (spec.deployerMnemonic !== undefined) {
    deployEnv.DEPLOYER_MNEMONIC = spec.deployerMnemonic;
  }

  const proc = spawn('bash', [FHEVM_ANVIL_SH, `--foundry-profile=${spec.foundryProfile}`, '--skip-fhetest'], {
    env: deployEnv,
    stdio: ['ignore', 'pipe', 'pipe'],
  });

  await _waitForDeploy(proc, spec, options?.readyTimeoutMs ?? DEFAULT_READY_TIMEOUT_MS);

  // A custom-deployer deploy regenerates the profile's committed FHEVMHostAddresses.sol
  // with the new addresses. The anvil already holds the deployed contracts, so restore
  // the file to keep the working tree clean (the default-mnemonic slot regenerates its
  // file identically and needs no restore).
  if (spec.deployerMnemonic !== undefined) {
    _restoreAddressesFile(spec);
  }

  return { spec, rpcUrl, reused: false, proc };
}

/** `git checkout` the profile's generated addresses file (best-effort). */
function _restoreAddressesFile(spec: AnvilSpec): void {
  const file = resolve(
    CONTRACTS_DIR,
    'src',
    SLOT_INFO[spec.foundryProfile].profileVersionDir,
    'host-contracts/addresses/FHEVMHostAddresses.sol',
  );
  try {
    execFileSync('git', ['checkout', '--', file], { cwd: CONTRACTS_DIR, stdio: 'ignore' });
  } catch {
    process.stderr.write(`[anvil:${spec.slot}] warning: could not restore ${file}\n`);
  }
}

function _waitForDeploy(proc: ChildProcess, spec: AnvilSpec, timeoutMs: number): Promise<void> {
  return new Promise<void>((resolvePromise, reject) => {
    const prefix = `[anvil:${spec.slot}]`;
    let settled = false;

    const finish = (err?: Error): void => {
      if (settled) {
        return;
      }
      settled = true;
      clearTimeout(timer);
      if (err !== undefined) {
        reject(err);
      } else {
        resolvePromise();
      }
    };

    proc.stdout?.on('data', (chunk: Buffer) => {
      const text = chunk.toString();
      process.stdout.write(`${prefix} ${text}`);
      if (text.includes(DEPLOY_MARKER)) {
        finish();
      }
    });
    proc.stderr?.on('data', (chunk: Buffer) => {
      process.stderr.write(`${prefix} ${chunk.toString()}`);
    });
    proc.on('exit', (code) => {
      finish(new Error(`${prefix} exited before deploy completed (code ${String(code)}).`));
    });
    proc.on('error', (err: Error) => {
      finish(err);
    });

    const timer = setTimeout(() => {
      finish(new Error(`${prefix} not ready within ${String(timeoutMs)}ms.`));
    }, timeoutMs);
  });
}

async function _rpcCall(rpcUrl: string, method: string, params: readonly unknown[]): Promise<unknown> {
  const res = await fetch(rpcUrl, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ jsonrpc: '2.0', id: 1, method, params }),
    signal: AbortSignal.timeout(PROBE_TIMEOUT_MS),
  });
  if (!res.ok) {
    return undefined;
  }
  const json = (await res.json()) as { result?: unknown };
  return json.result;
}

/** Returns the anvil's chain id, or `undefined` if nothing answers on `rpcUrl`. */
async function _probeChainId(rpcUrl: string): Promise<number | undefined> {
  try {
    const result = await _rpcCall(rpcUrl, 'eth_chainId', []);
    return typeof result === 'string' ? Number.parseInt(result, 16) : undefined;
  } catch {
    return undefined;
  }
}

/** Whether `address` has deployed bytecode (i.e. not `0x`). */
async function _hasCode(rpcUrl: string, address: string): Promise<boolean> {
  try {
    const result = await _rpcCall(rpcUrl, 'eth_getCode', [address, 'latest']);
    return typeof result === 'string' && result.length > 2;
  } catch {
    return false;
  }
}

/** SIGKILLs whatever holds `port` (foreign anvil we didn't spawn). No-op if free. */
function _killPort(port: number): void {
  let output: string;
  try {
    output = execFileSync('lsof', ['-ti', `tcp:${String(port)}`], { encoding: 'utf-8' });
  } catch {
    return; // lsof exits non-zero when nothing is listening
  }
  for (const line of output.split('\n')) {
    const pid = Number.parseInt(line.trim(), 10);
    if (Number.isInteger(pid) && pid > 0) {
      try {
        process.kill(pid, 'SIGKILL');
      } catch {
        // already gone
      }
    }
  }
}

function _sleep(ms: number): Promise<void> {
  return new Promise((resolvePromise) => {
    setTimeout(resolvePromise, ms);
  });
}

async function _waitForPortDown(rpcUrl: string, timeoutMs: number): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    if ((await _probeChainId(rpcUrl)) === undefined) {
      return;
    }
    await _sleep(200);
  }
  throw new Error(`Port still in use after kill: ${rpcUrl}`);
}

function _stopProc(proc: ChildProcess | undefined): Promise<void> {
  return new Promise<void>((resolvePromise) => {
    if (proc === undefined || proc.exitCode !== null) {
      resolvePromise();
      return;
    }
    proc.once('exit', () => resolvePromise());
    proc.kill('SIGTERM');
    const force = setTimeout(() => {
      if (proc.exitCode === null) {
        proc.kill('SIGKILL');
      }
    }, 5_000);
    force.unref();
  });
}
