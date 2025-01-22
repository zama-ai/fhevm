import { ethers } from "hardhat";

import type { Div } from "../../types";
import { getSigners } from "../signers";

export async function deployDivFixture(): Promise<Div> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory("Div");
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();

  return contract;
}
