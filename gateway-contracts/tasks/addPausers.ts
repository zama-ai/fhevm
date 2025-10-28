import { task, types } from "hardhat/config";

import { getRequiredEnvVar, loadGatewayAddresses } from "./utils/loadVariables";

// Add pausers to the PauserSet contract
// Note: Internal PauserSet address is defined in the `addresses/` directory. It should be used
// for local testing. By default, we use the PAUSER_SET_ADDRESS env var, as done in deployment
task("task:addGatewayPausers")
  .addParam("useInternalPauserSetAddress", "If internal PauserSet address should be used", false, types.boolean)
  .setAction(async function ({ useInternalGatewayConfigAddress }, hre) {
    await hre.run("compile:specific", { contract: "contracts/immutable" });
    console.log("Adding pausers to PauserSet contract");

    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const numPausers = parseInt(getRequiredEnvVar("NUM_PAUSERS"));
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    // Parse the pauser(s)
    const pausers = [];
    for (let idx = 0; idx < numPausers; idx++) {
      pausers.push(getRequiredEnvVar(`PAUSER_ADDRESS_${idx}`));
    }

    if (useInternalGatewayConfigAddress) {
      loadGatewayAddresses();
    }
    const pauserSetAddress = getRequiredEnvVar("PAUSER_SET_ADDRESS");

    // Add pauser(s)
    const pauserSet = await hre.ethers.getContractAt("PauserSet", pauserSetAddress, deployer);
    for (const pauser of pausers) {
      await pauserSet.addPauser(pauser);
    }

    console.log("In PauserSet contract:", pauserSetAddress, "\n");
    console.log("Added pausers:", pausers, "\n");
    console.log("Pausers registration done!");
  });
