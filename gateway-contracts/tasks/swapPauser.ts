import { task, types } from "hardhat/config";

import { getPauserSetContract } from "./utils/loadVariables";

task("task:swapGatewayPauser")
  .addParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .addParam("oldPauserAddress", "Address of the pauser to replace", undefined, types.string)
  .addParam("newPauserAddress", "Address of the new pauser", undefined, types.string)
  .setAction(async function ({ useInternalProxyAddress, oldPauserAddress, newPauserAddress }, hre) {
    const pauserSet = await getPauserSetContract(useInternalProxyAddress, hre);
    const tx = await pauserSet.swapPauser(oldPauserAddress, newPauserAddress);
    await tx.wait();
    console.log("Swapped pauser:", oldPauserAddress, "->", newPauserAddress);
  });
