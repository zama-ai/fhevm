import { ethers } from "hardhat";

// Create a new random address
export function createRandomAddress() {
  return ethers.Wallet.createRandom().address;
}

// Create a list of random addresses
export function createRandomAddresses(length: number) {
  return Array.from({ length }, () => createRandomAddress());
}
