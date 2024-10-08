import dotenv from 'dotenv';
import fs from 'fs';
import { task } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';

function stripContractName(input: string): string {
  const colonIndex = input.lastIndexOf('/');
  if (colonIndex !== -1) {
    return input.substring(0, colonIndex);
  }
  return input;
}

task('task:upgradeACL')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: lib/ACL.sol:ACL',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/ACLUpgradedExample.sol:ACLUpgradedExample',
  )
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync('lib/.env.acl'));
    const proxyAddress = parsedEnv.ACL_CONTRACT_ADDRESS;
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    await run('compile:specific', { contract: stripContractName(taskArguments.currentImplementation) });
    await run('compile:specific', { contract: stripContractName(taskArguments.newImplementation) });
    const currentImplementation = await ethers.getContractFactory(taskArguments.currentImplementation, deployer);
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const newImplementationFactory = await ethers.getContractFactory(taskArguments.newImplementation, deployer);
    await upgrades.upgradeProxy(proxy, newImplementationFactory);
    console.log('Waiting 2 minutes before contract verification... Please wait...');
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    const implementationACLAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationACLAddress,
      constructorArguments: [],
    });
  });

task('task:upgradeTFHEExecutor')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: lib/TFHEExecutor.sol:TFHEExecutor',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/TFHEExecutorUpgradedExample.sol:TFHEExecutorUpgradedExample',
  )
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync('lib/.env.exec'));
    const proxyAddress = parsedEnv.TFHE_EXECUTOR_CONTRACT_ADDRESS;
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    await run('compile:specific', { contract: stripContractName(taskArguments.currentImplementation) });
    await run('compile:specific', { contract: stripContractName(taskArguments.newImplementation) });
    const currentImplementation = await ethers.getContractFactory(taskArguments.currentImplementation, deployer);
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const newImplementationFactory = await ethers.getContractFactory(taskArguments.newImplementation, deployer);
    await upgrades.upgradeProxy(proxy, newImplementationFactory);
    console.log('Waiting 2 minutes before contract verification... Please wait...');
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    const implementationACLAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationACLAddress,
      constructorArguments: [],
    });
  });

task('task:upgradeKMSVerifier')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: lib/KMSVerifier.sol:KMSVerifier',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/KMSVerifierUpgradedExample.sol:KMSVerifierUpgradedExample',
  )
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync('lib/.env.kmsverifier'));
    const proxyAddress = parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS;
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    await run('compile:specific', { contract: stripContractName(taskArguments.currentImplementation) });
    await run('compile:specific', { contract: stripContractName(taskArguments.newImplementation) });
    const currentImplementation = await ethers.getContractFactory(taskArguments.currentImplementation, deployer);
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const newImplementationFactory = await ethers.getContractFactory(taskArguments.newImplementation, deployer);
    await upgrades.upgradeProxy(proxy, newImplementationFactory);
    console.log('Waiting 2 minutes before contract verification... Please wait...');
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    const implementationACLAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationACLAddress,
      constructorArguments: [],
    });
  });

task('task:upgradeInputVerifier')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: lib/InputVerifier.coprocessor.sol:InputVerifier',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/InputVerifierUpgradedExample.coprocessor.sol:InputVerifierUpgradedExample',
  )
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync('lib/.env.inputverifier'));
    const proxyAddress = parsedEnv.INPUT_VERIFIER_CONTRACT_ADDRESS;
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    await run('compile:specific', { contract: stripContractName(taskArguments.currentImplementation) });
    await run('compile:specific', { contract: stripContractName(taskArguments.newImplementation) });
    const currentImplementation = await ethers.getContractFactory(taskArguments.currentImplementation, deployer);
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const newImplementationFactory = await ethers.getContractFactory(taskArguments.newImplementation, deployer);
    await upgrades.upgradeProxy(proxy, newImplementationFactory);
    console.log('Waiting 2 minutes before contract verification... Please wait...');
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    const implementationACLAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationACLAddress,
      constructorArguments: [],
    });
  });

task('task:upgradeFHEPayment')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: lib/FHEPayment.sol:FHEPayment',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/FHEPaymentUpgradedExample.sol:FHEPaymentUpgradedExample',
  )
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync('lib/.env.fhepayment'));
    const proxyAddress = parsedEnv.FHE_PAYMENT_CONTRACT_ADDRESS;
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    await run('compile:specific', { contract: stripContractName(taskArguments.currentImplementation) });
    await run('compile:specific', { contract: stripContractName(taskArguments.newImplementation) });
    const currentImplementation = await ethers.getContractFactory(taskArguments.currentImplementation, deployer);
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const newImplementationFactory = await ethers.getContractFactory(taskArguments.newImplementation, deployer);
    await upgrades.upgradeProxy(proxy, newImplementationFactory);
    console.log('Waiting 2 minutes before contract verification... Please wait...');
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    const implementationACLAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationACLAddress,
      constructorArguments: [],
    });
  });

task('task:upgradeGatewayContract')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: gateway/GatewayContract.sol:GatewayContract',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: example/GatewayContractUpgradedExample.sol:GatewayContractUpgradedExample',
  )
  .addParam('privateKey', 'The deployer private key')
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync('gateway/.env.gateway'));
    const proxyAddress = parsedEnv.GATEWAY_CONTRACT_PREDEPLOY_ADDRESS;
    const deployer = new ethers.Wallet(taskArguments.privateKey).connect(ethers.provider);
    await run('compile:specific', { contract: stripContractName(taskArguments.currentImplementation) });
    await run('compile:specific', { contract: stripContractName(taskArguments.newImplementation) });
    const currentImplementation = await ethers.getContractFactory(taskArguments.currentImplementation, deployer);
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const newImplementationFactory = await ethers.getContractFactory(taskArguments.newImplementation, deployer);
    await upgrades.upgradeProxy(proxy, newImplementationFactory);
    console.log('Waiting 2 minutes before contract verification... Please wait...');
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    const implementationACLAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationACLAddress,
      constructorArguments: [],
    });
  });
