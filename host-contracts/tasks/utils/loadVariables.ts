import dotenv from 'dotenv';
import fs from 'fs';
import { HardhatRuntimeEnvironment } from 'hardhat/types';
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

export async function getPauserSetContract(useInternalProxyAddress: boolean, hre: HardhatRuntimeEnvironment) {
  await hre.run('compile:specific', { contract: 'contracts/immutable' });

  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  if (useInternalProxyAddress) {
    loadHostAddresses();
  }
  const pauserSetAddress = getRequiredEnvVar('PAUSER_SET_CONTRACT_ADDRESS');

  return hre.ethers.getContractAt('PauserSet', pauserSetAddress, deployer);
}
