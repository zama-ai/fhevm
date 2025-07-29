import hre from "hardhat";

const DEFAULT_BALANCE = "0x1000000000000000000000000000000000000000";

// Add fund to the given address
export async function fund(address: string, balance: string = DEFAULT_BALANCE) {
  await hre.ethers.provider.send("hardhat_setBalance", [address, balance]);
}

// Create a new random address
export function createRandomAddress() {
  return hre.ethers.Wallet.createRandom().address;
}

// Create a list of random addresses
export function createRandomAddresses(length: number) {
  return Array.from({ length }, () => createRandomAddress());
}

// Create a new random wallet
export function createRandomWallet() {
  return hre.ethers.Wallet.createRandom().connect(hre.ethers.provider);
}

// Create a new random wallet with some funds
export async function createAndFundRandomWallet() {
  const user = createRandomWallet();
  await fund(user.address);
  return user;
}
