import { Interface, Wallet, ZeroAddress } from "ethers";
import { task, types } from "hardhat/config";
import { HardhatRuntimeEnvironment, TaskArguments } from "hardhat/types";

import { getRequiredEnvVar, loadGatewayAddresses } from "./utils";

const REINITIALIZE_FUNCTION_PREFIX = "reinitializeV"; // Prefix for reinitialize functions
const NO_PRIORITY_COPROCESSOR_TX_SENDER = ZeroAddress;

// This file defines generic tasks that can be used to upgrade the implementation of already deployed contracts.

type AbiFunction = {
  type?: string;
  name?: string;
  inputs?: { type?: string }[];
};

function getImplementationDirectory(input: string): string {
  const colonIndex = input.lastIndexOf("/");
  if (colonIndex !== -1) {
    return input.substring(0, colonIndex);
  }
  return input;
}

function getReinitializeFunction(abi: AbiFunction[]) {
  return abi.find((item) => item.type === "function" && item.name?.includes(REINITIALIZE_FUNCTION_PREFIX));
}

function getFunctionSignature(fn: AbiFunction): string {
  return `${fn.name}(${(fn.inputs ?? []).map((input) => input.type).join(",")})`;
}

function formatCastArg(arg: unknown): string {
  if (Array.isArray(arg)) {
    return `[${arg.map(formatCastArg).join(",")}]`;
  }
  return String(arg);
}

function shellQuote(arg: string): string {
  return `'${arg.replace(/'/g, `'\\''`)}'`;
}

// GatewayConfig v8 optionally enables the priority coprocessor during the upgrade
// so phase 2 can be configured in the same proposal as the implementation upgrade.
function getGatewayConfigV8ReinitializeArgs(taskArgs: TaskArguments): unknown[] {
  return [taskArgs.priorityCoprocessorTxSender ?? NO_PRIORITY_COPROCESSOR_TX_SENDER];
}

async function getGatewayConfigContract(taskArgs: TaskArguments, hre: HardhatRuntimeEnvironment) {
  if (taskArgs.useInternalProxyAddress) {
    loadGatewayAddresses();
  }
  const proxyAddress = getRequiredEnvVar("GATEWAY_CONFIG_ADDRESS");
  const deployer = new Wallet(getRequiredEnvVar("DEPLOYER_PRIVATE_KEY")).connect(hre.ethers.provider);
  return hre.ethers.getContractAt("GatewayConfig", proxyAddress, deployer);
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
  const reinitializeFunction = getReinitializeFunction(newImplementationArtifact.abi);
  if (!reinitializeFunction?.name) {
    throw new Error(`No reinitialize function found in ${newImplementation}`);
  }

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

// Relies on incremental compilation: run only on a clean working tree whose generated
// addresses/GatewayAddresses.sol matches the target environment, otherwise the implementation
// embeds the wrong addresses.
async function deployImplementationForPreparedUpgrade(
  proxyAddress: string,
  expectedArtifactName: string,
  currentImplementation: string,
  newImplementation: string,
  verifyContract: boolean,
  hre: HardhatRuntimeEnvironment,
  reinitializeArgs: unknown[] = [],
): Promise<void> {
  await compileImplementations(currentImplementation, newImplementation, hre);
  await checkImplementationArtifacts(expectedArtifactName, currentImplementation, newImplementation, hre);

  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(hre.ethers.provider);
  const currentImplementationFactory = await hre.ethers.getContractFactory(currentImplementation, deployer);
  await hre.upgrades.forceImport(proxyAddress, currentImplementationFactory);

  const newImplementationArtifact = await hre.artifacts.readArtifact(newImplementation);
  const reinitializeFunction = getReinitializeFunction(newImplementationArtifact.abi);
  if (!reinitializeFunction?.name) {
    throw new Error(`No reinitialize function found in ${newImplementation}`);
  }
  const newImplementationFactory = await hre.ethers.getContractFactory(newImplementation, deployer);

  console.log(`Deploying "${newImplementation}" for prepared upgrade on proxy ${proxyAddress}...`);
  const implementationAddress = String(
    await hre.upgrades.prepareUpgrade(proxyAddress, newImplementationFactory, {
      kind: "uups",
    }),
  );
  console.log("New implementation deployed at:", implementationAddress);

  const reinitializeFunctionSignature = getFunctionSignature(reinitializeFunction);
  const reinitializeCalldata = hre.ethers.Interface.from(newImplementationArtifact.abi).encodeFunctionData(
    reinitializeFunction.name,
    reinitializeArgs,
  );
  const outerCalldata = new Interface([
    "function upgradeToAndCall(address newImplementation, bytes data) payable",
  ]).encodeFunctionData("upgradeToAndCall", [implementationAddress, reinitializeCalldata]);

  console.log("proxyAddress:", proxyAddress);
  console.log("newImplementationAddress:", implementationAddress);
  console.log("innerFunctionSignature:", reinitializeFunctionSignature);
  console.log(`${reinitializeFunction.name} calldata:`, reinitializeCalldata);
  console.log("upgradeToAndCall(address,bytes) calldata:", outerCalldata);
  console.log(
    `To double check, run: cast calldata ${shellQuote(reinitializeFunctionSignature)} ${reinitializeArgs
      .map((arg) => shellQuote(formatCastArg(arg)))
      .join(" ")}`.trim(),
  );

  if (verifyContract) {
    console.log("Waiting 2 minutes before contract verification... Please wait...");
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    await hre.run("verify:verify", {
      address: implementationAddress,
      contract: newImplementation,
      constructorArguments: [],
    });
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

async function prepareUpgradeContract(
  contractName: string,
  addressEnvVar: string,
  taskArgs: TaskArguments,
  hre: HardhatRuntimeEnvironment,
  reinitializeArgs: unknown[] = [],
) {
  if (taskArgs.useInternalProxyAddress) {
    loadGatewayAddresses();
  }
  const proxyAddress = getRequiredEnvVar(addressEnvVar);

  await deployImplementationForPreparedUpgrade(
    proxyAddress,
    contractName,
    taskArgs.currentImplementation,
    taskArgs.newImplementation,
    taskArgs.verifyContract,
    hre,
    reinitializeArgs,
  );
}

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

task("task:prepareUpgradeDecryption")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/Decryption.sol:Decryption",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/Decryption.sol:Decryption",
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
    await prepareUpgradeContract("Decryption", "DECRYPTION_ADDRESS", taskArgs, hre);
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
  .addOptionalParam(
    "priorityCoprocessorTxSender",
    "Priority coprocessor transaction sender to set during reinitialization; zero leaves priority mode disabled. The host InputVerifier must accept the priority signer at threshold=1 before user inputs rely on priority mode",
    NO_PRIORITY_COPROCESSOR_TX_SENDER,
    types.string,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await upgradeContract(
      "GatewayConfig",
      "GATEWAY_CONFIG_ADDRESS",
      taskArgs,
      hre,
      getGatewayConfigV8ReinitializeArgs(taskArgs),
    );
  });

task("task:prepareUpgradeGatewayConfig")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/GatewayConfig.sol:GatewayConfig",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/GatewayConfig.sol:GatewayConfig",
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
  .addOptionalParam(
    "priorityCoprocessorTxSender",
    "Priority coprocessor transaction sender to set during reinitialization; zero leaves priority mode disabled. The host InputVerifier must accept the priority signer at threshold=1 before user inputs rely on priority mode",
    NO_PRIORITY_COPROCESSOR_TX_SENDER,
    types.string,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    await prepareUpgradeContract(
      "GatewayConfig",
      "GATEWAY_CONFIG_ADDRESS",
      taskArgs,
      hre,
      getGatewayConfigV8ReinitializeArgs(taskArgs),
    );
  });

task("task:setPriorityCoprocessorTxSender")
  .addParam(
    "priorityCoprocessorTxSender",
    "Registered coprocessor transaction sender to prioritize. The host InputVerifier must accept the priority signer at threshold=1 before user inputs rely on priority mode",
  )
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    const gatewayConfig = await getGatewayConfigContract(taskArgs, hre);
    const priorityCoprocessorTxSender = hre.ethers.getAddress(taskArgs.priorityCoprocessorTxSender);
    const tx = await gatewayConfig.setPriorityCoprocessorTxSender(priorityCoprocessorTxSender);
    console.log("Setting priority coprocessor transaction sender with tx:", tx.hash);
    await tx.wait();
  });

task(
  "task:removePriorityCoprocessorTxSender",
  "Remove priority coprocessor mode. Widen the host InputVerifier before user inputs rely on threshold mode again",
)
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function (taskArgs: TaskArguments, hre) {
    const gatewayConfig = await getGatewayConfigContract(taskArgs, hre);
    const tx = await gatewayConfig.removePriorityCoprocessorTxSender();
    console.log("Removing priority coprocessor transaction sender with tx:", tx.hash);
    await tx.wait();
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

task("task:prepareUpgradeKMSGeneration")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/KMSGeneration.sol:KMSGeneration",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/KMSGeneration.sol:KMSGeneration",
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
    await prepareUpgradeContract("KMSGeneration", "KMS_GENERATION_ADDRESS", taskArgs, hre);
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

task("task:prepareUpgradeInputVerification")
  .addParam(
    "currentImplementation",
    "The currently deployed implementation solidity contract path and name, eg: contracts/InputVerification.sol:InputVerification",
  )
  .addParam(
    "newImplementation",
    "The new implementation solidity contract path and name, eg: contracts/InputVerification.sol:InputVerification",
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
    await prepareUpgradeContract("InputVerification", "INPUT_VERIFICATION_ADDRESS", taskArgs, hre);
  });
