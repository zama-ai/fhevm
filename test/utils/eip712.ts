import { ethers } from "hardhat";

export interface EIP712Type {
  name: string;
  type: string;
}

export interface EIP712 {
  domain: {
    chainId: number;
    name: string;
    verifyingContract: string;
    version: string;
  };
  message: {
    [key: string]: string | string[] | number | number[];
  };
  primaryType: string;
  types: {
    [key: string]: EIP712Type[];
  };
}

export function createEIP712ResponseMessage(chainId: number, verifyingContract: string): EIP712 {
  if (!ethers.isAddress(verifyingContract)) {
    throw new Error("Invalid verifying contract address.");
  }
  return {
    types: {
      EIP712Domain: [
        { name: "name", type: "string" },
        { name: "version", type: "string" },
        { name: "chainId", type: "uint256" },
        { name: "verifyingContract", type: "address" },
      ],
      EIP712ResponseMessage: [
        { name: "handles", type: "bytes32[]" },
        { name: "userAddress", type: "address" },
        { name: "contractAddress", type: "address" },
        { name: "contractChainId", type: "uint256" },
      ],
    },
    primaryType: "EIP712ResponseMessage",
    domain: {
      name: "ZKPoK verification",
      version: "1",
      chainId,
      verifyingContract,
    },
    message: {
      handles: ["0x3132333435363738390000000000000000000000000000000000000000000000"],
      userAddress: "0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97",
      contractAddress: "0x68d30f47F19c07bCCEf4Ac7FAE2Dc12FCa3e0dC9",
      contractChainId: "11155111",
    },
  };
}
