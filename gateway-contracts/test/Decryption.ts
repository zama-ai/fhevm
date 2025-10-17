import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import hre from "hardhat";

import {
  CiphertextCommits,
  Decryption,
  Decryption__factory,
  GatewayConfig,
  IDecryption,
  KMSGeneration,
  MultichainACL,
} from "../typechain-types";
// The type needs to be imported separately because it is not properly detected by the linter
// as this type is defined as a shared structs instead of directly in the IDecryption interface
import {
  CtHandleContractPairStruct,
  SnsCiphertextMaterialStruct,
} from "../typechain-types/contracts/interfaces/IDecryption";
import {
  EIP712,
  createByteInput,
  createBytes32,
  createBytes32s,
  createCtHandle,
  createCtHandles,
  createEIP712RequestDelegatedUserDecrypt,
  createEIP712RequestUserDecrypt,
  createEIP712ResponsePublicDecrypt,
  createEIP712ResponseUserDecrypt,
  createRandomAddress,
  createRandomAddresses,
  createRandomWallet,
  getKeyId,
  getPublicDecryptId,
  getSignaturesDelegatedUserDecryptRequest,
  getSignaturesPublicDecrypt,
  getSignaturesUserDecryptRequest,
  getSignaturesUserDecryptResponse,
  getUserDecryptId,
  loadHostChainIds,
  loadTestVariablesFixture,
  toValues,
} from "./utils";

// Constants for the Decryption contract
const MAX_USER_DECRYPT_DURATION_DAYS = 365;
const MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10;
const MAX_DECRYPTION_REQUEST_BITS = 2048;

// Get the current date in seconds. This is needed because Solidity works with seconds, not milliseconds
// See https://docs.soliditylang.org/en/develop/units-and-global-variables.html#time-units
function getDateInSeconds(): number {
  return Math.floor(Date.now() / 1000);
}

describe("Decryption", function () {
  // Get the registered host chains' chain IDs
  const hostChainIds = loadHostChainIds();
  const hostChainId = hostChainIds[0];

  // Get the gateway's chain ID
  const gatewayChainId = hre.network.config.chainId!;

  // Define input values
  const keyId = getKeyId(1);
  const ciphertextDigest = createBytes32();
  const snsCiphertextDigest = createBytes32();

  // Define an euint256 ctHandle (which has a bit size of 256 bits)
  const euint256CtHandle = createCtHandle(hostChainId, 8);

  // Create ciphertext handles for the host chain ID with different TFHE-rs types
  // Note that the list is made so that the total bit size represented by these handles (2+10+256=268 bits)
  // does not exceed 2048 bits (the maximum allowed for a single list of handles)
  const ctHandles = [createCtHandle(hostChainId, 0), createCtHandle(hostChainId, 2), euint256CtHandle];
  const ctHandle = ctHandles[0];

  // Define other valid ctHandles (they will not be added in the ciphertext commits contract and allowed for
  // public decryption or account access by default)
  const newCtHandles = createCtHandles(3, hostChainId);
  const newCtHandle = newCtHandles[0];

  // Define a new key ID
  const newKeyId = getKeyId(2);

  // Define a handle with an invalid FHE type (see `FheType.sol`)
  const invalidFHEType = 255;
  const invalidFHETypeCtHandle = createCtHandle(hostChainId, invalidFHEType);

  // Define a handle with an unsupported FHE type (see `FHETypeBitSizes.sol`)
  const unsupportedFHEType = 13;
  const unsupportedFHETypeCtHandle = createCtHandle(hostChainId, unsupportedFHEType);

  // Define fake values
  const fakeTxSender = createRandomWallet();
  const fakeSigner = createRandomWallet();
  const tooLowDecryptionId = 0;
  const tooHighDecryptionId = getPublicDecryptId(1000) + getUserDecryptId(1000);

  // Define extra data for version 0
  const extraDataV0 = hre.ethers.solidityPacked(["uint8"], [0]);

  let gatewayConfig: GatewayConfig;
  let kmsGeneration: KMSGeneration;
  let multichainACL: MultichainACL;
  let ciphertextCommits: CiphertextCommits;
  let decryption: Decryption;
  let owner: Wallet;
  let pauser: Wallet;
  let snsCiphertextMaterials: SnsCiphertextMaterialStruct[];
  let kmsSignatures: string[];
  let kmsTxSenders: HardhatEthersSigner[];
  let kmsSigners: HardhatEthersSigner[];
  let coprocessorTxSenders: HardhatEthersSigner[];

  // Add ciphertext materials
  async function prepareAddCiphertextFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const { ciphertextCommits, coprocessorTxSenders } = fixtureData;

    let snsCiphertextMaterials: SnsCiphertextMaterialStruct[] = [];

    // Allow public decryption
    for (const ctHandle of ctHandles) {
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await ciphertextCommits
          .connect(coprocessorTxSenders[i])
          .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);
      }

      // Store the SNS ciphertext materials for event checks
      snsCiphertextMaterials.push({
        ctHandle,
        keyId,
        snsCiphertextDigest,
        coprocessorTxSenderAddresses: coprocessorTxSenders.map((s) => s.address),
      });
    }

    return { ...fixtureData, snsCiphertextMaterials, keyId };
  }

  describe("Deployment", function () {
    let decryptionFactory: Decryption__factory;

    beforeEach(async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      decryption = fixtureData.decryption;
      owner = fixtureData.owner;

      // Get the Decryption contract factory
      decryptionFactory = await hre.ethers.getContractFactory("Decryption", owner);
    });

    it("Should revert because initialization is not from an empty proxy", async function () {
      await expect(
        hre.upgrades.upgradeProxy(decryption, decryptionFactory, {
          call: { fn: "initializeFromEmptyProxy" },
        }),
      ).to.be.revertedWithCustomError(decryption, "NotInitializingFromEmptyProxy");
    });
  });

  describe("Public Decryption", function () {
    let eip712Message: EIP712;

    // Expected decryption request ID (after a first request) for a public decryption request
    // The IDs won't increase between requests made in different "describe" sections as the blockchain
    // state is cleaned each time `loadFixture` is called
    const decryptionId = getPublicDecryptId(1);

    // Create input values
    const decryptedResult = createByteInput();

    // Define fake values
    const fakeDecryptedResult = createByteInput();

    // Allow handles for public decryption
    async function preparePublicDecryptEIP712Fixture() {
      const fixtureData = await loadFixture(prepareAddCiphertextFixture);
      const { multichainACL, decryption, kmsSigners, coprocessorTxSenders } = fixtureData;

      // Allow public decryption
      for (const ctHandle of ctHandles) {
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await multichainACL.connect(coprocessorTxSenders[i]).allowPublicDecrypt(ctHandle, extraDataV0);
        }
      }

      // Create EIP712 messages
      const decryptionAddress = await decryption.getAddress();
      const eip712Message = createEIP712ResponsePublicDecrypt(
        gatewayChainId,
        decryptionAddress,
        ctHandles,
        decryptedResult,
        extraDataV0,
      );

      // Sign the message with all KMS signers
      const kmsSignatures = await getSignaturesPublicDecrypt(eip712Message, kmsSigners);

      return { ...fixtureData, eip712Message, kmsSignatures };
    }

    beforeEach(async function () {
      // Initialize globally used variables before each test
      const fixtureData = await loadFixture(preparePublicDecryptEIP712Fixture);
      gatewayConfig = fixtureData.gatewayConfig;
      kmsGeneration = fixtureData.kmsGeneration;
      multichainACL = fixtureData.multichainACL;
      ciphertextCommits = fixtureData.ciphertextCommits;
      decryption = fixtureData.decryption;
      owner = fixtureData.owner;
      pauser = fixtureData.pauser;
      snsCiphertextMaterials = fixtureData.snsCiphertextMaterials;
      kmsSignatures = fixtureData.kmsSignatures;
      kmsTxSenders = fixtureData.kmsTxSenders;
      kmsSigners = fixtureData.kmsSigners;
      coprocessorTxSenders = fixtureData.coprocessorTxSenders;
      eip712Message = fixtureData.eip712Message;
    });

    it("Should request a public decryption with multiple ctHandles", async function () {
      // Request public decryption
      const requestTx = await decryption.publicDecryptionRequest(ctHandles, extraDataV0);

      // Check request event
      await expect(requestTx)
        .to.emit(decryption, "PublicDecryptionRequest")
        .withArgs(decryptionId, toValues(snsCiphertextMaterials), extraDataV0);
    });

    it("Should request a public decryption with a single ctHandle", async function () {
      // Request public decryption with a single ctHandle
      const requestTx = await decryption.publicDecryptionRequest([ctHandles[0]], extraDataV0);

      const singleSnsCiphertextMaterials = snsCiphertextMaterials.slice(0, 1);

      // Check request event
      await expect(requestTx)
        .to.emit(decryption, "PublicDecryptionRequest")
        .withArgs(decryptionId, toValues(singleSnsCiphertextMaterials), extraDataV0);
    });

    it("Should revert because ctHandles list is empty", async function () {
      // Check that the request fails because the list of handles is empty
      await expect(decryption.publicDecryptionRequest([], extraDataV0)).to.be.revertedWithCustomError(
        decryption,
        "EmptyCtHandles",
      );
    });

    it("Should revert because handle represents an invalid FHE type", async function () {
      // Check that the request fails because the ctHandle represents an invalid FHE type
      await expect(decryption.publicDecryptionRequest([invalidFHETypeCtHandle], extraDataV0))
        .to.be.revertedWithCustomError(decryption, "InvalidFHEType")
        .withArgs(invalidFHEType);
    });

    it("Should revert because handle represents an unsupported FHE type", async function () {
      // Check that the request fails because the ctHandle represents an unsupported FHE type
      await expect(decryption.publicDecryptionRequest([unsupportedFHETypeCtHandle], extraDataV0))
        .to.be.revertedWithCustomError(decryption, "UnsupportedFHEType")
        .withArgs(unsupportedFHEType);
    });

    it("Should revert because total bit size exceeds the maximum allowed", async function () {
      // Create a list of 12 euint256 ctHandles (each has a bit size of 256 bits)
      const numCtHandles = 12;
      const largeBitSizeCtHandles = Array(numCtHandles).fill(euint256CtHandle);

      // Calculate the new total bit size of this list
      const totalBitSize = numCtHandles * 256;

      // Check that the request fails because the total bit size exceeds the maximum allowed
      await expect(decryption.publicDecryptionRequest(largeBitSizeCtHandles, extraDataV0))
        .to.be.revertedWithCustomError(decryption, "MaxDecryptionRequestBitSizeExceeded")
        .withArgs(MAX_DECRYPTION_REQUEST_BITS, totalBitSize);
    });

    it("Should revert because handles are not allowed for public decryption", async function () {
      // Check that the request fails because the handles are not allowed for public decryption
      await expect(decryption.publicDecryptionRequest(newCtHandles, extraDataV0))
        .to.be.revertedWithCustomError(decryption, "PublicDecryptNotAllowed")
        .withArgs(newCtHandles[0]);
    });

    it("Should revert because ciphertext material has not been added", async function () {
      // Allow public decryption for handles that have not been added
      // We need to do this because `publicDecryptionRequest` first checks if the handles
      // have been allowed for public decryption
      for (const newCtHandle of newCtHandles) {
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await multichainACL.connect(coprocessorTxSenders[i]).allowPublicDecrypt(newCtHandle, extraDataV0);
        }
      }

      // Check that the request fails because the ciphertext material is unavailable
      await expect(decryption.publicDecryptionRequest(newCtHandles, extraDataV0))
        .to.be.revertedWithCustomError(ciphertextCommits, "CiphertextMaterialNotFound")
        .withArgs(newCtHandles[0]);
    });

    it("Should revert because the message sender is not a KMS transaction sender", async function () {
      // Check that the transaction fails because the msg.sender is not a registered KMS transaction sender
      await expect(
        decryption
          .connect(fakeTxSender)
          .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[0], extraDataV0),
      )
        .to.be.revertedWithCustomError(decryption, "NotKmsTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should revert because the signer is not a KMS signer", async function () {
      // Request public decryption
      // This step is necessary, else the decryptionId won't be set in the state and the
      // signature verification will use wrong handles
      await decryption.publicDecryptionRequest(ctHandles, extraDataV0);

      // Create a fake signature from the fake signer
      const [fakeSignature] = await getSignaturesPublicDecrypt(eip712Message, [fakeSigner]);

      // Check that the signature verification fails because the signer is not a registered KMS signer
      await expect(
        decryption
          .connect(kmsTxSenders[0])
          .publicDecryptionResponse(decryptionId, decryptedResult, fakeSignature, extraDataV0),
      )
        .to.be.revertedWithCustomError(decryption, "NotKmsSigner")
        .withArgs(fakeSigner.address);
    });

    it("Should revert because of two responses with same signature", async function () {
      // Request public decryption
      await decryption.publicDecryptionRequest(ctHandles, extraDataV0);

      // Trigger a first public decryption response
      await decryption
        .connect(kmsTxSenders[0])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[0], extraDataV0);

      // Check that a KMS node cannot sign a second time for the same public decryption
      await expect(
        decryption
          .connect(kmsTxSenders[0])
          .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[0], extraDataV0),
      )
        .to.be.revertedWithCustomError(decryption, "KmsNodeAlreadySigned")
        .withArgs(decryptionId, kmsSigners[0].address);
    });

    it("Should revert because of ctMaterials tied to different key IDs", async function () {
      // Store the handles with a new key ID and allow them for public decryption
      for (const newCtHandle of newCtHandles) {
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await ciphertextCommits
            .connect(coprocessorTxSenders[i])
            .addCiphertextMaterial(newCtHandle, newKeyId, ciphertextDigest, snsCiphertextDigest);

          await multichainACL.connect(coprocessorTxSenders[i]).allowPublicDecrypt(newCtHandle, extraDataV0);
        }
      }

      // Request public decryption with ctMaterials tied to different key IDs
      const requestTx = decryption.publicDecryptionRequest([...ctHandles, newCtHandle], extraDataV0);

      // Check that different key IDs are not allowed for batched public decryption
      await expect(requestTx)
        .to.be.revertedWithCustomError(decryption, "DifferentKeyIdsNotAllowed")
        .withArgs(
          toValues(snsCiphertextMaterials[0]),
          toValues({
            ctHandle: newCtHandle,
            keyId: newKeyId,
            snsCiphertextDigest,
            coprocessorTxSenderAddresses: coprocessorTxSenders.map((s) => s.address),
          }),
        );
    });

    it("Should public decrypt with 3 valid responses", async function () {
      // Request public decryption
      await decryption.publicDecryptionRequest(ctHandles, extraDataV0);

      // Trigger three valid public decryption responses
      await decryption
        .connect(kmsTxSenders[0])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[0], extraDataV0);
      await decryption
        .connect(kmsTxSenders[1])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[1], extraDataV0);

      const responseTx3 = await decryption
        .connect(kmsTxSenders[2])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[2], extraDataV0);

      // Consensus should be reached at the third response
      // Check 3rd response event: it should only contain 3 valid signatures
      await expect(responseTx3)
        .to.emit(decryption, "PublicDecryptionResponse")
        .withArgs(decryptionId, decryptedResult, [kmsSignatures[0], kmsSignatures[1], kmsSignatures[2]], extraDataV0);

      // Check that the public decryption is done
      expect(await decryption.isDecryptionDone(decryptionId)).to.be.true;
    });

    it("Should public decrypt with 3 valid responses and ignore the other valid one", async function () {
      // Request public decryption
      await decryption.publicDecryptionRequest(ctHandles, extraDataV0);

      // Trigger four valid public decryption responses
      const responseTx1 = await decryption
        .connect(kmsTxSenders[0])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[0], extraDataV0);

      const responseTx2 = await decryption
        .connect(kmsTxSenders[1])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[1], extraDataV0);

      await decryption
        .connect(kmsTxSenders[2])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[2], extraDataV0);

      const responseTx4 = await decryption
        .connect(kmsTxSenders[3])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[3], extraDataV0);

      // Check that the 1st, 2nd and 4th responses do not emit an event:
      // - 1st and 2nd responses are ignored because consensus is not reached yet
      // - 4th response is ignored (not reverted) even though it is late
      await expect(responseTx1).to.not.emit(decryption, "PublicDecryptionResponse");
      await expect(responseTx2).to.not.emit(decryption, "PublicDecryptionResponse");
      await expect(responseTx4).to.not.emit(decryption, "PublicDecryptionResponse");
    });

    it("Should public decrypt with 3 valid and 1 malicious signatures", async function () {
      // Request public decryption
      await decryption.publicDecryptionRequest(ctHandles, extraDataV0);

      const decryptionAddress = await decryption.getAddress();

      // Create a malicious EIP712 message: the decryptedResult is different from the expected one
      // but the signature is valid (the malicious decryptedResult is given to the response call)
      const fakeEip712Message = createEIP712ResponsePublicDecrypt(
        gatewayChainId,
        decryptionAddress,
        ctHandles,
        fakeDecryptedResult,
        extraDataV0,
      );
      const [fakeKmsSignature] = await getSignaturesPublicDecrypt(fakeEip712Message, kmsSigners.slice(0, 1));

      // Trigger a malicious public decryption response with:
      // - the first KMS transaction sender (expected)
      // - a fake decrypted result (unexpected)
      // - a fake signature based on the fake decrypted result (unexpected)
      await decryption
        .connect(kmsTxSenders[0])
        .publicDecryptionResponse(decryptionId, fakeDecryptedResult, fakeKmsSignature, extraDataV0);

      // Trigger a first valid public decryption response with:
      // - the second KMS transaction sender
      // - the second KMS signer's signature
      await decryption
        .connect(kmsTxSenders[1])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[1], extraDataV0);

      // Trigger a second valid public decryption response with:
      // - the third KMS transaction sender
      // - the third KMS signer's signature
      const responseTx3 = await decryption
        .connect(kmsTxSenders[2])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[2], extraDataV0);

      // Trigger a third valid public decryption response with:
      // - the fourth KMS transaction sender
      // - the fourth KMS signer's signature
      const responseTx4 = await decryption
        .connect(kmsTxSenders[3])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[3], extraDataV0);

      // Consensus should not be reached at the third transaction since the first was malicious
      // Check 3rd transaction events: it should not emit an event for public decryption response
      await expect(responseTx3).to.not.emit(decryption, "PublicDecryptionResponse");

      // Consensus should be reached at the fourth transaction
      // Check 4th transaction events: it should only contain 3 valid signatures
      await expect(responseTx4)
        .to.emit(decryption, "PublicDecryptionResponse")
        .withArgs(decryptionId, decryptedResult, kmsSignatures.slice(1, 4), extraDataV0);
    });

    it("Should get all valid KMS transaction senders from public decryption consensus", async function () {
      // Request public decryption
      await decryption.publicDecryptionRequest(ctHandles, extraDataV0);

      // Trigger 2 valid public decryption responses
      await decryption
        .connect(kmsTxSenders[0])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[0], extraDataV0);

      await decryption
        .connect(kmsTxSenders[1])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[1], extraDataV0);

      // Check that the KMS transaction senders list is empty because consensus is not reached yet
      const decryptionConsensusTxSenders1 = await decryption.getDecryptionConsensusTxSenders(decryptionId);
      expect(decryptionConsensusTxSenders1).to.deep.equal([]);

      // Trigger a third valid public decryption response
      await decryption
        .connect(kmsTxSenders[2])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[2], extraDataV0);

      const expectedKmsTxSenderAddresses2 = kmsTxSenders.slice(0, 3).map((s) => s.address);

      // Check that the KMS transaction senders that were involved in the consensus are the 3 KMS
      // transaction senders, at the moment the consensus is reached
      const decryptionConsensusTxSenders2 = await decryption.getDecryptionConsensusTxSenders(decryptionId);
      expect(decryptionConsensusTxSenders2).to.deep.equal(expectedKmsTxSenderAddresses2);

      // Trigger a fourth valid public decryption response
      await decryption
        .connect(kmsTxSenders[3])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[3], extraDataV0);

      const expectedKmsTxSenderAddresses3 = kmsTxSenders.map((s) => s.address);

      // Check that the KMS transaction senders that were involved in the consensus are the 4 KMS
      // transaction senders, after the consensus is reached
      const decryptionConsensusTxSenders3 = await decryption.getDecryptionConsensusTxSenders(decryptionId);
      expect(decryptionConsensusTxSenders3).to.deep.equal(expectedKmsTxSenderAddresses3);
    });

    it("Should get valid KMS transaction senders from public decryption consensus and ignore malicious ones", async function () {
      // Request public decryption
      await decryption.publicDecryptionRequest(ctHandles, extraDataV0);

      // Trigger 3 valid public decryption responses
      await decryption
        .connect(kmsTxSenders[0])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[0], extraDataV0);

      await decryption
        .connect(kmsTxSenders[1])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[1], extraDataV0);

      await decryption
        .connect(kmsTxSenders[2])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[2], extraDataV0);

      const decryptionAddress = await decryption.getAddress();

      // Create a malicious EIP712 message: the decryptedResult is different from the expected one
      // but the signature is valid (the malicious decryptedResult is given to the response call)
      const fakeEip712Message = createEIP712ResponsePublicDecrypt(
        gatewayChainId,
        decryptionAddress,
        ctHandles,
        fakeDecryptedResult,
        extraDataV0,
      );
      const [fakeKmsSignature] = await getSignaturesPublicDecrypt(fakeEip712Message, kmsSigners.slice(3, 4));

      // Trigger a fourth invalid public decryption response
      await decryption
        .connect(kmsTxSenders[3])
        .publicDecryptionResponse(decryptionId, fakeDecryptedResult, fakeKmsSignature, extraDataV0);

      const expectedKmsTxSenderAddresses = kmsTxSenders.slice(0, 3).map((s) => s.address);

      // Check that the KMS transaction senders that were involved in the consensus are the first 3
      // KMS transaction senders (the fourth one is ignored because the response is invalid)
      const decryptionConsensusTxSenders = await decryption.getDecryptionConsensusTxSenders(decryptionId);
      expect(decryptionConsensusTxSenders).to.deep.equal(expectedKmsTxSenderAddresses);
    });

    it("Should revert in case of invalid decryptionId in public decryption response", async function () {
      // Check that a public decryption response with a too low (invalid) decryptionId reverts
      await expect(
        decryption
          .connect(kmsTxSenders[0])
          .publicDecryptionResponse(tooLowDecryptionId, decryptedResult, kmsSignatures[0], extraDataV0),
      ).to.be.revertedWithCustomError(decryption, "DecryptionNotRequested");

      // Check that a public decryption response with too high (not requested yet) decryptionId reverts
      await expect(
        decryption
          .connect(kmsTxSenders[0])
          .publicDecryptionResponse(tooHighDecryptionId, decryptedResult, kmsSignatures[0], extraDataV0),
      ).to.be.revertedWithCustomError(decryption, "DecryptionNotRequested");
    });

    it("Should revert because the contract is paused", async function () {
      // Pause the contract
      await decryption.connect(pauser).pause();

      // Try calling paused public decryption request
      await expect(decryption.publicDecryptionRequest(ctHandles, extraDataV0)).to.be.revertedWithCustomError(
        decryption,
        "EnforcedPause",
      );
    });

    describe("Checks", function () {
      it("Should be true because public decryption is ready", async function () {
        expect(await decryption.isPublicDecryptionReady(ctHandles, extraDataV0)).to.be.true;
      });

      it("Should be false because handles have not been allowed for public decryption", async function () {
        expect(await decryption.isPublicDecryptionReady(newCtHandles, extraDataV0)).to.be.false;
      });

      it("Should be false because ciphertext material has not been added", async function () {
        expect(await decryption.isPublicDecryptionReady(newCtHandles, extraDataV0)).to.be.false;
      });

      it("Should be false because the public decryption is not done", async function () {
        expect(await decryption.isDecryptionDone(decryptionId)).to.be.false;
      });
    });
  });

  describe("User Decryption", function () {
    let userSignature: string;
    let userDecryptedShares: string[];
    let eip712RequestMessage: EIP712;
    let eip712ResponseMessages: EIP712[];

    // Expected decryption request ID (after a first request) for a user decryption request
    // The IDs won't increase between requests made in different "describe" sections as the blockchain
    // state is cleaned each time `loadFixture` is called
    const decryptionId = getUserDecryptId(1);

    // Create valid input values
    const user = createRandomWallet();
    const contractAddress = createRandomAddress();
    const publicKey = createByteInput();
    const startTimestamp = getDateInSeconds();
    const durationDays = 120;
    const contractsInfo: IDecryption.ContractsInfoStruct = {
      addresses: [contractAddress],
      chainId: hostChainId,
    };
    const requestValidity: IDecryption.RequestValidityStruct = {
      startTimestamp,
      durationDays,
    };

    // Define the ctHandleContractPairs (the handles have been added and allowed by default)
    const ctHandleContractPairs: CtHandleContractPairStruct[] = ctHandles.map((ctHandle) => ({
      contractAddress,
      ctHandle,
    }));

    // Define new valid inputs (the handles have neither been added nor allowed by default)
    const newCtHandleContractPair: CtHandleContractPairStruct = {
      contractAddress,
      ctHandle: newCtHandle,
    };

    // Define fake values
    const fakeUserAddress = createRandomAddress();
    const fakeContractAddresses = createRandomAddresses(3);
    const fakeContractAddress = fakeContractAddresses[0];
    const fakeContractAddressCtHandleContractPairs: CtHandleContractPairStruct[] = [
      {
        contractAddress: fakeContractAddress,
        ctHandle,
      },
    ];

    // Define utility values
    const tenDaysInSeconds = 10 * 24 * 60 * 60;

    // Allow access the the handles for the user and the contract
    async function prepareUserDecryptEIP712Fixture() {
      const fixtureData = await loadFixture(prepareAddCiphertextFixture);
      const { decryption, multichainACL, kmsSigners, coprocessorTxSenders } = fixtureData;

      // Allow user decryption for the user and contract address over all handles
      for (const ctHandle of ctHandles) {
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await multichainACL.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, user.address, extraDataV0);
          await multichainACL.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, contractAddress, extraDataV0);
        }
      }

      // Create EIP712 messages
      const decryptionAddress = await decryption.getAddress();
      const eip712RequestMessage = createEIP712RequestUserDecrypt(
        decryptionAddress,
        publicKey,
        contractsInfo.addresses as string[],
        contractsInfo.chainId as number,
        requestValidity.startTimestamp.toString(),
        requestValidity.durationDays.toString(),
        extraDataV0,
      );

      // Sign the message with the user
      const [userSignature] = await getSignaturesUserDecryptRequest(eip712RequestMessage, [user]);

      const userDecryptedShares = createBytes32s(kmsSigners.length);

      const eip712ResponseMessages = userDecryptedShares.map((userDecryptedShare) =>
        createEIP712ResponseUserDecrypt(
          gatewayChainId,
          decryptionAddress,
          publicKey,
          ctHandles,
          userDecryptedShare,
          extraDataV0,
        ),
      );

      // Sign the message with all KMS signers
      const kmsSignatures = await getSignaturesUserDecryptResponse(eip712ResponseMessages, kmsSigners);

      return {
        ...fixtureData,
        userDecryptedShares,
        eip712RequestMessage,
        eip712ResponseMessages,
        userSignature,
        kmsSignatures,
        requestValidity,
      };
    }

    beforeEach(async function () {
      // Initialize globally used variables before each test
      const fixtureData = await loadFixture(prepareUserDecryptEIP712Fixture);
      gatewayConfig = fixtureData.gatewayConfig;
      kmsGeneration = fixtureData.kmsGeneration;
      multichainACL = fixtureData.multichainACL;
      ciphertextCommits = fixtureData.ciphertextCommits;
      decryption = fixtureData.decryption;
      owner = fixtureData.owner;
      pauser = fixtureData.pauser;
      snsCiphertextMaterials = fixtureData.snsCiphertextMaterials;
      userSignature = fixtureData.userSignature;
      kmsSignatures = fixtureData.kmsSignatures;
      kmsTxSenders = fixtureData.kmsTxSenders;
      kmsSigners = fixtureData.kmsSigners;
      coprocessorTxSenders = fixtureData.coprocessorTxSenders;
      userDecryptedShares = fixtureData.userDecryptedShares;
      eip712RequestMessage = fixtureData.eip712RequestMessage;
      eip712ResponseMessages = fixtureData.eip712ResponseMessages;
    });

    it("Should request a user decryption with multiple ctHandleContractPairs", async function () {
      // Request user decryption
      const requestTx = await decryption.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        contractsInfo,
        user.address,
        publicKey,
        userSignature,
        extraDataV0,
      );

      // Check request event
      await expect(requestTx)
        .to.emit(decryption, "UserDecryptionRequest")
        .withArgs(decryptionId, toValues(snsCiphertextMaterials), user.address, publicKey, extraDataV0);
    });

    it("Should request a user decryption with a single ctHandleContractPair", async function () {
      // Create single list of inputs
      const singleCtHandleContractPair: CtHandleContractPairStruct[] = ctHandleContractPairs.slice(0, 1);
      const singleSnsCiphertextMaterials = snsCiphertextMaterials.slice(0, 1);

      // Request user decryption
      const requestTx = await decryption.userDecryptionRequest(
        singleCtHandleContractPair,
        requestValidity,
        contractsInfo,
        user.address,
        publicKey,
        userSignature,
        extraDataV0,
      );

      // Check request event
      await expect(requestTx)
        .to.emit(decryption, "UserDecryptionRequest")
        .withArgs(decryptionId, toValues(singleSnsCiphertextMaterials), user.address, publicKey, extraDataV0);
    });

    it("Should revert because ctHandleContractPairs is empty", async function () {
      await expect(
        decryption.userDecryptionRequest(
          [],
          requestValidity,
          contractsInfo,
          user.address,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      ).to.be.revertedWithCustomError(decryption, "EmptyCtHandleContractPairs");
    });

    it("Should revert because contract addresses is empty", async function () {
      const emptyContractsInfo: IDecryption.ContractsInfoStruct = {
        addresses: [],
        chainId: hostChainId,
      };
      await expect(
        decryption.userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          emptyContractsInfo,
          user.address,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      ).to.be.revertedWithCustomError(decryption, "EmptyContractAddresses");
    });

    it("Should revert because contract addresses exceeds maximum length allowed", async function () {
      // Create a list of contract addresses exceeding the maximum length allowed
      const largeContractsInfo: IDecryption.ContractsInfoStruct = {
        addresses: createRandomAddresses(15),
        chainId: hostChainId,
      };

      await expect(
        decryption.userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          largeContractsInfo,
          user.address,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "ContractAddressesMaxLengthExceeded")
        .withArgs(MAX_USER_DECRYPT_CONTRACT_ADDRESSES, largeContractsInfo.addresses.length);
    });

    it("Should revert because durationDays is null", async function () {
      // Create an invalid validity request with a durationDays that is 0
      const invalidRequestValidity: IDecryption.RequestValidityStruct = {
        startTimestamp,
        durationDays: 0,
      };

      await expect(
        decryption.userDecryptionRequest(
          ctHandleContractPairs,
          invalidRequestValidity,
          contractsInfo,
          user.address,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "InvalidNullDurationDays")
        .withArgs();
    });

    it("Should revert because durationDays exceeds maximum allowed", async function () {
      // Create an invalid validity request with a durationDays that exceeds the maximum allowed
      const largeDurationDays = MAX_USER_DECRYPT_DURATION_DAYS + 1;
      const invalidRequestValidity: IDecryption.RequestValidityStruct = {
        startTimestamp,
        durationDays: largeDurationDays,
      };

      await expect(
        decryption.userDecryptionRequest(
          ctHandleContractPairs,
          invalidRequestValidity,
          contractsInfo,
          user.address,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "MaxDurationDaysExceeded")
        .withArgs(MAX_USER_DECRYPT_DURATION_DAYS, largeDurationDays);
    });

    it("Should revert because the start timestamp is in the future", async function () {
      // Create an invalid validity request with a start timestamp in the future by delaying it by 10 days
      const futureRequestValidity: IDecryption.RequestValidityStruct = {
        startTimestamp: startTimestamp + tenDaysInSeconds,
        durationDays,
      };

      // We do not check the actual values in the error message as the block.timestamp will change
      // between the request and the error emission
      await expect(
        decryption.userDecryptionRequest(
          ctHandleContractPairs,
          futureRequestValidity,
          contractsInfo,
          user.address,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      ).to.be.revertedWithCustomError(decryption, "StartTimestampInFuture");
    });

    it("Should revert because the user decryption request has expired", async function () {
      // Create a expired validity request.
      // Note that we currently allow a past start timestamp. Here, we set it 10 days in the past,
      // but we allow the request for 1 day only
      const expiredRequestValidity: IDecryption.RequestValidityStruct = {
        startTimestamp: startTimestamp - tenDaysInSeconds,
        durationDays: 1,
      };

      // We do not check the actual values in the error message as the block.timestamp will change
      // between the request and the error emission
      await expect(
        decryption.userDecryptionRequest(
          ctHandleContractPairs,
          expiredRequestValidity,
          contractsInfo,
          user.address,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      ).to.be.revertedWithCustomError(decryption, "UserDecryptionRequestExpired");
    });

    it("Should revert because handle represents an invalid FHE type", async function () {
      // Create an input containing a single handle with an invalid FHE type
      const invalidFHETypeCtHandleContractPairs: CtHandleContractPairStruct[] = [
        {
          contractAddress,
          ctHandle: invalidFHETypeCtHandle,
        },
      ];

      // Check that the request fails because the ctHandle found in the ctHandleContractPairs
      // represents an unsupported FHE type
      // Note that the user signature is not correct here but the FHE type validity is checked first
      await expect(
        decryption.userDecryptionRequest(
          invalidFHETypeCtHandleContractPairs,
          requestValidity,
          contractsInfo,
          user.address,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "InvalidFHEType")
        .withArgs(invalidFHEType);
    });

    it("Should revert because handle represents an unsupported FHE type", async function () {
      // Create an input containing a single handle with an unsupported FHE type
      const unsupportedFHETypeCtHandleContractPairs: CtHandleContractPairStruct[] = [
        {
          contractAddress,
          ctHandle: unsupportedFHETypeCtHandle,
        },
      ];

      // Check that the request fails because the ctHandle found in the ctHandleContractPairs
      // represents an unsupported FHE type
      // Note that the user signature is not correct here but the FHE type validity is checked first
      await expect(
        decryption.userDecryptionRequest(
          unsupportedFHETypeCtHandleContractPairs,
          requestValidity,
          contractsInfo,
          user.address,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "UnsupportedFHEType")
        .withArgs(unsupportedFHEType);
    });

    it("Should revert because total bit size exceeds the maximum allowed", async function () {
      // Build a ctHandleContractPair containing the euint256 handle (which has a bit size of 256 bits)
      const euint256CtHandleContractPair: CtHandleContractPairStruct = {
        contractAddress,
        ctHandle: euint256CtHandle,
      };

      // Create a list of 12 euint256 ctHandles (each has a bit size of 256 bits)
      const numCtHandles = 12;
      const largeByteSizeCtHandleContractPairs = Array(numCtHandles).fill(euint256CtHandleContractPair);

      // Calculate the new total bit size of this list
      const totalBitSize = numCtHandles * 256;

      // Check that the request fails because the total bit size exceeds the maximum allowed
      // Note that the user signature is not correct here but the FHE type validity is checked first
      await expect(
        decryption.userDecryptionRequest(
          largeByteSizeCtHandleContractPairs,
          requestValidity,
          contractsInfo,
          user.address,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "MaxDecryptionRequestBitSizeExceeded")
        .withArgs(MAX_DECRYPTION_REQUEST_BITS, totalBitSize);
    });

    it("Should revert because the user address is a contract address", async function () {
      // Define fake ctHandleContractPairs with user address as contract address
      const userAddressCtHandleContractPairs: CtHandleContractPairStruct[] = [
        {
          contractAddress: user.address,
          ctHandle,
        },
      ];

      // Include the user address in the list of contract addresses
      const userInContractsInfo: IDecryption.ContractsInfoStruct = {
        addresses: [user.address],
        chainId: hostChainId,
      };

      await expect(
        decryption.userDecryptionRequest(
          userAddressCtHandleContractPairs,
          requestValidity,
          userInContractsInfo,
          user.address,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "UserAddressInContractAddresses")
        .withArgs(user.address, userInContractsInfo.addresses);
    });

    it("Should revert because the user is not allowed for user decryption on a ctHandle", async function () {
      await expect(
        decryption.userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          contractsInfo,
          fakeUserAddress,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "AccountNotAllowedToUseCiphertext")
        .withArgs(ctHandleContractPairs[0].ctHandle, fakeUserAddress);
    });

    it("Should revert because a contract is not allowed for user decryption on a ctHandle", async function () {
      await expect(
        decryption.userDecryptionRequest(
          fakeContractAddressCtHandleContractPairs,
          requestValidity,
          contractsInfo,
          user.address,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "AccountNotAllowedToUseCiphertext")
        .withArgs(fakeContractAddressCtHandleContractPairs[0].ctHandle, fakeContractAddress);
    });

    it("Should revert because ciphertext material has not been added", async function () {
      // Allow access to the handle for the user and contract accounts
      // We need to do this because `userDecryptionRequest` first checks if the accounts have access
      // to the handle
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await multichainACL.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, user.address, extraDataV0);
        await multichainACL.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, contractAddress, extraDataV0);
      }

      await expect(
        decryption.userDecryptionRequest(
          [newCtHandleContractPair],
          requestValidity,
          contractsInfo,
          user.address,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      )
        .to.be.revertedWithCustomError(ciphertextCommits, "CiphertextMaterialNotFound")
        .withArgs(newCtHandle);
    });

    it("Should revert because of invalid EIP712 user request signature", async function () {
      // Sign the message with the user
      const [fakeSignature] = await getSignaturesUserDecryptRequest(eip712RequestMessage, [fakeSigner]);

      // Request user decryption
      const requestTx = decryption.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        contractsInfo,
        user.address,
        publicKey,
        fakeSignature,
        extraDataV0,
      );

      // Check request event
      await expect(requestTx).to.be.revertedWithCustomError(decryption, "InvalidUserSignature").withArgs(fakeSignature);
    });

    it("Should revert because the response signer is not a registered KMS signer", async function () {
      // Request user decryption
      // This step is necessary, else the decryptionId won't be set in the state and the
      // signature verification will use wrong handles
      await decryption.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        contractsInfo,
        user.address,
        publicKey,
        userSignature,
        extraDataV0,
      );

      // Create a fake signature from the fake signer
      const [fakeSignature] = await getSignaturesUserDecryptResponse(eip712ResponseMessages.slice(0, 1), [fakeSigner]);

      // Check that the transaction fails because the signer is not a registered KMS signer
      await expect(
        decryption
          .connect(kmsTxSenders[0])
          .userDecryptionResponse(decryptionId, userDecryptedShares[0], fakeSignature, extraDataV0),
      )
        .to.be.revertedWithCustomError(decryption, "NotKmsSigner")
        .withArgs(fakeSigner.address);
    });

    it("Should revert because the message sender is not a KMS transaction sender", async function () {
      // Check that the transaction fails because the msg.sender is not a registered KMS transaction sender
      await expect(
        decryption
          .connect(fakeTxSender)
          .userDecryptionResponse(decryptionId, userDecryptedShares[0], kmsSignatures[0], extraDataV0),
      )
        .to.be.revertedWithCustomError(decryption, "NotKmsTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should revert because contract in ctHandleContractPairs not included in contractAddresses list", async function () {
      const fakeContractsInfo: IDecryption.ContractsInfoStruct = {
        addresses: fakeContractAddresses,
        chainId: hostChainId,
      };

      // Create EIP712 message using the fake contract address list
      const decryptionAddress = await decryption.getAddress();
      const fakeEip712RequestMessage = createEIP712RequestUserDecrypt(
        decryptionAddress,
        publicKey,
        fakeContractsInfo.addresses as string[],
        fakeContractsInfo.chainId as number,
        requestValidity.startTimestamp.toString(),
        requestValidity.durationDays.toString(),
        extraDataV0,
      );

      // Sign the message with the user
      const [fakeUserSignature] = await getSignaturesUserDecryptRequest(fakeEip712RequestMessage, [user]);

      // Request user decryption
      const requestTx = decryption.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        fakeContractsInfo,
        user.address,
        publicKey,
        fakeUserSignature,
        extraDataV0,
      );

      // Check that the request fails because the contract address is not included in the contractAddresses list
      await expect(requestTx)
        .to.be.revertedWithCustomError(decryption, "ContractNotInContractAddresses")
        .withArgs(contractAddress, fakeContractAddresses);
    });

    it("Should revert because of ctMaterials tied to different key IDs", async function () {
      // Store the handle with a new key ID and allow the user and contract accounts to use it
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await ciphertextCommits
          .connect(coprocessorTxSenders[i])
          .addCiphertextMaterial(newCtHandle, newKeyId, ciphertextDigest, snsCiphertextDigest);
        await multichainACL.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, user.address, extraDataV0);
        await multichainACL.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, contractAddress, extraDataV0);
      }

      // Request user decryption with ctMaterials tied to different key IDs
      const requestTx = decryption.userDecryptionRequest(
        [...ctHandleContractPairs, newCtHandleContractPair],
        requestValidity,
        contractsInfo,
        user.address,
        publicKey,
        userSignature,
        extraDataV0,
      );

      // Check that different key IDs are not allowed for batched user decryption
      await expect(requestTx)
        .to.revertedWithCustomError(decryption, "DifferentKeyIdsNotAllowed")
        .withArgs(
          toValues(snsCiphertextMaterials[0]),
          toValues({
            ctHandle: newCtHandle,
            keyId: newKeyId,
            snsCiphertextDigest,
            coprocessorTxSenderAddresses: coprocessorTxSenders.map((s) => s.address),
          }),
        );
    });

    it("Should revert because of two responses with same signature", async function () {
      // Request user decryption
      await decryption.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        contractsInfo,
        user.address,
        publicKey,
        userSignature,
        extraDataV0,
      );

      // Trigger a first user decryption response
      await decryption
        .connect(kmsTxSenders[0])
        .userDecryptionResponse(decryptionId, userDecryptedShares[0], kmsSignatures[0], extraDataV0);

      // Check that a KMS node cannot sign a second time for the same user decryption
      await expect(
        decryption
          .connect(kmsTxSenders[0])
          .userDecryptionResponse(decryptionId, userDecryptedShares[0], kmsSignatures[0], extraDataV0),
      )
        .to.be.revertedWithCustomError(decryption, "KmsNodeAlreadySigned")
        .withArgs(decryptionId, kmsSigners[0].address);
    });

    it("Should user decrypt with 3 valid responses", async function () {
      // Request user decryption
      await decryption.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        contractsInfo,
        user.address,
        publicKey,
        userSignature,
        extraDataV0,
      );

      // Trigger three valid user decryption responses using different KMS transaction senders
      const responseTx1 = await decryption
        .connect(kmsTxSenders[0])
        .userDecryptionResponse(decryptionId, userDecryptedShares[0], kmsSignatures[0], extraDataV0);

      const responseTx2 = await decryption
        .connect(kmsTxSenders[1])
        .userDecryptionResponse(decryptionId, userDecryptedShares[1], kmsSignatures[1], extraDataV0);

      const responseTx3 = await decryption
        .connect(kmsTxSenders[2])
        .userDecryptionResponse(decryptionId, userDecryptedShares[2], kmsSignatures[2], extraDataV0);

      // Check UserDecryptionResponse events are emitted for each response
      await expect(responseTx1)
        .to.emit(decryption, "UserDecryptionResponse")
        .withArgs(decryptionId, 0n, userDecryptedShares[0], kmsSignatures[0], extraDataV0);
      await expect(responseTx2)
        .to.emit(decryption, "UserDecryptionResponse")
        .withArgs(decryptionId, 1n, userDecryptedShares[1], kmsSignatures[1], extraDataV0);
      await expect(responseTx3)
        .to.emit(decryption, "UserDecryptionResponse")
        .withArgs(decryptionId, 2n, userDecryptedShares[2], kmsSignatures[2], extraDataV0);

      // Threshold should be reached at the third response (reconstruction threshold)
      // Check 3rd response event: it should emit the threshold reached event
      await expect(responseTx3).to.emit(decryption, "UserDecryptionResponseThresholdReached").withArgs(decryptionId);

      // Check that the user decryption is done
      expect(await decryption.isDecryptionDone(decryptionId)).to.be.true;
    });

    it("Should user decrypt with 3 valid responses and ignore the other valid one", async function () {
      // Request user decryption
      await decryption.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        contractsInfo,
        user.address,
        publicKey,
        userSignature,
        extraDataV0,
      );

      // Trigger 4 valid user decryption responses using different KMS transaction senders
      const responseTx1 = await decryption
        .connect(kmsTxSenders[0])
        .userDecryptionResponse(decryptionId, userDecryptedShares[0], kmsSignatures[0], extraDataV0);

      const responseTx2 = await decryption
        .connect(kmsTxSenders[1])
        .userDecryptionResponse(decryptionId, userDecryptedShares[1], kmsSignatures[1], extraDataV0);

      await decryption
        .connect(kmsTxSenders[2])
        .userDecryptionResponse(decryptionId, userDecryptedShares[2], kmsSignatures[2], extraDataV0);

      const responseTx4 = await decryption
        .connect(kmsTxSenders[3])
        .userDecryptionResponse(decryptionId, userDecryptedShares[3], kmsSignatures[3], extraDataV0);

      // Check that the 1st, 2nd and 4th responses do not emit an event:
      // - 1st and 2nd responses are ignored because threshold is not reached yet
      // - 4th response is ignored (not reverted) even though they are late
      await expect(responseTx1).to.not.emit(decryption, "UserDecryptionResponseThresholdReached");
      await expect(responseTx2).to.not.emit(decryption, "UserDecryptionResponseThresholdReached");
      await expect(responseTx4).to.not.emit(decryption, "UserDecryptionResponseThresholdReached");
    });

    // Note: there is no test with "malicious" responses for user decryption because all shares are
    // different and we do not do the reconstruction onchain, hence consensus only considers the
    // decryption IDs

    it("Should get all KMS transaction senders from user decryption consensus", async function () {
      // Request user decryption
      await decryption.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        contractsInfo,
        user.address,
        publicKey,
        userSignature,
        extraDataV0,
      );

      // Trigger a valid user decryption response using the first KMS transaction sender
      await decryption
        .connect(kmsTxSenders[0])
        .userDecryptionResponse(decryptionId, userDecryptedShares[0], kmsSignatures[0], extraDataV0);

      const expectedKmsTxSenderAddresses1 = kmsTxSenders.slice(0, 1).map((s) => s.address);

      // Get the KMS transaction sender that answered first, before the consensus is reached
      // Since consensus only depends on the decryption ID, the list represents the KMS transaction sender
      // that answered, and is accessible before the consensus is reached
      const decryptionConsensusTxSenders1 = await decryption.getDecryptionConsensusTxSenders(decryptionId);
      expect(decryptionConsensusTxSenders1).to.deep.equal(expectedKmsTxSenderAddresses1);

      // Trigger 2 valid user decryption responses using different KMS transaction senders
      await decryption
        .connect(kmsTxSenders[1])
        .userDecryptionResponse(decryptionId, userDecryptedShares[1], kmsSignatures[1], extraDataV0);

      await decryption
        .connect(kmsTxSenders[2])
        .userDecryptionResponse(decryptionId, userDecryptedShares[2], kmsSignatures[2], extraDataV0);

      const expectedKmsTxSenderAddresses2 = kmsTxSenders.slice(0, 3).map((s) => s.address);

      // Get the KMS transaction senders that were involved in the consensus, at the moment the consensus is reached
      const decryptionConsensusTxSenders2 = await decryption.getDecryptionConsensusTxSenders(decryptionId);
      expect(decryptionConsensusTxSenders2).to.deep.equal(expectedKmsTxSenderAddresses2);

      await decryption
        .connect(kmsTxSenders[3])
        .userDecryptionResponse(decryptionId, userDecryptedShares[3], kmsSignatures[3], extraDataV0);

      const expectedKmsTxSenderAddresses3 = kmsTxSenders.map((s) => s.address);

      // Get the KMS transaction senders that were involved in the consensus, after the consensus is reached
      const decryptionConsensusTxSenders3 = await decryption.getDecryptionConsensusTxSenders(decryptionId);
      expect(decryptionConsensusTxSenders3).to.deep.equal(expectedKmsTxSenderAddresses3);
    });

    it("Should revert in case of invalid decryptionId in user decryption response", async function () {
      // Check that a user decryption response with a too low (invalid) decryptionId reverts
      await expect(
        decryption
          .connect(kmsTxSenders[0])
          .userDecryptionResponse(tooLowDecryptionId, userDecryptedShares[0], kmsSignatures[0], extraDataV0),
      ).to.be.revertedWithCustomError(decryption, "DecryptionNotRequested");

      // Check that a user decryption response with too high (not requested yet) decryptionId reverts
      await expect(
        decryption
          .connect(kmsTxSenders[0])
          .userDecryptionResponse(tooHighDecryptionId, userDecryptedShares[0], kmsSignatures[0], extraDataV0),
      ).to.be.revertedWithCustomError(decryption, "DecryptionNotRequested");
    });

    it("Should revert because the contract is paused", async function () {
      // Pause the contract
      await decryption.connect(pauser).pause();

      // Try calling paused user decryption request
      await expect(
        decryption.userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          contractsInfo,
          user.address,
          publicKey,
          userSignature,
          extraDataV0,
        ),
      ).to.be.revertedWithCustomError(decryption, "EnforcedPause");
    });

    describe("Checks", function () {
      it("Should be true because user decryption is ready", async function () {
        expect(await decryption.isUserDecryptionReady(user.address, ctHandleContractPairs, extraDataV0)).to.be.true;
      });

      it("Should be false because the user is not allowed for user decryption on a ctHandle", async function () {
        expect(await decryption.isUserDecryptionReady(fakeUserAddress, ctHandleContractPairs, extraDataV0)).to.be.false;
      });

      it("Should be false because a contract is not allowed for user decryption on a ctHandle", async function () {
        expect(
          await decryption.isUserDecryptionReady(user.address, fakeContractAddressCtHandleContractPairs, extraDataV0),
        ).to.be.false;
      });

      it("Should be false because ciphertext material has not been added", async function () {
        expect(await decryption.isUserDecryptionReady(user.address, [newCtHandleContractPair], extraDataV0)).to.be
          .false;
      });

      it("Should be false because the user decryption is not done", async function () {
        expect(await decryption.isDecryptionDone(decryptionId)).to.be.false;
      });
    });
  });

  describe("Pause", async function () {
    beforeEach(async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      decryption = fixtureData.decryption;
      owner = fixtureData.owner;
      pauser = fixtureData.pauser;
    });

    it("Should pause the contract with the pauser and unpause with the owner", async function () {
      // Check that the contract is not paused
      expect(await decryption.paused()).to.be.false;

      // Pause the contract with the pauser address
      await expect(decryption.connect(pauser).pause()).to.emit(decryption, "Paused").withArgs(pauser);
      expect(await decryption.paused()).to.be.true;

      // Unpause the contract with the owner address
      await expect(decryption.connect(owner).unpause()).to.emit(decryption, "Unpaused").withArgs(owner);
      expect(await decryption.paused()).to.be.false;
    });

    it("Should revert on pause because sender is not the pauser", async function () {
      const fakePauser = createRandomWallet();

      await expect(decryption.connect(fakePauser).pause())
        .to.be.revertedWithCustomError(decryption, "NotPauserOrGatewayConfig")
        .withArgs(fakePauser.address);
    });

    it("Should revert on unpause because sender is not the owner", async function () {
      // Pause the contract with the pauser address
      await decryption.connect(pauser).pause();

      const fakeOwner = createRandomWallet();

      await expect(decryption.connect(fakeOwner).unpause())
        .to.be.revertedWithCustomError(decryption, "NotOwnerOrGatewayConfig")
        .withArgs(fakeOwner.address);
    });
  });
});
