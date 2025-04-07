import hre from "hardhat";

const DEFAULT_BALANCE = "0x1000000000000000000000000000000000000000";

// Add fund to the given address
export async function fund(address: string, balance: string = DEFAULT_BALANCE) {
  await hre.ethers.provider.send("hardhat_setBalance", [address, balance]);
}

// Create a new random user with some funds
export async function createAndFundRandomUser() {
  const user = hre.ethers.Wallet.createRandom().connect(hre.ethers.provider);
  await fund(user.address, DEFAULT_BALANCE);
  return user;
}
