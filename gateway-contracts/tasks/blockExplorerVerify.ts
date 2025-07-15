import dotenv from "dotenv";
import fs from "fs";
import { task, types } from "hardhat/config";

import { getRequiredEnvVar } from "./utils/loadVariables";

task("task:verifyCiphertextCommits")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.ciphertext_commits"));
      proxyAddress = parsedEnv.CIPHERTEXT_COMMITS_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("CIPHERTEXT_COMMITS_ADDRESS");
    }

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
    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.decryption"));
      proxyAddress = parsedEnv.DECRYPTION_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("DECRYPTION_ADDRESS");
    }

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
    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.gateway_config"));
      proxyAddress = parsedEnv.GATEWAY_CONFIG_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("GATEWAY_CONFIG_ADDRESS");
    }

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
    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.input_verification"));
      proxyAddress = parsedEnv.INPUT_VERIFICATION_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("INPUT_VERIFICATION_ADDRESS");
    }

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
    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.kms_management"));
      proxyAddress = parsedEnv.KMS_MANAGEMENT_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("KMS_MANAGEMENT_ADDRESS");
    }

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
    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync("addresses/.env.multichain_acl"));
      proxyAddress = parsedEnv.MULTICHAIN_ACL_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("MULTICHAIN_ACL_ADDRESS");
    }

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
