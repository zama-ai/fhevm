import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { BigNumberish, EventLog, Wallet } from "ethers";
import hre from "hardhat";

import {
  CiphertextCommits,
  Decryption,
  Decryption__factory,
  GatewayConfig,
  IDecryption,
  KmsManagement,
  MultichainAcl,
} from "../typechain-types";
// The type needs to be imported separately because it is not properly detected by the linter
// as this type is defined as a shared structs instead of directly in the IDecryption interface
import {
  CtHandleContractPairStruct,
  DelegationAccountsStruct,
  SnsCiphertextMaterialStruct,
} from "../typechain-types/contracts/interfaces/IDecryption";
import {
  EIP712,
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
  getSignaturesDelegatedUserDecryptRequest,
  getSignaturesPublicDecrypt,
  getSignaturesUserDecryptRequest,
  getSignaturesUserDecryptResponse,
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

// Create a new key, rotate it and activate it. It returns the new key ID.
async function createAndRotateKey(
  sourceKeyId: BigNumberish,
  kmsManagement: KmsManagement,
  owner: Wallet,
  coprocessorTxSenders: HardhatEthersSigner[],
  kmsTxSenders: HardhatEthersSigner[],
  fheParamsName: string,
): Promise<BigNumberish> {
  const newKeyId = hre.ethers.toBigInt(hre.ethers.randomBytes(32));
  // Trigger a preprocessing keygen request
  let txRequest = await kmsManagement.connect(owner).preprocessKeygenRequest(fheParamsName);

  // Get the preKeyRequestId from the event in the transaction receipt
  let receipt = await txRequest.wait();
  let event = receipt?.logs[0] as EventLog;
  const preKeyRequestId = Number(event?.args[0]);

  // Define a preKeyId for the preprocessing keygen response
  const preKeyId = hre.ethers.toBigInt(hre.ethers.randomBytes(32));

  // Trigger preprocessing keygen responses for all KMS nodes
  for (let i = 0; i < kmsTxSenders.length; i++) {
    await kmsManagement.connect(kmsTxSenders[i]).preprocessKeygenResponse(preKeyRequestId, preKeyId);
  }

  // Trigger a keygen request
  await kmsManagement.connect(owner).keygenRequest(preKeyId);

  // Trigger keygen responses for all KMS nodes
  for (let i = 0; i < kmsTxSenders.length; i++) {
    await kmsManagement.connect(kmsTxSenders[i]).keygenResponse(preKeyId, newKeyId);
  }

  // Trigger a preprocessing kskgen request
  txRequest = await kmsManagement.connect(owner).preprocessKskgenRequest(fheParamsName);

  // Get the preKeyRequestId from the event in the transaction receipt
  receipt = await txRequest.wait();
  event = receipt?.logs[0] as EventLog;
  const preKskRequestId = Number(event?.args[0]);

  // Define a preKskId for the preprocessing kskgen response
  const preKskId = hre.ethers.toBigInt(hre.ethers.randomBytes(32));

  // Trigger preprocessing kskgen responses for all KMS nodes
  for (let i = 0; i < kmsTxSenders.length; i++) {
    await kmsManagement.connect(kmsTxSenders[i]).preprocessKskgenResponse(preKskRequestId, preKskId);
  }

  // Trigger a kskgen request
  await kmsManagement.connect(owner).kskgenRequest(preKskId, sourceKeyId, newKeyId);

  // Define a kskId for kskgen response
  const kskId = hre.ethers.toBigInt(hre.ethers.randomBytes(32));

  // Trigger kskgen responses for all KMS nodes
  for (let i = 0; i < kmsTxSenders.length; i++) {
    await kmsManagement.connect(kmsTxSenders[i]).kskgenResponse(preKskId, kskId);
  }

  // Request activation of the key
  await kmsManagement.connect(owner).activateKeyRequest(newKeyId);

  // Trigger activation responses for all coprocessors
  for (let i = 0; i < coprocessorTxSenders.length; i++) {
    await kmsManagement.connect(coprocessorTxSenders[i]).activateKeyResponse(newKeyId);
  }

  return newKeyId;
}

describe("Decryption", function () {
  // Get the registered host chains' chain IDs
  const hostChainIds = loadHostChainIds();
  const hostChainId = hostChainIds[0];

  // Get the gateway's chain ID
  const gatewayChainId = hre.network.config.chainId!;

  // Expected decryption request ID (after a first request) for each kind of decryption request
  // The IDs won't increase between requests made in different "describe" sections as the blockchain
  // state is cleaned each time `loadFixture` is called
  const decryptionId = 1;

  // Define input values
  const ciphertextDigest = createBytes32();
  const snsCiphertextDigest = createBytes32();

  // Define the ebytes128 ctHandle (which has a bit size of 1024 bits)
  const ebytes128CtHandle = createCtHandle(hostChainId, 10);

  // Create ciphertext handles for the host chain ID with different TFHE-rs types
  // Note that the list is made so that the total bit size represented by these handles (1034 bits)
  // does not exceed 2048 bits (the maximum allowed for a single list of handles)
  const ctHandles = [createCtHandle(hostChainId, 0), createCtHandle(hostChainId, 2), ebytes128CtHandle];
  const ctHandle = ctHandles[0];

  // Define other valid ctHandles (they will not be added in the ciphertext commits contract and allowed for
  // public decryption or account access by default)
  const newCtHandles = createCtHandles(3, hostChainId);
  const newCtHandle = newCtHandles[0];

  // Define a handle with an invalid FHE type (see `FheType.sol`)
  const invalidFHEType = 255;
  const invalidFHETypeCtHandle = createCtHandle(hostChainId, invalidFHEType);

  // Define a handle with an unsupported FHE type (see `FHETypeBitSizes.sol`)
  const unsupportedFHEType = 13;
  const unsupportedFHETypeCtHandle = createCtHandle(hostChainId, unsupportedFHEType);

  // Define fake values
  const fakeTxSender = createRandomWallet();
  const fakeSigner = createRandomWallet();

  let gatewayConfig: GatewayConfig;
  let kmsManagement: KmsManagement;
  let multichainAcl: MultichainAcl;
  let ciphertextCommits: CiphertextCommits;
  let decryption: Decryption;
  let owner: Wallet;
  let pauser: HardhatEthersSigner;
  let snsCiphertextMaterials: SnsCiphertextMaterialStruct[];
  let kmsSignatures: string[];
  let kmsTxSenders: HardhatEthersSigner[];
  let kmsSigners: HardhatEthersSigner[];
  let coprocessorTxSenders: HardhatEthersSigner[];
  let keyId1: BigNumberish;
  let fheParamsName: string;
  let coprocessorContextId: BigNumberish;

  // Trigger a key generation in KmsManagement contract and activate the key
  async function prepareAddCiphertextFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const { kmsManagement, ciphertextCommits, owner, kmsTxSenders, coprocessorTxSenders, fheParamsName } = fixtureData;

    // Trigger a preprocessing keygen request
    const txRequest = await kmsManagement.connect(owner).preprocessKeygenRequest(fheParamsName);

    // Get the preKeyRequestId from the event in the transaction receipt
    const receipt = await txRequest.wait();
    const event = receipt?.logs[0] as EventLog;
    const preKeyRequestId = Number(event?.args[0]);

    // Define a preKeyId for the preprocessing keygen response
    const preKeyId = 1;

    // Trigger preprocessing keygen responses for all KMS nodes
    for (let i = 0; i < kmsTxSenders.length; i++) {
      await kmsManagement.connect(kmsTxSenders[i]).preprocessKeygenResponse(preKeyRequestId, preKeyId);
    }

    // Trigger a keygen request
    await kmsManagement.connect(owner).keygenRequest(preKeyId);

    // Define a keyId for keygen response
    const keyId1 = 1;

    // Trigger keygen responses for all KMS nodes
    for (let i = 0; i < kmsTxSenders.length; i++) {
      await kmsManagement.connect(kmsTxSenders[i]).keygenResponse(preKeyId, keyId1);
    }

    // Request activation of the key
    await kmsManagement.connect(owner).activateKeyRequest(keyId1);

    // Trigger activation responses for all coprocessors
    for (let i = 0; i < coprocessorTxSenders.length; i++) {
      await kmsManagement.connect(coprocessorTxSenders[i]).activateKeyResponse(keyId1);
    }

    // Define the first coprocessor context ID
    const coprocessorContextId = 1;

    let snsCiphertextMaterials: SnsCiphertextMaterialStruct[] = [];

    // Allow public decryption
    for (const ctHandle of ctHandles) {
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await ciphertextCommits
          .connect(coprocessorTxSenders[i])
          .addCiphertextMaterial(ctHandle, keyId1, ciphertextDigest, snsCiphertextDigest);
      }

      // Store the SNS ciphertext materials for event checks
      snsCiphertextMaterials.push({
        ctHandle,
        keyId: keyId1,
        snsCiphertextDigest,
        coprocessorTxSenderAddresses: coprocessorTxSenders.map((s) => s.address),
        coprocessorContextId,
      });
    }

    return { ...fixtureData, snsCiphertextMaterials, keyId1, coprocessorContextId };
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

    // Create input values
    const decryptedResult = createBytes32();

    // Allow handles for public decryption
    async function preparePublicDecryptEIP712Fixture() {
      const fixtureData = await loadFixture(prepareAddCiphertextFixture);
      const { multichainAcl, decryption, kmsSigners, coprocessorTxSenders } = fixtureData;

      // Allow public decryption
      for (const ctHandle of ctHandles) {
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await multichainAcl.connect(coprocessorTxSenders[i]).allowPublicDecrypt(ctHandle);
        }
      }

      // Create EIP712 messages
      const decryptionAddress = await decryption.getAddress();
      const eip712Message = createEIP712ResponsePublicDecrypt(
        gatewayChainId,
        decryptionAddress,
        ctHandles,
        decryptedResult,
      );

      // Sign the message with all KMS signers
      const kmsSignatures = await getSignaturesPublicDecrypt(eip712Message, kmsSigners);

      return { ...fixtureData, eip712Message, kmsSignatures };
    }

    beforeEach(async function () {
      // Initialize globally used variables before each test
      const fixtureData = await loadFixture(preparePublicDecryptEIP712Fixture);
      gatewayConfig = fixtureData.gatewayConfig;
      kmsManagement = fixtureData.kmsManagement;
      multichainAcl = fixtureData.multichainAcl;
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
      keyId1 = fixtureData.keyId1;
      fheParamsName = fixtureData.fheParamsName;
      coprocessorContextId = fixtureData.coprocessorContextId;
    });

    it("Should request a public decryption with multiple ctHandles", async function () {
      // Request public decryption
      const requestTx = await decryption.publicDecryptionRequest(ctHandles);

      // Check request event
      await expect(requestTx)
        .to.emit(decryption, "PublicDecryptionRequest")
        .withArgs(decryptionId, toValues(snsCiphertextMaterials));
    });

    it("Should request a public decryption with a single ctHandle", async function () {
      // Request public decryption with a single ctHandle
      const requestTx = await decryption.publicDecryptionRequest([ctHandles[0]]);

      const singleSnsCiphertextMaterials = snsCiphertextMaterials.slice(0, 1);

      // Check request event
      await expect(requestTx)
        .to.emit(decryption, "PublicDecryptionRequest")
        .withArgs(decryptionId, toValues(singleSnsCiphertextMaterials));
    });

    it("Should revert because ctHandles list is empty", async function () {
      // Check that the request fails because the list of handles is empty
      await expect(decryption.publicDecryptionRequest([])).to.be.revertedWithCustomError(decryption, "EmptyCtHandles");
    });

    it("Should revert because handle represents an invalid FHE type", async function () {
      // Check that the request fails because the ctHandle represents an invalid FHE type
      await expect(decryption.publicDecryptionRequest([invalidFHETypeCtHandle]))
        .to.be.revertedWithCustomError(decryption, "InvalidFHEType")
        .withArgs(invalidFHEType);
    });

    it("Should revert because handle represents an unsupported FHE type", async function () {
      // Check that the request fails because the ctHandle represents an unsupported FHE type
      await expect(decryption.publicDecryptionRequest([unsupportedFHETypeCtHandle]))
        .to.be.revertedWithCustomError(decryption, "UnsupportedFHEType")
        .withArgs(unsupportedFHEType);
    });

    it("Should revert because total bit size exceeds the maximum allowed", async function () {
      // Create a list of 3 ebytes128 ctHandles (each has a bit size of 1024 bits)
      const largeBitSizeCtHandles = [ebytes128CtHandle, ebytes128CtHandle, ebytes128CtHandle];

      // Calculate the new total bit size of this list
      const totalBitSize = 3072;

      // Check that the request fails because the total bit size exceeds the maximum allowed
      await expect(decryption.publicDecryptionRequest(largeBitSizeCtHandles))
        .to.be.revertedWithCustomError(decryption, "MaxDecryptionRequestBitSizeExceeded")
        .withArgs(MAX_DECRYPTION_REQUEST_BITS, totalBitSize);
    });

    it("Should revert because handles are not allowed for public decryption", async function () {
      // Check that the request fails because the handles are not allowed for public decryption
      await expect(decryption.publicDecryptionRequest(newCtHandles))
        .to.be.revertedWithCustomError(multichainAcl, "PublicDecryptNotAllowed")
        .withArgs(newCtHandles[0]);
    });

    it("Should revert because ciphertext material has not been added", async function () {
      // Allow public decryption for handles that have not been added
      // We need to do this because `checkPublicDecryptionReady` first checks if the handles
      // have been allowed for public decryption
      for (const newCtHandle of newCtHandles) {
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await multichainAcl.connect(coprocessorTxSenders[i]).allowPublicDecrypt(newCtHandle);
        }
      }

      // Check that the request fails because the ciphertext material is unavailable
      await expect(decryption.publicDecryptionRequest(newCtHandles))
        .to.be.revertedWithCustomError(ciphertextCommits, "CiphertextMaterialNotFound")
        .withArgs(newCtHandles[0]);
    });

    it("Should revert because the message sender is not a KMS transaction sender", async function () {
      // Check that the transaction fails because the msg.sender is not a registered KMS transaction sender
      await expect(
        decryption.connect(fakeTxSender).publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[0]),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "NotKmsTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should revert because the signer is not a KMS signer", async function () {
      // Request public decryption
      // This step is necessary, else the decryptionId won't be set in the state and the
      // signature verification will use wrong handles
      await decryption.publicDecryptionRequest(ctHandles);

      // Create a fake signature from the fake signer
      const [fakeSignature] = await getSignaturesPublicDecrypt(eip712Message, [fakeSigner]);

      // Check that the signature verification fails because the signer is not a registered KMS signer
      await expect(
        decryption.connect(kmsTxSenders[0]).publicDecryptionResponse(decryptionId, decryptedResult, fakeSignature),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "NotKmsSigner")
        .withArgs(fakeSigner.address);
    });

    it("Should revert because of two responses with same signature", async function () {
      // Request public decryption
      await decryption.publicDecryptionRequest(ctHandles);

      // Trigger a first public decryption response
      await decryption
        .connect(kmsTxSenders[0])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[0]);

      // Check that a KMS node cannot sign a second time for the same public decryption
      await expect(
        decryption.connect(kmsTxSenders[0]).publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[0]),
      )
        .to.be.revertedWithCustomError(decryption, "KmsNodeAlreadySigned")
        .withArgs(decryptionId, kmsSigners[0].address);
    });

    it("Should revert because of ctMaterials tied to different key IDs", async function () {
      const keyId2 = await createAndRotateKey(
        keyId1,
        kmsManagement,
        owner,
        coprocessorTxSenders,
        kmsTxSenders,
        fheParamsName,
      );

      // Store the handles and allow them for public decryption
      for (const newCtHandle of newCtHandles) {
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await ciphertextCommits
            .connect(coprocessorTxSenders[i])
            .addCiphertextMaterial(newCtHandle, keyId2, ciphertextDigest, snsCiphertextDigest);

          await multichainAcl.connect(coprocessorTxSenders[i]).allowPublicDecrypt(newCtHandle);
        }
      }

      // Request public decryption with ctMaterials tied to different key IDs
      const requestTx = decryption.publicDecryptionRequest([...ctHandles, newCtHandle]);

      // Check that different key IDs are not allowed for batched public decryption
      await expect(requestTx)
        .to.be.revertedWithCustomError(decryption, "DifferentKeyIdsNotAllowed")
        .withArgs(
          toValues(snsCiphertextMaterials[0]),
          toValues({
            ctHandle: newCtHandle,
            keyId: keyId2,
            snsCiphertextDigest,
            coprocessorTxSenderAddresses: coprocessorTxSenders.map((s) => s.address),
            coprocessorContextId: coprocessorContextId,
          }),
        );
    });

    it("Should reach consensus with 3 valid responses", async function () {
      // Request public decryption
      await decryption.publicDecryptionRequest(ctHandles);

      // Trigger three valid public decryption responses
      await decryption
        .connect(kmsTxSenders[0])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[0]);
      await decryption
        .connect(kmsTxSenders[1])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[1]);

      const responseTx3 = await decryption
        .connect(kmsTxSenders[2])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[2]);

      // Consensus should be reached at the third response
      // Check 3rd response event: it should only contain 3 valid signatures
      await expect(responseTx3)
        .to.emit(decryption, "PublicDecryptionResponse")
        .withArgs(decryptionId, decryptedResult, [kmsSignatures[0], kmsSignatures[1], kmsSignatures[2]]);

      // Check that the public decryption is done
      await expect(decryption.checkDecryptionDone(decryptionId)).to.not.be.reverted;
    });

    it("Should ignore other valid responses", async function () {
      // Request public decryption
      await decryption.publicDecryptionRequest(ctHandles);

      // Trigger four valid public decryption responses
      const responseTx1 = await decryption
        .connect(kmsTxSenders[0])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[0]);

      const responseTx2 = await decryption
        .connect(kmsTxSenders[1])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[1]);

      await decryption
        .connect(kmsTxSenders[2])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[2]);

      const responseTx4 = await decryption
        .connect(kmsTxSenders[3])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[3]);

      // Check that the 1st, 2nd and 4th responses do not emit an event:
      // - 1st and 2nd responses are ignored because consensus is not reached yet
      // - 4th response is ignored (not reverted) even though it is late
      await expect(responseTx1).to.not.emit(decryption, "PublicDecryptionResponse");
      await expect(responseTx2).to.not.emit(decryption, "PublicDecryptionResponse");
      await expect(responseTx4).to.not.emit(decryption, "PublicDecryptionResponse");
    });

    it("Should revert because the contract is paused", async function () {
      // Pause the contract
      await decryption.connect(owner).pause();

      // Try calling paused public decryption request
      await expect(decryption.publicDecryptionRequest(ctHandles)).to.be.revertedWithCustomError(
        decryption,
        "EnforcedPause",
      );

      // Try calling paused public decryption response
      await expect(
        decryption.connect(kmsTxSenders[0]).publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[0]),
      ).to.be.revertedWithCustomError(decryption, "EnforcedPause");
    });

    it("Should public decrypt with 3 valid and 1 malicious signatures", async function () {
      // Request public decryption
      await decryption.publicDecryptionRequest(ctHandles);

      const decryptionAddress = await decryption.getAddress();

      // Create a malicious EIP712 message: the decryptedResult is different from the expected one
      // but the signature is valid (the malicious decryptedResult will be given to the response call)
      const fakeDecryptedResult = createBytes32();
      const fakeEip712Message = createEIP712ResponsePublicDecrypt(
        gatewayChainId,
        decryptionAddress,
        ctHandles,
        fakeDecryptedResult,
      );
      const [fakeKmsSignature] = await getSignaturesPublicDecrypt(fakeEip712Message, kmsSigners);

      // Trigger a malicious public decryption response with:
      // - the first KMS transaction sender (expected)
      // - a fake signature (unexpected)
      await decryption
        .connect(kmsTxSenders[0])
        .publicDecryptionResponse(decryptionId, fakeDecryptedResult, fakeKmsSignature);

      // Trigger a first valid public decryption response with:
      // - the second KMS transaction sender
      // - the second KMS signer's signature
      await decryption
        .connect(kmsTxSenders[1])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[1]);

      // Trigger a second valid public decryption response with:
      // - the third KMS transaction sender
      // - the third KMS signer's signature
      const responseTx3 = await decryption
        .connect(kmsTxSenders[2])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[2]);

      // Trigger a third valid proof verification response with:
      // - the fourth coprocessor transaction sender
      // - the fourth coprocessor signer's signature
      const responseTx4 = await decryption
        .connect(kmsTxSenders[3])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[3]);

      // Consensus should not be reached at the third transaction since the first was malicious
      // Check 3rd transaction events: it should not emit an event for public decryption response
      await expect(responseTx3).to.not.emit(decryption, "PublicDecryptionResponse");

      // Consensus should be reached at the fourth transaction
      // Check 4th transaction events: it should only contain 3 valid signatures
      await expect(responseTx4)
        .to.emit(decryption, "PublicDecryptionResponse")
        .withArgs(decryptionId, decryptedResult, kmsSignatures.slice(1, 4));
    });

    describe("Checks", function () {
      it("Should not revert because public decryption is ready", async function () {
        await expect(decryption.checkPublicDecryptionReady(ctHandles)).to.not.be.reverted;
      });

      it("Should revert because handles have not been allowed for public decryption", async function () {
        await expect(decryption.checkPublicDecryptionReady(newCtHandles))
          .to.be.revertedWithCustomError(multichainAcl, "PublicDecryptNotAllowed")
          .withArgs(newCtHandles[0]);
      });

      it("Should revert because ciphertext material has not been added", async function () {
        // Allow public decryption for handles that have not been added
        // We need to do this because `checkPublicDecryptionReady` first checks if the handles
        // have been allowed for public decryption
        for (const newCtHandle of newCtHandles) {
          for (let i = 0; i < coprocessorTxSenders.length; i++) {
            await multichainAcl.connect(coprocessorTxSenders[i]).allowPublicDecrypt(newCtHandle);
          }
        }

        await expect(decryption.checkPublicDecryptionReady(newCtHandles))
          .to.be.revertedWithCustomError(ciphertextCommits, "CiphertextMaterialNotFound")
          .withArgs(newCtHandles[0]);
      });

      it("Should revert because the public decryption is not done", async function () {
        await expect(decryption.checkDecryptionDone(decryptionId))
          .to.be.revertedWithCustomError(decryption, "DecryptionNotDone")
          .withArgs(decryptionId);
      });
    });
  });

  describe("User Decryption", function () {
    let userSignature: string;
    let userDecryptedShares: string[];
    let eip712RequestMessage: EIP712;
    let eip712ResponseMessages: EIP712[];

    // Create valid input values
    const user = createRandomWallet();
    const contractAddress = createRandomAddress();
    const contractAddresses = [contractAddress];
    const publicKey = createBytes32();
    const startTimestamp = getDateInSeconds();
    const durationDays = 120;
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
      const { decryption, multichainAcl, kmsSigners, coprocessorTxSenders } = fixtureData;

      // Allow user decryption for the user and contract address over all handles
      for (const ctHandle of ctHandles) {
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, user.address);
          await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, contractAddress);
        }
      }

      // Create EIP712 messages
      const decryptionAddress = await decryption.getAddress();
      const eip712RequestMessage = createEIP712RequestUserDecrypt(
        decryptionAddress,
        publicKey,
        contractAddresses,
        hostChainId,
        requestValidity.startTimestamp.toString(),
        requestValidity.durationDays.toString(),
      );

      // Sign the message with the user
      const [userSignature] = await getSignaturesUserDecryptRequest(eip712RequestMessage, [user]);

      const userDecryptedShares = createBytes32s(kmsSigners.length);

      const eip712ResponseMessages = userDecryptedShares.map((userDecryptedShare) =>
        createEIP712ResponseUserDecrypt(gatewayChainId, decryptionAddress, publicKey, ctHandles, userDecryptedShare),
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
      kmsManagement = fixtureData.kmsManagement;
      multichainAcl = fixtureData.multichainAcl;
      ciphertextCommits = fixtureData.ciphertextCommits;
      decryption = fixtureData.decryption;
      owner = fixtureData.owner;
      snsCiphertextMaterials = fixtureData.snsCiphertextMaterials;
      userSignature = fixtureData.userSignature;
      kmsSignatures = fixtureData.kmsSignatures;
      kmsTxSenders = fixtureData.kmsTxSenders;
      coprocessorTxSenders = fixtureData.coprocessorTxSenders;
      userDecryptedShares = fixtureData.userDecryptedShares;
      eip712RequestMessage = fixtureData.eip712RequestMessage;
      eip712ResponseMessages = fixtureData.eip712ResponseMessages;
      keyId1 = fixtureData.keyId1;
      fheParamsName = fixtureData.fheParamsName;
    });

    it("Should request a user decryption with multiple ctHandleContractPairs", async function () {
      // Request user decryption
      const requestTx = await decryption.userDecryptionRequest(
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
        .to.emit(decryption, "UserDecryptionRequest")
        .withArgs(decryptionId, toValues(snsCiphertextMaterials), user.address, publicKey);
    });

    it("Should request a user decryption with a single ctHandleContractPair", async function () {
      // Create single list of inputs
      const singleCtHandleContractPair: CtHandleContractPairStruct[] = ctHandleContractPairs.slice(0, 1);
      const singleSnsCiphertextMaterials = snsCiphertextMaterials.slice(0, 1);

      // Request user decryption
      const requestTx = await decryption.userDecryptionRequest(
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
        .to.emit(decryption, "UserDecryptionRequest")
        .withArgs(decryptionId, toValues(singleSnsCiphertextMaterials), user.address, publicKey);
    });

    it("Should revert because ctHandleContractPairs is empty", async function () {
      await expect(
        decryption.userDecryptionRequest(
          [],
          requestValidity,
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        ),
      ).to.be.revertedWithCustomError(decryption, "EmptyCtHandleContractPairs");
    });

    it("Should revert because contract addresses is empty", async function () {
      await expect(
        decryption.userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          hostChainId,
          [],
          user.address,
          publicKey,
          userSignature,
        ),
      ).to.be.revertedWithCustomError(decryption, "EmptyContractAddresses");
    });

    it("Should revert because contract addresses exceeds maximum length allowed", async function () {
      // Create a list of contract addresses exceeding the maximum length allowed
      const largeContractAddresses = createRandomAddresses(15);

      await expect(
        decryption.userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          hostChainId,
          largeContractAddresses,
          user.address,
          publicKey,
          userSignature,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "ContractAddressesMaxLengthExceeded")
        .withArgs(MAX_USER_DECRYPT_CONTRACT_ADDRESSES, largeContractAddresses.length);
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
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
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
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
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
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
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
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
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
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
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
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "UnsupportedFHEType")
        .withArgs(unsupportedFHEType);
    });

    it("Should revert because total bit size exceeds the maximum allowed", async function () {
      // Build a ctHandleContractPair containing the ebytes128 handle (which has a bit size of 1024 bits)
      const ebytes128CtHandleContractPair: CtHandleContractPairStruct = {
        contractAddress,
        ctHandle: ebytes128CtHandle,
      };

      // Create a list of 3 ebytes128 ctHandles (each has a bit size of 1024 bits)
      const largeByteSizeCtHandleContractPairs = [
        ebytes128CtHandleContractPair,
        ebytes128CtHandleContractPair,
        ebytes128CtHandleContractPair,
      ];

      // Calculate the new total bit size of this list
      const totalBitSize = 3072;

      // Check that the request fails because the total bit size exceeds the maximum allowed
      // Note that the user signature is not correct here but the FHE type validity is checked first
      await expect(
        decryption.userDecryptionRequest(
          largeByteSizeCtHandleContractPairs,
          requestValidity,
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
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
      const contractAddressesWithUserAddress = [user.address];

      await expect(
        decryption.userDecryptionRequest(
          userAddressCtHandleContractPairs,
          requestValidity,
          hostChainId,
          contractAddressesWithUserAddress,
          user.address,
          publicKey,
          userSignature,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "UserAddressInContractAddresses")
        .withArgs(user.address, contractAddressesWithUserAddress);
    });

    it("Should revert because the user is not allowed for user decryption on a ctHandle", async function () {
      await expect(
        decryption.userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          hostChainId,
          contractAddresses,
          fakeUserAddress,
          publicKey,
          userSignature,
        ),
      )
        .to.be.revertedWithCustomError(multichainAcl, "AccountNotAllowedToUseCiphertext")
        .withArgs(ctHandleContractPairs[0].ctHandle, fakeUserAddress);
    });

    it("Should revert because a contract is not allowed for user decryption on a ctHandle", async function () {
      await expect(
        decryption.userDecryptionRequest(
          fakeContractAddressCtHandleContractPairs,
          requestValidity,
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        ),
      )
        .to.be.revertedWithCustomError(multichainAcl, "AccountNotAllowedToUseCiphertext")
        .withArgs(fakeContractAddressCtHandleContractPairs[0].ctHandle, fakeContractAddress);
    });

    it("Should revert because ciphertext material has not been added", async function () {
      // Allow access to the handle for the user and contract accounts
      // We need to do this because `userDecryptionRequest` first checks if the accounts have access
      // to the handle
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, user.address);
        await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, contractAddress);
      }

      await expect(
        decryption.userDecryptionRequest(
          [newCtHandleContractPair],
          requestValidity,
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
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
        hostChainId,
        contractAddresses,
        user.address,
        publicKey,
        fakeSignature,
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
        hostChainId,
        contractAddresses,
        user.address,
        publicKey,
        userSignature,
      );

      // Create a fake signature from the fake signer
      const [fakeSignature] = await getSignaturesUserDecryptResponse(eip712ResponseMessages.slice(0, 1), [fakeSigner]);

      // Check that the transaction fails because the signer is not a registered KMS signer
      await expect(
        decryption.connect(kmsTxSenders[0]).userDecryptionResponse(decryptionId, userDecryptedShares[0], fakeSignature),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "NotKmsSigner")
        .withArgs(fakeSigner.address);
    });

    it("Should revert because the message sender is not a KMS transaction sender", async function () {
      // Check that the transaction fails because the msg.sender is not a registered KMS transaction sender
      await expect(
        decryption.connect(fakeTxSender).userDecryptionResponse(decryptionId, userDecryptedShares[0], kmsSignatures[0]),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "NotKmsTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should revert because contract in ctHandleContractPairs not included in contractAddresses list", async function () {
      // Create EIP712 message using the fake contract address list
      const decryptionAddress = await decryption.getAddress();
      const fakeEip712RequestMessage = createEIP712RequestUserDecrypt(
        decryptionAddress,
        publicKey,
        fakeContractAddresses,
        hostChainId,
        requestValidity.startTimestamp.toString(),
        requestValidity.durationDays.toString(),
      );

      // Sign the message with the user
      const [fakeUserSignature] = await getSignaturesUserDecryptRequest(fakeEip712RequestMessage, [user]);

      // Request user decryption
      const requestTx = decryption.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        hostChainId,
        fakeContractAddresses,
        user.address,
        publicKey,
        fakeUserSignature,
      );

      // Check that the request fails because the contract address is not included in the contractAddresses list
      await expect(requestTx)
        .to.be.revertedWithCustomError(decryption, "ContractNotInContractAddresses")
        .withArgs(contractAddress, fakeContractAddresses);
    });

    it("Should revert because of ctMaterials tied to different key IDs", async function () {
      const keyId2 = await createAndRotateKey(
        keyId1,
        kmsManagement,
        owner,
        coprocessorTxSenders,
        kmsTxSenders,
        fheParamsName,
      );

      // Store the handle and allow the user and contract accounts to use it
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await ciphertextCommits
          .connect(coprocessorTxSenders[i])
          .addCiphertextMaterial(newCtHandle, keyId2, ciphertextDigest, snsCiphertextDigest);
        await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, user.address);
        await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, contractAddress);
      }

      // Request user decryption with ctMaterials tied to different key IDs
      const requestTx = decryption.userDecryptionRequest(
        [...ctHandleContractPairs, newCtHandleContractPair],
        requestValidity,
        hostChainId,
        contractAddresses,
        user.address,
        publicKey,
        userSignature,
      );

      // Check that different key IDs are not allowed for batched user decryption
      await expect(requestTx)
        .to.revertedWithCustomError(decryption, "DifferentKeyIdsNotAllowed")
        .withArgs(
          toValues(snsCiphertextMaterials[0]),
          toValues({
            ctHandle: newCtHandle,
            keyId: keyId2,
            snsCiphertextDigest,
            coprocessorTxSenderAddresses: coprocessorTxSenders.map((s) => s.address),
            coprocessorContextId: coprocessorContextId,
          }),
        );
    });

    it("Should revert because of two responses with same signature", async function () {
      // Request user decryption
      await decryption.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        hostChainId,
        contractAddresses,
        user.address,
        publicKey,
        userSignature,
      );

      // Trigger a first user decryption response
      await decryption
        .connect(kmsTxSenders[0])
        .userDecryptionResponse(decryptionId, userDecryptedShares[0], kmsSignatures[0]);

      // Check that a KMS node cannot sign a second time for the same user decryption
      await expect(
        decryption
          .connect(kmsTxSenders[0])
          .userDecryptionResponse(decryptionId, userDecryptedShares[0], kmsSignatures[0]),
      )
        .to.be.revertedWithCustomError(decryption, "KmsNodeAlreadySigned")
        .withArgs(decryptionId, kmsSigners[0].address);
    });

    it("Should reach consensus with 3 valid responses", async function () {
      // Request user decryption
      await decryption.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        hostChainId,
        contractAddresses,
        user.address,
        publicKey,
        userSignature,
      );

      // Trigger three valid user decryption responses using different KMS transaction senders
      await decryption
        .connect(kmsTxSenders[0])
        .userDecryptionResponse(decryptionId, userDecryptedShares[0], kmsSignatures[0]);

      await decryption
        .connect(kmsTxSenders[1])
        .userDecryptionResponse(decryptionId, userDecryptedShares[1], kmsSignatures[1]);

      const responseTx3 = await decryption
        .connect(kmsTxSenders[2])
        .userDecryptionResponse(decryptionId, userDecryptedShares[2], kmsSignatures[2]);

      // Consensus should be reached at the third response (reconstruction threshold)
      // Check 3rd response event: it should only contain 3 valid signatures
      await expect(responseTx3)
        .to.emit(decryption, "UserDecryptionResponse")
        .withArgs(decryptionId, userDecryptedShares.slice(0, 3), kmsSignatures.slice(0, 3));

      // Check that the user decryption is done
      await expect(decryption.checkDecryptionDone(decryptionId)).to.not.be.reverted;
    });

    it("Should ignore other valid responses", async function () {
      // Request user decryption
      await decryption.userDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        hostChainId,
        contractAddresses,
        user.address,
        publicKey,
        userSignature,
      );

      // Trigger three valid user decryption responses using different KMS transaction senders
      const responseTx1 = await decryption
        .connect(kmsTxSenders[0])
        .userDecryptionResponse(decryptionId, userDecryptedShares[0], kmsSignatures[0]);

      const responseTx2 = await decryption
        .connect(kmsTxSenders[1])
        .userDecryptionResponse(decryptionId, userDecryptedShares[1], kmsSignatures[1]);

      await decryption
        .connect(kmsTxSenders[2])
        .userDecryptionResponse(decryptionId, userDecryptedShares[2], kmsSignatures[2]);

      const responseTx4 = await decryption
        .connect(kmsTxSenders[3])
        .userDecryptionResponse(decryptionId, userDecryptedShares[3], kmsSignatures[3]);

      // Check that the 1st, 2nd and 4th responses do not emit an event:
      // - 1st and 2nd responses are ignored because consensus is not reached yet
      // - 4th response is ignored (not reverted) even though they are late
      await expect(responseTx1).to.not.emit(decryption, "UserDecryptionResponse");
      await expect(responseTx2).to.not.emit(decryption, "UserDecryptionResponse");
      await expect(responseTx4).to.not.emit(decryption, "UserDecryptionResponse");
    });

    it("Should revert because the contract is paused", async function () {
      // Pause the contract
      await decryption.connect(owner).pause();

      // Try calling paused user decryption request
      await expect(
        decryption.userDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          hostChainId,
          contractAddresses,
          user.address,
          publicKey,
          userSignature,
        ),
      ).to.be.revertedWithCustomError(decryption, "EnforcedPause");

      // Try calling paused user decryption response
      await expect(
        decryption
          .connect(kmsTxSenders[0])
          .userDecryptionResponse(decryptionId, userDecryptedShares[0], kmsSignatures[0]),
      ).to.be.revertedWithCustomError(decryption, "EnforcedPause");
    });

    describe("Checks", function () {
      it("Should not revert because user decryption is ready", async function () {
        await expect(decryption.checkUserDecryptionReady(user.address, ctHandleContractPairs)).to.not.be.reverted;
      });

      it("Should revert because the user is not allowed for user decryption on a ctHandle", async function () {
        await expect(decryption.checkUserDecryptionReady(fakeUserAddress, ctHandleContractPairs))
          .to.be.revertedWithCustomError(multichainAcl, "AccountNotAllowedToUseCiphertext")
          .withArgs(ctHandleContractPairs[0].ctHandle, fakeUserAddress);
      });

      it("Should revert because a contract is not allowed for user decryption on a ctHandle", async function () {
        await expect(decryption.checkUserDecryptionReady(user.address, fakeContractAddressCtHandleContractPairs))
          .to.be.revertedWithCustomError(multichainAcl, "AccountNotAllowedToUseCiphertext")
          .withArgs(fakeContractAddressCtHandleContractPairs[0].ctHandle, fakeContractAddress);
      });

      it("Should revert because ciphertext material has not been added", async function () {
        // Allow access to the handle for the user and contract accounts
        // We need to do this because `checkUserDecryptionReady` first checks if the accounts
        // have access to the handle
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, user.address);
          await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, contractAddress);
        }

        await expect(decryption.checkUserDecryptionReady(user.address, [newCtHandleContractPair]))
          .to.be.revertedWithCustomError(ciphertextCommits, "CiphertextMaterialNotFound")
          .withArgs(newCtHandle);
      });

      it("Should revert because the user decryption is not done", async function () {
        await expect(decryption.checkDecryptionDone(decryptionId))
          .to.be.revertedWithCustomError(decryption, "DecryptionNotDone")
          .withArgs(decryptionId);
      });
    });
  });

  describe("Delegated User Decryption", function () {
    let delegatedSignature: string;
    let eip712RequestMessage: EIP712;
    let userDecryptedShares: string[];

    // Create valid input values
    // The delegated account needs a wallet in order to sign
    const delegatorAddress = createRandomAddress();
    const delegatedAccount = createRandomWallet();
    const delegatedAddress = delegatedAccount.address;
    const delegationAccounts: DelegationAccountsStruct = {
      delegatorAddress,
      delegatedAddress,
    };
    const contractAddress = createRandomAddress();
    const contractAddresses = [contractAddress];
    const publicKey = createBytes32();
    const startTimestamp = getDateInSeconds();
    const durationDays = 120;
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
      ctHandle: newCtHandles[0],
    };

    // Define fake values
    const fakeDelegatorAddress = createRandomAddress();
    const fakeContractAddresses = createRandomAddresses(3);
    const fakeContractAddress = fakeContractAddresses[0];
    const fakeContractAddressCtHandleContractPairs: CtHandleContractPairStruct[] = [
      {
        contractAddress: fakeContractAddress,
        ctHandle,
      },
    ];
    const fakeDelegatorDelegationAccounts: DelegationAccountsStruct = {
      delegatorAddress: fakeDelegatorAddress,
      delegatedAddress,
    };

    // Define utility values
    const tenDaysInSeconds = 10 * 24 * 60 * 60;

    // Allow handles for user decryption
    async function prepareDelegatedUserDecryptEIP712Fixture() {
      const fixtureData = await loadFixture(prepareAddCiphertextFixture);
      const { decryption, multichainAcl, kmsSigners, coprocessorTxSenders } = fixtureData;

      // Allow account
      for (const ctHandle of ctHandles) {
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, delegatorAddress);
          await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, contractAddress);
        }
      }

      // Delegate account
      for (const txSender of coprocessorTxSenders) {
        await multichainAcl.connect(txSender).delegateAccount(hostChainId, delegationAccounts, contractAddresses);
      }

      // Create EIP712 messages
      const decryptionAddress = await decryption.getAddress();
      const eip712RequestMessage = createEIP712RequestDelegatedUserDecrypt(
        decryptionAddress,
        publicKey,
        contractAddresses,
        delegatorAddress.toString(),
        hostChainId,
        requestValidity.startTimestamp.toString(),
        requestValidity.durationDays.toString(),
      );

      // Sign the message with the delegated account
      const [delegatedSignature] = await getSignaturesDelegatedUserDecryptRequest(eip712RequestMessage, [
        delegatedAccount,
      ]);

      const userDecryptedShares = createBytes32s(kmsSigners.length);

      const eip712ResponseMessages: EIP712[] = userDecryptedShares.map((userDecryptedShare) =>
        createEIP712ResponseUserDecrypt(
          gatewayChainId,
          decryptionAddress,
          publicKey,
          ctHandleContractPairs.map((pair) => pair.ctHandle.toString()),
          userDecryptedShare,
        ),
      );

      // Sign the message with all KMS signers
      const kmsSignatures = await getSignaturesUserDecryptResponse(eip712ResponseMessages, kmsSigners);

      return {
        ...fixtureData,
        eip712RequestMessage,
        userDecryptedShares,
        delegatedSignature,
        kmsSignatures,
        requestValidity,
      };
    }

    beforeEach(async function () {
      // Initialize globally used variables before each test
      const fixtureData = await loadFixture(prepareDelegatedUserDecryptEIP712Fixture);
      kmsManagement = fixtureData.kmsManagement;
      multichainAcl = fixtureData.multichainAcl;
      ciphertextCommits = fixtureData.ciphertextCommits;
      decryption = fixtureData.decryption;
      owner = fixtureData.owner;
      snsCiphertextMaterials = fixtureData.snsCiphertextMaterials;
      delegatedSignature = fixtureData.delegatedSignature;
      kmsSignatures = fixtureData.kmsSignatures;
      kmsTxSenders = fixtureData.kmsTxSenders;
      coprocessorTxSenders = fixtureData.coprocessorTxSenders;
      eip712RequestMessage = fixtureData.eip712RequestMessage;
      userDecryptedShares = fixtureData.userDecryptedShares;
      keyId1 = fixtureData.keyId1;
      fheParamsName = fixtureData.fheParamsName;
    });

    it("Should request a user decryption with multiple ctHandleContractPairs", async function () {
      // Request user decryption
      const requestTx = await decryption.delegatedUserDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        delegationAccounts,
        hostChainId,
        contractAddresses,
        publicKey,
        delegatedSignature,
      );

      // Check request event
      await expect(requestTx)
        .to.emit(decryption, "UserDecryptionRequest")
        .withArgs(decryptionId, toValues(snsCiphertextMaterials), delegationAccounts.delegatedAddress, publicKey);
    });

    it("Should request a user decryption with a single ctHandleContractPair", async function () {
      const singleCtHandleContractPairs = ctHandleContractPairs.slice(0, 1);
      const singleSnsCiphertextMaterials = snsCiphertextMaterials.slice(0, 1);

      // Request user decryption
      const requestTx = await decryption.delegatedUserDecryptionRequest(
        singleCtHandleContractPairs,
        requestValidity,
        delegationAccounts,
        hostChainId,
        contractAddresses,
        publicKey,
        delegatedSignature,
      );

      // Check request event
      await expect(requestTx)
        .to.emit(decryption, "UserDecryptionRequest")
        .withArgs(decryptionId, toValues(singleSnsCiphertextMaterials), delegationAccounts.delegatedAddress, publicKey);
    });

    it("Should revert because ctHandleContractPairs is empty", async function () {
      await expect(
        decryption.delegatedUserDecryptionRequest(
          [],
          requestValidity,
          delegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedSignature,
        ),
      ).to.be.revertedWithCustomError(decryption, "EmptyCtHandleContractPairs");
    });

    it("Should revert because contract addresses is empty", async function () {
      await expect(
        decryption.delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          delegationAccounts,
          hostChainId,
          [],
          publicKey,
          delegatedSignature,
        ),
      ).to.be.revertedWithCustomError(decryption, "EmptyContractAddresses");
    });

    it("Should revert because contract addresses exceeds maximum length allowed", async function () {
      // Create a list of contract addresses exceeding the maximum length allowed
      const largeContractAddresses = createRandomAddresses(15);

      await expect(
        decryption.delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          delegationAccounts,
          hostChainId,
          largeContractAddresses,
          publicKey,
          delegatedSignature,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "ContractAddressesMaxLengthExceeded")
        .withArgs(MAX_USER_DECRYPT_CONTRACT_ADDRESSES, largeContractAddresses.length);
    });

    it("Should revert because durationDays is null", async function () {
      // Create an invalid validity request with a durationDays that is 0
      const invalidRequestValidity: IDecryption.RequestValidityStruct = {
        startTimestamp,
        durationDays: 0,
      };

      await expect(
        decryption.delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          invalidRequestValidity,
          delegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedSignature,
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
        decryption.delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          invalidRequestValidity,
          delegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedSignature,
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
        decryption.delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          futureRequestValidity,
          delegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedSignature,
        ),
      ).to.be.revertedWithCustomError(decryption, "StartTimestampInFuture");
    });

    it("Should revert because the delegated user decryption request has expired", async function () {
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
        decryption.delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          expiredRequestValidity,
          delegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedSignature,
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
        decryption.delegatedUserDecryptionRequest(
          invalidFHETypeCtHandleContractPairs,
          requestValidity,
          delegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedSignature,
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
        decryption.delegatedUserDecryptionRequest(
          unsupportedFHETypeCtHandleContractPairs,
          requestValidity,
          delegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedSignature,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "UnsupportedFHEType")
        .withArgs(unsupportedFHEType);
    });

    it("Should revert because total bit size exceeds the maximum allowed", async function () {
      // Build a ctHandleContractPair containing the ebytes128 handle (which has a bit size of 1024 bits)
      const ebytes128CtHandleContractPair: CtHandleContractPairStruct = {
        contractAddress,
        ctHandle: ebytes128CtHandle,
      };

      // Create a list of 3 ebytes128 ctHandles (each has a bit size of 1024 bits)
      const largeByteSizeCtHandleContractPairs = [
        ebytes128CtHandleContractPair,
        ebytes128CtHandleContractPair,
        ebytes128CtHandleContractPair,
      ];

      // Calculate the new total bit size of this list
      const totalBitSize = 3072;

      // Check that the request fails because the total bit size exceeds the maximum allowed
      // Note that the user signature is not correct here but the FHE type validity is checked first
      await expect(
        decryption.delegatedUserDecryptionRequest(
          largeByteSizeCtHandleContractPairs,
          requestValidity,
          delegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedSignature,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "MaxDecryptionRequestBitSizeExceeded")
        .withArgs(MAX_DECRYPTION_REQUEST_BITS, totalBitSize);
    });

    it("Should revert because the delegator address is a contract address", async function () {
      // Define fake ctHandleContractPairs with delegator address as contract address
      const delegatorAddressCtHandleContractPairs: CtHandleContractPairStruct[] = [
        {
          contractAddress: delegatorAddress,
          ctHandle,
        },
      ];

      const contractAddressesWithDelegatorAddress = [delegatorAddress];

      // Check that the request fails because the delegated address is included in the ctHandleContractPairs list
      await expect(
        decryption.delegatedUserDecryptionRequest(
          delegatorAddressCtHandleContractPairs,
          requestValidity,
          delegationAccounts,
          hostChainId,
          contractAddressesWithDelegatorAddress,
          publicKey,
          delegatedSignature,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "DelegatorAddressInContractAddresses")
        .withArgs(delegatorAddress, contractAddressesWithDelegatorAddress);
    });

    it("Should revert because the delegator is not allowed to access a handle", async function () {
      await expect(
        decryption.delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          fakeDelegatorDelegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedSignature,
        ),
      )
        .to.be.revertedWithCustomError(multichainAcl, "AccountNotAllowedToUseCiphertext")
        .withArgs(ctHandles[0], fakeDelegatorAddress);
    });

    it("Should revert because a contract is not allowed for user decryption on a ctHandle", async function () {
      await expect(
        decryption.delegatedUserDecryptionRequest(
          fakeContractAddressCtHandleContractPairs,
          requestValidity,
          delegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedSignature,
        ),
      )
        .to.be.revertedWithCustomError(multichainAcl, "AccountNotAllowedToUseCiphertext")
        .withArgs(ctHandles[0], fakeContractAddress);
    });

    it("Should revert because ciphertext material has not been added", async function () {
      // Allow access to handles for the user and contract accounts
      // The associated ciphertext material has not yet been added to the CiphertextStorage state.
      for (const newCtHandle of newCtHandles) {
        for (const coprocessorTxSender of coprocessorTxSenders) {
          await multichainAcl
            .connect(coprocessorTxSender)
            .allowAccount(newCtHandle, delegationAccounts.delegatorAddress);
          await multichainAcl.connect(coprocessorTxSender).allowAccount(newCtHandle, contractAddress);
        }
      }

      // Check that the request fails because the ciphertext material is unavailable
      // Note: the function should be reverted on the unavailable ctHandle since it loops over the handles
      await expect(
        decryption.delegatedUserDecryptionRequest(
          [newCtHandleContractPair],
          requestValidity,
          delegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedSignature,
        ),
      )
        .to.be.revertedWithCustomError(ciphertextCommits, "CiphertextMaterialNotFound")
        .withArgs(newCtHandles[0]);
    });

    it("Should revert because the delegated address has not been delegated for a contract", async function () {
      await expect(
        decryption.delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          delegationAccounts,
          hostChainId,
          [...contractAddresses, fakeContractAddress],
          publicKey,
          delegatedSignature,
        ),
      )
        .to.be.revertedWithCustomError(multichainAcl, "AccountNotDelegated")
        .withArgs(hostChainId, toValues(delegationAccounts), fakeContractAddress);
    });

    it("Should revert because of invalid EIP712 user request signature", async function () {
      // Sign the message with a fake signer
      const [fakeSignature] = await getSignaturesDelegatedUserDecryptRequest(eip712RequestMessage, [fakeSigner]);

      // Request user decryption
      const requestTx = decryption.delegatedUserDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        delegationAccounts,
        hostChainId,
        contractAddresses,
        publicKey,
        fakeSignature,
      );

      // Check that the request has been reverted because of an invalid EIP712 user request signature
      await expect(requestTx).to.be.revertedWithCustomError(decryption, "InvalidUserSignature").withArgs(fakeSignature);
    });

    it("Should revert because contract in ctHandleContractPairs is not included in contractAddresses list", async function () {
      // Check that the request fails because the contract address is not included in the contractAddresses list
      await expect(
        decryption.delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          delegationAccounts,
          hostChainId,
          fakeContractAddresses,
          publicKey,
          delegatedSignature,
        ),
      )
        .to.be.revertedWithCustomError(decryption, "ContractNotInContractAddresses")
        .withArgs(contractAddress, fakeContractAddresses);
    });

    it("Should revert because of ctMaterials tied to different key IDs", async function () {
      // Define a new key ID
      const keyId2 = await createAndRotateKey(
        keyId1,
        kmsManagement,
        owner,
        coprocessorTxSenders,
        kmsTxSenders,
        fheParamsName,
      );

      // Store the handle and allow the user and contract accounts to use it
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await ciphertextCommits
          .connect(coprocessorTxSenders[i])
          .addCiphertextMaterial(newCtHandle, keyId2, ciphertextDigest, snsCiphertextDigest);
        await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, delegatorAddress);
        await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, contractAddress);
      }

      // Request user decryption with ctMaterials tied to different key IDs
      const requestTx = decryption.delegatedUserDecryptionRequest(
        [...ctHandleContractPairs, newCtHandleContractPair],
        requestValidity,
        delegationAccounts,
        hostChainId,
        contractAddresses,
        publicKey,
        delegatedSignature,
      );

      // Check that different key IDs are not allowed for batched user decryption
      await expect(requestTx)
        .to.be.revertedWithCustomError(decryption, "DifferentKeyIdsNotAllowed")
        .withArgs(
          toValues(snsCiphertextMaterials[0]),
          toValues({
            ctHandle: newCtHandle,
            keyId: keyId2,
            snsCiphertextDigest,
            coprocessorTxSenderAddresses: coprocessorTxSenders.map((s) => s.address),
            coprocessorContextId: coprocessorContextId,
          }),
        );
    });

    it("Should reach consensus with 3 valid responses", async function () {
      // Request user decryption
      await decryption.delegatedUserDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        delegationAccounts,
        hostChainId,
        contractAddresses,
        publicKey,
        delegatedSignature,
      );

      // Trigger three valid user decryption responses using different KMS transaction senders
      await decryption
        .connect(kmsTxSenders[0])
        .userDecryptionResponse(decryptionId, userDecryptedShares[0], kmsSignatures[0]);

      await decryption
        .connect(kmsTxSenders[1])
        .userDecryptionResponse(decryptionId, userDecryptedShares[1], kmsSignatures[1]);

      const responseTx3 = await decryption
        .connect(kmsTxSenders[2])
        .userDecryptionResponse(decryptionId, userDecryptedShares[2], kmsSignatures[2]);

      // Consensus should be reached at the third response (reconstruction threshold)
      // Check 3rd response event: it should only contain 3 valid signatures
      await expect(responseTx3)
        .to.emit(decryption, "UserDecryptionResponse")
        .withArgs(decryptionId, userDecryptedShares.slice(0, 3), kmsSignatures.slice(0, 3));

      // Check that the user decryption is done
      await expect(decryption.checkDecryptionDone(decryptionId)).to.not.be.reverted;
    });

    it("Should ignore other valid responses", async function () {
      // Request user decryption
      await decryption.delegatedUserDecryptionRequest(
        ctHandleContractPairs,
        requestValidity,
        delegationAccounts,
        hostChainId,
        contractAddresses,
        publicKey,
        delegatedSignature,
      );

      // Trigger three valid user decryption responses using different KMS transaction senders
      const responseTx1 = await decryption
        .connect(kmsTxSenders[0])
        .userDecryptionResponse(decryptionId, userDecryptedShares[0], kmsSignatures[0]);

      const responseTx2 = await decryption
        .connect(kmsTxSenders[1])
        .userDecryptionResponse(decryptionId, userDecryptedShares[1], kmsSignatures[1]);

      await decryption
        .connect(kmsTxSenders[2])
        .userDecryptionResponse(decryptionId, userDecryptedShares[2], kmsSignatures[2]);

      const responseTx4 = await decryption
        .connect(kmsTxSenders[3])
        .userDecryptionResponse(decryptionId, userDecryptedShares[3], kmsSignatures[3]);

      // Check that the 1st, 2nd and 4th responses do not emit an event:
      // - 1st and 2nd responses are ignored because consensus is not reached yet
      // - 4th response is ignored (not reverted) even though they are late
      await expect(responseTx1).to.not.emit(decryption, "UserDecryptionResponse");
      await expect(responseTx2).to.not.emit(decryption, "UserDecryptionResponse");
      await expect(responseTx4).to.not.emit(decryption, "UserDecryptionResponse");
    });

    it("Should revert because the contract is paused", async function () {
      // Pause the contract
      await decryption.connect(owner).pause();

      // Try calling paused delegated user decryption request
      await expect(
        decryption.delegatedUserDecryptionRequest(
          ctHandleContractPairs,
          requestValidity,
          delegationAccounts,
          hostChainId,
          contractAddresses,
          publicKey,
          delegatedSignature,
        ),
      ).to.be.revertedWithCustomError(decryption, "EnforcedPause");
    });

    describe("Checks", function () {
      it("Should not revert because delegated user decryption is ready", async function () {
        await expect(
          decryption.checkDelegatedUserDecryptionReady(
            hostChainId,
            delegationAccounts,
            ctHandleContractPairs,
            contractAddresses,
          ),
        ).to.not.be.reverted;
      });

      it("Should revert because the delegator is not allowed for user decryption on a ctHandle", async function () {
        // Delegate the fake delegation accounts for the contract addresses
        for (const txSender of coprocessorTxSenders) {
          await multichainAcl
            .connect(txSender)
            .delegateAccount(hostChainId, fakeDelegatorDelegationAccounts, contractAddresses);
        }

        await expect(
          decryption.checkDelegatedUserDecryptionReady(
            hostChainId,
            fakeDelegatorDelegationAccounts,
            ctHandleContractPairs,
            contractAddresses,
          ),
        )
          .to.be.revertedWithCustomError(multichainAcl, "AccountNotAllowedToUseCiphertext")
          .withArgs(ctHandles[0], fakeDelegatorAddress);
      });

      it("Should revert because a contract is not allowed for user decryption on a ctHandle", async function () {
        await expect(
          decryption.checkDelegatedUserDecryptionReady(
            hostChainId,
            delegationAccounts,
            fakeContractAddressCtHandleContractPairs,
            contractAddresses,
          ),
        )
          .to.be.revertedWithCustomError(multichainAcl, "AccountNotAllowedToUseCiphertext")
          .withArgs(fakeContractAddressCtHandleContractPairs[0].ctHandle, fakeContractAddress);
      });

      it("Should revert because the delegated address has not been delegated for a contract", async function () {
        await expect(
          decryption.checkDelegatedUserDecryptionReady(hostChainId, delegationAccounts, ctHandleContractPairs, [
            fakeContractAddress,
          ]),
        )
          .to.be.revertedWithCustomError(multichainAcl, "AccountNotDelegated")
          .withArgs(hostChainId, toValues(delegationAccounts), fakeContractAddress);
      });

      it("Should revert because ciphertext material has not been added", async function () {
        // Allow access to the handle for the user and contract accounts
        // We need to do this because `checkDelegatedUserDecryptionReady` first checks if the accounts
        // have access to the handle
        for (let i = 0; i < coprocessorTxSenders.length; i++) {
          await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, delegatorAddress);
          await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(newCtHandle, contractAddress);
        }

        await expect(
          decryption.checkDelegatedUserDecryptionReady(
            hostChainId,
            delegationAccounts,
            [newCtHandleContractPair],
            contractAddresses,
          ),
        )
          .to.be.revertedWithCustomError(ciphertextCommits, "CiphertextMaterialNotFound")
          .withArgs(newCtHandle);
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

    it("Should pause and unpause contract with owner address", async function () {
      // Check that the contract is not paused
      expect(await decryption.paused()).to.be.false;

      // Pause the contract with the owner address
      await expect(decryption.connect(owner).pause()).to.emit(decryption, "Paused").withArgs(owner);
      expect(await decryption.paused()).to.be.true;

      // Unpause the contract with the owner address
      await expect(decryption.connect(owner).unpause()).to.emit(decryption, "Unpaused").withArgs(owner);
      expect(await decryption.paused()).to.be.false;
    });

    it("Should pause contract with pauser address", async function () {
      // Check that the contract is not paused
      expect(await decryption.paused()).to.be.false;

      // Pause the contract with the pauser address
      await expect(decryption.connect(pauser).pause()).to.emit(decryption, "Paused").withArgs(pauser);
      expect(await decryption.paused()).to.be.true;
    });

    it("Should revert on pause because sender is not owner or pauser address", async function () {
      const notOwnerOrPauser = createRandomWallet();
      await expect(decryption.connect(notOwnerOrPauser).pause())
        .to.be.revertedWithCustomError(decryption, "NotOwnerOrPauser")
        .withArgs(notOwnerOrPauser.address);
    });
  });
});
