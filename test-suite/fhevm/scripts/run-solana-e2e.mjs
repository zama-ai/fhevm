#!/usr/bin/env node

import { spawn, spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, '../../..');
const stackModePath = path.join(
  repoRoot,
  'test-suite/fhevm/env/staging/.stack-mode.local',
);
const solanaHostRoot = path.join(repoRoot, 'solana-host-contracts');
const addressesEnvPath = path.join(solanaHostRoot, 'addresses/.env.host');
const solanaExampleEnvPath = path.join(solanaHostRoot, '.env.example');
const listenerManifestPath = path.join(
  repoRoot,
  'coprocessor/fhevm-engine/Cargo.toml',
);
const localCliManifestPath = path.join(
  solanaHostRoot,
  'local-cli/Cargo.toml',
);
const anchorAuthorityKeypairPath = path.join(
  solanaHostRoot,
  'tests/fixtures/anchor-authority.json',
);

const DB_CONTAINER = 'coprocessor-and-kms-db';
const DB_URL = 'postgresql://postgres:postgres@127.0.0.1:5432/coprocessor';
const RELAYER_URL = 'http://127.0.0.1:3000';
const SOLANA_RPC_URL = 'http://127.0.0.1:18999';
const EVM_HOST_RPC_URL = 'http://127.0.0.1:8545';
const SOLANA_E2E_COMMITMENT = 'confirmed';
const SOLANA_LISTENER_HEALTH_PORT = 18085;
const SOLANA_LISTENER_HEALTH_URL = `http://127.0.0.1:${SOLANA_LISTENER_HEALTH_PORT}/healthz`;
const REQUIRED_STACK_CONTAINERS = [
  'host-node',
  'gateway-node',
  'coprocessor-and-kms-db',
  'coprocessor-gw-listener',
  'coprocessor-tfhe-worker',
  'coprocessor-zkproof-worker',
  'coprocessor-sns-worker',
  'coprocessor-transaction-sender',
];
const REQUIRED_SOLANA_STACK_CONTAINERS = [
  'gateway-node',
  'coprocessor-and-kms-db',
  'coprocessor-host-listener',
  'coprocessor-gw-listener',
  'coprocessor-tfhe-worker',
  'coprocessor-zkproof-worker',
  'coprocessor-sns-worker',
  'coprocessor-transaction-sender',
];
const HOST_LISTENER_CONTAINER_PATTERN = /^coprocessor\d*-host-listener(?:-poller)?$/;
const EVM_HOST_NODE_CONTAINER = 'host-node';

const verbose = process.argv.includes('--verbose');
const testType = process.argv
  .slice(2)
  .find((arg) => !arg.startsWith('-'));

if (!testType) {
  printUsage();
  process.exit(1);
}

let localnetChild = null;
let solanaListenerChild = null;
let solanaListenerLogs = '';
let stoppedHostListenerContainers = [];
let stoppedHostNode = false;
let deployedSolanaStackMode = false;

process.on('SIGINT', () => {
  cleanup();
  process.exit(130);
});
process.on('SIGTERM', () => {
  cleanup();
  process.exit(143);
});
process.on('exit', cleanup);

main()
  .then(() => {
    cleanup();
    process.exit(0);
  })
  .catch((error) => {
    console.error(error instanceof Error ? error.message : String(error));
    cleanup();
    process.exit(1);
  });

async function main() {
  const solanaEnv = parseEnvFile(solanaExampleEnvPath);
  const deployedStackMode = readStackMode();
  const deployedSolanaStack = deployedStackMode === 'solana';
  deployedSolanaStackMode = deployedSolanaStack;
  await ensureDockerDbReady();
  if (deployedSolanaStack) {
    await ensureDeployedSolanaStack();
  } else {
    await ensureFullCoprocessorStack();
  }

  if (!deployedSolanaStack || !(await solanaRpcHealthy())) {
    const localnet = await ensureLocalnet(solanaEnv);
    if (localnet.child) {
      localnetChild = localnet.child;
    }
  }

  const addresses = parseEnvFile(addressesEnvPath);
  const chainId = Number(addresses.SOLANA_HOST_CHAIN_ID);
  if (!Number.isFinite(chainId)) {
    throw new Error('invalid SOLANA_HOST_CHAIN_ID in addresses/.env.host');
  }

  await resetChainState(chainId);
  if (!deployedSolanaStack) {
    await startSolanaListener(addresses);
  }

  switch (testType) {
    case 'solana-input-proof':
      await runInputProofCase(addresses, solanaEnv);
      break;
    case 'solana-user-decryption':
      await ensureRelayerReachable();
      await runUserDecryptionCase(addresses, solanaEnv);
      break;
    case 'solana-public-decrypt-http-ebool':
      await runPublicDecryptCase(addresses, solanaEnv, 'scenario-public-ebool');
      break;
    case 'solana-public-decrypt-http-mixed':
      await runPublicDecryptCase(addresses, solanaEnv, 'scenario-public-mixed');
      break;
    case 'solana-erc20':
      await runErc20Case(addresses, solanaEnv);
      break;
    default:
      printUsage();
      throw new Error(`unknown Solana e2e test type: ${testType}`);
  }
}

function printUsage() {
  console.error(`Usage:
  node test-suite/fhevm/scripts/run-solana-e2e.mjs <test-type> [--verbose]

Supported test types:
  solana-input-proof
  solana-user-decryption
  solana-public-decrypt-http-ebool
  solana-public-decrypt-http-mixed
  solana-erc20`);
}

async function runInputProofCase(addresses, solanaEnv) {
  const expectedValue = '18446744073709550042';
  const scenario = runLocalCliScenario('scenario-input-proof', solanaEnv);
  await waitForSolanaTransactionsCommitted(
    scenario.signatures,
    SOLANA_E2E_COMMITMENT,
  );
  await waitForSolanaListenerCaughtUp(
    Number(addresses.SOLANA_HOST_CHAIN_ID),
    await maxCommittedSlot(scenario.signatures, SOLANA_E2E_COMMITMENT),
  );

  const lastCaughtUpBlock = Number(
    sqlScalar(
      `SELECT COALESCE(last_caught_up_block, 0) FROM host_listener_poller_state WHERE chain_id = ${Number(
        addresses.SOLANA_HOST_CHAIN_ID,
      )}`,
    ),
  );
  const computations = Number(
    sqlScalar(
      `SELECT COUNT(*) FROM computations WHERE host_chain_id = ${Number(
        addresses.SOLANA_HOST_CHAIN_ID,
      )}`,
    ),
  );
  const allowedHandles = Number(
    sqlScalar(
      `SELECT COUNT(*) FROM allowed_handles WHERE host_chain_id = ${Number(
        addresses.SOLANA_HOST_CHAIN_ID,
      )}`,
    ),
  );
  const pbsComputations = Number(
    sqlScalar(
      `SELECT COUNT(*) FROM pbs_computations WHERE host_chain_id = ${Number(
        addresses.SOLANA_HOST_CHAIN_ID,
      )}`,
    ),
  );

  if (lastCaughtUpBlock <= 0) {
    throw new Error('solana-input-proof: listener did not update host_listener_poller_state');
  }
  if (computations !== 0) {
    throw new Error(
      `solana-input-proof: expected no computation rows for requestUint64NonTrivial-style flow, got computations=${computations}`,
    );
  }
  if (allowedHandles < 1 || pbsComputations < 1) {
    throw new Error(
      `solana-input-proof: expected durable ACL rows from same-batch Allow after VerifyInput, got allowed_handles=${allowedHandles}, pbs_computations=${pbsComputations}`,
    );
  }

  console.log('Solana input-proof incompatibility check passed');
  console.log(`Signatures: ${scenario.signatures.join(', ')}`);
  console.log(
    `EVM expectation: the requestUint64NonTrivial-style flow accepts the encrypted input for value ${expectedValue} and leaves durable host-side authorization behind it.`,
  );
  console.log(
    'Observed Solana behavior: the flow works at the host level, but the durable downstream signal is still the follow-up ACL event rather than the bare VerifyInput event itself.',
  );
}

async function runUserDecryptionCase(addresses, solanaEnv) {
  const expectedDecryptedValue = 49n;
  const scenario = runLocalCliScenario('scenario-test-input-add42', solanaEnv);
  await waitForSolanaTransactionsCommitted(
    scenario.signatures,
    SOLANA_E2E_COMMITMENT,
  );
  await waitForSolanaListenerCaughtUp(
    Number(addresses.SOLANA_HOST_CHAIN_ID),
    await maxCommittedSlot(scenario.signatures, SOLANA_E2E_COMMITMENT),
  );

  const handle = scenario.final_handles[0];
  if (!handle) {
    throw new Error('solana-user-decryption: scenario did not return a final handle');
  }

  const chainId = Number(addresses.SOLANA_HOST_CHAIN_ID);
  const allowedRows = countAllowedHandleRows(chainId, handle);
  const publicDecryptRows = countPublicDecryptAllowRows(chainId, handle);
  const pbsRows = countPbsRows(chainId, handle);
  if (allowedRows < 2) {
    throw new Error(
      `solana-user-decryption: expected host-side app + user allow rows for handle ${handle}, got ${allowedRows}`,
    );
  }
  if (publicDecryptRows < 1 || pbsRows < 1) {
    throw new Error(
      `solana-user-decryption: expected the add42 flow to mark the result decryptable on the Solana host, got public_allow_rows=${publicDecryptRows}, pbs_rows=${pbsRows}`,
    );
  }

  const body = {
    handleContractPairs: [
      {
        handle,
        contractAddress: addresses.SOLANA_TEST_INPUT_STATE_PDA,
      },
    ],
    requestValidity: {
      startTimestamp: '1700000000',
      durationDays: '1',
    },
    contractsChainId: addresses.SOLANA_HOST_CHAIN_ID,
    contractAddresses: [addresses.SOLANA_TEST_INPUT_STATE_PDA],
    userAddress: addresses.SOLANA_HOST_AUTHORITY,
    signature: '11'.repeat(65),
    publicKey: 'aa',
    extraData: '0x00',
  };

  const response = await fetch(`${RELAYER_URL}/v2/user-decrypt`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(body),
  });
  const responseText = await response.text();

  if (response.ok) {
    throw new Error(
      `solana-user-decryption: expected relayer validation to reject Solana addresses, got ${response.status} ${responseText}`,
    );
  }

  const normalized = responseText.toLowerCase();
  if (
    !normalized.includes('must start with 0x') &&
    !normalized.includes('42 characters long')
  ) {
    throw new Error(
      `solana-user-decryption: expected Ethereum-address validation failure, got ${response.status} ${responseText}`,
    );
  }

  console.log('Solana user-decryption incompatibility check passed');
  console.log(`Handle: ${handle}`);
  console.log(
    `EVM expectation: the add42ToInput64-style flow would user-decrypt this handle to ${expectedDecryptedValue}.`,
  );
  console.log(
    'Observed incompatibility: the Solana host emitted the expected durable allow rows, but the relayer still validates contract/user identifiers as Ethereum 20-byte 0x addresses, so the request is rejected before ACL/KMS checks.',
  );
}

async function runPublicDecryptCase(addresses, solanaEnv, scenarioName) {
  await ensureRelayerReachable();
  const scenario = runLocalCliScenario(scenarioName, solanaEnv);
  await waitForSolanaTransactionsCommitted(
    scenario.signatures,
    SOLANA_E2E_COMMITMENT,
  );
  await waitForSolanaListenerCaughtUp(
    Number(addresses.SOLANA_HOST_CHAIN_ID),
    await maxCommittedSlot(scenario.signatures, SOLANA_E2E_COMMITMENT),
  );
  const handles = Array.isArray(scenario.final_handles) ? scenario.final_handles : [];
  if (handles.length === 0) {
    throw new Error(`${scenarioName}: scenario did not return final handles`);
  }

  const chainId = Number(addresses.SOLANA_HOST_CHAIN_ID);
  for (const handle of handles) {
    const publicAllowRows = countPublicDecryptAllowRows(chainId, handle);
    const pbsRows = countPbsRows(chainId, handle);
    if (publicAllowRows < 1 || pbsRows < 1) {
      throw new Error(
        `${scenarioName}: expected handle ${handle} to be publicly decryptable on the Solana host, got public_allow_rows=${publicAllowRows}, pbs_rows=${pbsRows}`,
      );
    }
  }

  const postResponse = await fetch(`${RELAYER_URL}/v2/public-decrypt`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({
      ciphertextHandles: handles,
      extraData: '0x00',
    }),
  });
  const postText = await postResponse.text();

  if (!postResponse.ok) {
    if (isKnownSolanaPublicDecryptIncompatibility(postText)) {
      console.log(`Solana ${scenarioName} incompatibility check passed`);
      console.log(`EVM expectation: ${expectedPublicDecryptOutcome(scenarioName)}`);
      console.log(
        `Observed incompatibility: ${normalizePublicDecryptMessage(postText)}`,
      );
      return;
    }
    throw new Error(
      `${scenarioName}: unexpected public decrypt POST failure ${postResponse.status} ${postText}`,
    );
  }

  const postPayload = JSON.parse(postText);
  const jobId = postPayload?.result?.jobId;
  if (!jobId) {
    throw new Error(`${scenarioName}: public decrypt response did not return a jobId`);
  }

  const statusPayload = await waitForPublicDecryptJob(jobId);
  if (statusPayload.status === 'succeeded') {
    console.log(`Solana ${scenarioName} end-to-end public decrypt succeeded`);
    console.log(`jobId=${jobId}`);
    return;
  }

  const errorMessage = JSON.stringify(statusPayload.error ?? statusPayload);
  if (isKnownSolanaPublicDecryptIncompatibility(errorMessage)) {
    console.log(`Solana ${scenarioName} incompatibility check passed`);
    console.log(`EVM expectation: ${expectedPublicDecryptOutcome(scenarioName)}`);
    console.log(
      `Observed incompatibility: ${normalizePublicDecryptMessage(errorMessage)}`,
    );
    return;
  }

  throw new Error(
    `${scenarioName}: public decrypt job failed unexpectedly: ${JSON.stringify(statusPayload)}`,
  );
}

async function runErc20Case(addresses, solanaEnv) {
  const mintExpectation = 'after mint(10000), Alice balance decrypts to 10000';
  const transferExpectation =
    'after transfer(1337), Alice balance decrypts to 8663 and Bob balance decrypts to 1337';
  let scenario;
  try {
    scenario = runLocalCliScenario('scenario-erc20', solanaEnv);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    if (isKnownSolanaErc20RuntimeIncompatibility(message)) {
      console.log('Solana ERC20 incompatibility check passed');
      console.log(`EVM expectation 1: ${mintExpectation}.`);
      console.log(`EVM expectation 2: ${transferExpectation}.`);
      console.log(
        `Observed incompatibility: ${summarizeSolanaErc20RuntimeIncompatibility(message)}`,
      );
      return;
    }
    throw error;
  }
  await waitForSolanaTransactionsCommitted(
    scenario.signatures,
    SOLANA_E2E_COMMITMENT,
  );
  await waitForSolanaListenerCaughtUp(
    Number(addresses.SOLANA_HOST_CHAIN_ID),
    await maxCommittedSlot(scenario.signatures, SOLANA_E2E_COMMITMENT),
  );

  const chainId = Number(addresses.SOLANA_HOST_CHAIN_ID);
  const mintBalanceHandle = requiredScenarioField(
    scenario,
    'mint_balance_handle',
    'solana-erc20',
  );
  const transferInputHandle = requiredScenarioField(
    scenario,
    'transfer_input_handle',
    'solana-erc20',
  );
  const aliceAfterTransferHandle = requiredScenarioField(
    scenario,
    'alice_balance_handle_after_transfer',
    'solana-erc20',
  );
  const bobAfterTransferHandle = requiredScenarioField(
    scenario,
    'bob_balance_handle_after_transfer',
    'solana-erc20',
  );

  await waitForComputationRow(chainId, mintBalanceHandle, 'solana-erc20 mint');
  await waitForCiphertextPresent(mintBalanceHandle, 'solana-erc20 mint');

  const mintState = queryComputationState(chainId, mintBalanceHandle);
  const mintAllowRows = countAllowedHandleRows(chainId, mintBalanceHandle);
  const mintPbsRows = countPbsRows(chainId, mintBalanceHandle);

  if (!mintState.exists || !mintState.isCompleted || mintState.isError) {
    throw new Error(
      `solana-erc20: expected mint handle to be fully computed, got ${formatComputationState(
        mintState,
      )}`,
    );
  }
  if (mintAllowRows < 2) {
    throw new Error(
      `solana-erc20: expected durable app + owner allow rows for mint balance handle, got ${mintAllowRows}`,
    );
  }
  if (mintPbsRows < 1) {
    throw new Error(
      `solana-erc20: expected at least one PBS row for mint balance handle, got ${mintPbsRows}`,
    );
  }

  await waitForComputationRow(
    chainId,
    aliceAfterTransferHandle,
    'solana-erc20 transfer(alice)',
  );
  await waitForComputationRow(
    chainId,
    bobAfterTransferHandle,
    'solana-erc20 transfer(bob)',
  );

  await sleep(10_000);

  const computationsUsingInput = countComputationsDependingOnHandle(
    chainId,
    transferInputHandle,
  );
  const transferInputCiphertexts = countCiphertextRows(transferInputHandle);
  const verifyProofRows = countVerifyProofRows(chainId);
  const aliceAfterState = queryComputationState(chainId, aliceAfterTransferHandle);
  const bobAfterState = queryComputationState(chainId, bobAfterTransferHandle);

  if (computationsUsingInput < 1) {
    throw new Error(
      'solana-erc20: expected at least one computation row to depend on the transfer input handle',
    );
  }
  if (transferInputCiphertexts !== 0) {
    throw new Error(
      `solana-erc20: expected no ciphertext row for Solana transfer input handle, got ${transferInputCiphertexts}`,
    );
  }
  if (verifyProofRows !== 0) {
    throw new Error(
      `solana-erc20: expected no verify_proofs rows for Solana host-chain input flow, got ${verifyProofRows}`,
    );
  }
  if (!aliceAfterState.exists || !bobAfterState.exists) {
    throw new Error(
      `solana-erc20: expected transfer output handles to be ingested, got alice=${formatComputationState(
        aliceAfterState,
      )} bob=${formatComputationState(bobAfterState)}`,
    );
  }
  if (aliceAfterState.isCompleted || bobAfterState.isCompleted) {
    throw new Error(
      `solana-erc20: expected transfer outputs to remain blocked on missing Solana input ciphertext, got alice=${formatComputationState(
        aliceAfterState,
      )} bob=${formatComputationState(bobAfterState)}`,
    );
  }

  console.log('Solana ERC20 incompatibility check passed');
  console.log(`Signatures: ${scenario.signatures.join(', ')}`);
  console.log(`Mint handle: ${mintBalanceHandle}`);
  console.log(`Transfer input handle: ${transferInputHandle}`);
  console.log(
    `EVM expectation 1: ${mintExpectation}.`,
  );
  console.log(
    'Observed Solana behavior: mint works end to end through the Solana host program, Solana host-listener, DB, and TFHE worker.',
  );
  console.log(
    `EVM expectation 2: ${transferExpectation}.`,
  );
  console.log(
    'Observed incompatibility: transfer-style external inputs still do not produce verify_proofs work or ciphertext material for the selected input handle, so the transfer output computations are ingested but remain blocked downstream.',
  );
}

function runLocalCliScenario(scenarioName, solanaEnv) {
  const args = [
    'run',
    '--manifest-path',
    localCliManifestPath,
    '--',
    scenarioName,
    '--addresses-env',
    addressesEnvPath,
    '--payer-keypair',
    anchorAuthorityKeypairPath,
  ];
  const result = runCommand('cargo', args, {
    cwd: repoRoot,
    env: { ...process.env, ...solanaEnv },
    capture: true,
  });
  try {
    return JSON.parse(result.stdout.trim());
  } catch (error) {
    throw new Error(
      `failed to parse local-cli JSON for ${scenarioName}: ${error}\n${result.stdout}`,
    );
  }
}

async function ensureLocalnet(solanaEnv) {
  if (await solanaRpcHealthy()) {
    runZsh(`make -C ${shellQuote(solanaHostRoot)} localnet-bootstrap`, {
      cwd: repoRoot,
      env: { ...process.env, ...solanaEnv },
      capture: !verbose,
    });
    await waitForAddresses();
    return { child: null };
  }

  const child = spawn(
    '/bin/zsh',
    ['-lic', `make -C ${shellQuote(solanaHostRoot)} localnet`],
    {
      cwd: repoRoot,
      env: { ...process.env, ...solanaEnv },
      stdio: verbose ? 'inherit' : 'ignore',
    },
  );

  const started = Date.now();
  let observedExitCode = null;
  while (Date.now() - started < 180_000) {
    if (child.exitCode !== null && observedExitCode === null) {
      observedExitCode = child.exitCode;
    }
    if (await solanaRpcHealthy()) {
      runZsh(`make -C ${shellQuote(solanaHostRoot)} localnet-bootstrap`, {
        cwd: repoRoot,
        env: { ...process.env, ...solanaEnv },
        capture: !verbose,
      });
      await waitForAddresses();
      return { child: child.exitCode === null ? child : null };
    }
    if (observedExitCode !== null && Date.now() - started > 20_000) {
      throw new Error(`solana localnet exited early with code ${observedExitCode}`);
    }
    await sleep(2000);
  }

  child.kill('SIGTERM');
  throw new Error('timed out waiting for Solana localnet to become healthy');
}

async function ensureFullCoprocessorStack() {
  for (const container of REQUIRED_STACK_CONTAINERS) {
    if (!dockerContainerRunning(container)) {
      throw new Error(
        `required stack container ${container} is not running; deploy the full stack first with ./test-suite/fhevm/fhevm-cli deploy --local`,
      );
    }
  }
}

async function ensureDeployedSolanaStack() {
  for (const container of REQUIRED_SOLANA_STACK_CONTAINERS) {
    if (!dockerContainerRunning(container)) {
      throw new Error(
        `required Solana stack container ${container} is not running; deploy the Solana stack first with ./test-suite/fhevm/fhevm-cli deploy --local --solana`,
      );
    }
  }
  if (!(await solanaRpcHealthy()) && verbose) {
    console.error(
      `Solana RPC ${SOLANA_RPC_URL} is not healthy; the test runner will bootstrap a validator for this test run.`,
    );
  }
}

async function startSolanaListener(addresses) {
  stopEvmHostNode();
  stoppedHostListenerContainers = stopRunningHostListeners();

  const args = [
    'run',
    '--manifest-path',
    listenerManifestPath,
    '-p',
    'solana-host-listener',
    '--bin',
    'solana_host_listener_poller',
    '--',
    '--addresses-env',
    addressesEnvPath,
    '--database-url',
    DB_URL,
    '--batch-size-slots',
    '512',
    '--poll-interval-ms',
    '500',
    '--retry-interval-ms',
    '1000',
    '--health-port',
    String(SOLANA_LISTENER_HEALTH_PORT),
    '--commitment',
    SOLANA_E2E_COMMITMENT,
  ];

  solanaListenerLogs = '';
  const child = spawn('cargo', args, {
    cwd: repoRoot,
    env: process.env,
    stdio: verbose ? ['ignore', 'inherit', 'inherit'] : ['ignore', 'pipe', 'pipe'],
  });
  solanaListenerChild = child;

  if (!verbose) {
    child.stdout?.on('data', (chunk) => {
      solanaListenerLogs += chunk.toString();
    });
    child.stderr?.on('data', (chunk) => {
      solanaListenerLogs += chunk.toString();
    });
  }

  const started = Date.now();
  while (Date.now() - started < 180_000) {
    if (child.exitCode !== null) {
      throw new Error(
        `solana host listener exited early with code ${child.exitCode}\n${solanaListenerLogs.trim()}`,
      );
    }
    if (await solanaListenerHealthy()) {
      return;
    }
    await sleep(1000);
  }

  child.kill('SIGTERM');
  solanaListenerChild = null;
  throw new Error('timed out waiting for solana host listener to become healthy');
}

async function waitForSolanaListenerCaughtUp(chainId, minimumSlot) {
  const started = Date.now();
  while (Date.now() - started < 120_000) {
    ensureSolanaListenerAlive();
    const current = Number(
      sqlScalar(
        `SELECT COALESCE(last_caught_up_block, 0) FROM host_listener_poller_state WHERE chain_id = ${chainId}`,
      ),
    );
    if (current >= minimumSlot) {
      return;
    }
    await sleep(1000);
  }
  throw new Error(
    `timed out waiting for Solana host listener to catch up to slot ${minimumSlot}`,
  );
}

function stopRunningHostListeners() {
  const names = dockerContainerNames().filter((name) =>
    HOST_LISTENER_CONTAINER_PATTERN.test(name),
  );
  const running = names.filter((name) => dockerContainerRunning(name));
  for (const name of running) {
    runCommand('docker', ['stop', name], {
      cwd: repoRoot,
      capture: !verbose,
    });
  }
  return running;
}

function restartStoppedHostListeners() {
  for (const name of stoppedHostListenerContainers) {
    runCommand('docker', ['start', name], {
      cwd: repoRoot,
      capture: !verbose,
    });
  }
  stoppedHostListenerContainers = [];
}

function stopEvmHostNode() {
  if (!dockerContainerRunning(EVM_HOST_NODE_CONTAINER)) {
    stoppedHostNode = false;
    return;
  }
  runCommand('docker', ['stop', EVM_HOST_NODE_CONTAINER], {
    cwd: repoRoot,
    capture: !verbose,
  });
  stoppedHostNode = true;
}

function restartEvmHostNode() {
  if (!stoppedHostNode) {
    return;
  }
  runCommand('docker', ['start', EVM_HOST_NODE_CONTAINER], {
    cwd: repoRoot,
    capture: !verbose,
  });
  waitForEvmHostNodeReady();
  stoppedHostNode = false;
}

function dockerContainerNames() {
  return runCommand('docker', ['ps', '-a', '--format', '{{.Names}}'], {
    cwd: repoRoot,
    capture: true,
  })
    .stdout.split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);
}

function dockerContainerRunning(name) {
  const result = runCommandAllowFailure(
    'docker',
    ['inspect', '-f', '{{.State.Running}}', name],
    {
      cwd: repoRoot,
      capture: true,
      allowSuccess: true,
    },
  );
  if (result.status !== 0) {
    return false;
  }
  return result.stdout.trim() === 'true';
}

async function solanaListenerHealthy() {
  try {
    const response = await fetch(SOLANA_LISTENER_HEALTH_URL);
    return response.ok;
  } catch {
    return false;
  }
}

function ensureSolanaListenerAlive() {
  if (deployedSolanaStackMode) {
    if (dockerContainerRunning('coprocessor-host-listener')) {
      return;
    }
    const logs = runCommandAllowFailure(
      'docker',
      ['logs', '--tail', '120', 'coprocessor-host-listener'],
      {
        cwd: repoRoot,
        capture: true,
        allowSuccess: true,
      },
    );
    const logOutput =
      logs.status === 0 ? logs.stdout.trim() || logs.stderr.trim() : '';
    throw new Error(
      `deployed solana host listener is not running${logOutput ? `\n${logOutput}` : ''}`,
    );
  }
  if (solanaListenerChild && solanaListenerChild.exitCode === null) {
    return;
  }
  throw new Error(
    `solana host listener exited unexpectedly${solanaListenerLogs ? `\n${solanaListenerLogs.trim()}` : ''}`,
  );
}

async function maxCommittedSlot(signatures, commitment) {
  let maxSlot = 0;
  for (const signature of signatures) {
    const transaction = await solanaRpcCall('getTransaction', [
      signature,
      {
        commitment,
        encoding: 'json',
        maxSupportedTransactionVersion: 0,
      },
    ]);
    const slot = Number(transaction?.slot ?? 0);
    if (!Number.isFinite(slot) || slot <= 0) {
      throw new Error(`failed to resolve committed slot for Solana signature ${signature}`);
    }
    maxSlot = Math.max(maxSlot, slot);
  }
  return maxSlot;
}

async function waitForSolanaTransactionsCommitted(signatures, minimumCommitment) {
  if (!Array.isArray(signatures) || signatures.length === 0) {
    return;
  }

  const pending = new Set(signatures);
  const started = Date.now();

  while (pending.size > 0) {
    if (Date.now() - started > 120_000) {
      throw new Error(
        `timed out waiting for Solana signatures to finalize: ${Array.from(pending).join(', ')}`,
      );
    }

    const pendingSignatures = Array.from(pending);
    const statuses = await solanaRpcCall('getSignatureStatuses', [
      pendingSignatures,
      { searchTransactionHistory: true },
    ]);
    const values = Array.isArray(statuses?.value) ? statuses.value : [];

    for (let index = 0; index < values.length; index += 1) {
      const status = values[index];
      const signature = pendingSignatures[index];
      if (!signature || !status) {
        continue;
      }
      if (status.err) {
        throw new Error(
          `Solana transaction ${signature} failed before listener ingestion: ${JSON.stringify(status.err)}`,
        );
      }
      if (commitmentSatisfied(status, minimumCommitment)) {
        pending.delete(signature);
      }
    }

    if (pending.size > 0) {
      await sleep(1000);
    }
  }
}

function commitmentSatisfied(status, minimumCommitment) {
  const observed = status.confirmationStatus;
  if (minimumCommitment === 'processed') {
    return observed === 'processed' || observed === 'confirmed' || observed === 'finalized';
  }
  if (minimumCommitment === 'confirmed') {
    return observed === 'confirmed' || observed === 'finalized';
  }
  if (minimumCommitment === 'finalized') {
    return observed === 'finalized' || (observed == null && status.confirmations == null);
  }
  return false;
}

async function ensureDockerDbReady() {
  sqlScalar('SELECT 1');
}

async function ensureRelayerReachable() {
  try {
    await fetch(`${RELAYER_URL}/v2/public-decrypt/not-a-uuid`);
  } catch (error) {
    throw new Error(
      `relayer is not reachable on ${RELAYER_URL}; deploy the stack first with ./fhevm-cli deploy --local (${error})`,
    );
  }
}

async function waitForPublicDecryptJob(jobId) {
  const started = Date.now();
  while (Date.now() - started < 120_000) {
    const response = await fetch(`${RELAYER_URL}/v2/public-decrypt/${jobId}`);
    const text = await response.text();
    if (!response.ok) {
      throw new Error(`public decrypt status check failed for ${jobId}: ${response.status} ${text}`);
    }
    const payload = JSON.parse(text);
    if (payload.status === 'queued') {
      await sleep(1000);
      continue;
    }
    return payload;
  }
  throw new Error(`timed out waiting for public decrypt job ${jobId}`);
}

function isKnownSolanaPublicDecryptIncompatibility(message) {
  const normalized = String(message).toLowerCase();
  return (
    normalized.includes('host chain id') ||
    normalized.includes('not supported') ||
    normalized.includes('unsupported chain') ||
    normalized.includes('no acl binding configured') ||
    normalized.includes('host acl failed') ||
    normalized.includes('not allowed on host acl') ||
    normalized.includes('call failed')
  );
}

function normalizePublicDecryptMessage(message) {
  return String(message).replace(/\s+/g, ' ').trim();
}

function isKnownSolanaErc20RuntimeIncompatibility(message) {
  const normalized = String(message).toLowerCase();
  return (
    normalized.includes('memory allocation failed') ||
    normalized.includes('out of memory') ||
    normalized.includes('program failed to complete') ||
    normalized.includes('sbf program panicked')
  );
}

function summarizeSolanaErc20RuntimeIncompatibility(message) {
  const normalized = String(message).toLowerCase();
  if (
    normalized.includes('memory allocation failed') ||
    normalized.includes('out of memory')
  ) {
    return 'the current Solana EncryptedERC20 transfer path runs out of program memory during the host CPI sequence, so the test cannot yet reach the downstream proof-materialization parity check';
  }
  if (
    normalized.includes('program failed to complete') ||
    normalized.includes('sbf program panicked')
  ) {
    return 'the current Solana EncryptedERC20 transfer path aborts inside the program before the EVM-style balance assertions can be checked';
  }
  return normalizePublicDecryptMessage(message);
}

async function clearChainState(chainId) {
  sqlExec(`
    DELETE FROM verify_proofs WHERE chain_id = ${chainId};
    DELETE FROM pbs_computations WHERE host_chain_id = ${chainId};
    DELETE FROM allowed_handles WHERE host_chain_id = ${chainId};
    DELETE FROM delegate_user_decrypt WHERE host_chain_id = ${chainId};
    DELETE FROM host_chain_blocks_valid WHERE chain_id = ${chainId};
    DELETE FROM host_listener_poller_state WHERE chain_id = ${chainId};
    DELETE FROM ciphertext_digest WHERE host_chain_id = ${chainId};
    DELETE FROM ciphertexts WHERE handle IN (SELECT output_handle FROM computations WHERE host_chain_id = ${chainId});
    DELETE FROM ciphertexts128 WHERE handle IN (SELECT output_handle FROM computations WHERE host_chain_id = ${chainId});
    DELETE FROM computations WHERE host_chain_id = ${chainId};
  `);
}

async function resetChainState(chainId) {
  const baselineSlot = Number(
    await solanaRpcCall('getSlot', [{ commitment: SOLANA_E2E_COMMITMENT }]),
  );
  if (!Number.isFinite(baselineSlot) || baselineSlot < 0) {
    throw new Error(`failed to resolve Solana baseline slot: ${baselineSlot}`);
  }

  await clearChainState(chainId);
  sqlExec(`
    INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block)
    VALUES (${chainId}, ${baselineSlot})
    ON CONFLICT (chain_id) DO UPDATE
    SET last_caught_up_block = EXCLUDED.last_caught_up_block,
        updated_at = NOW();
  `);

  const started = Date.now();
  while (Date.now() - started < 30_000) {
    const slot = Number(
      await solanaRpcCall('getSlot', [{ commitment: SOLANA_E2E_COMMITMENT }]),
    );
    if (slot > baselineSlot) {
      return;
    }
    await sleep(500);
  }

  throw new Error(`timed out waiting for Solana slot to advance past baseline ${baselineSlot}`);
}

function sqlScalar(query) {
  return sqlExec(query).stdout.trim();
}

function sqlExec(query) {
  return runCommand(
    'docker',
    [
      'exec',
      '-e',
      'PGPASSWORD=postgres',
      DB_CONTAINER,
      'psql',
      '-U',
      'postgres',
      '-d',
      'coprocessor',
      '-t',
      '-A',
      '-v',
      'ON_ERROR_STOP=1',
      '-c',
      oneLineSql(query),
    ],
    {
      cwd: repoRoot,
      capture: true,
    },
  );
}

async function solanaRpcHealthy() {
  try {
    await solanaRpcCall('getHealth', []);
    return true;
  } catch {
    return false;
  }
}

async function solanaRpcCall(method, params = []) {
  const response = await fetch(SOLANA_RPC_URL, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({
      jsonrpc: '2.0',
      id: 1,
      method,
      params,
    }),
  });

  if (!response.ok) {
    throw new Error(`Solana RPC ${method} failed with HTTP ${response.status}`);
  }

  const payload = await response.json();
  if (payload.error) {
    throw new Error(`Solana RPC ${method} returned error: ${JSON.stringify(payload.error)}`);
  }

  return payload.result;
}

async function waitForAddresses() {
  const started = Date.now();
  while (Date.now() - started < 120_000) {
    if (fs.existsSync(addressesEnvPath)) {
      const parsed = parseEnvFile(addressesEnvPath);
      if (parsed.SOLANA_HOST_RPC_URL) {
        return;
      }
    }
    await sleep(1000);
  }
  throw new Error(`timed out waiting for ${addressesEnvPath}`);
}

function parseEnvFile(filePath) {
  const contents = fs.readFileSync(filePath, 'utf8');
  const values = {};
  for (const rawLine of contents.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (!line || line.startsWith('#')) {
      continue;
    }
    const separatorIndex = line.indexOf('=');
    if (separatorIndex === -1) {
      continue;
    }
    const key = line.slice(0, separatorIndex).trim();
    const value = line.slice(separatorIndex + 1).trim().replace(/^"|"$/g, '');
    values[key] = value;
  }
  return values;
}

function requiredScenarioField(scenario, field, label) {
  const value = scenario?.[field];
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`${label}: scenario did not return ${field}`);
  }
  return value;
}

async function waitForComputationRow(chainId, handle, label) {
  await waitForCondition(
    () => computationExists(chainId, handle),
    60_000,
    `${label}: computation row for ${handle}`,
  );
}

async function waitForCiphertextPresent(handle, label) {
  await waitForCondition(
    () => countCiphertextRows(handle) > 0,
    120_000,
    `${label}: ciphertext row for ${handle}`,
  );
}

async function waitForCondition(check, timeoutMs, description) {
  const started = Date.now();
  while (Date.now() - started < timeoutMs) {
    if (check()) {
      return;
    }
    await sleep(1000);
  }
  throw new Error(`timed out waiting for ${description}`);
}

function computationExists(chainId, handle) {
  return (
    Number(
      sqlScalar(`
        SELECT COUNT(*)
        FROM computations
        WHERE host_chain_id = ${chainId}
          AND output_handle = decode('${stripHexPrefix(handle)}', 'hex')
      `),
    ) > 0
  );
}

function queryComputationState(chainId, handle) {
  const exists = computationExists(chainId, handle);
  if (!exists) {
    return {
      exists: false,
      isCompleted: false,
      isAllowed: false,
      isError: false,
      errorMessage: '',
    };
  }

  return {
    exists: true,
    isCompleted:
      Number(
        sqlScalar(`
          SELECT COALESCE(is_completed::int, 0)
          FROM computations
          WHERE host_chain_id = ${chainId}
            AND output_handle = decode('${stripHexPrefix(handle)}', 'hex')
          LIMIT 1
        `),
      ) === 1,
    isAllowed:
      Number(
        sqlScalar(`
          SELECT COALESCE(is_allowed::int, 0)
          FROM computations
          WHERE host_chain_id = ${chainId}
            AND output_handle = decode('${stripHexPrefix(handle)}', 'hex')
          LIMIT 1
        `),
      ) === 1,
    isError:
      Number(
        sqlScalar(`
          SELECT COALESCE(is_error::int, 0)
          FROM computations
          WHERE host_chain_id = ${chainId}
            AND output_handle = decode('${stripHexPrefix(handle)}', 'hex')
          LIMIT 1
        `),
      ) === 1,
    errorMessage: sqlScalar(`
      SELECT COALESCE(error_message, '')
      FROM computations
      WHERE host_chain_id = ${chainId}
        AND output_handle = decode('${stripHexPrefix(handle)}', 'hex')
      LIMIT 1
    `),
  };
}

function formatComputationState(state) {
  if (!state.exists) {
    return 'missing';
  }
  return `exists(completed=${state.isCompleted}, allowed=${state.isAllowed}, error=${state.isError}, message=${JSON.stringify(
    state.errorMessage,
  )})`;
}

function countAllowedHandleRows(chainId, handle) {
  return Number(
    sqlScalar(`
      SELECT COUNT(*)
      FROM allowed_handles
      WHERE host_chain_id = ${chainId}
        AND handle = decode('${stripHexPrefix(handle)}', 'hex')
    `),
  );
}

function countPublicDecryptAllowRows(chainId, handle) {
  return Number(
    sqlScalar(`
      SELECT COUNT(*)
      FROM allowed_handles
      WHERE host_chain_id = ${chainId}
        AND handle = decode('${stripHexPrefix(handle)}', 'hex')
        AND event_type = 1
    `),
  );
}

function countPbsRows(chainId, handle) {
  return Number(
    sqlScalar(`
      SELECT COUNT(*)
      FROM pbs_computations
      WHERE host_chain_id = ${chainId}
        AND handle = decode('${stripHexPrefix(handle)}', 'hex')
    `),
  );
}

function countCiphertextRows(handle) {
  return Number(
    sqlScalar(`
      SELECT COUNT(*)
      FROM ciphertexts
      WHERE handle = decode('${stripHexPrefix(handle)}', 'hex')
    `),
  );
}

function countVerifyProofRows(chainId) {
  return Number(
    sqlScalar(`
      SELECT COUNT(*)
      FROM verify_proofs
      WHERE chain_id = ${chainId}
    `),
  );
}

function countComputationsDependingOnHandle(chainId, handle) {
  return Number(
    sqlScalar(`
      SELECT COUNT(*)
      FROM computations
      WHERE host_chain_id = ${chainId}
        AND EXISTS (
          SELECT 1
          FROM unnest(dependencies) AS dependency
          WHERE dependency = decode('${stripHexPrefix(handle)}', 'hex')
        )
    `),
  );
}

function stripHexPrefix(value) {
  return String(value).replace(/^0x/i, '');
}

function expectedPublicDecryptOutcome(scenarioName) {
  if (scenarioName === 'scenario-public-ebool') {
    return 'public decrypt returns { handle: true }';
  }
  if (scenarioName === 'scenario-public-mixed') {
    return "public decrypt returns { bool: true, uint32: 242, address: '0xfC4382C084fCA3f4fB07c3BCDA906C01797595a8' }";
  }
  return 'public decrypt returns the same clear values as the EVM test';
}

function readStackMode() {
  if (!fs.existsSync(stackModePath)) {
    return 'evm';
  }
  const parsed = parseEnvFile(stackModePath);
  return parsed.HOST_MODE ?? 'evm';
}

function runZsh(command, options = {}) {
  return runCommand('/bin/zsh', ['-lic', command], options);
}

function runCommand(command, args, options = {}) {
  if (verbose) {
    console.error(`$ ${command} ${args.join(' ')}`);
  }

  const result = spawnSync(command, args, {
    cwd: options.cwd ?? repoRoot,
    env: options.env ?? process.env,
    encoding: 'utf8',
    stdio: options.capture === false ? 'inherit' : ['ignore', 'pipe', 'pipe'],
  });

  if (result.status !== 0) {
    const stderr = (result.stderr ?? '').trim();
    const stdout = (result.stdout ?? '').trim();
    throw new Error(
      `command failed (${command} ${args.join(' ')}):\n${stdout}${stdout && stderr ? '\n' : ''}${stderr}`,
    );
  }

  return {
    stdout: result.stdout ?? '',
    stderr: result.stderr ?? '',
  };
}

function runCommandAllowFailure(command, args, options = {}) {
  if (verbose) {
    console.error(`$ ${command} ${args.join(' ')}`);
  }

  const result = spawnSync(command, args, {
    cwd: options.cwd ?? repoRoot,
    env: options.env ?? process.env,
    encoding: 'utf8',
    stdio: options.capture === false ? 'inherit' : ['ignore', 'pipe', 'pipe'],
  });

  if (result.status === 0 && !options.allowSuccess) {
    throw new Error(`command unexpectedly succeeded (${command} ${args.join(' ')})`);
  }

  return {
    status: result.status,
    stdout: result.stdout ?? '',
    stderr: result.stderr ?? '',
  };
}

function oneLineSql(query) {
  return query
    .split('\n')
    .map((line) => line.trim())
    .filter(Boolean)
    .join(' ');
}

function shellQuote(value) {
  return `'${String(value).replace(/'/g, `'\"'\"'`)}'`;
}

function cleanup() {
  if (solanaListenerChild && solanaListenerChild.exitCode === null) {
    solanaListenerChild.kill('SIGTERM');
  }
  solanaListenerChild = null;
  try {
    restartEvmHostNode();
  } catch (error) {
    console.error(
      `failed to restart original host-node container: ${error instanceof Error ? error.message : String(error)}`,
    );
  }
  if (stoppedHostListenerContainers.length > 0) {
    try {
      restartStoppedHostListeners();
    } catch (error) {
      console.error(
        `failed to restart original host-listener containers: ${error instanceof Error ? error.message : String(error)}`,
      );
    }
  }
  if (localnetChild && localnetChild.exitCode === null) {
    localnetChild.kill('SIGTERM');
  }
  localnetChild = null;
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function waitForEvmHostNodeReady() {
  const started = Date.now();
  while (Date.now() - started < 30_000) {
    const result = spawnSync(
      'curl',
      [
        '-sS',
        '-H',
        'content-type: application/json',
        '-d',
        '{"jsonrpc":"2.0","id":1,"method":"eth_chainId","params":[]}',
        EVM_HOST_RPC_URL,
      ],
      {
        cwd: repoRoot,
        env: process.env,
        encoding: 'utf8',
        stdio: ['ignore', 'pipe', 'pipe'],
      },
    );
    if (result.status === 0) {
      try {
        const payload = JSON.parse(result.stdout || '{}');
        if (payload.result) {
          return;
        }
      } catch {
        // Retry until the node returns valid JSON-RPC.
      }
    }
    spawnSync('sleep', ['1'], {
      cwd: repoRoot,
      env: process.env,
      stdio: 'ignore',
    });
  }
  throw new Error('timed out waiting for host-node JSON-RPC to become ready');
}
