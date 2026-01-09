import { Wallet } from "ethers";
import { task, types } from "hardhat/config";
import { HardhatEthersHelpers } from "hardhat/types";

import { getRequiredAddressEnvVar, getRequiredEnvVar, loadGatewayAddresses } from "./utils";

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

  // Get the proxy address
  const proxyAddress = getRequiredAddressEnvVar(name);

  const contract = await ethers.getContractAt(name, proxyAddress, account);

  return { contract, proxyAddress };
}

// Helper function to pause a contract
async function pauseSingleContract(name: string, ethers: HardhatEthersHelpers, useInternalAddress: boolean) {
  // Get the contract and its address
  const { contract, proxyAddress } = await getGatewayContract(name, ethers, useInternalAddress, "PAUSER_PRIVATE_KEY");

  // Pause the contract
  await contract.pause();

  console.log(`${name} contract successfully paused at address: ${proxyAddress}\n`);
}

// Helper function to unpause a contract
async function unpauseSingleContract(name: string, ethers: HardhatEthersHelpers, useInternalAddress: boolean) {
  // Get the contract and its address
  // NOTE: this task won't work once ownership will be transferred from initial deployer to the multisig
  const { contract, proxyAddress } = await getGatewayContract(name, ethers, useInternalAddress, "DEPLOYER_PRIVATE_KEY");

  // Unpause the contract
  await contract.unpause();

  console.log(`${name} contract successfully unpaused at address: ${proxyAddress}\n`);
}

// Pause all Gateway contracts via GatewayConfig
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
    // NOTE: this task won't work once ownership will be transferred from initial deployer to the multisig
    const { contract, proxyAddress } = await getGatewayContract(
      name,
      hre.ethers,
      useInternalProxyAddress,
      "DEPLOYER_PRIVATE_KEY",
    );

    // Unpause all the Gateway contracts
    await contract.unpauseAllGatewayContracts();

    console.log(`All Gateway contracts successfully unpaused through contract ${name} at address: ${proxyAddress}`);
  });
