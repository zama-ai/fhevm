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

task('task:computeDecryptionOracleAddress')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployerAddress = new ethers.Wallet(taskArguments.privateKey).address;
    const decryptionoracleAddressPrecomputed = ethers.getCreateAddress({
      from: deployerAddress,
      nonce: 1, // deployer is supposed to have nonce 0 when deploying DecryptionOracle (0 nonce for implementation, +1 for UUPS)
    });
    const envFilePath = path.join(__dirname, '../addresses/.env.decryptionoracle');
    const content = `DECRYPTION_ORACLE_ADDRESS=${decryptionoracleAddressPrecomputed}`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log('decryptionOracleAddress written to addresses/.env.decryptionoracle successfully!');
    } catch (err) {
      console.error('Failed to write to addresses/.env.decryptionoracle:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant DECRYPTION_ORACLE_ADDRESS = ${decryptionoracleAddressPrecomputed};
`;

    try {
      fs.writeFileSync('./addresses/DecryptionOracleAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log('addresses/DecryptionOracleAddress.sol file has been generated successfully.');
    } catch (error) {
      console.error('Failed to write addresses/DecryptionOracleAddress.sol', error);
    }
  });

task('task:launchFhevm')
  .addOptionalParam('skipGetCoin', 'Skip calling getCoin()', false, types.boolean)
  .addOptionalParam(
    'useAddress',
    'Use address instead of privte key for the Decryption Oracle Relayer',
    false,
    types.boolean,
  )
  .setAction(async function (taskArgs, hre) {
    const privKeyDeployer = process.env.PRIVATE_KEY_DECRYPTION_ORACLE_DEPLOYER;
    const deployerAddress = new hre.ethers.Wallet(privKeyDeployer!).address;
    let relayerAddress;
    if (!taskArgs.useAddress) {
      const privKeyRelayer = process.env.PRIVATE_KEY_DECRYPTION_ORACLE_RELAYER;
      relayerAddress = new hre.ethers.Wallet(privKeyRelayer!).address;
    } else {
      relayerAddress = process.env.ADDRESS_DECRYPTION_ORACLE_RELAYER;
    }
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
    await hre.run('task:deployDecryptionOracle', { privateKey: privKeyDeployer, ownerAddress: deployerAddress });
  });

task('task:getBalances').setAction(async function (taskArgs, hre) {
  const privKeyDeployer = process.env.PRIVATE_KEY_DECRYPTION_ORACLE_DEPLOYER;
  const privKeyRelayer = process.env.PRIVATE_KEY_DECRYPTION_ORACLE_RELAYER;
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
