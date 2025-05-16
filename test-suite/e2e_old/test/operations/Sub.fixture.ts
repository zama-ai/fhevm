import { ethers } from "hardhat";

import type { Sub } from "../../types";
import { getSigners } from "../signers";

export async function deploySubFixture(): Promise<Sub> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory("Sub");
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();

  return contract;
}
