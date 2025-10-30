import { task, types } from "hardhat/config";

import { getHreByNetworkName } from "./utils/getHreByNetworkName";

/** Promise to sleep for a given number of milliseconds. */
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

interface DeploymentConfig {
  ethereumNetwork: string;
  gatewayNetwork: string;
}

const CONFIGS: Record<string, DeploymentConfig> = {
  mainnet: {
    ethereumNetwork: "ethereum-mainnet",
    gatewayNetwork: "gateway-mainnet",
  },
  testnet: {
    ethereumNetwork: "ethereum-testnet",
    gatewayNetwork: "gateway-testnet",
  },
};

const PROTOCOL_FEES_BURNER = "ProtocolFeesBurner";
const FEES_SENDER_TO_BURNER = "FeesSenderToBurner";

/**
 * Unified deployment task, deploying ProtocolFeesBurner and FeesSenderToBurner on Ethereum & Gateway.
 * Choose whether to deploy on testnet on mainnet with the `--preset <mainnet|testnet>` flag.
 * Verify both contracts with the `--verify true` flag.
 */
task("deploy:feesBurner")
  .addOptionalParam("preset", "Deployment preset to use.", undefined, types.string)
  .addOptionalParam(
    "verify",
    "Activate verification of the ProtocolFeesBurner contract on Etherscan",
    false,
    types.boolean,
  )
  .setAction(async function ({ preset, verify }, hre) {
    const config = CONFIGS[preset];

    if (!config) {
      throw new Error(`Unknown config: ${preset}. Available: ${Object.keys(CONFIGS).join(", ")}`);
    }

    // Set HardhatRuntimeEnvironment to Ethereum <Mainnet|Testnet>
    hre = await getHreByNetworkName(config.ethereumNetwork);

    // Deploy ProtocolFeesBurner on Ethereum
    await hre.run("deploy", {
      tags: PROTOCOL_FEES_BURNER,
    });

    const protocolFeesBurnerAddress = (await hre.deployments.get(PROTOCOL_FEES_BURNER)).address;

    // Deploy FeesSenderToBurner on Gateway
    // Set HardhatRuntimeEnvironment to Gateway <Mainnet|Testnet>
    hre = await getHreByNetworkName(config.gatewayNetwork);
    process.env["PROTOCOL_FEES_BURNER_ADDRESS"] = protocolFeesBurnerAddress;
    await hre.run("deploy", {
      tags: FEES_SENDER_TO_BURNER,
    });

    const feesSenderToBurnerAddress = (await hre.deployments.get(FEES_SENDER_TO_BURNER)).address;

    if (verify) {
      // Wait 2 minutes for proper indexing on the networks.
      await sleep(120000);

      // Verify ProtocolFeesBurnerAddress
      // Switch back to Ethereum <Mainnet|Testnet>
      hre = await getHreByNetworkName(config.ethereumNetwork);
      await hre.run("task:verifyProtocolFeesBurner", {
        protocolFeesBurner: protocolFeesBurnerAddress,
      });

      // Set back HardhatRuntimeEnvironment to Gateway <Mainnet|Testnet>
      hre = await getHreByNetworkName(config.gatewayNetwork);
      await hre.run("task:verifyFeesSenderToBurner", {
        feesSenderToBurner: feesSenderToBurnerAddress,
        protocolFeesBurner: protocolFeesBurnerAddress,
      });
    }
  });
