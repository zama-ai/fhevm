import dotenv from 'dotenv';
import fs from 'fs';
import { ethers } from 'hardhat';

export async function initializeFHEPayment() {
  const fhePaymentFactory = await ethers.getContractFactory('fhevmTemp/contracts/FHEPayment.sol:FHEPayment');
  const parsedFHEPayment = dotenv.parse(fs.readFileSync('node_modules/fhevm-core-contracts/addresses/.env.fhepayment'));
  const fhePayment = fhePaymentFactory.attach(parsedFHEPayment.FHE_PAYMENT_CONTRACT_ADDRESS);
  return fhePayment;
}

export const FHE_GASPRICE_NATIVE_RATIO = 0n; // 1000n;
export const MIN_FHE_GASPRICE = 0n; // 10_000_000n;
