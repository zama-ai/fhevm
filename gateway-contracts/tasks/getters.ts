import dotenv from "dotenv";
import { task, types } from "hardhat/config";
import type { HardhatEthersHelpers } from "hardhat/types";
import path from "path";

import { CoprocessorContexts, GatewayConfig } from "../typechain-types";
import { ADDRESSES_DIR } from "../hardhat.config";
import { getRequiredEnvVar } from "./utils/loadVariables";

async function loadGatewayConfigContract(ethers: HardhatEthersHelpers): Promise<GatewayConfig> {
  const proxyAddress = getRequiredEnvVar("GATEWAY_CONFIG_ADDRESS");

  console.log("In GatewayConfig contract:", proxyAddress, "\n");

  const contract = await ethers.getContractAt("GatewayConfig", proxyAddress);
  return contract as GatewayConfig;
}

async function loadCoprocessorContextsContract(ethers: HardhatEthersHelpers): Promise<CoprocessorContexts> {
  const proxyAddress = getRequiredEnvVar("COPROCESSOR_CONTEXTS_ADDRESS");

  console.log("In CoprocessorContexts contract:", proxyAddress, "\n");

  const contract = await ethers.getContractAt("CoprocessorContexts", proxyAddress);
  return contract as CoprocessorContexts;
}

task("task:getKmsSigners")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    await hre.run("compile:specific", { contract: "contracts" });
    console.log("Get registered KMS signers");

    if (useInternalProxyAddress) {
      dotenv.config({ path: path.join(ADDRESSES_DIR, ".env.gateway"), override: true });
    }
    const gatewayConfig = await loadGatewayConfigContract(hre.ethers);
    const listCurrentKMSSigners = await gatewayConfig.getKmsSigners();

    console.log("Registered KMS signers: ", listCurrentKMSSigners);
  });

task("task:getCoprocessorSigners")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .addOptionalParam(
    "coprocessorContextId",
    "The ID of the coprocessor context to get the signers from. If not provided, the active context will be used.",
    undefined,
    types.int,
  )
  .setAction(async function ({ useInternalProxyAddress, coprocessorContextId }, hre) {
    await hre.run("compile:specific", { contract: "contracts" });
    if (coprocessorContextId) {
      console.log("Getting registered coprocessor signers from coprocessor context: ", coprocessorContextId);
    } else {
      console.log("Getting registered coprocessor signers from active coprocessor context.");
    }

    if (useInternalProxyAddress) {
      dotenv.config({ path: path.join(ADDRESSES_DIR, ".env.gateway"), override: true });
    }
    const coprocessorContexts = await loadCoprocessorContextsContract(hre.ethers);
    const contextId = coprocessorContextId || (await coprocessorContexts.getActiveCoprocessorContextId());
    const listCurrentCoprocessorSigners = await coprocessorContexts.getCoprocessorSigners(contextId);

    console.log(
      "Registered coprocessor signers for context ID ",
      contextId,
      ": ",
      listCurrentCoprocessorSigners,
    );
  });

task("task:getHostChains")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    await hre.run("compile:specific", { contract: "contracts" });
    console.log("Get registered host chains");

    if (useInternalProxyAddress) {
      dotenv.config({ path: path.join(ADDRESSES_DIR, ".env.gateway"), override: true });
    }
    const gatewayConfig = await loadGatewayConfigContract(hre.ethers);
    const listCurrentHostChains = await gatewayConfig.getHostChains();

    console.log("Registered host chains: ", listCurrentHostChains);
  });

task("task:getAllRegisteredEntities")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .addOptionalParam(
    "coprocessorContextId",
    "The ID of the coprocessor context to get the signers from. If not provided, the active context will be used.",
    undefined,
    types.int,
  )
  .setAction(async function ({ useInternalProxyAddress, coprocessorContextId }, hre) {
    await hre.run("task:getKmsSigners", { useInternalProxyAddress });
    await hre.run("task:getCoprocessorSigners", { useInternalProxyAddress, coprocessorContextId });
    await hre.run("task:getHostChains", { useInternalProxyAddress });
  });
