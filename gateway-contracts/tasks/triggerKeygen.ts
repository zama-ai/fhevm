import dotenv from "dotenv";
import { Wallet } from "ethers";
import fs from "fs";
import { task, types } from "hardhat/config";
import { HardhatEthersHelpers } from "hardhat/types";
import path from "path";

import { ADDRESSES_DIR } from "../hardhat.config";
import { getRequiredEnvVar } from "./utils/loadVariables";
import { pascalCaseToSnakeCase } from "./utils/stringOps";

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
    console.log("Trigger key generation in KmsManagement contract.");

    // Get the deployer wallet
    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    // Get contract factories
    if (useInternalKmsManagementAddress) {
      const envFilePath = path.join(ADDRESSES_DIR, `.env.gateway`);

      if (!fs.existsSync(envFilePath)) {
        throw new Error(`Environment file not found: ${envFilePath}`);
      }

      dotenv.config({ path: envFilePath, override: true });
    }

    const proxyAddress = getRequiredEnvVar("KMS_MANAGEMENT_ADDRESS");

    // Add host chains
    const kmsManagement = await hre.ethers.getContractAt("KmsManagement", proxyAddress, deployer);
    await kmsManagement.keygen(paramsType);

    console.log("In KmsManagement contract:", proxyAddress, "\n");
    console.log("Keygen triggering done!");
  });
