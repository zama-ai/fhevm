import hre from "hardhat";

// Create a byte input of a given length
export function createByteInput(length: number = 32): string {
  return hre.ethers.hexlify(hre.ethers.randomBytes(length));
}

// Create a bytes32
export function createBytes32() {
  return createByteInput(32);
}

// Create a ctHandle (bytes32)
export function createCtHandle() {
  return createBytes32();
}

// Create a list of ctHandles (bytes32[])
export function createCtHandles(length: number) {
  return Array.from({ length }, () => createCtHandle());
}
