import { task, types } from "hardhat/config";

import { getRequiredEnvVar } from "./utils/loadVariables";

// Verify the ProtocolFeesBurner contract at the given address.
task("task:verifyProtocolFeesBurner")
  .addParam("protocolFeesBurner", "Address of the already deployed ProtocolFeesBurner contract that should be verified")
  .setAction(async function ({ protocolFeesBurner }, hre) {
    const zamaERC20Address = getRequiredEnvVar("ZAMA_ERC20_ADDRESS");
    const apiKey = getRequiredEnvVar("ETHERSCAN_API");

    if (typeof hre.config.etherscan.apiKey !== "string") {
      console.log(
        "Verification on Ethereum requires using Etherscan API V2, only available when the etherscan.apiKey field is a string.",
      );
      hre.config.etherscan.apiKey = apiKey;
    }

    await hre.run("verify:verify", {
      address: protocolFeesBurner,
      constructorArguments: [zamaERC20Address],
    });
  });

// Verify the FeesSenderToBurner contract at the given address.
task("task:verifyFeesSenderToBurner")
  .addParam("feesSenderToBurner", "Address of the already deployed FeesSenderToBurner contract that should be verified")
  .addOptionalParam(
    "protocolFeesBurner",
    "Address of the ProtocolFeesBurner contract used in the constructor when deploying the FeesSenderToBurner contract being verified.",
    undefined,
    types.string,
  )
  .setAction(async function ({ feesSenderToBurner, protocolFeesBurner }, hre) {
    const oftAddress = getRequiredEnvVar("ZAMA_OFT_ADDRESS");
    const protocolFeesBurnerAddress = protocolFeesBurner ?? getRequiredEnvVar("PROTOCOL_FEES_BURNER_ADDRESS");

    if (typeof hre.config.etherscan.apiKey === "string") {
      console.log(
        "Verification on Gateway requires using BlockScout API, only available when the etherscan.apiKey field is not a string.",
      );
      hre.config.etherscan.apiKey = { "gateway-testnet": "empty", "gateway-mainnet": "empty" };
    }

    if (!protocolFeesBurner) {
      throw new Error("The ProtocolFeesBurner address cannot be empty for verification.");
    }

    await hre.run("verify:verify", {
      address: feesSenderToBurner,
      constructorArguments: [oftAddress, protocolFeesBurnerAddress],
    });
  });
