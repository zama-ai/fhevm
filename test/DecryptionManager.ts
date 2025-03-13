import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { BigNumberish, EventLog } from "ethers";
import hre from "hardhat";

import { IDecryptionManager, KeyManager } from "../typechain-types";
// The type needs to be imported separately because it is not properly detected by the linter
// as this type is defined as a shared structs instead of directly in the IDecryptionManager interface
import { CtHandleContractPairStruct } from "../typechain-types/contracts/interfaces/IDecryptionManager";
import {
  createEIP712RequestDelegatedUserDecrypt,
  createEIP712RequestUserDecrypt,
  createEIP712ResponsePublicDecrypt,
  createEIP712ResponseUserDecrypt,
  deployDecryptionManagerFixture,
  getSignaturesDelegatedUserDecryptRequest,
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
      httpz,
      keyManager,
      ciphertextManager,
      aclManager,
      decryptionManager,
      owner,
      admins,
      user,
      kmsSigners,
      coprocessorSigners,
      signers,
      fheParamsName,
    } = await loadFixture(deployDecryptionManagerFixture);

    // Trigger a preprocessing keygen request
    const txRequest = await keyManager.connect(admins[0]).preprocessKeygenRequest(fheParamsName);

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
    const keyId1 = 1;

    // Trigger keygen responses for all KMS nodes
    for (let i = 0; i < kmsSigners.length; i++) {
      await keyManager.connect(kmsSigners[i]).keygenResponse(preKeyId, keyId1);
    }

    // Request activation of the key
    await keyManager.connect(admins[0]).activateKeyRequest(keyId1);

    // Trigger activation responses for all coprocessors
    for (let i = 0; i < coprocessorSigners.length; i++) {
      await keyManager.connect(coprocessorSigners[i]).activateKeyResponse(keyId1);
    }

    return {
      httpz,
      ciphertextManager,
      aclManager,
      decryptionManager,
      keyManager,
      owner,
      admins,
      user,
      kmsSigners,
      coprocessorSigners,
      signers,
      keyId1,
      fheParamsName,
    };
  }

  // Deploy the DecryptionManager and add SNS ciphertext materials associated to the handles
  async function deployAddCiphertextFixture() {
    const {
      httpz,
      keyManager,
      ciphertextManager,
      aclManager,
      decryptionManager,
      admins,
      kmsSigners,
      coprocessorSigners,
      user,
      keyId1,
      fheParamsName,
    } = await loadFixture(deployWithActivatedKeyFixture);

    // Define dummy ciphertext values
    const ciphertextDigest = hre.ethers.hexlify(hre.ethers.randomBytes(32));
    const snsCiphertextDigest = hre.ethers.hexlify(hre.ethers.randomBytes(32));

    let snsCiphertextMaterials = [];

    // Allow public decryption
    for (const ctHandle of ctHandles) {
      for (let i = 0; i < coprocessorSigners.length; i++) {
        await ciphertextManager
          .connect(coprocessorSigners[i])
          .addCiphertextMaterial(ctHandle, keyId1, chainId, ciphertextDigest, snsCiphertextDigest);
      }

      // Store the SNS ciphertext materials for event checks
      snsCiphertextMaterials.push([ctHandle, keyId1, snsCiphertextDigest, coprocessorSigners.map((s) => s.address)]);
    }

    return {
      httpz,
      keyManager,
      aclManager,
      decryptionManager,
      ciphertextManager,
      admins,
      kmsSigners,
      coprocessorSigners,
      user,
      snsCiphertextMaterials,
      keyId1,
      fheParamsName,
    };
  }

  // Create a new key, rotate it and activate it. It returns the new key ID.
  async function createAndRotateKey(
    sourceKeyId: BigNumberish,
    keyManager: KeyManager,
    admins: HardhatEthersSigner[],
    coprocessorSigners: HardhatEthersSigner[],
    kmsSigners: HardhatEthersSigner[],
    fheParamsName: string,
  ): Promise<BigNumberish> {
    const newKeyId = hre.ethers.toBigInt(hre.ethers.randomBytes(32));
    // Trigger a preprocessing keygen request
    let txRequest = await keyManager.connect(admins[0]).preprocessKeygenRequest(fheParamsName);

    // Get the preKeyRequestId from the event in the transaction receipt
    let receipt = await txRequest.wait();
    let event = receipt?.logs[0] as EventLog;
    const preKeyRequestId = Number(event?.args[0]);

    // Define a preKeyId for the preprocessing keygen response
    const preKeyId = hre.ethers.toBigInt(hre.ethers.randomBytes(32));

    // Trigger preprocessing keygen responses for all KMS nodes
    for (let i = 0; i < kmsSigners.length; i++) {
      await keyManager.connect(kmsSigners[i]).preprocessKeygenResponse(preKeyRequestId, preKeyId);
    }

    // Trigger a keygen request
    await keyManager.connect(admins[0]).keygenRequest(preKeyId);

    // Trigger keygen responses for all KMS nodes
    for (let i = 0; i < kmsSigners.length; i++) {
      await keyManager.connect(kmsSigners[i]).keygenResponse(preKeyId, newKeyId);
    }

    // Trigger a preprocessing kskgen request
    txRequest = await keyManager.connect(admins[0]).preprocessKskgenRequest(fheParamsName);

    // Get the preKeyRequestId from the event in the transaction receipt
    receipt = await txRequest.wait();
    event = receipt?.logs[0] as EventLog;
    const preKskRequestId = Number(event?.args[0]);

    // Define a preKskId for the preprocessing kskgen response
    const preKskId = hre.ethers.toBigInt(hre.ethers.randomBytes(32));

    // Trigger preprocessing kskgen responses for all KMS nodes
    for (let i = 0; i < kmsSigners.length; i++) {
      await keyManager.connect(kmsSigners[i]).preprocessKskgenResponse(preKskRequestId, preKskId);
    }

    // Trigger a kskgen request
    await keyManager.connect(admins[0]).kskgenRequest(preKskId, sourceKeyId, newKeyId);

    // Define a kskId for kskgen response
    const kskId = hre.ethers.toBigInt(hre.ethers.randomBytes(32));

    // Trigger kskgen responses for all KMS nodes
    for (let i = 0; i < kmsSigners.length; i++) {
      await keyManager.connect(kmsSigners[i]).kskgenResponse(preKskId, kskId);
    }

    // Request activation of the key
    await keyManager.connect(admins[0]).activateKeyRequest(newKeyId);

    // Trigger activation responses for all coprocessors
    for (let i = 0; i < coprocessorSigners.length; i++) {
      await keyManager.connect(coprocessorSigners[i]).activateKeyResponse(newKeyId);
    }

    return newKeyId;
  }

  describe("Public Decryption", function () {
    // Expected public decryption id (after first request)
    const publicDecryptionId = 1;

    // Create a dummy decrypted result
    const decryptedResult = hre.ethers.randomBytes(32);

    // Deploy the DecryptionManager and allow handles for public decryption
    async function deployAllowPublicDecryptionFixture() {
      const {
        httpz,
        keyManager,
        aclManager,
        decryptionManager,
        ciphertextManager,
        admins,
        kmsSigners,
        coprocessorSigners,
        user,
        snsCiphertextMaterials,
        keyId1,
        fheParamsName,
      } = await loadFixture(deployAddCiphertextFixture);

      // Allow public decryption
      for (const ctHandle of ctHandles) {
        for (let i = 0; i < coprocessorSigners.length; i++) {
          await aclManager.connect(coprocessorSigners[i]).allowPublicDecrypt(chainId, ctHandle);
        }
      }

      return {
        keyManager,
        httpz,
        aclManager,
        decryptionManager,
        ciphertextManager,
        admins,
        kmsSigners,
        coprocessorSigners,
        user,
        snsCiphertextMaterials,
        keyId1,
        fheParamsName,
      };
    }

    async function deployGetEIP712Fixture() {
      const { httpz, decryptionManager, kmsSigners, user } = await loadFixture(deployAllowPublicDecryptionFixture);

      // Create EIP712 messages and get associated KMS nodes' signatures
      const decryptionManagerAddress = await decryptionManager.getAddress();
      const eip712Message = createEIP712ResponsePublicDecrypt(
        chainId,
        decryptionManagerAddress,
        ctHandles,
        decryptedResult,
      );

      return { httpz, decryptionManager, kmsSigners, user, eip712Message };
    }

    it("Should request a public decryption with multiple ctHandles", async function () {
      const { decryptionManager, user, snsCiphertextMaterials } = await loadFixture(deployAllowPublicDecryptionFixture);

      // Request public decryption (any user can do so)
      const requestTx = await decryptionManager.connect(user).publicDecryptionRequest(ctHandles);

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "PublicDecryptionRequest")
        .withArgs(publicDecryptionId, snsCiphertextMaterials);
    });

    it("Should request a public decryption with a single ctHandle", async function () {
      const { decryptionManager, user, snsCiphertextMaterials } = await loadFixture(deployAllowPublicDecryptionFixture);

      // Request public decryption with a single ctHandle
      const requestTx = await decryptionManager.connect(user).publicDecryptionRequest([ctHandles[0]]);

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "PublicDecryptionRequest")
        .withArgs(publicDecryptionId, [snsCiphertextMaterials[0]]);
    });

    it("Should request a public decryption with empty ctHandles list", async function () {
      const { decryptionManager, user } = await loadFixture(deployAllowPublicDecryptionFixture);

      // Request public decryption with an empty list of ctHandles
      const requestTx = await decryptionManager.connect(user).publicDecryptionRequest([]);

      // Check request event
      await expect(requestTx).to.emit(decryptionManager, "PublicDecryptionRequest").withArgs(publicDecryptionId, []);
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

    it("Should revert because the signer is not a KMS node", async function () {
      const { httpz, decryptionManager, user, eip712Message } = await loadFixture(deployGetEIP712Fixture);

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
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.KMS_NODE_ROLE());
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
        .to.be.revertedWithCustomError(decryptionManager, "KmsSignerAlreadyResponded")
        .withArgs(publicDecryptionId, firstKmsSigner.address);
    });

    it("Should revert because of ctMaterials tied to different key IDs", async function () {
      const {
        keyManager,
        decryptionManager,
        ciphertextManager,
        aclManager,
        admins,
        kmsSigners,
        coprocessorSigners,
        user,
        keyId1,
        fheParamsName,
      } = await loadFixture(deployAllowPublicDecryptionFixture);

      const keyId2 = await createAndRotateKey(
        keyId1,
        keyManager,
        admins,
        coprocessorSigners,
        kmsSigners,
        fheParamsName,
      );

      // Define ciphertext dummy values
      const ctHandle = 2050;
      const ciphertextDigest = hre.ethers.hexlify(hre.ethers.randomBytes(32));
      const snsCiphertextDigest = hre.ethers.hexlify(hre.ethers.randomBytes(32));

      // Store the ciphertext and allow public decryption
      for (let i = 0; i < coprocessorSigners.length; i++) {
        await ciphertextManager
          .connect(coprocessorSigners[i])
          .addCiphertextMaterial(ctHandle, keyId2, chainId, ciphertextDigest, snsCiphertextDigest);
      }
      for (let i = 0; i < coprocessorSigners.length; i++) {
        await aclManager.connect(coprocessorSigners[i]).allowPublicDecrypt(chainId, ctHandle);
      }

      // Request public decryption with ctMaterials tied to different key IDs
      const requestTx = decryptionManager.connect(user).publicDecryptionRequest([...ctHandles, ctHandle]);

      // Check that different key IDs are not allowed for batched public decryption
      await expect(requestTx)
        .to.revertedWithCustomError(decryptionManager, "DifferentKeyIdsNotAllowed")
        .withArgs(keyId2);
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

    // Deploy the DecryptionManager and allow access the the handles for the user and the contract
    async function deployAllowAccountFixture() {
      const {
        keyManager,
        aclManager,
        decryptionManager,
        ciphertextManager,
        admins,
        kmsSigners,
        coprocessorSigners,
        user,
        snsCiphertextMaterials,
        keyId1,
        fheParamsName,
      } = await loadFixture(deployAddCiphertextFixture);

      const contractAddress = hre.ethers.Wallet.createRandom().address;

      // Allow user decryption
      for (const ctHandle of ctHandles) {
        for (let i = 0; i < coprocessorSigners.length; i++) {
          await aclManager.connect(coprocessorSigners[i]).allowAccount(chainId, ctHandle, user.address);
          await aclManager.connect(coprocessorSigners[i]).allowAccount(chainId, ctHandle, contractAddress);
        }
      }

      return {
        keyManager,
        aclManager,
        decryptionManager,
        ciphertextManager,
        admins,
        kmsSigners,
        coprocessorSigners,
        user,
        contractAddress,
        snsCiphertextMaterials,
        keyId1,
        fheParamsName,
      };
    }

    async function deployUserDecryptEIP712Fixture() {
      const {
        keyManager,
        aclManager,
        decryptionManager,
        ciphertextManager,
        admins,
        coprocessorSigners,
        kmsSigners,
        user,
        contractAddress,
        snsCiphertextMaterials,
        keyId1,
        fheParamsName,
      } = await loadFixture(deployAllowAccountFixture);

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
        keyManager,
        aclManager,
        decryptionManager,
        ciphertextManager,
        admins,
        coprocessorSigners,
        kmsSigners,
        user,
        contractAddress,
        publicKey,
        requestValidity,
        snsCiphertextMaterials,
        keyId1,
        eip712RequestMessage,
        eip712ResponseMessage,
        fheParamsName,
      };
    }

    it("Should request a user decryption with multiple ctHandleContractPairs", async function () {
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
      let ctHandleContractPairs: CtHandleContractPairStruct[] = [];
      for (const ctHandle of ctHandles) {
        ctHandleContractPairs.push({
          contractAddress,
          ctHandle,
        });
      }

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
        .withArgs(userDecryptionId, snsCiphertextMaterials, publicKey);
    });

    it("Should request a user decryption with a single ctHandleContractPair", async function () {
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
      const ctHandleContractPairs: CtHandleContractPairStruct[] = [
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

    it("Should request a user decryption with empty ctHandleContractPairs list", async function () {
      const { decryptionManager, user, contractAddress, publicKey, requestValidity, eip712RequestMessage } =
        await loadFixture(deployUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractsChainId = chainId;
      const contractAddresses = [contractAddress];
      const ctHandleContractPairs: CtHandleContractPairStruct[] = [];

      // Sign the message with the user
      const [userSignature] = await getSignaturesUserDecryptRequest(eip712RequestMessage, [user]);

      // Request user decryption with an empty list of ctHandleContractPairs
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
        .withArgs(userDecryptionId, [], publicKey);
    });

    it("Should revert because contractAddresses exceeds maximum length allowed", async function () {
      const { decryptionManager, user } = await loadFixture(deployDecryptionManagerFixture);

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
      const { decryptionManager, user } = await loadFixture(deployDecryptionManagerFixture);

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
      const ctHandleContractPairs: CtHandleContractPairStruct[] = [
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
      // TODO: Enable the allow check test back once the allow flow is fully implemented
      // @dev See https://github.com/zama-ai/gateway-l2/issues/188
      // await expect(
      //   decryptionManager
      //     .connect(user)
      //     .userDecryptionRequest(
      //       ctHandleContractPairs,
      //       requestValidity,
      //       contractsChainId,
      //       [contractAddress],
      //       user.address,
      //       publicKey,
      //       userSignature,
      //     ),
      // )
      //   .to.be.revertedWithCustomError(aclManager, "AccountNotAllowedToUseCiphertext")
      //   .withArgs(ctHandles[0], user.address);
    });

    it("Should revert because of invalid EIP712 user request signature", async function () {
      const { decryptionManager, user, contractAddress, publicKey, requestValidity, eip712RequestMessage } =
        await loadFixture(deployUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = [contractAddress];
      const ctHandleContractPairs: CtHandleContractPairStruct[] = [
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
      const { decryptionManager, user, contractAddress } = await loadFixture(deployAllowAccountFixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = [hre.ethers.Wallet.createRandom().address];
      const publicKey = hre.ethers.randomBytes(32);
      const ctHandleContractPairs: CtHandleContractPairStruct[] = [
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

    it("Should revert because of ctMaterials tied to different key IDs", async function () {
      const {
        keyManager,
        decryptionManager,
        ciphertextManager,
        aclManager,
        admins,
        coprocessorSigners,
        kmsSigners,
        user,
        keyId1,
        contractAddress,
        requestValidity,
        publicKey,
        eip712RequestMessage,
        fheParamsName,
      } = await loadFixture(deployUserDecryptEIP712Fixture);

      const keyId2 = await createAndRotateKey(
        keyId1,
        keyManager,
        admins,
        coprocessorSigners,
        kmsSigners,
        fheParamsName,
      );

      // Define ciphertext dummy values
      const ctHandle = 2050;
      const ciphertextDigest = hre.ethers.hexlify(hre.ethers.randomBytes(32));
      const snsCiphertextDigest = hre.ethers.hexlify(hre.ethers.randomBytes(32));

      // Store the ciphertext and allow public decryption
      for (let i = 0; i < coprocessorSigners.length; i++) {
        await ciphertextManager
          .connect(coprocessorSigners[i])
          .addCiphertextMaterial(ctHandle, keyId2, chainId, ciphertextDigest, snsCiphertextDigest);
      }
      for (let i = 0; i < coprocessorSigners.length; i++) {
        await aclManager.connect(coprocessorSigners[i]).allowAccount(chainId, ctHandle, user.address);
        await aclManager.connect(coprocessorSigners[i]).allowAccount(chainId, ctHandle, contractAddress);
      }

      // Create dummy input data for the user decryption request
      const contractsChainId = chainId;
      const contractAddresses = [contractAddress];
      const ctHandleContractPairs: CtHandleContractPairStruct[] = [
        {
          contractAddress,
          ctHandle: ctHandles[0],
        },
        {
          contractAddress,
          ctHandle,
        },
      ];

      // Sign the message with the user
      const [userSignature] = await getSignaturesUserDecryptRequest(eip712RequestMessage, [user]);

      // Request user decryption with ctMaterials tied to different key IDs
      const requestTx = decryptionManager
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

      // Check that different key IDs are not allowed for batched user decryption
      await expect(requestTx)
        .to.revertedWithCustomError(decryptionManager, "DifferentKeyIdsNotAllowed")
        .withArgs(keyId2);
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
      const ctHandleContractPairs: CtHandleContractPairStruct[] = [
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
      const ctHandleContractPairs: CtHandleContractPairStruct[] = [
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

  describe("Delegated User Decryption", function () {
    // Expected user decryption id (after first request)
    const userDecryptionId = 1;

    // Create a dummy reencrypted share
    const reencryptedShare = hre.ethers.randomBytes(32);

    // Deploy the DecryptionManager and allow handles for user decryption
    async function deployAllowDelegatedUserDecryptionFixture() {
      const {
        keyManager,
        aclManager,
        decryptionManager,
        ciphertextManager,
        admins,
        kmsSigners,
        coprocessorSigners,
        user,
        snsCiphertextMaterials,
        keyId1,
        fheParamsName,
      } = await loadFixture(deployAddCiphertextFixture);

      const delegationAccounts: IDecryptionManager.DelegationAccountsStruct = {
        userAddress: user.address,
        delegatedAddress: hre.ethers.Wallet.createRandom().address,
      };

      // Allow user decryption and build ctHandleContractPairs
      const ctHandleContractPairs: CtHandleContractPairStruct[] = [];
      for (const ctHandle of ctHandles) {
        const contractAddress = hre.ethers.Wallet.createRandom().address;
        for (let i = 0; i < coprocessorSigners.length; i++) {
          await aclManager
            .connect(coprocessorSigners[i])
            .allowAccount(chainId, ctHandle, delegationAccounts.delegatedAddress);
          await aclManager.connect(coprocessorSigners[i]).allowAccount(chainId, ctHandle, contractAddress);
        }
        ctHandleContractPairs.push({
          contractAddress,
          ctHandle,
        });
      }

      // Delegate account
      for (const coprocessorSigner of coprocessorSigners) {
        await aclManager.connect(coprocessorSigner).delegateAccount(
          chainId,
          user.address,
          delegationAccounts.delegatedAddress,
          ctHandleContractPairs.map((pair) => pair.contractAddress),
        );
      }

      return {
        keyManager,
        aclManager,
        decryptionManager,
        ciphertextManager,
        admins,
        kmsSigners,
        coprocessorSigners,
        user,
        ctHandleContractPairs,
        delegationAccounts,
        snsCiphertextMaterials,
        keyId1,
        fheParamsName,
      };
    }

    async function deployDelegatedUserDecryptEIP712Fixture() {
      const {
        keyManager,
        aclManager,
        decryptionManager,
        ciphertextManager,
        admins,
        coprocessorSigners,
        kmsSigners,
        user,
        ctHandleContractPairs,
        delegationAccounts,
        snsCiphertextMaterials,
        keyId1,
        fheParamsName,
      } = await loadFixture(deployAllowDelegatedUserDecryptionFixture);

      const publicKey = hre.ethers.randomBytes(32);
      const requestValidity: IDecryptionManager.RequestValidityStruct = {
        durationDays: 120,
        startTimestamp: Date.now(),
      };

      // Create EIP712 messages and get associated KMS nodes' signatures
      const decryptionManagerAddress = await decryptionManager.getAddress();
      const eip712RequestMessage = createEIP712RequestDelegatedUserDecrypt(
        chainId,
        decryptionManagerAddress,
        publicKey,
        ctHandleContractPairs.map((pair) => pair.contractAddress.toString()),
        delegationAccounts.delegatedAddress.toString(),
        chainId,
        requestValidity.startTimestamp.toString(),
        requestValidity.durationDays.toString(),
      );
      const eip712ResponseMessage = createEIP712ResponseUserDecrypt(
        chainId,
        decryptionManagerAddress,
        publicKey,
        ctHandleContractPairs.map((pair) => parseInt(pair.ctHandle.toString())),
        reencryptedShare,
      );

      return {
        keyManager,
        aclManager,
        decryptionManager,
        ciphertextManager,
        admins,
        coprocessorSigners,
        kmsSigners,
        user,
        ctHandleContractPairs,
        delegationAccounts,
        publicKey,
        requestValidity,
        snsCiphertextMaterials,
        keyId1,
        eip712RequestMessage,
        eip712ResponseMessage,
        fheParamsName,
      };
    }

    it("Should request a user decryption with multiple ctHandleContractPairs", async function () {
      const {
        decryptionManager,
        user,
        ctHandleContractPairs,
        delegationAccounts,
        publicKey,
        requestValidity,
        snsCiphertextMaterials,
        eip712RequestMessage,
      } = await loadFixture(deployDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractsChainId = chainId;
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      // Sign the message with the user
      const [userSignature] = await getSignaturesDelegatedUserDecryptRequest(eip712RequestMessage, [user]);

      // Request user decryption
      const requestTx = await decryptionManager
        .connect(user)
        .delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          delegationAccounts,
          contractsChainId,
          contractAddresses,
          publicKey,
          userSignature,
        );

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "UserDecryptionRequest")
        .withArgs(userDecryptionId, snsCiphertextMaterials, publicKey);
    });

    it("Should request a user decryption with a single ctHandleContractPair", async function () {
      const {
        decryptionManager,
        user,
        delegationAccounts,
        ctHandleContractPairs,
        publicKey,
        requestValidity,
        snsCiphertextMaterials,
        eip712RequestMessage,
      } = await loadFixture(deployDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractsChainId = chainId;
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      // Sign the message with the user
      const [userSignature] = await getSignaturesDelegatedUserDecryptRequest(eip712RequestMessage, [user]);

      // Request user decryption
      const requestTx = await decryptionManager
        .connect(user)
        .delegatedUserDecryptionRequest(
          [ctHandleContractPairs[0]],
          requestValidity,
          delegationAccounts,
          contractsChainId,
          contractAddresses,
          publicKey,
          userSignature,
        );

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "UserDecryptionRequest")
        .withArgs(userDecryptionId, [snsCiphertextMaterials[0]], publicKey);
    });

    it("Should request a user decryption with empty ctHandleContractPairs list", async function () {
      const {
        decryptionManager,
        user,
        delegationAccounts,
        ctHandleContractPairs,
        publicKey,
        requestValidity,
        eip712RequestMessage,
      } = await loadFixture(deployDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractsChainId = chainId;
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      // Sign the message with the user
      const [userSignature] = await getSignaturesDelegatedUserDecryptRequest(eip712RequestMessage, [user]);

      // Request user decryption with an empty list of ctHandleContractPairs
      const requestTx = await decryptionManager
        .connect(user)
        .delegatedUserDecryptionRequest(
          [],
          requestValidity,
          delegationAccounts,
          contractsChainId,
          contractAddresses,
          publicKey,
          userSignature,
        );

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "UserDecryptionRequest")
        .withArgs(userDecryptionId, [], publicKey);
    });

    it("Should revert because contractAddresses exceeds maximum length allowed", async function () {
      const { decryptionManager, user } = await loadFixture(deployDecryptionManagerFixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = [];
      for (let i = 0; i < 11; i++) {
        contractAddresses.push(hre.ethers.Wallet.createRandom().address);
      }

      // Check that the request fails because the given contractAddresses exceeds the maximum length allowed
      await expect(
        decryptionManager
          .connect(user)
          .delegatedUserDecryptionRequest(
            [],
            { durationDays: 120, startTimestamp: Date.now() },
            { userAddress: user.address, delegatedAddress: hre.ethers.Wallet.createRandom().address },
            chainId,
            contractAddresses,
            hre.ethers.randomBytes(32),
            hre.ethers.randomBytes(32),
          ),
      )
        .to.be.revertedWithCustomError(decryptionManager, "ContractAddressesMaxLengthExceeded")
        .withArgs(10, contractAddresses.length);
    });

    it("Should revert because durationDays exceeds maximum allowed", async function () {
      const { decryptionManager, user } = await loadFixture(deployDecryptionManagerFixture);

      // Create dummy input data for the user decryption request
      const requestValidity: IDecryptionManager.RequestValidityStruct = {
        durationDays: 400,
        startTimestamp: Date.now(),
      };

      // Check that the request fails because the given requestValidity.durationDays exceeds the maximum allowed
      await expect(
        decryptionManager
          .connect(user)
          .delegatedUserDecryptionRequest(
            [],
            requestValidity,
            { userAddress: user.address, delegatedAddress: hre.ethers.Wallet.createRandom().address },
            chainId,
            [],
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
      const delegatedAddress = hre.ethers.Wallet.createRandom().address;
      const ctHandleContractPairs: CtHandleContractPairStruct[] = [
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
          .delegatedUserDecryptionRequest(
            ctHandleContractPairs,
            requestValidity,
            { userAddress: user.address, delegatedAddress },
            contractsChainId,
            [contractAddress],
            publicKey,
            userSignature,
          ),
      )
        .to.be.revertedWithCustomError(aclManager, "AccountNotAllowedToUseCiphertext")
        .withArgs(ctHandles[0], delegatedAddress);
    });

    it("Should revert because of invalid EIP712 user request signature", async function () {
      const {
        decryptionManager,
        user,
        delegationAccounts,
        ctHandleContractPairs,
        publicKey,
        requestValidity,
        eip712RequestMessage,
      } = await loadFixture(deployDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);
      const fakeSigners = await hre.ethers.getSigners();

      // Sign the message with the user
      const [fakeSignature] = await getSignaturesDelegatedUserDecryptRequest(eip712RequestMessage, [
        fakeSigners.pop()!,
      ]);

      // Request user decryption
      const requestTx = decryptionManager
        .connect(user)
        .delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          delegationAccounts,
          chainId,
          contractAddresses,
          publicKey,
          fakeSignature,
        );

      // Check request event
      await expect(requestTx)
        .to.be.revertedWithCustomError(decryptionManager, "InvalidUserSignature")
        .withArgs(fakeSignature);
    });

    it("Should revert because contract in ctHandleContractPairs not included in contractAddresses list", async function () {
      const { decryptionManager, user, delegationAccounts, ctHandleContractPairs } = await loadFixture(
        deployAllowDelegatedUserDecryptionFixture,
      );

      // Create dummy input data for the user decryption request
      const contractAddresses = [hre.ethers.Wallet.createRandom().address];
      const publicKey = hre.ethers.randomBytes(32);
      const requestValidity: IDecryptionManager.RequestValidityStruct = {
        durationDays: 120,
        startTimestamp: Date.now(),
      };

      // Create EIP712 messages and get associated KMS nodes' signatures
      const decryptionManagerAddress = await decryptionManager.getAddress();
      const eip712RequestMessage = createEIP712RequestDelegatedUserDecrypt(
        chainId,
        decryptionManagerAddress,
        publicKey,
        ctHandleContractPairs.map((pair) => pair.contractAddress.toString()),
        delegationAccounts.delegatedAddress.toString(),
        chainId,
        requestValidity.startTimestamp.toString(),
        requestValidity.durationDays.toString(),
      );

      // Sign the message with the user
      const [userSignature] = await getSignaturesDelegatedUserDecryptRequest(eip712RequestMessage, [user]);

      // Request user decryption
      const requestTx = decryptionManager
        .connect(user)
        .delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          delegationAccounts,
          chainId,
          contractAddresses,
          publicKey,
          userSignature,
        );

      // Check request event
      await expect(requestTx)
        .to.be.revertedWithCustomError(decryptionManager, "ContractNotInContractAddresses")
        .withArgs(ctHandleContractPairs[0].contractAddress);
    });

    it("Should revert because of ctMaterials tied to different key IDs", async function () {
      const {
        keyManager,
        decryptionManager,
        ciphertextManager,
        aclManager,
        admins,
        coprocessorSigners,
        kmsSigners,
        user,
        keyId1,
        ctHandleContractPairs,
        delegationAccounts,
        requestValidity,
        publicKey,
        eip712RequestMessage,
        fheParamsName,
      } = await loadFixture(deployDelegatedUserDecryptEIP712Fixture);

      const keyId2 = await createAndRotateKey(
        keyId1,
        keyManager,
        admins,
        coprocessorSigners,
        kmsSigners,
        fheParamsName,
      );

      // Define ciphertext dummy values
      const ciphertextDigest = hre.ethers.hexlify(hre.ethers.randomBytes(32));
      const snsCiphertextDigest = hre.ethers.hexlify(hre.ethers.randomBytes(32));
      const ctHandleContractPair = {
        ctHandle: 2050,
        contractAddress: ctHandleContractPairs[0].contractAddress,
      };

      // Store the ciphertext and allow public decryption
      for (let i = 0; i < coprocessorSigners.length; i++) {
        await ciphertextManager
          .connect(coprocessorSigners[i])
          .addCiphertextMaterial(ctHandleContractPair.ctHandle, keyId2, chainId, ciphertextDigest, snsCiphertextDigest);
      }
      for (let i = 0; i < coprocessorSigners.length; i++) {
        await aclManager
          .connect(coprocessorSigners[i])
          .allowAccount(chainId, ctHandleContractPair.ctHandle, delegationAccounts.delegatedAddress);
        await aclManager
          .connect(coprocessorSigners[i])
          .allowAccount(chainId, ctHandleContractPair.ctHandle, ctHandleContractPair.contractAddress);
      }

      // Create dummy input data for the user decryption request
      const contractsChainId = chainId;
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      // Sign the message with the user
      const [userSignature] = await getSignaturesDelegatedUserDecryptRequest(eip712RequestMessage, [user]);

      // Request user decryption with ctMaterials tied to different key IDs
      const requestTx = decryptionManager
        .connect(user)
        .delegatedUserDecryptionRequest(
          [ctHandleContractPair, ...ctHandleContractPairs.slice(1)],
          requestValidity,
          delegationAccounts,
          contractsChainId,
          contractAddresses,
          publicKey,
          userSignature,
        );

      // Check that different key IDs are not allowed for batched user decryption
      await expect(requestTx)
        .to.revertedWithCustomError(decryptionManager, "DifferentKeyIdsNotAllowed")
        .withArgs(keyId1);
    });

    it("Should reach consensus with 3 valid responses", async function () {
      const {
        decryptionManager,
        user,
        ctHandleContractPairs,
        requestValidity,
        delegationAccounts,
        publicKey,
        kmsSigners,
        eip712RequestMessage,
        eip712ResponseMessage,
      } = await loadFixture(deployDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractsChainId = chainId;
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      // Sign the message with the user
      const [userSignature] = await getSignaturesDelegatedUserDecryptRequest(eip712RequestMessage, [user]);

      // Request user decryption
      await decryptionManager
        .connect(user)
        .delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          delegationAccounts,
          contractsChainId,
          contractAddresses,
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
        ctHandleContractPairs,
        requestValidity,
        delegationAccounts,
        kmsSigners,
        eip712RequestMessage,
        eip712ResponseMessage,
      } = await loadFixture(deployDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractsChainId = chainId;
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      // Sign the message with the user
      const [userSignature] = await getSignaturesDelegatedUserDecryptRequest(eip712RequestMessage, [user]);

      // Request user decryption
      await decryptionManager
        .connect(user)
        .delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          delegationAccounts,
          contractsChainId,
          contractAddresses,
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
