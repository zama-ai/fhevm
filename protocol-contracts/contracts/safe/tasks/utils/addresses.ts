import { HardhatEthersHelpers } from "hardhat/types";
import { getRequiredEnvVar } from "./loadVariables";

export async function getSafeProxyAddress(ethers: HardhatEthersHelpers) {
  const safeProxyAddress = getRequiredEnvVar("SAFE_PROXY_ADDRESS");
  const safeProxy = await ethers.getContractAt("SafeL2", safeProxyAddress);
  return { safeProxy, safeProxyAddress };
}
