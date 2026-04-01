import { task, types } from "hardhat/config";

import { getRequiredEnvVar, getPauserSetContract } from "./utils/loadVariables";

task("task:addGatewayPausers")
  .addParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    const numPausers = parseInt(getRequiredEnvVar("NUM_PAUSERS"));

    const pausers = [];
    for (let idx = 0; idx < numPausers; idx++) {
      pausers.push(getRequiredEnvVar(`PAUSER_ADDRESS_${idx}`));
    }

    const pauserSet = await getPauserSetContract(useInternalProxyAddress, hre);
    for (const pauser of pausers) {
      await pauserSet.addPauser(pauser);
    }

    console.log("Added pausers:", pausers);
  });
