import chalk from 'chalk';
import dotenv from 'dotenv';
import fs from 'fs';
import { task } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';

task('task:deployERC20').setAction(async function (taskArguments: TaskArguments, { ethers }) {
  const signers = await ethers.getSigners();
  const erc20Factory = await ethers.getContractFactory('EncryptedERC20');
  const encryptedERC20 = await erc20Factory.connect(signers[0]).deploy('Naraggara', 'NARA');
  await encryptedERC20.waitForDeployment();
  console.log('EncryptedERC20 deployed to: ', await encryptedERC20.getAddress());
});

task('task:deployGateway')
  .addParam('privateKey', 'The deployer private key')
  .addParam('ownerAddress', 'The owner address')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    const envConfig2 = dotenv.parse(fs.readFileSync('lib/.env.kmsverifier'));
    const gatewayFactory = await ethers.getContractFactory('GatewayContract');
    const Gateway = await gatewayFactory
      .connect(deployer)
      .deploy(taskArguments.ownerAddress, envConfig2.KMS_VERIFIER_CONTRACT_ADDRESS);
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

task('task:deployIdentity').setAction(async function (taskArguments: TaskArguments, { ethers }) {
  const signers = await ethers.getSigners();

  const identityRegistryFactory = await ethers.getContractFactory('IdentityRegistry');
  const identityRegistry = await identityRegistryFactory.connect(signers[0]).deploy();

  const erc20RulesFactory = await ethers.getContractFactory('ERC20Rules');
  const erc20Rules = await erc20RulesFactory.connect(signers[0]).deploy();
  await identityRegistry.waitForDeployment();
  await erc20Rules.waitForDeployment();

  const compliantERC20Factory = await ethers.getContractFactory('CompliantERC20');
  const compliantERC20 = await compliantERC20Factory
    .connect(signers[0])
    .deploy(await identityRegistry.getAddress(), await erc20Rules.getAddress(), 'CompliantToken', 'CTOK');
  await compliantERC20.waitForDeployment();

  const registryAddress = await identityRegistry.getAddress();
  const erc20Address = await compliantERC20.getAddress();
  console.log(chalk.bold('Available methods:'));
  console.log(`npx hardhat task:identity:initRegistry --registry ${registryAddress}`);
  console.log(`npx hardhat task:identity:grantAccess --registry ${registryAddress} --erc20 ${erc20Address}`);
  console.log(`npx hardhat task:identity:mint --erc20 ${erc20Address}`);
  console.log(`npx hardhat task:identity:transfer --erc20 ${erc20Address} --from carol --to dave --amount 2000`);
  console.log(`npx hardhat task:identity:balanceOf --erc20 ${erc20Address} --user alice`);
});

task('task:deployACL').setAction(async function (taskArguments: TaskArguments, { ethers }) {
  const deployer = (await ethers.getSigners())[9];
  const factory = await ethers.getContractFactory('ACL');
  const acl = await factory.connect(deployer).deploy();
  await acl.waitForDeployment();
  const address = await acl.getAddress();
  const envConfigAcl = dotenv.parse(fs.readFileSync('lib/.env.acl'));
  if (address !== envConfigAcl.ACL_CONTRACT_ADDRESS) {
    throw new Error(`The nonce of the deployer account is not corret. Please relaunch a clean instance of the fhEVM`);
  }
  console.log('ACL was deployed at address:', address);
});

task('task:deployTFHEExecutor').setAction(async function (taskArguments: TaskArguments, { ethers }) {
  const deployer = (await ethers.getSigners())[9];
  const factory = await ethers.getContractFactory('TFHEExecutor');
  const exec = await factory.connect(deployer).deploy();
  await exec.waitForDeployment();
  const address = await exec.getAddress();
  const envConfig = dotenv.parse(fs.readFileSync('lib/.env.exec'));
  if (address !== envConfig.TFHE_EXECUTOR_CONTRACT_ADDRESS) {
    throw new Error(`The nonce of the deployer account is not corret. Please relaunch a clean instance of the fhEVM`);
  }
  console.log('TFHEExecutor was deployed at address:', address);
});

task('task:deployKMSVerifier').setAction(async function (taskArguments: TaskArguments, { ethers }) {
  const deployer = (await ethers.getSigners())[9];
  const factory = await ethers.getContractFactory('KMSVerifier');
  const exec = await factory.connect(deployer).deploy();
  await exec.waitForDeployment();
  const address = await exec.getAddress();
  const envConfig = dotenv.parse(fs.readFileSync('lib/.env.kmsverifier'));
  if (address !== envConfig.KMS_VERIFIER_CONTRACT_ADDRESS) {
    throw new Error(`The nonce of the deployer account is not corret. Please relaunch a clean instance of the fhEVM`);
  }
  console.log('KMSVerifier was deployed at address:', address);
});

task('task:deployFHEPayment').setAction(async function (taskArguments: TaskArguments, { ethers }) {
  const deployer = (await ethers.getSigners())[9];
  const factory = await ethers.getContractFactory('FHEPayment');
  const exec = await factory.connect(deployer).deploy();
  await exec.waitForDeployment();
  const address = await exec.getAddress();
  const envConfig = dotenv.parse(fs.readFileSync('lib/.env.fhepayment'));
  if (address !== envConfig.FHE_PAYMENT_CONTRACT_ADDRESS) {
    throw new Error(`The nonce of the deployer account is not corret. Please relaunch a clean instance of the fhEVM`);
  }
  console.log('FHEPayment was deployed at address:', address);
});
