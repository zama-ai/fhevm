import dotenv from "dotenv";
import fs from "fs";
import { task } from "hardhat/config";

import { getRequiredEnvVar } from "./utils/loadVariables";

// Add L1 networks metadata to the HTTPZ contract
task("task:addNetworksToHttpz").setAction(async function (_, { ethers }) {
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const numNetworks = parseInt(getRequiredEnvVar("NUM_NETWORKS"));
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);

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

  const parsedEnvHttpz = dotenv.parse(fs.readFileSync("addresses/.env.httpz"));
  const proxyAddress = parsedEnvHttpz.HTTPZ_ADDRESS;

  // Add L1 networks
  const httpz = await ethers.getContractAt("HTTPZ", proxyAddress, deployer);
  for (const network of layer1Networks) {
    await httpz.addNetwork(network);
  }

  console.log("In HTTPZ contract:", proxyAddress, "\n");
  console.log("Added L1 networks:", layer1Networks, "\n");
});
