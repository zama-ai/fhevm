import { task } from "hardhat/config";
import { getDeployedAddress } from "./utils/loadVariables";
import { execTransaction } from "./utils/execTransaction";

task("task:enableAdminModule").setAction(async function (
  _,
  { ethers, network, getNamedAccounts },
) {
  const adminModuleAddress = await getDeployedAddress(
    network.name,
    "AdminModule",
  );
  const safeProxyAddress = await getDeployedAddress(
    network.name,
    "SafeL2Proxy",
  );
  const safeProxy = await ethers.getContractAt("SafeL2", safeProxyAddress);

  const enableModuleData = safeProxy.interface.encodeFunctionData(
    "enableModule",
    [adminModuleAddress],
  );

  const { deployer } = await getNamedAccounts();

  const signer = await ethers.getSigner(deployer);
  await execTransaction(
    [signer],
    safeProxy,
    safeProxy.target,
    0,
    enableModuleData,
    0,
  );

  console.log(
    "AdminModule was successfully enabled in the Safe : please double check by calling isEnabledModule(ADMIN_MODULE_ADDRESS) on the SafeProxy",
  );
});
