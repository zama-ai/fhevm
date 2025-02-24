import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { EventLog } from "ethers";
import hre from "hardhat";

import { deployDecryptionManagerFixture } from "./utils/deploys";
import { createEIP712ResponsePublicDecrypt, getSignaturesPublicDecrypt } from "./utils/eip712";

describe("DecryptionManager", function () {
  // Deploy contracts, trigger a key generation in KeyManager contract and activate the key
  async function deployWithActivatedKeyFixture() {
    const {
      keyManager,
      ciphertextStorage,
      aclManager,
      decryptionManager,
      owner,
      admins,
      user,
      kmsSigners,
      coprocessorSigners,
      signers,
    } = await loadFixture(deployDecryptionManagerFixture);

    // Trigger a preprocessing keygen request
    const txRequest = await keyManager.connect(admins[0]).preprocessKeygenRequest();

    // Get the preKeyRequestId from the event in the transaction receipt
    const receipt = await txRequest.wait();
    const event = receipt?.logs[0] as EventLog;
    const preKeyRequestId = Number(event?.args[0]);

    // Define a preKeyId for the preprocessing keygen response
    const preKeyId = 1;

    // Trigger preprocessing keygen responses for all KMS nodes
    for (let i = 0; i < kmsSigners.length; i++) {
      await keyManager.connect(kmsSigners[i]).preprocessKeygenResponse(preKeyRequestId, preKeyId);
    }

    // Trigger a keygen request
    await keyManager.connect(admins[0]).keygenRequest(preKeyId);

    // Define a keyId for keygen response
    const keyId = 1;

    // Trigger keygen responses for all KMS nodes
    for (let i = 0; i < kmsSigners.length; i++) {
      await keyManager.connect(kmsSigners[i]).keygenResponse(preKeyId, keyId);
    }

    // Request activation of the key
    await keyManager.connect(admins[0]).activateKeyRequest(keyId);

    // Trigger activation responses for all coprocessors
    for (let i = 0; i < coprocessorSigners.length; i++) {
      await keyManager.connect(coprocessorSigners[i]).activateKeyResponse(keyId);
    }

    return {
      ciphertextStorage,
      aclManager,
      decryptionManager,
      owner,
      admins,
      user,
      kmsSigners,
      coprocessorSigners,
      signers,
      keyId,
    };
  }

  describe("Public Decryption", function () {
    const chainId = hre.network.config.chainId!;

    // Create 3 dummy ciphertext handles
    const ctHandles = [2025, 2026, 2027];

    // Expected public decryption id (after first request)
    const publicDecryptionId = 1;

    // Create a dummy decrypted result
    const decryptedResult = hre.ethers.randomBytes(32);

    // Deploy the DecryptionManager and add ciphertext materials associated to the handles
    async function deployAddCiphertextFixture() {
      const { ciphertextStorage, aclManager, decryptionManager, kmsSigners, coprocessorSigners, user, keyId } =
        await loadFixture(deployWithActivatedKeyFixture);

      // Define dummy ciphertext values
      const ciphertext64 = "0x02";
      const ciphertext128 = "0x03";

      let ciphertextMaterials = [];

      // Allow public decryption
      for (const ctHandle of ctHandles) {
        for (let i = 0; i < coprocessorSigners.length; i++) {
          await ciphertextStorage
            .connect(coprocessorSigners[i])
            .addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);
        }

        // Store the ciphertext materials for event checks
        ciphertextMaterials.push([ctHandle, keyId, ciphertext128]);
      }

      return { aclManager, decryptionManager, kmsSigners, coprocessorSigners, user, ciphertextMaterials };
    }

    // Deploy the DecryptionManager and allow handles for public decryption
    async function deployAllowPublicDecryptionFixture() {
      const { aclManager, decryptionManager, kmsSigners, coprocessorSigners, user, ciphertextMaterials } =
        await loadFixture(deployAddCiphertextFixture);

      // Allow public decryption
      for (const ctHandle of ctHandles) {
        for (let i = 0; i < coprocessorSigners.length; i++) {
          await aclManager.connect(coprocessorSigners[i]).allowPublicDecrypt(chainId, ctHandle);
        }
      }

      return { decryptionManager, kmsSigners, coprocessorSigners, user, ciphertextMaterials };
    }

    async function deployGetEIP712Fixture() {
      const { decryptionManager, kmsSigners, user } = await loadFixture(deployAllowPublicDecryptionFixture);

      // Create EIP712 messages and get associated KMS nodes' signatures
      const decryptionManagerAddress = await decryptionManager.getAddress();
      const eip712Message = createEIP712ResponsePublicDecrypt(
        chainId,
        decryptionManagerAddress,
        ctHandles,
        decryptedResult,
      );

      return { decryptionManager, kmsSigners, user, eip712Message };
    }

    it("Should request a public decryption", async function () {
      const { decryptionManager, user, ciphertextMaterials } = await loadFixture(deployAllowPublicDecryptionFixture);

      // Request public decryption (any user can do so)
      const requestTx = await decryptionManager.connect(user).publicDecryptionRequest(ctHandles);

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "PublicDecryptionRequest")
        .withArgs(publicDecryptionId, ciphertextMaterials);
    });

    it("Should revert because handles are not allowed for public decryption", async function () {
      const { aclManager, decryptionManager, user } = await loadFixture(deployDecryptionManagerFixture);

      // Check that the request fails because the handles are not allowed for public decryption
      // Note: the function should be reverted on the first handle since it loops over the handles
      // in order internally
      await expect(decryptionManager.connect(user).publicDecryptionRequest(ctHandles))
        .to.be.revertedWithCustomError(aclManager, "PublicDecryptNotAllowed")
        .withArgs(ctHandles[0]);
    });

    it("Should revert because invalid signer in response", async function () {
      const { decryptionManager, user, eip712Message } = await loadFixture(deployGetEIP712Fixture);

      // Request public decryption
      // This step is necessary, else the publicDecryptionId won't be set in the state and the
      // signature verification will use wrong handles
      await decryptionManager.connect(user).publicDecryptionRequest(ctHandles);

      // Sign the message with the user
      const [userSignature] = await getSignaturesPublicDecrypt(eip712Message, [user]);

      // Check that the signature verification fails because the signer is not a registered KMS node
      await expect(
        decryptionManager.connect(user).publicDecryptionResponse(publicDecryptionId, decryptedResult, userSignature),
      )
        .to.be.revertedWithCustomError(decryptionManager, "InvalidKmsSigner")
        .withArgs(user.address);
    });

    it("Should revert because of two responses with same signature", async function () {
      const { decryptionManager, kmsSigners, user, eip712Message } = await loadFixture(deployGetEIP712Fixture);

      const firstKmsSigner = kmsSigners[0];

      // Request public decryption
      await decryptionManager.connect(user).publicDecryptionRequest(ctHandles);

      // Sign the message with the first KMS node and get its signature
      const [signature1] = await getSignaturesPublicDecrypt(eip712Message, [firstKmsSigner]);

      // Trigger a first public decryption response
      await decryptionManager
        .connect(firstKmsSigner)
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature1);

      // Check that a KMS node cannot sign a second time for the same public decryption
      await expect(
        decryptionManager
          .connect(firstKmsSigner)
          .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature1),
      )
        .to.be.revertedWithCustomError(decryptionManager, "KmsSignerAlreadySigned")
        .withArgs(publicDecryptionId, firstKmsSigner.address);
    });

    it("Should reach consensus with 2 valid responses", async function () {
      const { decryptionManager, kmsSigners, user, eip712Message } = await loadFixture(deployGetEIP712Fixture);

      // Request public decryption
      await decryptionManager.connect(user).publicDecryptionRequest(ctHandles);

      // Sign the message with all KMS nodes and get the first 2 signatures
      const [signature1, signature2] = await getSignaturesPublicDecrypt(eip712Message, kmsSigners);

      // Trigger two valid public decryption responses
      await decryptionManager
        .connect(kmsSigners[0])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature1);

      const responseTx2 = await decryptionManager
        .connect(kmsSigners[1])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature2);

      // Consensus should be reached at the second response
      // Check 2nd response event: it should only contain 2 valid signatures
      await expect(responseTx2)
        .to.emit(decryptionManager, "PublicDecryptionResponse")
        .withArgs(publicDecryptionId, decryptedResult, [signature1, signature2]);

      // Check that the public decryption is done
      const isPublicDecryptionDone = await decryptionManager.connect(user).isPublicDecryptionDone(publicDecryptionId);
      expect(isPublicDecryptionDone).to.be.true;
    });

    it("Should ignore other valid responses", async function () {
      const { decryptionManager, kmsSigners, user, eip712Message } = await loadFixture(deployGetEIP712Fixture);

      // Request public decryption
      await decryptionManager.connect(user).publicDecryptionRequest(ctHandles);

      // Sign the message with all KMS nodes and get their signatures
      const [signature1, signature2, signature3, signature4] = await getSignaturesPublicDecrypt(
        eip712Message,
        kmsSigners,
      );

      // Trigger four valid public decryption responses
      const responseTx1 = await decryptionManager
        .connect(kmsSigners[0])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature1);

      await decryptionManager
        .connect(kmsSigners[1])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature2);

      const responseTx3 = await decryptionManager
        .connect(kmsSigners[2])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature3);

      const responseTx4 = await decryptionManager
        .connect(kmsSigners[3])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature4);

      // Check that the 1st, 3rd and 4th responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet
      // - 3rd and 4th responses are ignored (not reverted) even though they are late
      await expect(responseTx1).to.not.emit(decryptionManager, "PublicDecryptionResponse");
      await expect(responseTx3).to.not.emit(decryptionManager, "PublicDecryptionResponse");
      await expect(responseTx4).to.not.emit(decryptionManager, "PublicDecryptionResponse");
    });
  });
});
