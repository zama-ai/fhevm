import { HardhatUpgrades } from '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import { Wallet } from 'ethers';
import * as fs from 'fs-extra';
import { task, types } from 'hardhat/config';
import type { HardhatEthersHelpers, TaskArguments } from 'hardhat/types';
import path from 'path';

import { getRequiredEnvVar } from './utils/loadVariables';

task('task:deployAllHostContracts').setAction(async function (_, hre) {
  if (process.env.SOLIDITY_COVERAGE !== 'true') {
    await hre.run('clean');
  }

  // Compile and deploy all host empty proxy contracts
  await hre.run('compile:specific', { contract: 'fhevmTemp/contracts/shared' });
  await hre.run('task:deployEmptyUUPSProxies');

  // Compile and deploy all host contracts
  await hre.run('compile:specific', { contract: 'examples' });
  await hre.run('compile:specific', { contract: 'fhevmTemp/contracts' });
  await hre.run('task:deployACL');
  await hre.run('task:deployFHEVMExecutor');
  await hre.run('task:deployKMSVerifier');
  await hre.run('task:deployInputVerifier');
  await hre.run('task:deployHCULimit');
  await hre.run('task:deployDecryptionOracle');

  console.info('Contract deployment done!');
});

async function deployEmptyUUPS(ethers: HardhatEthersHelpers, upgrades: HardhatUpgrades, deployer: Wallet) {
  console.info('Deploying an EmptyUUPS proxy contract...');
  const factory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const UUPSEmpty = await upgrades.deployProxy(factory, [deployer.address], {
    initializer: 'initialize',
    kind: 'uups',
  });
  await UUPSEmpty.waitForDeployment();
  const UUPSEmptyAddress = await UUPSEmpty.getAddress();
  console.info('EmptyUUPS proxy contract successfully deployed!');
  return UUPSEmptyAddress;
}

task('task:deployEmptyUUPSProxies').setAction(async function (
  _taskArguments: TaskArguments,
  { ethers, upgrades, run },
) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const aclAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setACLAddress', {
    address: aclAddress,
  });

  const fhevmExecutorAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setFHEVMExecutorAddress', {
    address: fhevmExecutorAddress,
  });

  const kmsVerifierAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setKMSVerifierAddress', {
    address: kmsVerifierAddress,
  });

  const inputVerifierAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setInputVerifierAddress', {
    address: inputVerifierAddress,
  });

  const HCULimitAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setHCULimitAddress', {
    address: HCULimitAddress,
  });

  const decryptionOracleAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setDecryptionOracleAddress', {
    address: decryptionOracleAddress,
  });
});

task('task:deployDecryptionOracle').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');

  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory(
    'fhevmTemp/contracts/shared/EmptyUUPSProxy.sol:EmptyUUPSProxy',
    deployer,
  );
  const newImplem = await ethers.getContractFactory('DecryptionOracle', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.decryptionoracle'));
  const proxyAddress = parsedEnv.DECRYPTION_ORACLE_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem);
  console.info('DecryptionOracle code set successfully at address:', proxyAddress);
});

task('task:deployACL').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory(
    'fhevmTemp/contracts/shared/EmptyUUPSProxy.sol:EmptyUUPSProxy',
    deployer,
  );
  const newImplem = await ethers.getContractFactory('ACL', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.host'));
  const proxyAddress = parsedEnv.ACL_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem);
  console.info('ACL code set successfully at address:', proxyAddress);
});

task('task:deployFHEVMExecutor').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory(
    'fhevmTemp/contracts/shared/EmptyUUPSProxy.sol:EmptyUUPSProxy',
    deployer,
  );
  let newImplem;
  newImplem = await ethers.getContractFactory('fhevmTemp/contracts/FHEVMExecutor.sol:FHEVMExecutor', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.host'));
  const proxyAddress = parsedEnv.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem);
  console.info('FHEVMExecutor code set successfully at address:', proxyAddress);
});

task('task:deployKMSVerifier').setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory(
    'fhevmTemp/contracts/shared/EmptyUUPSProxy.sol:EmptyUUPSProxy',
    deployer,
  );
  const newImplem = await ethers.getContractFactory('fhevmTemp/contracts/KMSVerifier.sol:KMSVerifier', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.host'));
  const proxyAddress = parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);

  const verifyingContractSource = process.env.DECRYPTION_ADDRESS!;
  const chainIDSource = +process.env.CHAIN_ID_GATEWAY!;
  const initialThreshold = +process.env.KMS_THRESHOLD!;
  let initialSigners: string[] = [];
  const numSigners = getRequiredEnvVar('NUM_KMS_NODES');

  for (let idx = 0; idx < +numSigners; idx++) {
    const kmsSignerAddress = getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`);
    initialSigners.push(kmsSignerAddress);
  }
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: {
      fn: 'initializeFromEmptyProxy',
      args: [verifyingContractSource, chainIDSource, initialSigners, initialThreshold],
    },
  });
  console.info('KMSVerifier code set successfully at address:', proxyAddress);
  console.info(`${numSigners} KMS signers were added to KMSVerifier at initialization`);
});

task('task:deployInputVerifier')
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private keys env variables for kms signers',
    true,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
    const currentImplementation = await ethers.getContractFactory(
      'fhevmTemp/contracts/shared/EmptyUUPSProxy.sol:EmptyUUPSProxy',
      deployer,
    );
    const newImplem = await ethers.getContractFactory('fhevmTemp/contracts/InputVerifier.sol:InputVerifier', deployer);
    const parsedEnv = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.host'));

    const proxyAddress = parsedEnv.INPUT_VERIFIER_CONTRACT_ADDRESS;
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const verifyingContractSource = process.env.INPUT_VERIFICATION_ADDRESS!;
    const chainIDSource = +process.env.CHAIN_ID_GATEWAY!;

    let initialSigners: string[] = [];
    const numSigners = getRequiredEnvVar('NUM_COPROCESSORS');
    for (let idx = 0; idx < +numSigners; idx++) {
      if (!taskArguments.useAddress) {
        const privKeySigner = getRequiredEnvVar(`PRIVATE_KEY_COPROCESSOR_ACCOUNT_${idx}`);
        const inputSigner = new ethers.Wallet(privKeySigner).connect(ethers.provider);
        initialSigners.push(inputSigner.address);
      } else {
        const inputSignerAddress = getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`);
        initialSigners.push(inputSignerAddress);
      }
    }

    await upgrades.upgradeProxy(proxy, newImplem, {
      call: { fn: 'initializeFromEmptyProxy', args: [verifyingContractSource, chainIDSource, initialSigners] },
    });
    console.info('InputVerifier code set successfully at address:', proxyAddress);
  });

task('task:deployHCULimit').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory(
    'fhevmTemp/contracts/shared/EmptyUUPSProxy.sol:EmptyUUPSProxy',
    deployer,
  );
  const newImplem = await ethers.getContractFactory('HCULimit', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.host'));
  const proxyAddress = parsedEnv.HCU_LIMIT_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem);
  console.info('HCULimit code set successfully at address:', proxyAddress);
});

task('task:setACLAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../fhevmTemp/addresses/.env.host');
    const content = `ACL_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.info(`ACL address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write ACL address:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant aclAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./fhevmTemp/addresses/FHEVMHostAddresses.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.info('./fhevmTemp/addresses/FHEVMHostAddresses.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./fhevmTemp/addresses/FHEVMHostAddresses.sol', error);
    }
  });

task('task:setFHEVMExecutorAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../fhevmTemp/addresses/.env.host');
    const content = `FHEVM_EXECUTOR_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.appendFileSync(envFilePath, content, { flag: 'a' });
      console.info(`FHEVMExecutor address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write FHEVMExecutor address:', err);
    }

    const solidityTemplate = `
address constant fhevmExecutorAdd = ${taskArguments.address};\n`;

    try {
      fs.appendFileSync('./fhevmTemp/addresses/FHEVMHostAddresses.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'a',
      });
      console.info('./fhevmTemp/addresses/FHEVMHostAddresses.sol file appended with fhevmExecutorAdd successfully!');
    } catch (error) {
      console.error('Failed to write ./fhevmTemp/addresses/FHEVMHostAddresses.sol', error);
    }
  });

task('task:setKMSVerifierAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../fhevmTemp/addresses/.env.host');
    const content = `KMS_VERIFIER_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.appendFileSync(envFilePath, content, { flag: 'a' });
      console.info(`KMSVerifier address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write KMSVerifier address:', err);
    }

    const solidityTemplate = `
address constant kmsVerifierAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./fhevmTemp/addresses/FHEVMHostAddresses.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'a',
      });
      console.info('./fhevmTemp/addresses/FHEVMHostAddresses.sol file appended with kmsVerifierAdd successfully!');
    } catch (error) {
      console.error('Failed to write ./fhevmTemp/addresses/FHEVMHostAddresses.sol', error);
    }
  });

task('task:setInputVerifierAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    // this script also computes the coprocessor address from its private key
    const envFilePath = path.join(__dirname, '../fhevmTemp/addresses/.env.host');
    const content = `INPUT_VERIFIER_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.appendFileSync(envFilePath, content, { flag: 'a' });
      console.log(`InputVerifier address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write InputVerifier address:', err);
    }

    const solidityTemplate = `
address constant inputVerifierAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./fhevmTemp/addresses/FHEVMHostAddresses.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'a',
      });
      console.log('./fhevmTemp/addresses/FHEVMHostAddresses.sol file appended with inputVerifierAdd successfully!');
    } catch (error) {
      console.error('Failed to write ./fhevmTemp/addresses/FHEVMHostAddresses.sol', error);
    }
  });

task('task:setHCULimitAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../fhevmTemp/addresses/.env.host');
    const content = `HCU_LIMIT_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.appendFileSync(envFilePath, content, { flag: 'a' });
      console.log(`HCULimit address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write HCULimit address:', err);
    }

    const solidityTemplate = `
address constant hcuLimitAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./fhevmTemp/addresses/FHEVMHostAddresses.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'a',
      });
      console.log('./fhevmTemp/addresses/FHEVMHostAddresses.sol file appended with hcuLimitAdd successfully!');
    } catch (error) {
      console.error('Failed to write ./fhevmTemp/addresses/FHEVMHostAddresses.sol', error);
    }
  });

task('task:setDecryptionOracleAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../fhevmTemp/addresses/.env.decryptionoracle');
    const content = `DECRYPTION_ORACLE_ADDRESS=${taskArguments.address}`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log('decryptionOracleAddress written to ./fhevmTemp/addresses/.env.decryptionoracle successfully!');
    } catch (err) {
      console.error('Failed to write to ./fhevmTemp/addresses/.env.decryptionoracle:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant decryptionOracleAdd = ${taskArguments.address};
`;

    try {
      fs.writeFileSync('./fhevmTemp/addresses/DecryptionOracleAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.log('./fhevmTemp/addresses/DecryptionOracleAddress.sol file has been generated successfully.');
    } catch (error) {
      console.error('Failed to write ./fhevmTemp/addresses/DecryptionOracleAddress.sol', error);
    }
  });
