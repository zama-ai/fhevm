// validator — the host-native Solana test validator the e2e side-stack runs on, absorbed from
// `setup-solana-side.sh` "[1/4] fresh validator (Yellowstone geyser) + program deploy".
//
// Host = native solana-test-validator (agave 4.1.2, pinned in solana-e2e.yml) with the Yellowstone
// geyser plugin (gRPC :10000) loaded via --geyser-plugin-config. The real validator runs on the
// host arch directly (multi-arch incl. Apple Silicon) and — unlike surfpool's LiteSVM — streams
// the SlotHashes/Clock sysvar accounts the off-chain reconstruction needs per slot. The RPC
// listener binds 0.0.0.0 on its own, so the dockerized KMS worker reaches it over
// host.docker.internal:8899. Local only — no mainnet exposure: the RPC URL is pinned to
// 127.0.0.1:8899 by the callers.

import { closeSync, openSync } from "node:fs";
import { copyFile, lstat, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import path from "node:path";

import { REPO_ROOT } from "../layout";
import { run } from "../utils/process";

export const VALIDATOR_RPC_URL = "http://127.0.0.1:8899";
/** The two PoC programs the side-stack deploys, in deploy order. */
export const SOLANA_E2E_PROGRAMS = ["zama_host", "confidential_token"] as const;

const SOLANA_DIR = path.join(REPO_ROOT, "solana");

/** Renders the committed Yellowstone config template against the resolved plugin cdylib path. */
export const renderGeyserConfig = (template: string, pluginLibPath: string): string =>
  template.replaceAll("@LIBPATH@", pluginLibPath);

/**
 * The validator start arguments.
 *
 * --bind-address must not be 0.0.0.0 on agave 4.x: TestValidator::start passes bind_ip_addr
 * straight through as the gossip advertised IP (test-validator/src/lib.rs:1045), discarding the
 * loopback fallback the CLI computed, and ContactInfo::set_gossip now rejects an unspecified
 * address — the validator panics in gossip before the geyser plugin is ever loaded. Nothing is
 * lost by pinning loopback: the RPC and pubsub listeners always bind 0.0.0.0 regardless of this
 * flag (test-validator/src/lib.rs:1110-1119), so the dockerized workers still reach
 * host.docker.internal:8899.
 *
 * --deactivate-feature B8JJ… (disable_sbpf_v0_v1_v2_deployment): solana-test-validator activates
 * every feature it knows about at genesis, which here makes it STRICTER than the network we
 * target. That feature has no account on mainnet-beta at all, so mainnet still accepts sbpf
 * v0/v1/v2 deployments, but a local validator with it on rejects ours with "Detected sbpf_version
 * required by the executable which are not enabled". Deactivating it matches mainnet. Drop this
 * flag once the programs are built as sbpf v3.
 */
export const validatorStartArgs = (parameters: {
  readonly ledgerDir: string;
  readonly geyserConfigPath: string;
}): string[] => [
  "solana-test-validator",
  "--reset",
  "--rpc-port",
  "8899",
  "--bind-address",
  "127.0.0.1",
  "--ledger",
  parameters.ledgerDir,
  "--deactivate-feature",
  "B8JJXCy5amZyWG9r7EnUYLwzXSXTxG7GZ1qZ1qggo83g",
  "--geyser-plugin-config",
  parameters.geyserConfigPath,
];

/** Matches the lifecycle-owned ledger path shape `demo/lifecycle.ts` allocates. */
export const isLifecycleLedgerPath = (ledgerDir: string, uid: number): boolean =>
  new RegExp(`^/tmp/fhevm-demo-${uid}/[0-9a-f]{24}\\.ledger$`).test(ledgerDir);

// lstat, not stat: a symlink planted at the ledger path must fail the check (the bash used
// `find -prune -type d` for the same reason), not be followed to some other owned directory.
const isOwnedDirectory = async (directory: string, uid: number): Promise<boolean> => {
  try {
    const info = await lstat(directory);
    return info.isDirectory() && info.uid === uid;
  } catch {
    return false;
  }
};

const processMatching = async (pattern: string): Promise<boolean> =>
  (await run(["pgrep", "-f", pattern], { allowFailure: true })).code === 0;

const killMatching = async (pattern: string): Promise<void> => {
  await run(["pkill", "-f", pattern], { allowFailure: true });
  await Bun.sleep(2_000);
};

export type ValidatorStartOptions = {
  /** Lifecycle runtime dir; set = lifecycle mode (own, never replace; pid files written here). */
  readonly lifecycleDir?: string;
  /** Lifecycle-owned ledger path (required in lifecycle mode; validated against the owned shape). */
  readonly ledgerDir?: string;
  /** Where the validator's stdout log lands (`$SOLANA_LOG_DIR/validator.log` in the bash). */
  readonly logDir: string;
  /** Resolved Yellowstone plugin cdylib; built on demand when omitted. */
  readonly pluginLibPath?: string;
};

/**
 * solana-test-validator prints only its startup banner to stdout; everything that explains a
 * failed start — geyser plugin load errors included — goes to `<ledger>/validator.log`. Dump
 * both, or a plugin that refuses to load looks like three lines of banner and no reason.
 */
const validatorDeathReport = async (logPath: string, ledgerDir: string): Promise<string> => {
  const tail = async (file: string, lines: number): Promise<string> => {
    try {
      return (await readFile(file, "utf8")).split("\n").slice(-lines).join("\n");
    } catch {
      return "(no log written)";
    }
  };
  return [
    `--- stdout (${logPath}) ---`,
    await tail(logPath, 20),
    `--- ledger log (${ledgerDir}/validator.log) ---`,
    await tail(path.join(ledgerDir, "validator.log"), 60),
  ].join("\n");
};

/**
 * Starts a fresh geyser validator and waits for RPC health. Lifecycle mode refuses to replace an
 * unowned validator and requires a short, owned ledger path; standalone mode replaces any running
 * validator and resets `<repo>/.solana-test-ledger`.
 */
export const startGeyserValidator = async (options: ValidatorStartOptions): Promise<void> => {
  const uid = process.getuid?.() ?? 0;
  if (options.lifecycleDir) {
    if (await processMatching("solana-test-validator")) {
      throw new Error("refusing to replace an unowned solana-test-validator in lifecycle mode");
    }
  } else {
    await killMatching("solana-test-validator");
  }

  const pluginLibPath =
    options.pluginLibPath ??
    (await run(["bash", path.join(SOLANA_DIR, "geyser", "build-yellowstone-plugin.sh")])).stdout.trim();
  if (!(await Bun.file(pluginLibPath).exists())) {
    throw new Error(`Yellowstone plugin not found (PLUGIN_LIB=${pluginLibPath})`);
  }
  const geyserConfigPath = path.join(SOLANA_DIR, "target", "yellowstone-config.runtime.json");
  await mkdir(path.dirname(geyserConfigPath), { recursive: true });
  await writeFile(
    geyserConfigPath,
    renderGeyserConfig(await readFile(path.join(SOLANA_DIR, "geyser", "yellowstone-config.json"), "utf8"), pluginLibPath),
  );
  console.log(`    geyser host: solana-test-validator + Yellowstone plugin ${pluginLibPath}`);

  let ledgerDir: string;
  if (options.lifecycleDir) {
    ledgerDir = options.ledgerDir ?? "";
    if (!isLifecycleLedgerPath(ledgerDir, uid)) {
      throw new Error("invalid lifecycle-owned Solana ledger path");
    }
    for (const directory of [path.dirname(ledgerDir), ledgerDir]) {
      if (!(await isOwnedDirectory(directory, uid))) {
        throw new Error(`Solana ledger path ${directory} is not a real owned directory`);
      }
    }
  } else {
    ledgerDir = path.join(REPO_ROOT, ".solana-test-ledger");
    await rm(ledgerDir, { recursive: true, force: true });
  }

  await mkdir(options.logDir, { recursive: true });
  const logPath = path.join(options.logDir, "validator.log");
  // One shared descriptor for stdout+stderr — the same interleaving `>log 2>&1` produces; the
  // child holds its own duplicate, so the parent copy closes right away.
  const logFd = openSync(logPath, "w");
  const proc = Bun.spawn(validatorStartArgs({ ledgerDir, geyserConfigPath }), {
    stdin: "ignore",
    stdout: logFd,
    stderr: logFd,
  });
  proc.unref();
  closeSync(logFd);
  if (options.lifecycleDir) {
    await writeFile(path.join(options.lifecycleDir, "validator.pid"), `${proc.pid}\n`);
  }

  for (;;) {
    try {
      const response = await fetch(VALIDATOR_RPC_URL, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ jsonrpc: "2.0", id: 1, method: "getHealth" }),
        signal: AbortSignal.timeout(2_000),
      });
      if (response.ok && /"ok"/.test(await response.text())) break;
    } catch {
      // Not up yet; fall through to the liveness check below.
    }
    const alive = options.lifecycleDir
      ? isProcessAlive(proc.pid)
      : await processMatching("solana-test-validator");
    if (!alive) {
      throw new Error(`geyser validator died:\n${await validatorDeathReport(logPath, ledgerDir)}`);
    }
    await Bun.sleep(1_000);
  }
};

const isProcessAlive = (pid: number): boolean => {
  try {
    process.kill(pid, 0);
    return true;
  } catch {
    return false;
  }
};

/**
 * Seeds the committed well-known PoC program keypairs so the build reuses them and the deployed
 * program IDs match each `declare_id!` (see scripts/e2e/test-keypairs/README.md). Always
 * overwrites target artifacts: they may have been generated by an older branch with different
 * program IDs.
 */
export const seedProgramKeypairs = async (
  deployDir: string = path.join(SOLANA_DIR, "target", "deploy"),
): Promise<void> => {
  await mkdir(deployDir, { recursive: true });
  for (const program of SOLANA_E2E_PROGRAMS) {
    await copyFile(
      path.join(SOLANA_DIR, "scripts", "e2e", "test-keypairs", `${program}-keypair.json`),
      path.join(deployDir, `${program}-keypair.json`),
    );
  }
};

/**
 * Ensures the deployer wallet exists, created only if absent so a developer's existing wallet is
 * untouched; fresh CI runners have none (otherwise deploy fails with "No default signer found").
 * Also pre-creates `~/.cache/solana`: cargo-build-sbf was observed on agave 2.1.x to panic with a
 * NotFound error instead of creating its platform-tools cache on a fresh machine. Kept across the
 * 4.1 bump because it costs nothing and nobody has re-tested the fresh-machine path.
 */
export const ensureDeployerWallet = async (keypairPath: string): Promise<void> => {
  await mkdir(path.dirname(keypairPath), { recursive: true });
  if (!(await Bun.file(keypairPath).exists())) {
    await run(["solana-keygen", "new", "--no-bip39-passphrase", "--silent", "-o", keypairPath]);
  }
  await mkdir(path.join(process.env.HOME ?? "", ".cache", "solana"), { recursive: true });
};

/** Airdrops deploy-fee SOL to the deployer wallet; tolerated to fail on an already-funded wallet. */
export const airdropDeployFees = async (keypairPath: string): Promise<void> => {
  await run(["solana", "airdrop", "500", "-u", VALIDATOR_RPC_URL, "-k", keypairPath], { allowFailure: true });
};
