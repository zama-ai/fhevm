import dotenv from "dotenv";
import { Wallet } from "ethers";
import fs from "fs";
import { task } from "hardhat/config";
import path from "path";

import { getRequiredEnvVar } from "../utils/loadVariables";

const ADDRESSES_DIR = path.join(__dirname, "../../addresses");

// Deploy the GatewayConfig contract
task("task:deployGatewayConfig").setAction(async function (_, { ethers, upgrades }) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Parse the protocol metadata
  const protocolMetadata = {
    name: getRequiredEnvVar("PROTOCOL_NAME"),
    website: getRequiredEnvVar("PROTOCOL_WEBSITE"),
  };

  // Parse the pauser address
  const pauserAddress = getRequiredEnvVar(`PAUSER_ADDRESS`);

  // Parse the KMS threshold
  const kmsThreshold = getRequiredEnvVar("KMS_THRESHOLD");

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

  // Upgrade proxy to GatewayConfig
  const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const newImplem = await ethers.getContractFactory("GatewayConfig", deployer);

  const parsedEnvGatewayConfig = dotenv.parse(fs.readFileSync(path.join(ADDRESSES_DIR, ".env.gateway_config")));
  const proxyAddress = parsedEnvGatewayConfig.GATEWAY_CONFIG_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: {
      fn: "initialize",
      args: [pauserAddress, protocolMetadata, kmsThreshold, kmsNodes, coprocessors],
    },
  });

  console.log("GatewayConfig contract deployed to:", proxyAddress);
  console.log("Pauser address:", pauserAddress, "\n");
  console.log("Protocol metadata:", protocolMetadata);
  console.log("KMS threshold:", kmsThreshold, "\n");
  console.log("KMS nodes:", kmsNodes, "\n");
  console.log("Coprocessors:", coprocessors, "\n");
});

// Deploy the InputVerification contract
task("task:deployInputVerification").setAction(async function (_, { ethers, upgrades }) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Upgrade proxy to InputVerification
  const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const newImplem = await ethers.getContractFactory("InputVerification", deployer);

  const parsedEnvInputVerification = dotenv.parse(fs.readFileSync(path.join(ADDRESSES_DIR, ".env.input_verification")));
  const proxyAddress = parsedEnvInputVerification.INPUT_VERIFICATION_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, { call: { fn: "initialize" } });

  console.log(`InputVerification code set successfully at address: ${proxyAddress}\n`);
});

// Deploy the KmsManagement contract
task("task:deployKmsManagement").setAction(async function (_, { ethers, upgrades }) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  const fheParamsName = getRequiredEnvVar("FHE_PARAMS_NAME");
  const fheParamsDigest = getRequiredEnvVar("FHE_PARAMS_DIGEST");

  // Upgrade proxy to KmsManagement
  const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const newImplem = await ethers.getContractFactory("KmsManagement", deployer);

  const parsedEnvKmsManagement = dotenv.parse(fs.readFileSync(path.join(ADDRESSES_DIR, ".env.kms_management")));
  const proxyAddress = parsedEnvKmsManagement.KMS_MANAGEMENT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: { fn: "initialize", args: [fheParamsName, fheParamsDigest] },
  });

  console.log(`KmsManagement code set successfully at address: ${proxyAddress}\n`);
});

// Deploy the CiphertextCommits contract
task("task:deployCiphertextCommits").setAction(async function (_, { ethers, upgrades }) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Upgrade proxy to CiphertextCommits
  const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const newImplem = await ethers.getContractFactory("CiphertextCommits", deployer);

  const parsedEnvCiphertextCommits = dotenv.parse(fs.readFileSync(path.join(ADDRESSES_DIR, ".env.ciphertext_commits")));
  const proxyAddress = parsedEnvCiphertextCommits.CIPHERTEXT_COMMITS_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, { call: { fn: "initialize" } });

  console.log(`CiphertextCommits code set successfully at address: ${proxyAddress}\n`);
});

// Deploy the MultichainAcl contract
task("task:deployMultichainAcl").setAction(async function (_, { ethers, upgrades }) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Upgrade proxy to MultichainAcl
  const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const newImplem = await ethers.getContractFactory("MultichainAcl", deployer);

  const parsedEnvMultichainAcl = dotenv.parse(fs.readFileSync(path.join(ADDRESSES_DIR, ".env.multichain_acl")));
  const proxyAddress = parsedEnvMultichainAcl.MULTICHAIN_ACL_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, { call: { fn: "initialize" } });

  console.log(`MultichainAcl code set successfully at address: ${proxyAddress}\n`);
});

// Deploy the Decryption contract
task("task:deployDecryption").setAction(async function (_, { ethers, upgrades }) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Upgrade proxy to Decryption
  const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const newImplem = await ethers.getContractFactory("Decryption", deployer);

  const parsedEnvDecryption = dotenv.parse(fs.readFileSync(path.join(ADDRESSES_DIR, ".env.decryption")));
  const proxyAddress = parsedEnvDecryption.DECRYPTION_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, { call: { fn: "initialize" } });

  console.log(`Decryption code set successfully at address: ${proxyAddress}\n`);
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
