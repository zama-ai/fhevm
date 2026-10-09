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

import { bytesToHex } from '@fhevm/sdk/base';
import {
  type KmsThresholds,
  createFinalizedRpc,
  fetchHostConfig,
  fetchKmsContext,
  findHostConfigPda,
  findKmsContextPda,
} from '@fhevm/solana-zama-host';
import type { Address, ReadonlyUint8Array } from '@solana/kit';

import { registerSolanaCoprocessorSql } from '../../../../solana/deploy/src/coprocessor';
import { deployHostProgram } from '../../../../solana/deploy/src/deploy-host';
import { deployProgramArtifacts } from '../../../../solana/deploy/src/deploy-programs';
import {
  COPROCESSOR_DB_CONTAINER,
  DEFAULT_CHAIN_ID,
  REPO_ROOT,
  SOLANA_HOST_CHAIN_ID,
  SOLANA_MERKLE_PROOF_PORT,
  SOLANA_MERKLE_DATABASE,
  SOLANA_MERKLE_DB_COMPONENT,
  SOLANA_MERKLE_DB_CONTAINER,
  SOLANA_MERKLE_INDEXER_HEALTH_PORT,
  SOLANA_MERKLE_POSTGRES_PORT,
  STATE_DIR,
  STATE_FILE,
  coprocessorDatabaseName,
  envPath,
  solanaListenerHealthPort,
} from '../layout';
import { restartZkproofWorker, waitForContainer } from '../flow/readiness';
import { composeUp } from '../flow/runtime-compose';
import { topologyForState } from '../stack-spec/stack-spec';
import { loadState } from '../state/state';
import type { State } from '../types';
import { LOCAL_SOLANA_ENDPOINTS } from './endpoints';
import { readEnvFile } from '../utils/fs';
import { run, runStreaming } from '../utils/process';
import { until } from '../utils/until';
import {
  type ActiveKmsPair,
  BRINGUP_KMS_CONTEXT_ID,
  readActiveKmsPair,
  readEvmKmsSignersForContext,
  readEvmKmsThresholds,
  readGatewayBootstrapInputs,
  readProtocolConfigAddress,
  uint256Bytes,
} from './addresses';
import {
  SOLANA_E2E_PROGRAMS,
  SOLANA_SPECIMEN_PROGRAMS,
  VALIDATOR_RPC_URL,
  airdropDeployFees,
  ensureDeployerWallet,
  genesisDeployedPrograms,
  seedProgramKeypairs,
  startGeyserValidator,
} from './validator';

export { bootstrapZamaHost, kmsCertificateThreshold } from '../../../../solana/deploy/src/bootstrap';

const SOLANA_DIR = path.join(REPO_ROOT, 'solana');
const ENGINE_DIR = path.join(REPO_ROOT, 'coprocessor', 'fhevm-engine');

const ARTIFACTS_DIR = path.join(SOLANA_DIR, 'target', 'deploy');

/** Builds every program before the validator starts: the deployed four load at genesis. */
const buildPrograms = async (): Promise<void> => {
  await runStreaming(['bash', 'scripts/build-programs.sh', 'preview-env', ...SOLANA_E2E_PROGRAMS], { cwd: SOLANA_DIR });
};

/**
 * The deployed programs are already in place at genesis, so the host step only checks bytecode and
 * bootstraps HostConfig; the specimens deploy from their committed keypairs.
 */
const deployPrograms = async (
  deployerKeypairPath: string,
  gateway: Awaited<ReturnType<typeof readGatewayBootstrapInputs>>,
  thresholds: ReturnType<typeof bootstrapThresholdsForState>,
): Promise<string> => {
  const ids = await deployHostProgram({
    rpcUrl: VALIDATOR_RPC_URL,
    deployerKeypairPath,
    artifactsDir: ARTIFACTS_DIR,
    chainId: SOLANA_HOST_CHAIN_ID,
    gateway,
    ...thresholds,
  });
  await assertKmsContextMatchesEvmHost(ids.zama_host!, BRINGUP_KMS_CONTEXT_ID);
  await deployProgramArtifacts({
    rpcUrl: VALIDATOR_RPC_URL,
    deployerKeypairPath,
    artifactsDir: ARTIFACTS_DIR,
    programs: SOLANA_SPECIMEN_PROGRAMS,
    programKeypairPaths: Object.fromEntries(
      SOLANA_SPECIMEN_PROGRAMS.map((program) => [program, path.join(ARTIFACTS_DIR, `${program}-keypair.json`)]),
    ),
  });
  return ids.zama_host!;
};

/**
 * Coprocessor `index`'s DB URL from its generated env, repointed at the host-published port.
 * Coprocessor 0 owns the unsuffixed `coprocessor` env.
 */
export const readCoprocessorDatabaseUrl = async (index = 0): Promise<string> => {
  const environment = await readEnvFile(envPath(index === 0 ? 'coprocessor' : `coprocessor.${index}`));
  const url = environment.DATABASE_URL;
  if (!url) throw new Error(`missing DATABASE_URL in the generated env of coprocessor ${index}`);
  return url.replace('@db:', '@127.0.0.1:');
};

/**
 * Coprocessor 0's registered signer from the generated env: the Merkle proof server runs for it,
 * and the generated `KMS_CONNECTOR_HOST_CHAINS` names it as that server's signer.
 */
export const readCoprocessorSignerAddress = async (): Promise<string> => {
  const address = (await readEnvFile(envPath('host-sc'))).COPROCESSOR_SIGNER_ADDRESS_0;
  if (!address) throw new Error('missing COPROCESSOR_SIGNER_ADDRESS_0 in the generated host-sc env');
  return address;
};

/** The running stack's fhevm-cli state. */
export const readStackState = async (): Promise<State> => {
  const state = await loadState();
  if (!state) throw new Error(`no fhevm-cli state at ${STATE_FILE}; run fhevm-cli up first`);
  return state;
};

/**
 * The deployer and fee-payer wallet, which is also the HostConfig admin: airdrop, program deploy,
 * the bootstrap and later admin instructions all sign with it. Every caller passes it explicitly,
 * so the side stack never depends on or mutates the developer's global `solana config` (URL or
 * keypair). Same override the demo deployer honors (deploy-demo-programs.sh).
 */
export const solanaDeployerKeypairPath = (): string =>
  process.env.SOLANA_DEPLOYER_KEYPAIR ?? `${process.env.HOME}/.config/solana/id.json`;

/**
 * The host bootstrap thresholds, from the scenario the EVM stack was rendered from: the
 * coprocessor topology and the KMS corruption threshold t.
 */
export const bootstrapThresholdsForState = (state: Pick<State, 'scenario'>) => ({
  coprocessorThreshold: topologyForState(state).threshold,
  kmsCorruptionThreshold: state.scenario.kms.threshold,
});

/** Throws unless the Solana KMS context carries the thresholds of the EVM host's KMS context. */
export const assertKmsThresholdsMatch = (solana: KmsThresholds, evm: KmsThresholds): void => {
  const mismatches = (Object.keys(evm) as (keyof KmsThresholds)[])
    .filter((name) => solana[name] !== evm[name])
    .map((name) => `${name}: solana=${solana[name]} evm=${evm[name]}`);
  if (mismatches.length) {
    throw new Error(`Solana KMS context thresholds differ from the EVM ProtocolConfig: ${mismatches.join(', ')}`);
  }
};

/** Codama decodes account bytes as `ReadonlyUint8Array`, which the SDK's `bytesToHex` does not take. */
const accountBytesHex = (bytes: ReadonlyUint8Array): `0x${string}` => `0x${Buffer.from(bytes).toString('hex')}`;

/**
 * Throws unless the Solana KMS context lists the EVM context's signers in the same order. zama-host's
 * certificate check counts distinct members of the set and ignores the order, but SDK user decrypt
 * maps KMS party ids to positions in this list (fhevm-internal#2182), so the order must be
 * ProtocolConfig's.
 */
export const assertKmsSignersMatch = (
  solana: readonly ReadonlyUint8Array[],
  evm: readonly ReadonlyUint8Array[],
): void => {
  const hex = (signers: readonly ReadonlyUint8Array[]) => signers.map(accountBytesHex);
  if (hex(solana).join() !== hex(evm).join()) {
    throw new Error(
      `Solana KMS context signers differ from the EVM ProtocolConfig: solana=[${hex(solana)}] evm=[${hex(evm)}]`,
    );
  }
};

/**
 * Both hosts accept certificates from the same KMS, so zama-host must hold the EVM context's
 * signers, in ProtocolConfig's order, and its thresholds. A threshold set too low or a wrong signer
 * list still passes every functional test, so both are checked here.
 */
export const assertKmsContextMatchesEvmHost = async (zamaHostId: string, contextId: Uint8Array): Promise<void> => {
  const [kmsContext] = await findKmsContextPda({ contextId }, { programAddress: zamaHostId as Address });
  const solana = (await fetchKmsContext(createFinalizedRpc(VALIDATOR_RPC_URL), kmsContext)).data;
  const hostRpcUrl = LOCAL_SOLANA_ENDPOINTS.hostRpc;
  const [evmThresholds, evmSigners] = await Promise.all([
    readEvmKmsThresholds({ hostRpcUrl }),
    readEvmKmsSignersForContext({ hostRpcUrl, contextId: BigInt(bytesToHex(contextId)) }),
  ]);
  assertKmsThresholdsMatch(solana.thresholds, evmThresholds);
  assertKmsSignersMatch(solana.signers, evmSigners);
};

/** Throws unless zama-host's active KMS context and epoch are the EVM ProtocolConfig's active pair. */
export const assertActiveKmsPairMatches = (
  solana: { readonly currentKmsContextId: ReadonlyUint8Array; readonly currentKmsEpochId: ReadonlyUint8Array },
  evm: ActiveKmsPair,
): void => {
  const active = `${accountBytesHex(solana.currentKmsContextId)}/${accountBytesHex(solana.currentKmsEpochId)}`;
  const expected = `${accountBytesHex(uint256Bytes(evm.kmsContextId))}/${accountBytesHex(uint256Bytes(evm.kmsEpochId))}`;
  if (active !== expected) {
    throw new Error(`zama-host's active KMS context/epoch ${active} differs from the EVM ProtocolConfig's ${expected}`);
  }
};

/** Reads zama-host's HostConfig back after a mirror and checks its active pair against the EVM host's. */
export const assertActiveKmsPairMatchesEvmHost = async (zamaHostId: string): Promise<void> => {
  const [hostConfig] = await findHostConfigPda({ programAddress: zamaHostId as Address });
  const [solana, evm] = await Promise.all([
    fetchHostConfig(createFinalizedRpc(VALIDATOR_RPC_URL), hostConfig),
    readActiveKmsPair({ hostRpcUrl: LOCAL_SOLANA_ENDPOINTS.hostRpc }),
  ]);
  assertActiveKmsPairMatches(solana.data, evm);
};

/** Both Postgres containers read their credentials from the generated `database.env`. */
const merkleDatabaseUrl = (coprocessorDatabaseUrl: string): string => {
  const url = new URL(coprocessorDatabaseUrl);
  url.port = String(SOLANA_MERKLE_POSTGRES_PORT);
  url.pathname = `/${SOLANA_MERKLE_DATABASE}`;
  return url.toString();
};

/**
 * Starts the Merkle record's Postgres in a new container. The record follows this validator's
 * ledger, which every provision starts fresh, so a record left from an earlier ledger would fail
 * its resume.
 */
const startMerkleDatabase = async (): Promise<void> => {
  await composeUp(SOLANA_MERKLE_DB_COMPONENT, [], { forceRecreate: true });
  await waitForContainer(SOLANA_MERKLE_DB_CONTAINER, 'healthy');
};

/** The validator's finalized slot: an existing block the Merkle indexer can start from. */
const finalizedSlot = async (): Promise<bigint> => {
  const { createFinalizedRpc } = await import('@fhevm/solana-zama-host');
  return createFinalizedRpc(VALIDATOR_RPC_URL).getSlot().send();
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
 * Registers the Solana host chain in every coprocessor's DB (host_chains i64 + keyset mirror) and
 * on the gateway (GatewayConfig.addHostChain). Both depend on the freshly-deployed program id and
 * post-keygen state, which is why they live here and not in the fhevm-cli config generator.
 */
const registerSolanaHostChain = async (parameters: {
  readonly zamaHostId: string;
  readonly composeProject: string;
  readonly coprocessorCount: number;
}): Promise<void> => {
  // The relax-chain-id migration is baked into the db-migration override; apply idempotently as a
  // safety net.
  const migration = await Bun.file(
    path.join(ENGINE_DIR, 'db-migration', 'migrations', '20260605120000_relax_chain_id_checks_for_solana_host.sql'),
  ).text();
  for (let index = 0; index < parameters.coprocessorCount; index += 1) {
    const psql = ['docker', 'exec', '-i', COPROCESSOR_DB_CONTAINER, 'psql', '-U', 'postgres', '-d', coprocessorDatabaseName(index)];
    await run(psql, { input: migration, allowFailure: true });
    await run([...psql, '-c', registerSolanaCoprocessorSql(parameters.zamaHostId, DEFAULT_CHAIN_ID, SOLANA_HOST_CHAIN_ID)]);
    // zkproof-worker loads the host-chains cache once at startup (fhevm-engine-common
    // HostChainsCache), as fhevm-cli's registerExtraChainInCoprocessor does for an EVM chain.
    await restartZkproofWorker(index, 'after registering the Solana host chain');
  }

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
 * Builds and runs one Solana host-listener per coprocessor against the validator and that
 * coprocessor's DB, as each preview coprocessor runs its own. Always rebuilt from THIS
 * worktree's source: its event decoders are generated (build.rs -> OUT_DIR) from the program
 * IDLs, so a stale prebuilt binary silently decodes zero events when the program's event layout
 * has moved (it drops every event whose generated struct no longer matches), leaving the
 * coprocessor with no work and the vertical hanging at SNS commit.
 *
 * gRPC transport: the listener takes every output, public ones included, from each execution's
 * `FheExecutedEvent`, and re-derives the handles only as a check.
 * Handle-derivation params are auto-detected from the on-chain HostConfig PDA at startup.
 */
export const startHostListeners = async (parameters: {
  readonly zamaHostId: string;
  readonly coprocessorCount: number;
  readonly grpcUrl: string;
  readonly logDir: string;
  readonly lifecycleDir?: string;
}): Promise<void> => {
  await replaceSolanaProcess('solana_host_listener', parameters.lifecycleDir);
  await buildSolanaBinary('solana_host_listener', HOST_LISTENER_PACKAGE);
  for (let index = 0; index < parameters.coprocessorCount; index += 1) {
    // Coprocessor 0 keeps the unsuffixed log and pid names the demo lifecycle reads.
    const suffix = index === 0 ? '' : `-${index}`;
    await spawnSolanaProcess(
      'solana_host_listener',
      [
        '--grpc-url',
        parameters.grpcUrl,
        '--database-url',
        await readCoprocessorDatabaseUrl(index),
        '--url',
        VALIDATOR_RPC_URL,
        '--program-id',
        parameters.zamaHostId,
        '--http-port',
        String(solanaListenerHealthPort(index)),
      ],
      path.join(parameters.logDir, `host-listener${suffix}.log`),
      parameters.lifecycleDir && path.join(parameters.lifecycleDir, `listener${suffix}.pid`),
    );
  }
};

/** Coprocessor `index`'s listener checkpoint slot, or 0 before it has recorded a block. */
const listenerCheckpointSlot = async (index: number): Promise<bigint> => {
  const result = await run(
    [
      'docker',
      'exec',
      COPROCESSOR_DB_CONTAINER,
      'psql',
      '-U',
      'postgres',
      '-d',
      coprocessorDatabaseName(index),
      '-tAc',
      'SELECT COALESCE(MAX(slot), 0) FROM solana_listener_checkpoint',
    ],
    { allowFailure: true },
  );
  return result.code === 0 ? BigInt(result.stdout.trim()) : 0n;
};

/**
 * Waits until every listener's checkpoint moves past `startSlots`. The listeners run detached, so
 * one that fails at startup, or that is up but receives no finalized blocks, fails here rather
 * than as a stalled scenario. Preview checks the same.
 */
const waitForListenerCheckpoints = async (startSlots: readonly bigint[]): Promise<void> => {
  for (const [index, startSlot] of startSlots.entries()) {
    await until(async () => (await listenerCheckpointSlot(index)) > startSlot, {
      description: `coprocessor ${index}'s Solana listener checkpoint to pass slot ${startSlot}`,
      timeoutMs: 120_000,
      intervalMs: 1_000,
    });
  }
};

/**
 * Builds and runs the Merkle indexer, which records every encrypted store's leaves into its own
 * database from `startSlot`, a block before any store exists.
 */
export const startMerkleIndexer = async (parameters: {
  readonly zamaHostId: string;
  readonly databaseUrl: string;
  readonly startSlot: bigint;
  readonly grpcUrl: string;
  readonly logDir: string;
  readonly lifecycleDir?: string;
}): Promise<void> => {
  await replaceSolanaProcess('solana_merkle_indexer', parameters.lifecycleDir);
  await buildSolanaBinary('solana_merkle_indexer', MERKLE_PROOF_SERVICE_PACKAGE);
  await spawnSolanaProcess(
    'solana_merkle_indexer',
    [
      '--grpc-url',
      parameters.grpcUrl,
      '--database-url',
      parameters.databaseUrl,
      '--url',
      VALIDATOR_RPC_URL,
      '--program-id',
      parameters.zamaHostId,
      '--start-slot',
      String(parameters.startSlot),
      '--http-port',
      String(SOLANA_MERKLE_INDEXER_HEALTH_PORT),
    ],
    path.join(parameters.logDir, 'merkle-indexer.log'),
    parameters.lifecycleDir && path.join(parameters.lifecycleDir, 'merkle-indexer.pid'),
  );
};

/**
 * Builds and runs the Merkle proof server the KMS connector reads, apart from the indexer, as the
 * coprocessor chart deploys it. It answers only the tx-senders of the live KMS contexts of the
 * primary host chain's `ProtocolConfig`, which the local connector's wallet is.
 */
export const startMerkleProofServer = async (parameters: {
  readonly databaseUrl: string;
  readonly ethereumRpcUrl: string;
  readonly protocolConfigAddress: string;
  /** The signer of the coprocessor this server answers for: the audience a request is signed for. */
  readonly coprocessorSignerAddress: string;
  readonly logDir: string;
  readonly lifecycleDir?: string;
}): Promise<void> => {
  await replaceSolanaProcess('solana_merkle_proof_server', parameters.lifecycleDir);
  await buildSolanaBinary('solana_merkle_proof_server', MERKLE_PROOF_SERVICE_PACKAGE);
  await spawnSolanaProcess(
    'solana_merkle_proof_server',
    [
      '--database-url',
      parameters.databaseUrl,
      '--http-port',
      String(SOLANA_MERKLE_PROOF_PORT),
      '--ethereum-rpc-url',
      parameters.ethereumRpcUrl,
      '--protocol-config-address',
      parameters.protocolConfigAddress,
      '--coprocessor-signer-address',
      parameters.coprocessorSignerAddress,
    ],
    path.join(parameters.logDir, 'merkle-proof-server.log'),
    parameters.lifecycleDir && path.join(parameters.lifecycleDir, 'merkle-proof-server.pid'),
  );
};

/** Stops a previous `binary`, or refuses to replace one the lifecycle does not own. */
const replaceSolanaProcess = async (binary: string, lifecycleDir: string | undefined): Promise<void> => {
  if (lifecycleDir) {
    if ((await run(['pgrep', '-f', binary], { allowFailure: true })).code === 0) {
      throw new Error(`refusing to replace an unowned ${binary} in lifecycle mode`);
    }
    return;
  }
  await run(['pkill', '-f', binary], { allowFailure: true });
  // Poll it gone rather than sleeping a flat second: the next step binds the same resources.
  await until(async () => (await run(['pgrep', '-f', binary], { allowFailure: true })).code !== 0, {
    description: `previous ${binary} to exit`,
    timeoutMs: 15_000,
    intervalMs: 100,
  });
};

const HOST_LISTENER_PACKAGE = ['-p', 'host-listener'] as const;
const MERKLE_PROOF_SERVICE_PACKAGE = ['-p', 'solana-merkle-proof-service'] as const;

const buildSolanaBinary = async (binary: string, cargoPackage: readonly string[]): Promise<void> => {
  const buildLog = `/tmp/${binary}-build.log`;
  const build = await run(['cargo', 'build', ...cargoPackage, '--bin', binary], {
    cwd: ENGINE_DIR,
    allowFailure: true,
  });
  await Bun.write(buildLog, `${build.stdout}\n${build.stderr}`);
  if (build.code !== 0) {
    throw new Error(`${binary} build failed; see ${buildLog}\n${build.stderr.split('\n').slice(-20).join('\n')}`);
  }
};

const spawnSolanaProcess = async (
  binary: string,
  args: readonly string[],
  logPath: string,
  pidFile: string | undefined,
): Promise<void> => {
  // One shared descriptor for stdout+stderr — the same interleaving `>log 2>&1` produces; the
  // child holds its own duplicate, so the parent copy closes right away.
  const logFd = openSync(logPath, 'a');
  const child = Bun.spawn([path.join(ENGINE_DIR, 'target', 'debug', binary), ...args], {
    stdin: 'ignore',
    stdout: logFd,
    stderr: logFd,
  });
  child.unref();
  closeSync(logFd);
  if (pidFile) await Bun.write(pidFile, `${child.pid}\n`);
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
export const provisionSolanaHostNode = async (state: State): Promise<{ zamaHostId: string }> => {
  const topology = topologyForState(state);
  const lifecycleDir = process.env.DEMO_LIFECYCLE_DIR || undefined;
  if (lifecycleDir && topology.count !== 1) {
    throw new Error(`the demo lifecycle owns one Solana listener; the topology has ${topology.count} coprocessors`);
  }
  const composeProject = lifecycleComposeProject(lifecycleDir);
  const logDir = process.env.SOLANA_LOG_DIR ?? '/tmp';
  const deployerKeypairPath = solanaDeployerKeypairPath();

  // The gateway reads come first: a missing .env.gateway or a down gateway RPC should fail here,
  // not after the multi-minute program build. The resolved values go to the log — on a bootstrap
  // failure or a wrong-signer-set incident this is the record of what was registered.
  console.log('==> [1/4] gather live gateway inputs');
  const gateway = await readGatewayBootstrapInputs({
    gatewayRpcUrl: process.env.GW_RPC ?? LOCAL_SOLANA_ENDPOINTS.gatewayRpc,
  });
  console.log(`    gateway_chain_id=${gateway.gatewayChainId}`);
  console.log(`    input_verification=${evmHex(gateway.inputVerificationContract)}`);
  console.log(`    decryption=${evmHex(gateway.decryptionContract)}`);
  console.log(`    coprocessor_signers=${gateway.coprocessorSigners.map(evmHex).join(',')}`);
  console.log(`    kms_signers=${gateway.kmsSigners.map(evmHex).join(',')}`);

  console.log('==> [2/4] program build, fresh validator (Yellowstone geyser) with the programs at genesis, host bootstrap');
  await seedProgramKeypairs();
  await ensureDeployerWallet(deployerKeypairPath);
  await buildPrograms();
  await startGeyserValidator({
    lifecycleDir,
    ledgerDir: process.env.SOLANA_LEDGER_DIR,
    logDir,
    pluginLibPath: process.env.PLUGIN_LIB || undefined,
    genesisUpgradeablePrograms: genesisDeployedPrograms(deployerKeypairPath),
  });
  await airdropDeployFees(deployerKeypairPath);
  const zamaHostId = await deployPrograms(deployerKeypairPath, gateway, bootstrapThresholdsForState(state));
  // Before any app creates an encrypted store, so the Merkle record holds every store from leaf 0.
  const merkleStartSlot = await finalizedSlot();

  console.log('==> [3/4] register Solana host chain (coprocessor DB + gateway)');
  await registerSolanaHostChain({ zamaHostId, composeProject, coprocessorCount: topology.count });

  console.log(
    `==> [4/4] run ${topology.count} Solana host-listener(s), and coprocessor 0's Merkle indexer and Merkle proof server`,
  );
  const grpcUrl = process.env.GRPC_URL ?? LOCAL_SOLANA_ENDPOINTS.listenerGrpc;
  const listenerStartSlots = await Promise.all(
    Array.from({ length: topology.count }, (_, index) => listenerCheckpointSlot(index)),
  );
  await startHostListeners({ zamaHostId, coprocessorCount: topology.count, grpcUrl, logDir, lifecycleDir });
  await startMerkleDatabase();
  const merkleUrl = merkleDatabaseUrl(await readCoprocessorDatabaseUrl());
  await startMerkleIndexer({
    zamaHostId,
    databaseUrl: merkleUrl,
    startSlot: merkleStartSlot,
    grpcUrl,
    logDir,
    lifecycleDir,
  });
  await startMerkleProofServer({
    databaseUrl: merkleUrl,
    ethereumRpcUrl: LOCAL_SOLANA_ENDPOINTS.hostRpc,
    protocolConfigAddress: await readProtocolConfigAddress(),
    coprocessorSignerAddress: await readCoprocessorSignerAddress(),
    logDir,
    lifecycleDir,
  });
  await waitForListenerCheckpoints(listenerStartSlots);

  console.log(
    `==> Solana side-stack ready. zama_host=${zamaHostId} host_chain_id=${SOLANA_HOST_CHAIN_ID}`,
  );
  return { zamaHostId };
};

if (import.meta.main) {
  await provisionSolanaHostNode(await readStackState());
  // The validator and host-listener are detached children; exit explicitly instead of waiting on
  // anything they hold open.
  process.exit(0);
}
