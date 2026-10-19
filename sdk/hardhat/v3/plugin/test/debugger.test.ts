// D6: the debugger reads a handle's cleartext off CleartextDB with no ACL, after checking the type
// byte. Values come from `trivialEncrypt` sent straight to the executor, so no consumer contract is
// needed; a caller with no permission at all still reads them.
//
// Tests import the BUILT payload (pkg/_esm); see connection.test.ts.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import { HardhatPluginError } from 'hardhat/plugins';
import { encodeFunctionData } from 'viem';

import plugin, { FhevmType } from '#esm/index.js';
import { developmentChain, developmentPublicClient } from '#esm/internal/clients.js';
import { precomputeLocalhostAddresses } from '#esm/internal/deploy.js';

const ALICE = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';
const ZERO_HANDLE = `0x${'0'.repeat(64)}` as const;
const pluginError = (fragment: string) => (e: unknown) =>
  e instanceof HardhatPluginError && e.message.includes(fragment);

void test('the debugger reads trivially encrypted values of every kind, with no ACL', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const { fhevm } = connection;
    const [executor] = fhevm.revertedWithCustomErrorArgs('FHEVMExecutor', 'ACLNotAllowed');
    const executorAddress = precomputeLocalhostAddresses().fhevmAddresses.fhevmExecutorAddress as `0x${string}`;
    const [from] = (await connection.provider.request({ method: 'eth_accounts' })) as Array<`0x${string}`>;
    const client = developmentPublicClient(connection.provider, await developmentChain(connection.provider));

    const trivialEncrypt = async (value: bigint, type: FhevmType): Promise<`0x${string}`> => {
      const hash = (await connection.provider.request({
        method: 'eth_sendTransaction',
        params: [
          {
            from,
            to: executorAddress,
            data: encodeFunctionData({ abi: executor.abi, functionName: 'trivialEncrypt', args: [value, type] }),
          },
        ],
      })) as `0x${string}`;
      const receipt = await client.getTransactionReceipt({ hash });
      const [event] = fhevm.parseCoprocessorEvents(receipt.logs);
      assert.equal(event?.eventName, 'TrivialEncrypt');
      return (event.args as { result: `0x${string}` }).result;
    };

    const euint32 = await trivialEncrypt(42n, FhevmType.euint32);
    const ebool = await trivialEncrypt(1n, FhevmType.ebool);
    const eaddress = await trivialEncrypt(BigInt(ALICE), FhevmType.eaddress);

    assert.equal(await fhevm.debugger.decryptEuint(FhevmType.euint32, euint32), 42n);
    assert.equal(await fhevm.debugger.decryptEbool(ebool), true);
    assert.equal(await fhevm.debugger.decryptEaddress(eaddress), ALICE);
    // The ACL would refuse the same read through the permissioned path.
    await assert.rejects(fhevm.publicDecryptEuint(FhevmType.euint32, euint32));

    // Type checks come from the handle itself.
    await assert.rejects(fhevm.debugger.decryptEbool(euint32), pluginError('is a euint32, not a ebool'));
    await assert.rejects(
      fhevm.debugger.decryptEuint(FhevmType.euint8, euint32),
      pluginError('is a euint32, not a euint8'),
    );
    await assert.rejects(fhevm.debugger.decryptEaddress(ebool), pluginError('is a ebool, not a eaddress'));
    await assert.rejects(
      fhevm.debugger.decryptEuint(FhevmType.ebool as unknown as FhevmType.euint8, euint32),
      pluginError('expected an euint type'),
    );
    await assert.rejects(fhevm.debugger.decryptEuint(FhevmType.euint32, ZERO_HANDLE), pluginError('not initialized'));
  } finally {
    await connection.close();
  }
});
