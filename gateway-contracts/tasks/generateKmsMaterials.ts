import dotenv from "dotenv";
import fs from "fs";
import { task, types } from "hardhat/config";
import path from "path";

import { ADDRESSES_DIR } from "../hardhat.config";
import { getRequiredEnvVar } from "./utils/loadVariables";

task("task:triggerKeygen")
  .addParam("paramsType", "The type of the parameters to use for the key generation.")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used.",
    false,
    types.boolean,
  )
  .setAction(async function ({ paramsType, useInternalProxyAddress }, hre) {
    await hre.run("compile:specific", { contract: "contracts" });
    console.log("Trigger key generation in KMSGeneration contract.");

    // Get the deployer wallet.
    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    if (useInternalProxyAddress) {
      const envFilePath = path.join(ADDRESSES_DIR, `.env.gateway`);

      if (!fs.existsSync(envFilePath)) {
        throw new Error(`Environment file not found: ${envFilePath}`);
      }
      dotenv.config({ path: envFilePath, override: true });
    }

    // Get KMSGeneration contract.
    const proxyAddress = getRequiredEnvVar("KMS_GENERATION_ADDRESS");
    const kmsGeneration = await hre.ethers.getContractAt("KMSGeneration", proxyAddress, deployer);

    // Request the key generation.
    await kmsGeneration.keygen(paramsType);

    console.log("Keygen triggering done!");
  });

task("task:triggerCrsgen")
  .addParam("maxBitLength", "The maximum bit length for the CRS generation.")
  .addParam("paramsType", "The type of the parameters to use for the CRS generation.")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used.",
    false,
    types.boolean,
  )
  .setAction(async function ({ maxBitLength, paramsType, useInternalProxyAddress }, hre) {
    await hre.run("compile:specific", { contract: "contracts" });
    console.log("Trigger CRS generation in KMSGeneration contract.");

    // Get the deployer wallet.
    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    if (useInternalProxyAddress) {
      const envFilePath = path.join(ADDRESSES_DIR, `.env.gateway`);

      if (!fs.existsSync(envFilePath)) {
        throw new Error(`Environment file not found: ${envFilePath}`);
      }

      dotenv.config({ path: envFilePath, override: true });
    }

    // Get KMSGeneration contract.
    const proxyAddress = getRequiredEnvVar("KMS_GENERATION_ADDRESS");
    const kmsGeneration = await hre.ethers.getContractAt("KMSGeneration", proxyAddress, deployer);

    // Request the CRS generation.
    await kmsGeneration.crsgenRequest(maxBitLength, paramsType);

    console.log("Crsgen triggering done!");
  });
