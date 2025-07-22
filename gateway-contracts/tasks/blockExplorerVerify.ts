import dotenv from "dotenv";
import { task, types } from "hardhat/config";
import path from "path";

import { ADDRESSES_DIR } from "../hardhat.config";
import { getRequiredEnvVar } from "./utils/loadVariables";

task("task:verifyCiphertextCommits")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      dotenv.config({ path: path.join(ADDRESSES_DIR, ".env.gateway"), override: true });
    }
    const proxyAddress = getRequiredEnvVar("CIPHERTEXT_COMMITS_ADDRESS");

    const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run("verify:verify", {
      address: proxyAddress,
      constructorArguments: [],
    });
    await run("verify:verify", {
      address: implementationAddress,
      constructorArguments: [],
    });
  });

task("task:verifyDecryption")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      dotenv.config({ path: path.join(ADDRESSES_DIR, ".env.gateway"), override: true });
    }
    const proxyAddress = getRequiredEnvVar("DECRYPTION_ADDRESS");

    const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run("verify:verify", {
      address: proxyAddress,
      constructorArguments: [],
    });
    await run("verify:verify", {
      address: implementationAddress,
      constructorArguments: [],
    });
  });

task("task:verifyGatewayConfig")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      dotenv.config({ path: path.join(ADDRESSES_DIR, ".env.gateway"), override: true });
    }
    const proxyAddress = getRequiredEnvVar("GATEWAY_CONFIG_ADDRESS");

    const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run("verify:verify", {
      address: proxyAddress,
      constructorArguments: [],
    });
    await run("verify:verify", {
      address: implementationAddress,
      constructorArguments: [],
    });
  });

task("task:verifyInputVerification")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      dotenv.config({ path: path.join(ADDRESSES_DIR, ".env.gateway"), override: true });
    }
    const proxyAddress = getRequiredEnvVar("INPUT_VERIFICATION_ADDRESS");

    const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run("verify:verify", {
      address: proxyAddress,
      constructorArguments: [],
    });
    await run("verify:verify", {
      address: implementationAddress,
      constructorArguments: [],
    });
  });

task("task:verifyKmsManagement")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      dotenv.config({ path: path.join(ADDRESSES_DIR, ".env.gateway"), override: true });
    }
    const proxyAddress = getRequiredEnvVar("KMS_MANAGEMENT_ADDRESS");

    const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run("verify:verify", {
      address: proxyAddress,
      constructorArguments: [],
    });
    await run("verify:verify", {
      address: implementationAddress,
      constructorArguments: [],
    });
  });

task("task:verifyMultichainAcl")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      dotenv.config({ path: path.join(ADDRESSES_DIR, ".env.gateway"), override: true });
    }
    const proxyAddress = getRequiredEnvVar("MULTICHAIN_ACL_ADDRESS");

    const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run("verify:verify", {
      address: proxyAddress,
      constructorArguments: [],
    });
    await run("verify:verify", {
      address: implementationAddress,
      constructorArguments: [],
    });
  });

task("task:verifyAllGatewayContracts")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    console.log("Verify GatewayConfig contract:");
    await hre.run("task:verifyGatewayConfig", { useInternalProxyAddress });

    console.log("Verify InputVerification contract:");
    await hre.run("task:verifyInputVerification", { useInternalProxyAddress });

    console.log("Verify KmsManagement contract:");
    await hre.run("task:verifyKmsManagement", { useInternalProxyAddress });

    console.log("Verify CiphertextCommits contract:");
    await hre.run("task:verifyCiphertextCommits", { useInternalProxyAddress });

    console.log("Verify MultichainAcl contract:");
    await hre.run("task:verifyMultichainAcl", { useInternalProxyAddress });

    console.log("Verify Decryption contract:");
    await hre.run("task:verifyDecryption", { useInternalProxyAddress });

    console.log("Contract verification done!");
  });
