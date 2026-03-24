import { task, types } from "hardhat/config";
import type { TaskArguments } from "hardhat/types";

import { GatewayConfig } from "../typechain-types";

import { getRequiredEnvVar, loadGatewayAddresses } from "./utils";

async function loadGatewayConfigContract(
  useInternalProxyAddress: boolean,
  ethers: typeof import("hardhat").ethers,
): Promise<GatewayConfig> {
  if (useInternalProxyAddress) {
    loadGatewayAddresses();
  }
  const gatewayConfigAddress = getRequiredEnvVar("GATEWAY_CONFIG_ADDRESS");
  const gatewayConfigFactory = await ethers.getContractFactory("./contracts/GatewayConfig.sol:GatewayConfig");
  return gatewayConfigFactory.attach(gatewayConfigAddress).connect(ethers.provider) as GatewayConfig;
}

task("task:getKmsSigners")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const gatewayConfig = await loadGatewayConfigContract(taskArguments.useInternalProxyAddress, ethers);
    const listCurrentKMSSigners = await gatewayConfig.getKmsSigners();
    console.log("The list of current KMS Signers stored inside GatewayConfig contract is: ", listCurrentKMSSigners);
  });

task("task:getCoprocessorSigners")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const gatewayConfig = await loadGatewayConfigContract(taskArguments.useInternalProxyAddress, ethers);
    const listCurrentCoprocessorSigners = await gatewayConfig.getCoprocessorSigners();
    console.log(
      "The list of current Coprocessor Signers stored inside GatewayConfig contract is: ",
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
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const gatewayConfig = await loadGatewayConfigContract(taskArguments.useInternalProxyAddress, ethers);
    const listCurrentHostChains = await gatewayConfig.getHostChains();
    console.log("The list of current host chains stored inside GatewayConfig contract is: ", listCurrentHostChains);
  });
