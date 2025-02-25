import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { EventLog } from "ethers";
import hre from "hardhat";

import { IDecryptionManager } from "../typechain-types";
import {
  createEIP712RequestUserDecrypt,
  createEIP712ResponsePublicDecrypt,
  createEIP712ResponseUserDecrypt,
  deployDecryptionManagerFixture,
  getSignaturesPublicDecrypt,
  getSignaturesUserDecryptRequest,
  getSignaturesUserDecryptResponse,
} from "./utils";

describe("DecryptionManager", function () {
  const chainId = hre.network.config.chainId!;

  // Create 3 dummy ciphertext handles
  const ctHandles = [2025, 2026, 2027];

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

  // Deploy the DecryptionManager and add SNS ciphertext materials associated to the handles
  async function deployAddCiphertextFixture() {
    const { ciphertextStorage, aclManager, decryptionManager, kmsSigners, coprocessorSigners, user, keyId } =
      await loadFixture(deployWithActivatedKeyFixture);

    // Define dummy ciphertext values
    const ciphertext = "0x02";
    const snsCiphertext = "0x03";

    let snsCiphertextMaterials = [];

    // Allow public decryption
    for (const ctHandle of ctHandles) {
      for (let i = 0; i < coprocessorSigners.length; i++) {
        await ciphertextStorage
          .connect(coprocessorSigners[i])
          .addCiphertext(ctHandle, keyId, chainId, ciphertext, snsCiphertext);
      }

      // Store the SNS ciphertext materials for event checks
      snsCiphertextMaterials.push([ctHandle, keyId, snsCiphertext]);
    }

    return { aclManager, decryptionManager, kmsSigners, coprocessorSigners, user, snsCiphertextMaterials };
  }

  describe("Public Decryption", function () {
    // Expected public decryption id (after first request)
    const publicDecryptionId = 1;

    // Create a dummy decrypted result
    const decryptedResult = hre.ethers.randomBytes(32);

    // Deploy the DecryptionManager and allow handles for public decryption
    async function deployAllowPublicDecryptionFixture() {
      const { aclManager, decryptionManager, kmsSigners, coprocessorSigners, user, snsCiphertextMaterials } =
        await loadFixture(deployAddCiphertextFixture);

      // Allow public decryption
      for (const ctHandle of ctHandles) {
        for (let i = 0; i < coprocessorSigners.length; i++) {
          await aclManager.connect(coprocessorSigners[i]).allowPublicDecrypt(chainId, ctHandle);
        }
      }

      return { decryptionManager, kmsSigners, coprocessorSigners, user, snsCiphertextMaterials };
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
      const { decryptionManager, user, snsCiphertextMaterials } = await loadFixture(deployAllowPublicDecryptionFixture);

      // Request public decryption (any user can do so)
      const requestTx = await decryptionManager.connect(user).publicDecryptionRequest(ctHandles);

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "PublicDecryptionRequest")
        .withArgs(publicDecryptionId, snsCiphertextMaterials);
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

  describe("User Decryption", function () {
    // Expected user decryption id (after first request)
    const userDecryptionId = 1;

    // Create a dummy reencrypted share
    const reencryptedShare = hre.ethers.randomBytes(32);

    // Deploy the DecryptionManager and allow handles for public decryption
    async function deployAllowUserDecryptionFixture() {
      const { aclManager, decryptionManager, kmsSigners, coprocessorSigners, user, snsCiphertextMaterials } =
        await loadFixture(deployAddCiphertextFixture);

      const contractAddress = hre.ethers.Wallet.createRandom().address;

      // Allow public decryption
      for (const ctHandle of ctHandles) {
        for (let i = 0; i < coprocessorSigners.length; i++) {
          await aclManager.connect(coprocessorSigners[i]).allowUserDecrypt(chainId, ctHandle, user.address);
          await aclManager.connect(coprocessorSigners[i]).allowUserDecrypt(chainId, ctHandle, contractAddress);
        }
      }

      return { decryptionManager, kmsSigners, coprocessorSigners, user, contractAddress, snsCiphertextMaterials };
    }

    async function deployUserDecryptEIP712Fixture() {
      const { decryptionManager, kmsSigners, user, contractAddress, snsCiphertextMaterials } = await loadFixture(
        deployAllowUserDecryptionFixture,
      );

      const publicKey = hre.ethers.randomBytes(32);
      const requestValidity: IDecryptionManager.RequestValidityStruct = {
        durationDays: 120,
        startTimestamp: Date.now(),
      };

      // Create EIP712 messages and get associated KMS nodes' signatures
      const decryptionManagerAddress = await decryptionManager.getAddress();
      const eip712RequestMessage = createEIP712RequestUserDecrypt(
        chainId,
        decryptionManagerAddress,
        publicKey,
        [contractAddress],
        chainId,
        requestValidity.startTimestamp.toString(),
        requestValidity.durationDays.toString(),
      );
      const eip712ResponseMessage = createEIP712ResponseUserDecrypt(
        chainId,
        decryptionManagerAddress,
        publicKey,
        [ctHandles[0]],
        reencryptedShare,
      );

      return {
        decryptionManager,
        kmsSigners,
        user,
        contractAddress,
        publicKey,
        requestValidity,
        snsCiphertextMaterials,
        eip712RequestMessage,
        eip712ResponseMessage,
      };
    }

    it("Should request a user decryption", async function () {
      const {
        decryptionManager,
        user,
        contractAddress,
        publicKey,
        requestValidity,
        snsCiphertextMaterials,
        eip712RequestMessage,
      } = await loadFixture(deployUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractsChainId = chainId;
      const contractAddresses = [contractAddress];
      const ctHandleContractPairs: IDecryptionManager.CtHandleContractPairStruct[] = [
        {
          contractAddress,
          ctHandle: ctHandles[0],
        },
      ];

      // Sign the message with the user
      const [userSignature] = await getSignaturesUserDecryptRequest(eip712RequestMessage, [user]);

      // Request user decryption
      const requestTx = await decryptionManager
        .connect(user)
        .userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          contractsChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        );

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "UserDecryptionRequest")
        .withArgs(userDecryptionId, [snsCiphertextMaterials[0]], publicKey);
    });

    it("Should revert because contractAddresses exceeds maximum length allowed", async function () {
      const { aclManager, decryptionManager, user } = await loadFixture(deployDecryptionManagerFixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = [];
      for (let i = 0; i < 11; i++) {
        contractAddresses.push(hre.ethers.Wallet.createRandom().address);
      }

      // Check that the request fails because the given contractAddresses exceeds the maximum length allowed
      await expect(
        decryptionManager
          .connect(user)
          .userDecryptionRequest(
            [],
            { durationDays: 120, startTimestamp: Date.now() },
            chainId,
            contractAddresses,
            user.address,
            hre.ethers.randomBytes(32),
            hre.ethers.randomBytes(32),
          ),
      )
        .to.be.revertedWithCustomError(decryptionManager, "ContractAddressesMaxLengthExceeded")
        .withArgs(10, contractAddresses.length);
    });

    it("Should revert because durationDays exceeds maximum allowed", async function () {
      const { aclManager, decryptionManager, user } = await loadFixture(deployDecryptionManagerFixture);

      // Create dummy input data for the user decryption request
      const requestValidity: IDecryptionManager.RequestValidityStruct = {
        durationDays: 400,
        startTimestamp: Date.now(),
      };

      // Check that the request fails because the given requestValidity.durationDays exceeds the maximum allowed
      await expect(
        decryptionManager
          .connect(user)
          .userDecryptionRequest(
            [],
            requestValidity,
            chainId,
            [],
            user.address,
            hre.ethers.randomBytes(32),
            hre.ethers.randomBytes(32),
          ),
      )
        .to.be.revertedWithCustomError(decryptionManager, "MaxDurationDaysExceeded")
        .withArgs(365, requestValidity.durationDays);
    });

    it("Should revert because user is not allowed for user decryption over given handles", async function () {
      const { aclManager, decryptionManager, user } = await loadFixture(deployDecryptionManagerFixture);

      // Create dummy input data for the user decryption request
      const contractsChainId = chainId;
      const contractAddress = hre.ethers.Wallet.createRandom().address;
      const publicKey = hre.ethers.randomBytes(32);
      const userSignature = hre.ethers.randomBytes(32);
      const ctHandleContractPairs: IDecryptionManager.CtHandleContractPairStruct[] = [
        {
          contractAddress,
          ctHandle: ctHandles[0],
        },
      ];
      const requestValidity: IDecryptionManager.RequestValidityStruct = {
        durationDays: 120,
        startTimestamp: Date.now(),
      };

      // Check that the request fails because the given userAddress is not allowed for user decryption
      // Note: the function should be reverted on the first handle since it loops over the handles
      // in order internally
      await expect(
        decryptionManager
          .connect(user)
          .userDecryptionRequest(
            ctHandleContractPairs,
            requestValidity,
            contractsChainId,
            [contractAddress],
            user.address,
            publicKey,
            userSignature,
          ),
      )
        .to.be.revertedWithCustomError(aclManager, "UserNotAllowedToUserDecrypt")
        .withArgs(ctHandles[0], user.address);
    });

    it("Should revert because of invalid EIP712 user request signature", async function () {
      const { decryptionManager, user, contractAddress, publicKey, requestValidity, eip712RequestMessage } =
        await loadFixture(deployUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = [contractAddress];
      const ctHandleContractPairs: IDecryptionManager.CtHandleContractPairStruct[] = [
        {
          contractAddress,
          ctHandle: ctHandles[0],
        },
      ];
      const fakeSigners = await hre.ethers.getSigners();

      // Sign the message with the user
      const [fakeSignature] = await getSignaturesUserDecryptRequest(eip712RequestMessage, [fakeSigners.pop()!]);

      // Request user decryption
      const requestTx = decryptionManager
        .connect(user)
        .userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          chainId,
          contractAddresses,
          user.address,
          publicKey,
          fakeSignature,
        );

      // Check request event
      await expect(requestTx)
        .to.be.revertedWithCustomError(decryptionManager, "InvalidUserSignature")
        .withArgs(fakeSignature);
    });

    it("Should revert because contract in ctHandleContractPairs not included in contractAddresses list", async function () {
      const { decryptionManager, user, contractAddress } = await loadFixture(deployAllowUserDecryptionFixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = [hre.ethers.Wallet.createRandom().address];
      const publicKey = hre.ethers.randomBytes(32);
      const ctHandleContractPairs: IDecryptionManager.CtHandleContractPairStruct[] = [
        {
          contractAddress,
          ctHandle: ctHandles[0],
        },
      ];
      const requestValidity: IDecryptionManager.RequestValidityStruct = {
        durationDays: 120,
        startTimestamp: Date.now(),
      };

      // Create EIP712 messages and get associated KMS nodes' signatures
      const decryptionManagerAddress = await decryptionManager.getAddress();
      const eip712Message = createEIP712RequestUserDecrypt(
        chainId,
        decryptionManagerAddress,
        publicKey,
        contractAddresses,
        chainId,
        requestValidity.startTimestamp.toString(),
        requestValidity.durationDays.toString(),
      );

      // Sign the message with the user
      const [userSignature] = await getSignaturesUserDecryptRequest(eip712Message, [user]);

      // Request user decryption
      const requestTx = decryptionManager
        .connect(user)
        .userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          chainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        );

      // Check request event
      await expect(requestTx)
        .to.be.revertedWithCustomError(decryptionManager, "ContractNotInContractAddresses")
        .withArgs(contractAddress);
    });

    it("Should reach consensus with 3 valid responses", async function () {
      const {
        decryptionManager,
        user,
        contractAddress,
        requestValidity,
        publicKey,
        kmsSigners,
        eip712RequestMessage,
        eip712ResponseMessage,
      } = await loadFixture(deployUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractsChainId = chainId;
      const contractAddresses = [contractAddress];
      const ctHandleContractPairs: IDecryptionManager.CtHandleContractPairStruct[] = [
        {
          contractAddress,
          ctHandle: ctHandles[0],
        },
      ];

      // Sign the message with the user
      const [userSignature] = await getSignaturesUserDecryptRequest(eip712RequestMessage, [user]);

      // Request user decryption
      await decryptionManager
        .connect(user)
        .userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          contractsChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        );

      // Sign the message with all KMS nodes and get the first 3 signatures
      const [signature1, signature2, signature3] = await getSignaturesUserDecryptResponse(
        eip712ResponseMessage,
        kmsSigners,
      );

      // Trigger three valid user decryption responses
      await decryptionManager
        .connect(kmsSigners[0])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, signature1);

      await decryptionManager
        .connect(kmsSigners[1])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, signature2);

      const responseTx3 = await decryptionManager
        .connect(kmsSigners[2])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, signature3);

      // Consensus should be reached at the third response (reconstruction threshold)
      // Check 3rd response event: it should only contain 3 valid signatures
      await expect(responseTx3)
        .to.emit(decryptionManager, "UserDecryptionResponse")
        .withArgs(
          userDecryptionId,
          [reencryptedShare, reencryptedShare, reencryptedShare],
          [signature1, signature2, signature3],
        );

      // Check that the user decryption is done
      const isUserDecryptionDone = await decryptionManager.connect(user).isUserDecryptionDone(userDecryptionId);
      expect(isUserDecryptionDone).to.be.true;
    });

    it("Should ignore other valid responses", async function () {
      const {
        decryptionManager,
        user,
        publicKey,
        requestValidity,
        contractAddress,
        kmsSigners,
        eip712RequestMessage,
        eip712ResponseMessage,
      } = await loadFixture(deployUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractsChainId = chainId;
      const contractAddresses = [contractAddress];
      const ctHandleContractPairs: IDecryptionManager.CtHandleContractPairStruct[] = [
        {
          contractAddress,
          ctHandle: ctHandles[0],
        },
      ];

      // Sign the message with the user
      const [userSignature] = await getSignaturesUserDecryptRequest(eip712RequestMessage, [user]);

      // Request user decryption
      await decryptionManager
        .connect(user)
        .userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          contractsChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        );

      // Sign the message with all KMS nodes and get the first 3 signatures
      const [signature1, signature2, signature3, signature4] = await getSignaturesUserDecryptResponse(
        eip712ResponseMessage,
        kmsSigners,
      );

      // Trigger three valid user decryption responses
      const responseTx1 = await decryptionManager
        .connect(kmsSigners[0])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, signature1);

      const responseTx2 = await decryptionManager
        .connect(kmsSigners[1])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, signature2);

      await decryptionManager
        .connect(kmsSigners[2])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, signature3);

      const responseTx4 = await decryptionManager
        .connect(kmsSigners[3])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, signature4);

      // Check that the 1st, 2nd and 4th responses do not emit an event:
      // - 1st and 2nd responses are ignored because consensus is not reached yet
      // - 4th response is ignored (not reverted) even though they are late
      await expect(responseTx1).to.not.emit(decryptionManager, "UserDecryptionResponse");
      await expect(responseTx2).to.not.emit(decryptionManager, "UserDecryptionResponse");
      await expect(responseTx4).to.not.emit(decryptionManager, "UserDecryptionResponse");
    });
  });
});
