import { HardhatUpgrades } from '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import { Wallet } from 'ethers';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { HardhatEthersHelpers, TaskArguments } from 'hardhat/types';
import path from 'path';

import { InputVerifier, KMSVerifier } from '../types';
import { getRequiredEnvVar } from './utils/loadVariables';

async function deployEmptyUUPSForACL(ethers: HardhatEthersHelpers, upgrades: HardhatUpgrades, deployer: Wallet) {
  console.log('Deploying an EmptyUUPSProxyACL proxy contract...');
  const factory = await ethers.getContractFactory('EmptyUUPSProxyACL', deployer);
  const UUPSEmptyACL = await upgrades.deployProxy(factory, [deployer.address], {
    initializer: 'initialize',
    kind: 'uups',
  });
  await UUPSEmptyACL.waitForDeployment();
  const UUPSEmptyACLAddress = await UUPSEmptyACL.getAddress();
  console.log('EmptyUUPSProxyACL proxy contract successfully deployed!');
  return UUPSEmptyACLAddress;
}

async function deployEmptyUUPS(ethers: HardhatEthersHelpers, upgrades: HardhatUpgrades, deployer: Wallet) {
  console.log('Deploying an EmptyUUPSProxy proxy contract...');
  const factory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const UUPSEmpty = await upgrades.deployProxy(factory, {
    initializer: 'initialize',
    kind: 'uups',
  });
  await UUPSEmpty.waitForDeployment();
  const UUPSEmptyAddress = await UUPSEmpty.getAddress();
  console.log('EmptyUUPSProxy proxy contract successfully deployed!');
  return UUPSEmptyAddress;
}

task('task:deployEmptyUUPSProxies').setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
  // Compile the EmptyUUPS proxy contract for ACL
  await run('compile:specific', { contract: 'contracts/emptyProxyACL' });

  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);

  // Ensure the addresses directory exists
  fs.mkdirSync(path.join(__dirname, '../addresses'), { recursive: true });

  const aclAddress = await deployEmptyUUPSForACL(ethers, upgrades, deployer);
  await run('task:setACLAddress', { address: aclAddress });

  // Compile the EmptyUUPS proxy contract for other contracts
  await run('compile:specific', { contract: 'contracts/emptyProxy' });

  const fhevmExecutorAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setFHEVMExecutorAddress', { address: fhevmExecutorAddress });

  const kmsVerifierAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setKMSVerifierAddress', { address: kmsVerifierAddress });

  const inputVerifierAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setInputVerifierAddress', { address: inputVerifierAddress });

  const HCULimitAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run('task:setHCULimitAddress', { address: HCULimitAddress });
});

task('task:deployDecryptionOracle').setAction(async function (_, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const factory = await ethers.getContractFactory('DecryptionOracle', deployer);
  const decryptionOracle = await upgrades.deployProxy(factory, [deployer.address], {
    initializer: 'initialize',
    kind: 'uups',
  });
  await decryptionOracle.waitForDeployment();
  const proxyAddress = await decryptionOracle.getAddress();
  console.log('DecryptionOracle code set successfully at address:', proxyAddress);
  // Ensure the addresses/ directory exists or create it
  fs.mkdirSync('./addresses', { recursive: true });
  const envFilePath = path.join(__dirname, '../addresses/.env.decryptionoracle');
  const content = `DECRYPTION_ORACLE_ADDRESS=${proxyAddress}`;
  try {
    fs.writeFileSync(envFilePath, content, { flag: 'w' });
    console.log('decryptionOracleAddress written to addresses/.env.decryptionoracle successfully!');
  } catch (err) {
    console.error('Failed to write to addresses/.env.decryptionoracle:', err);
  }

  const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant decryptionOracleAdd = ${proxyAddress};
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

task('task:deployACL').setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('ACL', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.host'));
  const proxyAddress = parsedEnv.ACL_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: { fn: 'initializeFromEmptyProxy' },
  });
  console.log('ACL code set successfully at address:', proxyAddress);
});

task('task:deployFHEVMExecutor').setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('./contracts/FHEVMExecutor.sol:FHEVMExecutor', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.host'));
  const proxyAddress = parsedEnv.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, { call: { fn: 'initializeFromEmptyProxy' } });
  console.log('FHEVMExecutor code set successfully at address:', proxyAddress);
});

task('task:deployKMSVerifier')
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private keys env variables for kms signers',
    true,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
    const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
    const newImplem = await ethers.getContractFactory('KMSVerifier', deployer);
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.host'));
    const proxyAddress = parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS;
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const verifyingContractSource = process.env.DECRYPTION_ADDRESS!;
    const chainIDSource = +process.env.CHAIN_ID_GATEWAY!;
    const initialThreshold = +process.env.PUBLIC_DECRYPTION_THRESHOLD!;
    let initialSigners: string[] = [];
    const numSigners = getRequiredEnvVar('NUM_KMS_NODES');
    for (let idx = 0; idx < +numSigners; idx++) {
      if (!taskArguments.useAddress) {
        const privKeySigner = getRequiredEnvVar(`PRIVATE_KEY_KMS_SIGNER_${idx}`);
        const kmsSigner = new ethers.Wallet(privKeySigner).connect(ethers.provider);
        initialSigners.push(kmsSigner.address);
      } else {
        const kmsSignerAddress = getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`);
        initialSigners.push(kmsSignerAddress);
      }
    }
    await upgrades.upgradeProxy(proxy, newImplem, {
      call: {
        fn: 'initializeFromEmptyProxy',
        args: [verifyingContractSource, chainIDSource, initialSigners, initialThreshold],
      },
    });
    console.log('KMSVerifier code set successfully at address:', proxyAddress);
    console.log(
      `${numSigners} KMS signers were added to KMSVerifier at initialization, list of KMS signers is:`,
      initialSigners,
    );
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
    const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
    const newImplem = await ethers.getContractFactory('./contracts/InputVerifier.sol:InputVerifier', deployer);
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.host'));
    const proxyAddress = parsedEnv.INPUT_VERIFIER_CONTRACT_ADDRESS;
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const verifyingContractSource = process.env.INPUT_VERIFICATION_ADDRESS!;
    const chainIDSource = +process.env.CHAIN_ID_GATEWAY!;

    // Load the initial coprocessor context from environment variables.
    let initialContextSigners: string[] = [];
    const numSigners = getRequiredEnvVar('NUM_COPROCESSORS');
    const initialContextId = getRequiredEnvVar('COPROCESSOR_CONTEXT_ID');
    for (let idx = 0; idx < parseInt(numSigners); idx++) {
      if (!taskArguments.useAddress) {
        const privKeySigner = getRequiredEnvVar(`PRIVATE_KEY_COPROCESSOR_ACCOUNT_${idx}`);
        const inputSigner = new ethers.Wallet(privKeySigner).connect(ethers.provider);
        initialContextSigners.push(inputSigner.address);
      } else {
        const inputSignerAddress = getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`);
        initialContextSigners.push(inputSignerAddress);
      }
    }

    await upgrades.upgradeProxy(proxy, newImplem, {
      call: {
        fn: 'initializeFromEmptyProxy',
        args: [verifyingContractSource, chainIDSource, initialContextId, initialContextSigners],
      },
    });
    console.log('InputVerifier code set successfully at address:', proxyAddress);
    console.log(
      `${numSigners} Coprocessor signers were added to InputVerifier at initialization for context ID ${initialContextId}, list of Coprocessor signers is:`,
      initialContextSigners,
    );
  });

task('task:deployHCULimit').setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('HCULimit', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.host'));
  const proxyAddress = parsedEnv.HCU_LIMIT_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, { call: { fn: 'initializeFromEmptyProxy' } });
  console.log('HCULimit code set successfully at address:', proxyAddress);
});

task('task:getKmsSigners')
  .addOptionalParam(
    'customKmsVerifierAddress',
    'Use a custom address for the KMSVerifier contract instead of the default one - ie stored inside .env.host',
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const factory = await ethers.getContractFactory('./contracts/KMSVerifier.sol:KMSVerifier');
    let kmsAdd;
    if (taskArguments.customKmsVerifierAddress) {
      kmsAdd = taskArguments.customKmsVerifierAddress;
    } else {
      kmsAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).KMS_VERIFIER_CONTRACT_ADDRESS;
    }
    const kmsVerifier = factory.attach(kmsAdd).connect(ethers.provider) as KMSVerifier;
    const listCurrentKMSSigners = await kmsVerifier.getKmsSigners();
    console.log('The list of current KMS Signers stored inside KMSVerifier contract is: ', listCurrentKMSSigners);
  });

task('task:getCoprocessorSigners')
  .addOptionalParam(
    'customInputVerifierAddress',
    'Use a custom address for the InputVerifier contract instead of the default one - ie stored inside .env.host',
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const factory = await ethers.getContractFactory('./contracts/InputVerifier.sol:InputVerifier');
    let inputVerifierAdd;
    if (taskArguments.customInputVerifierAddress) {
      inputVerifierAdd = taskArguments.customInputVerifierAddress;
    } else {
      inputVerifierAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).INPUT_VERIFIER_CONTRACT_ADDRESS;
    }
    const inputVerifier = factory.attach(inputVerifierAdd).connect(ethers.provider) as InputVerifier;
    const listCurrentCoprocessorSigners = await inputVerifier.getCoprocessorSigners();
    console.log(
      'The list of current Coprocessor Signers stored inside InputVerifier contract is: ',
      listCurrentCoprocessorSigners,
    );
  });

task('task:setDecryptionOracleAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../addresses/.env.decryptionoracle');
    const content = `DECRYPTION_ORACLE_ADDRESS=${taskArguments.address}`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log('DecryptionOracle address written to addresses/.env.decryptionoracle successfully!');
    } catch (err) {
      console.error('Failed to write to addresses/.env.decryptionoracle:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant decryptionOracleAdd = ${taskArguments.address};
`;

    try {
      fs.writeFileSync('./addresses/DecryptionOracleAddress.sol', solidityTemplate, { encoding: 'utf8', flag: 'w' });
      console.log('addresses/DecryptionOracleAddress.sol file has been generated successfully.');
    } catch (error) {
      console.error('Failed to write addresses/DecryptionOracleAddress.sol', error);
    }
  });

task('task:setACLAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../addresses/.env.host');
    const content = `ACL_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.log(`ACL address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write ACL address:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant aclAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./addresses/FHEVMHostAddresses.sol', solidityTemplate, { encoding: 'utf8', flag: 'w' });
      console.log('./addresses/FHEVMHostAddresses.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./addresses/FHEVMHostAddresses.sol', error);
    }
  });

task('task:setFHEVMExecutorAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../addresses/.env.host');
    const content = `FHEVM_EXECUTOR_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.appendFileSync(envFilePath, content, { flag: 'a' });
      console.log(`FHEVMExecutor address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write FHEVMExecutor address:', err);
    }

    const solidityTemplate = `
address constant fhevmExecutorAdd = ${taskArguments.address};\n`;

    try {
      fs.appendFileSync('./addresses/FHEVMHostAddresses.sol', solidityTemplate, { encoding: 'utf8', flag: 'a' });
      console.log('./addresses/FHEVMHostAddresses.sol file appended with fhevmExecutorAdd successfully!');
    } catch (error) {
      console.error('Failed to write ./addresses/FHEVMHostAddresses.sol', error);
    }
  });

task('task:setKMSVerifierAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../addresses/.env.host');
    const content = `KMS_VERIFIER_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.appendFileSync(envFilePath, content, { flag: 'a' });
      console.log(`KMSVerifier address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write KMSVerifier address:', err);
    }

    const solidityTemplate = `
address constant kmsVerifierAdd = ${taskArguments.address};\n`;

    try {
      fs.appendFileSync('./addresses/FHEVMHostAddresses.sol', solidityTemplate, { encoding: 'utf8', flag: 'a' });
      console.log('./addresses/FHEVMHostAddresses.sol file appended with kmsVerifierAdd successfully!');
    } catch (error) {
      console.error('Failed to write ./addresses/FHEVMHostAddresses.sol', error);
    }
  });

task('task:setInputVerifierAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    // this script also computes the coprocessor address from its private key
    const envFilePath = path.join(__dirname, '../addresses/.env.host');
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
      fs.appendFileSync('./addresses/FHEVMHostAddresses.sol', solidityTemplate, { encoding: 'utf8', flag: 'a' });
      console.log('./addresses/FHEVMHostAddresses.sol file appended with inputVerifierAdd successfully!');
    } catch (error) {
      console.error('Failed to write ./addresses/FHEVMHostAddresses.sol', error);
    }
  });

task('task:setHCULimitAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../addresses/.env.host');
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
      fs.appendFileSync('./addresses/FHEVMHostAddresses.sol', solidityTemplate, { encoding: 'utf8', flag: 'a' });
      console.log('./addresses/FHEVMHostAddresses.sol appended with hcuLimitAdd successfully!');
    } catch (error) {
      console.error('Failed to write ./addresses/FHEVMHostAddresses.sol', error);
    }
  });

task('task:setPauserSetAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../addresses/.env.host');
    const content = `PAUSER_SET_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.appendFileSync(envFilePath, content, { flag: 'a' });
      console.log(`PauserSet address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write PauserSet address:', err);
    }

    const solidityTemplate = `
address constant pauserSetAdd = ${taskArguments.address};\n`;

    try {
      fs.appendFileSync('./addresses/FHEVMHostAddresses.sol', solidityTemplate, { encoding: 'utf8', flag: 'a' });
      console.log('./addresses/FHEVMHostAddresses.sol appended with hcuLimitAdd successfully!');
    } catch (error) {
      console.error('Failed to write ./addresses/FHEVMHostAddresses.sol', error);
    }
  });

task('task:addInputSigners')
  .addParam('privateKey', 'The deployer private key')
  .addParam('numSigners', 'Number of coprocessor signers to add')
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private keys env variables for kms signers',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'customInputVerifierAddress',
    'Use a custom address for the InputVerifier contract instead of the default one - ie stored inside .env.host',
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('./contracts/InputVerifier.sol:InputVerifier', deployer);
    let inputAdd;
    if (taskArguments.customInputVerifierAddress) {
      inputAdd = taskArguments.customInputVerifierAddress;
    } else {
      inputAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).INPUT_VERIFIER_CONTRACT_ADDRESS;
    }
    const inputVerifier = (await factory.attach(inputAdd)) as InputVerifier;
    for (let idx = 0; idx < taskArguments.numSigners; idx++) {
      if (!taskArguments.useAddress) {
        const privKeySigner = process.env[`PRIVATE_KEY_COPROCESSOR_ACCOUNT_${idx}`]!;
        const inputSigner = new ethers.Wallet(privKeySigner).connect(ethers.provider);
        const tx = await inputVerifier.addSigner(inputSigner.address);
        await tx.wait();
        console.log(`Coprocessor signer no${idx} (${inputSigner.address}) was added to InputVerifier contract`);
      } else {
        const inputSignerAddress = process.env[`COPROCESSOR_SIGNER_ADDRESS_1${idx}`]!;
        const tx = await inputVerifier.addSigner(inputSignerAddress);
        await tx.wait();
        console.log(`Coprocessor signer no${idx} (${inputSignerAddress}) was added to InputVerifier contract`);
      }
    }
  });

// Deploy the PauserSet contract
task('task:deployPauserSet').setAction(async function (_, hre) {
  // Get a deployer wallet
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  console.log('Deploying PauserSet...');
  const pauserSetFactory = await hre.ethers.getContractFactory('PauserSet', deployer);
  const pauserSet = await pauserSetFactory.deploy();
  const pauserSetAddress = await pauserSet.getAddress();

  await hre.run('task:setPauserSetAddress', {
    address: pauserSetAddress,
  });
});

task('task:deployAllHostContracts').setAction(async function (_, hre) {
  if (process.env.SOLIDITY_COVERAGE !== 'true') {
    await hre.run('clean');
  }

  await hre.run('task:deployEmptyUUPSProxies');
  await hre.run('compile:specific', { contract: 'contracts/immutable' });
  await hre.run('task:deployPauserSet');

  // The deployEmptyUUPSProxies task may have updated the contracts' addresses in `addresses/*.sol`.
  // Thus, we must re-compile the contracts with these new addresses, otherwise the old ones will be
  // used.
  await hre.run('compile:specific', { contract: 'contracts' });
  await hre.run('compile:specific', { contract: 'decryptionOracle' });

  await hre.run('task:deployACL');
  await hre.run('task:deployFHEVMExecutor');
  await hre.run('task:deployKMSVerifier');
  await hre.run('task:deployInputVerifier');
  await hre.run('task:deployHCULimit');
  await hre.run('task:deployDecryptionOracle');

  console.log('Contract deployment done!');
});
