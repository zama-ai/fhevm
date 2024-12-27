import dotenv from 'dotenv';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';

import { KMSVerifier } from '../types';

task('task:deployDecryptionOracle')
  .addParam('privateKey', 'The deployer private key')
  .addParam('ownerAddress', 'The owner address')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('DecryptionOracle', deployer);
    const DecryptionOracle = await upgrades.deployProxy(factory, [taskArguments.ownerAddress], {
      initializer: 'initialize',
      kind: 'uups',
    });
    await DecryptionOracle.waitForDeployment();
    const DecryptionOracleAddress = await DecryptionOracle.getAddress();
    const envConfig = dotenv.parse(
      fs.readFileSync('node_modules/fhevm-core-contracts/addresses/.env.decryptionoracle'),
    );
    if (DecryptionOracleAddress !== envConfig.DECRYPTION_ORACLE_ADDRESS) {
      throw new Error(
        `The nonce of the deployer account is not null. Please use another deployer private key or relaunch a clean instance of the fhEVM`,
      );
    }
    console.log('DecryptionOracle was deployed at address: ', DecryptionOracleAddress);
  });

task('task:deployACL')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('fhevmTemp/contracts/ACL.sol:ACL', deployer);
    const acl = await upgrades.deployProxy(factory, [deployer.address], { initializer: 'initialize', kind: 'uups' });
    await acl.waitForDeployment();
    const address = await acl.getAddress();
    const envConfigAcl = dotenv.parse(fs.readFileSync('node_modules/fhevm-core-contracts/addresses/.env.acl'));
    if (address !== envConfigAcl.ACL_CONTRACT_ADDRESS) {
      throw new Error(
        `The nonce of the deployer account is not correct. Please relaunch a clean instance of the fhEVM`,
      );
    }
    console.log('ACL was deployed at address:', address);
  });

task('task:deployTFHEExecutor')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    let factory;
    if (process.env.HARDHAT_TFHEEXECUTOR_EVENTS !== '1') {
      factory = await ethers.getContractFactory('fhevmTemp/contracts/TFHEExecutor.sol:TFHEExecutor', deployer);
    } else {
      factory = await ethers.getContractFactory(
        'fhevmTemp/contracts/TFHEExecutorWithEvents.sol:TFHEExecutorWithEvents',
        deployer,
      );
    }
    const exec = await upgrades.deployProxy(factory, [deployer.address], { initializer: 'initialize', kind: 'uups' });
    await exec.waitForDeployment();
    const address = await exec.getAddress();
    const envConfig = dotenv.parse(fs.readFileSync('node_modules/fhevm-core-contracts/addresses/.env.exec'));
    if (address !== envConfig.TFHE_EXECUTOR_CONTRACT_ADDRESS) {
      throw new Error(
        `The nonce of the deployer account is not correct. Please relaunch a clean instance of the fhEVM`,
      );
    }
    console.log('TFHEExecutor was deployed at address:', address);
  });

task('task:deployKMSVerifier')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('fhevmTemp/contracts/KMSVerifier.sol:KMSVerifier', deployer);
    const kms = await upgrades.deployProxy(factory, [deployer.address], { initializer: 'initialize', kind: 'uups' });
    await kms.waitForDeployment();
    const address = await kms.getAddress();
    const envConfig = dotenv.parse(fs.readFileSync('node_modules/fhevm-core-contracts/addresses/.env.kmsverifier'));
    if (address !== envConfig.KMS_VERIFIER_CONTRACT_ADDRESS) {
      throw new Error(
        `The nonce of the deployer account is not correct. Please relaunch a clean instance of the fhEVM`,
      );
    }
    console.log('KMSVerifier was deployed at address:', address);
  });

task('task:deployInputVerifier')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    let factory;
    if (process.env.IS_COPROCESSOR === 'true') {
      factory = await ethers.getContractFactory(
        'fhevmTemp/contracts/InputVerifier.coprocessor.sol:InputVerifier',
        deployer,
      );
    } else {
      factory = await ethers.getContractFactory('fhevmTemp/contracts/InputVerifier.native.sol:InputVerifier', deployer);
    }
    const kms = await upgrades.deployProxy(factory, [deployer.address], { initializer: 'initialize', kind: 'uups' });
    await kms.waitForDeployment();
    const address = await kms.getAddress();
    const envConfig = dotenv.parse(fs.readFileSync('node_modules/fhevm-core-contracts/addresses/.env.inputverifier'));
    if (address !== envConfig.INPUT_VERIFIER_CONTRACT_ADDRESS) {
      throw new Error(
        `The nonce of the deployer account is not correct. Please relaunch a clean instance of the fhEVM`,
      );
    }
    console.log('InputVerifier was deployed at address:', address);
  });

task('task:deployFHEGasLimit')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('fhevmTemp/contracts/FHEGasLimit.sol:FHEGasLimit', deployer);
    const payment = await upgrades.deployProxy(factory, [deployer.address], {
      initializer: 'initialize',
      kind: 'uups',
    });
    await payment.waitForDeployment();
    const address = await payment.getAddress();
    const envConfig = dotenv.parse(fs.readFileSync('node_modules/fhevm-core-contracts/addresses/.env.fhegaslimit'));
    if (address !== envConfig.FHE_GASLIMIT_CONTRACT_ADDRESS) {
      throw new Error(
        `The nonce of the deployer account is not correct. Please relaunch a clean instance of the fhEVM`,
      );
    }
    console.log('FHEGasLimit was deployed at address:', address);
  });

task('task:addSigners')
  .addParam('privateKey', 'The deployer private key')
  .addParam('numSigners', 'Number of KMS signers to add')
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private keys env variables for kms signers',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'customKmsVerifierAddress',
    'Use a custom address for the KMSVerifier contract instead of the default one - ie stored inside .env.kmsverifier',
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('fhevmTemp/contracts/KMSVerifier.sol:KMSVerifier', deployer);
    let kmsAdd;
    if (taskArguments.customKmsVerifierAddress) {
      kmsAdd = taskArguments.customKmsVerifierAddress;
    } else {
      kmsAdd = dotenv.parse(
        fs.readFileSync('node_modules/fhevm-core-contracts/addresses/.env.kmsverifier'),
      ).KMS_VERIFIER_CONTRACT_ADDRESS;
    }
    const kmsVerifier = await factory.attach(kmsAdd);
    for (let idx = 0; idx < taskArguments.numSigners; idx++) {
      if (!taskArguments.useAddress) {
        const privKeySigner = process.env[`PRIVATE_KEY_KMS_SIGNER_${idx}`];
        const kmsSigner = new ethers.Wallet(privKeySigner).connect(ethers.provider);
        const tx = await kmsVerifier.addSigner(kmsSigner.address);
        await tx.wait();
        console.log(`KMS signer no${idx} (${kmsSigner.address}) was added to KMSVerifier contract`);
      } else {
        const kmsSignerAddress = process.env[`ADDRESS_KMS_SIGNER_${idx}`];
        const tx = await kmsVerifier.addSigner(kmsSignerAddress);
        await tx.wait();
        console.log(`KMS signer no${idx} (${kmsSignerAddress}) was added to KMSVerifier contract`);
      }
    }
  });

task('task:getAllSigners')
  .addOptionalParam(
    'customKmsVerifierAddress',
    'Use a custom address for the KMSVerifier contract instead of the default one - ie stored inside .env.kmsverifier',
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const factory = await ethers.getContractFactory('fhevmTemp/contracts/KMSVerifier.sol:KMSVerifier');
    let kmsAdd;
    if (taskArguments.customKmsVerifierAddress) {
      kmsAdd = taskArguments.customKmsVerifierAddress;
    } else {
      kmsAdd = dotenv.parse(
        fs.readFileSync('node_modules/fhevm-core-contracts/addresses/.env.kmsverifier'),
      ).KMS_VERIFIER_CONTRACT_ADDRESS;
    }
    const kmsVerifier = (await factory.attach(kmsAdd).connect(ethers.provider)) as KMSVerifier;
    const listCurrentKMSSigners = await kmsVerifier.getSigners();
    console.log('The list of current KMS Signers stored inside KMSVerifier contract is: ', listCurrentKMSSigners);
  });

task('task:removeSigner')
  .addParam('privateKey', 'The KMSVerifier owner private key')
  .addParam('kmsSignerAddress', 'The KMS Signer address you wish to remove')
  .addOptionalParam(
    'customKmsVerifierAddress',
    'Use a custom address for the KMSVerifier contract instead of the default one - ie stored inside .env.kmsverifier',
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('fhevmTemp/contracts/KMSVerifier.sol:KMSVerifier', deployer);
    let kmsAdd;
    if (taskArguments.customKmsVerifierAddress) {
      kmsAdd = taskArguments.customKmsVerifierAddress;
    } else {
      kmsAdd = dotenv.parse(
        fs.readFileSync('node_modules/fhevm-core-contracts/addresses/.env.kmsverifier'),
      ).KMS_VERIFIER_CONTRACT_ADDRESS;
    }
    const kmsVerifier = (await factory.attach(kmsAdd)) as KMSVerifier;
    const tx = await kmsVerifier.removeSigner(taskArguments.kmsSignerAddress);
    await tx.wait();
    console.log(`KMS signer with address (${taskArguments.kmsSignerAddress}) was removed from KMSVerifier contract`);
  });
