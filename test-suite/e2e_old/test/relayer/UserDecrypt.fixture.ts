import { ethers } from "hardhat";

import type { UserDecrypt } from "../../types";
import { getSigners } from "../signers";

export async function deployUserDecryptFixture(): Promise<UserDecrypt> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory("UserDecrypt");
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();

  return contract;
}
