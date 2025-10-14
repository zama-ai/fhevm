import { HardhatEthersHelpers } from "@nomicfoundation/hardhat-ethers/types";
import { HardhatUpgrades } from "@openzeppelin/hardhat-upgrades";
import { Wallet } from "ethers";
import fs from "fs";
import { task } from "hardhat/config";
import path from "path";

import { ADDRESSES_DIR } from "../../hardhat.config";
import { getRequiredEnvVar } from "../utils/loadVariables";
import { GATEWAY_CONFIG_EMPTY_PROXY_NAME, REGULAR_EMPTY_PROXY_NAME } from "./utils";

// Deploy a new EmptyUUPSProxyGatewayConfig contract
async function deployEmptyUUPSGatewayConfig(ethers: HardhatEthersHelpers, upgrades: HardhatUpgrades, deployer: Wallet) {
  const factory = await ethers.getContractFactory(GATEWAY_CONFIG_EMPTY_PROXY_NAME, deployer);

  // The empty proxy for the GatewayConfig contract is owned by the deployed at first
  const UUPSEmpty = await upgrades.deployProxy(factory, [deployer.address], {
    initializer: "initialize",
    kind: "uups",
  });
  await UUPSEmpty.waitForDeployment();
  const UUPSEmptyAddress = await UUPSEmpty.getAddress();
  console.log(`${GATEWAY_CONFIG_EMPTY_PROXY_NAME} proxy contract successfully deployed!`);
  return UUPSEmptyAddress;
}

// Deploy a new EmptyUUPSProxy contract
async function deployEmptyUUPS(ethers: HardhatEthersHelpers, upgrades: HardhatUpgrades, deployer: Wallet) {
  const factory = await ethers.getContractFactory(REGULAR_EMPTY_PROXY_NAME, deployer);

  // The regular empty proxies are directly owned by the GatewayConfig's owner
  const UUPSEmpty = await upgrades.deployProxy(factory, [], {
    initializer: "initialize",
    kind: "uups",
  });
  await UUPSEmpty.waitForDeployment();
  const UUPSEmptyAddress = await UUPSEmpty.getAddress();
  console.log(`${REGULAR_EMPTY_PROXY_NAME} proxy contract successfully deployed!`);
  return UUPSEmptyAddress;
}

// Deploy all the EmptyUUPS proxy contracts
task("task:deployEmptyUUPSProxies").setAction(async function (_, { ethers, upgrades, run }) {
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

  // Compile the EmptyUUPSGatewayConfig proxy contract
  await run("compile:specific", { contract: "contracts/emptyProxyGatewayConfig" });

  // The GatewayConfig contract must be deployed first as the following contracts' empty proxies need
  // its address in order to make them owned by the GatewayConfig contract's owner.
  console.log("Deploying an EmptyUUPS proxy contract for GatewayConfig...");
  const gatewayConfigAddress = await deployEmptyUUPSGatewayConfig(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "GatewayConfig",
    address: gatewayConfigAddress,
  });

  // Compile the EmptyUUPS proxy contract
  // The regular EmptyUUPS proxy contracts should only be compiled after the GatewayConfig address is
  // set, as they are made owned by the GatewayConfig contract's owner.
  await run("compile:specific", { contract: "contracts/emptyProxy" });

  console.log("Deploying an EmptyUUPS proxy contract for MultichainACL...");
  const multichainACLAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "MultichainACL",
    address: multichainACLAddress,
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

  console.log("Deploying an EmptyUUPS proxy contract for KMSGeneration...");
  const kmsGenerationAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "KMSGeneration",
    address: kmsGenerationAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for InputVerification...");
  const inputVerificationAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "InputVerification",
    address: inputVerificationAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for CoprocessorContexts...");
  const coprocessorContextsAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "CoprocessorContexts",
    address: coprocessorContextsAddress,
  });
});
