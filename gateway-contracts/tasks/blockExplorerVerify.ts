import { task, types } from "hardhat/config";

import { getRequiredEnvVar, loadGatewayAddresses } from "./utils";

task("task:verifyGatewayConfig")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadGatewayAddresses();
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

task("task:verifyKMSGeneration")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadGatewayAddresses();
    }
    const proxyAddress = getRequiredEnvVar("KMS_GENERATION_ADDRESS");

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

task("task:verifyPauserSet")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadGatewayAddresses();
    }
    const implementationAddress = getRequiredEnvVar("PAUSER_SET_ADDRESS");
    await run("verify:verify", {
      address: implementationAddress,
      constructorArguments: [],
    });
  });

task("task:verifyProtocolPayment")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadGatewayAddresses();
    }
    const proxyAddress = getRequiredEnvVar("PROTOCOL_PAYMENT_ADDRESS");

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
    try {
      console.log("Verify GatewayConfig contract:");
      await hre.run("task:verifyGatewayConfig", { useInternalProxyAddress });
    } catch (error) {
      console.error("An error occurred:", error);
    }
    try {
      console.log("Verify KMSGeneration contract:");
      await hre.run("task:verifyKMSGeneration", { useInternalProxyAddress });
    } catch (error) {
      console.error("An error occurred:", error);
    }
    try {
      console.log("Verify PauserSet contract:");
      await hre.run("task:verifyPauserSet", { useInternalProxyAddress });
    } catch (error) {
      console.error("An error occurred:", error);
    }
    try {
      console.log("Verify ProtocolPayment contract:");
      await hre.run("task:verifyProtocolPayment", { useInternalProxyAddress });
    } catch (error) {
      console.error("An error occurred:", error);
    }
    console.log("Contract verification done!");
  });
