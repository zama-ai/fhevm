import { task } from "hardhat/config";

import { getRequiredEnvVar } from "./utils/loadVariables";

// Verify the SafeL2 contract
// Example usage:
// npx hardhat task:verifySafeL2 \
// --safeL2Address \
// "0x1234567890123456789012345678901234567890"
task("task:verifySafeL2")
  .addParam(
    "safeL2Address",
    "address of the already deployed SafeL2 contract that should be verified",
  )
  .setAction(async function ({ safeL2Address }, { run }) {
    await run("verify:verify", {
      address: safeL2Address,
      constructorArguments: [],
    });
  });

// Verify the AdminModule contract
// Example usage:
// npx hardhat task:verifyAdminModule \
// --adminModuleAddress \
// "0x1234567890123456789012345678901234567890"
task("task:verifyAdminModule")
  .addParam(
    "adminModuleAddress",
    "address of the already deployed AdminModule contract that should be verified",
  )
  .setAction(async function ({ adminModuleAddress }, { run }) {
    const adminAddress = getRequiredEnvVar("ADMIN_ADDRESS");
    const safeProxyAddress = getRequiredEnvVar("SAFE_ADDRESS");

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
