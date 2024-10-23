import dotenv from 'dotenv';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';

task('task:deployGateway')
  .addParam('privateKey', 'The deployer private key')
  .addParam('ownerAddress', 'The owner address')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('GatewayContract', deployer);
    const Gateway = await upgrades.deployProxy(factory, [taskArguments.ownerAddress], {
      initializer: 'initialize',
      kind: 'uups',
    });
    await Gateway.waitForDeployment();
    const GatewayContractAddress = await Gateway.getAddress();
    const envConfig = dotenv.parse(fs.readFileSync('gateway/.env.gateway'));
    if (GatewayContractAddress !== envConfig.GATEWAY_CONTRACT_PREDEPLOY_ADDRESS) {
      throw new Error(
        `The nonce of the deployer account is not null. Please use another deployer private key or relaunch a clean instance of the fhEVM`,
      );
    }
    console.log('GatewayContract was deployed at address: ', GatewayContractAddress);
  });

task('task:deployACL')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('ACL', deployer);
    const acl = await upgrades.deployProxy(factory, [deployer.address], { initializer: 'initialize', kind: 'uups' });
    await acl.waitForDeployment();
    const address = await acl.getAddress();
    const envConfigAcl = dotenv.parse(fs.readFileSync('lib/.env.acl'));
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
      factory = await ethers.getContractFactory('lib/TFHEExecutor.sol:TFHEExecutor', deployer);
    } else {
      factory = await ethers.getContractFactory('lib/TFHEExecutor.events.sol:TFHEExecutor', deployer);
    }
    const exec = await upgrades.deployProxy(factory, [deployer.address], { initializer: 'initialize', kind: 'uups' });
    await exec.waitForDeployment();
    const address = await exec.getAddress();
    const envConfig = dotenv.parse(fs.readFileSync('lib/.env.exec'));
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
    const factory = await ethers.getContractFactory('KMSVerifier', deployer);
    const kms = await upgrades.deployProxy(factory, [deployer.address], { initializer: 'initialize', kind: 'uups' });
    await kms.waitForDeployment();
    const address = await kms.getAddress();
    const envConfig = dotenv.parse(fs.readFileSync('lib/.env.kmsverifier'));
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
      factory = await ethers.getContractFactory('lib/InputVerifier.coprocessor.sol:InputVerifier', deployer);
    } else {
      factory = await ethers.getContractFactory('lib/InputVerifier.native.sol:InputVerifier', deployer);
    }
    const kms = await upgrades.deployProxy(factory, [deployer.address], { initializer: 'initialize', kind: 'uups' });
    await kms.waitForDeployment();
    const address = await kms.getAddress();
    const envConfig = dotenv.parse(fs.readFileSync('lib/.env.inputverifier'));
    if (address !== envConfig.INPUT_VERIFIER_CONTRACT_ADDRESS) {
      throw new Error(
        `The nonce of the deployer account is not correct. Please relaunch a clean instance of the fhEVM`,
      );
    }
    console.log('InputVerifier was deployed at address:', address);
  });

task('task:deployFHEPayment')
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('FHEPayment', deployer);
    const payment = await upgrades.deployProxy(factory, [deployer.address], {
      initializer: 'initialize',
      kind: 'uups',
    });
    await payment.waitForDeployment();
    const address = await payment.getAddress();
    const envConfig = dotenv.parse(fs.readFileSync('lib/.env.fhepayment'));
    if (address !== envConfig.FHE_PAYMENT_CONTRACT_ADDRESS) {
      throw new Error(
        `The nonce of the deployer account is not correct. Please relaunch a clean instance of the fhEVM`,
      );
    }
    console.log('FHEPayment was deployed at address:', address);
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
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const factory = await ethers.getContractFactory('KMSVerifier', deployer);
    const kmsAdd = dotenv.parse(fs.readFileSync('lib/.env.kmsverifier')).KMS_VERIFIER_CONTRACT_ADDRESS;
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
