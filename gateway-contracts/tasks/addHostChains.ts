import dotenv from "dotenv";
import { task, types } from "hardhat/config";
import path from "path";

import { ADDRESSES_DIR } from "../hardhat.config";
import { getRequiredEnvVar } from "./utils/loadVariables";

// Add host chains metadata to the GatewayConfig contract
// Note: Internal GatewayConfig address is defined in the `addresses/` directory. It should be used
// for local testing. By default, we use the GATEWAY_CONFIG_ADDRESS env var, as done in deployment
task("task:addHostChainsToGatewayConfig")
  .addParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
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

    if (useInternalProxyAddress) {
      dotenv.config({ path: path.join(ADDRESSES_DIR, ".env.gateway"), override: true });
    }
    const proxyAddress = getRequiredEnvVar("GATEWAY_CONFIG_ADDRESS");

    console.log("In GatewayConfig contract:", proxyAddress, "\n");

    // Add host chains
    const gatewayConfig = await hre.ethers.getContractAt("GatewayConfig", proxyAddress, deployer);
    for (const hostChain of hostChains) {
      console.log("Adding host chain: ", hostChain);
      const tx = await gatewayConfig.addHostChain(hostChain);

      // Wait for confirmation before adding next host chain
      await tx.wait();
      console.log("Host chain added !\n");
    }

    console.log("Host chains registration done!");
  });
