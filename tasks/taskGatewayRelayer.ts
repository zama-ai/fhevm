import { exec as oldExec } from 'child_process';
import dotenv from 'dotenv';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';
import path from 'path';
import { promisify } from 'util';

const exec = promisify(oldExec);

const getCoin = async (address: string) => {
  const containerName = process.env['TEST_CONTAINER_NAME'] || 'fhevm';
  const response = await exec(`docker exec -i ${containerName} faucet ${address} | grep height`);
  const res = JSON.parse(response.stdout);
  if (res.raw_log.match('account sequence mismatch')) await getCoin(address);
};

task('task:computePredeployAddress')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployerAddress = new ethers.Wallet(taskArguments.privateKey).address;
    const gatewayContractAddressPrecomputed = ethers.getCreateAddress({
      from: deployerAddress,
      nonce: 0, // deployer is supposed to have nonce 0 when deploying GatewayContract
    });
    const envFilePath = path.join(__dirname, '../gateway/.env.gateway');
    const content = `GATEWAY_CONTRACT_PREDEPLOY_ADDRESS=${gatewayContractAddressPrecomputed}\n`;
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
      fs.writeFileSync('./gateway/lib/PredeployAddress.sol', solidityTemplate, { encoding: 'utf8', flag: 'w' });
      console.log('gateway/lib/PredeployAddress.sol file has been generated successfully.');
    } catch (error) {
      console.error('Failed to write gateway/lib/PredeployAddress.sol', error);
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
  .setAction(async function (taskArgs, hre) {
    const privKeyDeployer = process.env.PRIVATE_KEY_GATEWAY_DEPLOYER;
    const privKeyOwner = process.env.PRIVATE_KEY_GATEWAY_OWNER;
    const privKeyRelayer = process.env.PRIVATE_KEY_GATEWAY_RELAYER;
    const deployerAddress = new hre.ethers.Wallet(privKeyDeployer!).address;
    const ownerAddress = new hre.ethers.Wallet(privKeyOwner!).address;
    const relayerAddress = new hre.ethers.Wallet(privKeyRelayer!).address;
    if (!taskArgs.skipGetCoin) {
      if (hre.network.name === 'hardhat') {
        const bal = '0x1000000000000000000000000000000000000000';
        const p1 = hre.network.provider.send('hardhat_setBalance', [deployerAddress, bal]);
        const p2 = hre.network.provider.send('hardhat_setBalance', [ownerAddress, bal]);
        const p3 = hre.network.provider.send('hardhat_setBalance', [relayerAddress, bal]);
        await Promise.all([p1, p2, p3]);
      } else {
        const p1 = getCoin(deployerAddress);
        const p2 = getCoin(ownerAddress);
        const p3 = getCoin(relayerAddress);
        await Promise.all([p1, p2, p3]);
        await new Promise((res) => setTimeout(res, 5000)); // wait 5 seconds
      }
    }
    console.log(`privateKey ${privKeyDeployer}`);
    console.log(`ownerAddress ${ownerAddress}`);
    await hre.run('task:deployGateway', { privateKey: privKeyDeployer, ownerAddress: ownerAddress });

    const parsedEnv = dotenv.parse(fs.readFileSync('gateway/.env.gateway'));
    const gatewayContractAddress = parsedEnv.GATEWAY_CONTRACT_PREDEPLOY_ADDRESS;

    await hre.run('task:addRelayer', {
      privateKey: privKeyOwner,
      gatewayAddress: gatewayContractAddress,
      relayerAddress: relayerAddress,
    });
  });

task('task:getBalances').setAction(async function (taskArgs, hre) {
  const privKeyDeployer = process.env.PRIVATE_KEY_GATEWAY_DEPLOYER;
  const privKeyOwner = process.env.PRIVATE_KEY_GATEWAY_OWNER;
  const privKeyRelayer = process.env.PRIVATE_KEY_GATEWAY_RELAYER;
  const deployerAddress = new hre.ethers.Wallet(privKeyDeployer!).address;
  const ownerAddress = new hre.ethers.Wallet(privKeyOwner!).address;
  const relayerAddress = new hre.ethers.Wallet(privKeyRelayer!).address;
  console.log(await hre.ethers.provider.getBalance(deployerAddress));
  console.log(await hre.ethers.provider.getBalance(ownerAddress));
  console.log(await hre.ethers.provider.getBalance(relayerAddress));
});
