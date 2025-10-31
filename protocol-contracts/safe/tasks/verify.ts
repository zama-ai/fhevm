import { task } from "hardhat/config";

import { getRequiredEnvVar, getDeployedAddress } from "./utils/loadVariables";

// Verify the SafeL2 contract
// Example usage:
// npx hardhat task:verifySafe --network gateway-testnet
task("task:verifySafe").setAction(async function (_, { run, network }) {
  const safeL2Address = await getDeployedAddress(network.name, "SafeL2");
  await run("verify:verify", {
    address: safeL2Address,
    constructorArguments: [],
  });
  const safeProxyFactoryAddress = await getDeployedAddress(
    network.name,
    "SafeProxyFactory",
  );
  await run("verify:verify", {
    address: safeProxyFactoryAddress,
    constructorArguments: [],
  });
  const safeL2ProxyAddress = await getDeployedAddress(
    network.name,
    "SafeL2Proxy",
  );
  await run("verify:verify", {
    address: safeL2ProxyAddress,
    constructorArguments: [safeL2Address],
  });
});

// Verify the AdminModule contract
// Example usage:
// npx hardhat task:verifyAdminModule --network gateway-testnet
task("task:verifyAdminModule").setAction(async function (_, { run, network }) {
  const safeProxyAddress = await getDeployedAddress(
    network.name,
    "SafeL2Proxy",
  );
  const adminModuleAddress = await getDeployedAddress(
    network.name,
    "AdminModule",
  );
  const adminAddress = getRequiredEnvVar("ADMIN_ADDRESS");
  await run("verify:verify", {
    address: adminModuleAddress,
    constructorArguments: [adminAddress, safeProxyAddress],
  });
});

// Verify the MultiSend contract
// Example usage:
// npx hardhat task:verifyMultiSend \
// --multiSendAddress \
// "0x123456789012345678901234567890124567890"
task("task:verifyMultiSend")
  .addParam(
    "multiSendAddress",
    "address of the already deployed MultiSend contract that should be verified",
  )
  .setAction(async function ({ multiSendAddress }, { run }) {
    await run("verify:verify", {
      address: multiSendAddress,
      constructorArguments: [],
    });
  });
