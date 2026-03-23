import dotenv from 'dotenv';
import fs from 'fs';
import path from 'path';

const HOST_ADDRESSES_DIR = path.resolve(__dirname, '../../addresses');
const HOST_ENV_FILE = path.join(HOST_ADDRESSES_DIR, '.env.host');
const HOST_ADDRESSES_FILE = path.join(HOST_ADDRESSES_DIR, 'FHEVMHostAddresses.sol');

// Get the required environment variable, throw an error if it's not set
// We only check if the variable is set, not if it's empty
export function getRequiredEnvVar(name: string): string {
  if (!(name in process.env)) {
    throw new Error(`"${name}" env variable is not set`);
  }
  return process.env[name]!;
}

export function loadInternalHostAddressesEnv() {
  if (!fs.existsSync(HOST_ENV_FILE) || !fs.existsSync(HOST_ADDRESSES_FILE)) {
    throw new Error(
      'Missing generated host addresses under host-contracts/addresses. '
        + 'These files are required because host contracts import addresses/FHEVMHostAddresses.sol. '
        + 'Generate them by running task:setACLAddress first, then task:setFHEVMExecutorAddress, '
        + 'task:setKMSVerifierAddress, task:setInputVerifierAddress, task:setHCULimitAddress, '
        + 'and task:setPauserSetAddress.',
    );
  }

  dotenv.config({ path: HOST_ENV_FILE, override: true });
}
