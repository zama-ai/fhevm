import { HardhatUpgrades } from "@openzeppelin/hardhat-upgrades";
import dotenv from "dotenv";
import { Wallet } from "ethers";
import fs from "fs";
import { task, types } from "hardhat/config";
import type { RunTaskFunction, TaskArguments } from "hardhat/types";

import { getRequiredEnvVar } from "./utils/loadVariables";

// This file defines generic tasks that can be used to upgrade the implementation of already deployed contracts.

function stripContractName(input: string): string {
  const colonIndex = input.lastIndexOf("/");
  if (colonIndex !== -1) {
    return input.substring(0, colonIndex);
  }
  return input;
}

// Upgrades the implementation of the proxy
async function upgradeCurrentToNew(
  proxyAddress: string,
  currentImplem: string,
  newImplem: string,
  verifyContract: boolean,
  upgrades: HardhatUpgrades,
  run: RunTaskFunction,
  ethers: any,
) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  await run("compile:specific", { contract: stripContractName(currentImplem) });
  await run("compile:specific", { contract: stripContractName(newImplem) });

  console.log(`Importing ${currentImplem} contract implementation at address ${proxyAddress}...`);
  const currentImplementation = await ethers.getContractFactory(currentImplem, deployer);
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  console.log("Proxy contract successfully loaded!");

  console.log(`Upgrading proxy to ${newImplem} contract implementation...`);
  const newImplementationFactory = await ethers.getContractFactory(newImplem, deployer);
  await upgrades.upgradeProxy(proxy, newImplementationFactory);
  console.log("Proxy contract successfully upgraded!");

  if (verifyContract) {
    console.log("Waiting 2 minutes before contract verification... Please wait...");
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run("verify:verify", {
      address: implementationAddress,
      constructorArguments: [],
    });
  }
}

task("task:upgradeACLManager")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/ACLManager.sol:ACLManager",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/examples/ACLManagerUpgradedExample.sol:ACLManagerUpgradedExample",
  )
  .addOptionalParam(
    "verifyContract",
    "Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)",
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.acl_manager"));
    const proxyAddress = parsedEnv.ACL_MANAGER_ADDRESS;
    await upgradeCurrentToNew(
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
  });

task("task:upgradeCiphertextManager")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/CiphertextManager.sol:CiphertextManager",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/examples/CiphertextManagerUpgradedExample.sol:CiphertextManagerUpgradedExample",
  )
  .addOptionalParam(
    "verifyContract",
    "Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)",
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.ciphertext_manager"));
    const proxyAddress = parsedEnv.CIPHERTEXT_MANAGER_ADDRESS;
    await upgradeCurrentToNew(
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
  });

task("task:upgradeDecryptionManager")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/DecryptionManager.sol:DecryptionManager",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/examples/DecryptionManagerUpgradedExample.sol:DecryptionManagerUpgradedExample",
  )
  .addOptionalParam(
    "verifyContract",
    "Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)",
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.decryption_manager"));
    const proxyAddress = parsedEnv.DECRYPTION_MANAGER_ADDRESS;
    await upgradeCurrentToNew(
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
  });

task("task:upgradeHTTPZ")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/HTTPZ.sol:HTTPZ",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/examples/HTTPZUpgradedExample.sol:HTTPZUpgradedExample",
  )
  .addOptionalParam(
    "verifyContract",
    "Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)",
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.httpz"));
    const proxyAddress = parsedEnv.HTTPZ_ADDRESS;
    await upgradeCurrentToNew(
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
  });

task("task:upgradeKeyManager")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/KeyManager.sol:KeyManager",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/examples/KeyManagerUpgradedExample.sol:KeyManagerUpgradedExample",
  )
  .addOptionalParam(
    "verifyContract",
    "Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)",
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.key_manager"));
    const proxyAddress = parsedEnv.KEY_MANAGER_ADDRESS;
    await upgradeCurrentToNew(
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
  });

task("task:upgradeZKPoKManager")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/ZKPoKManager.sol:ZKPoKManager",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/examples/ZKPoKManagerUpgradedExample.sol:ZKPoKManagerUpgradedExample",
  )
  .addOptionalParam(
    "verifyContract",
    "Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)",
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.zkpok_manager"));
    const proxyAddress = parsedEnv.ZKPOK_MANAGER_ADDRESS;
    await upgradeCurrentToNew(
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
  });
