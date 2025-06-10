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

// Create a list of random wallets
export function createRandomWallets(length: number) {
  return Array.from({ length }, () => createRandomWallet());
}

// Create a new random wallet with some funds
export async function createAndFundRandomWallet() {
  const wallet = createRandomWallet();
  await fund(wallet.address);
  return wallet;
}

// Create a list of random wallets with some funds
export async function createAndFundRandomWallets(length: number) {
  return await Promise.all(Array.from({ length }, () => createAndFundRandomWallet()));
}
