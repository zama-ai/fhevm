import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre, { ethers } from "hardhat";

import { deployInitKmsHTTPZFixture } from "./utils/deploys";
import { createEIP712ResponsePublicDecrypt, getSignaturesPublicDecrypt } from "./utils/eip712";

describe("DecryptionManager", function () {
  async function deployDecryptionManagerFixture() {
    const { httpz, owner, admin, kmsSigners, signers } = await loadFixture(deployInitKmsHTTPZFixture);

    const DecryptionManager = await hre.ethers.getContractFactory("DecryptionManager", owner);

    // TODO: Replace with actual ACL Manager contract once implemented
    // TODO: Replace with actual Payment Manager contract once implemented
    const decryptionManager = await DecryptionManager.connect(owner).deploy(
      httpz,
      "0x1234567890abcdef1234567890abcdef12345678",
      "0x1234567890abcdef1234567890abcdef12345678",
    );

    return { decryptionManager, owner, admin, kmsSigners, signers };
  }

  describe("Public Decryption", function () {
    it("Should request and respond to public decryption", async function () {
      const { decryptionManager, kmsSigners, signers } = await loadFixture(deployDecryptionManagerFixture);

      // Define a user
      const user = signers[0];

      // Create 3 dummy ciphertext handles
      const ctHandles = [1, 2, 3];

      // Create a dummy decrypted result
      const decryptedResult = hre.ethers.randomBytes(32);

      // Create EIP712 messages and get associated KMS nodes' signatures
      const decryptionManagerAddress = await decryptionManager.getAddress();
      const eip712Message = createEIP712ResponsePublicDecrypt(
        hre.network.config.chainId!,
        decryptionManagerAddress,
        ctHandles,
        decryptedResult,
      );

      // Sign the message with all KMS nodes and get their signatures
      const [signature1, signature2, signature3, signature4] = await getSignaturesPublicDecrypt(
        eip712Message,
        kmsSigners,
      );

      // Expected public decryption id
      const publicDecryptionId = 1;

      // Request public decryption (any user can do so)
      const requestTx = await decryptionManager.connect(user).publicDecryptionRequest(ctHandles);

      // TODO: Check the arguments once the ACLManager is implemented
      // Check request event
      await expect(requestTx).to.emit(decryptionManager, "PublicDecryptionRequest");
      // .withArgs(publicDecryptionId, ctHandleCiphertext128Pairs);

      const [userSignature] = await getSignaturesPublicDecrypt(eip712Message, [user]);

      // Check that someone else than a KMS node cannot request public decryption response
      // because the signature is not valid (it does not belong to a KMS node)
      await expect(
        decryptionManager.connect(user).publicDecryptionResponse(publicDecryptionId, decryptedResult, userSignature),
      )
        .to.be.revertedWithCustomError(decryptionManager, "InvalidKmsSigner")
        .withArgs(user.address);

      // Trigger 4 public decryption responses, one for each KMS node
      await decryptionManager
        .connect(kmsSigners[0])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature1);

      await decryptionManager
        .connect(kmsSigners[1])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature2);

      const responseTx3 = await decryptionManager
        .connect(kmsSigners[2])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature3);

      // Check response event: it should only contain 3 valid signatures
      await expect(responseTx3)
        .to.emit(decryptionManager, "PublicDecryptionResponse")
        .withArgs(publicDecryptionId, decryptedResult, [signature1, signature2, signature3]);

      // Check that the public decryption is done
      const isPublicDecryptionDone = await decryptionManager.connect(user).isPublicDecryptionDone(publicDecryptionId);
      expect(isPublicDecryptionDone).to.be.true;

      // The 4th response should be ignored and not emit an event
      const responseTx4 = await decryptionManager
        .connect(kmsSigners[3])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature4);

      // Check that the the 4th response does not emit an event
      await expect(responseTx4).to.not.emit(decryptionManager, "PublicDecryptionResponse");

      // Check that a KMS node cannot sign several times for the same public decryption
      await expect(
        decryptionManager
          .connect(kmsSigners[0])
          .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature1),
      )
        .to.be.revertedWithCustomError(decryptionManager, "KmsSignerAlreadySigned")
        .withArgs(publicDecryptionId, kmsSigners[0].address);
    });
  });
});
