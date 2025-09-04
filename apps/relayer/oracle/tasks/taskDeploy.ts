import { HardhatUpgrades } from '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import { Wallet } from 'ethers';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { HardhatEthersHelpers, TaskArguments } from 'hardhat/types';
import path from 'path';

import { getRequiredEnvVar } from './utils/loadVariables';

async function deployEmptyUUPS(ethers: HardhatEthersHelpers, upgrades: HardhatUpgrades, deployer: Wallet) {
  console.log('Deploying an EmptyUUPS proxy contract...');
  const factory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const UUPSEmpty = await upgrades.deployProxy(factory, [deployer.address], {
    initializer: 'initialize',
    kind: 'uups',
  });
  await UUPSEmpty.waitForDeployment();
  const UUPSEmptyAddress = await UUPSEmpty.getAddress();
  console.log('EmptyUUPS proxy contract successfully deployed!');
  return UUPSEmptyAddress;
}

task('task:deployEmptyUUPSProxies').setAction(async function(taskArguments: TaskArguments, { ethers, upgrades, run }) {
  // Compile the EmptyUUPS proxy contract
  await run('compile:specific', { contract: 'contracts/emptyProxy' });

  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);

  // Ensure the addresses directory exists
  fs.mkdirSync(path.join(__dirname, '../addresses'), { recursive: true });

  const decryptionOracleAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setDecryptionOracleAddress', {
    address: decryptionOracleAddress,
  });
});

task('task:deployDecryptionOracle').setAction(async function(taskArguments: TaskArguments, { ethers, upgrades, run }) {
  await run('compile:specific', { contract: 'contracts' });
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('DecryptionOracle', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.decryptionoracle'));
  const proxyAddress = parsedEnv.DECRYPTION_ORACLE_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: { fn: 'reinitialize' },
  });
  console.log('DecryptionOracle code set successfully at address:', proxyAddress);
});

task('task:setDecryptionOracleAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function(taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../addresses/.env.decryptionoracle');
    const content = `DECRYPTION_ORACLE_ADDRESS=${taskArguments.address}`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log('decryptionOracleAddress written to addresses/.env.decryptionoracle successfully!');
    } catch (err) {
      console.error('Failed to write to addresses/.env.decryptionoracle:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant DECRYPTION_ORACLE_ADDRESS = ${taskArguments.address};
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

task('task:deployAllContracts').setAction(async function(_, hre) {
  if (process.env.SOLIDITY_COVERAGE !== 'true') {
    await hre.run('clean');
  }
  await hre.run('task:deployEmptyUUPSProxies');
  await hre.run('compile:specific', { contract: 'decryptionOracle' });
  await hre.run('task:deployDecryptionOracle');

  console.log('Contract deployment done!');
});
