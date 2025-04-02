import dotenv from "dotenv";
import fs from "fs";
import { task, types } from "hardhat/config";

import { getRequiredEnvVar } from "./utils/loadVariables";

// Add L1 networks metadata to the HTTPZ contract
// Note: Internal HTTPZ address is defined in the `addresses/` directory. It should be used
// for local testing. By default, we use the HTTPZ_ADDRESS env var, as done in deployment
task("task:addNetworksToHttpz")
  .addParam("useInternalHttpzAddress", "If internal HTTPZ address should be used", false, types.boolean)
  .setAction(async function (taskArgs, hre) {
    await hre.run("clean");
    await hre.run("compile");
    console.log("Register networks to HTTPZ contract");

    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const numNetworks = parseInt(getRequiredEnvVar("NUM_NETWORKS"));
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    // Parse the L1 network
    const layer1Networks = [];
    for (let idx = 0; idx < numNetworks; idx++) {
      layer1Networks.push({
        chainId: getRequiredEnvVar(`NETWORK_CHAIN_ID_${idx}`),
        httpzExecutor: getRequiredEnvVar(`NETWORK_HTTPZ_EXECUTOR_${idx}`),
        aclAddress: getRequiredEnvVar(`NETWORK_ACL_ADDRESS_${idx}`),
        name: getRequiredEnvVar(`NETWORK_NAME_${idx}`),
        website: getRequiredEnvVar(`NETWORK_WEBSITE_${idx}`),
      });
    }

    let proxyAddress: string;
    if (taskArgs.useInternalHttpzAddress) {
      const parsedEnvHttpz = dotenv.parse(fs.readFileSync("addresses/.env.httpz"));
      proxyAddress = parsedEnvHttpz.HTTPZ_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("HTTPZ_ADDRESS");
    }

    // Add L1 networks
    const httpz = await hre.ethers.getContractAt("HTTPZ", proxyAddress, deployer);
    for (const network of layer1Networks) {
      await httpz.addNetwork(network);
    }

    console.log("In HTTPZ contract:", proxyAddress, "\n");
    console.log("Added L1 networks:", layer1Networks, "\n");
    console.log("Networks registration done!");
  });
