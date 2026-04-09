/// <reference types="bun-types" />

import { spawnSync } from 'node:child_process';
import process from 'node:process';

import {
  DB_CONTAINER,
  LOCAL_CLI_MANIFEST_PATH as localCliManifestPath,
  REPO_ROOT as repoRoot,
  SOLANA_RPC_URL,
  SOLANA_E2E_COMMITMENT,
  INPUT_PROOF_U64_VALUE,
  ADD42_INPUT_VALUE,
  TOKEN_TRANSFER_INPUT_VALUE,
} from './solana.config';
import {
  setupSolanaContext,
  anchorAuthorityKeypairPath,
  tokenRecipientKeypairPath,
  type EnvRecord,
} from './solana.fixture';
import {
  buildGatewayBackedInputProof,
  loadCoprocessorSigners,
  requiredEnvValue,
  sleep,
  stripHexPrefix,
} from './solana.proof-helpers';
import {
  executePublicDecryptRequest,
  executeUserDecryptRequest,
  formatPublicDecryptOutcome,
  formatUserDecryptOutcome,
  includesPublicDecryptErrorMessage,
  isExpiredUserDecryptRejection,
  isUserDecryptAclRejection,
} from './solana.relayer-helpers';

const verbose = process.argv.includes('--verbose') || process.env.VERBOSE === '1';

let testAddresses: EnvRecord;
let testLocalnetEnv: EnvRecord;
let testSuiteEnv: EnvRecord;
let testChainId: number;
let testAddressesEnvPath: string;

type BunTestApi = typeof import('bun:test');

function registerSolanaE2eTests({ beforeAll, describe, it }: BunTestApi) {
  describe('Solana e2e', () => {
    beforeAll(async () => {
      const ctx = await setupSolanaContext();
      testAddresses = ctx.addresses;
      testLocalnetEnv = ctx.localnetEnv;
      testSuiteEnv = ctx.testSuiteEnv;
      testChainId = ctx.chainId;
      testAddressesEnvPath = ctx.addressesEnvPath;

      sqlScalar('SELECT 1');
      if (!(await solanaRpcHealthy())) {
        throw new Error(`Solana RPC ${SOLANA_RPC_URL} is not healthy`);
      }
    });

    it('solana-input-proof', async () => {
      await resetChainState(testChainId);
      await runInputProofCase(testAddresses, testLocalnetEnv, testSuiteEnv);
    }, 300_000);

    it('solana-user-decryption', async () => {
      await resetChainState(testChainId);
      await ensureRelayerReachable();
      await runUserDecryptionCase(testAddresses, testLocalnetEnv, testSuiteEnv);
    }, 300_000);

    it('solana-public-decrypt-http-ebool', async () => {
      await resetChainState(testChainId);
      await runPublicDecryptCase(testAddresses, testLocalnetEnv, 'scenario-public-ebool');
    }, 300_000);

    it('solana-public-decrypt-http-mixed', async () => {
      await resetChainState(testChainId);
      await runPublicDecryptCase(testAddresses, testLocalnetEnv, 'scenario-public-mixed');
    }, 300_000);

    it('solana-confidential-token', async () => {
      await resetChainState(testChainId);
      await runConfidentialTokenCase(testAddresses, testLocalnetEnv, testSuiteEnv);
    }, 300_000);
  });
}

if (typeof Bun !== 'undefined') {
  registerSolanaE2eTests(require('bun:test') as BunTestApi);
}

async function runInputProofCase(addresses: EnvRecord, solanaEnv: EnvRecord, testSuiteEnv: EnvRecord) {
  await ensureRelayerReachable();
  const identities = runLocalCliJson('runtime-identities', solanaEnv);
  const contractIdentity = requiredScenarioField(
    identities,
    'test_input_contract_id_hex',
    'solana-input-proof',
  );
  const userIdentity = requiredScenarioField(
    identities,
    'payer_user_id_hex',
    'solana-input-proof',
  );
  const aclIdentity = requiredScenarioField(
    identities,
    'host_program_id_hex',
    'solana-input-proof',
  );
  const chainId = Number(addresses.SOLANA_HOST_CHAIN_ID);

  const proofBundle = await buildGatewayBackedInputProof({
    relayerUrl: requiredEnvValue(testSuiteEnv, 'RELAYER_URL', 'solana-input-proof'),
    inputVerificationAddress: requiredEnvValue(
      testSuiteEnv,
      'INPUT_VERIFICATION_ADDRESS',
      'solana-input-proof',
    ),
    gatewayChainId: requiredEnvValue(testSuiteEnv, 'CHAIN_ID_GATEWAY', 'solana-input-proof'),
    hostChainId: addresses.SOLANA_HOST_CHAIN_ID,
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
  ]);
  await waitForSolanaTransactionsCommitted(
    scenarioSignatures(scenario),
    SOLANA_E2E_COMMITMENT,
  );
  await waitForSolanaListenerCaughtUp(
    chainId,
    await maxCommittedSlot(scenarioSignatures(scenario), SOLANA_E2E_COMMITMENT),
  );

  const add42ProofBundle = await buildGatewayBackedInputProof({
    relayerUrl: requiredEnvValue(testSuiteEnv, 'RELAYER_URL', 'solana-input-proof'),
    inputVerificationAddress: requiredEnvValue(
      testSuiteEnv,
      'INPUT_VERIFICATION_ADDRESS',
      'solana-input-proof',
    ),
    gatewayChainId: requiredEnvValue(testSuiteEnv, 'CHAIN_ID_GATEWAY', 'solana-input-proof'),
    hostChainId: addresses.SOLANA_HOST_CHAIN_ID,
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
  ]);
  await waitForSolanaTransactionsCommitted(
    scenarioSignatures(add42Scenario),
    SOLANA_E2E_COMMITMENT,
  );
  await waitForSolanaListenerCaughtUp(
    chainId,
    await maxCommittedSlot(scenarioSignatures(add42Scenario), SOLANA_E2E_COMMITMENT),
  );

  const resultHandle = scenarioFinalHandles(add42Scenario)[0];
  if (!resultHandle) {
    throw new Error('solana-input-proof: add42 scenario did not return a final handle');
  }
  await waitForCiphertextPresent(resultHandle, 'solana-input-proof add42');

  const userDecryptResult = await executeUserDecryptRequest({
    handle: resultHandle,
    contractId: contractIdentity,
    userIdentity,
    contractIds: [contractIdentity],
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
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
  console.log(`requestUint64NonTrivial signatures: ${scenarioSignatures(scenario).join(', ')}`);
  console.log(`add42ToInput64 signatures: ${scenarioSignatures(add42Scenario).join(', ')}`);
  console.log(
    `add42ToInput64 handle=${resultHandle} userDecrypt=49 publicDecrypt=49`,
  );
}

async function runUserDecryptionCase(addresses: EnvRecord, solanaEnv: EnvRecord, testSuiteEnv: EnvRecord) {
  const identities = runLocalCliJson('runtime-identities', solanaEnv);
  const scenario = runLocalCliScenario('scenario-user-decrypt', solanaEnv);
  await waitForSolanaTransactionsCommitted(
    scenarioSignatures(scenario),
    SOLANA_E2E_COMMITMENT,
  );
  await waitForSolanaListenerCaughtUp(
    Number(addresses.SOLANA_HOST_CHAIN_ID),
    await maxCommittedSlot(scenarioSignatures(scenario), SOLANA_E2E_COMMITMENT),
  );

  const chainId = Number(addresses.SOLANA_HOST_CHAIN_ID);
  const contractId = requiredScenarioField(
    scenario,
    'contract_id_hex',
    'solana-user-decryption',
  );
  const userIdentity = requiredScenarioField(
    identities,
    'payer_user_id_hex',
    'solana-user-decryption',
  );
  const otherUserIdentity = requiredScenarioField(
    identities,
    'token_recipient_user_id_hex',
    'solana-user-decryption',
  );
  const handles = scenarioFinalHandles(scenario);
  const expectedClearValues = scenarioExpectedClearValues(scenario);
  if (handles.length !== 8) {
    throw new Error(
      `solana-user-decryption: expected 8 fixture handles, got ${handles.length}`,
    );
  }

  const actualClearValues: Record<string, unknown> = {};
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
    const decryptResult = await executeUserDecryptRequest({
      handle,
      contractId,
      userIdentity,
      contractIds: [contractId],
      nativeSignerKeypairPath: anchorAuthorityKeypairPath,
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
    'confidential_token_contract_id_hex',
    'solana-user-decryption',
  );

  const unauthorizedDecrypt = await executeUserDecryptRequest({
    handle: boolHandle,
    contractId,
    userIdentity: otherUserIdentity,
    contractIds: [contractId],
    nativeSignerKeypairPath: tokenRecipientKeypairPath,
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

  const wrongContract = await executeUserDecryptRequest({
    handle: boolHandle,
    contractId: wrongContractAddress,
    userIdentity,
    contractIds: [wrongContractAddress],
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
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
    contractId,
    userIdentity,
    contractIds: [contractId],
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
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
  console.log(`userId=${userIdentity}`);
  console.log(`clearValues=${JSON.stringify(actualClearValues)}`);
}

async function runPublicDecryptCase(addresses: EnvRecord, solanaEnv: EnvRecord, scenarioName: string) {
  await ensureRelayerReachable();
  const scenario = runLocalCliScenario(scenarioName, solanaEnv);
  await waitForSolanaTransactionsCommitted(
    scenarioSignatures(scenario),
    SOLANA_E2E_COMMITMENT,
  );
  await waitForSolanaListenerCaughtUp(
    Number(addresses.SOLANA_HOST_CHAIN_ID),
    await maxCommittedSlot(scenarioSignatures(scenario), SOLANA_E2E_COMMITMENT),
  );
  const handles = scenarioFinalHandles(scenario);
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
    const nonPublicScenario = runLocalCliScenario('scenario-user-decrypt', solanaEnv);
    await waitForSolanaTransactionsCommitted(
      scenarioSignatures(nonPublicScenario),
      SOLANA_E2E_COMMITMENT,
    );
    await waitForSolanaListenerCaughtUp(
      chainId,
      await maxCommittedSlot(scenarioSignatures(nonPublicScenario), SOLANA_E2E_COMMITMENT),
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
        includesPublicDecryptErrorMessage(rejectedResult, 'not allowed for public decryption') ||
        includesPublicDecryptErrorMessage(rejectedResult, 'not allowed on host acl') ||
        includesPublicDecryptErrorMessage(rejectedResult, 'acl check failed')
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

async function runConfidentialTokenCase(addresses: EnvRecord, solanaEnv: EnvRecord, testSuiteEnv: EnvRecord) {
  await ensureRelayerReachable();
  const identities = runLocalCliJson('runtime-identities', solanaEnv);
  const contractIdentity = requiredScenarioField(
    identities,
    'confidential_token_contract_id_hex',
    'solana-confidential-token',
  );
  const chainId = Number(addresses.SOLANA_HOST_CHAIN_ID);
  const alicePubkey = requiredScenarioField(identities, 'payer_pubkey', 'solana-confidential-token');
  const aliceIdentity = requiredScenarioField(
    identities,
    'payer_user_id_hex',
    'solana-confidential-token',
  );
  const bobPubkey = requiredScenarioField(identities, 'token_recipient_pubkey', 'solana-confidential-token');
  const bobIdentity = requiredScenarioField(
    identities,
    'token_recipient_user_id_hex',
    'solana-confidential-token',
  );
  const aclIdentity = requiredScenarioField(identities, 'host_program_id_hex', 'solana-confidential-token');

  function runTokenCommand(commandName: string, extraArgs: string[] = [], payerKeypairPath = anchorAuthorityKeypairPath) {
    return runLocalCliJson(commandName, solanaEnv, [
      '--payer-keypair',
      payerKeypairPath,
      ...extraArgs,
    ]);
  }

  async function waitForResultBatch(_label: string, ...results: Array<Record<string, unknown>>) {
    const signatures = results
      .flatMap((result) =>
        Array.isArray(result?.signatures)
          ? result.signatures as string[]
          : result?.signature
            ? [result.signature as string]
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

  async function buildTokenAmountProof(value: string | number, userIdentity: string, label: string) {
    return buildGatewayBackedInputProof({
      relayerUrl: requiredEnvValue(testSuiteEnv, 'RELAYER_URL', label),
      inputVerificationAddress: requiredEnvValue(
        testSuiteEnv,
        'INPUT_VERIFICATION_ADDRESS',
        label,
      ),
      gatewayChainId: requiredEnvValue(testSuiteEnv, 'CHAIN_ID_GATEWAY', label),
      hostChainId: addresses.SOLANA_HOST_CHAIN_ID,
      contractIdentity,
      userIdentity,
      aclIdentity,
      value: String(value),
      type: 'uint64',
      coprocessorSigners: loadCoprocessorSigners(addresses),
      coprocessorThreshold: requiredEnvValue(addresses, 'COPROCESSOR_THRESHOLD', label),
    });
  }

  function singleReturnedHandle(result: Record<string, unknown>, label: string): string {
    const handle = (result?.returned_handles as string[] | undefined)?.[0];
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
  const mintPbsRows = countPbsRows(chainId, mintBalanceHandle);

  if (!mintState.exists || !mintState.isCompleted || mintState.isError) {
    throw new Error(
      `solana-confidential-token: expected mint handle to be fully computed, got ${formatComputationState(
        mintState,
      )}`,
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
    contractId: contractIdentity,
    userIdentity: aliceIdentity,
    contractIds: [contractIdentity],
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
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

  await Promise.all([
    waitForComputationRow(chainId, aliceAfterTransferHandle, 'solana-confidential-token transfer(alice)'),
    waitForComputationRow(chainId, bobAfterTransferHandle, 'solana-confidential-token transfer(bob)'),
    waitForCiphertextPresent(transferInputHandle, 'solana-confidential-token transfer input'),
    waitForCiphertextPresent(aliceAfterTransferHandle, 'solana-confidential-token transfer(alice)'),
    waitForCiphertextPresent(bobAfterTransferHandle, 'solana-confidential-token transfer(bob)'),
  ]);

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
    contractId: contractIdentity,
    userIdentity: aliceIdentity,
    contractIds: [contractIdentity],
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
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
    contractId: contractIdentity,
    userIdentity: bobIdentity,
    contractIds: [contractIdentity],
    nativeSignerKeypairPath: tokenRecipientKeypairPath,
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
    contractId: contractIdentity,
    userIdentity: bobIdentity,
    contractIds: [contractIdentity],
    nativeSignerKeypairPath: tokenRecipientKeypairPath,
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
    contractId: contractIdentity,
    userIdentity: aliceIdentity,
    contractIds: [contractIdentity],
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
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
    contractId: contractIdentity,
    userIdentity: bobIdentity,
    contractIds: [contractIdentity],
    nativeSignerKeypairPath: tokenRecipientKeypairPath,
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
    contractId: contractIdentity,
    userIdentity: aliceIdentity,
    contractIds: [contractIdentity],
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
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
    contractId: contractIdentity,
    userIdentity: bobIdentity,
    contractIds: [contractIdentity],
    nativeSignerKeypairPath: tokenRecipientKeypairPath,
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
    contractId: contractIdentity,
    userIdentity: aliceIdentity,
    contractIds: [contractIdentity],
    nativeSignerKeypairPath: anchorAuthorityKeypairPath,
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
    contractId: contractIdentity,
    userIdentity: bobIdentity,
    contractIds: [contractIdentity],
    nativeSignerKeypairPath: tokenRecipientKeypairPath,
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

// ---------------------------------------------------------------------------
// local-cli helpers
// ---------------------------------------------------------------------------

function runLocalCliScenario(
  scenarioName: string,
  solanaEnv: EnvRecord,
  extraArgs: string[] = [],
): Record<string, unknown> {
  return runLocalCliJson(scenarioName, solanaEnv, extraArgs);
}

/** Extracts the transaction signatures array from a local-cli JSON result. */
function scenarioSignatures(result: Record<string, unknown>): string[] {
  return Array.isArray(result?.signatures) ? result.signatures as string[] : [];
}

/** Extracts the final_handles array from a local-cli JSON result. */
function scenarioFinalHandles(result: Record<string, unknown>): string[] {
  return Array.isArray(result?.final_handles) ? result.final_handles as string[] : [];
}

function runLocalCliJson(
  commandName: string,
  solanaEnv: EnvRecord,
  extraArgs: string[] = [],
): Record<string, unknown> {
  const args = [
    'run',
    '--manifest-path',
    localCliManifestPath,
    '--',
    commandName,
    '--addresses-env',
    testAddressesEnvPath,
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

// ---------------------------------------------------------------------------
// Solana RPC helpers
// ---------------------------------------------------------------------------

async function solanaRpcHealthy(): Promise<boolean> {
  try {
    await solanaRpcCall('getHealth', []);
    return true;
  } catch {
    return false;
  }
}

async function solanaRpcCall(method: string, params: unknown[] = []): Promise<unknown> {
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

  const payload = await response.json() as Record<string, unknown>;
  if (payload.error) {
    throw new Error(`Solana RPC ${method} returned error: ${JSON.stringify(payload.error)}`);
  }

  return payload.result;
}

async function waitForSolanaListenerCaughtUp(chainId: number, minimumSlot: number): Promise<void> {
  const started = Date.now();
  let current = 0;
  while (Date.now() - started < 120_000) {
    current = Number(
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
    `timed out waiting for Solana host listener to catch up to slot ${minimumSlot} (current ${current})`,
  );
}

async function maxCommittedSlot(signatures: string[], commitment: string): Promise<number> {
  let maxSlot = 0;
  for (const signature of signatures) {
    const transaction = await solanaRpcCall('getTransaction', [
      signature,
      {
        commitment,
        encoding: 'json',
        maxSupportedTransactionVersion: 0,
      },
    ]) as Record<string, unknown> | null;
    const slot = Number(transaction?.slot ?? 0);
    if (!Number.isFinite(slot) || slot <= 0) {
      throw new Error(`failed to resolve committed slot for Solana signature ${signature}`);
    }
    maxSlot = Math.max(maxSlot, slot);
  }
  return maxSlot;
}

async function waitForSolanaTransactionsCommitted(
  signatures: string[],
  minimumCommitment: string,
): Promise<void> {
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
    ]) as Record<string, unknown>;
    const values = Array.isArray(statuses?.value) ? statuses.value as Array<Record<string, unknown>> : [];

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

function commitmentSatisfied(status: Record<string, unknown>, minimumCommitment: string): boolean {
  const observed = status.confirmationStatus as string | undefined;
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

async function ensureRelayerReachable(): Promise<void> {
  const relayerUrl = testSuiteEnv?.RELAYER_URL ?? 'http://127.0.0.1:3000';
  try {
    await fetch(`${relayerUrl}/v2/public-decrypt/not-a-uuid`);
  } catch (error) {
    throw new Error(
      `relayer is not reachable on ${relayerUrl}; deploy the stack first with ./fhevm-cli deploy --local (${error})`,
    );
  }
}

// ---------------------------------------------------------------------------
// Chain state management
// ---------------------------------------------------------------------------

async function clearChainState(chainId: number): Promise<void> {
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

async function resetChainState(chainId: number): Promise<void> {
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

// ---------------------------------------------------------------------------
// DB query helpers
// ---------------------------------------------------------------------------

function sqlScalar(query: string): string {
  return sqlExec(query).stdout.trim();
}

function sqlExec(query: string): { stdout: string; stderr: string } {
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

async function waitForComputationRow(chainId: number, handle: string, label: string): Promise<void> {
  await waitForCondition(
    () => computationExists(chainId, handle),
    60_000,
    `${label}: computation row for ${handle}`,
  );
}

async function waitForCiphertextPresent(handle: string, label: string): Promise<void> {
  await waitForCondition(
    () => countCiphertextRows(handle) > 0,
    120_000,
    `${label}: ciphertext row for ${handle}`,
  );
}

async function waitForCondition(
  check: () => boolean,
  timeoutMs: number,
  description: string,
): Promise<void> {
  const started = Date.now();
  while (Date.now() - started < timeoutMs) {
    if (check()) {
      return;
    }
    await sleep(1000);
  }
  throw new Error(`timed out waiting for ${description}`);
}

function computationExists(chainId: number, handle: string): boolean {
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

interface ComputationState {
  exists: boolean;
  isCompleted: boolean;
  isAllowed: boolean;
  isError: boolean;
  errorMessage: string;
}

function queryComputationState(chainId: number, handle: string): ComputationState {
  const exists = computationExists(chainId, handle);
  if (!exists) {
    return { exists: false, isCompleted: false, isAllowed: false, isError: false, errorMessage: '' };
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

function formatComputationState(state: ComputationState): string {
  if (!state.exists) {
    return 'missing';
  }
  return `exists(completed=${state.isCompleted}, allowed=${state.isAllowed}, error=${state.isError}, message=${JSON.stringify(
    state.errorMessage,
  )})`;
}

function countPublicDecryptAllowRows(chainId: number, handle: string): number {
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

function countPbsRows(chainId: number, handle: string): number {
  return Number(
    sqlScalar(`
      SELECT COUNT(*)
      FROM pbs_computations
      WHERE host_chain_id = ${chainId}
        AND handle = decode('${stripHexPrefix(handle)}', 'hex')
    `),
  );
}

function countCiphertextRows(handle: string): number {
  return Number(
    sqlScalar(`
      SELECT COUNT(*)
      FROM ciphertexts
      WHERE handle = decode('${stripHexPrefix(handle)}', 'hex')
    `),
  );
}

function countComputationsDependingOnHandle(chainId: number, handle: string): number {
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

// ---------------------------------------------------------------------------
// Handle / scenario helpers
// ---------------------------------------------------------------------------

function assertMintHandleMetadata(handle: string, hostChainId: string, label: string): void {
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

function scenarioExpectedClearValues(scenario: Record<string, unknown>): Record<string, unknown> {
  const handles = scenarioFinalHandles(scenario);
  const expected = Array.isArray(scenario.expected) ? scenario.expected as Array<Record<string, unknown>> : [];
  if (handles.length !== expected.length) {
    throw new Error(
      `scenario ${scenario.scenario ?? 'unknown'} expected ${expected.length} clear values for ${handles.length} handles`,
    );
  }

  const clearValues: Record<string, unknown> = {};
  for (let index = 0; index < handles.length; index += 1) {
    clearValues[handles[index]] = normalizeScenarioExpectedValue(expected[index]);
  }
  return clearValues;
}

function normalizeScenarioExpectedValue(expected: Record<string, unknown>): unknown {
  const outputType = String(expected?.output_type ?? '').toLowerCase();
  const value = String(expected?.value ?? '');

  if (outputType === 'bool') {
    if (value === 'true') return true;
    if (value === 'false') return false;
    throw new Error(`unexpected bool scenario value: ${value}`);
  }

  if (outputType === 'address') {
    return value.toLowerCase();
  }

  return value;
}

// ---------------------------------------------------------------------------
// Misc utilities
// ---------------------------------------------------------------------------

function requiredScenarioField(
  scenario: Record<string, unknown>,
  field: string,
  label: string,
): string {
  const value = scenario?.[field];
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`${label}: scenario did not return ${field}`);
  }
  return value;
}


function runCommand(
  command: string,
  args: string[],
  options: { cwd?: string; env?: NodeJS.ProcessEnv; capture?: boolean } = {},
): { stdout: string; stderr: string } {
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

function oneLineSql(query: string): string {
  return query
    .split('\n')
    .map((line) => line.trim())
    .filter(Boolean)
    .join(' ');
}
