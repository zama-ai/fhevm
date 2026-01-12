import { Wallet } from "ethers";
import { task, types } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";

import { getRequiredAddressEnvVar, getRequiredEnvVar, loadGatewayAddresses } from "../utils";
import { setPaymentBridgingContractAddresses } from "./paymentBridging/setAddresses";
import { GATEWAY_CONFIG_EMPTY_PROXY_NAME, REGULAR_EMPTY_PROXY_NAME } from "./utils";

// Helper function to deploy a contract implementation to its proxy
async function deployContractImplementation(
  name: string,
  hre: HardhatRuntimeEnvironment,
  emptyProxyName: string,
  useInternalProxyAddress: boolean = false,
  initializeArgs?: unknown[],
  addressName?: string,
): Promise<string> {
  const { ethers, upgrades } = hre;

  // Get a deployer wallet
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Get contract factories
  const proxyImplementation = await ethers.getContractFactory(emptyProxyName, deployer);
  const newImplem = await ethers.getContractFactory(name, deployer);

  if (useInternalProxyAddress) {
    loadGatewayAddresses();
  }

  // Get the proxy address
  const proxyAddress = getRequiredAddressEnvVar(addressName ?? name);

  // Force import
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);

  // Set the upgrade options
  const upgradeOptions = {
    call: {
      fn: "initializeFromEmptyProxy",
      args: [] as unknown[],
    },
  };
  if (Array.isArray(initializeArgs) && initializeArgs.length > 0) {
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

  // Parse the coprocessor threshold
  const coprocessorThreshold = getRequiredEnvVar("COPROCESSOR_THRESHOLD");

  // Parse the KMS nodes
  const numKmsNodes = parseInt(getRequiredEnvVar("NUM_KMS_NODES"));
  const kmsNodes = [];
  for (let idx = 0; idx < numKmsNodes; idx++) {
    kmsNodes.push({
      txSenderAddress: getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`),
      signerAddress: getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`),
      ipAddress: getRequiredEnvVar(`KMS_NODE_IP_ADDRESS_${idx}`),
      storageUrl: getRequiredEnvVar(`KMS_NODE_STORAGE_URL_${idx}`),
      apiUrl: getRequiredEnvVar(`KMS_NODE_API_URL_${idx}`),
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
      apiUrl: getRequiredEnvVar(`COPROCESSOR_API_URL_${idx}`),
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
  console.log("Coprocessors:", coprocessors);
  console.log("Custodians:", custodians);

  // Define the thresholds struct
  const thresholds = {
    mpcThreshold,
    publicDecryptionThreshold,
    userDecryptionThreshold,
    kmsGenThreshold,
    coprocessorThreshold,
  };

  // The GatewayConfig contract is not deployed using the same empty proxy as the other contracts,
  // as it is made ownable
  await deployContractImplementation("GatewayConfig", hre, GATEWAY_CONFIG_EMPTY_PROXY_NAME, true, [
    protocolMetadata,
    thresholds,
    kmsNodes,
    coprocessors,
    custodians,
  ]);
});

// Deploy the KMSGeneration contract
task("task:deployKMSGeneration").setAction(async function (_, hre) {
  await deployContractImplementation("KMSGeneration", hre, REGULAR_EMPTY_PROXY_NAME, true);
});

// Deploy the ProtocolPayment contract
task("task:deployProtocolPayment").setAction(async function (_, hre) {
  const inputVerificationPrice = getRequiredEnvVar("INPUT_VERIFICATION_PRICE");
  const publicDecryptionPrice = getRequiredEnvVar("PUBLIC_DECRYPTION_PRICE");
  const userDecryptionPrice = getRequiredEnvVar("USER_DECRYPTION_PRICE");

  await deployContractImplementation("ProtocolPayment", hre, REGULAR_EMPTY_PROXY_NAME, true, [
    inputVerificationPrice,
    publicDecryptionPrice,
    userDecryptionPrice,
  ]);
});

// Deploy the DecryptionRegistry contract
task("task:deployDecryptionRegistry").setAction(async function (_, hre) {
  await deployContractImplementation("DecryptionRegistry", hre, REGULAR_EMPTY_PROXY_NAME, true, undefined, "Decryption");
});

// Deploy the InputVerificationRegistry contract
task("task:deployInputVerificationRegistry").setAction(async function (_, hre) {
  await deployContractImplementation(
    "InputVerificationRegistry",
    hre,
    REGULAR_EMPTY_PROXY_NAME,
    true,
    undefined,
    "InputVerification",
  );
});

// Deploy setup contracts, needed before deploying the implementation contracts
task("task:deploySetupContracts")
  .addOptionalParam(
    "deployMockedZamaOft",
    "If mocked payment bridging contracts should be deployed",
    false,
    types.boolean,
  )
  .setAction(async function ({ deployMockedZamaOft }, hre) {
    // Deploy the mocked payment bridging contracts if needed
    if (deployMockedZamaOft) {
      await hre.run("task:deployMockedZamaOFT");
    }

    // Deploy the EmptyUUPS proxy contracts
    await hre.run("task:deployEmptyUUPSProxies");

    // Deploy the PauserSet contract
    await hre.run("task:deployPauserSet");

    // Register the payment bridging contract addresses
    // Note: these contracts should already be deployed and their address registered as env vars
    setPaymentBridgingContractAddresses();
  });

// Deploy implementation contracts
task("task:deployImplementationContracts").setAction(async function (_, hre) {
  // Compile the implementation contracts
  // The deployEmptyUUPSProxies task has generated the contracts' addresses in `addresses/*.sol`.
  // Contracts thus need to be compiled after deploying the EmptyUUPS proxy contracts in order to
  // use these addresses. Otherwise, irrelevant addresses will be used and, although deployment will
  // succeed, most transactions made to the contracts will revert as inter-contract calls will fail.
  await hre.run("compile:specific", { contract: "contracts" });

  console.log("Deploy GatewayConfig contract:");
  await hre.run("task:deployGatewayConfig");

  console.log("Deploy KMSGeneration contract:");
  await hre.run("task:deployKMSGeneration");

  console.log("Deploy ProtocolPayment contract:");
  await hre.run("task:deployProtocolPayment");

  console.log("Deploy DecryptionRegistry contract:");
  await hre.run("task:deployDecryptionRegistry");

  console.log("Deploy InputVerificationRegistry contract:");
  await hre.run("task:deployInputVerificationRegistry");

  console.log("Contract deployment done!");
});

// Deploy all the contracts
task("task:deployAllGatewayContracts").setAction(async function (_, hre) {
  // Deploy all the setup contracts
  await hre.run("task:deploySetupContracts");

  // Deploy all the implementation contracts
  await hre.run("task:deployImplementationContracts");
});

// Deploy all the contracts, including the mocked ZamaOFT one (FOR TESTS ONLY)
task("task:deployAllGatewayContractsForTests").setAction(async function (_, hre) {
  // Deploy all the setup contracts, including the mocked ZamaOFT one
  await hre.run("task:deploySetupContracts", { deployMockedZamaOft: true });

  // Deploy all the implementation contracts
  await hre.run("task:deployImplementationContracts");
});

// Deploy a single contract, after the GatewayConfig contract has been deployed
// The new contract address will be appended to the .env.gateway and GatewayAddresses.sol files
task("task:deploySingleContract")
  .addParam("name", "The name of the contract")
  .addParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ name, useInternalProxyAddress }, hre) {
    // Deploy the EmptyUUPS proxy contracts
    await hre.run("task:deploySingleEmptyUUPSProxy", { name, useInternalProxyAddress });

    // Add a small delay to ensure the proxy transaction is confirmed
    const delay = 2000; // 2 seconds
    console.log(`Waiting for proxy transaction to be confirmed for ${delay}ms...`);
    await new Promise((resolve) => setTimeout(resolve, delay));

    // Compile the implementation contracts
    // The deployEmptyUUPSProxies task has generated the contracts' addresses in `addresses/*.sol`.
    // Contracts thus need to be compiled after deploying the EmptyUUPS proxy contracts in order to
    // use these addresses. Otherwise, irrelevant addresses will be used and, although deployment will
    // succeed, most transactions made to the contracts will revert as inter-contract calls will fail.
    await hre.run("compile:specific", { contract: "contracts" });

    console.log(`Deploy ${name} contract:`);
    await hre.run(`task:deploy${name}`);

    console.log(`Contract deployment done: ${name} !`);
  });
