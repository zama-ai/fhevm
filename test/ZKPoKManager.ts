import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

describe("ZKPoKManager", function () {
  const keychainId = [1, 2, 3];

  async function deployZKPoKManagerFixture() {
    const ZKPoKManager = await hre.ethers.getContractFactory("ZKPoKManager");
    const zkpokManager = await ZKPoKManager.deploy("0xDA9FeD390f02F559E62240a112aBd2FAe06DCdB5");
    return { zkpokManager };
  }

  describe("ZKP Verification", function () {
    it("Should success on verifyProofRequest", async function () {
      // Given
      const { zkpokManager } = await loadFixture(deployZKPoKManagerFixture);
      const keychainId = "123";
      const zkProofId = "0";
      const chainId = "789";
      const contractAddress = "0xa83114A443dA1CecEFC50368531cACE9F37fCCcb";
      const userAddress = "0x388C818CA8B9251b393131C08a736A67ccB19297";
      const ctProofHandle = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9]);

      // When
      const result = zkpokManager.verifyProofRequest(keychainId, chainId, contractAddress, userAddress, ctProofHandle);

      // Then
      await expect(result)
        .to.emit(zkpokManager, "VerifyProofRequest")
        .withArgs(keychainId, zkProofId, chainId, contractAddress, userAddress, ctProofHandle);
    });

    it("Should success on verifyProofResponse", async function () {
      // Given
      const { zkpokManager } = await loadFixture(deployZKPoKManagerFixture);
      /// @param zkProofId The ID of the requested ZKProof verification
      /// @param handles The Coprocessor's computed handles
      /// @param signature The Coprocessor's signature
      const zkProofId = "123";
      const handles = [hre.ethers.encodeBytes32String("123456789")];
      const signature1 = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9]);

      // When
      const result = zkpokManager.verifyProofResponse(zkProofId, handles, signature1);

      // Then
      await expect(result).to.emit(zkpokManager, "VerifyProofResponse").withArgs(zkProofId, handles, []);
    });
  });
});
