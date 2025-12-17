import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { ethers } from "hardhat";

const ACCOUNT_NAMES = ["deployer", "admin", "regulator", "royalties", "alice", "bob", "charlie", "delta"] as const;

type AccountNames = (typeof ACCOUNT_NAMES)[number];

export type Signers = {
  [K in AccountNames]: HardhatEthersSigner;
};

export const getSigners = async (): Promise<Signers> => {
  const signers: Signers = {} as Signers;

  const ethSigners = await ethers.getSigners();
  for (let index = 0; index < ACCOUNT_NAMES.length; index++) {
    const name = ACCOUNT_NAMES[index] as AccountNames;
    signers[name] = ethSigners[index];
  }

  return signers;
};
