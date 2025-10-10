import dotenv from "dotenv";
import { Wallet } from "ethers";
import fs from "fs";
import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import path from "path";

import { ADDRESSES_DIR } from "../../hardhat.config";
import { CoprocessorV2Struct } from "../../typechain-types/contracts/CoprocessorContexts";
import { getRequiredEnvVar } from "../utils/loadVariables";
import { pascalCaseToSnakeCase } from "../utils/stringOps";
import { GATEWAY_CONFIG_EMPTY_PROXY_NAME, REGULAR_EMPTY_PROXY_NAME } from "./utils";

// Helper function to deploy a contract implementation to its proxy
async function deployContractImplementation(
  name: string,
  hre: HardhatRuntimeEnvironment,
  emptyProxyName: string,
  initializeArgs?: unknown[],
): Promise<string> {
  const { ethers, upgrades } = hre;

  // Get a deployer wallet
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Get contract factories
  const proxyImplementation = await ethers.getContractFactory(emptyProxyName, deployer);
  const newImplem = await ethers.getContractFactory(name, deployer);

  const envFilePath = path.join(ADDRESSES_DIR, `.env.gateway`);
  if (!fs.existsSync(envFilePath)) {
    throw new Error(`Environment file not found: ${envFilePath}`);
  }
  dotenv.config({ path: envFilePath, override: true });

  // Determine env variable name for the proxy contract address
  const nameSnakeCase = pascalCaseToSnakeCase(name);
  const addressEnvVarName = `${nameSnakeCase.toUpperCase()}_ADDRESS`;

  // Get the proxy address
  const proxyAddress = getRequiredEnvVar(addressEnvVarName);
  if (!proxyAddress) {
    throw new Error(`Address variable ${addressEnvVarName} not found in ${envFilePath}`);
  }

  // Force import
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);

  // Set the upgrade options
  const upgradeOptions = {
    call: {
      fn: "initializeFromEmptyProxy",
      args: [] as unknown[],
    },
  };
  if (initializeArgs !== undefined && initializeArgs.length > 0) {
    upgradeOptions.call.args = initializeArgs;
  }

  // Upgrade the proxy
  await upgrades.upgradeProxy(proxy, newImplem, upgradeOptions);

  console.log(`${name} implementation set successfully at address: ${proxyAddress}\n`);
  return proxyAddress;
}

// Deploy the GatewayConfig contract
task("task:deployGatewayConfig").setAction(async function (_, hre) {
  // Parse the protocol metadata
  const protocolMetadata = {
    name: getRequiredEnvVar("PROTOCOL_NAME"),
    website: getRequiredEnvVar("PROTOCOL_WEBSITE"),
  };

  // Parse the MPC threshold
  const mpcThreshold = getRequiredEnvVar("MPC_THRESHOLD");

  // Parse the decryption response thresholds
  const publicDecryptionThreshold = getRequiredEnvVar("PUBLIC_DECRYPTION_THRESHOLD");
  const userDecryptionThreshold = getRequiredEnvVar("USER_DECRYPTION_THRESHOLD");

  // Parse the KMS public material generation threshold
  const kmsGenThreshold = getRequiredEnvVar("KMS_GENERATION_THRESHOLD");

  // Parse the KMS nodes
  const numKmsNodes = parseInt(getRequiredEnvVar("NUM_KMS_NODES"));
  const kmsNodes = [];
  for (let idx = 0; idx < numKmsNodes; idx++) {
    kmsNodes.push({
      txSenderAddress: getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`),
      signerAddress: getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`),
      ipAddress: getRequiredEnvVar(`KMS_NODE_IP_ADDRESS_${idx}`),
      storageUrl: getRequiredEnvVar(`KMS_NODE_STORAGE_URL_${idx}`),
    });
  }

  // Parse the custodians
  const numCustodians = parseInt(getRequiredEnvVar("NUM_CUSTODIANS"));
  const custodians = [];
  for (let idx = 0; idx < numCustodians; idx++) {
    custodians.push({
      txSenderAddress: getRequiredEnvVar(`CUSTODIAN_TX_SENDER_ADDRESS_${idx}`),
      signerAddress: getRequiredEnvVar(`CUSTODIAN_SIGNER_ADDRESS_${idx}`),
      encryptionKey: getRequiredEnvVar(`CUSTODIAN_ENCRYPTION_KEY_${idx}`),
    });
  }
  console.log("Protocol metadata:", protocolMetadata);
  console.log("MPC threshold:", mpcThreshold);
  console.log("Public decryption threshold:", publicDecryptionThreshold);
  console.log("User decryption threshold:", userDecryptionThreshold);
  console.log("KMS nodes:", kmsNodes);
  console.log("Custodians:", custodians);

  // The GatewayConfig contract is not deployed using the same empty proxy as the other contracts,
  // as it is made ownable
  await deployContractImplementation("GatewayConfig", hre, GATEWAY_CONFIG_EMPTY_PROXY_NAME, [
    protocolMetadata,
    mpcThreshold,
    publicDecryptionThreshold,
    userDecryptionThreshold,
    kmsGenThreshold,
    kmsNodes,
    custodians,
  ]);
});

// Deploy the CoprocessorContexts contract
task("task:deployCoprocessorContexts").setAction(async function (_, hre) {
  // Parse the coprocessor feature set
  const coprocessorsFeatureSet = getRequiredEnvVar("COPROCESSORS_FEATURE_SET");

  // Parse the coprocessors
  const numCoprocessors = parseInt(getRequiredEnvVar("NUM_COPROCESSORS"));
  const coprocessors: CoprocessorV2Struct[] = [];
  for (let idx = 0; idx < numCoprocessors; idx++) {
    coprocessors.push({
      name: getRequiredEnvVar(`COPROCESSOR_NAME_${idx}`),
      txSenderAddress: getRequiredEnvVar(`COPROCESSOR_TX_SENDER_ADDRESS_${idx}`),
      signerAddress: getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`),
      storageUrl: getRequiredEnvVar(`COPROCESSOR_STORAGE_URL_${idx}`),
    });
  }
  console.log("Coprocessors:", coprocessors);

  await deployContractImplementation("CoprocessorContexts", hre, REGULAR_EMPTY_PROXY_NAME, [
    coprocessorsFeatureSet,
    coprocessors,
  ]);
});

// Deploy the InputVerification contract
task("task:deployInputVerification").setAction(async function (_, hre) {
  await deployContractImplementation("InputVerification", hre, REGULAR_EMPTY_PROXY_NAME);
});

// Deploy the KMSGeneration contract
task("task:deployKMSGeneration").setAction(async function (_, hre) {
  await deployContractImplementation("KMSGeneration", hre, REGULAR_EMPTY_PROXY_NAME);
});

// Deploy the CiphertextCommits contract
task("task:deployCiphertextCommits").setAction(async function (_, hre) {
  await deployContractImplementation("CiphertextCommits", hre, REGULAR_EMPTY_PROXY_NAME);
});

// Deploy the MultichainACL contract
task("task:deployMultichainACL").setAction(async function (_, hre) {
  await deployContractImplementation("MultichainACL", hre, REGULAR_EMPTY_PROXY_NAME);
});

// Deploy the Decryption contract
task("task:deployDecryption").setAction(async function (_, hre) {
  await deployContractImplementation("Decryption", hre, REGULAR_EMPTY_PROXY_NAME);
});

// Deploy all the contracts
task("task:deployAllGatewayContracts").setAction(async function (_, hre) {
  // Deploy the EmptyUUPS proxy contracts
  await hre.run("task:deployEmptyUUPSProxies");

  await hre.run("compile:specific", { contract: "contracts/immutable" });
  await hre.run("task:deployPauserSet");

  // Compile the implementation contracts
  // The deployEmptyUUPSProxies task has generated the contracts' addresses in `addresses/*.sol`.
  // Contracts thus need to be compiled after deploying the EmptyUUPS proxy contracts in order to
  // use these addresses. Otherwise, irrelevant addresses will be used and, although deployment will
  // succeed, most transactions made to the contracts will revert as inter-contract calls will fail.
  await hre.run("compile:specific", { contract: "contracts" });

  console.log("Deploy GatewayConfig contract:");
  await hre.run("task:deployGatewayConfig");

  console.log("Deploy CoprocessorContexts contract:");
  await hre.run("task:deployCoprocessorContexts");

  console.log("Deploy InputVerification contract:");
  await hre.run("task:deployInputVerification");

  console.log("Deploy KMSGeneration contract:");
  await hre.run("task:deployKMSGeneration");

  console.log("Deploy CiphertextCommits contract:");
  await hre.run("task:deployCiphertextCommits");

  console.log("Deploy MultichainACL contract:");
  await hre.run("task:deployMultichainACL");

  console.log("Deploy Decryption contract:");
  await hre.run("task:deployDecryption");

  console.log("Contract deployment done!");
});
