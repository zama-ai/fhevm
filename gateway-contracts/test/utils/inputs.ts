import hre from "hardhat";

// Define the maximum value for a uint64
export const UINT64_MAX = (BigInt(1) << BigInt(64)) - BigInt(1);

// Create a byte input of a given length
export function createByteInput(length: number = 64): string {
  return hre.ethers.hexlify(hre.ethers.randomBytes(length));
}

// Create a bytes32
export function createBytes32(): string {
  return createByteInput(32);
}

// Create a list of bytes32
export function createBytes32s(length: number): string[] {
  return Array.from({ length }, () => createBytes32());
}

// Create a ctHandle (bytes32) with a given chain ID (uint64) and FHE type (uint8)
// A ctHandle has the following format:
// [21 first random bytes from hashing] | index_21 | chainID_22...29 | fheType_30 | version_31
export function createCtHandle(chainId: number = 0, fheType: number = 0): string {
  if (chainId < 0 || chainId > UINT64_MAX) {
    throw new Error("chainId must be a valid uint64");
  }
  if (fheType < 0 || fheType > 255) {
    throw new Error("fheType must be a valid uint8");
  }

  const ctHandle = hre.ethers.randomBytes(32);

  // Convert chainId to 8 bytes
  const chainIdBytes = hre.ethers.getBytes(hre.ethers.zeroPadValue(hre.ethers.toBeHex(chainId), 8));

  // Replace bytes 22 to 29 (8 bytes) with the chainId bytes
  for (let i = 0; i < 8; i++) {
    ctHandle[22 + i] = chainIdBytes[i];
  }

  // Replace byte 30 with the fheType (single byte)
  ctHandle[30] = fheType;

  return hre.ethers.hexlify(ctHandle);
}

// Create a list of ctHandles (bytes32[])
export function createCtHandles(length: number, chainId: number = 0, fheType: number = 0): string[] {
  return Array.from({ length }, () => createCtHandle(chainId, fheType));
}

// Defined in IKmsManagement.sol
export enum ParamsTypeEnum {
  Default = 0,
  Test = 1,
}

export enum KeyTypeEnum {
  Server = 0,
  Public = 1,
}
