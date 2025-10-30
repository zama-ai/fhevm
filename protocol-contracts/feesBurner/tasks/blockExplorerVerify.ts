import { task } from "hardhat/config";

import { getRequiredEnvVar } from "../deploy/utils/loadVariables";

// Verify the ProtocolFeesBurner contract at the given address.
task("task:verifyProtocolFeesBurner")
  .addParam("protocolFeesBurner", "address of the already deployed ProtocolFeesBurner contract that should be verified")
  .setAction(async function ({ protocolFeesBurner }, { run }) {
    const zamaERC20Address = getRequiredEnvVar("ZAMA_ERC20_ADDRESS");

    await run("verify:verify", {
      address: protocolFeesBurner,
      constructorArguments: [zamaERC20Address],
    });
  });

// Verify the FeesSenderToBurner contract at the given address.
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
