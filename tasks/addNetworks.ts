import dotenv from "dotenv";
import fs from "fs";
import { task, types } from "hardhat/config";

import { getRequiredEnvVar } from "./utils/loadVariables";

// Add host networks metadata to the GatewayConfig contract
// Note: Internal GatewayConfig address is defined in the `addresses/` directory. It should be used
// for local testing. By default, we use the GATEWAY_CONFIG_ADDRESS env var, as done in deployment
task("task:addNetworksToGatewayConfig")
  .addParam("useInternalGatewayConfigAddress", "If internal GatewayConfig address should be used", false, types.boolean)
  .setAction(async function (taskArgs, hre) {
    await hre.run("clean");
    await hre.run("compile");
    console.log("Register networks to GatewayConfig contract");

    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const numNetworks = parseInt(getRequiredEnvVar("NUM_NETWORKS"));
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    // Parse the host network(s)
    const hostNetworks = [];
    for (let idx = 0; idx < numNetworks; idx++) {
      hostNetworks.push({
        chainId: getRequiredEnvVar(`NETWORK_CHAIN_ID_${idx}`),
        httpzExecutor: getRequiredEnvVar(`NETWORK_HTTPZ_EXECUTOR_${idx}`),
        aclAddress: getRequiredEnvVar(`NETWORK_ACL_ADDRESS_${idx}`),
        name: getRequiredEnvVar(`NETWORK_NAME_${idx}`),
        website: getRequiredEnvVar(`NETWORK_WEBSITE_${idx}`),
      });
    }

    let proxyAddress: string;
    if (taskArgs.useInternalGatewayConfigAddress) {
      const parsedEnvGatewayConfig = dotenv.parse(fs.readFileSync("addresses/.env.gateway_config"));
      proxyAddress = parsedEnvGatewayConfig.GATEWAY_CONFIG_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("GATEWAY_CONFIG_ADDRESS");
    }

    // Add host networks
    const gatewayConfig = await hre.ethers.getContractAt("GatewayConfig", proxyAddress, deployer);
    for (const network of hostNetworks) {
      await gatewayConfig.addNetwork(network);
    }

    console.log("In GatewayConfig contract:", proxyAddress, "\n");
    console.log("Added host networks:", hostNetworks, "\n");
    console.log("Networks registration done!");
  });
