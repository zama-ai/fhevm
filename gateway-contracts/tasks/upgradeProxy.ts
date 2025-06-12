import dotenv from "dotenv";
import { Wallet } from "ethers";
import fs from "fs";
import { task, types } from "hardhat/config";
import { HardhatRuntimeEnvironment, TaskArguments } from "hardhat/types";

import { getRequiredEnvVar } from "./utils/loadVariables";

// This file defines generic tasks that can be used to upgrade the implementation of already deployed contracts.

function getImplementationDirectory(input: string): string {
  const colonIndex = input.lastIndexOf("/");
  if (colonIndex !== -1) {
    return input.substring(0, colonIndex);
  }
  return input;
}

// Upgrades the implementation of the proxy
async function upgradeCurrentToNew(
  proxyAddress: string,
  currentImplementation: string,
  newImplementation: string,
  verifyContract: boolean,
  hre: HardhatRuntimeEnvironment,
) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  console.log(`Importing ${currentImplementation} contract implementation at address ${proxyAddress}...`);
  const currentImplementationFactory = await hre.ethers.getContractFactory(currentImplementation, deployer);
  const currentProxyContract = await hre.upgrades.forceImport(proxyAddress, currentImplementationFactory);
  console.log("Proxy contract successfully loaded!");

  console.log(`Upgrading proxy to ${newImplementation} contract implementation...`);
  const newImplementationFactory = await hre.ethers.getContractFactory(newImplementation, deployer);
  await hre.upgrades.upgradeProxy(currentProxyContract, newImplementationFactory);
  console.log("Proxy contract successfully upgraded!");

  if (verifyContract) {
    console.log("Waiting 2 minutes before contract verification... Please wait...");
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    const implementationAddress = await hre.upgrades.erc1967.getImplementationAddress(proxyAddress);
    await hre.run("verify:verify", {
      address: implementationAddress,
      constructorArguments: [],
    });
  }
}

async function compileImplementations(
  currentImplementation: string,
  newImplementation: string,
  hre: HardhatRuntimeEnvironment,
): Promise<void> {
  await hre.run("compile:specific", { contract: getImplementationDirectory(currentImplementation) });
  await hre.run("compile:specific", { contract: getImplementationDirectory(newImplementation) });
}

async function checkImplementationArtifacts(
  expectedArtifactName: string,
  currentImplementation: string,
  newImplementation: string,
  hre: HardhatRuntimeEnvironment,
): Promise<void> {
  const currentImplementationArtifact = await hre.artifacts.readArtifact(currentImplementation);
  if (currentImplementationArtifact.contractName !== expectedArtifactName) {
    throw new Error(
      `The current implementation artifact does not match the expected contract name "${expectedArtifactName}". Found: ${currentImplementationArtifact.contractName}`,
    );
  }

  const newImplementationArtifact = await hre.artifacts.readArtifact(newImplementation);
  if (newImplementationArtifact.contractName !== expectedArtifactName) {
    throw new Error(
      `The new implementation artifact does not match the expected contract name "${expectedArtifactName}". Found: ${newImplementationArtifact.contractName}`,
    );
  }
}

task("task:upgradeMultichainAcl")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/MultichainAcl.sol:MultichainAcl",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/examples/MultichainAclUpgradedExample.sol:MultichainAclUpgradedExample",
  )
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .addOptionalParam(
    "verifyContract",
    "Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)",
    false,
    types.boolean,
  )
  .setAction(async function (
    { currentImplementation, newImplementation, useInternalProxyAddress, verifyContract }: TaskArguments,
    hre,
  ) {
    await compileImplementations(currentImplementation, newImplementation, hre);

    await checkImplementationArtifacts("MultichainAcl", currentImplementation, newImplementation, hre);

    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.multichain_acl"));
      proxyAddress = parsedEnv.MULTICHAIN_ACL_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("MULTICHAIN_ACL_ADDRESS");
    }

    await upgradeCurrentToNew(proxyAddress, currentImplementation, newImplementation, verifyContract, hre);
  });

task("task:upgradeCiphertextCommits")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/CiphertextCommits.sol:CiphertextCommits",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/examples/CiphertextCommitsUpgradedExample.sol:CiphertextCommitsUpgradedExample",
  )
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .addOptionalParam(
    "verifyContract",
    "Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)",
    false,
    types.boolean,
  )
  .setAction(async function (
    { currentImplementation, newImplementation, useInternalProxyAddress, verifyContract }: TaskArguments,
    hre,
  ) {
    await compileImplementations(currentImplementation, newImplementation, hre);

    await checkImplementationArtifacts("CiphertextCommits", currentImplementation, newImplementation, hre);

    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.ciphertext_commits"));
      proxyAddress = parsedEnv.CIPHERTEXT_COMMITS_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("CIPHERTEXT_COMMITS_ADDRESS");
    }

    await upgradeCurrentToNew(proxyAddress, currentImplementation, newImplementation, verifyContract, hre);
  });

task("task:upgradeDecryption")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/Decryption.sol:Decryption",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/examples/DecryptionUpgradedExample.sol:DecryptionUpgradedExample",
  )
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .addOptionalParam(
    "verifyContract",
    "Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)",
    false,
    types.boolean,
  )
  .setAction(async function (
    { currentImplementation, newImplementation, useInternalProxyAddress, verifyContract }: TaskArguments,
    hre,
  ) {
    await compileImplementations(currentImplementation, newImplementation, hre);

    await checkImplementationArtifacts("Decryption", currentImplementation, newImplementation, hre);

    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.decryption"));
      proxyAddress = parsedEnv.DECRYPTION_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("DECRYPTION_ADDRESS");
    }

    await upgradeCurrentToNew(proxyAddress, currentImplementation, newImplementation, verifyContract, hre);
  });

task("task:upgradeGatewayConfig")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/GatewayConfig.sol:GatewayConfig",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/examples/GatewayConfigUpgradedExample.sol:GatewayConfigUpgradedExample",
  )
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .addOptionalParam(
    "verifyContract",
    "Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)",
    false,
    types.boolean,
  )
  .setAction(async function (
    { currentImplementation, newImplementation, useInternalProxyAddress, verifyContract }: TaskArguments,
    hre,
  ) {
    await compileImplementations(currentImplementation, newImplementation, hre);

    await checkImplementationArtifacts("GatewayConfig", currentImplementation, newImplementation, hre);

    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.gateway_config"));
      proxyAddress = parsedEnv.GATEWAY_CONFIG_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("GATEWAY_CONFIG_ADDRESS");
    }

    await upgradeCurrentToNew(proxyAddress, currentImplementation, newImplementation, verifyContract, hre);
  });

task("task:upgradeKmsManagement")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/KmsManagement.sol:KmsManagement",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/examples/KmsManagementUpgradedExample.sol:KmsManagementUpgradedExample",
  )
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .addOptionalParam(
    "verifyContract",
    "Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)",
    false,
    types.boolean,
  )
  .setAction(async function (
    { currentImplementation, newImplementation, useInternalProxyAddress, verifyContract }: TaskArguments,
    hre,
  ) {
    await compileImplementations(currentImplementation, newImplementation, hre);

    await checkImplementationArtifacts("KmsManagement", currentImplementation, newImplementation, hre);

    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.kms_management"));
      proxyAddress = parsedEnv.KMS_MANAGEMENT_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("KMS_MANAGEMENT_ADDRESS");
    }

    await upgradeCurrentToNew(proxyAddress, currentImplementation, newImplementation, verifyContract, hre);
  });

task("task:upgradeInputVerification")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/InputVerification.sol:InputVerification",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/examples/InputVerificationUpgradedExample.sol:InputVerificationUpgradedExample",
  )
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .addOptionalParam(
    "verifyContract",
    "Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)",
    false,
    types.boolean,
  )
  .setAction(async function (
    { currentImplementation, newImplementation, useInternalProxyAddress, verifyContract }: TaskArguments,
    hre,
  ) {
    await compileImplementations(currentImplementation, newImplementation, hre);

    await checkImplementationArtifacts("InputVerification", currentImplementation, newImplementation, hre);

    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.input_verification"));
      proxyAddress = parsedEnv.INPUT_VERIFICATION_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("INPUT_VERIFICATION_ADDRESS");
    }

    await upgradeCurrentToNew(proxyAddress, currentImplementation, newImplementation, verifyContract, hre);
  });
