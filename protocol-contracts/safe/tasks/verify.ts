import { task } from "hardhat/config";

import { getRequiredEnvVar } from "./utils/loadVariables";

task("task:verifyAdminModule")
  .addParam(
    "adminModule",
    "address of the already deployed AdminModule contract that should be verified",
  )
  .setAction(async function ({ adminModule }, { run }) {
    const adminAddress = getRequiredEnvVar("ADMIN_ADDRESS");
    const safeProxyAddress = getRequiredEnvVar("SAFE_ADDRESS");

    await run("verify:verify", {
      address: adminModule,
      constructorArguments: [adminAddress, safeProxyAddress],
    });
  });
