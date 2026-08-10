// deploy — brings the Solana side-stack online against a live fhevm-cli backend: program build +
// deploy on the geyser validator, the typed zama-host bootstrap (HostConfig + active KMS context
// from the REAL live gateway/ProtocolConfig values), the Solana host-chain registration
// (coprocessor DB + gateway), and the host-listener. Absorbed from `setup-solana-side.sh`; the
// bootstrap is the typed replacement for the retired live-client's `BOOTSTRAP=1` mode (the last
// production duty that Rust crate had), built from the same generated Codama client the scenarios
// use. Mock inputs and test shims stay OFF: everything is read live via `addresses.ts`, so the
// whole sequence is reproducible from a clean `fhevm-cli up --scenario solana`.
//
// Run directly (from test-suite/fhevm, after `fhevm-cli up --scenario solana`):
//   bun run src/solana/deploy.ts

import path from "node:path";

import { fetchEncodedAccount, type TransactionSigner } from "@solana/kit";

import { closeSync, openSync } from "node:fs";

import { envPath, REPO_ROOT, STATE_DIR } from "../layout";
import { readEnvFile } from "../utils/fs";
import { run, runStreaming } from "../utils/process";
import {
  BRINGUP_KMS_CONTEXT_ID,
  readGatewayBootstrapInputs,
  SOLANA_HOST_CHAIN_ID,
  SOLANA_HOST_CHAIN_ID_I64,
  type GatewayBootstrapInputs,
} from "./addresses";
import { zamaEventAuthorityAddress } from "./fhe-execute";
import {
  getDefineKmsContextInstructionAsync,
  getInitializeHostConfigInstructionAsync,
} from "./internal/generated/zamaHost/instructions/index.js";
import { findHostConfigPda } from "./internal/generated/zamaHost/pdas/index.js";
import { ZAMA_HOST_PROGRAM_ADDRESS } from "./internal/generated/zamaHost/programAddress.js";
import { createProvisioningContext, type SolanaProvisioningContext } from "./provision";
import {
  airdropDeployFees,
  ensureDeployerWallet,
  seedProgramKeypairs,
  SOLANA_E2E_PROGRAMS,
  startGeyserValidator,
  VALIDATOR_RPC_URL,
} from "./validator";

export type KmsCertificateThresholds = {
  /** Matching signatures a certificate needs: 2t+1 of the registered signer set. */
  readonly certificateThreshold: number;
};

/**
 * Derives the on-chain certificate threshold from the KMS corruption threshold t. A centralized
 * KMS (t=0) signs with one key; a threshold-mode KMS needs 2t+1 matching signatures, and KMS core
 * requires parties == 3t+1 (see scenarios/four-party-threshold-kms.yaml).
 */
export const kmsCertificateThresholds = (
  kmsCorruptionThreshold: number,
  registeredSignerCount: number,
): KmsCertificateThresholds => {
  const certificateThreshold = 2 * kmsCorruptionThreshold + 1;
  if (certificateThreshold > registeredSignerCount) {
    throw new Error(
      `KMS_THRESHOLD=${kmsCorruptionThreshold} needs 2t+1=${certificateThreshold} certificate ` +
        `signatures but only ${registeredSignerCount} KMS signers are registered on the gateway`,
    );
  }
  return { certificateThreshold };
};

export type BootstrapZamaHostParams = {
  readonly payer: TransactionSigner;
  readonly gateway: GatewayBootstrapInputs;
  /**
   * Input-attestation n-of-m. The PoC coprocessor emits a single attestation signature, so 1
   * keeps the live flow green while the full registered set is stored (EVM `InputVerifier`
   * parity).
   */
  readonly coprocessorThreshold?: number;
  /** KMS corruption threshold t; 0 is the centralized PoC default. */
  readonly kmsCorruptionThreshold?: number;
};

/**
 * Initializes the zama-host HostConfig (idempotent: the gateway outlives validator resets, the
 * config account does not, so a re-run against a fresh validator recreates it and a re-run against
 * a configured one skips it) and defines the active KMS context from the registered signer set.
 */
export const bootstrapZamaHost = async (
  context: SolanaProvisioningContext,
  params: BootstrapZamaHostParams,
): Promise<void> => {
  const eventAuthority = await zamaEventAuthorityAddress();
  const [hostConfig] = await findHostConfigPda();
  const shared = { eventAuthority, program: ZAMA_HOST_PROGRAM_ADDRESS } as const;

  const existing = await fetchEncodedAccount(context.rpc, hostConfig);
  if (existing.exists) {
    console.log("host_config already initialized — skipping initialize_host_config");
  } else {
    await context.sendTransaction(params.payer, [
      await getInitializeHostConfigInstructionAsync({
        payer: params.payer,
        admin: params.payer,
        chainId: SOLANA_HOST_CHAIN_ID,
        gatewayChainId: params.gateway.gatewayChainId,
        inputVerificationContract: params.gateway.inputVerificationContract,
        coprocessorSigners: [...params.gateway.coprocessorSigners],
        coprocessorThreshold: params.coprocessorThreshold ?? 1,
        decryptionContract: params.gateway.decryptionContract,
        grantDenyListEnabled: false,
        ...shared,
      }),
    ]);
    console.log("OK initialize_host_config");
  }

  const kmsCorruptionThreshold = params.kmsCorruptionThreshold ?? 0;
  const { certificateThreshold } = kmsCertificateThresholds(
    kmsCorruptionThreshold,
    params.gateway.kmsSigners.length,
  );
  await context.sendTransaction(params.payer, [
    await getDefineKmsContextInstructionAsync({
      admin: params.payer,
      contextId: BRINGUP_KMS_CONTEXT_ID,
      signers: [...params.gateway.kmsSigners],
      thresholds: {
        publicDecryption: certificateThreshold,
        userDecryption: certificateThreshold,
        kmsGen: certificateThreshold,
        // Mirrors the gateway's MPC_THRESHOLD, which is t itself and NOT 2t+1 (fhevm-cli
        // generates MPC_THRESHOLD=t alongside the =2t+1 decryption thresholds; see
        // src/kms-threshold.test.ts). Stored for fidelity, never gates on-chain verification.
        mpc: kmsCorruptionThreshold,
      },
      ...shared,
    }),
  ]);
  console.log(
    `OK define_kms_context (signers: ${params.gateway.kmsSigners.length}, ` +
      `t=${kmsCorruptionThreshold}, cert_threshold=${certificateThreshold})`,
  );
};

/** Reads the standard 64-byte Solana CLI keypair file into a kit signer. */
const loadKeypairSigner = async (keypairPath: string): Promise<TransactionSigner> => {
  const { createKeyPairSignerFromBytes } = await import("@solana/kit");
  const bytes = Uint8Array.from(JSON.parse(await Bun.file(keypairPath).text()) as number[]);
  return createKeyPairSignerFromBytes(bytes);
};

const SOLANA_DIR = path.join(REPO_ROOT, "solana");
const ENGINE_DIR = path.join(REPO_ROOT, "coprocessor", "fhevm-engine");

/**
 * Two separate `-p` builds, not one workspace build: these produce exactly the two
 * `target/deploy/*.so` the deploy loop reads, and naming them keeps the e2e from building the
 * batcher and the demo vault it never deploys. --use-rpc: deploy over RPC (8899) since the
 * container doesn't publish the TPU ports. Returns the deployed zama_host program id.
 */
const buildAndDeployPrograms = async (deployerKeypairPath: string): Promise<string> => {
  console.log("    building zama_host + confidential_token");
  for (const program of SOLANA_E2E_PROGRAMS) {
    await runStreaming(["anchor", "build", "--ignore-keys", "--no-idl", "-p", program], { cwd: SOLANA_DIR });
  }
  for (const program of SOLANA_E2E_PROGRAMS) {
    await run([
      "solana",
      "program",
      "deploy",
      "-u",
      VALIDATOR_RPC_URL,
      "-k",
      deployerKeypairPath,
      "--use-rpc",
      "--program-id",
      path.join(SOLANA_DIR, "target", "deploy", `${program}-keypair.json`),
      path.join(SOLANA_DIR, "target", "deploy", `${program}.so`),
    ]);
  }
  const zamaHostId = (
    await run(["solana", "address", "-k", path.join(SOLANA_DIR, "target", "deploy", "zama_host-keypair.json")])
  ).stdout.trim();
  console.log(`    zama_host=${zamaHostId} deployed`);
  return zamaHostId;
};

/** The coprocessor DB URL from the generated env, repointed at the host-published port. */
const readCoprocessorDatabaseUrl = async (): Promise<string> => {
  const environment = await readEnvFile(envPath("coprocessor"));
  const url = environment.DATABASE_URL;
  if (!url) throw new Error("missing DATABASE_URL in the generated coprocessor env");
  return url.replace("@db:", "@127.0.0.1:");
};

/**
 * The gateway addHostChain compose invocation, as argv. Pure so the unit suite can pin the one
 * lifecycle-critical property: every compose call runs under the per-boot `-p` project, never an
 * ambient default.
 */
export const gatewayAddHostChainArgs = (composeProject: string): string[] => [
  "docker",
  "compose",
  "-f",
  path.join(REPO_ROOT, "test-suite", "fhevm", "docker-compose", "gateway-sc-docker-compose.yml"),
  "-p",
  composeProject,
  "run",
  "--rm",
  "--no-deps",
  "-e",
  "NUM_HOST_CHAINS=1",
  "-e",
  `HOST_CHAIN_CHAIN_ID_0=${SOLANA_HOST_CHAIN_ID}`,
  "-e",
  "HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_0=0x0000000000000000000000000000000000000000",
  "-e",
  "HOST_CHAIN_ACL_ADDRESS_0=0x0000000000000000000000000000000000000000",
  "-e",
  "HOST_CHAIN_NAME_0=solana",
  "-e",
  "HOST_CHAIN_WEBSITE_0=https://zama.ai",
  "gateway-sc-add-network",
];

/**
 * Registers the Solana host chain in the coprocessor DB (host_chains i64 + keyset mirror) and on
 * the gateway (GatewayConfig.addHostChain). Both depend on the freshly-deployed program id and
 * post-keygen state, which is why they live here and not in the fhevm-cli config generator.
 */
const registerSolanaHostChain = async (parameters: {
  readonly zamaHostId: string;
  readonly composeProject: string;
}): Promise<void> => {
  // The relax-chain-id migration is baked into the db-migration override; apply idempotently as a
  // safety net.
  const migration = path.join(
    ENGINE_DIR,
    "db-migration",
    "migrations",
    "20260605120000_relax_chain_id_checks_for_solana_host.sql",
  );
  await run(["docker", "exec", "-i", "coprocessor-and-kms-db", "psql", "-U", "postgres", "-d", "coprocessor"], {
    input: await Bun.file(migration).text(),
    allowFailure: true,
  });
  await run([
    "docker",
    "exec",
    "coprocessor-and-kms-db",
    "psql",
    "-U",
    "postgres",
    "-d",
    "coprocessor",
    "-c",
    `INSERT INTO host_chains (chain_id,name,acl_contract_address) VALUES (${SOLANA_HOST_CHAIN_ID_I64},'solana','${parameters.zamaHostId}') ON CONFLICT DO NOTHING;
     INSERT INTO keys (key_id_gw,key_id,pks_key,sks_key,cks_key,sns_pk,chain_id,block_hash)
       SELECT key_id_gw,key_id,pks_key,sks_key,cks_key,sns_pk,${SOLANA_HOST_CHAIN_ID_I64},block_hash
         FROM keys WHERE chain_id=12345 ON CONFLICT DO NOTHING;`,
  ]);
  // zkproof-worker loads the host-chains cache once at startup (fhevm-engine-common
  // HostChainsCache), so it must be restarted to pick up the freshly-registered Solana host —
  // mirroring fhevm-cli's own registerExtraChainInCoprocessor (insert row + restart).
  await run(["docker", "restart", "coprocessor-zkproof-worker"]);
  for (let attempt = 0; attempt < 30; attempt += 1) {
    const running = await run(
      ["docker", "inspect", "-f", "{{.State.Running}}", "coprocessor-zkproof-worker"],
      { allowFailure: true },
    );
    if (running.stdout.trim() === "true") break;
    await Bun.sleep(1_000);
  }

  const gatewayVersion = (
    await run(["docker", "inspect", "gateway-sc-add-network", "--format", "{{.Config.Image}}"])
  ).stdout
    .trim()
    .replace(/^.*:/, "");
  // The gateway persists across local-validator resets, so addHostChain reverts with the
  // "host chain already registered" custom error (0x96a56828) on re-runs; tolerate that.
  const addHostChain = await run(gatewayAddHostChainArgs(parameters.composeProject), {
    env: { GATEWAY_VERSION: gatewayVersion, FHEVM_STATE_DIR: STATE_DIR },
    allowFailure: true,
  });
  const output = `${addHostChain.stdout}\n${addHostChain.stderr}`;
  if (output.includes("0x96a56828")) {
    console.log("    Solana host chain already registered on the gateway — ok");
  } else if (addHostChain.code !== 0 || /reverted|error occurred/i.test(output)) {
    throw new Error(`gateway addHostChain failed:\n${output.split("\n").slice(-6).join("\n")}`);
  }
};

/**
 * Builds and runs the Solana host-listener against the validator + DB. Always rebuilt from THIS
 * worktree's source: its event decoders are generated (build.rs -> OUT_DIR) from the program
 * IDLs, so a stale prebuilt binary silently decodes zero events when the program's event layout
 * has moved (it drops every event whose generated struct no longer matches), leaving the
 * coprocessor with no work and the vertical hanging at SNS commit.
 *
 * gRPC transport + off-chain reconstruction: the listener ingests ordinary outputs rebuilt from
 * transaction instructions; created-public lifecycle outputs retain a narrow CPI event.
 * Handle-derivation params are auto-detected from the on-chain HostConfig PDA at startup.
 */
const startHostListener = async (parameters: {
  readonly zamaHostId: string;
  readonly databaseUrl: string;
  readonly grpcUrl: string;
  readonly logDir: string;
  readonly lifecycleDir?: string;
}): Promise<void> => {
  if (parameters.lifecycleDir) {
    if ((await run(["pgrep", "-f", "solana_host_listener"], { allowFailure: true })).code === 0) {
      throw new Error("refusing to replace an unowned solana_host_listener in lifecycle mode");
    }
  } else {
    await run(["pkill", "-f", "solana_host_listener"], { allowFailure: true });
    await Bun.sleep(1_000);
  }
  const buildLog = "/tmp/solana-host-listener-build.log";
  const build = await run(
    ["cargo", "build", "-p", "host-listener", "--features", "solana-grpc,solana-reconstruct", "--bin", "solana_host_listener"],
    { cwd: ENGINE_DIR, allowFailure: true },
  );
  await Bun.write(buildLog, `${build.stdout}\n${build.stderr}`);
  if (build.code !== 0) {
    throw new Error(
      `host-listener (grpc,reconstruct) build failed; see ${buildLog}\n${build.stderr.split("\n").slice(-20).join("\n")}`,
    );
  }
  // One shared descriptor for stdout+stderr — the same interleaving `>log 2>&1` produces; the
  // child holds its own duplicate, so the parent copy closes right away.
  const logFd = openSync(path.join(parameters.logDir, "host-listener.log"), "w");
  const listener = Bun.spawn(
    [
      path.join(ENGINE_DIR, "target", "debug", "solana_host_listener"),
      "--grpc-url",
      parameters.grpcUrl,
      "--database-url",
      parameters.databaseUrl,
      "--url",
      VALIDATOR_RPC_URL,
      "--program-id",
      parameters.zamaHostId,
      `--host-chain-id=${SOLANA_HOST_CHAIN_ID_I64}`,
    ],
    { stdin: "ignore", stdout: logFd, stderr: logFd },
  );
  listener.unref();
  closeSync(logFd);
  if (parameters.lifecycleDir) {
    await Bun.write(path.join(parameters.lifecycleDir, "listener.pid"), `${listener.pid}\n`);
  }
};

const readIntegerEnv = (name: string, fallback: number): number => {
  const raw = process.env[name];
  if (raw === undefined || raw === "") return fallback;
  const value = Number.parseInt(raw, 10);
  if (!Number.isSafeInteger(value) || value < 0) throw new Error(`${name} must be a non-negative integer`);
  return value;
};

/** Validates the lifecycle Compose project shape `demo/lifecycle.ts` allocates. */
export const lifecycleComposeProject = (lifecycleDir: string | undefined): string => {
  if (!lifecycleDir) return "fhevm";
  const project = process.env.FHEVM_COMPOSE_PROJECT;
  if (
    !project ||
    !/^fhevm-demo-[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/.test(project)
  ) {
    throw new Error(`invalid lifecycle Compose project: ${project ?? "(unset)"}`);
  }
  return project;
};

const evmHex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString("hex")}`;

if (import.meta.main) {
  const lifecycleDir = process.env.DEMO_LIFECYCLE_DIR || undefined;
  const composeProject = lifecycleComposeProject(lifecycleDir);
  const logDir = process.env.SOLANA_LOG_DIR ?? "/tmp";
  // Deployer/fee-payer wallet: airdrop, program deploy, and the bootstrap all sign with it, and it
  // is passed explicitly everywhere so this setup never depends on or mutates the developer's
  // global `solana config` (URL or keypair). Same override the demo deployer honors
  // (deploy-demo-programs.sh).
  const deployerKeypairPath =
    process.env.SOLANA_DEPLOYER_KEYPAIR ?? `${process.env.HOME}/.config/solana/id.json`;

  // The gateway reads come first: a missing .env.gateway or a down gateway RPC should fail here,
  // not after the multi-minute program build. The resolved values go to the log — on a bootstrap
  // failure or a wrong-signer-set incident this is the record of what was registered.
  console.log("==> [1/5] gather live gateway inputs");
  const gateway = await readGatewayBootstrapInputs({
    gatewayRpcUrl: process.env.GW_RPC ?? "http://127.0.0.1:8546",
  });
  console.log(`    gateway_chain_id=${gateway.gatewayChainId}`);
  console.log(`    input_verification=${evmHex(gateway.inputVerificationContract)}`);
  console.log(`    decryption=${evmHex(gateway.decryptionContract)}`);
  console.log(`    coprocessor_signers=${gateway.coprocessorSigners.map(evmHex).join(",")}`);
  console.log(`    kms_signers=${gateway.kmsSigners.map(evmHex).join(",")}`);

  console.log("==> [2/5] fresh validator (Yellowstone geyser) + program deploy");
  await seedProgramKeypairs();
  await ensureDeployerWallet(deployerKeypairPath);
  await startGeyserValidator({
    lifecycleDir,
    ledgerDir: process.env.SOLANA_LEDGER_DIR,
    logDir,
    pluginLibPath: process.env.PLUGIN_LIB || undefined,
  });
  await airdropDeployFees(deployerKeypairPath);
  const zamaHostId = await buildAndDeployPrograms(deployerKeypairPath);

  console.log("==> [3/5] bootstrap zama-host (real gateway/ProtocolConfig values, mock/test OFF)");
  const payer = await loadKeypairSigner(deployerKeypairPath);
  const context = createProvisioningContext(VALIDATOR_RPC_URL, "ws://127.0.0.1:8900");
  await bootstrapZamaHost(context, {
    payer,
    gateway,
    coprocessorThreshold: readIntegerEnv("COPROCESSOR_THRESHOLD", 1),
    kmsCorruptionThreshold: readIntegerEnv("KMS_THRESHOLD", 0),
  });

  console.log("==> [4/5] register Solana host chain (coprocessor DB + gateway)");
  await registerSolanaHostChain({ zamaHostId, composeProject });

  console.log("==> [5/5] run Solana host-listener");
  await startHostListener({
    zamaHostId,
    databaseUrl: await readCoprocessorDatabaseUrl(),
    grpcUrl: process.env.GRPC_URL ?? "http://127.0.0.1:10000",
    logDir,
    lifecycleDir,
  });

  console.log(
    `==> Solana side-stack ready. zama_host=${zamaHostId} host_chain_id=${SOLANA_HOST_CHAIN_ID} (i64 ${SOLANA_HOST_CHAIN_ID_I64})`,
  );
  // The validator and host-listener are detached children; exit explicitly instead of waiting on
  // anything they hold open.
  process.exit(0);
}
