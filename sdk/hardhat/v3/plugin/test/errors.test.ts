// D4a2: the error engine. Unit: the decoder owns exactly one contract per selector, the formatter fills
// the table from ABI-named arguments, the extractor reads both hardhat 3 error shapes. Live: a revert
// inside the deployed ACL reaches the caller with the FHEVM message, on the SAME error object.
//
// Tests import the BUILT payload (pkg/_esm); see connection.test.ts.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import { encodeErrorResult, encodeFunctionData, hexToBytes } from 'viem';

import plugin from '../pkg/_esm/index.js';
import { developmentChain, developmentPublicClient } from '../pkg/_esm/internal/clients.js';
import { FhevmCleartextContractsRepository } from '../pkg/_esm/internal/contracts.js';
import { precomputeLocalhostAddresses } from '../pkg/_esm/internal/deploy.js';
import { decodeFhevmError } from '../pkg/_esm/internal/errors/decode.js';
import { extractRevertData } from '../pkg/_esm/internal/errors/decorate.js';
import { formatFhevmErrorMessages } from '../pkg/_esm/internal/errors/messages.js';
import { transactionParties } from '../pkg/_esm/internal/requests.js';

const ALICE = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';
const HANDLE = `0x${'11'.repeat(32)}` as const;
const TX_HASH = `0x${'22'.repeat(32)}` as const;

async function withRepository<T>(fn: (repository: FhevmCleartextContractsRepository) => Promise<T> | T): Promise<T> {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const client = developmentPublicClient(connection.provider, await developmentChain(connection.provider));
    const { fhevmAddresses, cleartextAddresses, pauserSetAddress } = precomputeLocalhostAddresses();
    return await fn(
      new FhevmCleartextContractsRepository(client, { ...fhevmAddresses, ...cleartextAddresses, pauserSetAddress }),
    );
  } finally {
    await connection.close();
  }
}

void test('the decoder owns a selector when exactly one FHEVM contract declares it', async () => {
  await withRepository((repository) => {
    const data = encodeErrorResult({ abi: repository.acl.abi, errorName: 'SenderNotAllowed', args: [ALICE] });
    const decoded = decodeFhevmError(repository, data);
    assert.equal(decoded?.wrapper.name, 'ACL');
    assert.equal(decoded.errorName, 'SenderNotAllowed');
    assert.deepEqual(decoded.args, [ALICE]);
    // Every proxy declares the OpenZeppelin initializer errors: several owners, so not ours to explain.
    const owners = [...repository.addressToContractMap().values()].filter((w) =>
      w.abi.some((item) => item.type === 'error' && item.name === 'InvalidInitialization'),
    );
    assert.ok(owners.length > 1, `InvalidInitialization owners: ${String(owners.length)}`);
    const shared = encodeErrorResult({
      abi: repository.acl.abi,
      errorName: 'InvalidInitialization',
      args: [],
    });
    assert.equal(decodeFhevmError(repository, shared), undefined);
    assert.equal(decodeFhevmError(repository, '0xdeadbeef'), undefined);
  });
});

void test('the formatter fills the table from ABI-named arguments, or falls back to the generic line', async () => {
  await withRepository((repository) => {
    const acl = decodeFhevmError(
      repository,
      encodeErrorResult({ abi: repository.acl.abi, errorName: 'SenderNotAllowed', args: [ALICE] }),
    );
    assert.ok(acl);
    const messages = formatFhevmErrorMessages(acl, {});
    assert.equal(messages.title?.includes("'SenderNotAllowed()'"), true, messages.title);
    assert.equal(messages.shortMessage?.includes(ALICE), true, messages.shortMessage);
    assert.equal(messages.message, `${messages.title}: ${messages.shortMessage}`);

    const denied = decodeFhevmError(
      repository,
      encodeErrorResult({ abi: repository.acl.abi, errorName: 'SenderDenied', args: [ALICE] }),
    );
    assert.ok(denied);
    assert.deepEqual(formatFhevmErrorMessages(denied, {}), {
      message: "VM Exception while processing transaction: reverted with FHEVM ACL custom error 'SenderDenied'",
    });
  });
});

void test('the extractor reads EDR and remote-node error shapes, and refuses the rest', () => {
  const edr = Object.assign(new Error('reverted'), {
    data: '0xabcd',
    transactionHash: TX_HASH,
    stackTrace: [{ address: hexToBytes(ALICE) }],
  });
  assert.deepEqual(extractRevertData(edr), { data: '0xabcd', transactionHash: TX_HASH, revertedAt: ALICE });
  const remote = Object.assign(new Error('reverted'), {
    data: { data: '0xabcd', transactionHash: TX_HASH, message: 'x' },
  });
  assert.deepEqual(extractRevertData(remote), { data: '0xabcd', transactionHash: TX_HASH });
  assert.equal(extractRevertData(Object.assign(new Error('x'), { data: '0x' })), undefined);
  assert.equal(extractRevertData(Object.assign(new Error('x'), { data: 'not hex' })), undefined);
  assert.equal(extractRevertData(new Error('x')), undefined);
  assert.equal(extractRevertData('string'), undefined);
});

void test('transactionParties reads the from/to of a call payload', () => {
  assert.deepEqual(
    transactionParties({ jsonrpc: '2.0', id: 1, method: 'eth_call', params: [{ from: ALICE, to: ALICE }] }),
    { from: ALICE, to: ALICE },
  );
  assert.deepEqual(transactionParties({ jsonrpc: '2.0', id: 1, method: 'eth_blockNumber', params: [] }), {});
});

void test('a revert inside the deployed ACL reaches the caller as the FHEVM message, on the same error', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const [from] = (await connection.provider.request({ method: 'eth_accounts' })) as string[];
    const { fhevmAddresses } = precomputeLocalhostAddresses();
    const repository = await withRepository((r) => r);
    const data = encodeFunctionData({ abi: repository.acl.abi, functionName: 'allow', args: [HANDLE, ALICE] });
    await assert.rejects(
      connection.provider.request({
        method: 'eth_sendTransaction',
        params: [{ from, to: fhevmAddresses.aclAddress, data }],
      }),
      (e: unknown) => {
        assert.ok(e instanceof Error);
        assert.ok(e.message.startsWith("FHEVM ACL permission error 'SenderNotAllowed()'"), e.message);
        assert.ok(e.message.includes(ALICE), e.message);
        assert.ok('data' in e, 'the revert data stays on the error');
        return true;
      },
    );
  } finally {
    await connection.close();
  }
});
