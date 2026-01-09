import { HardhatEthersHelpers } from "@nomicfoundation/hardhat-ethers/types";
import { HardhatUpgrades } from "@openzeppelin/hardhat-upgrades";
import { Wallet } from "ethers";
import { task, types } from "hardhat/config";

import { GATEWAY_ADDRESSES_ENV_FILE_NAME, GATEWAY_ADDRESSES_SOLIDITY_FILE_NAME } from "../../hardhat.config";
import { getRequiredEnvVar, loadGatewayAddresses } from "../utils";
import {
  GATEWAY_CONFIG_EMPTY_PROXY_NAME,
  REGULAR_EMPTY_PROXY_NAME,
  createEnvAddressesFile,
  createSolidityAddressesFile,
  setGatewayContractAddress,
} from "./utils";

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

  // Empty the .env.gateway file for the subsequent tasks to append the contract addresses.
  createEnvAddressesFile(GATEWAY_ADDRESSES_ENV_FILE_NAME);

  // Initialize the solidity file for gateway addresses
  createSolidityAddressesFile(GATEWAY_ADDRESSES_SOLIDITY_FILE_NAME);

  // Compile the EmptyUUPSGatewayConfig proxy contract
  await run("compile:specific", { contract: "contracts/emptyProxyGatewayConfig" });

  // The GatewayConfig contract must be deployed first as the following contracts' empty proxies need
  // its address in order to make them owned by the GatewayConfig contract's owner.
  console.log(`Deploying an ${GATEWAY_CONFIG_EMPTY_PROXY_NAME} proxy contract for GatewayConfig...`);
  const gatewayConfigAddress = await deployEmptyUUPSGatewayConfig(ethers, upgrades, deployer);
  setGatewayContractAddress("GatewayConfig", gatewayConfigAddress);

  // Compile the EmptyUUPS proxy contract
  // The regular EmptyUUPS proxy contracts should only be compiled after the GatewayConfig address is
  // set, as they are made owned by the GatewayConfig contract's owner.
  await run("compile:specific", { contract: "contracts/emptyProxy" });

  console.log("Deploying an EmptyUUPS proxy contract for KMSGeneration...");
  const kmsGenerationAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  setGatewayContractAddress("KMSGeneration", kmsGenerationAddress);

  console.log("Deploying an EmptyUUPS proxy contract for ProtocolPayment...");
  const protocolPaymentAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  setGatewayContractAddress("ProtocolPayment", protocolPaymentAddress);
});

// Deploy a single regular EmptyUUPS proxy contract for a given contract, after the GatewayConfig
// contract has been deployed
// The new contract address will be appended to the .env.gateway and GatewayAddresses.sol files
task("task:deploySingleEmptyUUPSProxy")
  .addParam("name", "The name of the contract")
  .addParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ name, useInternalProxyAddress }, { ethers, upgrades, run }) {
    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

    if (useInternalProxyAddress) {
      loadGatewayAddresses();
    }

    // Make sure the gateway config contract address is set
    getRequiredEnvVar("GATEWAY_CONFIG_ADDRESS");

    // Compile the EmptyUUPS proxy contract
    // The regular EmptyUUPS proxy contracts should only be compiled after the GatewayConfig address is
    // set, as they are made owned by the GatewayConfig contract's owner.
    await run("compile:specific", { contract: "contracts/emptyProxy" });

    console.log(`Deploying an EmptyUUPS proxy contract for ${name}...`);
    const contractAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    setGatewayContractAddress(name, contractAddress);
  });
