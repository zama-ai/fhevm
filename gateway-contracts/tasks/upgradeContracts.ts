import { Wallet } from "ethers";
import { task, types } from "hardhat/config";
import { HardhatRuntimeEnvironment, TaskArguments } from "hardhat/types";

import { getRequiredEnvVar, loadGatewayAddresses } from "./utils";

const REINITIALIZE_FUNCTION_PREFIX = "reinitializeV"; // Prefix for reinitialize functions

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
  reinitializeArgs: unknown[] = [],
) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  console.log(`Importing ${currentImplementation} contract implementation at address ${proxyAddress}...`);
  const currentImplementationFactory = await hre.ethers.getContractFactory(currentImplementation, deployer);
  const currentProxyContract = await hre.upgrades.forceImport(proxyAddress, currentImplementationFactory);
  console.log("Proxy contract successfully loaded!");

  console.log(
    `Upgrading proxy to "${newImplementation}" implementation with reinitialize arguments:`,
    reinitializeArgs,
  );

  // Get reinitialize function from the new implementation artifact
  const newImplementationArtifact = await hre.artifacts.readArtifact(newImplementation);
  const reinitializeFunction = newImplementationArtifact.abi.find(
    (item) => item.type === "function" && item.name.includes(REINITIALIZE_FUNCTION_PREFIX),
  );

  // Prepare the new implementation factory and execute the upgrade by calling the reinitialize function
  const newImplementationFactory = await hre.ethers.getContractFactory(newImplementation, deployer);

  await hre.upgrades.upgradeProxy(currentProxyContract, newImplementationFactory, {
    call: {
      fn: reinitializeFunction.name,
      args: reinitializeArgs,
    },
  });
  console.log("Proxy contract successfully upgraded!");

  if (verifyContract) {
    console.log("Waiting 2 minutes before contract verification... Please wait...");
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    const implementationAddress = await hre.upgrades.erc1967.getImplementationAddress(proxyAddress);
    await hre.run("verify:verify", {
      address: implementationAddress,
      contract: newImplementation,
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

  const hasReinitializeFunction = newImplementationArtifact.abi.some(
    (item) => item.type === "function" && item.name.includes(REINITIALIZE_FUNCTION_PREFIX),
  );
  if (!hasReinitializeFunction) {
    throw new Error(
      `The new implementation artifact does not contain a reinitialize function. Please ensure the contract has a reinitialize function defined.`,
    );
  }
}

// Helper to perform a standard upgrade: compile, check artifacts, load address, upgrade
async function upgradeContract(
  contractName: string,
  addressEnvVar: string,
  taskArgs: TaskArguments,
  hre: HardhatRuntimeEnvironment,
  reinitializeArgs: unknown[] = [],
) {
  await compileImplementations(taskArgs.currentImplementation, taskArgs.newImplementation, hre);
  await checkImplementationArtifacts(contractName, taskArgs.currentImplementation, taskArgs.newImplementation, hre);

  if (taskArgs.useInternalProxyAddress) {
    loadGatewayAddresses();
  }
  const proxyAddress = getRequiredEnvVar(addressEnvVar);

  await upgradeCurrentToNew(
    proxyAddress,
    taskArgs.currentImplementation,
    taskArgs.newImplementation,
    taskArgs.verifyContract,
    hre,
    reinitializeArgs,
  );
}

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
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await upgradeContract("CiphertextCommits", "CIPHERTEXT_COMMITS_ADDRESS", taskArgs, hre);
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
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await upgradeContract("Decryption", "DECRYPTION_ADDRESS", taskArgs, hre);
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
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    const kmsContextId = getRequiredEnvVar("KMS_CONTEXT_ID");
    await upgradeContract("GatewayConfig", "GATEWAY_CONFIG_ADDRESS", taskArgs, hre, [kmsContextId]);
  });

task("task:upgradeKMSGeneration")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/KMSGeneration.sol:KMSGeneration",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/examples/KMSGenerationUpgradedExample.sol:KMSGenerationUpgradedExample",
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
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await upgradeContract("KMSGeneration", "KMS_GENERATION_ADDRESS", taskArgs, hre);
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
    true,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await upgradeContract("InputVerification", "INPUT_VERIFICATION_ADDRESS", taskArgs, hre);
  });
