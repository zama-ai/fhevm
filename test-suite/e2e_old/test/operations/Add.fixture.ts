import { ethers } from "hardhat";

import type { Add } from "../../types";
import { getSigners } from "../signers";

export async function deployAddFixture(): Promise<Add> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory("Add");
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();

  return contract;
}
