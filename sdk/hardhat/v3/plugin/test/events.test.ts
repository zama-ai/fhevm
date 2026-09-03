// D4b: the operator-event vocabulary matches the executor ABI, and `parseCoprocessorEvents` decodes the
// executor's logs — hand-encoded in both log shapes, then live off a `trivialEncrypt` receipt.
//
// Tests import the BUILT payload (pkg/_esm); see connection.test.ts.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import { encodeAbiParameters, encodeEventTopics, encodeFunctionData } from 'viem';

import plugin from '#esm/index.js';
import type { FhevmLog } from '#esm/index.js';
import { developmentChain, developmentPublicClient } from '#esm/internal/clients.js';
import { precomputeLocalhostAddresses } from '#esm/internal/deploy.js';
import { COPROCESSOR_EVENT_NAMES } from '#esm/internal/events.js';

const ALICE = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';
const TX_HASH = `0x${'22'.repeat(32)}` as const;
const HANDLE = `0x${'11'.repeat(32)}` as const;

void test('the event vocabulary is exactly the executor ABI minus its proxy events', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const [executor] = connection.fhevm.revertedWithCustomErrorArgs('FHEVMExecutor', 'ACLNotAllowed');
    const abiEvents = executor.abi
      .filter((item) => item.type === 'event')
      .map((item) => item.name)
      .filter((name) => name !== 'Initialized' && name !== 'Upgraded')
      .sort();
    assert.deepEqual([...COPROCESSOR_EVENT_NAMES].sort(), abiEvents);
  } finally {
    await connection.close();
  }
});

void test('parseCoprocessorEvents decodes executor logs in the viem and the ethers shape, and nothing else', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const { fhevm } = connection;
    const [executor] = fhevm.revertedWithCustomErrorArgs('FHEVMExecutor', 'ACLNotAllowed');
    const executorAddress = precomputeLocalhostAddresses().fhevmAddresses.fhevmExecutorAddress as `0x${string}`;
    const topics = encodeEventTopics({
      abi: executor.abi,
      eventName: 'TrivialEncrypt',
      args: { caller: ALICE },
    }).filter((topic): topic is `0x${string}` => typeof topic === 'string');
    const data = encodeAbiParameters([{ type: 'uint256' }, { type: 'uint8' }, { type: 'bytes32' }], [42n, 4, HANDLE]);
    const base = { address: executorAddress, data, topics, transactionHash: TX_HASH, transactionIndex: 0 };
    const viemLog: FhevmLog = { ...base, blockNumber: 7n, logIndex: 3 };
    const ethersLog: FhevmLog = { ...base, blockNumber: 7, index: 3 };
    const foreign: FhevmLog = { ...viemLog, address: ALICE };

    assert.deepEqual(fhevm.parseCoprocessorEvents(null), []);
    assert.deepEqual(fhevm.parseCoprocessorEvents(undefined), []);
    assert.deepEqual(fhevm.parseCoprocessorEvents([foreign]), []);
    const events = fhevm.parseCoprocessorEvents([viemLog, ethersLog]);
    assert.equal(events.length, 2);
    for (const event of events) {
      assert.equal(event.eventName, 'TrivialEncrypt');
      assert.deepEqual(event.args, { caller: ALICE, pt: 42n, toType: 4, result: HANDLE });
      assert.equal(event.index, 3);
      assert.equal(event.blockNumber, 7);
      assert.equal(event.transactionHash, TX_HASH);
    }
  } finally {
    await connection.close();
  }
});

void test('a live trivialEncrypt on the deployed executor yields one TrivialEncrypt event', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const { fhevm } = connection;
    const [executor] = fhevm.revertedWithCustomErrorArgs('FHEVMExecutor', 'ACLNotAllowed');
    const executorAddress = precomputeLocalhostAddresses().fhevmAddresses.fhevmExecutorAddress as `0x${string}`;
    const [from] = (await connection.provider.request({ method: 'eth_accounts' })) as Array<`0x${string}`>;
    const hash = (await connection.provider.request({
      method: 'eth_sendTransaction',
      params: [
        {
          from,
          to: executorAddress,
          data: encodeFunctionData({ abi: executor.abi, functionName: 'trivialEncrypt', args: [42n, 4] }),
        },
      ],
    })) as `0x${string}`;
    const client = developmentPublicClient(connection.provider, await developmentChain(connection.provider));
    const receipt = await client.getTransactionReceipt({ hash });

    const events = fhevm.parseCoprocessorEvents(receipt.logs);
    assert.equal(events.length, 1);
    const [event] = events;
    assert.equal(event?.eventName, 'TrivialEncrypt');
    assert.equal((event.args as { pt: bigint }).pt, 42n);
    assert.equal(event.transactionHash, hash);
  } finally {
    await connection.close();
  }
});
