import { ethers } from 'hardhat';

export function createRandomAddress() {
  return ethers.getAddress(ethers.hexlify(ethers.randomBytes(20)));
}
