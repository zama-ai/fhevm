#!/usr/bin/env bun

// @ts-ignore: resolved by Bun at runtime; @types/node resolves this after `bun install`
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import process from 'node:process';

const repoRoot = path.resolve(import.meta.dir, '../../../../');
const requireFromLibrary = createRequire(
  path.join(repoRoot, 'library-solidity/package.json'),
);
const requireFromGatewayContracts = createRequire(
  path.join(repoRoot, 'gateway-contracts/package.json'),
);
const TFHE = requireFromLibrary('node-tfhe');
const { ethers } = requireFromLibrary('ethers');
const createHash = requireFromLibrary('keccak');
const { ed25519 } = requireFromGatewayContracts('@noble/curves/ed25519');
const {
  generateKeypair: generateRelayerKeypair,
} = requireFromLibrary('@zama-fhe/relayer-sdk/node');
const TKMS = requireFromLibrary('node-tkms');
const runtimeRoot = path.join(repoRoot, '.fhevm/runtime');
const runtimeStatePath = path.join(repoRoot, '.fhevm/state/state.json');
const solanaHostRoot = path.join(repoRoot, 'solana-host-contracts');
const legacyAddressesEnvPath = path.join(solanaHostRoot, 'addresses/.env.host');
const solanaExampleEnvPath = path.join(solanaHostRoot, '.env.example');
const legacyTestSuiteEnvPath = path.join(
  repoRoot,
  'test-suite/fhevm/env/staging/.env.test-suite.local',
);
const legacyGatewayEnvPath = path.join(
  repoRoot,
  'test-suite/fhevm/env/staging/.env.gateway-sc.local',
);
const legacyHostEnvPath = path.join(
  repoRoot,
  'test-suite/fhevm/env/staging/.env.host-sc.local',
);
const localCliManifestPath = path.join(
  solanaHostRoot,
  'local-cli/Cargo.toml',
);
const anchorAuthorityKeypairPath = path.join(
  solanaHostRoot,
  'tests/fixtures/anchor-authority.json',
);
const tokenRecipientKeypairPath = path.join(
  solanaHostRoot,
  'tests/fixtures/confidential-token-recipient.json',
);
const runtimeEnvPath = (name) => path.join(runtimeRoot, 'env', `${name}.env`);
const runtimeAddressesEnvPath = (key) =>
  path.join(runtimeRoot, 'addresses', key, '.env.host');

const DB_CONTAINER = 'coprocessor-and-kms-db';
const DB_URL = 'postgresql://postgres:postgres@127.0.0.1:5432/coprocessor';
const RELAYER_URL = 'http://127.0.0.1:3000';
const SOLANA_RPC_URL = 'http://127.0.0.1:18999';
const SOLANA_E2E_COMMITMENT = 'confirmed';
const INPUT_PROOF_U64_VALUE = '18446744073709550042';
const ADD42_INPUT_VALUE = '7';
const TOKEN_TRANSFER_INPUT_VALUE = '1337';
const SERIALIZED_SIZE_LIMIT_CIPHERTEXT = BigInt(1024 * 1024 * 512);
const SERIALIZED_SIZE_LIMIT_PK = BigInt(1024 * 1024 * 512);
const SERIALIZED_SIZE_LIMIT_CRS = BigInt(1024 * 1024 * 512);
const RAW_CT_HASH_DOMAIN_SEPARATOR = 'ZK-w_rct';
const HANDLE_HASH_DOMAIN_SEPARATOR = 'ZK-w_hdl';
const INPUT_PROOF_EXTRA_DATA_VERSION = 0x01;
const DECRYPTION_EXTRA_DATA_VERSION = 0x01;
const MAX_UINT64 = BigInt('18446744073709551615');
const INPUT_ENCRYPTION_TYPES = {
  2: 0,
  8: 2,
  16: 3,
  32: 4,
  64: 5,
  128: 6,
  160: 7,
  256: 8,
};

const verbose = process.argv.includes('--verbose');
const testType = process.argv
  .slice(2)
  .find((arg) => !arg.startsWith('-'));

if (!testType) {
  printUsage();
  process.exit(1);
}

let cachedSolanaEd25519VerifierPromise = null;

function readRuntimeState() {
  if (!fs.existsSync(runtimeStatePath)) {
    return null;
  }
  try {
    return JSON.parse(fs.readFileSync(runtimeStatePath, 'utf8'));
  } catch {
    return null;
  }
}

function solanaChainKeyFromState() {
  const state = readRuntimeState();
  const hostChains = state?.scenario?.hostChains;
  if (!Array.isArray(hostChains)) {
    return 'host';
  }
  return hostChains.find((chain) => (chain?.chainKind || 'evm') === 'solana')?.key || 'host';
}

function activeAddressesEnvPath() {
  const runtimePath = runtimeAddressesEnvPath(solanaChainKeyFromState());
  return fs.existsSync(runtimePath) ? runtimePath : legacyAddressesEnvPath;
}

function activeEnvPath(name, legacyPath) {
  const runtimePath = runtimeEnvPath(name);
  return fs.existsSync(runtimePath) ? runtimePath : legacyPath;
}

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
    if (error instanceof Error) {
      console.error(error.stack ?? error.message);
    } else {
      console.error(String(error));
    }
    cleanup();
    process.exit(1);
  });

async function main() {
  const solanaEnv = parseEnvFile(solanaExampleEnvPath);
  const testSuiteEnvPath = activeEnvPath('test-suite', legacyTestSuiteEnvPath);
  const testSuiteEnv: Record<string, string> = fs.existsSync(testSuiteEnvPath) ? parseEnvFile(testSuiteEnvPath) : {};
  const localnetEnv = {
    ...solanaEnv,
    ...(testSuiteEnv.CHAIN_ID_HOST_SOLANA || testSuiteEnv.CHAIN_ID_HOST
      ? { SOLANA_HOST_CHAIN_ID: testSuiteEnv.CHAIN_ID_HOST_SOLANA || testSuiteEnv.CHAIN_ID_HOST }
      : {}),
    ...(testSuiteEnv.CHAIN_ID_GATEWAY ? { CHAIN_ID_GATEWAY: testSuiteEnv.CHAIN_ID_GATEWAY } : {}),
    ...(testSuiteEnv.INPUT_VERIFICATION_ADDRESS
      ? { INPUT_VERIFICATION_ADDRESS: testSuiteEnv.INPUT_VERIFICATION_ADDRESS }
      : {}),
    ...(testSuiteEnv.DECRYPTION_ADDRESS
      ? { DECRYPTION_ADDRESS: testSuiteEnv.DECRYPTION_ADDRESS }
      : {}),
    ...(testSuiteEnv.SOLANA_DOCKER_HOST_NODE_CONTAINER
      ? { SOLANA_DOCKER_HOST_NODE_CONTAINER: testSuiteEnv.SOLANA_DOCKER_HOST_NODE_CONTAINER }
      : {}),
  };

  await ensureStackDeployed();

  const addresses = parseEnvFile(activeAddressesEnvPath());
  const chainId = Number(addresses.SOLANA_HOST_CHAIN_ID);
  if (!Number.isFinite(chainId)) {
    throw new Error('invalid SOLANA_HOST_CHAIN_ID in addresses env');
  }

  await resetChainState(chainId);

  switch (testType) {
    case 'solana-input-proof':
      await runInputProofCase(addresses, localnetEnv, testSuiteEnv);
      break;
    case 'solana-user-decryption':
      await ensureRelayerReachable();
      await runUserDecryptionCase(addresses, localnetEnv, testSuiteEnv);
      break;
    case 'solana-public-decrypt-http-ebool':
      await runPublicDecryptCase(addresses, localnetEnv, 'scenario-public-ebool');
      break;
    case 'solana-public-decrypt-http-mixed':
      await runPublicDecryptCase(addresses, localnetEnv, 'scenario-public-mixed');
      break;
    case 'solana-confidential-token':
      await runConfidentialTokenCase(addresses, localnetEnv, testSuiteEnv);
      break;
    default:
      printUsage();
      throw new Error(`unknown Solana e2e test type: ${testType}`);
  }
}

function printUsage() {
  console.error(`Usage:
  bun test-suite/e2e/test/solana/run-solana-e2e.ts <test-type> [--verbose]

Supported test types:
  solana-input-proof
  solana-user-decryption
  solana-public-decrypt-http-ebool
  solana-public-decrypt-http-mixed
  solana-confidential-token`);
}

async function ensureStackDeployed() {
  const state = readRuntimeState();
  if (!state) {
    throw new Error(
      'Stack has not been deployed; run `./fhevm-cli deploy --local --solana` first',
    );
  }
  const hasSolana = state?.scenario?.hostChains?.some(
    (c: any) => (c?.chainKind || 'evm') === 'solana',
  );
  if (!hasSolana) {
    throw new Error(
      'Active stack has no Solana host chain; redeploy with --solana or --multi-chain',
    );
  }
  sqlScalar('SELECT 1');
  if (!(await solanaRpcHealthy())) {
    throw new Error(`Solana RPC ${SOLANA_RPC_URL} is not healthy`);
  }
  for (const container of requiredSolanaContainers(state)) {
    if (!dockerContainerRunning(container)) {
      throw new Error(
        `Required container ${container} is not running; deploy the stack first with ./fhevm-cli deploy --local --solana`,
      );
    }
  }
}

function requiredSolanaContainers(state: any) {
  const chains = state?.scenario?.hostChains ?? [];
  const hasMixed =
    chains.some((c: any) => (c?.chainKind || 'evm') === 'evm') &&
    chains.some((c: any) => c?.chainKind === 'solana');
  return hasMixed
    ? [
        'host-node-solana',
        'gateway-node',
        'coprocessor-and-kms-db',
        'coprocessor-host-listener-solana',
        'coprocessor-gw-listener',
        'coprocessor-tfhe-worker',
        'coprocessor-zkproof-worker',
        'coprocessor-sns-worker',
        'coprocessor-transaction-sender',
      ]
    : [
        'host-node',
        'gateway-node',
        'coprocessor-and-kms-db',
        'coprocessor-host-listener',
        'coprocessor-gw-listener',
        'coprocessor-tfhe-worker',
        'coprocessor-zkproof-worker',
        'coprocessor-sns-worker',
        'coprocessor-transaction-sender',
      ];
}

function solanaListenerContainerName() {
  const state = readRuntimeState();
  const chains = state?.scenario?.hostChains ?? [];
  const hasMixed =
    chains.some((c: any) => (c?.chainKind || 'evm') === 'evm') &&
    chains.some((c: any) => c?.chainKind === 'solana');
  return hasMixed ? 'coprocessor-host-listener-solana' : 'coprocessor-host-listener';
}

function ensureSolanaListenerAlive() {
  const containerName = solanaListenerContainerName();
  if (dockerContainerRunning(containerName)) {
    return;
  }
  const logs = runCommandAllowFailure('docker', ['logs', '--tail', '120', containerName], {
    cwd: repoRoot,
    capture: true,
    allowSuccess: true,
  });
  const logOutput = logs.status === 0 ? logs.stdout.trim() || logs.stderr.trim() : '';
  throw new Error(
    `Solana host listener ${containerName} is not running${logOutput ? `\n${logOutput}` : ''}`,
  );
}

function cleanup() {
  // No spawned processes to clean up in deployed mode.
}

async function runInputProofCase(addresses, solanaEnv, testSuiteEnv) {
  await ensureRelayerReachable();
  const aliceWallet = new ethers.Wallet(
    normalizeHexPrivateKey(
      requiredEnvValue(testSuiteEnv, 'DEPLOYER_PRIVATE_KEY', 'solana-input-proof'),
    ),
  );
  const identities = runLocalCliJson('runtime-identities', solanaEnv);
  const contractAddress = requiredScenarioField(
    identities,
    'test_input_contract_evm_address',
    'solana-input-proof',
  );
  const contractIdentity = requiredScenarioField(
    identities,
    'test_input_state_pda_hex',
    'solana-input-proof',
  );
  const userIdentity = requiredScenarioField(
    identities,
    'payer_pubkey_hex',
    'solana-input-proof',
  );
  const aclIdentity = requiredScenarioField(
    identities,
    'host_program_id_hex',
    'solana-input-proof',
  );
  const nativeContractIdByAddress = buildSolanaContractIdentityMap(identities);
  const nativeContractIdentity = getSolanaContractIdentityByAddress(
    nativeContractIdByAddress,
    contractAddress,
  );
  const chainId = Number(addresses.SOLANA_HOST_CHAIN_ID);

  const proofBundle = await buildGatewayBackedInputProof({
    relayerUrl: RELAYER_URL,
    aclAddress: null,
    inputVerificationAddress: requiredEnvValue(
      testSuiteEnv,
      'INPUT_VERIFICATION_ADDRESS',
      'solana-input-proof',
    ),
    gatewayChainId: requiredEnvValue(testSuiteEnv, 'CHAIN_ID_GATEWAY', 'solana-input-proof'),
    hostChainId: addresses.SOLANA_HOST_CHAIN_ID,
    contractAddress,
    userAddress: aliceWallet.address,
    contractIdentity,
    userIdentity,
    aclIdentity,
    value: INPUT_PROOF_U64_VALUE,
    type: 'uint64',
    coprocessorSigners: loadCoprocessorSigners(addresses),
    coprocessorThreshold: requiredEnvValue(
      addresses,
      'COPROCESSOR_THRESHOLD',
      'solana-input-proof',
    ),
  });
  const scenario = runLocalCliScenario('scenario-input-proof', solanaEnv, [
    '--input-handle',
    proofBundle.selectedHandle,
    '--input-proof',
    proofBundle.inputProof,
    '--user-evm-address',
    aliceWallet.address,
  ]);
  await waitForSolanaTransactionsCommitted(
    scenario.signatures,
    SOLANA_E2E_COMMITMENT,
  );
  await waitForSolanaListenerCaughtUp(
    chainId,
    await maxCommittedSlot(scenario.signatures, SOLANA_E2E_COMMITMENT),
  );

  const add42ProofBundle = await buildGatewayBackedInputProof({
    relayerUrl: RELAYER_URL,
    aclAddress: null,
    inputVerificationAddress: requiredEnvValue(
      testSuiteEnv,
      'INPUT_VERIFICATION_ADDRESS',
      'solana-input-proof',
    ),
    gatewayChainId: requiredEnvValue(testSuiteEnv, 'CHAIN_ID_GATEWAY', 'solana-input-proof'),
    hostChainId: addresses.SOLANA_HOST_CHAIN_ID,
    contractAddress,
    userAddress: aliceWallet.address,
    contractIdentity,
    userIdentity,
    aclIdentity,
    value: ADD42_INPUT_VALUE,
    type: 'uint64',
    coprocessorSigners: loadCoprocessorSigners(addresses),
    coprocessorThreshold: requiredEnvValue(
      addresses,
      'COPROCESSOR_THRESHOLD',
      'solana-input-proof',
    ),
  });
  const add42Scenario = runLocalCliScenario('scenario-test-input-add42', solanaEnv, [
    '--input-handle',
    add42ProofBundle.selectedHandle,
    '--input-proof',
    add42ProofBundle.inputProof,
    '--user-evm-address',
    aliceWallet.address,
  ]);
  await waitForSolanaTransactionsCommitted(
    add42Scenario.signatures,
    SOLANA_E2E_COMMITMENT,
  );
  await waitForSolanaListenerCaughtUp(
    chainId,
    await maxCommittedSlot(add42Scenario.signatures, SOLANA_E2E_COMMITMENT),
  );

  const resultHandle = add42Scenario.final_handles?.[0];
  if (!resultHandle) {
    throw new Error('solana-input-proof: add42 scenario did not return a final handle');
  }
  await waitForCiphertextPresent(resultHandle, 'solana-input-proof add42');

  const userDecryptResult = await executeUserDecryptRequest({
    handle: resultHandle,
    contractAddress,
    userIdentity,
    nativeContractIdentities: nativeContractIdentity ? [nativeContractIdentity] : null,
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
    userWallet: aliceWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-input-proof add42',
  });
  if (!userDecryptResult.success || userDecryptResult.decryptedValue !== '49') {
    throw new Error(
      `solana-input-proof: expected add42 user decrypt clear value 49, got ${formatUserDecryptOutcome(
        userDecryptResult,
      )}${userDecryptResult.decryptedValue !== undefined ? ` value=${userDecryptResult.decryptedValue}` : ''}`,
    );
  }

  const publicDecryptResult = await executePublicDecryptRequest({
    handles: [resultHandle],
    label: 'solana-input-proof add42',
  });
  if (!publicDecryptResult.success) {
    throw new Error(
      `solana-input-proof: expected public decrypt to succeed for add42 result, got ${formatPublicDecryptOutcome(
        publicDecryptResult,
      )}`,
    );
  }
  if (publicDecryptResult.clearValues[resultHandle] !== '49') {
    throw new Error(
      `solana-input-proof: expected public decrypt clear value 49 for handle ${resultHandle}, got ${publicDecryptResult.clearValues[resultHandle]}`,
    );
  }

  console.log('Solana input-proof passed');
  console.log(`requestUint64NonTrivial signatures: ${scenario.signatures.join(', ')}`);
  console.log(`add42ToInput64 signatures: ${add42Scenario.signatures.join(', ')}`);
  console.log(
    `add42ToInput64 handle=${resultHandle} userDecrypt=49 publicDecrypt=49`,
  );
}

async function runUserDecryptionCase(addresses, solanaEnv, testSuiteEnv) {
  const aliceWallet = new ethers.Wallet(
    normalizeHexPrivateKey(
      requiredEnvValue(testSuiteEnv, 'DEPLOYER_PRIVATE_KEY', 'solana-user-decryption'),
    ),
  );
  const bobWallet = ethers.Wallet.createRandom();
  const identities = runLocalCliJson('runtime-identities', solanaEnv);
  const nativeContractIdByAddress = buildSolanaContractIdentityMap(identities);
  const scenario = runLocalCliScenario('scenario-user-decrypt', solanaEnv, [
    '--user-evm-address',
    aliceWallet.address,
  ]);
  await waitForSolanaTransactionsCommitted(
    scenario.signatures,
    SOLANA_E2E_COMMITMENT,
  );
  await waitForSolanaListenerCaughtUp(
    Number(addresses.SOLANA_HOST_CHAIN_ID),
    await maxCommittedSlot(scenario.signatures, SOLANA_E2E_COMMITMENT),
  );

  const chainId = Number(addresses.SOLANA_HOST_CHAIN_ID);
  const contractAddress = requiredScenarioField(
    scenario,
    'contract_evm_address',
    'solana-user-decryption',
  );
  const nativeContractIdentity = getSolanaContractIdentityByAddress(
    nativeContractIdByAddress,
    contractAddress,
  );
  const userIdentity = requiredScenarioField(
    identities,
    'payer_pubkey_hex',
    'solana-user-decryption',
  );
  const handles = Array.isArray(scenario.final_handles) ? scenario.final_handles : [];
  const expectedClearValues = scenarioExpectedClearValues(scenario);
  if (handles.length !== 8) {
    throw new Error(
      `solana-user-decryption: expected 8 fixture handles, got ${handles.length}`,
    );
  }

  const actualClearValues = {};
  for (const handle of handles) {
    await waitForComputationRow(chainId, handle, `solana-user-decryption ${handle}`);
    await waitForCiphertextPresent(handle, `solana-user-decryption ${handle}`);
    const handleState = queryComputationState(chainId, handle);
    if (!handleState.exists || !handleState.isCompleted || handleState.isError) {
      throw new Error(
        `solana-user-decryption: expected handle ${handle} to be fully computed, got ${formatComputationState(
          handleState,
        )}`,
      );
    }
    const allowedRows = countAllowedHandleRows(chainId, handle);
    if (allowedRows < 4) {
      throw new Error(
        `solana-user-decryption: expected app + app-evm + user + user-evm allow rows for handle ${handle}, got ${allowedRows}`,
      );
    }

    const decryptResult = await executeUserDecryptRequest({
      handle,
      contractAddress,
      userIdentity,
      nativeContractIdentities: nativeContractIdentity ? [nativeContractIdentity] : null,
      nativeSignerKeypairPath: anchorAuthorityKeypairPath,
      userWallet: aliceWallet,
      addresses,
      testSuiteEnv,
      label: `solana-user-decryption ${handle}`,
    });
    if (!decryptResult.success) {
      throw new Error(
        `solana-user-decryption: expected a successful user decrypt job for handle ${handle}, got ${formatUserDecryptOutcome(
          decryptResult,
        )}`,
      );
    }
    actualClearValues[handle] = decryptResult.decryptedValue;
  }

  if (JSON.stringify(actualClearValues) !== JSON.stringify(expectedClearValues)) {
    throw new Error(
      `solana-user-decryption: clear values mismatch.\nexpected=${JSON.stringify(
        expectedClearValues,
      )}\nactual=${JSON.stringify(actualClearValues)}`,
    );
  }

  const boolHandle = requiredScenarioField(
    scenario,
    'x_bool_handle',
    'solana-user-decryption',
  );
  const uint8Handle = requiredScenarioField(
    scenario,
    'x_uint8_handle',
    'solana-user-decryption',
  );
  const wrongContractAddress = requiredScenarioField(
    identities,
    'confidential_token_contract_evm_address',
    'solana-user-decryption',
  );
  const wrongContractIdentity = getSolanaContractIdentityByAddress(
    nativeContractIdByAddress,
    wrongContractAddress,
  );

  const unauthorizedDecrypt = await executeUserDecryptRequest({
    handle: boolHandle,
    contractAddress,
    nativeContractIdentities: nativeContractIdentity ? [nativeContractIdentity] : null,
    userWallet: bobWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-user-decryption unauthorized',
  });
  if (
    unauthorizedDecrypt.success ||
    !isUserDecryptAclRejection(unauthorizedDecrypt)
  ) {
    throw new Error(
      `solana-user-decryption: expected unauthorized user decrypt to be rejected, got ${formatUserDecryptOutcome(
        unauthorizedDecrypt,
      )}`,
    );
  }

  const userEqualsContract = await executeUserDecryptRequest({
    handle: boolHandle,
    contractAddress: aliceWallet.address,
    userWallet: aliceWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-user-decryption user-equals-contract',
  });
  if (
    userEqualsContract.success ||
    !isUserEqualsContractRejection(userEqualsContract)
  ) {
    throw new Error(
      `solana-user-decryption: expected userAddress == contractAddress to be rejected, got ${formatUserDecryptOutcome(
        userEqualsContract,
      )}`,
    );
  }

  const wrongContract = await executeUserDecryptRequest({
    handle: boolHandle,
    contractAddress: wrongContractAddress,
    userIdentity,
    nativeContractIdentities: wrongContractIdentity ? [wrongContractIdentity] : null,
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
    userWallet: aliceWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-user-decryption wrong-contract',
  });
  if (
    wrongContract.success ||
    !isUserDecryptAclRejection(wrongContract)
  ) {
    throw new Error(
      `solana-user-decryption: expected wrong contract to be rejected, got ${formatUserDecryptOutcome(
        wrongContract,
      )}`,
    );
  }

  const expiredRequest = await executeUserDecryptRequest({
    handle: uint8Handle,
    contractAddress,
    userIdentity,
    nativeContractIdentities: nativeContractIdentity ? [nativeContractIdentity] : null,
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
    userWallet: aliceWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-user-decryption expired',
    startTimestamp: Math.floor(Date.now() / 1000) - 20 * 86400,
    durationDays: 10,
  });
  if (
    expiredRequest.success ||
    !isExpiredUserDecryptRejection(expiredRequest)
  ) {
    throw new Error(
      `solana-user-decryption: expected expired request to be rejected, got ${formatUserDecryptOutcome(
        expiredRequest,
      )}`,
    );
  }

  console.log('Solana user-decryption passed');
  console.log(`Handles: ${handles.join(', ')}`);
  console.log(`userAddress=${aliceWallet.address}`);
  console.log(`clearValues=${JSON.stringify(actualClearValues)}`);
}

async function executeUserDecryptRequest({
  handle,
  handles,
  contractAddress,
  contractAddresses,
  userIdentity,
  nativeContractIdentities,
  nativeSignerKeypairPath,
  handleContractPairs,
  userWallet,
  addresses,
  testSuiteEnv,
  label,
  startTimestamp,
  durationDays,
}) {
  const { publicKey, privateKey } = generateRelayerKeypair();
  const requestHandles = Array.isArray(handles) && handles.length > 0 ? handles : [handle];
  const requestContractAddresses =
    Array.isArray(contractAddresses) && contractAddresses.length > 0
      ? contractAddresses
      : [contractAddress];
  const requestHandleContractPairs =
    Array.isArray(handleContractPairs) && handleContractPairs.length > 0
      ? handleContractPairs
      : requestHandles.map((requestHandle) => ({
          handle: requestHandle,
          contractAddress,
        }));
  const requestNativeContractIdentities =
    Array.isArray(nativeContractIdentities) && nativeContractIdentities.length > 0
      ? nativeContractIdentities
      : [];
  const requestStartTimestamp =
    startTimestamp ?? (await latestGatewayBlockTimestamp(testSuiteEnv, label));
  const requestDurationDays = durationDays ?? 1;
  const userAddress = userWallet.address;
  const useNativeSolanaAuth =
    typeof nativeSignerKeypairPath === 'string' &&
    nativeSignerKeypairPath.length > 0 &&
    userIdentity &&
    requestNativeContractIdentities.length === requestContractAddresses.length;
  const extraData = useNativeSolanaAuth
    ? buildDecryptionExtraData(
        loadKmsContextId({ ...addresses, ...testSuiteEnv }),
        [userIdentity, ...requestNativeContractIdentities],
        await buildNativeSolanaAuthSigner(testSuiteEnv, nativeSignerKeypairPath),
      )
    : userIdentity && requestNativeContractIdentities.length === requestContractAddresses.length
      ? buildDecryptionExtraData(
          loadKmsContextId({ ...addresses, ...testSuiteEnv }),
          [userIdentity, ...requestNativeContractIdentities],
        )
      : '0x00';
  const eip712 = buildUserDecryptEip712({
    verifyingContract: requiredEnvValue(testSuiteEnv, 'DECRYPTION_ADDRESS', label),
    contractsChainId: Number(addresses.SOLANA_HOST_CHAIN_ID),
    publicKey,
    contractAddresses: requestContractAddresses,
    startTimestamp: requestStartTimestamp,
    durationDays: requestDurationDays,
    extraData,
  });
  const signature = useNativeSolanaAuth
    ? signEd25519TypedData(eip712, loadSolanaEd25519PrivateKey(nativeSignerKeypairPath))
    : await userWallet.signTypedData(
        eip712.domain,
        Object.fromEntries(
          Object.entries(eip712.types).filter(([typeName]) => typeName !== 'EIP712Domain'),
        ),
        eip712.message,
      );

  const body = {
    handleContractPairs: requestHandleContractPairs,
    requestValidity: {
      startTimestamp: String(requestStartTimestamp),
      durationDays: String(requestDurationDays),
    },
    contractsChainId: addresses.SOLANA_HOST_CHAIN_ID,
    contractAddresses: requestContractAddresses,
    userAddress,
    signature: signature.replace(/^0x/, ''),
    publicKey: publicKey.replace(/^0x/, ''),
    extraData,
    ...(userIdentity ? { userId: userIdentity } : {}),
    ...(requestNativeContractIdentities.length > 0
      ? { contractIds: requestNativeContractIdentities }
      : {}),
  };

  const response = await fetch(`${RELAYER_URL}/v2/user-decrypt`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(body),
  });
  const responseText = await response.text();

  if (!response.ok) {
    return {
      accepted: false,
      httpStatus: response.status,
      responseText,
      errorMessage: extractErrorMessage(responseText),
    };
  }

  const postPayload = JSON.parse(responseText);
  const jobId = postPayload?.result?.jobId;
  if (!jobId) {
    return {
      accepted: false,
      httpStatus: response.status,
      responseText,
      parseError: 'missing jobId',
      errorMessage: extractErrorMessage(responseText),
    };
  }

  const statusResult = await waitForUserDecryptJob(jobId);
  if (statusResult.httpStatus !== 200 || statusResult.payload?.status !== 'succeeded') {
    return {
      accepted: true,
      success: false,
      jobId,
      httpStatus: statusResult.httpStatus,
      payload: statusResult.payload,
      errorMessage: extractPayloadErrorMessage(statusResult.payload),
    };
  }

  const clearValues = decryptUserDecryptResult({
    responsePayload: statusResult.payload,
    handles: requestHandles,
    userAddress,
    publicKey,
    privateKey,
    gatewayChainId: requiredEnvValue(testSuiteEnv, 'CHAIN_ID_GATEWAY', label),
    verifyingContract: requiredEnvValue(testSuiteEnv, 'DECRYPTION_ADDRESS', label),
    kmsSigners: loadKmsSigners(addresses),
    extraData,
  });

  return {
    accepted: true,
    success: true,
    jobId,
    clearValues,
    decryptedValue: clearValues[requestHandles[0]],
    userAddress,
  };
}

function buildUserDecryptEip712({
  verifyingContract,
  contractsChainId,
  publicKey,
  contractAddresses,
  startTimestamp,
  durationDays,
  extraData,
}) {
  if (!ethers.isAddress(verifyingContract)) {
    throw new Error(`invalid verifying contract address: ${verifyingContract}`);
  }
  if (
    !Array.isArray(contractAddresses) ||
    contractAddresses.some((address) => !ethers.isAddress(address))
  ) {
    throw new Error(`invalid contract address list for user decrypt: ${contractAddresses}`);
  }

  return {
    types: {
      EIP712Domain: [
        { name: 'name', type: 'string' },
        { name: 'version', type: 'string' },
        { name: 'chainId', type: 'uint256' },
        { name: 'verifyingContract', type: 'address' },
      ],
      UserDecryptRequestVerification: [
        { name: 'publicKey', type: 'bytes' },
        { name: 'contractAddresses', type: 'address[]' },
        { name: 'startTimestamp', type: 'uint256' },
        { name: 'durationDays', type: 'uint256' },
        { name: 'extraData', type: 'bytes' },
      ],
    },
    primaryType: 'UserDecryptRequestVerification',
    domain: {
      name: 'Decryption',
      version: '1',
      chainId: contractsChainId,
      verifyingContract,
    },
    message: {
      publicKey: ensure0xHex(publicKey),
      contractAddresses,
      startTimestamp: String(startTimestamp),
      durationDays: String(durationDays),
      extraData,
    },
  };
}

function formatUserDecryptOutcome(result) {
  if (!result.accepted) {
    return `http=${result.httpStatus} body=${result.responseText ?? ''}${
      result.parseError ? ` parseError=${result.parseError}` : ''
    }`;
  }
  return `http=${result.httpStatus} payload=${JSON.stringify(result.payload)}`;
}

function includesErrorMessage(result, snippet) {
  const message = String(
    result?.errorMessage ??
      result?.responseText ??
      JSON.stringify(result?.payload ?? {}),
  ).toLowerCase();
  return message.includes(String(snippet).toLowerCase());
}

function isUserDecryptAclRejection(result) {
  return (
    includesErrorMessage(result, 'not authorized to user decrypt handle') ||
    includesErrorMessage(result, 'not allowed on host acl') ||
    includesErrorMessage(result, 'acl check failed')
  );
}

function isUserEqualsContractRejection(result) {
  return (
    includesErrorMessage(
      result,
      'should not be equal to contract address when requesting user decryption',
    ) ||
    includesErrorMessage(result, 'useraddressincontractaddresses') ||
    includesErrorMessage(result, '0xdc4d78b1')
  );
}

function isExpiredUserDecryptRejection(result) {
  return (
    includesErrorMessage(result, 'user decrypt request has expired') ||
    includesErrorMessage(result, 'userdecryptionrequestexpired') ||
    includesErrorMessage(result, '0x30348040')
  );
}

function extractErrorMessage(responseText) {
  if (typeof responseText !== 'string' || responseText.length === 0) {
    return null;
  }
  try {
    const payload = JSON.parse(responseText);
    return (
      payload?.error?.message ??
      payload?.result?.error?.message ??
      payload?.message ??
      responseText
    );
  } catch {
    return responseText;
  }
}

function extractPayloadErrorMessage(payload) {
  if (!payload || typeof payload !== 'object') {
    return null;
  }
  return (
    payload?.error?.message ??
    payload?.result?.error?.message ??
    payload?.message ??
    null
  );
}

function handleLiteral(handle) {
  return `\\\\x${stripHexPrefix(handle).toLowerCase()}`;
}

function assertMintHandleMetadata(handle, hostChainId, label) {
  const normalized = handle.startsWith('0x') ? handle : `0x${handle}`;
  if (normalized.slice(44, 46) !== 'ff') {
    throw new Error(`${label}: expected mint handle index byte to be ff, got ${normalized}`);
  }
  if (normalized.slice(46, 62) !== Number(hostChainId).toString(16).padStart(16, '0')) {
    throw new Error(
      `${label}: expected mint handle chain id bytes to be ${Number(hostChainId)
        .toString(16)
        .padStart(16, '0')}, got ${normalized.slice(46, 62)}`,
    );
  }
  if (normalized.slice(62, 64) !== '05') {
    throw new Error(`${label}: expected mint handle FHE type byte 05 (euint64), got ${normalized}`);
  }
  if (normalized.slice(64, 66) !== '00') {
    throw new Error(`${label}: expected mint handle version byte 00, got ${normalized}`);
  }
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
    await waitForComputationRow(chainId, handle, `${scenarioName} ${handle}`);
    await waitForCiphertextPresent(handle, `${scenarioName} ${handle}`);
    const computationState = queryComputationState(chainId, handle);
    const publicAllowRows = countPublicDecryptAllowRows(chainId, handle);
    const pbsRows = countPbsRows(chainId, handle);
    if (!computationState.exists || !computationState.isCompleted || computationState.isError) {
      throw new Error(
        `${scenarioName}: expected handle ${handle} to be fully computed before public decrypt, got ${formatComputationState(
          computationState,
        )}`,
      );
    }
    if (publicAllowRows < 1 || pbsRows < 1) {
      throw new Error(
        `${scenarioName}: expected handle ${handle} to be publicly decryptable on the Solana host, got public_allow_rows=${publicAllowRows}, pbs_rows=${pbsRows}`,
      );
    }
  }

  const publicDecryptResult = await executePublicDecryptRequest({
    handles,
    label: scenarioName,
  });
  if (!publicDecryptResult.success) {
    throw new Error(
      `${scenarioName}: public decrypt failed unexpectedly: ${formatPublicDecryptOutcome(
        publicDecryptResult,
      )}`,
    );
  }

  const actualValues = publicDecryptResult.clearValues;
  const expectedValues = scenarioExpectedClearValues(scenario);
  if (JSON.stringify(actualValues) !== JSON.stringify(expectedValues)) {
    throw new Error(
      `${scenarioName}: clear values mismatch.\nexpected=${JSON.stringify(
        expectedValues,
      )}\nactual=${JSON.stringify(actualValues)}`,
    );
  }

  if (scenarioName === 'scenario-public-ebool') {
    const identities = runLocalCliJson('runtime-identities', solanaEnv);
    const nonPublicScenario = runLocalCliScenario('scenario-user-decrypt', solanaEnv, [
      '--user-evm-address',
      requiredScenarioField(identities, 'user_evm_address', `${scenarioName} negative`),
    ]);
    await waitForSolanaTransactionsCommitted(
      nonPublicScenario.signatures,
      SOLANA_E2E_COMMITMENT,
    );
    await waitForSolanaListenerCaughtUp(
      chainId,
      await maxCommittedSlot(nonPublicScenario.signatures, SOLANA_E2E_COMMITMENT),
    );
    const nonPublicHandle = requiredScenarioField(
      nonPublicScenario,
      'x_uint8_handle',
      `${scenarioName} negative`,
    );
    const rejectedResult = await executePublicDecryptRequest({
      handles: [nonPublicHandle],
      label: `${scenarioName} negative`,
    });
    if (
      rejectedResult.success ||
      !(
        includesErrorMessage(rejectedResult, 'not allowed for public decryption') ||
        includesErrorMessage(rejectedResult, 'not allowed on host acl') ||
        includesErrorMessage(rejectedResult, 'acl check failed')
      )
    ) {
      throw new Error(
        `${scenarioName}: expected non-public handle ${nonPublicHandle} to be rejected, got ${formatPublicDecryptOutcome(
          rejectedResult,
        )}`,
      );
    }
  }

  console.log(`Solana ${scenarioName} public decrypt passed`);
  console.log(`jobId=${publicDecryptResult.jobId}`);
  console.log(`clearValues=${JSON.stringify(actualValues)}`);
}

async function executePublicDecryptRequest({ handles, label }) {
  const response = await fetch(`${RELAYER_URL}/v2/public-decrypt`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({
      ciphertextHandles: handles,
      extraData: '0x00',
    }),
  });
  const responseText = await response.text();
  if (!response.ok) {
    return {
      accepted: false,
      httpStatus: response.status,
      responseText,
      errorMessage: extractErrorMessage(responseText),
    };
  }

  const postPayload = JSON.parse(responseText);
  const jobId = postPayload?.result?.jobId;
  if (!jobId) {
    return {
      accepted: false,
      httpStatus: response.status,
      responseText,
      parseError: 'missing jobId',
      errorMessage: extractErrorMessage(responseText),
    };
  }

  const statusResult = await waitForPublicDecryptJob(jobId);
  if (statusResult.httpStatus !== 200 || statusResult.payload?.status !== 'succeeded') {
    return {
      accepted: true,
      success: false,
      jobId,
      httpStatus: statusResult.httpStatus,
      payload: statusResult.payload,
      responseText: statusResult.responseText,
      errorMessage:
        extractPayloadErrorMessage(statusResult.payload) ??
        extractErrorMessage(statusResult.responseText),
    };
  }

  return {
    accepted: true,
    success: true,
    jobId,
    httpStatus: statusResult.httpStatus,
    payload: statusResult.payload,
    clearValues: decodePublicDecryptClearValues(
      handles,
      statusResult.payload?.result?.decryptedValue,
    ),
  };
}

function formatPublicDecryptOutcome(result) {
  if (!result.accepted) {
    return `http=${result.httpStatus} body=${result.responseText ?? ''}${
      result.parseError ? ` parseError=${result.parseError}` : ''
    }`;
  }
  return `http=${result.httpStatus} payload=${JSON.stringify(result.payload)}`;
}

async function runConfidentialTokenCase(addresses, solanaEnv, testSuiteEnv) {
  await ensureRelayerReachable();
  const aliceWallet = new ethers.Wallet(
    normalizeHexPrivateKey(
      requiredEnvValue(testSuiteEnv, 'DEPLOYER_PRIVATE_KEY', 'solana-confidential-token'),
    ),
  );
  const bobWallet = ethers.Wallet.createRandom();
  const identities = runLocalCliJson('runtime-identities', solanaEnv);
  const contractAddress = requiredScenarioField(
    identities,
    'confidential_token_contract_evm_address',
    'solana-confidential-token',
  );
  const contractIdentity = requiredScenarioField(
    identities,
    'confidential_token_state_pda_hex',
    'solana-confidential-token',
  );
  const nativeContractIdByAddress = buildSolanaContractIdentityMap(identities);
  const nativeContractIdentity = getSolanaContractIdentityByAddress(
    nativeContractIdByAddress,
    contractAddress,
  );
  const chainId = Number(addresses.SOLANA_HOST_CHAIN_ID);
  const alicePubkey = requiredScenarioField(identities, 'payer_pubkey', 'solana-confidential-token');
  const aliceIdentity = requiredScenarioField(identities, 'payer_pubkey_hex', 'solana-confidential-token');
  const aliceUserAddress = requiredScenarioField(
    identities,
    'user_evm_address',
    'solana-confidential-token',
  );
  const bobPubkey = requiredScenarioField(identities, 'token_recipient_pubkey', 'solana-confidential-token');
  const bobIdentity = requiredScenarioField(identities, 'token_recipient_pubkey_hex', 'solana-confidential-token');
  const bobUserAddress = requiredScenarioField(
    identities,
    'token_recipient_evm_address',
    'solana-confidential-token',
  );
  const aclIdentity = requiredScenarioField(identities, 'host_program_id_hex', 'solana-confidential-token');

  function runTokenCommand(commandName, extraArgs = [], payerKeypairPath = anchorAuthorityKeypairPath) {
    return runLocalCliJson(commandName, solanaEnv, [
      '--payer-keypair',
      payerKeypairPath,
      ...extraArgs,
    ]);
  }

  async function waitForResultBatch(label, ...results) {
    const signatures = results
      .flatMap((result) =>
        Array.isArray(result?.signatures)
          ? result.signatures
          : result?.signature
            ? [result.signature]
            : [],
      )
      .filter(Boolean);
    if (signatures.length === 0) {
      return;
    }
    await waitForSolanaTransactionsCommitted(signatures, SOLANA_E2E_COMMITMENT);
    await waitForSolanaListenerCaughtUp(
      chainId,
      await maxCommittedSlot(signatures, SOLANA_E2E_COMMITMENT),
    );
  }

  async function buildTokenAmountProof(value, userIdentity, userAddress, label) {
    return buildGatewayBackedInputProof({
      relayerUrl: RELAYER_URL,
      aclAddress: null,
      inputVerificationAddress: requiredEnvValue(
        testSuiteEnv,
        'INPUT_VERIFICATION_ADDRESS',
        label,
      ),
      gatewayChainId: requiredEnvValue(testSuiteEnv, 'CHAIN_ID_GATEWAY', label),
      hostChainId: addresses.SOLANA_HOST_CHAIN_ID,
      contractAddress,
      userAddress,
      contractIdentity,
      userIdentity,
      aclIdentity,
      value: String(value),
      type: 'uint64',
      coprocessorSigners: loadCoprocessorSigners(addresses),
      coprocessorThreshold: requiredEnvValue(addresses, 'COPROCESSOR_THRESHOLD', label),
    });
  }

  function singleReturnedHandle(result, label) {
    const handle = result?.returned_handles?.[0];
    if (!handle) {
      throw new Error(`${label}: command did not return a handle`);
    }
    return handle;
  }

  const resetMint = runTokenCommand('token-reset');
  const mint1000 = runTokenCommand('token-mint-to', [
    '--amount',
    '1000',
  ]);
  const mintBalanceQuery = runTokenCommand('token-balance', [
    '--owner',
    alicePubkey,
  ]);
  const mintTotalSupply = runTokenCommand('token-supply');
  await waitForResultBatch('solana-confidential-token mint', resetMint, mint1000, mintBalanceQuery);
  const mintBalanceHandle = singleReturnedHandle(mintBalanceQuery, 'solana-confidential-token mint balance');
  assertMintHandleMetadata(mintBalanceHandle, addresses.SOLANA_HOST_CHAIN_ID, 'solana-confidential-token');

  await waitForComputationRow(chainId, mintBalanceHandle, 'solana-confidential-token mint');
  await waitForCiphertextPresent(mintBalanceHandle, 'solana-confidential-token mint');

  const mintState = queryComputationState(chainId, mintBalanceHandle);
  const mintAllowRows = countAllowedHandleRows(chainId, mintBalanceHandle);
  const mintPbsRows = countPbsRows(chainId, mintBalanceHandle);

  if (!mintState.exists || !mintState.isCompleted || mintState.isError) {
    throw new Error(
      `solana-confidential-token: expected mint handle to be fully computed, got ${formatComputationState(
        mintState,
      )}`,
    );
  }
  if (mintAllowRows < 3) {
    throw new Error(
      `solana-confidential-token: expected durable contract + owner allow rows for mint balance handle, got ${mintAllowRows}`,
    );
  }
  if (mintPbsRows < 1) {
    throw new Error(
      `solana-confidential-token: expected at least one PBS row for mint balance handle, got ${mintPbsRows}`,
    );
  }
  if (String(mintTotalSupply.total_supply) !== '1000') {
    throw new Error(
      `solana-confidential-token: expected total supply 1000 after mint, got ${String(mintTotalSupply.total_supply)}`,
    );
  }

  const mintDecrypt = await executeUserDecryptRequest({
    handle: mintBalanceHandle,
    contractAddress,
    userIdentity: aliceIdentity,
    nativeContractIdentities: nativeContractIdentity ? [nativeContractIdentity] : null,
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
    userWallet: aliceWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-confidential-token mint',
  });
  if (!mintDecrypt.success || mintDecrypt.decryptedValue !== '1000') {
    throw new Error(
      `solana-confidential-token: expected Alice mint balance to decrypt to 1000, got ${formatUserDecryptOutcome(
        mintDecrypt,
      )}${mintDecrypt.decryptedValue !== undefined ? ` value=${mintDecrypt.decryptedValue}` : ''}`,
    );
  }

  const resetSuccess = runTokenCommand('token-reset');
  const mint10000ForSuccess = runTokenCommand('token-mint-to', [
    '--amount',
    '10000',
  ]);
  const successTransferProof = await buildTokenAmountProof(
    TOKEN_TRANSFER_INPUT_VALUE,
    aliceIdentity,
    aliceUserAddress,
    'solana-confidential-token transfer success',
  );
  const successTransfer = runTokenCommand('token-transfer', [
    '--recipient',
    bobPubkey,
    '--input-handle',
    successTransferProof.selectedHandle,
    '--input-proof',
    successTransferProof.inputProof,
  ]);
  const successAliceBalanceQuery = runTokenCommand('token-balance', [
    '--owner',
    alicePubkey,
  ]);
  const successBobBalanceQuery = runTokenCommand('token-balance', [
    '--owner',
    bobPubkey,
  ]);
  await waitForResultBatch(
    'solana-confidential-token transfer success',
    resetSuccess,
    mint10000ForSuccess,
    successTransfer,
    successAliceBalanceQuery,
    successBobBalanceQuery,
  );

  const transferInputHandle = successTransferProof.selectedHandle;
  const aliceAfterTransferHandle = singleReturnedHandle(
    successAliceBalanceQuery,
    'solana-confidential-token transfer(alice)',
  );
  const bobAfterTransferHandle = singleReturnedHandle(
    successBobBalanceQuery,
    'solana-confidential-token transfer(bob)',
  );

  await waitForComputationRow(
    chainId,
    aliceAfterTransferHandle,
    'solana-confidential-token transfer(alice)',
  );
  await waitForComputationRow(
    chainId,
    bobAfterTransferHandle,
    'solana-confidential-token transfer(bob)',
  );
  await waitForCiphertextPresent(
    transferInputHandle,
    'solana-confidential-token transfer input',
  );
  await waitForCiphertextPresent(
    aliceAfterTransferHandle,
    'solana-confidential-token transfer(alice)',
  );
  await waitForCiphertextPresent(
    bobAfterTransferHandle,
    'solana-confidential-token transfer(bob)',
  );

  const computationsUsingInput = countComputationsDependingOnHandle(
    chainId,
    transferInputHandle,
  );
  const transferInputCiphertexts = countCiphertextRows(transferInputHandle);
  const aliceAfterState = queryComputationState(chainId, aliceAfterTransferHandle);
  const bobAfterState = queryComputationState(chainId, bobAfterTransferHandle);

  if (computationsUsingInput < 1) {
    throw new Error(
      'solana-confidential-token: expected at least one computation row to depend on the transfer input handle',
    );
  }
  if (transferInputCiphertexts < 1) {
    throw new Error(
      `solana-confidential-token: expected ciphertext material for transfer input handle, got ${transferInputCiphertexts}`,
    );
  }
  if (!aliceAfterState.exists || !bobAfterState.exists) {
    throw new Error(
      `solana-confidential-token: expected transfer output handles to be ingested, got alice=${formatComputationState(
        aliceAfterState,
      )} bob=${formatComputationState(bobAfterState)}`,
    );
  }
  if (!aliceAfterState.isCompleted || !bobAfterState.isCompleted) {
    throw new Error(
      `solana-confidential-token: expected transfer outputs to be fully computed once the real input-proof path is used, got alice=${formatComputationState(
        aliceAfterState,
      )} bob=${formatComputationState(bobAfterState)}`,
    );
  }

  const aliceAfterDecrypt = await executeUserDecryptRequest({
    handle: aliceAfterTransferHandle,
    contractAddress,
    userIdentity: aliceIdentity,
    nativeContractIdentities: nativeContractIdentity ? [nativeContractIdentity] : null,
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
    userWallet: aliceWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-confidential-token alice-after-transfer',
  });
  if (!aliceAfterDecrypt.success || aliceAfterDecrypt.decryptedValue !== '8663') {
    throw new Error(
      `solana-confidential-token: expected Alice post-transfer balance to decrypt to 8663, got ${formatUserDecryptOutcome(
        aliceAfterDecrypt,
      )}${aliceAfterDecrypt.decryptedValue ? ` value=${aliceAfterDecrypt.decryptedValue}` : ''}`,
    );
  }

  const bobAfterDecrypt = await executeUserDecryptRequest({
    handle: bobAfterTransferHandle,
    contractAddress,
    userIdentity: bobIdentity,
    nativeContractIdentities: nativeContractIdentity ? [nativeContractIdentity] : null,
    nativeSignerKeypairPath: tokenRecipientKeypairPath,
    userWallet: bobWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-confidential-token bob-after-transfer',
  });
  if (!bobAfterDecrypt.success || bobAfterDecrypt.decryptedValue !== '1337') {
    throw new Error(
      `solana-confidential-token: expected recipient post-transfer balance to decrypt to 1337, got ${formatUserDecryptOutcome(
        bobAfterDecrypt,
      )}${bobAfterDecrypt.decryptedValue ? ` value=${bobAfterDecrypt.decryptedValue}` : ''}`,
    );
  }

  const unauthorizedDecrypt = await executeUserDecryptRequest({
    handle: aliceAfterTransferHandle,
    contractAddress,
    userIdentity: bobIdentity,
    nativeContractIdentities: nativeContractIdentity ? [nativeContractIdentity] : null,
    nativeSignerKeypairPath: tokenRecipientKeypairPath,
    userWallet: bobWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-confidential-token unauthorized',
  });
  if (
    unauthorizedDecrypt.success ||
    !isUserDecryptAclRejection(unauthorizedDecrypt)
  ) {
    throw new Error(
      `solana-confidential-token: expected the recipient to be rejected for Alice's balance handle, got ${formatUserDecryptOutcome(
        unauthorizedDecrypt,
      )}`,
    );
  }

  const successTotalSupply = runTokenCommand('token-supply');
  if (String(successTotalSupply.total_supply) !== '10000') {
    throw new Error(
      `solana-confidential-token: expected total supply 10000 after successful transfer flow, got ${String(successTotalSupply.total_supply)}`,
    );
  }

  const resetFailed = runTokenCommand('token-reset');
  const mint1000ForFailure = runTokenCommand('token-mint-to', [
    '--amount',
    '1000',
  ]);
  const failedTransferProof = await buildTokenAmountProof(
    TOKEN_TRANSFER_INPUT_VALUE,
    aliceIdentity,
    aliceUserAddress,
    'solana-confidential-token transfer failure',
  );
  const failedTransfer = runTokenCommand('token-transfer', [
    '--recipient',
    bobPubkey,
    '--input-handle',
    failedTransferProof.selectedHandle,
    '--input-proof',
    failedTransferProof.inputProof,
  ]);
  const failedAliceBalanceQuery = runTokenCommand('token-balance', [
    '--owner',
    alicePubkey,
  ]);
  const failedBobBalanceQuery = runTokenCommand('token-balance', [
    '--owner',
    bobPubkey,
  ]);
  await waitForResultBatch(
    'solana-confidential-token transfer failure',
    resetFailed,
    mint1000ForFailure,
    failedTransfer,
    failedAliceBalanceQuery,
    failedBobBalanceQuery,
  );
  const failedAliceHandle = singleReturnedHandle(
    failedAliceBalanceQuery,
    'solana-confidential-token failed transfer(alice)',
  );
  const failedBobHandle = singleReturnedHandle(
    failedBobBalanceQuery,
    'solana-confidential-token failed transfer(bob)',
  );
  const failedAliceDecrypt = await executeUserDecryptRequest({
    handle: failedAliceHandle,
    contractAddress,
    userIdentity: aliceIdentity,
    nativeContractIdentities: nativeContractIdentity ? [nativeContractIdentity] : null,
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
    userWallet: aliceWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-confidential-token failed alice',
  });
  if (!failedAliceDecrypt.success || failedAliceDecrypt.decryptedValue !== '1000') {
    throw new Error(
      `solana-confidential-token: expected failed-transfer Alice balance to remain 1000, got ${formatUserDecryptOutcome(
        failedAliceDecrypt,
      )}${failedAliceDecrypt.decryptedValue !== undefined ? ` value=${failedAliceDecrypt.decryptedValue}` : ''}`,
    );
  }
  const failedBobDecrypt = await executeUserDecryptRequest({
    handle: failedBobHandle,
    contractAddress,
    userIdentity: bobIdentity,
    nativeContractIdentities: nativeContractIdentity ? [nativeContractIdentity] : null,
    nativeSignerKeypairPath: tokenRecipientKeypairPath,
    userWallet: bobWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-confidential-token failed bob',
  });
  if (!failedBobDecrypt.success || failedBobDecrypt.decryptedValue !== '0') {
    throw new Error(
      `solana-confidential-token: expected failed-transfer recipient balance to remain 0, got ${formatUserDecryptOutcome(
        failedBobDecrypt,
      )}${failedBobDecrypt.decryptedValue !== undefined ? ` value=${failedBobDecrypt.decryptedValue}` : ''}`,
    );
  }

  const resetTransferFrom = runTokenCommand('token-reset');
  const mint10000ForTransferFrom = runTokenCommand('token-mint-to', [
    '--amount',
    '10000',
  ]);
  const approveProof = await buildTokenAmountProof(
    TOKEN_TRANSFER_INPUT_VALUE,
    aliceIdentity,
    aliceUserAddress,
    'solana-confidential-token approve',
  );
  const approve = runTokenCommand('token-approve-delegate', [
    '--delegate',
    bobPubkey,
    '--input-handle',
    approveProof.selectedHandle,
    '--input-proof',
    approveProof.inputProof,
  ]);
  const overAllowanceProof = await buildTokenAmountProof(
    '1338',
    bobIdentity,
    bobUserAddress,
    'solana-confidential-token transferFrom over-allowance',
  );
  const transferFromOver = runTokenCommand(
    'token-transfer-as-delegate',
    [
      '--source',
      alicePubkey,
      '--recipient',
      bobPubkey,
      '--input-handle',
      overAllowanceProof.selectedHandle,
      '--input-proof',
      overAllowanceProof.inputProof,
    ],
    tokenRecipientKeypairPath,
  );
  const overAliceBalanceQuery = runTokenCommand('token-balance', [
    '--owner',
    alicePubkey,
  ]);
  const overBobBalanceQuery = runTokenCommand('token-balance', [
    '--owner',
    bobPubkey,
  ]);
  await waitForResultBatch(
    'solana-confidential-token transferFrom over-allowance',
    resetTransferFrom,
    mint10000ForTransferFrom,
    approve,
    transferFromOver,
    overAliceBalanceQuery,
    overBobBalanceQuery,
  );
  const overAliceHandle = singleReturnedHandle(
    overAliceBalanceQuery,
    'solana-confidential-token over-allowance(alice)',
  );
  const overBobHandle = singleReturnedHandle(
    overBobBalanceQuery,
    'solana-confidential-token over-allowance(bob)',
  );
  const overAliceDecrypt = await executeUserDecryptRequest({
    handle: overAliceHandle,
    contractAddress,
    userIdentity: aliceIdentity,
    nativeContractIdentities: nativeContractIdentity ? [nativeContractIdentity] : null,
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
    userWallet: aliceWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-confidential-token over-allowance alice',
  });
  if (!overAliceDecrypt.success || overAliceDecrypt.decryptedValue !== '10000') {
    throw new Error(
      `solana-confidential-token: expected over-allowance Alice balance to remain 10000, got ${formatUserDecryptOutcome(
        overAliceDecrypt,
      )}${overAliceDecrypt.decryptedValue !== undefined ? ` value=${overAliceDecrypt.decryptedValue}` : ''}`,
    );
  }
  const overBobDecrypt = await executeUserDecryptRequest({
    handle: overBobHandle,
    contractAddress,
    userIdentity: bobIdentity,
    nativeContractIdentities: nativeContractIdentity ? [nativeContractIdentity] : null,
    nativeSignerKeypairPath: tokenRecipientKeypairPath,
    userWallet: bobWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-confidential-token over-allowance bob',
  });
  if (!overBobDecrypt.success || overBobDecrypt.decryptedValue !== '0') {
    throw new Error(
      `solana-confidential-token: expected over-allowance recipient balance to remain 0, got ${formatUserDecryptOutcome(
        overBobDecrypt,
      )}${overBobDecrypt.decryptedValue !== undefined ? ` value=${overBobDecrypt.decryptedValue}` : ''}`,
    );
  }

  const exactAllowanceProof = await buildTokenAmountProof(
    TOKEN_TRANSFER_INPUT_VALUE,
    bobIdentity,
    bobUserAddress,
    'solana-confidential-token transferFrom exact-allowance',
  );
  const transferFromExact = runTokenCommand(
    'token-transfer-as-delegate',
    [
      '--source',
      alicePubkey,
      '--recipient',
      bobPubkey,
      '--input-handle',
      exactAllowanceProof.selectedHandle,
      '--input-proof',
      exactAllowanceProof.inputProof,
    ],
    tokenRecipientKeypairPath,
  );
  const exactAliceBalanceQuery = runTokenCommand('token-balance', [
    '--owner',
    alicePubkey,
  ]);
  const exactBobBalanceQuery = runTokenCommand('token-balance', [
    '--owner',
    bobPubkey,
  ]);
  await waitForResultBatch(
    'solana-confidential-token transferFrom exact-allowance',
    transferFromExact,
    exactAliceBalanceQuery,
    exactBobBalanceQuery,
  );
  const exactAliceHandle = singleReturnedHandle(
    exactAliceBalanceQuery,
    'solana-confidential-token exact-allowance(alice)',
  );
  const exactBobHandle = singleReturnedHandle(
    exactBobBalanceQuery,
    'solana-confidential-token exact-allowance(bob)',
  );
  const exactAliceDecrypt = await executeUserDecryptRequest({
    handle: exactAliceHandle,
    contractAddress,
    userIdentity: aliceIdentity,
    nativeContractIdentities: nativeContractIdentity ? [nativeContractIdentity] : null,
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
    userWallet: aliceWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-confidential-token exact-allowance alice',
  });
  if (!exactAliceDecrypt.success || exactAliceDecrypt.decryptedValue !== '8663') {
    throw new Error(
      `solana-confidential-token: expected exact-allowance Alice balance to be 8663, got ${formatUserDecryptOutcome(
        exactAliceDecrypt,
      )}${exactAliceDecrypt.decryptedValue !== undefined ? ` value=${exactAliceDecrypt.decryptedValue}` : ''}`,
    );
  }
  const exactBobDecrypt = await executeUserDecryptRequest({
    handle: exactBobHandle,
    contractAddress,
    userIdentity: bobIdentity,
    nativeContractIdentities: nativeContractIdentity ? [nativeContractIdentity] : null,
    nativeSignerKeypairPath: tokenRecipientKeypairPath,
    userWallet: bobWallet,
    addresses,
    testSuiteEnv,
    label: 'solana-confidential-token exact-allowance bob',
  });
  if (!exactBobDecrypt.success || exactBobDecrypt.decryptedValue !== '1337') {
    throw new Error(
      `solana-confidential-token: expected exact-allowance recipient balance to be 1337, got ${formatUserDecryptOutcome(
        exactBobDecrypt,
      )}${exactBobDecrypt.decryptedValue !== undefined ? ` value=${exactBobDecrypt.decryptedValue}` : ''}`,
    );
  }

  console.log('Solana ConfidentialToken passed');
  console.log(`Mint handle: ${mintBalanceHandle}`);
  console.log(`Transfer input handle: ${transferInputHandle}`);
  console.log(`mintBalance=1000 totalSupply=${mintTotalSupply.total_supply}`);
  console.log(`aliceAfterTransfer=${aliceAfterDecrypt.decryptedValue}`);
  console.log(`recipientAfterTransfer=${bobAfterDecrypt.decryptedValue}`);
  console.log(`aliceAfterTransferFrom=${exactAliceDecrypt.decryptedValue}`);
  console.log(`recipientAfterTransferFrom=${exactBobDecrypt.decryptedValue}`);
}

function runLocalCliScenario(scenarioName, solanaEnv, extraArgs = []) {
  return runLocalCliJson(scenarioName, solanaEnv, extraArgs);
}

function runLocalCliJson(commandName, solanaEnv, extraArgs = []) {
  const addressesEnvPath = activeAddressesEnvPath();
  const args = [
    'run',
    '--manifest-path',
    localCliManifestPath,
    '--',
    commandName,
    '--addresses-env',
    addressesEnvPath,
    '--payer-keypair',
    anchorAuthorityKeypairPath,
    ...extraArgs,
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
      `failed to parse local-cli JSON for ${commandName}: ${error}\n${result.stdout}`,
    );
  }
}

async function buildGatewayBackedInputProof({
  relayerUrl,
  aclAddress,
  inputVerificationAddress,
  gatewayChainId,
  hostChainId,
  contractAddress,
  userAddress,
  contractIdentity,
  userIdentity,
  aclIdentity,
  value,
  type,
  coprocessorSigners,
  coprocessorThreshold,
}) {
  return buildGatewayBackedInputProofInline({
    relayerUrl,
    aclAddress,
    inputVerificationAddress,
    gatewayChainId: Number(gatewayChainId),
    hostChainId: Number(hostChainId),
    contractAddress,
    userAddress,
    contractIdentity,
    userIdentity,
    aclIdentity,
    value: String(value),
    type,
    coprocessorSigners,
    coprocessorThreshold: Number(coprocessorThreshold),
  });
}

function loadCoprocessorSigners(envValues) {
  const gatewayEnvPath = activeEnvPath('gateway-sc', legacyGatewayEnvPath);
  const hostEnvPath = activeEnvPath('host-sc', legacyHostEnvPath);
  const gatewayEnv = fs.existsSync(gatewayEnvPath)
    ? parseEnvFile(gatewayEnvPath)
    : {};
  const hostEnv = fs.existsSync(hostEnvPath) ? parseEnvFile(hostEnvPath) : {};
  const mergedEnv = { ...gatewayEnv, ...hostEnv, ...envValues };
  const count = Number(
    requiredEnvValue(mergedEnv, 'NUM_COPROCESSORS', 'Solana proof builder'),
  );
  const signers = [];
  for (let index = 0; index < count; index += 1) {
    signers.push(
      ethers.getAddress(
        requiredEnvValue(
          mergedEnv,
          `COPROCESSOR_SIGNER_ADDRESS_${index}`,
          'Solana proof builder',
        ),
      ),
    );
  }
  return signers;
}

function loadKmsSigners(envValues) {
  const gatewayEnvPath = activeEnvPath('gateway-sc', legacyGatewayEnvPath);
  const hostEnvPath = activeEnvPath('host-sc', legacyHostEnvPath);
  const gatewayEnv = fs.existsSync(gatewayEnvPath)
    ? parseEnvFile(gatewayEnvPath)
    : {};
  const hostEnv = fs.existsSync(hostEnvPath) ? parseEnvFile(hostEnvPath) : {};
  const mergedEnv = { ...gatewayEnv, ...hostEnv, ...envValues };
  const count = Number(
    requiredEnvValue(mergedEnv, 'NUM_KMS_NODES', 'Solana user decrypt'),
  );
  const signers = [];
  for (let index = 0; index < count; index += 1) {
    signers.push(
      ethers.getAddress(
        requiredEnvValue(
          mergedEnv,
          `KMS_SIGNER_ADDRESS_${index}`,
          'Solana user decrypt',
        ),
      ),
    );
  }
  return signers;
}

function signEd25519TypedData(eip712, privateKey) {
  const primaryType = requiredString(eip712.primaryType, 'primaryType');
  const digest = ethers.TypedDataEncoder.hash(
    eip712.domain,
    {
      [primaryType]: requiredValue(eip712.types?.[primaryType], `${primaryType} typed-data fields`),
    },
    eip712.message,
  );
  return bytesToHex(ed25519.sign(hexToBytes(digest), privateKey), true);
}

function loadSolanaKeypairBytes(keypairPath) {
  const raw = fs.readFileSync(keypairPath, 'utf8');
  const parsed = JSON.parse(raw);
  if (!Array.isArray(parsed)) {
    throw new Error(`invalid Solana keypair file ${keypairPath}: expected JSON byte array`);
  }
  return Uint8Array.from(parsed);
}

function loadSolanaEd25519PrivateKey(keypairPath) {
  const keypairBytes = loadSolanaKeypairBytes(keypairPath);
  if (keypairBytes.length < 32) {
    throw new Error(`invalid Solana keypair file ${keypairPath}: expected at least 32 bytes`);
  }
  return keypairBytes.slice(0, 32);
}

function loadSolanaEd25519PublicKey(keypairPath) {
  const keypairBytes = loadSolanaKeypairBytes(keypairPath);
  if (keypairBytes.length >= 64) {
    return keypairBytes.slice(32, 64);
  }
  return ed25519.getPublicKey(loadSolanaEd25519PrivateKey(keypairPath));
}

async function buildNativeSolanaAuthSigner(testSuiteEnv, keypairPath) {
  const verifierAddress = await ensureSolanaEd25519Verifier(testSuiteEnv);
  const publicKey = loadSolanaEd25519PublicKey(keypairPath);
  return bytesToHex(
    Uint8Array.from([
      ...hexToBytes(verifierAddress),
      ...publicKey,
    ]),
    true,
  );
}

async function ensureSolanaEd25519Verifier(testSuiteEnv) {
  if (!cachedSolanaEd25519VerifierPromise) {
    cachedSolanaEd25519VerifierPromise = deploySolanaEd25519Verifier(testSuiteEnv)
      .catch((error) => {
        cachedSolanaEd25519VerifierPromise = null;
        throw error;
      });
  }
  return cachedSolanaEd25519VerifierPromise;
}

async function deploySolanaEd25519Verifier(testSuiteEnv) {
  const gatewayEnv = parseEnvFile(activeEnvPath('gateway-sc', legacyGatewayEnvPath));
  const rpcUrl = normalizeStackHttpUrl(
    requiredEnvValue(testSuiteEnv, 'GATEWAY_RPC_URL', 'solana-native-auth'),
  ).toString();
  const privateKey = normalizeHexPrivateKey(
    requiredEnvValue(gatewayEnv, 'DEPLOYER_PRIVATE_KEY', 'solana-native-auth'),
  );
  const signer = new ethers.Wallet(privateKey, new ethers.JsonRpcProvider(rpcUrl));
  let nextNonce = await signer.getNonce('pending');

  const sha512 = await deployGatewayVerifierArtifact('Sha512', signer, {}, nextNonce);
  nextNonce += 1;
  const ed25519Pow = await deployGatewayVerifierArtifact(
    'Ed25519_pow',
    signer,
    {},
    nextNonce,
  );
  nextNonce += 1;
  const ed25519Library = await deployGatewayVerifierArtifact('Ed25519', signer, {
    Sha512: await sha512.getAddress(),
    Ed25519_pow: await ed25519Pow.getAddress(),
  }, nextNonce);
  nextNonce += 1;
  const verifier = await deployGatewayVerifierArtifact('SolanaEd25519Verifier', signer, {
    Ed25519: await ed25519Library.getAddress(),
  }, nextNonce);
  return verifier.getAddress();
}

function loadGatewayVerifierArtifact(name) {
  const artifactPath = path.join(
    repoRoot,
    'gateway-contracts/artifacts/contracts/verifiers',
    `${name}.sol`,
    `${name}.json`,
  );
  return JSON.parse(fs.readFileSync(artifactPath, 'utf8'));
}

function linkGatewayArtifactBytecode(artifact, libraries = {}) {
  if (typeof artifact?.bytecode !== 'string') {
    throw new Error('gateway verifier artifact is missing bytecode');
  }

  let bytecode = stripHexPrefix(artifact.bytecode);
  for (const [fileName, contracts] of Object.entries(artifact.linkReferences ?? {})) {
    for (const [contractName, references] of Object.entries(contracts)) {
      const address = libraries[contractName];
      if (!address) {
        throw new Error(`missing library address for ${fileName}:${contractName}`);
      }
      const normalizedAddress = stripHexPrefix(ethers.getAddress(address));
      for (const reference of references) {
        const start = reference.start * 2;
        const length = reference.length * 2;
        bytecode =
          `${bytecode.slice(0, start)}${normalizedAddress}${bytecode.slice(start + length)}`;
      }
    }
  }
  return `0x${bytecode}`;
}

async function deployGatewayVerifierArtifact(name, signer, libraries = {}, nonce) {
  const artifact = loadGatewayVerifierArtifact(name);
  const bytecode = linkGatewayArtifactBytecode(artifact, libraries);
  const factory = new ethers.ContractFactory(artifact.abi, bytecode, signer);
  const contract = await factory.deploy({ nonce });
  await contract.waitForDeployment();
  return contract;
}

function loadKmsContextId(envValues) {
  const gatewayEnvPath = activeEnvPath('gateway-sc', legacyGatewayEnvPath);
  const hostEnvPath = activeEnvPath('host-sc', legacyHostEnvPath);
  const gatewayEnv = fs.existsSync(gatewayEnvPath)
    ? parseEnvFile(gatewayEnvPath)
    : {};
  const hostEnv = fs.existsSync(hostEnvPath) ? parseEnvFile(hostEnvPath) : {};
  const mergedEnv = { ...envValues, ...gatewayEnv, ...hostEnv };
  return requiredEnvValue(mergedEnv, 'KMS_CONTEXT_ID', 'Solana user decrypt');
}

async function buildGatewayBackedInputProofInline({
  relayerUrl,
  aclAddress,
  inputVerificationAddress,
  gatewayChainId,
  hostChainId,
  contractAddress,
  userAddress,
  contractIdentity = null,
  userIdentity = null,
  aclIdentity = null,
  value,
  type,
  coprocessorSigners,
  coprocessorThreshold,
}) {
  const normalizedRelayerUrl = cleanUrl(relayerUrl);
  const normalizedContractIdentity = normalizeIdentityHex(contractIdentity);
  const normalizedUserIdentity = normalizeIdentityHex(userIdentity);
  const normalizedAclIdentity = normalizeIdentityHex(aclIdentity);
  const normalizedAclAddress = normalizedAclIdentity
    ? null
    : ethers.getAddress(requiredString(aclAddress, 'aclAddress'));
  const normalizedInputVerificationAddress = ethers.getAddress(inputVerificationAddress);
  const normalizedContractAddress = ethers.getAddress(
    requiredString(contractAddress, 'contractAddress'),
  );
  const normalizedUserAddress = ethers.getAddress(requiredString(userAddress, 'userAddress'));
  const inputProofExtraData =
    normalizedContractIdentity && normalizedUserIdentity
      ? buildInputProofExtraData(normalizedContractIdentity, normalizedUserIdentity)
      : '0x00';

  if (!Number.isFinite(gatewayChainId) || gatewayChainId <= 0) {
    throw new Error(`invalid gateway chain id: ${gatewayChainId}`);
  }
  if (!Number.isFinite(hostChainId) || hostChainId <= 0) {
    throw new Error(`invalid host chain id: ${hostChainId}`);
  }
  if (!Number.isFinite(coprocessorThreshold) || coprocessorThreshold <= 0) {
    throw new Error(`invalid coprocessor threshold: ${coprocessorThreshold}`);
  }
  if (!Array.isArray(coprocessorSigners) || coprocessorSigners.length === 0) {
    throw new Error('at least one coprocessor signer is required');
  }

  const keyMaterial = await getGatewayInputProofKeys(normalizedRelayerUrl);
  const { ciphertext, bits } = encryptGatewayInputValue({
    aclAddress: normalizedAclAddress,
    aclIdentity: normalizedAclIdentity,
    hostChainId,
    tfheCompactPublicKey: keyMaterial.publicKey,
    publicParams: keyMaterial.publicParams,
    contractAddress: normalizedContractAddress,
    userAddress: normalizedUserAddress,
    contractIdentity: normalizedContractIdentity,
    userIdentity: normalizedUserIdentity,
    inputType: type,
    value,
  });

  const payload = {
    ciphertextWithInputVerification: bytesToHex(ciphertext),
    contractChainId: `0x${hostChainId.toString(16)}`,
    extraData: inputProofExtraData,
    contractAddress: normalizedContractAddress,
    userAddress: normalizedUserAddress,
    ...(normalizedContractIdentity ? { contractId: normalizedContractIdentity } : {}),
    ...(normalizedUserIdentity ? { userId: normalizedUserIdentity } : {}),
  };

  const postPayload = await fetchJsonExpectOk(`${normalizedRelayerUrl}/v2/input-proof`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(payload),
  });
  const jobId = postPayload?.result?.jobId;
  if (!jobId) {
    throw new Error(
      `relayer input-proof response did not return a jobId: ${JSON.stringify(postPayload)}`,
    );
  }

  const statusPayload = await waitForGatewayInputProofJob(normalizedRelayerUrl, jobId);
  if (statusPayload?.status !== 'succeeded' || !statusPayload?.result?.accepted) {
    throw new Error(`input-proof job did not succeed: ${JSON.stringify(statusPayload)}`);
  }

  const handles = computeGatewayInputHandles(
    ciphertext,
    bits,
    normalizedAclIdentity ?? normalizedAclAddress,
    hostChainId,
    currentCiphertextVersion(),
  );
  const responseHandles = (statusPayload.result.handles ?? []).map((handle) =>
    hexToBytes(handle),
  );
  assertGatewayHandleListsMatch(handles, responseHandles);

  const signatures = statusPayload.result.signatures ?? [];
  verifyGatewayInputProofSignatures({
    handles,
    signatures,
    userAddress: normalizedUserAddress,
    contractAddress: normalizedContractAddress,
    contractIdentity: normalizedContractIdentity,
    userIdentity: normalizedUserIdentity,
    hostChainId,
    extraData: inputProofExtraData,
    gatewayChainId,
    inputVerificationAddress: normalizedInputVerificationAddress,
    coprocessorSigners,
    threshold: coprocessorThreshold,
  });

  let inputProofHex = encodeSingleByte(handles.length);
  inputProofHex += encodeSingleByte(signatures.length);
  for (const handle of handles) {
    inputProofHex += bytesToHex(handle);
  }
  for (const signature of signatures) {
    inputProofHex += stripHexPrefix(signature);
  }
  inputProofHex += stripHexPrefix(inputProofExtraData);

  return {
    handles: handles.map((handle) => bytesToHex(handle, true)),
    selectedHandle: bytesToHex(handles[0], true),
    inputProof: `0x${inputProofHex}`,
    jobId,
  };
}

async function getGatewayInputProofKeys(relayerUrl) {
  const data = await fetchJsonExpectOk(`${relayerUrl}/v2/keyurl`);
  const fheKeyInfo = data?.response?.fheKeyInfo ?? data?.response?.fhe_key_info;
  const publicKeyRecord = fheKeyInfo?.[0]?.fhePublicKey ?? fheKeyInfo?.[0]?.fhe_public_key;
  const crsRecord = data?.response?.crs?.['2048'];
  const publicKeyUrl = normalizeGatewayInputDownloadUrl(publicKeyRecord?.urls?.[0]);
  const publicKeyId = publicKeyRecord?.dataId ?? publicKeyRecord?.data_id;
  const publicParamsUrl = normalizeGatewayInputDownloadUrl(crsRecord?.urls?.[0]);
  const publicParamsId = crsRecord?.dataId ?? crsRecord?.data_id;

  if (!publicKeyUrl || !publicKeyId || !publicParamsUrl || !publicParamsId) {
    throw new Error(`unexpected /v2/keyurl response: ${JSON.stringify(data)}`);
  }

  const [publicKeyBytes, publicParamsBytes] = await Promise.all([
    fetchBinaryExpectOk(publicKeyUrl),
    fetchBinaryExpectOk(publicParamsUrl),
  ]);

  return {
    publicKey: TFHE.TfheCompactPublicKey.safe_deserialize(
      publicKeyBytes,
      SERIALIZED_SIZE_LIMIT_PK,
    ),
    publicParams: {
      2048: {
        publicParams: TFHE.CompactPkeCrs.safe_deserialize(
          publicParamsBytes,
          SERIALIZED_SIZE_LIMIT_CRS,
        ),
        publicParamsId,
      },
    },
    publicKeyId,
  };
}

function encryptGatewayInputValue({
  aclAddress,
  aclIdentity,
  hostChainId,
  tfheCompactPublicKey,
  publicParams,
  contractAddress,
  userAddress,
  contractIdentity,
  userIdentity,
  inputType,
  value,
}) {
  const builder = TFHE.CompactCiphertextList.builder(tfheCompactPublicKey);
  const bits = [];
  addGatewayEncryptedValue(builder, bits, inputType, value);

  const totalBits = bits.reduce((sum, bitWidth) => sum + bitWidth, 0);
  if (totalBits > 2048) {
    throw new Error(`too many encrypted bits for one input proof: ${totalBits}`);
  }

  const auxData =
    contractIdentity && userIdentity && aclIdentity
      ? buildGatewayInputAuxDataV1(contractIdentity, userIdentity, aclIdentity, hostChainId)
      : buildGatewayInputAuxData(contractAddress, userAddress, aclAddress, hostChainId);
  const encrypted = builder.build_with_proof_packed(
    publicParams[2048].publicParams,
    auxData,
    TFHE.ZkComputeLoad.Verify,
  );

  return {
    ciphertext: encrypted.safe_serialize(SERIALIZED_SIZE_LIMIT_CIPHERTEXT),
    bits,
  };
}

function addGatewayEncryptedValue(builder, bits, inputType, rawValue) {
  switch (inputType) {
    case 'bool': {
      const value =
        rawValue === 'true' ? true : rawValue === 'false' ? false : BigInt(rawValue) === 1n;
      builder.push_boolean(value);
      bits.push(2);
      return;
    }
    case 'uint8':
      assertGatewayUintRange(rawValue, 8);
      builder.push_u8(Number(rawValue));
      bits.push(8);
      return;
    case 'uint16':
      assertGatewayUintRange(rawValue, 16);
      builder.push_u16(Number(rawValue));
      bits.push(16);
      return;
    case 'uint32':
      assertGatewayUintRange(rawValue, 32);
      builder.push_u32(Number(rawValue));
      bits.push(32);
      return;
    case 'uint64':
      assertGatewayUintRange(rawValue, 64);
      builder.push_u64(BigInt(rawValue));
      bits.push(64);
      return;
    case 'uint128':
      assertGatewayUintRange(rawValue, 128);
      builder.push_u128(BigInt(rawValue));
      bits.push(128);
      return;
    case 'address': {
      const normalized = ethers.getAddress(rawValue);
      builder.push_u160(BigInt(normalized));
      bits.push(160);
      return;
    }
    case 'uint256':
      assertGatewayUintRange(rawValue, 256);
      builder.push_u256(BigInt(rawValue));
      bits.push(256);
      return;
    default:
      throw new Error(`unsupported gateway input type ${inputType}`);
  }
}

function assertGatewayUintRange(rawValue, width) {
  const value = BigInt(rawValue);
  if (value < 0n || value >= 1n << BigInt(width)) {
    throw new Error(`value ${rawValue} does not fit in ${width} bits`);
  }
}

function buildGatewayInputAuxData(contractAddress, userAddress, aclAddress, hostChainId) {
  const contractBytes = hexToBytes(contractAddress);
  const userBytes = hexToBytes(userAddress);
  const aclBytes = hexToBytes(aclAddress);
  const chainBytes = Uint8Array.from(
    Buffer.from(hostChainId.toString(16).padStart(64, '0'), 'hex'),
  );
  const auxData = new Uint8Array(
    contractBytes.length + userBytes.length + aclBytes.length + chainBytes.length,
  );
  auxData.set(contractBytes, 0);
  auxData.set(userBytes, 20);
  auxData.set(aclBytes, 40);
  auxData.set(chainBytes, auxData.length - chainBytes.length);
  return auxData;
}

function buildGatewayInputAuxDataV1(contractIdentity, userIdentity, aclIdentity, hostChainId) {
  const contractBytes = hexToBytes(contractIdentity);
  const userBytes = hexToBytes(userIdentity);
  const aclBytes = hexToBytes(aclIdentity);
  const chainBytes = Uint8Array.from(
    Buffer.from(hostChainId.toString(16).padStart(64, '0'), 'hex'),
  );
  const auxData = new Uint8Array(
    contractBytes.length + userBytes.length + aclBytes.length + chainBytes.length,
  );
  auxData.set(contractBytes, 0);
  auxData.set(userBytes, 32);
  auxData.set(aclBytes, 64);
  auxData.set(chainBytes, 96);
  return auxData;
}

function buildInputProofExtraData(contractIdentity, userIdentity) {
  return `0x${Buffer.concat([
    Buffer.from([INPUT_PROOF_EXTRA_DATA_VERSION]),
    Buffer.from(hexToBytes(contractIdentity)),
    Buffer.from(hexToBytes(userIdentity)),
  ]).toString('hex')}`;
}

function buildDecryptionExtraData(contextId, identities, authSigner = null) {
  if (!Array.isArray(identities) || identities.length === 0) {
    throw new Error('decryption extraData requires at least one identity');
  }
  const normalizedContextId = BigInt(contextId);
  const parts = [
    Buffer.from([DECRYPTION_EXTRA_DATA_VERSION]),
    Buffer.from(
      normalizedContextId.toString(16).padStart(64, '0'),
      'hex',
    ),
    Buffer.from([identities.length]),
  ];
  for (const identity of identities) {
    parts.push(Buffer.from(hexToBytes(identity)));
  }
  if (authSigner) {
    const authSignerBytes = hexToBytes(authSigner);
    if (authSignerBytes.length < 20 || authSignerBytes.length > 255) {
      throw new Error(`invalid authSigner length: ${authSignerBytes.length}`);
    }
    parts.push(Buffer.from([authSignerBytes.length]));
    parts.push(Buffer.from(authSignerBytes));
  }
  return `0x${Buffer.concat(parts).toString('hex')}`;
}

function normalizeIdentityHex(value) {
  if (!value) {
    return null;
  }
  const normalized = ensure0xHex(value);
  const bytes = hexToBytes(normalized);
  if (bytes.length !== 32) {
    throw new Error(`expected 32-byte host identity, got ${bytes.length} bytes`);
  }
  return normalized.toLowerCase();
}

function buildSolanaContractIdentityMap(identities) {
  const entries = [
    [
      identities?.test_input_contract_evm_address,
      identities?.test_input_state_pda_hex,
    ],
    [
      identities?.confidential_token_contract_evm_address,
      identities?.confidential_token_state_pda_hex,
    ],
  ].filter(([address, contractIdentity]) => address && contractIdentity);
  return new Map(
    entries.map(([address, contractIdentity]) => [
      ethers.getAddress(address),
      normalizeIdentityHex(`0x${stripHexPrefix(contractIdentity)}`),
    ]),
  );
}

function getSolanaContractIdentityByAddress(identityMap, address) {
  if (!identityMap || !address) {
    return undefined;
  }
  return identityMap.get(ethers.getAddress(address));
}

function requiredString(value, fieldName) {
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`missing ${fieldName}`);
  }
  return value;
}

function requiredValue(value, fieldName) {
  if (value === undefined || value === null) {
    throw new Error(`missing ${fieldName}`);
  }
  return value;
}

function currentCiphertextVersion() {
  return 0;
}

function computeGatewayInputHandles(ciphertextWithZKProof, bitwidths, aclAddress, chainId, ciphertextVersion) {
  const blobHash = createHash('keccak256')
    .update(Buffer.from(RAW_CT_HASH_DOMAIN_SEPARATOR))
    .update(Buffer.from(ciphertextWithZKProof))
    .digest();
  const aclAddressBytes = Buffer.from(hexToBytes(aclAddress));
  const chainIdHex = chainId.toString(16).padStart(64, '0');
  const chainIdBytes = Buffer.from(chainIdHex, 'hex');

  return bitwidths.map((bitwidth, encryptionIndex) => {
    const encryptionType = INPUT_ENCRYPTION_TYPES[bitwidth];
    const handleHash = createHash('keccak256')
      .update(Buffer.from(HANDLE_HASH_DOMAIN_SEPARATOR))
      .update(blobHash)
      .update(Buffer.from([encryptionIndex]))
      .update(aclAddressBytes)
      .update(chainIdBytes)
      .digest();
    const handle = new Uint8Array(32);
    handle.set(handleHash, 0);
    if (BigInt(chainId) > MAX_UINT64) {
      throw new Error(`chain id ${chainId} exceeds the supported 8-byte handle field`);
    }
    handle[21] = encryptionIndex;
    handle.set(hexToBytes(chainIdHex).slice(24, 32), 22);
    handle[30] = encryptionType;
    handle[31] = ciphertextVersion;
    return handle;
  });
}

function assertGatewayHandleListsMatch(expectedHandles, responseHandles) {
  if (expectedHandles.length !== responseHandles.length) {
    throw new Error(
      `relayer returned ${responseHandles.length} handles, expected ${expectedHandles.length}`,
    );
  }
  for (let index = 0; index < expectedHandles.length; index += 1) {
    const expected = bytesToHex(expectedHandles[index], true);
    const actual = bytesToHex(responseHandles[index], true);
    if (expected !== actual) {
      throw new Error(`handle mismatch at index ${index}: expected ${expected}, got ${actual}`);
    }
  }
}

function verifyGatewayInputProofSignatures({
  handles,
  signatures,
  userAddress,
  contractAddress,
  contractIdentity,
  userIdentity,
  hostChainId,
  extraData,
  gatewayChainId,
  inputVerificationAddress,
  coprocessorSigners,
  threshold,
}) {
  const domain = {
    name: 'InputVerification',
    version: '1',
    chainId: gatewayChainId,
    verifyingContract: inputVerificationAddress,
  };
  const types = {
    CiphertextVerification: [
      { name: 'ctHandles', type: 'bytes32[]' },
      { name: 'userAddress', type: 'address' },
      { name: 'contractAddress', type: 'address' },
      { name: 'contractChainId', type: 'uint256' },
      { name: 'extraData', type: 'bytes' },
    ],
  };
  const recoveredAddresses = signatures.map((signature) =>
    ethers.verifyTypedData(
      domain,
      types,
      {
        ctHandles: handles,
        userAddress,
        contractAddress,
        contractChainId: hostChainId,
        extraData,
      },
      ensure0xHex(signature),
    ),
  );

  const normalizedExpected = new Set(coprocessorSigners.map((address) => ethers.getAddress(address)));
  const normalizedRecovered = recoveredAddresses.map((address) => ethers.getAddress(address));
  const uniqueRecovered = new Set(normalizedRecovered);
  if (uniqueRecovered.size !== normalizedRecovered.length) {
    throw new Error('duplicate coprocessor signer recovered from input-proof response');
  }
  for (const address of normalizedRecovered) {
    if (!normalizedExpected.has(address)) {
      throw new Error(`unexpected coprocessor signer in input-proof response: ${address}`);
    }
  }
  if (normalizedRecovered.length < threshold) {
    throw new Error(
      `coprocessor signer threshold not reached: got ${normalizedRecovered.length}, need ${threshold}`,
    );
  }
}

function normalizeGatewayInputDownloadUrl(url) {
  if (!url) {
    return url;
  }
  const parsed = normalizeStackHttpUrl(url);
  return parsed.toString();
}

async function latestGatewayBlockTimestamp(testSuiteEnv, label) {
  const gatewayRpcUrl = normalizeStackHttpUrl(
    requiredEnvValue(testSuiteEnv, 'GATEWAY_RPC_URL', label),
  ).toString();
  const payload = await fetchJsonExpectOk(gatewayRpcUrl, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({
      jsonrpc: '2.0',
      id: 1,
      method: 'eth_getBlockByNumber',
      params: ['latest', false],
    }),
  });
  const blockTimestampHex = payload?.result?.timestamp;
  if (typeof blockTimestampHex !== 'string') {
    throw new Error(
      `${label}: gateway RPC eth_getBlockByNumber returned no timestamp: ${JSON.stringify(payload)}`,
    );
  }
  return Number(BigInt(blockTimestampHex));
}

function normalizeStackHttpUrl(url) {
  const parsed = new URL(url);
  if (
    parsed.hostname === '0.0.0.0' ||
    parsed.hostname === 'minio' ||
    parsed.hostname === 'gateway-node' ||
    parsed.hostname === 'host-node' ||
    parsed.hostname === 'host-node-solana'
  ) {
    parsed.hostname = '127.0.0.1';
  }
  return parsed;
}

async function fetchJsonExpectOk(url, init) {
  const response = await fetch(url, init);
  const text = await response.text();
  if (!response.ok) {
    throw new Error(`request failed for ${url}: ${response.status} ${text}`);
  }
  return JSON.parse(text);
}

async function fetchBinaryExpectOk(url) {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`binary download failed for ${url}: ${response.status}`);
  }
  return new Uint8Array(await response.arrayBuffer());
}

async function waitForGatewayInputProofJob(relayerUrl, jobId) {
  const started = Date.now();
  while (Date.now() - started < 120_000) {
    const response = await fetch(`${relayerUrl}/v2/input-proof/${jobId}`);
    const text = await response.text();
    if (!response.ok) {
      throw new Error(`input-proof status failed for ${jobId}: ${response.status} ${text}`);
    }
    const payload = JSON.parse(text);
    if (payload.status === 'queued') {
      await sleep(1000);
      continue;
    }
    return payload;
  }
  throw new Error(`timed out waiting for input-proof job ${jobId}`);
}

function encodeSingleByte(value) {
  const hex = Number(value).toString(16);
  return hex.length % 2 === 0 ? hex : `0${hex}`;
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
    let payload = null;
    if (text) {
      try {
        payload = JSON.parse(text);
      } catch (error) {
        throw new Error(
          `public decrypt status check returned non-JSON payload for ${jobId}: ${error}\n${text}`,
        );
      }
    }
    if (response.status === 202 && payload?.status === 'queued') {
      await sleep(1000);
      continue;
    }
    return { httpStatus: response.status, payload, responseText: text };
  }
  throw new Error(`timed out waiting for public decrypt job ${jobId}`);
}

async function waitForUserDecryptJob(jobId) {
  const started = Date.now();
  while (Date.now() - started < 120_000) {
    const response = await fetch(`${RELAYER_URL}/v2/user-decrypt/${jobId}`);
    const text = await response.text();
    let payload = null;
    if (text) {
      try {
        payload = JSON.parse(text);
      } catch (error) {
        throw new Error(
          `user decrypt status check returned non-JSON payload for ${jobId}: ${error}\n${text}`,
        );
      }
    }

    if (response.status === 202 && payload?.status === 'queued') {
      await sleep(1000);
      continue;
    }
    return { httpStatus: response.status, payload };
  }
  throw new Error(`timed out waiting for user decrypt job ${jobId}`);
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

function requiredEnvValue(values, field, label) {
  const value = values?.[field];
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`${label}: missing ${field}`);
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

function ensure0xHex(value) {
  const normalized = String(value);
  return normalized.startsWith('0x') ? normalized : `0x${normalized}`;
}

function bytesToHex(bytes, withPrefix = false) {
  return `${withPrefix ? '0x' : ''}${Buffer.from(bytes).toString('hex')}`;
}

function cleanUrl(url) {
  const normalized = String(url);
  return normalized.endsWith('/') ? normalized.slice(0, -1) : normalized;
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

function scenarioExpectedClearValues(scenario) {
  const handles = Array.isArray(scenario.final_handles) ? scenario.final_handles : [];
  const expected = Array.isArray(scenario.expected) ? scenario.expected : [];
  if (handles.length !== expected.length) {
    throw new Error(
      `scenario ${scenario.scenario ?? 'unknown'} expected ${expected.length} clear values for ${handles.length} handles`,
    );
  }

  const clearValues = {};
  for (let index = 0; index < handles.length; index += 1) {
    clearValues[handles[index]] = normalizeScenarioExpectedValue(expected[index]);
  }
  return clearValues;
}

function normalizeScenarioExpectedValue(expected) {
  const outputType = String(expected?.output_type ?? '').toLowerCase();
  const value = String(expected?.value ?? '');

  if (outputType === 'bool') {
    if (value === 'true') {
      return true;
    }
    if (value === 'false') {
      return false;
    }
    throw new Error(`unexpected bool scenario value: ${value}`);
  }

  if (outputType === 'address') {
    return value.toLowerCase();
  }

  return value;
}

function decodePublicDecryptClearValues(handles, decryptedValueHex) {
  if (!Array.isArray(handles) || handles.length === 0) {
    throw new Error('public decrypt decode requires at least one handle');
  }
  if (typeof decryptedValueHex !== 'string') {
    throw new Error('public decrypt response missing decryptedValue');
  }

  const normalized = decryptedValueHex.startsWith('0x')
    ? decryptedValueHex.slice(2)
    : decryptedValueHex;
  if (normalized.length % 64 !== 0) {
    throw new Error(
      `public decrypt decryptedValue length must be a multiple of 32 bytes, got ${normalized.length / 2} bytes`,
    );
  }

  const words = [];
  for (let offset = 0; offset < normalized.length; offset += 64) {
    words.push(normalized.slice(offset, offset + 64));
  }
  if (words.length < handles.length) {
    throw new Error(
      `public decrypt response returned ${words.length} words for ${handles.length} handles`,
    );
  }

  const clearValues = {};
  for (let index = 0; index < handles.length; index += 1) {
    clearValues[handles[index]] = decodePublicDecryptWord(handles[index], words[index]);
  }
  return clearValues;
}

function decodePublicDecryptWord(handle, wordHex) {
  const typeDiscriminant = extractHandleTypeDiscriminant(handle);
  const bytes = hexToBytes(wordHex);

  switch (typeDiscriminant) {
    case 0:
      return bytes[31] === 1;
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
    case 8:
      return BigInt(`0x${wordHex}`).toString(10);
    case 7:
      return `0x${Buffer.from(bytes.slice(12, 32)).toString('hex')}`.toLowerCase();
    default:
      throw new Error(
        `unsupported public decrypt handle type ${typeDiscriminant} for handle ${handle}`,
      );
  }
}

function extractHandleTypeDiscriminant(handle) {
  const normalized = handle.startsWith('0x') ? handle.slice(2) : handle;
  if (normalized.length < 4) {
    throw new Error(`handle too short to decode type: ${handle}`);
  }
  return Number.parseInt(normalized.slice(-4, -2), 16);
}

function hexToBytes(hexValue) {
  const normalized = hexValue.startsWith('0x') ? hexValue.slice(2) : hexValue;
  if (normalized.length % 2 !== 0) {
    throw new Error(`hex string must have even length: ${hexValue}`);
  }
  return Uint8Array.from(Buffer.from(normalized, 'hex'));
}

function normalizeHexPrivateKey(value) {
  return value.startsWith('0x') ? value : `0x${value}`;
}

function bytesToDecimalString(bytes) {
  if (!bytes || bytes.length === 0) {
    return '0';
  }
  return BigInt(`0x${Buffer.from(bytes).toString('hex')}`).toString(10);
}

function decodeUserDecryptBytes(handle, bytes) {
  const typeDiscriminant = extractHandleTypeDiscriminant(handle);
  const normalizedBytes = bytes instanceof Uint8Array ? bytes : Uint8Array.from(bytes ?? []);

  switch (typeDiscriminant) {
    case 0:
      return normalizedBytes.some((value) => value !== 0);
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
    case 8:
      return bytesToDecimalString(normalizedBytes);
    case 7: {
      const addressBytes =
        normalizedBytes.length >= 20
          ? normalizedBytes.slice(normalizedBytes.length - 20)
          : normalizedBytes;
      return `0x${Buffer.from(addressBytes).toString('hex')}`.toLowerCase();
    }
    default:
      throw new Error(
        `unsupported user decrypt handle type ${typeDiscriminant} for handle ${handle}`,
      );
  }
}

function decryptUserDecryptResult({
  responsePayload,
  handles,
  userAddress,
  publicKey,
  privateKey,
  gatewayChainId,
  verifyingContract,
  kmsSigners,
  extraData,
}) {
  const aggregatedResponse = responsePayload?.result?.result;
  if (!Array.isArray(aggregatedResponse) || aggregatedResponse.length === 0) {
    throw new Error(
      `user decrypt response payload does not contain re-encrypted shares: ${JSON.stringify(
        responsePayload,
      )}`,
    );
  }
  const normalizedAggregatedResponse = aggregatedResponse.map((share) => ({
    payload:
      typeof share?.payload === 'string'
        ? share.payload.replace(/^0x/i, '')
        : share?.payload,
    signature:
      typeof share?.signature === 'string'
        ? share.signature.replace(/^0x/i, '')
        : share?.signature,
    extra_data:
      typeof share?.extraData === 'string'
        ? share.extraData.replace(/^0x/i, '')
        : typeof share?.extra_data === 'string'
          ? share.extra_data.replace(/^0x/i, '')
          : undefined,
  }));

  verifyUserDecryptResponseSignatures({
    aggregatedResponse,
    handles,
    publicKey,
    gatewayChainId,
    verifyingContract,
    kmsSigners,
    extraData,
  });

  const publicKeyBytes = TKMS.u8vec_to_ml_kem_pke_pk(hexToBytes(publicKey));
  const privateKeyBytes = TKMS.u8vec_to_ml_kem_pke_sk(hexToBytes(privateKey));
  const indexedKmsSigners = kmsSigners.map((signer, index) =>
    TKMS.new_server_id_addr(index + 1, signer),
  );
  const client = TKMS.new_client(indexedKmsSigners, userAddress, 'default');
  const decrypted = TKMS.process_user_decryption_resp_from_js(
    client,
    null,
    null,
    normalizedAggregatedResponse,
    publicKeyBytes,
    privateKeyBytes,
    false,
  );
  const clearValues = {};
  for (let index = 0; index < handles.length; index += 1) {
    clearValues[handles[index]] = decodeUserDecryptBytes(
      handles[index],
      decrypted[index]?.bytes ?? [],
    );
  }
  return clearValues;
}

function verifyUserDecryptResponseSignatures({
  aggregatedResponse,
  handles,
  publicKey,
  gatewayChainId,
  verifyingContract,
  kmsSigners,
  extraData,
}) {
  const domain = {
    name: 'Decryption',
    version: '1',
    chainId: Number(gatewayChainId),
    verifyingContract: ethers.getAddress(verifyingContract),
  };
  const types = {
    UserDecryptResponseVerification: [
      { name: 'publicKey', type: 'bytes' },
      { name: 'ctHandles', type: 'bytes32[]' },
      { name: 'userDecryptedShare', type: 'bytes' },
      { name: 'extraData', type: 'bytes' },
    ],
  };
  const normalizedExpected = new Set(kmsSigners.map((signer) => ethers.getAddress(signer)));
  const normalizedRecovered = aggregatedResponse.map((share) => {
    const sharePayload = requiredString(share?.payload, 'user decrypt share payload');
    const shareSignature = ensure0xHex(requiredString(share?.signature, 'user decrypt share signature'));
    const shareExtraData =
      typeof share?.extraData === 'string'
        ? share.extraData
        : typeof share?.extra_data === 'string'
          ? share.extra_data
          : extraData;
    return ethers.getAddress(
      ethers.verifyTypedData(
        domain,
        types,
        {
          publicKey: ensure0xHex(requiredString(publicKey, 'user decrypt public key')),
          ctHandles: handles.map((handle) => ensure0xHex(requiredString(handle, 'user decrypt handle'))),
          userDecryptedShare: ensure0xHex(sharePayload),
          extraData: ensure0xHex(shareExtraData ?? '0x'),
        },
        shareSignature,
      ),
    );
  });

  const uniqueRecovered = new Set(normalizedRecovered);
  if (uniqueRecovered.size !== normalizedRecovered.length) {
    throw new Error('duplicate KMS signer recovered from user decrypt response');
  }
  for (const signer of normalizedRecovered) {
    if (!normalizedExpected.has(signer)) {
      throw new Error(`unexpected KMS signer in user decrypt response: ${signer}`);
    }
  }
  if (normalizedRecovered.length === 0) {
    throw new Error('user decrypt response did not contain any signed shares');
  }
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

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
