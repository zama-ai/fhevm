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

  console.log("Deploying an EmptyUUPS proxy contract for ACLManager...");
  const aclManagerAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "AclManager",
    address: aclManagerAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for CiphertextManager...");
  const ciphertextManagerAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "CiphertextManager",
    address: ciphertextManagerAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for DecryptionManager...");
  const decryptionManagerAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "DecryptionManager",
    address: decryptionManagerAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for HTTPZ...");
  const httpzAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "Httpz",
    address: httpzAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for KeyManager...");
  const keyManagerAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "KeyManager",
    address: keyManagerAddress,
  });

  console.log("Deploying an EmptyUUPS proxy contract for ZKPoKManager...");
  const zkpokManagerAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
  await run("task:setContractAddress", {
    name: "ZkpokManager",
    address: zkpokManagerAddress,
  });
});

// Deploy the HTTPZ contract
task("task:deployHttpz").setAction(async function (_, { ethers, upgrades }) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Parse the protocol metadata
  const protocolMetadata = {
    name: getRequiredEnvVar("PROTOCOL_NAME"),
    website: getRequiredEnvVar("PROTOCOL_WEBSITE"),
  };

  // Parse the admin address
  const adminAddress = getRequiredEnvVar(`ADMIN_ADDRESS`);

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

  // Upgrade proxy to HTTPZ
  const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const newImplem = await ethers.getContractFactory("HTTPZ", deployer);

  const parsedEnvHttpz = dotenv.parse(fs.readFileSync("addresses/.env.httpz"));
  const proxyAddress = parsedEnvHttpz.HTTPZ_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: {
      fn: "initialize",
      args: [protocolMetadata, adminAddress, kmsThreshold, kmsNodes, coprocessors],
    },
  });

  console.log("HTTPZ contract deployed to:", proxyAddress);
  console.log("Protocol metadata:", protocolMetadata);
  console.log("Admin address:", adminAddress, "\n");
  console.log("KMS threshold:", kmsThreshold, "\n");
  console.log("KMS nodes:", kmsNodes, "\n");
  console.log("Coprocessors:", coprocessors, "\n");
});

// Deploy the ZKPoKManager contract
task("task:deployZkpokManager").setAction(async function (_, { ethers, upgrades }) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Upgrade proxy to ZKPoKManager
  const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const newImplem = await ethers.getContractFactory("ZKPoKManager", deployer);

  const parsedEnvZkpokManager = dotenv.parse(fs.readFileSync("addresses/.env.zkpok_manager"));
  const proxyAddress = parsedEnvZkpokManager.ZKPOK_MANAGER_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, { call: { fn: "initialize" } });

  console.log(`ZKPoKManager code set successfully at address: ${proxyAddress}\n`);
});

// Deploy the KeyManager contract
task("task:deployKeyManager").setAction(async function (_, { ethers, upgrades }) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  const fheParamsName = getRequiredEnvVar("FHE_PARAMS_NAME");
  const fheParamsDigest = getRequiredEnvVar("FHE_PARAMS_DIGEST");

  // Upgrade proxy to KeyManager
  const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const newImplem = await ethers.getContractFactory("KeyManager", deployer);

  const parsedEnvKeyManager = dotenv.parse(fs.readFileSync("addresses/.env.key_manager"));
  const proxyAddress = parsedEnvKeyManager.KEY_MANAGER_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: { fn: "initialize", args: [fheParamsName, fheParamsDigest] },
  });

  console.log(`KeyManager code set successfully at address: ${proxyAddress}\n`);
});

// Deploy the CiphertextManager contract
task("task:deployCiphertextManager").setAction(async function (_, { ethers, upgrades }) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Upgrade proxy to CiphertextManager
  const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const newImplem = await ethers.getContractFactory("CiphertextManager", deployer);

  const parsedEnvCiphertextManager = dotenv.parse(fs.readFileSync("addresses/.env.ciphertext_manager"));
  const proxyAddress = parsedEnvCiphertextManager.CIPHERTEXT_MANAGER_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, { call: { fn: "initialize" } });

  console.log(`CiphertextManager code set successfully at address: ${proxyAddress}\n`);
});

// Deploy the ACLManager contract
task("task:deployAclManager").setAction(async function (_, { ethers, upgrades }) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Upgrade proxy to ACLManager
  const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const newImplem = await ethers.getContractFactory("ACLManager", deployer);

  const parsedEnvAclManager = dotenv.parse(fs.readFileSync("addresses/.env.acl_manager"));
  const proxyAddress = parsedEnvAclManager.ACL_MANAGER_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, { call: { fn: "initialize" } });

  console.log(`ACLManager code set successfully at address: ${proxyAddress}\n`);
});

// Deploy the DecryptionManager contract
task("task:deployDecryptionManager").setAction(async function (_, { ethers, upgrades }) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Upgrade proxy to DecryptionManager
  const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const newImplem = await ethers.getContractFactory("DecryptionManager", deployer);

  const parsedEnvDecryptionManager = dotenv.parse(fs.readFileSync("addresses/.env.decryption_manager"));
  const proxyAddress = parsedEnvDecryptionManager.DECRYPTION_MANAGER_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, { call: { fn: "initialize" } });

  console.log(`DecryptionManager code set successfully at address: ${proxyAddress}\n`);
});

// Deploy all the contracts
task("task:deployAllContracts").setAction(async function (_, hre) {
  await hre.run("clean");
  await hre.run("compile:specific", { contract: "contracts/emptyProxy" });
  await hre.run("task:deployEmptyUUPSProxies");

  // The deployEmptyUUPSProxies task may have updated the contracts' addresses in `addresses/*.sol`.
  // Thus, we must re-compile the contracts with these new addresses, otherwise the old ones will be
  // used.
  await hre.run("compile:specific", { contract: "contracts" });

  console.log("Deploy HTTPZ contract:");
  await hre.run("task:deployHttpz");

  console.log("Deploy ZKPoKManager contract:");
  await hre.run("task:deployZkpokManager");

  console.log("Deploy KeyManager contract:");
  await hre.run("task:deployKeyManager");

  console.log("Deploy CiphertextManager contract:");
  await hre.run("task:deployCiphertextManager");

  console.log("Deploy ACLManager contract:");
  await hre.run("task:deployAclManager");

  console.log("Deploy DecryptionManager contract:");
  await hre.run("task:deployDecryptionManager");

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
