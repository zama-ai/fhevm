import hre from "hardhat";

// Define the maximum value for a uint64
export const UINT64_MAX = (BigInt(1) << BigInt(64)) - BigInt(1);

// Create a byte input of a given length
export function createByteInput(length: number = 32): string {
  return hre.ethers.hexlify(hre.ethers.randomBytes(length));
}

// Create a bytes32
export function createBytes32(): string {
  return createByteInput(32);
}

// Create a ctHandle (bytes32)
export function createCtHandle(): string {
  return createBytes32();
}

// Create a list of ctHandles (bytes32[])
export function createCtHandles(length: number): string[] {
  return Array.from({ length }, () => createCtHandle());
}

// Create a ctHandle (bytes32) with a given chainId (uint64)
export function createCtHandleWithChainId(chainId: number): string {
  if (chainId < 0 || chainId > UINT64_MAX) {
    throw new Error("chainId must be a valid uint64");
  }

  const ctHandle = hre.ethers.randomBytes(32);

  // Convert chainId to 8 bytes
  const chainIdBytes = hre.ethers.getBytes(hre.ethers.zeroPadValue(hre.ethers.toBeHex(chainId), 8));

  // Replace bytes 22 to 29 (8 bytes) with the chainId bytes
  for (let i = 0; i < 8; i++) {
    ctHandle[22 + i] = chainIdBytes[i];
  }

  return hre.ethers.hexlify(ctHandle);
}

// Create a list of ctHandles (bytes32[])
export function createCtHandlesWithChainId(length: number, chainId: number): string[] {
  return Array.from({ length }, () => createCtHandleWithChainId(chainId));
}
