import dotenv from 'dotenv';
import fs from 'fs';
import path from 'path';

import { ADDRESSES_DIR, HOST_ADDRESSES_ENV_FILE_NAME } from '../../hardhat.config';

// Get the required environment variable, throw an error if it's not set or empty
export function getRequiredEnvVar(name: string): string {
  if (!(name in process.env)) {
    throw new Error(`"${name}" env variable is not set`);
  }
  const value = process.env[name]!;
  if (value.trim() === '') {
    throw new Error(`"${name}" env variable is set but empty`);
  }
  return value;
}

// Load the addresses as environment variables from the env file
export function loadAddressEnvVarsFromFile(fileName: string) {
  const envFilePath = path.join(ADDRESSES_DIR, fileName);

  if (!fs.existsSync(envFilePath)) {
    throw new Error(`Environment file for addresses not found: ${envFilePath}`);
  }

  dotenv.config({ path: envFilePath, override: true });
}

export function loadHostAddresses() {
  loadAddressEnvVarsFromFile(HOST_ADDRESSES_ENV_FILE_NAME);
}
