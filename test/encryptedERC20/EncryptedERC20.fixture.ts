import { ethers } from "hardhat";

import type { EncryptedERC20 } from "../../types/contracts/EncryptedERC20";

export async function deployEncryptedERC20Fixture(): Promise<EncryptedERC20> {
  const signers = await ethers.getSigners();
  const admin = signers[0];

  const contractFactory = await ethers.getContractFactory("EncryptedERC20");
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}
