import chalk from 'chalk';
import { task } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';

task('task:addRelayer')
  .addParam('privateKey', 'The owner private key')
  .addParam('oracleAddress', 'The OraclePredeploy address')
  .addParam('relayerAddress', 'The relayer address')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const codeAtAddress = await ethers.provider.getCode(taskArguments.oracleAddress);
    if (codeAtAddress === '0x') {
      throw Error(`${taskArguments.oracleAddress} is not a smart contract`);
    }
    const owner = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const oracle = await ethers.getContractAt('OraclePredeploy', taskArguments.oracleAddress, owner);
    const tx = await oracle.addRelayer(taskArguments.relayerAddress);
    const rcpt = await tx.wait();
    if (rcpt!.status === 1) {
      console.log(`Account ${taskArguments.oracleAddress} was succesfully added as an oracle relayer`);
    } else {
      console.log('Adding relayer failed');
    }
  });

task('task:removeRelayer')
  .addParam('privateKey', 'The owner private key')
  .addParam('oracleAddress', 'The OraclePredeploy address')
  .addParam('relayerAddress', 'The relayer address')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const codeAtAddress = await ethers.provider.getCode(taskArguments.oracleAddress);
    if (codeAtAddress === '0x') {
      throw Error(`${taskArguments.oracleAddress} is not a smart contract`);
    }
    const owner = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const oracle = await ethers.getContractAt('OraclePredeploy', taskArguments.oracleAddress, owner);
    const tx = await oracle.removeRelayer(taskArguments.relayerAddress);
    const rcpt = await tx.wait();
    if (rcpt!.status === 1) {
      console.log(`Account ${taskArguments.oracleAddress} was succesfully removed from authorized relayers`);
    } else {
      console.log('Removing relayer failed');
    }
  });
