// D4a3: the two public error helpers. `revertedWithCustomErrorArgs` hands chai an ethers-shaped
// interface backed by viem; `tryParseFhevmError` structures an InputVerifier `InvalidSigner` revert and
// answers undefined for everything else.
//
// Tests import the BUILT payload (pkg/_esm); see connection.test.ts.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import { HardhatPluginError } from 'hardhat/plugins';
import { encodeErrorResult, encodeFunctionData } from 'viem';

import plugin from '../pkg/_esm/index.js';
import { precomputeLocalhostAddresses } from '../pkg/_esm/internal/deploy.js';

const ALICE = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';
const HANDLE = `0x${'11'.repeat(32)}` as const;

void test('revertedWithCustomErrorArgs answers the matcher contract: abi plus an ethers-shaped interface', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const [contract, name] = connection.fhevm.revertedWithCustomErrorArgs('ACL', 'SenderNotAllowed');
    assert.equal(name, 'SenderNotAllowed');
    assert.ok(contract.abi.length > 0);
    const fragment = contract.interface.getError('SenderNotAllowed');
    assert.ok(fragment);
    assert.equal(fragment.selector.length, 10);
    assert.equal(contract.interface.getError(fragment.selector.toUpperCase())?.name, 'SenderNotAllowed');
    assert.equal(contract.interface.getError('NoSuchError'), null);
    const data = encodeErrorResult({ abi: contract.abi, errorName: 'SenderNotAllowed', args: [ALICE] });
    assert.deepEqual(contract.interface.decodeErrorResult(fragment, data).toArray(), [ALICE]);

    const isPluginError = (e: unknown): boolean => e instanceof HardhatPluginError;
    assert.throws(() => connection.fhevm.revertedWithCustomErrorArgs('ACL', 'NoSuchError'), isPluginError);
  } finally {
    await connection.close();
  }
});

void test('tryParseFhevmError structures an InvalidSigner revert and ignores the rest', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const { fhevm } = connection;
    const [inputVerifier] = fhevm.revertedWithCustomErrorArgs('InputVerifier', 'InvalidSigner');
    const data = encodeErrorResult({ abi: inputVerifier.abi, errorName: 'InvalidSigner', args: [ALICE] });

    // Bare provider shape, and the shapes ethers wraps it in.
    for (const e of [
      Object.assign(new Error('reverted'), { data }),
      Object.assign(new Error('wrapped'), { error: { data } }),
      Object.assign(new Error('call exception'), { info: { error: { data } } }),
    ]) {
      const parsed = await fhevm.tryParseFhevmError(e);
      assert.equal(parsed?.type, 'InputVerifier');
      assert.equal(parsed.name, 'InvalidSigner');
      assert.ok(parsed.longMessage.includes('createEncryptedInput'));
    }

    assert.equal(await fhevm.tryParseFhevmError(new Error('plain')), undefined);
    assert.equal(await fhevm.tryParseFhevmError('not an error'), undefined);

    // A live ACL revert is ours, but not an InputVerifier one: undefined, not a throw.
    const [from] = (await connection.provider.request({ method: 'eth_accounts' })) as string[];
    const [acl] = fhevm.revertedWithCustomErrorArgs('ACL', 'SenderNotAllowed');
    const call = encodeFunctionData({ abi: acl.abi, functionName: 'allow', args: [HANDLE, ALICE] });
    const aclAddress = precomputeLocalhostAddresses().fhevmAddresses.aclAddress;
    let caught: unknown;
    try {
      await connection.provider.request({
        method: 'eth_sendTransaction',
        params: [{ from, to: aclAddress, data: call }],
      });
    } catch (e) {
      caught = e;
    }
    assert.ok(caught instanceof Error);
    assert.equal(await fhevm.tryParseFhevmError(caught), undefined);
  } finally {
    await connection.close();
  }
});
