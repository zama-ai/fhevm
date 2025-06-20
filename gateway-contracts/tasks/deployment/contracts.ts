import dotenv from "dotenv";
import { Wallet } from "ethers";
import fs from "fs";
import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import path from "path";

import { getRequiredEnvVar } from "../utils/loadVariables";
import { pascalCaseToSnakeCase } from "../utils/stringOps";

const ADDRESSES_DIR = path.join(__dirname, "../../addresses");

// Helper function to deploy a contract implementation to its proxy
async function deployContractImplementation(
  name: string,
  hre: HardhatRuntimeEnvironment,
  initializeArgs?: unknown[],
): Promise<string> {
  const { ethers, upgrades } = hre;

  // Get a deployer wallet
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Get contract factories
  const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const newImplem = await ethers.getContractFactory(name, deployer);

  // Determine env file path and env variable name
  const nameSnakeCase = pascalCaseToSnakeCase(name);
  const envFilePath = path.join(ADDRESSES_DIR, `.env.${nameSnakeCase}`);
  const addressEnvVarName = `${nameSnakeCase.toUpperCase()}_ADDRESS`;

  // Get the proxy address
  if (!fs.existsSync(envFilePath)) {
    throw new Error(`Environment file not found: ${envFilePath}`);
  }
  const parsedEnv = dotenv.parse(fs.readFileSync(envFilePath));
  const proxyAddress = parsedEnv[addressEnvVarName];
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

  // Parse the pauser address
  const pauserAddress = getRequiredEnvVar(`PAUSER_ADDRESS`);

  // Parse the MPC threshold
  const mpcThreshold = getRequiredEnvVar("MPC_THRESHOLD");

  // Parse the decryption response thresholds
  const publicDecryptionThreshold = getRequiredEnvVar("PUBLIC_DECRYPTION_THRESHOLD");
  const userDecryptionThreshold = getRequiredEnvVar("USER_DECRYPTION_THRESHOLD");

  // Parse the KMS nodes
  const numKmsNodes = parseInt(getRequiredEnvVar("NUM_KMS_NODES"));
  const kmsNodes = [];
  for (let idx = 0; idx < numKmsNodes; idx++) {
    kmsNodes.push({
      txSenderAddress: getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`),
      signerAddress: getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`),
      ipAddress: getRequiredEnvVar(`KMS_NODE_IP_ADDRESS_${idx}`),
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

  console.log("Pauser address:", pauserAddress);
  console.log("Protocol metadata:", protocolMetadata);
  console.log("MPC threshold:", mpcThreshold);
  console.log("Public decryption threshold:", publicDecryptionThreshold);
  console.log("User decryption threshold:", userDecryptionThreshold);
  console.log("KMS nodes:", kmsNodes);
  console.log("Custodians:", custodians);

  await deployContractImplementation("GatewayConfig", hre, [
    pauserAddress,
    protocolMetadata,
    mpcThreshold,
    publicDecryptionThreshold,
    userDecryptionThreshold,
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
  const coprocessors = [];
  for (let idx = 0; idx < numCoprocessors; idx++) {
    coprocessors.push({
      name: getRequiredEnvVar(`COPROCESSOR_NAME_${idx}`),
      txSenderAddress: getRequiredEnvVar(`COPROCESSOR_TX_SENDER_ADDRESS_${idx}`),
      signerAddress: getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`),
      s3BucketUrl: getRequiredEnvVar(`COPROCESSOR_S3_BUCKET_URL_${idx}`),
    });
  }
  console.log("Coprocessors:", coprocessors);

  await deployContractImplementation("CoprocessorContexts", hre, [coprocessorsFeatureSet, coprocessors]);
});

// Deploy the InputVerification contract
task("task:deployInputVerification").setAction(async function (_, hre) {
  await deployContractImplementation("InputVerification", hre);
});

// Deploy the KmsManagement contract
task("task:deployKmsManagement").setAction(async function (_, hre) {
  const fheParamsName = getRequiredEnvVar("FHE_PARAMS_NAME");
  const fheParamsDigest = getRequiredEnvVar("FHE_PARAMS_DIGEST");

  console.log("FHE params name:", fheParamsName);
  console.log("FHE params digest:", fheParamsDigest);

  await deployContractImplementation("KmsManagement", hre, [fheParamsName, fheParamsDigest]);
});

// Deploy the CiphertextCommits contract
task("task:deployCiphertextCommits").setAction(async function (_, hre) {
  await deployContractImplementation("CiphertextCommits", hre);
});

// Deploy the MultichainAcl contract
task("task:deployMultichainAcl").setAction(async function (_, hre) {
  await deployContractImplementation("MultichainAcl", hre);
});

// Deploy the Decryption contract
task("task:deployDecryption").setAction(async function (_, hre) {
  await deployContractImplementation("Decryption", hre);
});

// Deploy all the contracts
task("task:deployAllGatewayContracts").setAction(async function (_, hre) {
  // Deploy the EmptyUUPS proxy contracts
  await hre.run("task:deployEmptyUUPSProxies");

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

  console.log("Deploy KmsManagement contract:");
  await hre.run("task:deployKmsManagement");

  console.log("Deploy CiphertextCommits contract:");
  await hre.run("task:deployCiphertextCommits");

  console.log("Deploy MultichainAcl contract:");
  await hre.run("task:deployMultichainAcl");

  console.log("Deploy Decryption contract:");
  await hre.run("task:deployDecryption");

  console.log("Contract deployment done!");
});
