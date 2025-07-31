import dotenv from "dotenv";
import { Wallet } from "ethers";
import fs from "fs";
import { task, types } from "hardhat/config";
import { HardhatEthersHelpers } from "hardhat/types";
import path from "path";

import { ADDRESSES_DIR } from "../hardhat.config";
import { getRequiredEnvVar } from "./utils/loadVariables";
import { pascalCaseToSnakeCase } from "./utils/stringOps";

// Helper function to get a Gateway contract and its proxy address
async function getGatewayContract(name: string, ethers: HardhatEthersHelpers, useInternalAddress: boolean) {
  // Get the pauser wallet
  const pauserPrivateKey = getRequiredEnvVar("PAUSER_PRIVATE_KEY");
  const pauser = new Wallet(pauserPrivateKey).connect(ethers.provider);

  // Get contract factories
  if (useInternalAddress) {
    const envFilePath = path.join(ADDRESSES_DIR, `.env.gateway`);

    if (!fs.existsSync(envFilePath)) {
      throw new Error(`Environment file not found: ${envFilePath}`);
    }

    dotenv.config({ path: envFilePath, override: true });
  }

  // Determine env variable name for the proxy contract address
  const nameSnakeCase = pascalCaseToSnakeCase(name);
  const addressEnvVarName = `${nameSnakeCase.toUpperCase()}_ADDRESS`;

  // Get the proxy address
  const proxyAddress = getRequiredEnvVar(addressEnvVarName);

  const contract = await ethers.getContractAt(name, proxyAddress, pauser);

  return { contract, proxyAddress };
}

// Helper function to pause a contract
async function pauseSingleContract(name: string, ethers: HardhatEthersHelpers, useInternalAddress: boolean) {
  // Get the contract and its address
  const { contract, proxyAddress } = await getGatewayContract(name, ethers, useInternalAddress);

  // Pause the contract
  await contract.pause();

  console.log(`${name} contract successfully paused at address: ${proxyAddress}\n`);
}

// Helper function to unpause a contract
async function unpauseSingleContract(name: string, ethers: HardhatEthersHelpers, useInternalAddress: boolean) {
  // Get the contract and its address
  const { contract, proxyAddress } = await getGatewayContract(name, ethers, useInternalAddress);

  // Unpause the contract
  await contract.unpause();

  console.log(`${name} contract successfully unpaused at address: ${proxyAddress}\n`);
}

// Pausing tasks
// Pause the InputVerification contract
task("task:pauseInputVerification")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { ethers }) {
    await pauseSingleContract("InputVerification", ethers, useInternalProxyAddress);
  });

// Pause the Decryption contract
task("task:pauseDecryption")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { ethers }) {
    await pauseSingleContract("Decryption", ethers, useInternalProxyAddress);
  });

// Unpausing tasks
// Unpause the InputVerification contract
task("task:unpauseInputVerification")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { ethers }) {
    await unpauseSingleContract("InputVerification", ethers, useInternalProxyAddress);
  });

// Unpause the Decryption contract
task("task:unpauseDecryption")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { ethers }) {
    await unpauseSingleContract("Decryption", ethers, useInternalProxyAddress);
  });

// Pause all the contracts
// The following contracts are pausable but don't have pausable functions yet, so they are
// not paused by the `pauseAllGatewayContracts()` function for now:
// - CiphertextCommits
// - MultichainAcl
// - GatewayConfig
// In addition, the `KmsManagement` contract is not used yet, so we don't need to pause it for now.
// See https://github.com/zama-ai/fhevm-internal/issues/180
task("task:pauseAllGatewayContracts")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    console.log("Pause all Gateway contracts:");

    const name = "GatewayConfig";

    // Get the GatewayConfig contract and its address
    const { contract, proxyAddress } = await getGatewayContract(name, hre.ethers, useInternalProxyAddress);

    // Pause all the Gateway contracts
    await contract.pauseAllGatewayContracts();

    console.log(`All Gateway contracts successfully paused through contract ${name} at address: ${proxyAddress}`);
  });

// Unpause all the contracts
// See comment above for the list of contracts that are not pausable yet.
task("task:unpauseAllGatewayContracts")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    console.log("Unpause all Gateway contracts:");
    const name = "GatewayConfig";

    // Get the GatewayConfig contract and its address
    const { contract, proxyAddress } = await getGatewayContract(name, hre.ethers, useInternalProxyAddress);

    // Unpause all the Gateway contracts
    await contract.unpauseAllGatewayContracts();

    console.log(`All Gateway contracts successfully unpaused through contract ${name} at address: ${proxyAddress}`);
  });
