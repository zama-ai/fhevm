import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { HDNodeWallet, Wallet } from "ethers";
import { ethers } from "hardhat";

import { EIP712, getSignaturesEIP712 } from "./interface";

// Create an EIP712 message for a ZKPoK response
export function createEIP712ResponseZKPoK(
  gatewayChainId: number,
  verifyingContract: string,
  ctHandles: string[],
  userAddress: string,
  contractAddress: string,
  contractChainId: number,
  extraData: string,
): EIP712 {
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
      CiphertextVerification: [
        { name: "ctHandles", type: "bytes32[]" },
        { name: "userAddress", type: "address" },
        { name: "contractAddress", type: "address" },
        { name: "contractChainId", type: "uint256" },
        { name: "extraData", type: "bytes" },
      ],
    },
    primaryType: "CiphertextVerification",
    domain: {
      name: "InputVerification",
      version: "1",
      chainId: gatewayChainId,
      verifyingContract,
    },
    message: {
      ctHandles,
      userAddress,
      contractAddress,
      contractChainId,
      extraData,
    },
  };
}

// Get signatures from signers using the EIP712 message response for proof verification
export async function getSignaturesZKPoK(
  eip712: EIP712,
  signers: (HardhatEthersSigner | HDNodeWallet | Wallet)[],
): Promise<string[]> {
  return getSignaturesEIP712(eip712, signers);
}
