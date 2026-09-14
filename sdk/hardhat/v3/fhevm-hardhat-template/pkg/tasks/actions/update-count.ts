import type { NewTaskActionFunction } from 'hardhat/types/tasks';

import { type CounterTaskArgs, firstWallet, parseAddress, parseValue } from './helpers.js';

export function updateCount(method: 'increment' | 'decrement'): NewTaskActionFunction<CounterTaskArgs> {
  return async ({ address, value }, hre) => {
    const contractAddress = parseAddress(address);
    const clearValue = parseValue(value);
    const connection = await hre.network.create();
    const wallet = await firstWallet(connection);
    const counter = await connection.viem.getContractAt('FHECounter', contractAddress);
    const encryptedValue = await connection.fhevm
      .createEncryptedInput(contractAddress, wallet.account.address)
      .add32(clearValue)
      .encrypt();
    const handle = encryptedValue.handles[0];
    const transactionHash = await counter.write[method]([handle, encryptedValue.inputProof]);
    console.log(`Waiting for transaction ${transactionHash}...`);
    const publicClient = await connection.viem.getPublicClient();
    const receipt = await publicClient.waitForTransactionReceipt({ hash: transactionHash });
    if (receipt.status !== 'success') throw new Error(`Transaction ${transactionHash} failed.`);
    console.log(`FHECounter ${method}(${String(clearValue)}) succeeded.`);
    return receipt;
  };
}
