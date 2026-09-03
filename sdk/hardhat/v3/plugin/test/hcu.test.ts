// D5a2: the HCU engine. Unit: the price bridge answers every priced event and refuses the rest, the
// handle parser reads the protocol layout, a synthetic receipt walks depth through a chain. Live: a
// `trivialEncrypt` receipt costs exactly the table's price.
//
// Tests import the BUILT payload (pkg/_esm); see connection.test.ts.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import { HardhatPluginError } from 'hardhat/plugins';
import { encodeAbiParameters, encodeEventTopics, encodeFunctionData } from 'viem';

import plugin, { FhevmType } from '../pkg/_esm/index.js';
import type { FhevmLog } from '../pkg/_esm/index.js';
import { developmentChain, developmentPublicClient } from '../pkg/_esm/internal/clients.js';
import { type FhevmContractWrapper, FhevmCleartextContractsRepository } from '../pkg/_esm/internal/contracts.js';
import { precomputeLocalhostAddresses } from '../pkg/_esm/internal/deploy.js';
import { COPROCESSOR_EVENT_NAMES } from '../pkg/_esm/internal/events.js';
import { parseFhevmHandle } from '../pkg/_esm/internal/fhevmHandle.js';
import { getFheTypeName, getFheTypeNameFromByte } from '../pkg/_esm/internal/hcu/fheTypeName.js';
import { computeTransactionHCU } from '../pkg/_esm/internal/hcu/hcu.js';
import { HCU_PRICE_BY_EVENT, getBucketedHCU, getHCU, hcuPriceOf } from '../pkg/_esm/internal/hcu/prices.js';
import { ALL_OPERATORS_PRICES } from '../pkg/_esm/internal/vendored/operatorsPrices.js';

const ALICE = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';
const TX_HASH = `0x${'22'.repeat(32)}` as const;
const isPluginError = (e: unknown): boolean => e instanceof HardhatPluginError;

/** A computed handle of `fhevmType` on chain 31337, hash bytes taken from `seed`. */
function handle(seed: number, fhevmType: FhevmType, computed = true): `0x${string}` {
  const hash = seed.toString(16).padStart(2, '0').repeat(21);
  const index = computed ? 'ff' : '00';
  const chainId = (31337).toString(16).padStart(16, '0');
  return `0x${hash}${index}${chainId}${fhevmType.toString(16).padStart(2, '0')}00`;
}

async function withExecutor<T>(
  fn: (executor: FhevmContractWrapper, connection: Awaited<ReturnType<typeof open>>) => Promise<T> | T,
): Promise<T> {
  const connection = await open();
  try {
    const client = developmentPublicClient(connection.provider, await developmentChain(connection.provider));
    const { fhevmAddresses, cleartextAddresses, pauserSetAddress } = precomputeLocalhostAddresses();
    const repository = new FhevmCleartextContractsRepository(client, {
      ...fhevmAddresses,
      ...cleartextAddresses,
      pauserSetAddress,
    });
    return await fn(repository.fhevmExecutor, connection);
  } finally {
    await connection.close();
  }
}

async function open(): ReturnType<Awaited<ReturnType<typeof createHardhatRuntimeEnvironment>>['network']['create']> {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  return hre.network.create();
}

void test('the price bridge covers every operator event and nothing else', () => {
  for (const name of COPROCESSOR_EVENT_NAMES) {
    assert.equal(hcuPriceOf(name) !== undefined, name !== 'VerifyInput', name);
  }
  assert.equal(hcuPriceOf('FheMulDiv'), ALL_OPERATORS_PRICES.fheMulDiv);
  assert.equal(Object.hasOwn(HCU_PRICE_BY_EVENT, 'toString'), false);
  assert.equal(getHCU('FheAdd', 'Uint32', { scalar: true }), ALL_OPERATORS_PRICES.fheAdd.scalar?.Uint32);
  assert.equal(getHCU('FheAdd', 'Uint32'), ALL_OPERATORS_PRICES.fheAdd.nonScalar?.Uint32);
  assert.equal(getHCU('TrivialEncrypt', 'Uint64'), ALL_OPERATORS_PRICES.trivialEncrypt.types?.Uint64);
  assert.equal(getHCU('FheSum', 'Uint8', { n: 25 }), ALL_OPERATORS_PRICES.fheSum.nBucketed?.Uint8?.le30);
  assert.throws(() => getHCU('FheSum', 'Uint8'), isPluginError);
  assert.throws(() => getHCU('FheSum', 'Uint64', { n: 1000 }), isPluginError);
  assert.throws(() => getHCU('VerifyInput', 'Uint8'), isPluginError);
  assert.throws(() => getHCU('FheAdd', 'Bool'), isPluginError);
  assert.equal(getBucketedHCU({ le10: 1, le30: 2 }, 10), 1);
  assert.equal(getBucketedHCU({ le10: 1, le30: 2 }, 11), 2);
});

void test('type names and handles follow the protocol layout', () => {
  assert.equal(getFheTypeName(FhevmType.euint32), 'Uint32');
  assert.equal(getFheTypeName(FhevmType.eaddress), 'Uint160');
  assert.throws(() => getFheTypeName(FhevmType.euint4), isPluginError);
  assert.equal(getFheTypeNameFromByte(0), 'Bool');
  assert.throws(() => getFheTypeNameFromByte(99), isPluginError);

  const h = handle(0xab, FhevmType.euint64);
  const info = parseFhevmHandle(h);
  assert.equal(info.chainId, 31337);
  assert.equal(info.fhevmType, FhevmType.euint64);
  assert.equal(info.typeName, 'euint64');
  assert.equal(info.computed, true);
  assert.equal(info.version, 0);
  assert.equal(parseFhevmHandle(handle(1, FhevmType.ebool, false)).computed, false);
  assert.throws(() => parseFhevmHandle('0x1234'), isPluginError);
  assert.throws(() => parseFhevmHandle(`0x${'00'.repeat(30)}6300`), isPluginError);
});

void test('a synthetic receipt walks HCU depth through the dependency chain', async () => {
  await withExecutor((executor) => {
    const h1 = handle(1, FhevmType.euint32);
    const h2 = handle(2, FhevmType.euint32);
    const h3 = handle(3, FhevmType.euint32);
    const log = (eventName: string, params: ReadonlyArray<{ type: string }>, values: readonly unknown[]): FhevmLog => ({
      address: executor.address,
      topics: encodeEventTopics({ abi: executor.abi, eventName, args: { caller: ALICE } }).filter(
        (t): t is `0x${string}` => typeof t === 'string',
      ),
      data: encodeAbiParameters(params, values),
      blockNumber: 1n,
      logIndex: 0,
      transactionHash: TX_HASH,
      transactionIndex: 0,
    });
    const trivial = log('TrivialEncrypt', [{ type: 'uint256' }, { type: 'uint8' }, { type: 'bytes32' }], [7n, 4, h1]);
    const add = log(
      'FheAdd',
      [{ type: 'bytes32' }, { type: 'bytes32' }, { type: 'bytes1' }, { type: 'bytes32' }],
      [h1, h2, '0x00', h3],
    );
    const info = computeTransactionHCU(executor, { status: 'success', transactionHash: TX_HASH, logs: [trivial, add] });

    const trivialPrice = ALL_OPERATORS_PRICES.trivialEncrypt.types?.Uint32 ?? -1;
    const addPrice = ALL_OPERATORS_PRICES.fheAdd.nonScalar?.Uint32 ?? -1;
    assert.equal(info.transactionHash, TX_HASH);
    assert.equal(info.globalHCU, trivialPrice + addPrice);
    assert.equal(info.HCUDepthByHandle[h1], trivialPrice);
    assert.equal(info.HCUDepthByHandle[h3], addPrice + trivialPrice);
    assert.equal(info.maxHCUDepth, addPrice + trivialPrice);

    // ethers shape, and a reverted receipt.
    assert.equal(
      computeTransactionHCU(executor, { status: 1, hash: TX_HASH, logs: [trivial] }).globalHCU,
      trivialPrice,
    );
    assert.throws(() => computeTransactionHCU(executor, { status: 0, hash: TX_HASH, logs: [] }), isPluginError);
  });
});

void test('a live trivialEncrypt costs exactly the table price', async () => {
  await withExecutor(async (executor, connection) => {
    const [from] = (await connection.provider.request({ method: 'eth_accounts' })) as Array<`0x${string}`>;
    const hash = (await connection.provider.request({
      method: 'eth_sendTransaction',
      params: [
        {
          from,
          to: executor.address,
          data: encodeFunctionData({ abi: executor.abi, functionName: 'trivialEncrypt', args: [42n, 4] }),
        },
      ],
    })) as `0x${string}`;
    const client = developmentPublicClient(connection.provider, await developmentChain(connection.provider));
    const receipt = await client.getTransactionReceipt({ hash });

    const info = computeTransactionHCU(executor, receipt);
    const price = ALL_OPERATORS_PRICES.trivialEncrypt.types?.Uint32 ?? -1;
    assert.equal(info.globalHCU, price);
    assert.equal(info.maxHCUDepth, price);
    assert.equal(Object.keys(info.HCUDepthByHandle).length, 1);
    assert.equal(info.transactionHash, hash);
  });
});
