import dotenv from 'dotenv';
import fs from 'fs';
import { ethers } from 'hardhat';

export async function initializeHCULimit() {
  const HCULimitFactory = await ethers.getContractFactory('HCULimit');
  const parsedHCULimit = dotenv.parse(fs.readFileSync('addresses/.env.host'));
  const HCULimit = HCULimitFactory.attach(parsedHCULimit.HCU_LIMIT_CONTRACT_ADDRESS);
  return HCULimit;
}
