import dotenv from 'dotenv';
import fs from 'fs';
import path from 'path';

const ADDRESSES_DIR = path.resolve(__dirname, '../../fhevmTemp/addresses');
const HOST_ADDRESSES_ENV_FILE_NAME = '.env.host';

// Get the required environment variable, throw an error if it's not set or empty
export function getRequiredEnvVar(name: string): string {
  const value = process.env[name];
  if (value === undefined) {
    throw new Error(`"${name}" env variable is not set`);
  }
  if (value.trim() === '') {
    throw new Error(`"${name}" env variable is set but empty`);
  }
  return value;
}

// Load the addresses as environment variables from the env file
export function loadAddressEnvVarsFromFile(fileName: string): void {
  const envFilePath = path.join(ADDRESSES_DIR, fileName);

  if (!fs.existsSync(envFilePath)) {
    throw new Error(`Environment file for addresses not found: ${envFilePath}`);
  }

  dotenv.config({ path: envFilePath, override: true });
}

export function loadHostAddresses(): void {
  loadAddressEnvVarsFromFile(HOST_ADDRESSES_ENV_FILE_NAME);
}
