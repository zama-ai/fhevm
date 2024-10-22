import { exec as oldExec } from 'child_process';
import dotenv from 'dotenv';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';
import path from 'path';
import { promisify } from 'util';
import { mustGetEnv } from './environment';

const exec = promisify(oldExec);

const getCoin = async (address: string) => {
  const containerName = process.env['TEST_CONTAINER_NAME'] || 'fhevm';
  const response = await exec(`docker exec -i ${containerName} faucet ${address} | grep height`);
  const res = JSON.parse(response.stdout);
  if (res.raw_log.match('account sequence mismatch')) await getCoin(address);
};

task('task:computeGatewayAddress')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployerAddress = new ethers.Wallet(taskArguments.privateKey).address;
    const gatewayContractAddressPrecomputed = ethers.getCreateAddress({
      from: deployerAddress,
      nonce: 1, // deployer is supposed to have nonce 0 when deploying GatewayContract (0 nonce for implementation, +1 for UUPS)
    });
    const envFilePath = path.join(__dirname, '../gateway/.env.gateway');
    const content = `GATEWAY_CONTRACT_PREDEPLOY_ADDRESS=${gatewayContractAddressPrecomputed}`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log('gatewayContractAddress written to gateway/.env.gateway successfully!');
    } catch (err) {
      console.error('Failed to write to gateway/.env.gateway:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant GATEWAY_CONTRACT_PREDEPLOY_ADDRESS = ${gatewayContractAddressPrecomputed};
`;

    try {
      fs.writeFileSync('./gateway/lib/GatewayContractAddress.sol', solidityTemplate, { encoding: 'utf8', flag: 'w' });
      console.log('gateway/lib/GatewayContractAddress.sol file has been generated successfully.');
    } catch (error) {
      console.error('Failed to write gateway/lib/GatewayContractAddress.sol', error);
    }
  });

task('task:addRelayer')
  .addParam('privateKey', 'The owner private key')
  .addParam('gatewayAddress', 'The GatewayContract address')
  .addParam('relayerAddress', 'The relayer address')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const codeAtAddress = await ethers.provider.getCode(taskArguments.gatewayAddress);
    if (codeAtAddress === '0x') {
      throw Error(`${taskArguments.gatewayAddress} is not a smart contract`);
    }
    const owner = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const gateway = await ethers.getContractAt('GatewayContract', taskArguments.gatewayAddress, owner);
    const tx = await gateway.addRelayer(taskArguments.relayerAddress);
    const rcpt = await tx.wait();
    if (rcpt!.status === 1) {
      console.log(`Account ${taskArguments.relayerAddress} was succesfully added as an gateway relayer`);
    } else {
      console.log('Adding relayer failed');
    }
  });

task('task:removeRelayer')
  .addParam('privateKey', 'The owner private key')
  .addParam('gatewayAddress', 'The GatewayContract address')
  .addParam('relayerAddress', 'The relayer address')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const codeAtAddress = await ethers.provider.getCode(taskArguments.gatewayAddress);
    if (codeAtAddress === '0x') {
      throw Error(`${taskArguments.gatewayAddress} is not a smart contract`);
    }
    const owner = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const gateway = await ethers.getContractAt('GatewayContract', taskArguments.gatewayAddress, owner);
    const tx = await gateway.removeRelayer(taskArguments.relayerAddress);
    const rcpt = await tx.wait();
    if (rcpt!.status === 1) {
      console.log(`Account ${taskArguments.relayerAddress} was succesfully removed from authorized relayers`);
    } else {
      console.log('Removing relayer failed');
    }
  });

task('task:launchFhevm')
  .addOptionalParam('skipGetCoin', 'Skip calling getCoin()', false, types.boolean)
  .addOptionalParam('useAddress', 'Use address instead of privte key for the Gateway Relayer', false, types.boolean)
  .setAction(async function (taskArgs, hre) {
    const privKeyDeployer = process.env.PRIVATE_KEY_GATEWAY_DEPLOYER;
    const deployerAddress = new hre.ethers.Wallet(privKeyDeployer!).address;
    const relayerAddress = !taskArgs.useAddress
      ? new hre.ethers.Wallet(mustGetEnv('PRIVATE_KEY_GATEWAY_RELAYER')).address
      : mustGetEnv('process.env.ADDRESS_GATEWAY_RELAYER');

    if (!taskArgs.skipGetCoin) {
      if (hre.network.name === 'hardhat') {
        const bal = '0x1000000000000000000000000000000000000000';
        const p1 = hre.network.provider.send('hardhat_setBalance', [deployerAddress, bal]);
        const p2 = hre.network.provider.send('hardhat_setBalance', [relayerAddress, bal]);
        await Promise.all([p1, p2]);
      } else {
        const p1 = getCoin(deployerAddress);
        const p2 = getCoin(relayerAddress);
        await Promise.all([p1, p2]);
        await new Promise((res) => setTimeout(res, 5000)); // wait 5 seconds
      }
    }
    await hre.run('task:deployGateway', { privateKey: privKeyDeployer, ownerAddress: deployerAddress });

    const parsedEnv = dotenv.parse(fs.readFileSync('gateway/.env.gateway'));
    const gatewayContractAddress = parsedEnv.GATEWAY_CONTRACT_PREDEPLOY_ADDRESS;

    await hre.run('task:addRelayer', {
      privateKey: privKeyDeployer,
      gatewayAddress: gatewayContractAddress,
      relayerAddress: relayerAddress,
    });
  });

task('task:getBalances').setAction(async function (taskArgs, hre) {
  const privKeyDeployer = process.env.PRIVATE_KEY_GATEWAY_DEPLOYER;
  const privKeyRelayer = process.env.PRIVATE_KEY_GATEWAY_RELAYER;
  const deployerAddress = new hre.ethers.Wallet(privKeyDeployer!).address;
  const relayerAddress = new hre.ethers.Wallet(privKeyRelayer!).address;
  console.log(await hre.ethers.provider.getBalance(deployerAddress));
  console.log(await hre.ethers.provider.getBalance(relayerAddress));
});

task('task:faucetToPrivate')
  .addParam('privateKey', 'The receiver private key')
  .setAction(async function (taskArgs, hre) {
    const receiverAddress = new hre.ethers.Wallet(taskArgs.privateKey).address;

    if (hre.network.name === 'hardhat') {
      const bal = '0x1000000000000000000000000000000000000000';
      await hre.network.provider.send('hardhat_setBalance', [receiverAddress, bal]);
    } else {
      await getCoin(receiverAddress);
      await new Promise((res) => setTimeout(res, 5000)); // wait 5 seconds
    }
  });

task('task:faucetToAddress')
  .addParam('address', 'The receiver address')
  .setAction(async function (taskArgs, hre) {
    const receiverAddress = taskArgs.address;

    if (hre.network.name === 'hardhat') {
      const bal = '0x1000000000000000000000000000000000000000';
      await hre.network.provider.send('hardhat_setBalance', [receiverAddress, bal]);
    } else {
      await getCoin(receiverAddress);
      await new Promise((res) => setTimeout(res, 5000)); // wait 5 seconds
    }
  });
