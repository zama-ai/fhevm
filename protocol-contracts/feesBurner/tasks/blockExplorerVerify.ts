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
  .setAction(async function ({ feesSenderToBurner }, hre) {
    const oftAddress = getRequiredEnvVar("ZAMA_OFT_ADDRESS");
    const protocolFeesBurner = getRequiredEnvVar("PROTOCOL_FEES_BURNER_ADDRESS");

    if (typeof hre.config.etherscan.apiKey === "string") {
      console.log(
        "Verification on Gateway requires using BlockScout API, only available when the etherscan.apiKey field is not a string.",
      );
      hre.config.etherscan.apiKey = { "gateway-testnet": "empty", "gateway-mainnet": "empty" };
    }

    await hre.run("verify:verify", {
      address: feesSenderToBurner,
      constructorArguments: [oftAddress, protocolFeesBurner],
    });
  });
