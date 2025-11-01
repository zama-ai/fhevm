import { Wallet } from "ethers";
import { task, types } from "hardhat/config";
import { HardhatEthersHelpers } from "hardhat/types";

import { getRequiredEnvVar, loadGatewayAddresses } from "./utils";
import { pascalCaseToSnakeCase } from "./utils/stringOps";

// Helper function to get a Gateway contract and its proxy address
async function getGatewayContract(
  name: string,
  ethers: HardhatEthersHelpers,
  useInternalAddress: boolean,
  envVarPrivateKeyName: string,
) {
  // Get the account (pauser for pausing OR deployer for unpausing) wallet
  const accountPrivateKey = getRequiredEnvVar(envVarPrivateKeyName);
  const account = new Wallet(accountPrivateKey).connect(ethers.provider);

  // Get contract factories
  if (useInternalAddress) {
    loadGatewayAddresses();
  }

  // Determine env variable name for the proxy contract address
  const nameSnakeCase = pascalCaseToSnakeCase(name);
  const addressEnvVarName = `${nameSnakeCase.toUpperCase()}_ADDRESS`;

  // Get the proxy address
  const proxyAddress = getRequiredEnvVar(addressEnvVarName);

  const contract = await ethers.getContractAt(name, proxyAddress, account);

  return { contract, proxyAddress };
}

// Helper function to pause a contract
async function pauseSingleContract(name: string, ethers: HardhatEthersHelpers, useInternalAddress: boolean) {
  // Get the contract and its address
  const { contract, proxyAddress } = await getGatewayContract(name, ethers, useInternalAddress, "PAUSER_PRIVATE_KEY");

  // Pause the contract
  const pauseTx = await contract.pause();
  const pauseReceipt = await pauseTx.wait();
  if (!pauseReceipt || pauseReceipt.status !== 1) {
    throw new Error(`Pausing ${name} contract failed (tx hash: ${pauseTx.hash})`);
  }

  console.log(`${name} contract successfully paused at address: ${proxyAddress}\n`);
}

// Helper function to unpause a contract
async function unpauseSingleContract(name: string, ethers: HardhatEthersHelpers, useInternalAddress: boolean) {
  // Get the contract and its address
  // NOTE: this task won't work once ownership will be transferred from initial deployer to the multisig
  const { contract, proxyAddress } = await getGatewayContract(name, ethers, useInternalAddress, "DEPLOYER_PRIVATE_KEY");

  // Unpause the contract
  const unpauseTx = await contract.unpause();
  const unpauseReceipt = await unpauseTx.wait();
  if (!unpauseReceipt || unpauseReceipt.status !== 1) {
    throw new Error(`Unpausing ${name} contract failed (tx hash: ${unpauseTx.hash})`);
  }

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
// - MultichainACL
// - GatewayConfig
// In addition, the `KMSGeneration` contract is not used yet, so we don't need to pause it for now.
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
    const { contract, proxyAddress } = await getGatewayContract(
      name,
      hre.ethers,
      useInternalProxyAddress,
      "PAUSER_PRIVATE_KEY",
    );

    // Pause all the Gateway contracts
    const pauseTx = await contract.pauseAllGatewayContracts();
    const pauseReceipt = await pauseTx.wait();
    if (!pauseReceipt || pauseReceipt.status !== 1) {
      throw new Error(`Pausing all Gateway contracts failed (tx hash: ${pauseTx.hash})`);
    }

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
    // NOTE: this task won't work once ownership will be transferred from initial deployer to the multisig
    const { contract, proxyAddress } = await getGatewayContract(
      name,
      hre.ethers,
      useInternalProxyAddress,
      "DEPLOYER_PRIVATE_KEY",
    );

    // Unpause all the Gateway contracts
    const unpauseTx = await contract.unpauseAllGatewayContracts();
    const unpauseReceipt = await unpauseTx.wait();
    if (!unpauseReceipt || unpauseReceipt.status !== 1) {
      throw new Error(`Unpausing all Gateway contracts failed (tx hash: ${unpauseTx.hash})`);
    }

    console.log(`All Gateway contracts successfully unpaused through contract ${name} at address: ${proxyAddress}`);
  });
