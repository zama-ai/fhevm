import dotenv from "dotenv";
import fs from "fs";
import path from "path";

import { ADDRESSES_DIR, GATEWAY_ADDRESSES_ENV_FILE_NAME } from "../../hardhat.config";
import { pascalCaseToAddressEnvVar } from "../utils";

// Get the required environment variable, throw an error if it's not set
// We only check if the variable is set, not if it's empty
export function getRequiredEnvVar(name: string): string {
  if (!(name in process.env)) {
    throw new Error(`"${name}" env variable is not set`);
  }
  return process.env[name]!;
}

// Load the addresses as environment variables from the env file
export function loadAddressEnvVarsFromFile(fileName: string) {
  const envFilePath = path.join(ADDRESSES_DIR, fileName);

  if (!fs.existsSync(envFilePath)) {
    throw new Error(`Environment file for addresses not found: ${envFilePath}`);
  }

  dotenv.config({ path: envFilePath, override: true });
}

export function loadGatewayAddresses() {
  loadAddressEnvVarsFromFile(GATEWAY_ADDRESSES_ENV_FILE_NAME);
}
