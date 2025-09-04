import { anyValue } from '@nomicfoundation/hardhat-chai-matchers/withArgs';
import { loadFixture, time } from '@nomicfoundation/hardhat-toolbox/network-helpers';
import { expect } from 'chai';
import hre from "hardhat";

// Define the maximum value for a uint64
const UINT64_MAX = (BigInt(1) << BigInt(64)) - BigInt(1);
const MAX_DECRYPTION_REQUEST_BITS = 2048;

// Create a byte input of a given length
export function createByteInput(length: number = 32): string {
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

const HOST_CHAIN_ID = 12345

describe('DecryptionOracle', function() {
  // We define a fixture to reuse the same setup in every test.
  // We use loadFixture to run this setup once, snapshot that state,
  // and reset Hardhat Network to that snapshot in every test.
  async function deployDecryptionOracleFixture() {
    // Contracts are deployed using the first signer/account by default
    const [owner, otherAccount] = await hre.ethers.getSigners();

    const DecryptionOracle = await hre.ethers.getContractFactory('DecryptionOracle');
    const decryptionOracle = await DecryptionOracle.deploy();

    return { owner, otherAccount, decryptionOracle };
  }

  const ebytes128CtHandle = createCtHandle(HOST_CHAIN_ID, 10);
  describe('Deployment', function() {
    it('Should deploy DecryptionOracle', async function() {
      const { decryptionOracle } = await loadFixture(deployDecryptionOracleFixture);

      expect(await decryptionOracle.getVersion()).to.equal('DecryptionOracle v0.1.0');
    });

    it("Should revert because total bit size exceeds the maximum allowed", async function() {
      const { decryptionOracle } = await loadFixture(deployDecryptionOracleFixture);
      // Create a list of 3 ebytes128 ctHandles (each has a bit size of 1024 bits)
      const largeBitSizeCtHandles = [ebytes128CtHandle, ebytes128CtHandle, ebytes128CtHandle];

      // Calculate the new total bit size of this list
      const totalBitSize = 3072;

      // Check that the request fails because the total bit size exceeds the maximum allowed
      // NOTE: using dummy selector
      await expect(decryptionOracle.requestDecryption(0, largeBitSizeCtHandles, "0xad3cb1cc"))
        .to.be.revertedWithCustomError(decryptionOracle, "MaxDecryptionRequestBitSizeExceeded")
        .withArgs(MAX_DECRYPTION_REQUEST_BITS, totalBitSize);
    });

  });
});
