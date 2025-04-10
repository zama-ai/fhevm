import { HardhatUpgrades } from '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import { Wallet } from 'ethers';
import * as fs from 'fs-extra';
import { task, types } from 'hardhat/config';
import type { HardhatEthersHelpers, TaskArguments } from 'hardhat/types';
import path from 'path';

import { getRequiredEnvVar } from './utils/loadVariables';

task('task:deployAllHostContracts').setAction(async function (_, hre) {
  await hre.run('clean');
  await hre.run('compile:specific', { contract: 'examples/' });
  await hre.run('compile:specific', { contract: 'httpzTemp/contracts/emptyProxy' });
  await hre.run('task:deployEmptyUUPSProxies');
  // It needs to recompile to account for the change in addresses.
  await hre.run('compile:specific', { contract: 'httpzTemp/contracts/' });
  await hre.run('task:deployACL');
  await hre.run('task:deployTFHEExecutor');
  await hre.run('task:deployKMSVerifier');
  await hre.run('task:deployInputVerifier');
  await hre.run('task:deployFHEGasLimit');
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

  const tfheExecutorAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setTFHEExecutorAddress', {
    address: tfheExecutorAddress,
  });

  const kmsVerifierAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setKMSVerifierAddress', {
    address: kmsVerifierAddress,
  });

  const inputVerifierAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setInputVerifierAddress', {
    address: inputVerifierAddress,
  });

  const fheGasLimitAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setFHEGasLimitAddress', {
    address: fheGasLimitAddress,
  });
});

task('task:deployDecryptionOracle').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');

  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory(
    'httpzTemp/contracts/emptyProxy/EmptyUUPSProxy.sol:EmptyUUPSProxy',
    deployer,
  );
  const newImplem = await ethers.getContractFactory('DecryptionOracle', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('httpzTemp/addresses/.env.decryptionoracle'));
  const proxyAddress = parsedEnv.DECRYPTION_ORACLE_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem);
  console.info('DecryptionOracle code set successfully at address:', proxyAddress);
});

task('task:deployACL').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory(
    'httpzTemp/contracts/emptyProxy/EmptyUUPSProxy.sol:EmptyUUPSProxy',
    deployer,
  );
  const newImplem = await ethers.getContractFactory('ACL', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('httpzTemp/addresses/.env.acl'));
  const proxyAddress = parsedEnv.ACL_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem);
  console.info('ACL code set successfully at address:', proxyAddress);
});

task('task:deployTFHEExecutor').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory(
    'httpzTemp/contracts/emptyProxy/EmptyUUPSProxy.sol:EmptyUUPSProxy',
    deployer,
  );
  let newImplem;
  if (process.env.HARDHAT_TFHEEXECUTOR_EVENTS !== '1') {
    newImplem = await ethers.getContractFactory('httpzTemp/contracts/TFHEExecutor.sol:TFHEExecutor', deployer);
  } else {
    newImplem = await ethers.getContractFactory('httpzTemp/contracts/TFHEExecutor.sol:TFHEExecutor', deployer);
  }
  const parsedEnv = dotenv.parse(fs.readFileSync('httpzTemp/addresses/.env.exec'));
  const proxyAddress = parsedEnv.TFHE_EXECUTOR_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem);
  console.info('TFHEExecutor code set successfully at address:', proxyAddress);
});

task('task:deployKMSVerifier').setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory(
    'httpzTemp/contracts/emptyProxy/EmptyUUPSProxy.sol:EmptyUUPSProxy',
    deployer,
  );
  const newImplem = await ethers.getContractFactory('httpzTemp/contracts/KMSVerifier.sol:KMSVerifier', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('httpzTemp/addresses/.env.kmsverifier'));
  const proxyAddress = parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);

  const verifyingContractSource = process.env.DECRYPTION_MANAGER_ADDRESS!;
  const chainIDSource = +process.env.CHAIN_ID_GATEWAY!;
  const initialThreshold = +process.env.KMS_THRESHOLD!;
  let initialSigners: string[] = [];
  const numSigners = getRequiredEnvVar('NUM_KMS_NODES');

  for (let idx = 0; idx < +numSigners; idx++) {
    const kmsSignerAddress = getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`);
    initialSigners.push(kmsSignerAddress);
  }
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: { fn: 'reinitialize', args: [verifyingContractSource, chainIDSource, initialSigners, initialThreshold] },
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
      'httpzTemp/contracts/emptyProxy/EmptyUUPSProxy.sol:EmptyUUPSProxy',
      deployer,
    );
    const newImplem = await ethers.getContractFactory('httpzTemp/contracts/InputVerifier.sol:InputVerifier', deployer);
    const parsedEnv = dotenv.parse(fs.readFileSync('httpzTemp/addresses/.env.inputverifier'));

    const proxyAddress = parsedEnv.INPUT_VERIFIER_CONTRACT_ADDRESS;
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const verifyingContractSource = process.env.ZKPOK_MANAGER_ADDRESS!;
    const chainIDSource = +process.env.CHAIN_ID_GATEWAY!;

    let initialSigners: string[] = [];
    const numSigners = +process.env.NUM_KMS_NODES!;
    for (let idx = 0; idx < numSigners; idx++) {
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
      call: { fn: 'reinitialize', args: [verifyingContractSource, chainIDSource, initialSigners] },
    });
    console.info('InputVerifier code set successfully at address:', proxyAddress);
  });

task('task:deployFHEGasLimit').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory(
    'httpzTemp/contracts/emptyProxy/EmptyUUPSProxy.sol:EmptyUUPSProxy',
    deployer,
  );
  const newImplem = await ethers.getContractFactory('FHEGasLimit', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('httpzTemp/addresses/.env.fhegaslimit'));
  const proxyAddress = parsedEnv.FHE_GASLIMIT_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem);
  console.info('FHEGasLimit code set successfully at address:', proxyAddress);
});

task('task:setACLAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../httpzTemp/addresses/.env.acl');
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
      fs.writeFileSync('./httpzTemp/addresses/ACLAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.info('./httpzTemp/addresses/ACLAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./httpzTemp/addresses/ACLAddress.sol', error);
    }
  });

task('task:setTFHEExecutorAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../httpzTemp/addresses/.env.exec');
    const content = `TFHE_EXECUTOR_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.info(`TFHEExecutor address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write TFHEExecutor address:', err);
    }

    const solidityTemplateCoprocessor = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant tfheExecutorAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./httpzTemp/addresses/TFHEExecutorAddress.sol', solidityTemplateCoprocessor, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.info('./httpzTemp/addresses/TFHEExecutorAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./httpzTemp/addresses/TFHEExecutorAddress.sol', error);
    }
  });

task('task:setKMSVerifierAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../httpzTemp/addresses/.env.kmsverifier');
    const content = `KMS_VERIFIER_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.info(`KMSVerifier address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write KMSVerifier address:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant kmsVerifierAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./httpzTemp/addresses/KMSVerifierAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.info('./httpzTemp/addresses/KMSVerifierAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./httpzTemp/addresses/KMSVerifierAddress.sol', error);
    }
  });

task('task:setInputVerifierAddress')
  .addParam('address', 'The address of the contract')
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private key env variable for coprocessor',
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    // this script also computes the coprocessor address from its private key
    const envFilePath = path.join(__dirname, '../httpzTemp/addresses/.env.inputverifier');
    const content = `INPUT_VERIFIER_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.info(`InputVerifier address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write InputVerifier address:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant inputVerifierAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./httpzTemp/addresses/InputVerifierAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.info('./httpzTemp/addresses/InputVerifierAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./httpzTemp/addresses/InputVerifierAddress.sol', error);
    }

    if (process.env.IS_COPROCESSOR) {
      let coprocAddress;
      if (!taskArguments.useAddress) {
        coprocAddress = new ethers.Wallet(process.env.PRIVATE_KEY_COPROCESSOR_ACCOUNT!).address;
      } else {
        coprocAddress = process.env.ADDRESS_COPROCESSOR_ACCOUNT;
      }
      const envFilePath2 = path.join(__dirname, '../httpzTemp/addresses/.env.coprocessor');
      const content2 = `COPROCESSOR_ADDRESS=${coprocAddress}\n`;
      try {
        fs.writeFileSync(envFilePath2, content2, { flag: 'w' });
        console.info(`Coprocessor address ${coprocAddress} written successfully!`);
      } catch (err) {
        console.error('Failed to write InputVerifier address:', err);
      }

      const solidityTemplate2 = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant coprocessorAdd = ${coprocAddress};\n`;

      try {
        fs.writeFileSync('./httpzTemp/addresses/CoprocessorAddress.sol', solidityTemplate2, {
          encoding: 'utf8',
          flag: 'w',
        });
        console.info('./httpzTemp/addresses/CoprocessorAddress.sol file generated successfully!');
      } catch (error) {
        console.error('Failed to write ./httpzTemp/addresses/CoprocessorAddress.sol', error);
      }
    }
  });

task('task:setFHEGasLimitAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../httpzTemp/addresses/.env.fhegaslimit');
    const content = `FHE_GASLIMIT_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.info(`FHEGasLimit address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write FHEGasLimit address:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant fheGasLimitAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./httpzTemp/addresses/FHEGasLimitAddress.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.info('./httpzTemp/addresses/FHEGasLimitAddress.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./httpzTemp/addresses/FHEGasLimitAddress.sol', error);
    }
  });
