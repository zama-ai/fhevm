import dotenv from "dotenv";
import { createInstance as createFhevmInstance } from "fhevmjs/node";
import * as fs from "fs";
import { network } from "hardhat";
import { NetworkConfig } from "hardhat/types";

const parsedEnv = dotenv.parse(fs.readFileSync(".env"));

export const createInstance = async () => {
  const instance = await createFhevmInstance({
    networkUrl: (network.config as NetworkConfig & { url: string }).url,
    gatewayUrl: parsedEnv.GATEWAY_URL,
    aclContractAddress: parsedEnv.ACL_CONTRACT_ADDRESS,
    kmsContractAddress: parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS,
  });
  return instance;
};
