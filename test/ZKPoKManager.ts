import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { ZKPoKManager } from "../typechain-types";
import { createEIP712ResponseMessage } from "./utils";

describe("ZKPoKManager", function () {
  async function deployZKPoKManagerFixture() {
    const ZKPoKManagerContract = await hre.ethers.getContractFactory("ZKPoKManager");
    // TODO: Implement the HTTPZ deployment and replace the first address
    const zkpokManager = await ZKPoKManagerContract.deploy(
      "0xDA9FeD390f02F559E62240a112aBd2FAe06DCdB5",
      "0xDA9FeD390f02F559E62240a112aBd2FAe06DCdB5",
    );
    return { zkpokManager };
  }

  describe("Verify proof request", async function () {
    const zkProofId = "0";
    let zkpokManagerInstance: ZKPoKManager;
    before(async function () {
      const { zkpokManager } = await loadFixture(deployZKPoKManagerFixture);
      zkpokManagerInstance = zkpokManager;
    });

    it("Should success", async function () {
      // Given
      const contractChainId = "789";
      const contractAddress = "0xa83114A443dA1CecEFC50368531cACE9F37fCCcb";
      const userAddress = "0x388C818CA8B9251b393131C08a736A67ccB19297";
      const ctProofHandle = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9]);

      // When
      const result = zkpokManagerInstance.verifyProofRequest(
        contractChainId,
        contractAddress,
        userAddress,
        ctProofHandle,
      );

      // Then
      await expect(result)
        .to.emit(zkpokManagerInstance, "VerifyProofRequest")
        .withArgs(zkProofId, contractChainId, contractAddress, userAddress, ctProofHandle);
    });
  });

  describe("Verify proof response", async function () {
    const zkProofId = "0";
    let signers: HardhatEthersSigner[];
    let zkpokManagerInstance: ZKPoKManager;

    beforeEach(async function () {
      const { zkpokManager } = await loadFixture(deployZKPoKManagerFixture);
      zkpokManagerInstance = zkpokManager;
      signers = await hre.ethers.getSigners();

      const contractChainId = "789";
      const contractAddress = "0xa83114A443dA1CecEFC50368531cACE9F37fCCcb";
      const userAddress = "0x388C818CA8B9251b393131C08a736A67ccB19297";
      const ctProofHandle = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9]);
      await zkpokManagerInstance.verifyProofRequest(contractChainId, contractAddress, userAddress, ctProofHandle);
    });

    it("Should success", async function () {
      // Given
      const handles = [hre.ethers.encodeBytes32String("123456789"), hre.ethers.encodeBytes32String("987654321")];
      const zkpokManagerAddress = await zkpokManagerInstance.getAddress();
      const eip712 = createEIP712ResponseMessage(hre.network.config.chainId!, zkpokManagerAddress);
      const signature1 = await signers[0].signTypedData(
        eip712.domain,
        { EIP712ResponseMessage: eip712.types.EIP712ResponseMessage },
        eip712.message,
      );
      const signature2 = await signers[1].signTypedData(
        eip712.domain,
        { EIP712ResponseMessage: eip712.types.EIP712ResponseMessage },
        eip712.message,
      );
      const signature3 = await signers[2].signTypedData(
        eip712.domain,
        { EIP712ResponseMessage: eip712.types.EIP712ResponseMessage },
        eip712.message,
      );

      // When
      await zkpokManagerInstance.verifyProofResponse(zkProofId, handles, signature1);
      await zkpokManagerInstance.verifyProofResponse(zkProofId, handles, signature2);
      let result = zkpokManagerInstance.verifyProofResponse(zkProofId, handles, signature3);

      // Then
      await expect(result)
        .to.emit(zkpokManagerInstance, "VerifyProofResponse")
        .withArgs(zkProofId, handles, [signature1, signature2, signature3]);
    });

    it("Should revert with CoprocessorHasAlreadySigned", async function () {
      // Given
      const handles = [hre.ethers.encodeBytes32String("123456789")];
      const zkpokManagerAddress = await zkpokManagerInstance.getAddress();
      const eip712 = createEIP712ResponseMessage(hre.network.config.chainId!, zkpokManagerAddress);
      const signature1 = await signers[0].signTypedData(
        eip712.domain,
        { EIP712ResponseMessage: eip712.types.EIP712ResponseMessage },
        eip712.message,
      );

      // When
      await zkpokManagerInstance.verifyProofResponse(zkProofId, handles, signature1);
      let result = zkpokManagerInstance.verifyProofResponse(zkProofId, handles, signature1);

      // Then
      await expect(result).revertedWithCustomError(zkpokManagerInstance, "CoprocessorHasAlreadySigned");
    });
  });
});
