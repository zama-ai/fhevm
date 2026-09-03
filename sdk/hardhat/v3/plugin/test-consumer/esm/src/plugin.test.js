// A real consumer's view: the INSTALLED payload (not the workspace source) on a programmatic hardhat 3
// environment — the tasks are registered, a connection deploys the cleartext stack, and a value goes
// through encrypt, an FHE operation on the executor, and back through the debugger. Plain JavaScript
// on purpose — a fixture that adds a toolchain stops representing a consumer.

import assert from 'node:assert/strict';
import test from 'node:test';

import plugin, { FhevmType, getHCU } from '@fhevm/hardhat-plugin';
import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import { createPublicClient, custom, encodeFunctionData, isHex, size } from 'viem';

const CONTRACT = '0x1111111111111111111111111111111111111111';
// The localhost stack is deterministic (CREATE from a fixed deployer at nonce 0), so its executor sits
// at a fixed address — the same one `hardhat node -vvv` prints and ZamaConfig compiles in.
const FHEVM_EXECUTOR = '0xe3a9105a3a932253A70F126eb1E3b589C643dD24';

test('the installed plugin registers the fhevm tasks and its module exports', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  assert.notEqual(hre.tasks.getTask(['fhevm', 'public-decrypt']), undefined);
  assert.notEqual(hre.tasks.getTask(['fhevm', 'user-decrypt']), undefined);
  assert.notEqual(hre.tasks.getTask(['fhevm', 'check-fhevm-compatibility']), undefined);
  assert.equal(typeof getHCU('FheAdd', 'Uint8'), 'number');
  assert.equal(FhevmType.euint32, 4);
});

test('a connection deploys the stack and runs an encrypt / decrypt round-trip', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const { fhevm } = connection;
    assert.equal(fhevm.isCleartext, true);
    assert.equal(fhevm.network.chainId, 31337);

    // Encrypt: an input handle bound to a contract and a user, plus its proof.
    const [from] = await connection.provider.request({ method: 'eth_accounts' });
    const { externalEuint, inputProof } = await fhevm.encryptUint(FhevmType.euint32, 7, CONTRACT, from);
    assert.ok(isHex(externalEuint) && size(externalEuint) === 32);
    assert.ok(isHex(inputProof) && size(inputProof) > 0);

    // An FHE operation straight on the deployed executor, then its event and its cleartext.
    const [executor] = fhevm.revertedWithCustomErrorArgs('FHEVMExecutor', 'ACLNotAllowed');
    const data = encodeFunctionData({
      abi: executor.abi,
      functionName: 'trivialEncrypt',
      args: [42n, FhevmType.euint32],
    });
    const hash = await connection.provider.request({
      method: 'eth_sendTransaction',
      params: [{ from, to: FHEVM_EXECUTOR, data }],
    });
    const client = createPublicClient({ transport: custom(connection.provider) });
    const receipt = await client.getTransactionReceipt({ hash });
    const [event] = fhevm.parseCoprocessorEvents(receipt.logs);
    assert.equal(event.eventName, 'TrivialEncrypt');

    const handle = event.args.result;
    assert.equal(fhevm.typeof(handle), 'euint32');
    assert.equal(await fhevm.debugger.decryptEuint(FhevmType.euint32, handle), 42n);
    assert.equal(fhevm.computeTransactionHCU(receipt).globalHCU, getHCU('TrivialEncrypt', 'Uint32'));
    // Nobody allowed it: the permissioned path refuses what the debugger read.
    await assert.rejects(fhevm.publicDecryptEuint(FhevmType.euint32, handle));
  } finally {
    await connection.close();
  }
});
