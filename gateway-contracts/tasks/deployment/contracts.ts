import dotenv from "dotenv";
import { EventLog, Wallet } from "ethers";
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

  // Parse the pauser smart account address
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

  // Parse the coprocessors
  const numCoprocessors = parseInt(getRequiredEnvVar("NUM_COPROCESSORS"));
  const coprocessors = [];
  for (let idx = 0; idx < numCoprocessors; idx++) {
    coprocessors.push({
      txSenderAddress: getRequiredEnvVar(`COPROCESSOR_TX_SENDER_ADDRESS_${idx}`),
      signerAddress: getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`),
      s3BucketUrl: getRequiredEnvVar(`COPROCESSOR_S3_BUCKET_URL_${idx}`),
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
  console.log("Coprocessors:", coprocessors);
  console.log("Custodians:", custodians);

  await deployContractImplementation("GatewayConfig", hre, [
    pauserAddress,
    protocolMetadata,
    mpcThreshold,
    publicDecryptionThreshold,
    userDecryptionThreshold,
    kmsNodes,
    coprocessors,
    custodians,
  ]);
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

task("task:deployPauserSmartAccount").setAction(async function (_, { ethers, network, run }) {
  // Get a deployer wallet
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Deploy a new Safe contract
  const safeFactory = await ethers.getContractFactory("Safe", deployer);
  const safe = await safeFactory.deploy();
  const safeAddress = await safe.getAddress();

  // Deploy a new SafeProxyFactory contract
  const safeProxyFactoryFactory = await ethers.getContractFactory("SafeProxyFactory", deployer);
  const safeProxyFactory = await safeProxyFactoryFactory.deploy();

  // Prepare the setup transaction data
  const owners = [await deployer.getAddress()]; // List of Safe owners.
  const threshold = 1; // Number of required confirmations for a Safe transaction.
  const to = ethers.ZeroAddress; // Contract address for optional delegate call.
  const data = "0x"; // Data payload for optional delegate call.
  const fallbackHandler = ethers.ZeroAddress; // Handler for fallback calls to this contract.
  const paymentToken = ethers.ZeroAddress; // Token that should be used for the payment (0 is ETH).
  const payment = 0; // Value that should be paid.
  const paymentReceiver = ethers.ZeroAddress; // Address that should receive the payment (or 0 if tx.origin).

  // Encode the setup function data
  const safeData = safe.interface.encodeFunctionData("setup", [
    owners,
    threshold,
    to,
    data,
    fallbackHandler,
    paymentToken,
    payment,
    paymentReceiver,
  ]);

  // Setup the Safe proxy factory
  const saltNonce = 0n;
  const txResponse = await safeProxyFactory.createProxyWithNonce(safeAddress, safeData, saltNonce);
  const txReceipt = await txResponse.wait();
  if (!txReceipt) {
    throw new Error("Create Safe proxy transaction receipt not found");
  }

  // Get the Safe proxy address from the ProxyCreation event
  const event = txReceipt.logs
    .filter((l) => l instanceof EventLog)
    .find((l) => l.eventName === safeProxyFactory.getEvent("ProxyCreation").name);
  if (!event) {
    throw new Error("ProxyCreation event not found in transaction receipt");
  }
  const safeProxyAddress = event.args.proxy;

  if (safeProxyAddress === ethers.ZeroAddress) {
    throw new Error("Safe proxy address not found");
  }

  await run("task:setContractAddress", {
    name: "PauserSmartAccount",
    address: safeProxyAddress,
  });
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

  console.log("Deploy PauserSmartAccount contract:");
  await hre.run("task:deployPauserSmartAccount");

  console.log("Deploy GatewayConfig contract:");
  await hre.run("task:deployGatewayConfig");

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
