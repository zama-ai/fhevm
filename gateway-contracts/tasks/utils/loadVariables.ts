import dotenv from "dotenv";
import fs from "fs";
import path from "path";

import { ADDRESSES_DIR, GATEWAY_ADDRESSES_ENV_FILE_NAME } from "../../hardhat.config";
import { pascalCaseToAddressEnvVar } from "../utils";

// Get the required environment variable, throw an error if it's not set or empty
export function getRequiredEnvVar(name: string): string {
  if (!(name in process.env)) {
    throw new Error(`"${name}" env variable is not set`);
  }
  const value = process.env[name]!;
  if (value.trim() === "") {
    throw new Error(`"${name}" env variable is set but empty`);
  }
  return value;
}

// Get a required positive-integer environment variable. Throws if it is missing, empty, non-integer, or
// not strictly positive, so a malformed count fails loudly instead of yielding NaN and silently no-op'ing.
export function getRequiredCountEnvVar(name: string): number {
  const value = getRequiredEnvVar(name);
  const count = Number(value);
  if (!Number.isInteger(count) || count <= 0) {
    throw new Error(`"${name}" env variable must be a positive integer, got "${value}"`);
  }
  return count;
}

// Get the required address from the environment variable, throw an error if it's not set or empty
export function getRequiredAddressEnvVar(name: string): string {
  const addressEnvVarName = pascalCaseToAddressEnvVar(name);
  return getRequiredEnvVar(addressEnvVarName);
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
