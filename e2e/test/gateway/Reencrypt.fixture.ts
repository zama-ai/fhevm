import { ethers } from "hardhat";

import type { Reencrypt } from "../../types";
import { getSigners } from "../signers";

export async function deployReencryptFixture(): Promise<Reencrypt> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory("Reencrypt");
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();

  return contract;
}
