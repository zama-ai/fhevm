import { HardhatEthersHelpers } from "@nomicfoundation/hardhat-ethers/types";
import { HardhatUpgrades } from "@openzeppelin/hardhat-upgrades";
import { Wallet } from "ethers";
import fs from "fs";
import { task } from "hardhat/config";
import type { TaskArguments } from "hardhat/types";
import path from "path";

import { ADDRESSES_DIR } from "../../hardhat.config";
import { getRequiredEnvVar } from "../utils/loadVariables";
import { pascalCaseToCamelCase, pascalCaseToSnakeCase } from "../utils/stringOps";

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
  .setAction(async function ({ name, address }: TaskArguments) {
    const nameSnakeCase = pascalCaseToSnakeCase(name);
    const envFilePath = path.join(ADDRESSES_DIR, ".env.gateway");
    const solidityFilePath = path.join(ADDRESSES_DIR, "GatewayAddresses.sol");
    const envContent = `${nameSnakeCase.toUpperCase()}_ADDRESS=${address}\n`;
    const solidityTemplate = `address constant ${pascalCaseToCamelCase(name)}Address = ${address};\n`;

    try {
      // Append the contract's address in the addresses/.env.gateway file
      fs.appendFileSync(envFilePath, envContent, { encoding: "utf8", flag: "a" });

      // Append the contract's address in the addresses/GatewayAddresses.sol file
      fs.appendFileSync(solidityFilePath, solidityTemplate, {
        encoding: "utf8",
        flag: "a",
      });
      console.log(`${name} address ${address} written successfully!`);
    } catch (err) {
      console.error(`Failed to write ${name} address:`, err);
    }
  });

// Deploy all the EmptyUUPS proxy contracts
task("task:deployEmptyUUPSProxies").setAction(async function (_, { ethers, upgrades, run }) {
  // Compile the EmptyUUPS proxy contract
  await run("compile:specific", { contract: "contracts/emptyProxy" });

  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Ensure the ADDRESSES_DIR exists or create it
  fs.mkdirSync(ADDRESSES_DIR, { recursive: true });

  // Empty the .env.gateway file for the subsequent tasks to append the contract addresses.
  const envFilePath = path.join(ADDRESSES_DIR, ".env.gateway");
  fs.writeFileSync(envFilePath, "", { flag: "w" });

  // Truncate the GatewayAddresses.sol file with the Solidity header for the subsequent tasks
  // to append the contract addresses.
  const solidityFilePath = path.join(ADDRESSES_DIR, "GatewayAddresses.sol");
  const solidityHeader = `// SPDX-License-Identifier: BSD-3-Clause-Clear\npragma solidity ^0.8.24;\n\n`;
  fs.writeFileSync(solidityFilePath, solidityHeader, {
    encoding: "utf8",
    flag: "w",
  });

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
