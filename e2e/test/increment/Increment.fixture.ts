import { ethers } from "hardhat";

import type { Increment } from "../../types";
import { getSigners } from "../signers";

export async function deployIncrementFixture(): Promise<Increment> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory("Increment");
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();

  return contract;
}
