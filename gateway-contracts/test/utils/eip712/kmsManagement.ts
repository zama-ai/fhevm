import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { BigNumberish, BytesLike, HDNodeWallet, Wallet } from "ethers";
import { ethers } from "hardhat";

import { EIP712, getSignaturesEIP712 } from "./interface";

// Create an EIP712 message for a preprocessing keygen response
export function createEIP712ResponsePrepKeygen(
  gatewayChainId: number,
  verifyingContract: string,
  prepKeygenId: BigInt,
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
      PrepKeygenVerification: [{ name: "prepKeygenId", type: "uint256" }],
    },
    primaryType: "PrepKeygenVerification",
    domain: {
      name: "KmsManagement",
      version: "1",
      chainId: gatewayChainId,
      verifyingContract,
    },
    message: {
      prepKeygenId,
    },
  };
}

// Get signatures from signers using the EIP712 message response for preprocessing keygen
export async function getSignaturesPrepKeygen(
  eip712: EIP712,
  signers: (HardhatEthersSigner | HDNodeWallet | Wallet)[],
): Promise<string[]> {
  return getSignaturesEIP712(eip712, "PrepKeygenVerification", signers);
}

// Create an EIP712 message for a keygen response
export function createEIP712ResponseKeygen(
  gatewayChainId: number,
  verifyingContract: string,
  prepKeygenId: BigInt,
  keyId: BigInt,
  keyDigests: { keyType: BigNumberish; digest: BytesLike }[],
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
      KeygenVerification: [
        { name: "prepKeygenId", type: "uint256" },
        { name: "keyId", type: "uint256" },
        { name: "keyDigests", type: "(uint8,bytes)[]" },
      ],
    },
    primaryType: "KeygenVerification",
    domain: {
      name: "KmsManagement",
      version: "1",
      chainId: gatewayChainId,
      verifyingContract,
    },
    message: {
      prepKeygenId,
      keyId,
      keyDigests,
    },
  };
}

// Get signatures from signers using the EIP712 message response for keygen
export async function getSignaturesKeygen(
  eip712: EIP712,
  signers: (HardhatEthersSigner | HDNodeWallet | Wallet)[],
): Promise<string[]> {
  return getSignaturesEIP712(eip712, "KeygenVerification", signers);
}

// Create an EIP712 message for a crsgen response
export function createEIP712ResponseCrsgen(
  gatewayChainId: number,
  verifyingContract: string,
  crsId: BigInt,
  maxBitLength: BigInt,
  crsDigest: string,
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
      CrsgenVerification: [
        { name: "crsId", type: "uint256" },
        { name: "maxBitLength", type: "uint256" },
        { name: "crsDigest", type: "bytes" },
      ],
    },
    primaryType: "CrsgenVerification",
    domain: {
      name: "KmsManagement",
      version: "1",
      chainId: gatewayChainId,
      verifyingContract,
    },
    message: {
      crsId,
      maxBitLength,
      crsDigest,
    },
  };
}

// Get signatures from signers using the EIP712 message response for crsgen
export async function getSignaturesCrsgen(
  eip712: EIP712,
  signers: (HardhatEthersSigner | HDNodeWallet | Wallet)[],
): Promise<string[]> {
  return getSignaturesEIP712(eip712, "CrsgenVerification", signers);
}
