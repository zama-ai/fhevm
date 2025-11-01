import MultiSendJson from "@safe-global/safe-contracts/build/artifacts/contracts/libraries/MultiSend.sol/MultiSend.json";
import { ContractFactory } from "ethers";
import { task } from "hardhat/config";

import { getRequiredEnvVar, getDeployedAddress } from "./utils/loadVariables";

task("task:acceptOwnership").setAction(async function (
  _,
  { getNamedAccounts, ethers, deployments, network },
) {
  // TODO
});
