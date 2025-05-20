import dotenv from "dotenv";
import fs from "fs";
import { task } from "hardhat/config";
import type { HardhatEthersHelpers, TaskArguments } from "hardhat/types";

import { GatewayConfig } from "../typechain-types";

async function loadGatewayConfigContract(
  customGatewayConfigAddress: string | undefined,
  ethers: HardhatEthersHelpers,
): Promise<GatewayConfig> {
  const gatewayConfigFactory = await ethers.getContractFactory("./contracts/GatewayConfig.sol:GatewayConfig");
  const gatewayConfigAddress = customGatewayConfigAddress
    ? customGatewayConfigAddress
    : dotenv.parse(fs.readFileSync("addresses/.env.gateway_config")).GATEWAY_CONFIG_ADDRESS;
  return gatewayConfigFactory.attach(gatewayConfigAddress).connect(ethers.provider) as GatewayConfig;
}

task("task:getKmsSigners")
  .addOptionalParam(
    "customGatewayConfigAddress",
    "Use a custom address for the GatewayConfig contract instead of the default one - ie stored inside .env.gateway_config",
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const gatewayConfig = await loadGatewayConfigContract(taskArguments.customGatewayConfigAddress, ethers);
    const listCurrentKMSSigners = await gatewayConfig.getKmsSigners();
    console.log("The list of current KMS Signers stored inside GatewayConfig contract is: ", listCurrentKMSSigners);
  });

task("task:getCoprocessorSigners")
  .addOptionalParam(
    "customGatewayConfigAddress",
    "Use a custom address for the GatewayConfig contract instead of the default one - ie stored inside .env.gateway_config",
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const gatewayConfig = await loadGatewayConfigContract(taskArguments.customGatewayConfigAddress, ethers);
    const listCurrentCoprocessorSigners = await gatewayConfig.getCoprocessorSigners();
    console.log(
      "The list of current Coprocessor Signers stored inside GatewayConfig contract is: ",
      listCurrentCoprocessorSigners,
    );
  });

task("task:getHostChains")
  .addOptionalParam(
    "customGatewayConfigAddress",
    "Use a custom address for the GatewayConfig contract instead of the default one - ie stored inside .env.gateway_config",
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const gatewayConfig = await loadGatewayConfigContract(taskArguments.customGatewayConfigAddress, ethers);
    const listCurrentHostChains = await gatewayConfig.getHostChains();
    console.log("The list of current host chains stored inside GatewayConfig contract is: ", listCurrentHostChains);
  });
