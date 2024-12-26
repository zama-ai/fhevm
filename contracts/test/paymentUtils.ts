import dotenv from 'dotenv';
import fs from 'fs';
import { ethers } from 'hardhat';

export async function initializeFHEGasLimit() {
  const fheGasLimitFactory = await ethers.getContractFactory('FHEGasLimit');
  const parsedFHEGasLimit = dotenv.parse(fs.readFileSync('addresses/.env.fhegaslimit'));
  const fheGasLimit = fheGasLimitFactory.attach(parsedFHEGasLimit.FHE_GASLIMIT_CONTRACT_ADDRESS);
  return fheGasLimit;
}

export const FHE_GASPRICE_NATIVE_RATIO = 0n; // 1000n;
export const MIN_FHE_GASPRICE = 0n; // 10_000_000n;
