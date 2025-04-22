import { HardhatEthersHelpers } from "@nomicfoundation/hardhat-ethers/types";
import { HardhatUpgrades } from "@openzeppelin/hardhat-upgrades";
import dotenv from "dotenv";
import { Wallet } from "ethers";
import fs from "fs";
import { task } from "hardhat/config";
import type { TaskArguments } from "hardhat/types";
import path from "path";

import { getRequiredEnvVar } from "./utils/loadVariables";

// Deploy a new EmptyUUPSProxy contract
async function deployEmptyUUPS(ethers: HardhatEthersHelpers, upgrades: HardhatUpgrades, deployer: Wallet) {
  const factory = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const UUPSEmpty = await upgrades.deployProxy(factory, [deployer.address], {
    initializer: "initialize",
    kind: "uups",
  });
  await UUPSEmpty.waitForDeployment();
  const UUPSEmptyAddress = await UUPSEmpty.getAddress();
  console.log("EmptyUUPS proxy contract successfully deployed!\n");
  return UUPSEmptyAddress;
}

task("task:deployEmptyUUPSProxies").setAction(async function (_, { ethers, upgrades, run }) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  console.log("Deploying an EmptyUUPS proxy contract for MultichainAcl...");
  const multichainAclAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "MultichainAcl",
    address: multichainAclAddress,
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

  console.log("Deploying an EmptyUUPS proxy contract for GatewayConfig...");
  const gatewayConfigAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "GatewayConfig",
    address: gatewayConfigAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for KmsManagement...");
  const kmsManagementAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "KmsManagement",
    address: kmsManagementAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for InputVerification...");
  const inputVerificationAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "InputVerification",
    address: inputVerificationAddress,
  });
});

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

  const parsedEnvGatewayConfig = dotenv.parse(fs.readFileSync("addresses/.env.gateway_config"));
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

  const parsedEnvInputVerification = dotenv.parse(fs.readFileSync("addresses/.env.input_verification"));
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

  const parsedEnvKmsManagement = dotenv.parse(fs.readFileSync("addresses/.env.kms_management"));
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

  const parsedEnvCiphertextCommits = dotenv.parse(fs.readFileSync("addresses/.env.ciphertext_commits"));
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

  const parsedEnvMultichainAcl = dotenv.parse(fs.readFileSync("addresses/.env.multichain_acl"));
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

  const parsedEnvDecryption = dotenv.parse(fs.readFileSync("addresses/.env.decryption"));
  const proxyAddress = parsedEnvDecryption.DECRYPTION_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, { call: { fn: "initialize" } });

  console.log(`Decryption code set successfully at address: ${proxyAddress}\n`);
});

// Deploy all the contracts
task("task:deployAllGatewayContracts").setAction(async function (_, hre) {
  await hre.run("clean");
  await hre.run("compile:specific", { contract: "contracts/emptyProxy" });
  await hre.run("task:deployEmptyUUPSProxies");

  // The deployEmptyUUPSProxies task may have updated the contracts' addresses in `addresses/*.sol`.
  // Thus, we must re-compile the contracts with these new addresses, otherwise the old ones will be
  // used.
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

// A helpher task to update a contract's address in their .sol and .env file in the `addresses` folder
task("task:setContractAddress")
  .addParam("name", "The name of the contract (PascalCase)")
  .addParam("address", "The address of the contract")
  .setAction(async function (taskArguments: TaskArguments) {
    const name = taskArguments.name;
    const address = taskArguments.address;

    // Write address of contract in its addresses/.env.xxx file
    const envFilePath = path.join(__dirname, `../addresses/.env.${pascalCaseToSnakeCase(name)}`);
    const content = `${pascalCaseToSnakeCase(name).toUpperCase()}_ADDRESS=${address}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: "w" });
      console.log(`${name} address ${address} written successfully!`);
    } catch (err) {
      console.error(`Failed to write ${name} address:`, err);
    }

    // Write address of contract in its addresses/xxxAddress.sol file
    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear\n
pragma solidity ^0.8.24;\n
address constant ${pascalCaseToCamelCase(name)}Address = ${address};\n`;

    try {
      fs.writeFileSync(`./addresses/${name}Address.sol`, solidityTemplate, {
        encoding: "utf8",
        flag: "w",
      });
      console.log(`./addresses/${name}Address.sol file generated successfully!\n`);
    } catch (error) {
      console.error(`Failed to write ./addresses/${name}Address.sol\n`, error);
    }
  });

function pascalCaseToSnakeCase(str: string) {
  return str
    .split(/\.?(?=[A-Z])/)
    .join("_")
    .toLowerCase();
}

function pascalCaseToCamelCase(str: string) {
  return str[0].toLowerCase() + str.substring(1);
}
