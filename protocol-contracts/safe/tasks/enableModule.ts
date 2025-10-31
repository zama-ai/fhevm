import MultiSendJson from "@safe-global/safe-contracts/build/artifacts/contracts/libraries/MultiSend.sol/MultiSend.json";
import { ContractFactory } from "ethers";
import { task } from "hardhat/config";

import { getRequiredEnvVar, getDeployedAddress } from "./utils/loadVariables";

task("task:enableAdminModule").setAction(async function (_, { run, network }) {
  const adminModuleAddress = await getDeployedAddress(
    network.name,
    "AdminModule",
  );
  // TODO
});
