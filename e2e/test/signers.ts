import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { ethers } from "hardhat";

export interface Signers {
  alice: HardhatEthersSigner;
  bob: HardhatEthersSigner;
  carol: HardhatEthersSigner;
  dave: HardhatEthersSigner;
  eve: HardhatEthersSigner;
}

let signers: Signers;

export const initSigners = async (): Promise<void> => {
  if (!signers) {
    const eSigners = await ethers.getSigners();
    signers = {
      alice: eSigners[0],
      bob: eSigners[1],
      carol: eSigners[2],
      dave: eSigners[3],
      eve: eSigners[4],
    };
  }
};

export const getSigners = async (): Promise<Signers> => {
  return signers;
};
