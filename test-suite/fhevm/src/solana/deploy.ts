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
import { closeSync, openSync } from 'node:fs';
import path from 'node:path';

import { SOLANA_LEAF_PROOF_API_KEY, SOLANA_LEAF_PROOF_PORT } from '../generate/solana';
import { REPO_ROOT, STATE_DIR, envPath } from '../layout';
import { readEnvFile } from '../utils/fs';
import { run, runStreaming } from '../utils/process';
import { until } from '../utils/until';
import { SOLANA_HOST_CHAIN_ID, SOLANA_HOST_CHAIN_ID_I64, readGatewayBootstrapInputs } from './addresses';
import { registerSolanaCoprocessorSql } from '../../../../solana/deploy/src/coprocessor';
import { deployHostProgram } from '../../../../solana/deploy/src/deploy-host';
import { deployProgramArtifacts } from '../../../../solana/deploy/src/deploy-programs';
import { withDeploymentLock } from '../../../../solana/deploy/src/lock';
import {
  SOLANA_E2E_PROGRAMS,
  VALIDATOR_RPC_URL,
  airdropDeployFees,
  ensureDeployerWallet,
  seedProgramKeypairs,
  startGeyserValidator,
} from './validator';

export { bootstrapZamaHost, kmsCertificateThreshold } from '../../../../solana/deploy/src/bootstrap';

const SOLANA_DIR = path.join(REPO_ROOT, 'solana');
const ENGINE_DIR = path.join(REPO_ROOT, 'coprocessor', 'fhevm-engine');

const buildAndDeployPrograms = async (
  deployerKeypairPath: string,
  gateway: Awaited<ReturnType<typeof readGatewayBootstrapInputs>>,
): Promise<string> => {
  await runStreaming(['bash', 'scripts/build-programs.sh', 'localnet', ...SOLANA_E2E_PROGRAMS], { cwd: SOLANA_DIR });
  const artifactsDir = path.join(SOLANA_DIR, 'target', 'deploy');
  const databaseUrl = await readCoprocessorDatabaseUrl();
  const ids = await deployHostProgram({
    databaseUrl,
    rpcUrl: VALIDATOR_RPC_URL,
    deployerKeypairPath,
    artifactsDir,
    gateway,
    coprocessorThreshold: readIntegerEnv('COPROCESSOR_THRESHOLD', 1),
    kmsCorruptionThreshold: readIntegerEnv('KMS_THRESHOLD', 0),
    programKeypairPath: path.join(artifactsDir, 'zama_host-keypair.json'),
  });
  const specimens = SOLANA_E2E_PROGRAMS.filter((program) => program !== 'zama_host');
  await withDeploymentLock(databaseUrl, VALIDATOR_RPC_URL, ids.zama_host!, (signal) =>
    deployProgramArtifacts({
      rpcUrl: VALIDATOR_RPC_URL,
      deployerKeypairPath,
      artifactsDir,
      programs: specimens,
      programKeypairPaths: Object.fromEntries(
        specimens.map((program) => [program, path.join(artifactsDir, `${program}-keypair.json`)]),
      ),
      signal,
    }),
  );
  return ids.zama_host!;
};

/** The coprocessor DB URL from the generated env, repointed at the host-published port. */
export const readCoprocessorDatabaseUrl = async (): Promise<string> => {
  const environment = await readEnvFile(envPath('coprocessor'));
  const url = environment.DATABASE_URL;
  if (!url) throw new Error('missing DATABASE_URL in the generated coprocessor env');
  return url.replace('@db:', '@127.0.0.1:');
};

/**
 * The gateway addHostChain compose invocation, as argv. Pure so the unit suite can pin the one
 * lifecycle-critical property: every compose call runs under the per-boot `-p` project, never an
 * ambient default.
 */
export const gatewayAddHostChainArgs = (composeProject: string): string[] => [
  'docker',
  'compose',
  '-f',
  path.join(REPO_ROOT, 'test-suite', 'fhevm', 'docker-compose', 'gateway-sc-docker-compose.yml'),
  '-p',
  composeProject,
  'run',
  '--rm',
  '--no-deps',
  '-e',
  'NUM_HOST_CHAINS=1',
  '-e',
  `HOST_CHAIN_CHAIN_ID_0=${SOLANA_HOST_CHAIN_ID}`,
  '-e',
  'HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_0=0x0000000000000000000000000000000000000000',
  '-e',
  'HOST_CHAIN_ACL_ADDRESS_0=0x0000000000000000000000000000000000000000',
  '-e',
  'HOST_CHAIN_NAME_0=solana',
  '-e',
  'HOST_CHAIN_WEBSITE_0=https://zama.ai',
  'gateway-sc-add-network',
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
    'db-migration',
    'migrations',
    '20260605120000_relax_chain_id_checks_for_solana_host.sql',
  );
  await run(['docker', 'exec', '-i', 'coprocessor-and-kms-db', 'psql', '-U', 'postgres', '-d', 'coprocessor'], {
    input: await Bun.file(migration).text(),
    allowFailure: true,
  });
  await run([
    'docker',
    'exec',
    'coprocessor-and-kms-db',
    'psql',
    '-U',
    'postgres',
    '-d',
    'coprocessor',
    '-c',
    registerSolanaCoprocessorSql(parameters.zamaHostId, '12345'),
  ]);
  // zkproof-worker loads the host-chains cache once at startup (fhevm-engine-common
  // HostChainsCache), so it must be restarted to pick up the freshly-registered Solana host —
  // mirroring fhevm-cli's own registerExtraChainInCoprocessor (insert row + restart).
  await run(['docker', 'restart', 'coprocessor-zkproof-worker']);
  await until(
    async () => {
      const running = await run(['docker', 'inspect', '-f', '{{.State.Running}}', 'coprocessor-zkproof-worker'], {
        allowFailure: true,
      });
      return running.stdout.trim() === 'true';
    },
    { timeoutMs: 30_000, intervalMs: 1_000, description: 'zkproof-worker restart' },
  );

  const gatewayVersion = (
    await run(['docker', 'inspect', 'gateway-sc-add-network', '--format', '{{.Config.Image}}'])
  ).stdout
    .trim()
    .replace(/^.*:/, '');
  // The gateway persists across local-validator resets, so addHostChain reverts with the
  // "host chain already registered" custom error (0x96a56828) on re-runs; tolerate that.
  const addHostChain = await run(gatewayAddHostChainArgs(parameters.composeProject), {
    env: { GATEWAY_VERSION: gatewayVersion, FHEVM_STATE_DIR: STATE_DIR },
    allowFailure: true,
  });
  const output = `${addHostChain.stdout}\n${addHostChain.stderr}`;
  if (output.includes('0x96a56828')) {
    console.log('    Solana host chain already registered on the gateway — ok');
  } else if (addHostChain.code !== 0 || /reverted|error occurred/i.test(output)) {
    throw new Error(`gateway addHostChain failed:\n${output.split('\n').slice(-6).join('\n')}`);
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
export const startHostListener = async (parameters: {
  readonly zamaHostId: string;
  readonly databaseUrl: string;
  readonly grpcUrl: string;
  readonly logDir: string;
  readonly lifecycleDir?: string;
}): Promise<void> => {
  if (parameters.lifecycleDir) {
    if ((await run(['pgrep', '-f', 'solana_host_listener'], { allowFailure: true })).code === 0) {
      throw new Error('refusing to replace an unowned solana_host_listener in lifecycle mode');
    }
  } else {
    await run(['pkill', '-f', 'solana_host_listener'], { allowFailure: true });
    // Poll it gone rather than sleeping a flat second: the next step binds the same resources.
    await until(async () => (await run(['pgrep', '-f', 'solana_host_listener'], { allowFailure: true })).code !== 0, {
      description: 'previous solana_host_listener to exit',
      timeoutMs: 15_000,
      intervalMs: 100,
    });
  }
  const buildLog = '/tmp/solana-host-listener-build.log';
  const build = await run(
    [
      'cargo',
      'build',
      '-p',
      'host-listener',
      '--features',
      'solana-grpc,solana-reconstruct',
      '--bin',
      'solana_host_listener',
    ],
    { cwd: ENGINE_DIR, allowFailure: true },
  );
  await Bun.write(buildLog, `${build.stdout}\n${build.stderr}`);
  if (build.code !== 0) {
    throw new Error(
      `host-listener (grpc,reconstruct) build failed; see ${buildLog}\n${build.stderr.split('\n').slice(-20).join('\n')}`,
    );
  }
  // One shared descriptor for stdout+stderr — the same interleaving `>log 2>&1` produces; the
  // child holds its own duplicate, so the parent copy closes right away.
  const logFd = openSync(path.join(parameters.logDir, 'host-listener.log'), 'a');
  const listener = Bun.spawn(
    [
      path.join(ENGINE_DIR, 'target', 'debug', 'solana_host_listener'),
      '--grpc-url',
      parameters.grpcUrl,
      '--database-url',
      parameters.databaseUrl,
      '--url',
      VALIDATOR_RPC_URL,
      '--program-id',
      parameters.zamaHostId,
      // The leaf-proof route the KMS connector reads. `--proof-api-key` has no default and the
      // binary refuses to start without it.
      '--http-port',
      String(SOLANA_LEAF_PROOF_PORT),
      '--proof-api-key',
      SOLANA_LEAF_PROOF_API_KEY,
    ],
    { stdin: 'ignore', stdout: logFd, stderr: logFd },
  );
  listener.unref();
  closeSync(logFd);
  if (parameters.lifecycleDir) {
    await Bun.write(path.join(parameters.lifecycleDir, 'listener.pid'), `${listener.pid}\n`);
  }
};

const readIntegerEnv = (name: string, fallback: number): number => {
  const raw = process.env[name];
  if (raw === undefined || raw === '') return fallback;
  const value = Number(raw);
  if (!Number.isSafeInteger(value) || value < 0) throw new Error(`${name} must be a non-negative integer`);
  return value;
};

/** Validates the lifecycle Compose project shape `demo/lifecycle.ts` allocates. */
export const lifecycleComposeProject = (lifecycleDir: string | undefined): string => {
  if (!lifecycleDir) return 'fhevm';
  const project = process.env.FHEVM_COMPOSE_PROJECT;
  if (
    !project ||
    !/^fhevm-demo-[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/.test(project)
  ) {
    throw new Error(`invalid lifecycle Compose project: ${project ?? '(unset)'}`);
  }
  return project;
};

const evmHex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString('hex')}`;

/**
 * Brings the Solana host node online against an already-running fhevm stack: fresh geyser
 * validator, program deploy, zama-host bootstrap from live gateway values, host-chain
 * registration, and the host-listener.
 *
 * Every input falls back to the same environment variable the standalone script read, so calling
 * this from `fhevm-cli up` and running `bun run src/solana/deploy.ts` provision identically.
 *
 * The validator and host-listener are detached (`unref`, own log descriptors), so this returns
 * once they are up rather than waiting on them.
 */
export const provisionSolanaHostNode = async (): Promise<{ zamaHostId: string }> => {
  const lifecycleDir = process.env.DEMO_LIFECYCLE_DIR || undefined;
  const composeProject = lifecycleComposeProject(lifecycleDir);
  const logDir = process.env.SOLANA_LOG_DIR ?? '/tmp';
  // Deployer/fee-payer wallet: airdrop, program deploy, and the bootstrap all sign with it, and it
  // is passed explicitly everywhere so this setup never depends on or mutates the developer's
  // global `solana config` (URL or keypair). Same override the demo deployer honors
  // (deploy-demo-programs.sh).
  const deployerKeypairPath = process.env.SOLANA_DEPLOYER_KEYPAIR ?? `${process.env.HOME}/.config/solana/id.json`;

  // The gateway reads come first: a missing .env.gateway or a down gateway RPC should fail here,
  // not after the multi-minute program build. The resolved values go to the log — on a bootstrap
  // failure or a wrong-signer-set incident this is the record of what was registered.
  console.log('==> [1/5] gather live gateway inputs');
  const gateway = await readGatewayBootstrapInputs({
    gatewayRpcUrl: process.env.GW_RPC ?? 'http://127.0.0.1:8546',
  });
  console.log(`    gateway_chain_id=${gateway.gatewayChainId}`);
  console.log(`    input_verification=${evmHex(gateway.inputVerificationContract)}`);
  console.log(`    decryption=${evmHex(gateway.decryptionContract)}`);
  console.log(`    coprocessor_signers=${gateway.coprocessorSigners.map(evmHex).join(',')}`);
  console.log(`    kms_signers=${gateway.kmsSigners.map(evmHex).join(',')}`);

  console.log('==> [2/5] fresh validator (Yellowstone geyser) + program deploy');
  await seedProgramKeypairs();
  await ensureDeployerWallet(deployerKeypairPath);
  await startGeyserValidator({
    lifecycleDir,
    ledgerDir: process.env.SOLANA_LEDGER_DIR,
    logDir,
    pluginLibPath: process.env.PLUGIN_LIB || undefined,
  });
  await airdropDeployFees(deployerKeypairPath);
  const zamaHostId = await buildAndDeployPrograms(deployerKeypairPath, gateway);

  console.log('==> [4/5] register Solana host chain (coprocessor DB + gateway)');
  await registerSolanaHostChain({ zamaHostId, composeProject });

  console.log('==> [5/5] run Solana host-listener');
  await startHostListener({
    zamaHostId,
    databaseUrl: await readCoprocessorDatabaseUrl(),
    grpcUrl: process.env.GRPC_URL ?? 'http://127.0.0.1:10000',
    logDir,
    lifecycleDir,
  });

  console.log(
    `==> Solana side-stack ready. zama_host=${zamaHostId} host_chain_id=${SOLANA_HOST_CHAIN_ID} (i64 ${SOLANA_HOST_CHAIN_ID_I64})`,
  );
  return { zamaHostId };
};

if (import.meta.main) {
  await provisionSolanaHostNode();
  // The validator and host-listener are detached children; exit explicitly instead of waiting on
  // anything they hold open.
  process.exit(0);
}
