import dotenv from "dotenv";
import fs from "fs";
import { task, types } from "hardhat/config";

import { getRequiredEnvVar } from "./utils/loadVariables";

// Add host chains metadata to the GatewayConfig contract
// Note: Internal GatewayConfig address is defined in the `addresses/` directory. It should be used
// for local testing. By default, we use the GATEWAY_CONFIG_ADDRESS env var, as done in deployment
task("task:addHostChainsToGatewayConfig")
  .addParam("useInternalGatewayConfigAddress", "If internal GatewayConfig address should be used", false, types.boolean)
  .setAction(async function (taskArgs, hre) {
    await hre.run("compile:specific", { contract: "contracts" });
    console.log("Register host chains to GatewayConfig contract");

    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const numHostChains = parseInt(getRequiredEnvVar("NUM_HOST_CHAINS"));
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    // Parse the host chain(s)
    const hostChains = [];
    for (let idx = 0; idx < numHostChains; idx++) {
      hostChains.push({
        chainId: getRequiredEnvVar(`HOST_CHAIN_CHAIN_ID_${idx}`),
        fhevmExecutorAddress: getRequiredEnvVar(`HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_${idx}`),
        aclAddress: getRequiredEnvVar(`HOST_CHAIN_ACL_ADDRESS_${idx}`),
        name: getRequiredEnvVar(`HOST_CHAIN_NAME_${idx}`),
        website: getRequiredEnvVar(`HOST_CHAIN_WEBSITE_${idx}`),
      });
    }

    let proxyAddress: string;
    if (taskArgs.useInternalGatewayConfigAddress) {
      const parsedEnvGatewayConfig = dotenv.parse(fs.readFileSync("addresses/.env.gateway_config"));
      proxyAddress = parsedEnvGatewayConfig.GATEWAY_CONFIG_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar("GATEWAY_CONFIG_ADDRESS");
    }

    // Add host chains
    const gatewayConfig = await hre.ethers.getContractAt("GatewayConfig", proxyAddress, deployer);
    for (const hostChain of hostChains) {
      await gatewayConfig.addHostChain(hostChain);
    }

    console.log("In GatewayConfig contract:", proxyAddress, "\n");
    console.log("Added host chains:", hostChains, "\n");
    console.log("Host chains registration done!");
  });
