import dotenv from "dotenv";
import { task, types } from "hardhat/config";
import path from "path";

import { ADDRESSES_DIR } from "../hardhat.config";
import { getRequiredEnvVar } from "./utils/loadVariables";

task("task:prssInit")
  .addParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    await hre.run("compile:specific", { contract: "contracts" });
    console.log("Initialize PRSS");

    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    if (useInternalProxyAddress) {
      dotenv.config({ path: path.join(ADDRESSES_DIR, ".env.gateway"), override: true });
    }
    const proxyAddress = getRequiredEnvVar("KMS_GENERATION_ADDRESS");

    const kmsGeneration = await hre.ethers.getContractAt("KMSGeneration", proxyAddress, deployer);
    await kmsGeneration.prssInit();

    console.log("PRSS initialization done!");
  });

task("task:keyReshareSameSet")
  .addParam("keyId", "The ID of the key to reshare")
  .addParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ keyId, useInternalProxyAddress }, hre) {
    await hre.run("compile:specific", { contract: "contracts" });
    console.log("Reshare same set for key ID:", keyId);

    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    if (useInternalProxyAddress) {
      dotenv.config({ path: path.join(ADDRESSES_DIR, ".env.gateway"), override: true });
    }
    const proxyAddress = getRequiredEnvVar("KMS_GENERATION_ADDRESS");

    const kmsGeneration = await hre.ethers.getContractAt("KMSGeneration", proxyAddress, deployer);
    await kmsGeneration.keyReshareSameSet(keyId);

    console.log("Key reshare same set done for key ID:", keyId);
  });
