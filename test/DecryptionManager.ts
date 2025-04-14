import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { BigNumberish, EventLog, HDNodeWallet, Wallet } from "ethers";
import hre from "hardhat";

import {
  ACLManager,
  CiphertextManager,
  DecryptionManager,
  HTTPZ,
  IDecryptionManager,
  KeyManager,
} from "../typechain-types";
// The type needs to be imported separately because it is not properly detected by the linter
// as this type is defined as a shared structs instead of directly in the IDecryptionManager interface
import {
  CtHandleContractPairStruct,
  DelegationAccountsStruct,
  SnsCiphertextMaterialStruct,
} from "../typechain-types/contracts/interfaces/IDecryptionManager";
import {
  EIP712,
  createAndFundRandomUser,
  createBytes32,
  createCtHandlesWithChainId,
  createEIP712RequestDelegatedUserDecrypt,
  createEIP712RequestUserDecrypt,
  createEIP712ResponsePublicDecrypt,
  createEIP712ResponseUserDecrypt,
  getSignaturesDelegatedUserDecryptRequest,
  getSignaturesPublicDecrypt,
  getSignaturesUserDecryptRequest,
  getSignaturesUserDecryptResponse,
  loadChainIds,
  loadTestVariablesFixture,
  toValues,
} from "./utils";

describe("DecryptionManager", function () {
  // Define the host chainId(s)
  const hostChainIds = loadChainIds();
  const hostChainId = hostChainIds[0];

  // Create 3 dummy ciphertext handles with the host chain ID
  const ctHandles = createCtHandlesWithChainId(3, hostChainId);
  const ctHandle = ctHandles[0];
  // Define the gateway chain ID
  const chainId = hre.network.config.chainId!;

  // Define input values
  const ciphertextDigest = createBytes32();
  const snsCiphertextDigest = createBytes32();

  // Define new valid ctHandles
  const newCtHandles = createCtHandlesWithChainId(3, hostChainId);
  const newCtHandle = newCtHandles[0];

  let httpz: HTTPZ;
  let keyManager: KeyManager;
  let aclManager: ACLManager;
  let ciphertextManager: CiphertextManager;
  let decryptionManager: DecryptionManager;
  let owner: Wallet;
  let user: HDNodeWallet;
  let snsCiphertextMaterials: SnsCiphertextMaterialStruct[];
  let kmsSignatures: string[];
  let kmsTxSenders: HardhatEthersSigner[];
  let coprocessorTxSenders: HardhatEthersSigner[];
  let fakeTxSender: HDNodeWallet;
  let fakeSigner: HDNodeWallet;
  let keyId1: BigNumberish;
  let fheParamsName: string;

  // Trigger a key generation in KeyManager contract and activate the key
  async function prepareWithActivatedKeyFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const { keyManager, owner, kmsTxSenders, coprocessorTxSenders, fheParamsName } = fixtureData;

    // Trigger a preprocessing keygen request
    const txRequest = await keyManager.connect(owner).preprocessKeygenRequest(fheParamsName);

    // Get the preKeyRequestId from the event in the transaction receipt
    const receipt = await txRequest.wait();
    const event = receipt?.logs[0] as EventLog;
    const preKeyRequestId = Number(event?.args[0]);

    // Define a preKeyId for the preprocessing keygen response
    const preKeyId = 1;

    // Trigger preprocessing keygen responses for all KMS nodes
    for (let i = 0; i < kmsTxSenders.length; i++) {
      await keyManager.connect(kmsTxSenders[i]).preprocessKeygenResponse(preKeyRequestId, preKeyId);
    }

    // Trigger a keygen request
    await keyManager.connect(owner).keygenRequest(preKeyId);

    // Define a keyId for keygen response
    const keyId1 = 1;

    // Trigger keygen responses for all KMS nodes
    for (let i = 0; i < kmsTxSenders.length; i++) {
      await keyManager.connect(kmsTxSenders[i]).keygenResponse(preKeyId, keyId1);
    }

    // Request activation of the key
    await keyManager.connect(owner).activateKeyRequest(keyId1);

    // Trigger activation responses for all coprocessors
    for (let i = 0; i < coprocessorTxSenders.length; i++) {
      await keyManager.connect(coprocessorTxSenders[i]).activateKeyResponse(keyId1);
    }

    return { ...fixtureData, keyId1 };
  }

  // Add SNS ciphertext materials associated to the handles
  async function prepareAddCiphertextFixture() {
    const fixtureData = await loadFixture(prepareWithActivatedKeyFixture);
    const { ciphertextManager, coprocessorTxSenders, keyId1 } = fixtureData;

    let snsCiphertextMaterials: SnsCiphertextMaterialStruct[] = [];

    // Allow public decryption
    for (const ctHandle of ctHandles) {
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await ciphertextManager
          .connect(coprocessorTxSenders[i])
          .addCiphertextMaterial(ctHandle, keyId1, ciphertextDigest, snsCiphertextDigest);
      }

      // Store the SNS ciphertext materials for event checks
      snsCiphertextMaterials.push({
        ctHandle,
        keyId: keyId1,
        snsCiphertextDigest,
        coprocessorTxSenderAddresses: coprocessorTxSenders.map((s) => s.address),
      });
    }

    return { ...fixtureData, snsCiphertextMaterials };
  }

  // Create a new key, rotate it and activate it. It returns the new key ID.
  async function createAndRotateKey(
    sourceKeyId: BigNumberish,
    keyManager: KeyManager,
    owner: Wallet,
    coprocessorTxSenders: HardhatEthersSigner[],
    kmsTxSenders: HardhatEthersSigner[],
    fheParamsName: string,
  ): Promise<BigNumberish> {
    const newKeyId = hre.ethers.toBigInt(hre.ethers.randomBytes(32));
    // Trigger a preprocessing keygen request
    let txRequest = await keyManager.connect(owner).preprocessKeygenRequest(fheParamsName);

    // Get the preKeyRequestId from the event in the transaction receipt
    let receipt = await txRequest.wait();
    let event = receipt?.logs[0] as EventLog;
    const preKeyRequestId = Number(event?.args[0]);

    // Define a preKeyId for the preprocessing keygen response
    const preKeyId = hre.ethers.toBigInt(hre.ethers.randomBytes(32));

    // Trigger preprocessing keygen responses for all KMS nodes
    for (let i = 0; i < kmsTxSenders.length; i++) {
      await keyManager.connect(kmsTxSenders[i]).preprocessKeygenResponse(preKeyRequestId, preKeyId);
    }

    // Trigger a keygen request
    await keyManager.connect(owner).keygenRequest(preKeyId);

    // Trigger keygen responses for all KMS nodes
    for (let i = 0; i < kmsTxSenders.length; i++) {
      await keyManager.connect(kmsTxSenders[i]).keygenResponse(preKeyId, newKeyId);
    }

    // Trigger a preprocessing kskgen request
    txRequest = await keyManager.connect(owner).preprocessKskgenRequest(fheParamsName);

    // Get the preKeyRequestId from the event in the transaction receipt
    receipt = await txRequest.wait();
    event = receipt?.logs[0] as EventLog;
    const preKskRequestId = Number(event?.args[0]);

    // Define a preKskId for the preprocessing kskgen response
    const preKskId = hre.ethers.toBigInt(hre.ethers.randomBytes(32));

    // Trigger preprocessing kskgen responses for all KMS nodes
    for (let i = 0; i < kmsTxSenders.length; i++) {
      await keyManager.connect(kmsTxSenders[i]).preprocessKskgenResponse(preKskRequestId, preKskId);
    }

    // Trigger a kskgen request
    await keyManager.connect(owner).kskgenRequest(preKskId, sourceKeyId, newKeyId);

    // Define a kskId for kskgen response
    const kskId = hre.ethers.toBigInt(hre.ethers.randomBytes(32));

    // Trigger kskgen responses for all KMS nodes
    for (let i = 0; i < kmsTxSenders.length; i++) {
      await keyManager.connect(kmsTxSenders[i]).kskgenResponse(preKskId, kskId);
    }

    // Request activation of the key
    await keyManager.connect(owner).activateKeyRequest(newKeyId);

    // Trigger activation responses for all coprocessors
    for (let i = 0; i < coprocessorTxSenders.length; i++) {
      await keyManager.connect(coprocessorTxSenders[i]).activateKeyResponse(newKeyId);
    }

    return newKeyId;
  }

  describe("Public Decryption", function () {
    // Expected public decryption id (after first request)
    const publicDecryptionId = 1;

    // Create input values
    const decryptedResult = hre.ethers.randomBytes(32);
    const dummySignature = hre.ethers.randomBytes(32);

    // Allow handles for public decryption
    async function prepareAllowPublicDecryptionFixture() {
      const fixtureData = await loadFixture(prepareAddCiphertextFixture);
      const { coprocessorTxSenders, aclManager } = fixtureData;

      // Allow public decryption
      for (const ctHandle of ctHandles) {
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await aclManager.connect(coprocessorTxSenders[i]).allowPublicDecrypt(ctHandle);
        }
      }

      return fixtureData;
    }

    async function prepareGetEIP712Fixture() {
      const fixtureData = await loadFixture(prepareAllowPublicDecryptionFixture);

      // Create EIP712 messages
      const decryptionManagerAddress = await fixtureData.decryptionManager.getAddress();
      const eip712Message = createEIP712ResponsePublicDecrypt(
        chainId,
        decryptionManagerAddress,
        ctHandles,
        decryptedResult,
      );

      return { ...fixtureData, eip712Message };
    }

    it("Should request a public decryption with multiple ctHandles", async function () {
      const { decryptionManager, user, snsCiphertextMaterials } = await loadFixture(
        prepareAllowPublicDecryptionFixture,
      );

      // Request public decryption (any user can do so)
      const requestTx = await decryptionManager.connect(user).publicDecryptionRequest(ctHandles);

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "PublicDecryptionRequest")
        .withArgs(publicDecryptionId, toValues(snsCiphertextMaterials));
    });

    it("Should request a public decryption with a single ctHandle", async function () {
      const { decryptionManager, user, snsCiphertextMaterials } = await loadFixture(
        prepareAllowPublicDecryptionFixture,
      );

      // Request public decryption with a single ctHandle
      const requestTx = await decryptionManager.connect(user).publicDecryptionRequest([ctHandles[0]]);

      const singleSnsCiphertextMaterials = snsCiphertextMaterials.slice(0, 1);

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "PublicDecryptionRequest")
        .withArgs(publicDecryptionId, toValues(singleSnsCiphertextMaterials));
    });

    it("Should request a public decryption with empty ctHandles list", async function () {
      const { decryptionManager, user } = await loadFixture(prepareAllowPublicDecryptionFixture);

      // Request public decryption with an empty list of ctHandles
      const requestTx = await decryptionManager.connect(user).publicDecryptionRequest([]);

      // Check request event
      await expect(requestTx).to.emit(decryptionManager, "PublicDecryptionRequest").withArgs(publicDecryptionId, []);
    });

    it("Should revert because handles are not allowed for public decryption", async function () {
      const { aclManager, decryptionManager, user } = await loadFixture(loadTestVariablesFixture);

      // Check that the request fails because the handles are not allowed for public decryption
      await expect(decryptionManager.publicDecryptionRequest(newCtHandles))
        .to.be.revertedWithCustomError(aclManager, "PublicDecryptNotAllowed")
        .withArgs(newCtHandles[0]);
    });

    it("Should revert because ciphertext material has not been added", async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      const { ciphertextManager, decryptionManager, coprocessorTxSenders, aclManager } = fixtureData;

      // Allow public decryption for handles that have not been added
      // We need to do this because `checkPublicDecryptionReady` first checks if the handles
      // have been allowed for public decryption
      for (const newCtHandle of newCtHandles) {
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await aclManager.connect(coprocessorTxSenders[i]).allowPublicDecrypt(newCtHandle);
        }
      }

      // Check that the request fails because the ciphertext material is unavailable
      await expect(decryptionManager.publicDecryptionRequest(newCtHandles))
        .to.be.revertedWithCustomError(ciphertextManager, "CiphertextMaterialNotFound")
        .withArgs(newCtHandles[0]);
    });

    it("Should revert because the message sender is not a KMS transaction sender", async function () {
      const { httpz, decryptionManager } = await loadFixture(prepareGetEIP712Fixture);

      const fakeTxSender = await createAndFundRandomUser();

      // Check that the transaction fails because the msg.sender is not a registered KMS transaction sender
      await expect(
        decryptionManager
          .connect(fakeTxSender)
          .publicDecryptionResponse(publicDecryptionId, decryptedResult, dummySignature),
      )
        .to.be.revertedWithCustomError(httpz, "NotKmsTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should revert because the signer is not a KMS signer", async function () {
      const { httpz, decryptionManager, user, kmsTxSenders, eip712Message } =
        await loadFixture(prepareGetEIP712Fixture);

      // Request public decryption
      // This step is necessary, else the publicDecryptionId won't be set in the state and the
      // signature verification will use wrong handles
      await decryptionManager.connect(user).publicDecryptionRequest(ctHandles);

      const fakeSigner = await createAndFundRandomUser();

      // Create a fake signature from the fake signer
      const [fakeSignature] = await getSignaturesPublicDecrypt(eip712Message, [fakeSigner]);

      // Check that the signature verification fails because the signer is not a registered KMS signer
      await expect(
        decryptionManager
          .connect(kmsTxSenders[0])
          .publicDecryptionResponse(publicDecryptionId, decryptedResult, fakeSignature),
      )
        .to.be.revertedWithCustomError(httpz, "NotKmsSigner")
        .withArgs(fakeSigner.address);
    });

    it("Should revert because of two responses with same signature", async function () {
      const { decryptionManager, kmsTxSenders, kmsSigners, user, eip712Message } =
        await loadFixture(prepareGetEIP712Fixture);

      const firstKmsTxSender = kmsTxSenders[0];
      const firstKmsSigner = kmsSigners[0];

      // Request public decryption
      await decryptionManager.connect(user).publicDecryptionRequest(ctHandles);

      // Sign the message with the first KMS node and get its signature
      const [signature1] = await getSignaturesPublicDecrypt(eip712Message, [firstKmsSigner]);

      // Trigger a first public decryption response
      await decryptionManager
        .connect(firstKmsTxSender)
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature1);

      // Check that a KMS node cannot sign a second time for the same public decryption
      await expect(
        decryptionManager
          .connect(firstKmsTxSender)
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
        owner,
        kmsTxSenders,
        coprocessorTxSenders,
        user,
        keyId1,
        fheParamsName,
      } = await loadFixture(prepareAllowPublicDecryptionFixture);

      const keyId2 = await createAndRotateKey(
        keyId1,
        keyManager,
        owner,
        coprocessorTxSenders,
        kmsTxSenders,
        fheParamsName,
      );

      // Store the ciphertext and allow public decryption
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await ciphertextManager
          .connect(coprocessorTxSenders[i])
          .addCiphertextMaterial(newCtHandle, keyId2, ciphertextDigest, snsCiphertextDigest);
      }
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await aclManager.connect(coprocessorTxSenders[i]).allowPublicDecrypt(newCtHandle);
      }

      // Request public decryption with ctMaterials tied to different key IDs
      const requestTx = decryptionManager.connect(user).publicDecryptionRequest([...ctHandles, newCtHandle]);

      // Check that different key IDs are not allowed for batched public decryption
      await expect(requestTx)
        .to.revertedWithCustomError(decryptionManager, "DifferentKeyIdsNotAllowed")
        .withArgs(keyId2);
    });

    it("Should reach consensus with 2 valid responses", async function () {
      const { decryptionManager, kmsTxSenders, kmsSigners, user, eip712Message } =
        await loadFixture(prepareGetEIP712Fixture);

      // Request public decryption
      await decryptionManager.connect(user).publicDecryptionRequest(ctHandles);

      // Sign the message with all KMS signers and get the first 2 signatures
      const [signature1, signature2] = await getSignaturesPublicDecrypt(eip712Message, kmsSigners);

      // Trigger two valid public decryption responses
      await decryptionManager
        .connect(kmsTxSenders[0])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature1);

      const responseTx2 = await decryptionManager
        .connect(kmsTxSenders[1])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature2);

      // Consensus should be reached at the second response
      // Check 2nd response event: it should only contain 2 valid signatures
      await expect(responseTx2)
        .to.emit(decryptionManager, "PublicDecryptionResponse")
        .withArgs(publicDecryptionId, decryptedResult, [signature1, signature2]);

      // Check that the public decryption is done
      await expect(decryptionManager.checkPublicDecryptionDone(publicDecryptionId)).to.not.be.reverted;
    });

    it("Should ignore other valid responses", async function () {
      const { decryptionManager, kmsTxSenders, kmsSigners, user, eip712Message } =
        await loadFixture(prepareGetEIP712Fixture);

      // Request public decryption
      await decryptionManager.connect(user).publicDecryptionRequest(ctHandles);

      // Sign the message with all KMS signers and get their signatures
      const [signature1, signature2, signature3, signature4] = await getSignaturesPublicDecrypt(
        eip712Message,
        kmsSigners,
      );

      // Trigger four valid public decryption responses
      const responseTx1 = await decryptionManager
        .connect(kmsTxSenders[0])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature1);

      await decryptionManager
        .connect(kmsTxSenders[1])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature2);

      const responseTx3 = await decryptionManager
        .connect(kmsTxSenders[2])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature3);

      const responseTx4 = await decryptionManager
        .connect(kmsTxSenders[3])
        .publicDecryptionResponse(publicDecryptionId, decryptedResult, signature4);

      // Check that the 1st, 3rd and 4th responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet
      // - 3rd and 4th responses are ignored (not reverted) even though they are late
      await expect(responseTx1).to.not.emit(decryptionManager, "PublicDecryptionResponse");
      await expect(responseTx3).to.not.emit(decryptionManager, "PublicDecryptionResponse");
      await expect(responseTx4).to.not.emit(decryptionManager, "PublicDecryptionResponse");
    });

    describe("Checks", function () {
      it("Should not revert because public decryption is ready", async function () {
        const { decryptionManager } = await loadFixture(prepareGetEIP712Fixture);

        await expect(decryptionManager.checkPublicDecryptionReady(ctHandles)).to.not.be.reverted;
      });

      it("Should revert because handles have not been allowed for public decryption", async function () {
        const fixtureData = await loadFixture(loadTestVariablesFixture);
        const { aclManager, decryptionManager } = fixtureData;

        await expect(decryptionManager.checkPublicDecryptionReady(newCtHandles))
          .to.be.revertedWithCustomError(aclManager, "PublicDecryptNotAllowed")
          .withArgs(newCtHandles[0]);
      });

      it("Should revert because ciphertext material has not been added", async function () {
        const fixtureData = await loadFixture(loadTestVariablesFixture);
        const { ciphertextManager, decryptionManager, coprocessorTxSenders, aclManager } = fixtureData;

        // Allow public decryption for handles that have not been added
        // We need to do this because `checkPublicDecryptionReady` first checks if the handles
        // have been allowed for public decryption
        for (const newCtHandle of newCtHandles) {
          for (let i = 0; i < coprocessorTxSenders.length; i++) {
            await aclManager.connect(coprocessorTxSenders[i]).allowPublicDecrypt(newCtHandle);
          }
        }

        await expect(decryptionManager.checkPublicDecryptionReady(newCtHandles))
          .to.be.revertedWithCustomError(ciphertextManager, "CiphertextMaterialNotFound")
          .withArgs(newCtHandles[0]);
      });

      it("Should revert because the public decryption is not done", async function () {
        const { decryptionManager } = await loadFixture(prepareGetEIP712Fixture);

        await expect(decryptionManager.checkPublicDecryptionDone(publicDecryptionId))
          .to.be.revertedWithCustomError(decryptionManager, "PublicDecryptionNotDone")
          .withArgs(publicDecryptionId);
      });
    });
  });

  describe("User Decryption", function () {
    let userSignature: string;
    let fakeUserAddress: string;
    let eip712RequestMessage: EIP712;
    let eip712ResponseMessage: EIP712;

    // Expected user decryption id (after first request)
    const userDecryptionId = 1;

    // Create input values
    const reencryptedShare = hre.ethers.randomBytes(32);
    const contractAddress = hre.ethers.Wallet.createRandom().address;
    const contractAddresses = [contractAddress];
    const publicKey = hre.ethers.randomBytes(32);
    const ctHandleContractPairs: CtHandleContractPairStruct[] = ctHandles.map((ctHandle) => ({
      contractAddress,
      ctHandle,
    }));
    const requestValidity: IDecryptionManager.RequestValidityStruct = {
      durationDays: 120,
      startTimestamp: Date.now(),
    };

    // Define new valid inputs
    const newCtHandleContractPairs: CtHandleContractPairStruct[] = newCtHandles.map((newCtHandle) => ({
      contractAddress,
      ctHandle: newCtHandle,
    }));

    // Allow access the the handles for the user and the contract
    async function prepareAllowAccountFixture() {
      const fixtureData = await loadFixture(prepareAddCiphertextFixture);
      const { coprocessorTxSenders, aclManager, user } = fixtureData;

      // Allow user decryption for the user and contract address over all handles
      for (const ctHandle of ctHandles) {
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await aclManager.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, user.address);
          await aclManager.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, contractAddress);
        }
      }

      return fixtureData;
    }

    async function prepareUserDecryptEIP712Fixture() {
      const fixtureData = await loadFixture(prepareAllowAccountFixture);
      const { decryptionManager, user, kmsSigners } = fixtureData;

      // Create EIP712 messages
      const decryptionManagerAddress = await decryptionManager.getAddress();
      const eip712RequestMessage = createEIP712RequestUserDecrypt(
        chainId,
        decryptionManagerAddress,
        publicKey,
        contractAddresses,
        hostChainId,
        requestValidity.startTimestamp.toString(),
        requestValidity.durationDays.toString(),
      );

      // Sign the message with the user
      const [userSignature] = await getSignaturesUserDecryptRequest(eip712RequestMessage, [user]);

      const eip712ResponseMessage = createEIP712ResponseUserDecrypt(
        chainId,
        decryptionManagerAddress,
        publicKey,
        ctHandles,
        reencryptedShare,
      );

      // Sign the message with all KMS signers
      const kmsSignatures = await getSignaturesUserDecryptResponse(eip712ResponseMessage, kmsSigners);

      return {
        ...fixtureData,
        eip712RequestMessage,
        eip712ResponseMessage,
        userSignature,
        kmsSignatures,
      };
    }

    beforeEach(async function () {
      // Initialize globally used variables before each test
      const fixtureData = await loadFixture(prepareUserDecryptEIP712Fixture);
      httpz = fixtureData.httpz;
      keyManager = fixtureData.keyManager;
      aclManager = fixtureData.aclManager;
      ciphertextManager = fixtureData.ciphertextManager;
      decryptionManager = fixtureData.decryptionManager;
      owner = fixtureData.owner;
      user = fixtureData.user;
      snsCiphertextMaterials = fixtureData.snsCiphertextMaterials;
      userSignature = fixtureData.userSignature;
      kmsSignatures = fixtureData.kmsSignatures;
      kmsTxSenders = fixtureData.kmsTxSenders;
      coprocessorTxSenders = fixtureData.coprocessorTxSenders;
      eip712RequestMessage = fixtureData.eip712RequestMessage;
      eip712ResponseMessage = fixtureData.eip712ResponseMessage;
      keyId1 = fixtureData.keyId1;
      fheParamsName = fixtureData.fheParamsName;

      fakeTxSender = await createAndFundRandomUser();
      fakeSigner = await createAndFundRandomUser();
      fakeUserAddress = (await createAndFundRandomUser()).address;
    });

    it("Should request a user decryption with multiple ctHandleContractPairs", async function () {
      // Request user decryption
      const requestTx = await decryptionManager.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        hostChainId,
        contractAddresses,
        user.address,
        publicKey,
        userSignature,
      );

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "UserDecryptionRequest")
        .withArgs(userDecryptionId, toValues(snsCiphertextMaterials), user.address, publicKey);
    });

    it("Should request a user decryption with a single ctHandleContractPair", async function () {
      // Create single list of inputs
      const singleCtHandleContractPair: CtHandleContractPairStruct[] = ctHandleContractPairs.slice(0, 1);
      const singleSnsCiphertextMaterials = snsCiphertextMaterials.slice(0, 1);

      // Request user decryption
      const requestTx = await decryptionManager
        .connect(user)
        .userDecryptionRequest(
          singleCtHandleContractPair,
          requestValidity,
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        );

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "UserDecryptionRequest")
        .withArgs(userDecryptionId, toValues(singleSnsCiphertextMaterials), user.address, publicKey);
    });

    it("Should request a user decryption with empty ctHandleContractPairs list", async function () {
      // Create dummy input data for the user decryption request
      const emptyCtHandleContractPairs: CtHandleContractPairStruct[] = [];

      // Request user decryption with an empty list of ctHandleContractPairs
      const requestTx = await decryptionManager
        .connect(user)
        .userDecryptionRequest(
          emptyCtHandleContractPairs,
          requestValidity,
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        );

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "UserDecryptionRequest")
        .withArgs(userDecryptionId, toValues(emptyCtHandleContractPairs), user.address, publicKey);
    });

    it("Should revert because contract addresses exceeds maximum length allowed", async function () {
      // Create dummy input data for the user decryption request
      const largeContractAddresses = [];
      for (let i = 0; i < 11; i++) {
        largeContractAddresses.push(hre.ethers.Wallet.createRandom().address);
      }

      // Check that the request fails because the given contract addresses exceeds the maximum length allowed
      await expect(
        decryptionManager.userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          hostChainId,
          largeContractAddresses,
          user.address,
          publicKey,
          userSignature,
        ),
      )
        .to.be.revertedWithCustomError(decryptionManager, "ContractAddressesMaxLengthExceeded")
        .withArgs(10, largeContractAddresses.length);
    });

    it("Should revert because durationDays exceeds maximum allowed", async function () {
      // Create a fake input data with a durationDays that exceeds the maximum allowed (currently: 365 days)
      const durationDays = 400;
      const fakeRequestValidity: IDecryptionManager.RequestValidityStruct = {
        durationDays,
        startTimestamp: Date.now(),
      };

      // Check that the request fails because the durationDays exceeds the maximum allowed
      await expect(
        decryptionManager.userDecryptionRequest(
          ctHandleContractPairs,
          fakeRequestValidity,
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        ),
      )
        .to.be.revertedWithCustomError(decryptionManager, "MaxDurationDaysExceeded")
        .withArgs(365, durationDays);
    });

    it("Should revert because the user address is a contract address", async function () {
      // Define fake ctHandleContractPairs with user address as contract address
      const fakeCtHandleContractPairs: CtHandleContractPairStruct[] = [
        {
          contractAddress: user.address,
          ctHandle,
        },
      ];

      const fakeContractAddresses = [user.address];

      await expect(
        decryptionManager.userDecryptionRequest(
          fakeCtHandleContractPairs,
          requestValidity,
          hostChainId,
          fakeContractAddresses,
          user.address,
          publicKey,
          userSignature,
        ),
      )
        .to.be.revertedWithCustomError(decryptionManager, "UserAddressInContractAddresses")
        .withArgs(user.address, fakeContractAddresses);
    });

    it("Should revert because the user is not allowed for user decryption on a ctHandle", async function () {
      await expect(
        decryptionManager.userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          hostChainId,
          contractAddresses,
          fakeUserAddress,
          publicKey,
          userSignature,
        ),
      )
        .to.be.revertedWithCustomError(aclManager, "AccountNotAllowedToUseCiphertext")
        .withArgs(fakeUserAddress, ctHandleContractPairs[0].ctHandle);
    });

    it("Should revert because a contract is not allowed for user decryption on a ctHandle", async function () {
      // Define fake ctHandleContractPairs with a contract address that is not allowed for user decryption
      const fakeContractAddress = hre.ethers.Wallet.createRandom().address;
      const fakeCtHandleContractPairs: CtHandleContractPairStruct[] = [
        {
          contractAddress: fakeContractAddress,
          ctHandle,
        },
      ];

      await expect(
        decryptionManager.userDecryptionRequest(
          fakeCtHandleContractPairs,
          requestValidity,
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        ),
      )
        .to.be.revertedWithCustomError(aclManager, "AccountNotAllowedToUseCiphertext")
        .withArgs(fakeContractAddress, fakeCtHandleContractPairs[0].ctHandle);
    });

    it("Should revert because ciphertext material has not been added", async function () {
      // Allow public decryption for handles that have not been added
      // We need to do this because `checkPublicDecryptionReady` first checks if the handles
      // have been allowed for public decryption
      for (const newCtHandle of newCtHandles) {
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await aclManager.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, user.address);
          await aclManager.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, contractAddress);
        }
      }

      await expect(
        decryptionManager.userDecryptionRequest(
          newCtHandleContractPairs,
          requestValidity,
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        ),
      )
        .to.be.revertedWithCustomError(ciphertextManager, "CiphertextMaterialNotFound")
        .withArgs(newCtHandleContractPairs[0].ctHandle);
    });

    it("Should revert because of invalid EIP712 user request signature", async function () {
      // Sign the message with the user
      const [fakeSignature] = await getSignaturesUserDecryptRequest(eip712RequestMessage, [fakeSigner]);

      // Request user decryption
      const requestTx = decryptionManager
        .connect(user)
        .userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          hostChainId,
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

    it("Should revert because the response signer is not a registered KMS signer", async function () {
      // Request user decryption
      // This step is necessary, else the publicDecryptionId won't be set in the state and the
      // signature verification will use wrong handles
      await decryptionManager.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        hostChainId,
        contractAddresses,
        user.address,
        publicKey,
        userSignature,
      );

      // Create a fake signature from the fake signer
      const [fakeSignature] = await getSignaturesUserDecryptResponse(eip712ResponseMessage, [fakeSigner]);

      // Check that the transaction fails because the signer is not a registered KMS signer
      await expect(
        decryptionManager
          .connect(kmsTxSenders[0])
          .userDecryptionResponse(userDecryptionId, reencryptedShare, fakeSignature),
      )
        .to.be.revertedWithCustomError(httpz, "NotKmsSigner")
        .withArgs(fakeSigner.address);
    });

    it("Should revert because the message sender is not a KMS transaction sender", async function () {
      // Check that the transaction fails because the msg.sender is not a registered KMS transaction sender
      await expect(
        decryptionManager
          .connect(fakeTxSender)
          .userDecryptionResponse(userDecryptionId, reencryptedShare, userSignature),
      )
        .to.be.revertedWithCustomError(httpz, "NotKmsTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should revert because contract in ctHandleContractPairs not included in contractAddresses list", async function () {
      // Create a fake contract address list
      const fakeContractAddresses = [(await createAndFundRandomUser()).address];

      // Create EIP712 message using the fake contract address list
      const decryptionManagerAddress = await decryptionManager.getAddress();
      const eip712RequestMessage = createEIP712RequestUserDecrypt(
        chainId,
        decryptionManagerAddress,
        publicKey,
        fakeContractAddresses,
        hostChainId,
        requestValidity.startTimestamp.toString(),
        requestValidity.durationDays.toString(),
      );

      // Sign the message with the user
      const [userSignature] = await getSignaturesUserDecryptRequest(eip712RequestMessage, [user]);

      // Request user decryption
      const requestTx = decryptionManager.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        hostChainId,
        fakeContractAddresses,
        user.address,
        publicKey,
        userSignature,
      );

      // Check that the request fails because the contract address is not included in the contractAddresses list
      await expect(requestTx)
        .to.be.revertedWithCustomError(decryptionManager, "ContractNotInContractAddresses")
        .withArgs(contractAddress, fakeContractAddresses);
    });

    it("Should revert because of ctMaterials tied to different key IDs", async function () {
      const keyId2 = await createAndRotateKey(
        keyId1,
        keyManager,
        owner,
        coprocessorTxSenders,
        kmsTxSenders,
        fheParamsName,
      );

      // Store the ciphertext and allow public decryption
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await ciphertextManager
          .connect(coprocessorTxSenders[i])
          .addCiphertextMaterial(newCtHandle, keyId2, ciphertextDigest, snsCiphertextDigest);
      }
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await aclManager.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, user.address);
        await aclManager.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, contractAddress);
      }

      // Create a fake input containing 2 handles tied to different key IDs
      const fakeCtHandleContractPairs: CtHandleContractPairStruct[] = [
        {
          contractAddress,
          ctHandle: ctHandles[0],
        },
        {
          contractAddress,
          ctHandle: newCtHandle,
        },
      ];

      // Sign the message with the user
      const [userSignature] = await getSignaturesUserDecryptRequest(eip712RequestMessage, [user]);

      // Request user decryption with ctMaterials tied to different key IDs
      const requestTx = decryptionManager.userDecryptionRequest(
        fakeCtHandleContractPairs,
        requestValidity,
        hostChainId,
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
      // Request user decryption
      await decryptionManager
        .connect(user)
        .userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        );

      // Trigger three valid user decryption responses using different KMS transaction senders
      await decryptionManager
        .connect(kmsTxSenders[0])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, kmsSignatures[0]);

      await decryptionManager
        .connect(kmsTxSenders[1])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, kmsSignatures[1]);

      const responseTx3 = await decryptionManager
        .connect(kmsTxSenders[2])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, kmsSignatures[2]);

      // Consensus should be reached at the third response (reconstruction threshold)
      // Check 3rd response event: it should only contain 3 valid signatures
      await expect(responseTx3)
        .to.emit(decryptionManager, "UserDecryptionResponse")
        .withArgs(
          userDecryptionId,
          [reencryptedShare, reencryptedShare, reencryptedShare],
          [kmsSignatures[0], kmsSignatures[1], kmsSignatures[2]],
        );

      // Check that the user decryption is done
      await expect(decryptionManager.checkUserDecryptionDone(userDecryptionId)).to.not.be.reverted;
    });

    it("Should ignore other valid responses", async function () {
      // Request user decryption
      await decryptionManager
        .connect(user)
        .userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        );

      // Trigger three valid user decryption responses using different KMS transaction senders
      const responseTx1 = await decryptionManager
        .connect(kmsTxSenders[0])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, kmsSignatures[0]);

      const responseTx2 = await decryptionManager
        .connect(kmsTxSenders[1])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, kmsSignatures[1]);

      await decryptionManager
        .connect(kmsTxSenders[2])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, kmsSignatures[2]);

      const responseTx4 = await decryptionManager
        .connect(kmsTxSenders[3])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, kmsSignatures[3]);

      // Check that the 1st, 2nd and 4th responses do not emit an event:
      // - 1st and 2nd responses are ignored because consensus is not reached yet
      // - 4th response is ignored (not reverted) even though they are late
      await expect(responseTx1).to.not.emit(decryptionManager, "UserDecryptionResponse");
      await expect(responseTx2).to.not.emit(decryptionManager, "UserDecryptionResponse");
      await expect(responseTx4).to.not.emit(decryptionManager, "UserDecryptionResponse");
    });

    describe("Checks", function () {
      it("Should not revert because user decryption is ready", async function () {
        await expect(decryptionManager.checkUserDecryptionReady(user.address, ctHandleContractPairs)).to.not.be
          .reverted;
      });

      it("Should revert because the user is not allowed for user decryption on a ctHandle", async function () {
        await expect(decryptionManager.checkUserDecryptionReady(fakeUserAddress, ctHandleContractPairs))
          .to.be.revertedWithCustomError(aclManager, "AccountNotAllowedToUseCiphertext")
          .withArgs(fakeUserAddress, ctHandleContractPairs[0].ctHandle);
      });

      it("Should revert because a contract is not allowed for user decryption on a ctHandle", async function () {
        // Define fake ctHandleContractPairs with a contract address that is not allowed for user decryption
        const fakeContractAddress = hre.ethers.Wallet.createRandom().address;
        const fakeCtHandleContractPairs: CtHandleContractPairStruct[] = [
          {
            contractAddress: fakeContractAddress,
            ctHandle,
          },
        ];

        await expect(decryptionManager.checkUserDecryptionReady(user.address, fakeCtHandleContractPairs))
          .to.be.revertedWithCustomError(aclManager, "AccountNotAllowedToUseCiphertext")
          .withArgs(fakeContractAddress, fakeCtHandleContractPairs[0].ctHandle);
      });

      it("Should revert because ciphertext material has not been added", async function () {
        // Allow public decryption for handles that have not been added
        // We need to do this because `checkPublicDecryptionReady` first checks if the handles
        // have been allowed for public decryption
        for (const newCtHandle of newCtHandles) {
          for (let i = 0; i < coprocessorTxSenders.length; i++) {
            await aclManager.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, user.address);
            await aclManager.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, contractAddress);
          }
        }

        await expect(decryptionManager.checkUserDecryptionReady(user.address, newCtHandleContractPairs))
          .to.be.revertedWithCustomError(ciphertextManager, "CiphertextMaterialNotFound")
          .withArgs(newCtHandleContractPairs[0].ctHandle);
      });

      it("Should revert because the user decryption is not done", async function () {
        await expect(decryptionManager.checkUserDecryptionDone(userDecryptionId))
          .to.be.revertedWithCustomError(decryptionManager, "UserDecryptionNotDone")
          .withArgs(userDecryptionId);
      });
    });
  });

  describe("Delegated User Decryption", function () {
    // Expected user decryption id (after first request)
    const userDecryptionId = 1;

    // Create a dummy reencrypted share
    const reencryptedShare = hre.ethers.randomBytes(32);

    // Allow handles for user decryption
    async function prepareAllowDelegatedUserDecryptionFixture() {
      const fixtureData = await loadFixture(prepareAddCiphertextFixture);
      const { coprocessorTxSenders, aclManager, user } = fixtureData;

      // Create a random user to receive delegation rights
      // This user will be used to sign the EIP712 message for delegated user decryption request
      const delegatedUser = await createAndFundRandomUser();

      const delegationAccounts: DelegationAccountsStruct = {
        delegatorAddress: user.address,
        delegatedAddress: delegatedUser.address,
      };

      // Allow user decryption and build ctHandleContractPairs
      const ctHandleContractPairs: CtHandleContractPairStruct[] = [];
      for (const ctHandle of ctHandles) {
        const contractAddress = hre.ethers.Wallet.createRandom().address;
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await aclManager.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, delegationAccounts.delegatorAddress);
          await aclManager.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, contractAddress);
        }
        ctHandleContractPairs.push({
          contractAddress,
          ctHandle,
        });
      }

      // Delegate account
      for (const txSender of coprocessorTxSenders) {
        await aclManager.connect(txSender).delegateAccount(
          hostChainId,
          delegationAccounts,
          ctHandleContractPairs.map((pair) => pair.contractAddress),
        );
      }

      return { ...fixtureData, delegatedUser, ctHandleContractPairs, delegationAccounts };
    }

    async function prepareDelegatedUserDecryptEIP712Fixture() {
      const fixtureData = await loadFixture(prepareAllowDelegatedUserDecryptionFixture);
      const { delegatedUser, decryptionManager, ctHandleContractPairs, delegationAccounts } = fixtureData;

      const publicKey = hre.ethers.randomBytes(32);
      const requestValidity: IDecryptionManager.RequestValidityStruct = {
        durationDays: 120,
        startTimestamp: Date.now(),
      };

      // Create EIP712 messages
      const decryptionManagerAddress = await decryptionManager.getAddress();
      const eip712RequestMessage = createEIP712RequestDelegatedUserDecrypt(
        chainId,
        decryptionManagerAddress,
        publicKey,
        ctHandleContractPairs.map((pair) => pair.contractAddress.toString()),
        delegationAccounts.delegatorAddress.toString(),
        hostChainId,
        requestValidity.startTimestamp.toString(),
        requestValidity.durationDays.toString(),
      );
      const eip712ResponseMessage = createEIP712ResponseUserDecrypt(
        chainId,
        decryptionManagerAddress,
        publicKey,
        ctHandleContractPairs.map((pair) => pair.ctHandle.toString()),
        reencryptedShare,
      );

      // Sign the message with the delegated user account
      const [delegatedUserSignature] = await getSignaturesDelegatedUserDecryptRequest(eip712RequestMessage, [
        delegatedUser,
      ]);

      return {
        ...fixtureData,
        delegatedUserSignature,
        ctHandleContractPairs,
        delegationAccounts,
        publicKey,
        requestValidity,
        eip712RequestMessage,
        eip712ResponseMessage,
      };
    }

    it("Should request a user decryption with multiple ctHandleContractPairs", async function () {
      const {
        decryptionManager,
        ctHandleContractPairs,
        delegationAccounts,
        publicKey,
        requestValidity,
        snsCiphertextMaterials,
        delegatedUserSignature,
      } = await loadFixture(prepareDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      // Request user decryption
      const requestTx = await decryptionManager.delegatedUserDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        delegationAccounts,
        hostChainId,
        contractAddresses,
        publicKey,
        delegatedUserSignature,
      );

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "UserDecryptionRequest")
        .withArgs(userDecryptionId, toValues(snsCiphertextMaterials), delegationAccounts.delegatedAddress, publicKey);
    });

    it("Should request a user decryption with a single ctHandleContractPair", async function () {
      const {
        decryptionManager,
        delegationAccounts,
        ctHandleContractPairs,
        publicKey,
        requestValidity,
        snsCiphertextMaterials,
        delegatedUserSignature,
      } = await loadFixture(prepareDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      const singleCtHandleContractPairs = ctHandleContractPairs.slice(0, 1);
      const singleSnsCiphertextMaterials = snsCiphertextMaterials.slice(0, 1);

      // Request user decryption
      const requestTx = await decryptionManager.delegatedUserDecryptionRequest(
        singleCtHandleContractPairs,
        requestValidity,
        delegationAccounts,
        hostChainId,
        contractAddresses,
        publicKey,
        delegatedUserSignature,
      );

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "UserDecryptionRequest")
        .withArgs(
          userDecryptionId,
          toValues(singleSnsCiphertextMaterials),
          delegationAccounts.delegatedAddress,
          publicKey,
        );
    });

    it("Should request a user decryption with empty ctHandleContractPairs list", async function () {
      const {
        decryptionManager,
        delegationAccounts,
        ctHandleContractPairs,
        publicKey,
        requestValidity,
        delegatedUserSignature,
      } = await loadFixture(prepareDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      // Request user decryption with an empty list of ctHandleContractPairs
      const requestTx = await decryptionManager.delegatedUserDecryptionRequest(
        [],
        requestValidity,
        delegationAccounts,
        hostChainId,
        contractAddresses,
        publicKey,
        delegatedUserSignature,
      );

      // Check request event
      await expect(requestTx)
        .to.emit(decryptionManager, "UserDecryptionRequest")
        .withArgs(userDecryptionId, [], delegationAccounts.delegatedAddress, publicKey);
    });

    it("Should revert because contractAddresses exceeds maximum length allowed", async function () {
      const { decryptionManager } = await loadFixture(loadTestVariablesFixture);

      // Create dummy input data for the user decryption request
      const delegatorAddress = hre.ethers.Wallet.createRandom().address;
      const delegatedAddress = hre.ethers.Wallet.createRandom().address;
      const contractAddresses = [];
      for (let i = 0; i < 11; i++) {
        contractAddresses.push(hre.ethers.Wallet.createRandom().address);
      }

      // Check that the request fails because the given contractAddresses exceeds the maximum length allowed
      await expect(
        decryptionManager.delegatedUserDecryptionRequest(
          [],
          { durationDays: 120, startTimestamp: Date.now() },
          { delegatorAddress, delegatedAddress },
          hostChainId,
          contractAddresses,
          hre.ethers.randomBytes(32),
          hre.ethers.randomBytes(32),
        ),
      )
        .to.be.revertedWithCustomError(decryptionManager, "ContractAddressesMaxLengthExceeded")
        .withArgs(10, contractAddresses.length);
    });

    it("Should revert because durationDays exceeds maximum allowed", async function () {
      const { decryptionManager } = await loadFixture(loadTestVariablesFixture);

      // Create dummy input data for the user decryption request
      const delegatorAddress = hre.ethers.Wallet.createRandom().address;
      const delegatedAddress = hre.ethers.Wallet.createRandom().address;
      const requestValidity: IDecryptionManager.RequestValidityStruct = {
        durationDays: 400,
        startTimestamp: Date.now(),
      };

      // Check that the request fails because the given requestValidity.durationDays exceeds the maximum allowed
      await expect(
        decryptionManager.delegatedUserDecryptionRequest(
          [],
          requestValidity,
          { delegatorAddress, delegatedAddress },
          hostChainId,
          [],
          hre.ethers.randomBytes(32),
          hre.ethers.randomBytes(32),
        ),
      )
        .to.be.revertedWithCustomError(decryptionManager, "MaxDurationDaysExceeded")
        .withArgs(365, requestValidity.durationDays);
    });

    it("Should revert because the delegator address is a contract address", async function () {
      const {
        decryptionManager,
        ctHandleContractPairs,
        delegationAccounts,
        publicKey,
        requestValidity,
        delegatedUserSignature,
      } = await loadFixture(prepareDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      const fakeContractAddresses = [...contractAddresses, delegationAccounts.delegatorAddress];

      // Check that the request fails because the delegated address is included in the ctHandleContractPairs list
      await expect(
        decryptionManager.delegatedUserDecryptionRequest(
          ctHandleContractPairs.concat([
            { contractAddress: delegationAccounts.delegatorAddress, ctHandle: ctHandles[0] },
          ]),
          requestValidity,
          delegationAccounts,
          hostChainId,
          fakeContractAddresses,
          publicKey,
          delegatedUserSignature,
        ),
      )
        .to.be.revertedWithCustomError(decryptionManager, "DelegatorAddressInContractAddresses")
        .withArgs(delegationAccounts.delegatorAddress, fakeContractAddresses);
    });

    it("Should revert because the delegator is not allowed for user decryption on a ctHandle", async function () {
      const { aclManager, decryptionManager } = await loadFixture(loadTestVariablesFixture);

      // Create dummy input data for the user decryption request
      const contractAddress = hre.ethers.Wallet.createRandom().address;
      const publicKey = hre.ethers.randomBytes(32);
      const userSignature = hre.ethers.randomBytes(32);
      const delegatorAddress = hre.ethers.Wallet.createRandom().address;
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
        decryptionManager.delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          { delegatorAddress, delegatedAddress },
          hostChainId,
          [contractAddress],
          publicKey,
          userSignature,
        ),
      )
        .to.be.revertedWithCustomError(aclManager, "AccountNotAllowedToUseCiphertext")
        .withArgs(delegatorAddress, ctHandles[0]);
    });

    it("Should revert because a contract is not allowed for user decryption on a ctHandle", async function () {
      const {
        aclManager,
        decryptionManager,
        ctHandleContractPairs,
        delegationAccounts,
        publicKey,
        requestValidity,
        delegatedUserSignature,
      } = await loadFixture(prepareDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      // Check that the request fails because the included contractAddress is not allowed for user decryption
      // Note: the function should be reverted on the not allowed contract address since it loops over the handles
      const fakeContractAddress = hre.ethers.Wallet.createRandom().address;
      await expect(
        decryptionManager.delegatedUserDecryptionRequest(
          ctHandleContractPairs.concat([{ contractAddress: fakeContractAddress, ctHandle: ctHandles[0] }]),
          requestValidity,
          delegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedUserSignature,
        ),
      )
        .to.be.revertedWithCustomError(aclManager, "AccountNotAllowedToUseCiphertext")
        .withArgs(fakeContractAddress, ctHandles[0]);
    });

    it("Should revert because ciphertext material has not been added", async function () {
      const {
        decryptionManager,
        aclManager,
        ciphertextManager,
        ctHandleContractPairs,
        delegationAccounts,
        publicKey,
        requestValidity,
        delegatedUserSignature,
        coprocessorTxSenders,
      } = await loadFixture(prepareDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      // Allow for user decryption a ctHandle whose associated ciphertext material has not
      // yet been added to the CiphertextStorage state.
      for (const coprocessorTxSender of coprocessorTxSenders) {
        await aclManager.connect(coprocessorTxSender).allowAccount(newCtHandle, delegationAccounts.delegatorAddress);
        await aclManager.connect(coprocessorTxSender).allowAccount(newCtHandle, contractAddresses[0]);
      }

      // Check that the request fails because the ciphertext material is unavailable
      // Note: the function should be reverted on the unavailable ctHandle since it loops over the handles
      await expect(
        decryptionManager.delegatedUserDecryptionRequest(
          ctHandleContractPairs.concat([{ contractAddress: contractAddresses[0], ctHandle: newCtHandle }]),
          requestValidity,
          delegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedUserSignature,
        ),
      )
        .to.be.revertedWithCustomError(ciphertextManager, "CiphertextMaterialNotFound")
        .withArgs(newCtHandle);
    });

    it("Should revert because the delegated address has not been delegated for a contract", async function () {
      const {
        aclManager,
        decryptionManager,
        ctHandleContractPairs,
        delegationAccounts,
        publicKey,
        requestValidity,
        delegatedUserSignature,
      } = await loadFixture(prepareDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      const notDelegatedContractAddress = hre.ethers.Wallet.createRandom().address;
      await expect(
        decryptionManager.delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          delegationAccounts,
          hostChainId,
          [...contractAddresses, notDelegatedContractAddress],
          publicKey,
          delegatedUserSignature,
        ),
      )
        .to.be.revertedWithCustomError(aclManager, "AccountNotDelegated")
        .withArgs(hostChainId, toValues(delegationAccounts), notDelegatedContractAddress);
    });

    it("Should revert because of invalid EIP712 user request signature", async function () {
      const {
        decryptionManager,
        delegationAccounts,
        ctHandleContractPairs,
        publicKey,
        requestValidity,
        eip712RequestMessage,
      } = await loadFixture(prepareDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);
      const fakeSigner = await createAndFundRandomUser();

      // Sign the message with a fake signer
      const [fakeSignature] = await getSignaturesDelegatedUserDecryptRequest(eip712RequestMessage, [fakeSigner]);

      // Request user decryption
      const requestTx = decryptionManager.delegatedUserDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        delegationAccounts,
        hostChainId,
        contractAddresses,
        publicKey,
        fakeSignature,
      );

      // Check that the request has been reverted because of an invalid EIP712 user request signature
      await expect(requestTx)
        .to.be.revertedWithCustomError(decryptionManager, "InvalidUserSignature")
        .withArgs(fakeSignature);
    });

    it("Should revert because contract in ctHandleContractPairs is not included in contractAddresses list", async function () {
      const {
        decryptionManager,
        delegationAccounts,
        ctHandleContractPairs,
        publicKey,
        requestValidity,
        delegatedUserSignature,
      } = await loadFixture(prepareDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const fakeContractAddresses = [hre.ethers.Wallet.createRandom().address];

      // Check that the request fails because the contract address is not included in the contractAddresses list
      await expect(
        decryptionManager.delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          delegationAccounts,
          hostChainId,
          fakeContractAddresses,
          publicKey,
          delegatedUserSignature,
        ),
      )
        .to.be.revertedWithCustomError(decryptionManager, "ContractNotInContractAddresses")
        .withArgs(ctHandleContractPairs[0].contractAddress, fakeContractAddresses);
    });

    it("Should revert because of ctMaterials tied to different key IDs", async function () {
      const {
        keyManager,
        decryptionManager,
        ciphertextManager,
        aclManager,
        owner,
        coprocessorTxSenders,
        kmsTxSenders,
        keyId1,
        ctHandleContractPairs,
        delegationAccounts,
        requestValidity,
        publicKey,
        delegatedUserSignature,
        fheParamsName,
      } = await loadFixture(prepareDelegatedUserDecryptEIP712Fixture);

      const keyId2 = await createAndRotateKey(
        keyId1,
        keyManager,
        owner,
        coprocessorTxSenders,
        kmsTxSenders,
        fheParamsName,
      );

      const ctHandleContractPair = {
        ctHandle: newCtHandle,
        contractAddress: ctHandleContractPairs[0].contractAddress,
      };

      // Store the ciphertext and allow public decryption
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await ciphertextManager
          .connect(coprocessorTxSenders[i])
          .addCiphertextMaterial(ctHandleContractPair.ctHandle, keyId2, ciphertextDigest, snsCiphertextDigest);
      }
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await aclManager
          .connect(coprocessorTxSenders[i])
          .allowAccount(ctHandleContractPair.ctHandle, delegationAccounts.delegatorAddress);
        await aclManager
          .connect(coprocessorTxSenders[i])
          .allowAccount(ctHandleContractPair.ctHandle, ctHandleContractPair.contractAddress);
      }

      // Create dummy input data for the user decryption request
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      // Request user decryption with ctMaterials tied to different key IDs
      const requestTx = decryptionManager.delegatedUserDecryptionRequest(
        [ctHandleContractPair, ...ctHandleContractPairs.slice(1)],
        requestValidity,
        delegationAccounts,
        hostChainId,
        contractAddresses,
        publicKey,
        delegatedUserSignature,
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
        kmsTxSenders,
        kmsSigners,
        eip712ResponseMessage,
        delegatedUserSignature,
      } = await loadFixture(prepareDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      // Request user decryption
      await decryptionManager
        .connect(user)
        .delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          delegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedUserSignature,
        );

      // Sign the message with all KMS signers and get the first 3 signatures
      const [signature1, signature2, signature3] = await getSignaturesUserDecryptResponse(
        eip712ResponseMessage,
        kmsSigners,
      );

      // Trigger three valid user decryption responses using different KMS transaction senders
      await decryptionManager
        .connect(kmsTxSenders[0])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, signature1);

      await decryptionManager
        .connect(kmsTxSenders[1])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, signature2);

      const responseTx3 = await decryptionManager
        .connect(kmsTxSenders[2])
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
      await expect(decryptionManager.checkUserDecryptionDone(userDecryptionId)).to.not.be.reverted;
    });

    it("Should ignore other valid responses", async function () {
      const {
        decryptionManager,
        publicKey,
        ctHandleContractPairs,
        requestValidity,
        delegationAccounts,
        kmsTxSenders,
        kmsSigners,
        eip712ResponseMessage,
        delegatedUserSignature,
      } = await loadFixture(prepareDelegatedUserDecryptEIP712Fixture);

      // Create dummy input data for the user decryption request
      const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

      // Request user decryption
      await decryptionManager.delegatedUserDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        delegationAccounts,
        hostChainId,
        contractAddresses,
        publicKey,
        delegatedUserSignature,
      );

      // Sign the message with all KMS signers and get the first 3 signatures
      const [signature1, signature2, signature3, signature4] = await getSignaturesUserDecryptResponse(
        eip712ResponseMessage,
        kmsSigners,
      );

      // Trigger three valid user decryption responses using different KMS transaction senders
      const responseTx1 = await decryptionManager
        .connect(kmsTxSenders[0])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, signature1);

      const responseTx2 = await decryptionManager
        .connect(kmsTxSenders[1])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, signature2);

      await decryptionManager
        .connect(kmsTxSenders[2])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, signature3);

      const responseTx4 = await decryptionManager
        .connect(kmsTxSenders[3])
        .userDecryptionResponse(userDecryptionId, reencryptedShare, signature4);

      // Check that the 1st, 2nd and 4th responses do not emit an event:
      // - 1st and 2nd responses are ignored because consensus is not reached yet
      // - 4th response is ignored (not reverted) even though they are late
      await expect(responseTx1).to.not.emit(decryptionManager, "UserDecryptionResponse");
      await expect(responseTx2).to.not.emit(decryptionManager, "UserDecryptionResponse");
      await expect(responseTx4).to.not.emit(decryptionManager, "UserDecryptionResponse");
    });

    describe("Checks", function () {
      it("Should not revert because delegated user decryption is ready", async function () {
        const { decryptionManager, ctHandleContractPairs, delegationAccounts } = await loadFixture(
          prepareDelegatedUserDecryptEIP712Fixture,
        );

        // Get the list of contract addresses
        const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);

        await expect(
          decryptionManager.checkDelegatedUserDecryptionReady(
            hostChainId,
            delegationAccounts,
            ctHandleContractPairs,
            contractAddresses,
          ),
        ).to.not.be.reverted;
      });

      it("Should revert because the delegator is not allowed for user decryption on a ctHandle", async function () {
        const { decryptionManager, aclManager, ctHandleContractPairs, delegationAccounts } = await loadFixture(
          prepareDelegatedUserDecryptEIP712Fixture,
        );

        // Define a fake user address
        const fakeUserAddress = (await createAndFundRandomUser()).address;

        // Define a fake delegation accounts
        const fakeDelegationAccounts: DelegationAccountsStruct = {
          delegatorAddress: fakeUserAddress,
          delegatedAddress: delegationAccounts.delegatedAddress,
        };

        await expect(
          decryptionManager.checkDelegatedUserDecryptionReady(
            hostChainId,
            fakeDelegationAccounts,
            ctHandleContractPairs,
            [],
          ),
        )
          .to.be.revertedWithCustomError(aclManager, "AccountNotAllowedToUseCiphertext")
          .withArgs(fakeUserAddress, ctHandleContractPairs[0].ctHandle);
      });

      it("Should revert because a contract is not allowed for user decryption on a ctHandle", async function () {
        const { decryptionManager, aclManager, delegationAccounts } = await loadFixture(
          prepareDelegatedUserDecryptEIP712Fixture,
        );

        // Define fake ctHandleContractPairs with a contract address that is not allowed for user decryption
        const fakeContractAddress = hre.ethers.Wallet.createRandom().address;
        const fakeCtHandleContractPairs: CtHandleContractPairStruct[] = [
          {
            contractAddress: fakeContractAddress,
            ctHandle,
          },
        ];

        await expect(
          decryptionManager.checkDelegatedUserDecryptionReady(
            hostChainId,
            delegationAccounts,
            fakeCtHandleContractPairs,
            [],
          ),
        )
          .to.be.revertedWithCustomError(aclManager, "AccountNotAllowedToUseCiphertext")
          .withArgs(fakeContractAddress, fakeCtHandleContractPairs[0].ctHandle);
      });

      it("Should revert because the delegated address has not been delegated for a contract", async function () {
        const { aclManager, decryptionManager, ctHandleContractPairs, delegationAccounts } = await loadFixture(
          prepareDelegatedUserDecryptEIP712Fixture,
        );

        // Define a contract address that has not been delegated for the delegated address
        const notDelegatedContractAddress = hre.ethers.Wallet.createRandom().address;

        await expect(
          decryptionManager.checkDelegatedUserDecryptionReady(hostChainId, delegationAccounts, ctHandleContractPairs, [
            notDelegatedContractAddress,
          ]),
        )
          .to.be.revertedWithCustomError(aclManager, "AccountNotDelegated")
          .withArgs(hostChainId, toValues(delegationAccounts), notDelegatedContractAddress);
      });

      it("Should revert because ciphertext material has not been added", async function () {
        const { decryptionManager, ctHandleContractPairs, ciphertextManager, delegationAccounts } = await loadFixture(
          prepareDelegatedUserDecryptEIP712Fixture,
        );

        const contractAddresses = ctHandleContractPairs.map((pair) => pair.contractAddress);
        const contractAddress = contractAddresses[0];

        // Define new valid inputs
        const newCtHandleContractPairs: CtHandleContractPairStruct[] = newCtHandles.map((newCtHandle) => ({
          contractAddress,
          ctHandle: newCtHandle,
        }));

        // Allow public decryption for handles that have not been added
        // We need to do this because `checkPublicDecryptionReady` first checks if the handles
        // have been allowed for public decryption
        for (const newCtHandle of newCtHandles) {
          for (let i = 0; i < coprocessorTxSenders.length; i++) {
            await aclManager.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, user.address);
            await aclManager.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, contractAddress);
          }
        }

        await expect(
          decryptionManager.checkDelegatedUserDecryptionReady(
            hostChainId,
            delegationAccounts,
            newCtHandleContractPairs,
            contractAddresses,
          ),
        )
          .to.be.revertedWithCustomError(ciphertextManager, "CiphertextMaterialNotFound")
          .withArgs(newCtHandleContractPairs[0].ctHandle);
      });
    });
  });
});
