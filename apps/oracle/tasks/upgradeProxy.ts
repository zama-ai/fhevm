import { HardhatUpgrades } from '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { RunTaskFunction, TaskArguments } from 'hardhat/types';

function stripContractName(input: string): string {
  const colonIndex = input.lastIndexOf('/');
  if (colonIndex !== -1) {
    return input.substring(0, colonIndex);
  }
  return input;
}

async function upgradeCurrentToNew(
  privateKey: string,
  proxyAddress: string,
  currentImplem: string,
  newImplem: string,
  verifyContract: boolean,
  upgrades: HardhatUpgrades,
  run: RunTaskFunction,
  ethers: any,
) {
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  await run('compile:specific', { contract: stripContractName(currentImplem) });
  await run('compile:specific', { contract: stripContractName(newImplem) });
  const currentImplementation = await ethers.getContractFactory(currentImplem, deployer);
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  const newImplementationFactory = await ethers.getContractFactory(newImplem, deployer);
  await upgrades.upgradeProxy(proxy, newImplementationFactory);
  if (verifyContract) {
    console.log('Waiting 2 minutes before contract verification... Please wait...');
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationAddress,
      constructorArguments: [],
    });
  }
}

task('task:upgradeDecryptionOracleContract')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: decryptionOracle/DecryptionOracle.sol:DecryptionOracle',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: example/DecryptionOracleUpgradedExample.sol:DecryptionOracleUpgradedExample',
  )
  .addParam('privateKey', 'The deployer private key')
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.decryptionoracle'));
    const proxyAddress = parsedEnv.DECRYPTION_ORACLE_ADDRESS;
    await upgradeCurrentToNew(
      taskArguments.privateKey,
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
  });
