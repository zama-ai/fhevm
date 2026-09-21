import { task, types } from "hardhat/config";
import { getRequiredEnvVar, getDeployedAddress } from "./utils/loadVariables";
import { execTransaction } from "./utils/execTransaction";

// Enable the AdminModule in the Safe
// Example usage:
// npx hardhat task:enableAdminModule --network <NETWORK>
// To target a Safe created outside hardhat-deploy, read its address from the
// SAFE_PROXY_ADDRESS env variable instead of the deployments/<network>/ artifact:
// npx hardhat task:enableAdminModule --use-safe-proxy-address-env --network <NETWORK>
task("task:enableAdminModule")
  .addFlag(
    "useSafeProxyAddressEnv",
    "Read the Safe proxy address from the SAFE_PROXY_ADDRESS env variable instead of the deployments/<network>/ artifact",
  )
  .setAction(async function (
    { useSafeProxyAddressEnv },
    { ethers, network, getNamedAccounts },
  ) {
    const adminModuleAddress = await getDeployedAddress(
      network.name,
      "AdminModule",
    );
    const safeProxyAddress =
      useSafeProxyAddressEnv || network.name === "hardhat"
        ? getRequiredEnvVar("SAFE_PROXY_ADDRESS")
        : await getDeployedAddress(network.name, "SafeL2Proxy");
    const safeProxyContract = await ethers.getContractAt(
      "SafeL2",
      safeProxyAddress,
    );

    const enableModuleData = safeProxyContract.interface.encodeFunctionData(
      "enableModule",
      [adminModuleAddress],
    );

    const { deployer } = await getNamedAccounts();

    const signer = await ethers.getSigner(deployer);
    await execTransaction(
      [signer],
      safeProxyContract,
      safeProxyContract.target,
      0,
      enableModuleData,
      0,
    );

    console.log(
      "AdminModule was successfully enabled in the Safe : please double check by calling isModuleEnabled(ADMIN_MODULE_ADDRESS) on the SafeProxy",
    );
  });

task("task:verifyAdminModuleEnabled")
  .addOptionalParam(
    "module",
    "The address of the AdminModule contract to verify",
    undefined,
    types.string,
  )
  .addFlag(
    "useSafeProxyAddressEnv",
    "Read the Safe proxy address from the SAFE_PROXY_ADDRESS env variable instead of the deployments/<network>/ artifact",
  )
  .setAction(async function (
    { useSafeProxyAddressEnv, module },
    { ethers, network },
  ) {
    const adminModuleAddress =
      module || (await getDeployedAddress(network.name, "AdminModule"));
    const safeProxyAddress =
      useSafeProxyAddressEnv || network.name === "hardhat"
        ? getRequiredEnvVar("SAFE_PROXY_ADDRESS")
        : await getDeployedAddress(network.name, "SafeL2Proxy");
    const safeProxyContract = await ethers.getContractAt(
      "SafeL2",
      safeProxyAddress,
    );
    const isEnabled =
      await safeProxyContract.isModuleEnabled(adminModuleAddress);
    if (isEnabled) {
      console.log(
        `✅ AdminModule (${adminModuleAddress}) is enabled in Safe proxy (${safeProxyAddress})`,
      );
    } else {
      console.log(
        `❌ AdminModule (${adminModuleAddress}) is NOT enabled in Safe proxy (${safeProxyAddress})`,
      );
      process.exitCode = 1;
    }
  });
