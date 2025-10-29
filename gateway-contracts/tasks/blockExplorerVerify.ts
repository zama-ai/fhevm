import { task, types } from "hardhat/config";

import { getRequiredEnvVar, loadGatewayAddresses } from "./utils";

task("task:verifyCiphertextCommits")
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
      loadGatewayAddresses();
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

task("task:verifyInputVerification")
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

task("task:verifyMultichainACL")
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

task("task:verifyAllGatewayContracts")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    try {
      // to not panic if Blockscout throws an error due to already verified implementation
      console.log("Verify GatewayConfig contract:");
      await hre.run("task:verifyGatewayConfig", { useInternalProxyAddress });
    } catch (error) {
      console.error("An error occurred:", error);
    }
    try {
      // to not panic if Blockscout throws an error due to already verified implementation
      console.log("Verify InputVerification contract:");
      await hre.run("task:verifyInputVerification", { useInternalProxyAddress });
    } catch (error) {
      console.error("An error occurred:", error);
    }
    try {
      // to not panic if Blockscout throws an error due to already verified implementation
      console.log("Verify KMSGeneration contract:");
      await hre.run("task:verifyKMSGeneration", { useInternalProxyAddress });
    } catch (error) {
      console.error("An error occurred:", error);
    }
    try {
      // to not panic if Blockscout throws an error due to already verified implementation
      console.log("Verify CiphertextCommits contract:");
      await hre.run("task:verifyCiphertextCommits", { useInternalProxyAddress });
    } catch (error) {
      console.error("An error occurred:", error);
    }
    try {
      // to not panic if Blockscout throws an error due to already verified implementation
      console.log("Verify MultichainACL contract:");
      await hre.run("task:verifyMultichainACL", { useInternalProxyAddress });
    } catch (error) {
      console.error("An error occurred:", error);
    }
    try {
      // to not panic if Blockscout throws an error due to already verified implementation
      console.log("Verify Decryption contract:");
      await hre.run("task:verifyDecryption", { useInternalProxyAddress });
    } catch (error) {
      console.error("An error occurred:", error);
    }
    try {
      // to not panic if Blockscout throws an error due to already verified implementation
      console.log("Verify PauserSet contract:");
      await hre.run("task:verifyPauserSet", { useInternalProxyAddress });
    } catch (error) {
      console.error("An error occurred:", error);
    }
    try {
      // to not panic if Blockscout throws an error due to already verified implementation
      console.log("Contract verification done!");
    } catch (error) {
      console.error("An error occurred:", error);
    }
  });
