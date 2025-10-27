import { task } from "hardhat/config";

import { getRequiredEnvVar } from "../deploy/utils/loadVariables";

task("task:verifyFeesSenderToBurner")
  .addParam("feesSenderToBurner", "address of the already deployed FeesSenderToBurner contract that should be verified")
  .setAction(async function ({ feesSenderToBurner }, { run }) {
    const oftAddress = getRequiredEnvVar("ZAMA_OFT_ADDRESS");
    const protocolFeesBurnerAddress = getRequiredEnvVar("PROTOCOL_FEES_BURNER_ADDRESS");

    await run("verify:verify", {
      address: feesSenderToBurner,
      constructorArguments: [oftAddress, protocolFeesBurnerAddress],
    });
  });
