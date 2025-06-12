import { HardhatEthersHelpers } from "@nomicfoundation/hardhat-ethers/types";
import { HardhatUpgrades } from "@openzeppelin/hardhat-upgrades";
import { Wallet } from "ethers";
import fs from "fs";
import { task } from "hardhat/config";
import type { TaskArguments } from "hardhat/types";
import path from "path";

import { getRequiredEnvVar } from "../utils/loadVariables";
import { pascalCaseToCamelCase, pascalCaseToSnakeCase } from "../utils/stringOps";

const ADDRESSES_DIR = path.join(__dirname, "../../addresses");

// Deploy a new EmptyUUPSProxy contract
async function deployEmptyUUPS(ethers: HardhatEthersHelpers, upgrades: HardhatUpgrades, deployer: Wallet) {
  const factory = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const UUPSEmpty = await upgrades.deployProxy(factory, [deployer.address], {
    initializer: "initialize",
    kind: "uups",
  });
  await UUPSEmpty.waitForDeployment();
  const UUPSEmptyAddress = await UUPSEmpty.getAddress();
  console.log("EmptyUUPS proxy contract successfully deployed!\n");
  return UUPSEmptyAddress;
}

// A helper task to update a contract's address in their .sol and .env file in the `addresses` directory
task("task:setContractAddress")
  .addParam("name", "The name of the contract (PascalCase)")
  .addParam("address", "The address of the contract")
  .setAction(async function (taskArguments: TaskArguments) {
    const name = taskArguments.name;
    const address = taskArguments.address;

    const nameSnakeCase = pascalCaseToSnakeCase(name);
    const envFilePath = path.join(ADDRESSES_DIR, `.env.${nameSnakeCase}`);
    const envContent = `${nameSnakeCase.toUpperCase()}_ADDRESS=${address}\n`;

    // Write the contract's address in its addresses/.env.xxx file
    try {
      // Ensure the ADDRESSES_DIR exists or create it
      fs.mkdirSync(ADDRESSES_DIR, { recursive: true });
      fs.writeFileSync(envFilePath, envContent, { flag: "w" });
      console.log(`${name} address ${address} written successfully!`);
    } catch (err) {
      console.error(`Failed to write ${name} address:`, err);
    }

    const solidityFilePath = path.join(ADDRESSES_DIR, `${name}Address.sol`);
    const solidityTemplate =
      `// SPDX-License-Identifier: BSD-3-Clause-Clear\n\n` +
      `pragma solidity ^0.8.24;\n\n` +
      `address constant ${pascalCaseToCamelCase(name)}Address = ${address};\n`;

    // Write the contract's address in its addresses/xxxAddress.sol file
    try {
      fs.writeFileSync(solidityFilePath, solidityTemplate, {
        encoding: "utf8",
        flag: "w",
      });
      console.log(`${solidityFilePath} file generated successfully!\n`);
    } catch (error) {
      console.error(`Failed to write ${solidityFilePath}\n`, error);
    }
  });

// Deploy all the EmptyUUPS proxy contracts
task("task:deployEmptyUUPSProxies").setAction(async function (_, { ethers, upgrades, run }) {
  // Compile the EmptyUUPS proxy contract
  await run("compile:specific", { contract: "contracts/emptyProxy" });

  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  console.log("Deploying an EmptyUUPS proxy contract for MultichainAcl...");
  const multichainAclAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "MultichainAcl",
    address: multichainAclAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for CiphertextCommits...");
  const ciphertextCommitsAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "CiphertextCommits",
    address: ciphertextCommitsAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for Decryption...");
  const decryptionAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "Decryption",
    address: decryptionAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for GatewayConfig...");
  const gatewayConfigAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "GatewayConfig",
    address: gatewayConfigAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for CoprocessorContexts...");
  const coprocessorContextsAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "CoprocessorContexts",
    address: coprocessorContextsAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for KmsManagement...");
  const kmsManagementAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "KmsManagement",
    address: kmsManagementAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for InputVerification...");
  const inputVerificationAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "InputVerification",
    address: inputVerificationAddress,
  });
});
