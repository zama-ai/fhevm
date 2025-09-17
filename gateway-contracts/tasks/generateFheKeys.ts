import dotenv from "dotenv";
import fs from "fs";
import { task, types } from "hardhat/config";
import path from "path";

import { ADDRESSES_DIR } from "../hardhat.config";
import { getRequiredEnvVar } from "./utils/loadVariables";

task("task:triggerKeygen")
  .addParam("paramsType", "The type of the parameters to use for the key generation.")
  .addOptionalParam(
    "useInternalKmsManagementAddress",
    "If proxy address from the /addresses directory should be used.",
    false,
    types.boolean,
  )
  .setAction(async function ({ paramsType, useInternalKmsManagementAddress }, hre) {
    await hre.run("compile:specific", { contract: "contracts" });
    console.log("Trigger key generation in KMSManagement contract.");

    // Get the deployer wallet.
    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    if (useInternalKmsManagementAddress) {
      const envFilePath = path.join(ADDRESSES_DIR, `.env.gateway`);

      if (!fs.existsSync(envFilePath)) {
        throw new Error(`Environment file not found: ${envFilePath}`);
      }
      dotenv.config({ path: envFilePath, override: true });
    }

    // Get KMSManagement contract.
    const proxyAddress = getRequiredEnvVar("KMS_MANAGEMENT_ADDRESS");
    const kmsManagement = await hre.ethers.getContractAt("KMSManagement", proxyAddress, deployer);

    // Call keygen function.
    await kmsManagement.keygen(paramsType);

    console.log("Keygen triggering done!");
  });

task("task:triggerCrsgen")
  .addParam("maxBitLength", "The maximum bit length for the CRS generation.")
  .addParam("paramsType", "The type of the parameters to use for the CRS generation.")
  .addOptionalParam(
    "useInternalKmsManagementAddress",
    "If proxy address from the /addresses directory should be used.",
    false,
    types.boolean,
  )
  .setAction(async function ({ maxBitLength, paramsType, useInternalKmsManagementAddress }, hre) {
    await hre.run("compile:specific", { contract: "contracts" });
    console.log("Trigger CRS generation in KMSManagement contract.");

    // Get the deployer wallet.
    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    if (useInternalKmsManagementAddress) {
      const envFilePath = path.join(ADDRESSES_DIR, `.env.gateway`);

      if (!fs.existsSync(envFilePath)) {
        throw new Error(`Environment file not found: ${envFilePath}`);
      }

      dotenv.config({ path: envFilePath, override: true });
    }

    // Get KMSManagement contract.
    const proxyAddress = getRequiredEnvVar("KMS_MANAGEMENT_ADDRESS");
    const kmsManagement = await hre.ethers.getContractAt("KMSManagement", proxyAddress, deployer);

    // Call crsgenRequest function.
    await kmsManagement.crsgenRequest(maxBitLength, paramsType);

    console.log("Crsgen triggering done!");
  });

task("task:getCrs").setAction(async function (_, hre) {
  await hre.run("compile:specific", { contract: "contracts" });
  console.log("Trigger CRS generation in KMSManagement contract.");

  // Get the deployer wallet.
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  const envFilePath = path.join(ADDRESSES_DIR, `.env.gateway`);
  dotenv.config({ path: envFilePath, override: true });

  // Get KMSManagement contract.
  const proxyAddress = getRequiredEnvVar("KMS_MANAGEMENT_ADDRESS");
  const kmsManagement = await hre.ethers.getContractAt("KMSManagement", proxyAddress, deployer);

  const crsgenRequestEvents = await kmsManagement.queryFilter(kmsManagement.filters.CrsgenRequest);
  console.log("CrsgenRequest events:", crsgenRequestEvents);

  const activateCrsEvents = await kmsManagement.queryFilter(kmsManagement.filters.ActivateCrs);
  console.log("ActivateCrs events:", activateCrsEvents);

  const crsId = await kmsManagement.getActiveCrsId();
  console.log("active CRS ID", crsId);
});

task("task:getKey").setAction(async function (_, hre) {
  await hre.run("compile:specific", { contract: "contracts" });
  console.log("Trigger Key generation in KMSManagement contract.");

  // Get the deployer wallet.
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  const envFilePath = path.join(ADDRESSES_DIR, `.env.gateway`);
  dotenv.config({ path: envFilePath, override: true });

  // Get KMSManagement contract.
  const proxyAddress = getRequiredEnvVar("KMS_MANAGEMENT_ADDRESS");
  const kmsManagement = await hre.ethers.getContractAt("KMSManagement", proxyAddress, deployer);

  const prepKeygenEvents = await kmsManagement.queryFilter(kmsManagement.filters.PrepKeygenRequest);
  console.log("PrepKeygenRequest events:", prepKeygenEvents);

  const keygenRequestEvents = await kmsManagement.queryFilter(kmsManagement.filters.KeygenRequest);
  console.log("KeygenRequest events:", keygenRequestEvents);

  const activateKeyEvents = await kmsManagement.queryFilter(kmsManagement.filters.ActivateKey);
  console.log("ActivateKey events:", activateKeyEvents);
});
