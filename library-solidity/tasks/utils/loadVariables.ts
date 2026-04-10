import dotenv from 'dotenv';
import fs from 'fs';
import { HardhatRuntimeEnvironment } from 'hardhat/types';
import path from 'path';

const ADDRESSES_DIR = path.resolve(__dirname, '../../fhevmTemp/addresses');
const HOST_ADDRESSES_ENV_FILE_NAME = '.env.host';

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

export async function getPauserSetContract(useInternalProxyAddress: boolean, hre: HardhatRuntimeEnvironment) {
  await hre.run('compile:specific', { contract: 'fhevmTemp/contracts/contracts/immutable' });

  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  if (useInternalProxyAddress) {
    loadHostAddresses();
  }
  const pauserSetAddress = getRequiredEnvVar('PAUSER_SET_CONTRACT_ADDRESS');

  return hre.ethers.getContractAt('PauserSet', pauserSetAddress, deployer);
}
