import { SignerWithAddress } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { EventLog, Wallet } from "ethers";
import hre from "hardhat";

import { KmsManagement, KmsManagement__factory } from "../typechain-types";
import { createRandomWallet, loadTestVariablesFixture } from "./utils";

describe("KmsManagement", function () {
  const fakeFheParamsName = "FAKE_FHE_PARAMS_NAME";

  const fakeOwner = createRandomWallet();

  // Run a preprocessing keygen
  async function prepareKmsManagementPreKeygenFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const { kmsManagement, owner, kmsTxSenders, fheParamsName } = fixtureData;

    // Trigger a preprocessing keygen request
    const txRequest = await kmsManagement.connect(owner).preprocessKeygenRequest(fheParamsName);

    const receipt = await txRequest.wait();

    // Get the preKeyRequestId from the event in the transaction receipt (preKeyRequestId is the first argument of the event)
    const event = receipt?.logs[0] as EventLog;
    const preKeyRequestId = Number(event?.args[0]);

    // Define a preKeyId for the keygen response
    const preKeyId = 1;

    // Trigger preprocessing keygen responses for all KMS nodes
    // Note: not all responses are strictly needed (the consensus can be reached before the last response(s))
    for (let i = 0; i < kmsTxSenders.length; i++) {
      await kmsManagement.connect(kmsTxSenders[i]).preprocessKeygenResponse(preKeyRequestId, preKeyId);
    }

    return { ...fixtureData, preKeyId };
  }

  // Run a keygen
  async function prepareKmsManagementKeygenFixture() {
    const fixtureData = await loadFixture(prepareKmsManagementPreKeygenFixture);
    const { kmsManagement, owner, kmsTxSenders, preKeyId } = fixtureData;

    // Trigger a keygen request
    await kmsManagement.connect(owner).keygenRequest(preKeyId);

    // Define 2 keyIds for keygen responses
    const keyId1 = 1;
    const keyId2 = 2;

    // Trigger keygen responses for all KMS nodes
    // Note: not all responses are strictly needed (the consensus can be reached before the last response(s))
    for (let i = 0; i < kmsTxSenders.length; i++) {
      await kmsManagement.connect(kmsTxSenders[i]).keygenResponse(preKeyId, keyId1);
      await kmsManagement.connect(kmsTxSenders[i]).keygenResponse(preKeyId, keyId2);
    }

    return { ...fixtureData, keyId1, keyId2 };
  }

  // Run a preprocessing KSK generation
  async function prepareKmsManagementPreKskgenFixture() {
    const fixtureData = await loadFixture(prepareKmsManagementKeygenFixture);
    const { kmsManagement, owner, kmsTxSenders, fheParamsName } = fixtureData;

    // Trigger a preprocessing KSK generation request
    const txRequest = await kmsManagement.connect(owner).preprocessKskgenRequest(fheParamsName);

    const receipt = await txRequest.wait();

    // Get the preKskRequestId from the event in the transaction receipt (preKskRequestId is the first argument of the event)
    const event = receipt?.logs[0] as EventLog;
    const preKskRequestId = Number(event?.args[0]);

    // Define a preKskId for the KSK generation response
    const preKskId = 1;

    // Trigger preprocessing KSK generation responses for all KMS nodes
    // Note: not all responses are strictly needed (the consensus can be reached before the last response(s))
    for (let i = 0; i < kmsTxSenders.length; i++) {
      await kmsManagement.connect(kmsTxSenders[i]).preprocessKskgenResponse(preKskRequestId, preKskId);
    }

    return { ...fixtureData, preKskId };
  }

  // Run a KSK generation and activate the first key
  async function prepareKmsManagementActivateFixture() {
    const fixtureData = await loadFixture(prepareKmsManagementPreKskgenFixture);
    const { kmsManagement, owner, kmsTxSenders, coprocessorTxSenders, preKskId, keyId1, keyId2 } = fixtureData;

    // Trigger a KSK generation request
    await kmsManagement.connect(owner).kskgenRequest(preKskId, keyId1, keyId2);

    // Define a kskId for KSK generation response
    const kskId = 1;

    // Trigger preprocessing KSK generation responses for all KMS nodes
    // Note: not all responses are strictly needed (the consensus can be reached before the last response(s))
    for (let i = 0; i < kmsTxSenders.length; i++) {
      await kmsManagement.connect(kmsTxSenders[i]).kskgenResponse(preKskId, kskId);
    }

    // Request activation of the first key
    await kmsManagement.connect(owner).activateKeyRequest(keyId1);

    // Trigger activation responses for all coprocessors
    for (let i = 0; i < coprocessorTxSenders.length; i++) {
      await kmsManagement.connect(coprocessorTxSenders[i]).activateKeyResponse(keyId1);
    }

    return { ...fixtureData, kskId };
  }

  describe("Deployment", function () {
    let kmsManagementFactory: KmsManagement__factory;
    let kmsManagement: KmsManagement;
    let owner: Wallet;
    let fheParamsName: string;
    let fheParamsDigest: string;

    beforeEach(async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      kmsManagement = fixtureData.kmsManagement;
      fheParamsName = fixtureData.fheParamsName;
      fheParamsDigest = fixtureData.fheParamsDigest;
      owner = fixtureData.owner;

      // Get the KmsManagement contract factory
      kmsManagementFactory = await hre.ethers.getContractFactory("KmsManagement", owner);
    });

    it("Should revert because initialization is not from an empty proxy", async function () {
      await expect(
        hre.upgrades.upgradeProxy(kmsManagement, kmsManagementFactory, {
          call: { fn: "initializeFromEmptyProxy", args: [fheParamsName, fheParamsDigest] },
        }),
      ).to.be.revertedWithCustomError(kmsManagement, "NotInitializingFromEmptyProxy");
    });
  });

  describe("Key generation", function () {
    it("Should revert if the FHE params are not initialized", async function () {
      const { kmsManagement, owner } = await loadFixture(loadTestVariablesFixture);

      // Check that a preprocessing keygen request cannot be triggered if the fheParams are not initialized
      await expect(
        kmsManagement.connect(owner).preprocessKeygenRequest(fakeFheParamsName),
      ).to.be.revertedWithCustomError(kmsManagement, "FheParamsNotInitialized");
    });

    it("Should revert because of access controls", async function () {
      const { gatewayConfig, kmsManagement, fheParamsName } = await loadFixture(loadTestVariablesFixture);

      // Check that someone else than the owner cannot trigger a preprocessing keygen request
      await expect(kmsManagement.connect(fakeOwner).preprocessKeygenRequest(fheParamsName))
        .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
        .withArgs(fakeOwner.address);

      // Check that someone else than a KMS transaction sender cannot trigger a preprocessing
      // keygen response
      await expect(kmsManagement.connect(fakeOwner).preprocessKeygenResponse(0, 0))
        .to.be.revertedWithCustomError(gatewayConfig, "NotKmsTxSender")
        .withArgs(fakeOwner.address);

      // Check that someone else than the owner cannot trigger a keygen request
      await expect(kmsManagement.connect(fakeOwner).keygenRequest(0))
        .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
        .withArgs(fakeOwner.address);

      // Check that someone else than the KMS transaction sender cannot trigger a keygen response
      await expect(kmsManagement.connect(fakeOwner).keygenResponse(0, 0))
        .to.be.revertedWithCustomError(gatewayConfig, "NotKmsTxSender")
        .withArgs(fakeOwner.address);
    });

    it("Should handle a preprocessed keygen", async function () {
      const { kmsManagement, owner, kmsTxSenders, fheParamsName, fheParamsDigest } =
        await loadFixture(loadTestVariablesFixture);

      // Define the expected preprocessing key request ID
      const expectedPreKeyRequestId = 1;

      // Trigger a preprocessing keygen request
      const txRequest = await kmsManagement.connect(owner).preprocessKeygenRequest(fheParamsName);

      // Check event
      await expect(txRequest)
        .to.emit(kmsManagement, "PreprocessKeygenRequest")
        .withArgs(expectedPreKeyRequestId, fheParamsDigest);

      // Define a preKeyId for the keygen response
      const preKeyId = 1;

      // Trigger a preprocessing keygen response with the first KMS node
      const txResponse1 = await kmsManagement
        .connect(kmsTxSenders[0])
        .preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(kmsManagement, "PreprocessKeygenResponse");

      // Check that a KMS node cannot respond twice to the same preprocessing keygen request
      await expect(kmsManagement.connect(kmsTxSenders[0]).preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId))
        .to.be.revertedWithCustomError(kmsManagement, "PreprocessKeygenKmsNodeAlreadyResponded")
        .withArgs(preKeyId, kmsTxSenders[0]);

      // Trigger a second preprocessing keygen response with the first KMS node
      await kmsManagement.connect(kmsTxSenders[1]).preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId);

      // Trigger a third preprocessing keygen response with the second KMS node, which should reach
      // consensus (4 / 2 + 1 = 3) and thus emit an event
      const txResponse3 = await kmsManagement
        .connect(kmsTxSenders[2])
        .preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId);

      // Check event
      await expect(txResponse3)
        .to.emit(kmsManagement, "PreprocessKeygenResponse")
        .withArgs(expectedPreKeyRequestId, preKeyId);

      // The 4th response should be ignored (not reverted) and not emit an event
      const txResponse4 = await kmsManagement
        .connect(kmsTxSenders[3])
        .preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId);

      // Check that the 4th response does not emit an event
      await expect(txResponse4).to.not.emit(kmsManagement, "PreprocessKeygenResponse");

      // Check that triggering a new preprocessing keygen request again gives a different preKeyRequestId
      // (the counter is incremented by 1)
      const txRequest2 = await kmsManagement.connect(owner).preprocessKeygenRequest(fheParamsName);
      await expect(txRequest2)
        .to.emit(kmsManagement, "PreprocessKeygenRequest")
        .withArgs(expectedPreKeyRequestId + 1, fheParamsDigest);
    });

    it("Should handle a keygen", async function () {
      const { kmsManagement, owner, kmsTxSenders, preKeyId, fheParamsDigest } = await loadFixture(
        prepareKmsManagementPreKeygenFixture,
      );

      // Check that a keygen request cannot be triggered if the preprocessing keygen is not done,
      // using a preKeyId different than the one given by the preprocessing keygen
      const fakePreKeyId = preKeyId + 1;
      await expect(kmsManagement.connect(owner).keygenRequest(fakePreKeyId))
        .to.be.revertedWithCustomError(kmsManagement, "KeygenPreprocessingRequired")
        .withArgs(fakePreKeyId);

      // Trigger a keygen request
      const txRequest = await kmsManagement.connect(owner).keygenRequest(preKeyId);

      // Check event
      await expect(txRequest).to.emit(kmsManagement, "KeygenRequest").withArgs(preKeyId, fheParamsDigest);

      // Check that a keygen request cannot be triggered again with the same preKeyId
      await expect(kmsManagement.connect(owner).keygenRequest(preKeyId))
        .to.be.revertedWithCustomError(kmsManagement, "KeygenRequestAlreadySent")
        .withArgs(preKeyId);

      // Define a keyId for keygen responses
      const keyId = 1;

      // Trigger a keygen response
      const txResponse1 = await kmsManagement.connect(kmsTxSenders[0]).keygenResponse(preKeyId, keyId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(kmsManagement, "KeygenResponse");

      // Check that a KMS node cannot respond twice to the same keygen request
      await expect(kmsManagement.connect(kmsTxSenders[0]).keygenResponse(preKeyId, keyId))
        .to.be.revertedWithCustomError(kmsManagement, "KeygenKmsNodeAlreadyResponded")
        .withArgs(keyId, kmsTxSenders[0]);

      // Trigger a second keygen response
      await kmsManagement.connect(kmsTxSenders[1]).keygenResponse(preKeyId, keyId);

      // Trigger a third keygen response with the second KMS node, which should reach
      // consensus (4 / 2 + 1 = 3) and thus emit an event
      const txResponse3 = await kmsManagement.connect(kmsTxSenders[2]).keygenResponse(preKeyId, keyId);

      // Check event
      await expect(txResponse3).to.emit(kmsManagement, "KeygenResponse").withArgs(preKeyId, keyId, fheParamsDigest);

      // The 4th response should be ignored (not reverted) and not emit an event
      const txResponse4 = await kmsManagement.connect(kmsTxSenders[3]).keygenResponse(preKeyId, keyId);

      // Check that the 4th response does not emit an event
      await expect(txResponse4).to.not.emit(kmsManagement, "KeygenResponse");
    });

    it("Should revert because the contract is paused", async function () {
      const { kmsManagement, owner, kmsTxSenders, preKeyId, fheParamsName } = await loadFixture(
        prepareKmsManagementPreKeygenFixture,
      );

      // Pause the contract
      await kmsManagement.connect(owner).pause();

      // Try calling paused preprocessing keygen request
      await expect(kmsManagement.connect(owner).preprocessKeygenRequest(fheParamsName)).to.be.revertedWithCustomError(
        kmsManagement,
        "EnforcedPause",
      );

      // Try calling paused preprocessing keygen response
      await expect(
        kmsManagement.connect(kmsTxSenders[0]).preprocessKeygenResponse(0, preKeyId),
      ).to.be.revertedWithCustomError(kmsManagement, "EnforcedPause");

      // Try calling paused keygen request
      await expect(kmsManagement.connect(owner).keygenRequest(preKeyId)).to.be.revertedWithCustomError(
        kmsManagement,
        "EnforcedPause",
      );

      // Try calling paused keygen response
      await expect(kmsManagement.connect(kmsTxSenders[0]).keygenResponse(preKeyId, 1)).to.be.revertedWithCustomError(
        kmsManagement,
        "EnforcedPause",
      );
    });
  });

  describe("CRS generation", async function () {
    it("Should revert if the FHE params are not initialized", async function () {
      const { kmsManagement, owner } = await loadFixture(loadTestVariablesFixture);

      // Check that a CRS generation request cannot be triggered if the fheParams are not initialized
      await expect(kmsManagement.connect(owner).crsgenRequest(fakeFheParamsName)).to.be.revertedWithCustomError(
        kmsManagement,
        "FheParamsNotInitialized",
      );
    });

    it("Should revert because of access controls", async function () {
      const { gatewayConfig, kmsManagement, fheParamsName } = await loadFixture(loadTestVariablesFixture);

      // Check that someone else than the owner cannot trigger a CRS generation request
      await expect(kmsManagement.connect(fakeOwner).crsgenRequest(fheParamsName))
        .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
        .withArgs(fakeOwner.address);

      // Check that someone else than the KMS transaction sender cannot trigger a CRS generation response
      await expect(kmsManagement.connect(fakeOwner).crsgenResponse(0, 0))
        .to.be.revertedWithCustomError(gatewayConfig, "NotKmsTxSender")
        .withArgs(fakeOwner.address);
    });

    it("Should handle a CRS generation", async function () {
      const { kmsManagement, owner, kmsTxSenders, fheParamsName, fheParamsDigest } =
        await loadFixture(loadTestVariablesFixture);

      // Define an expected preCrsId
      const expectedPreCrsId = 1;

      // Trigger a CRS generation request
      const txRequest = await kmsManagement.connect(owner).crsgenRequest(fheParamsName);

      // Check event
      await expect(txRequest).to.emit(kmsManagement, "CrsgenRequest").withArgs(expectedPreCrsId, fheParamsDigest);

      // Define a crsId for responses
      const crsId = 1;

      // Trigger a CRS generation response with the first KMS node
      const txResponse1 = await kmsManagement.connect(kmsTxSenders[0]).crsgenResponse(expectedPreCrsId, crsId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(kmsManagement, "CrsgenResponse");

      // Check that a KMS node cannot respond twice to the same CRS generation request
      await expect(kmsManagement.connect(kmsTxSenders[0]).crsgenResponse(expectedPreCrsId, crsId))
        .to.be.revertedWithCustomError(kmsManagement, "CrsgenKmsNodeAlreadyResponded")
        .withArgs(crsId, kmsTxSenders[0]);

      // Trigger a second CRS generation response with the first KMS node
      await kmsManagement.connect(kmsTxSenders[1]).crsgenResponse(expectedPreCrsId, crsId);

      // Trigger a third CRS generation response with the second KMS node, which should reach
      // consensus (4 / 2 + 1 = 3) and thus emit an event
      const txResponse3 = await kmsManagement.connect(kmsTxSenders[2]).crsgenResponse(expectedPreCrsId, crsId);

      // Check event
      await expect(txResponse3)
        .to.emit(kmsManagement, "CrsgenResponse")
        .withArgs(expectedPreCrsId, crsId, fheParamsDigest);

      // The 4th response should be ignored (not reverted) and not emit an event
      const txResponse4 = await kmsManagement.connect(kmsTxSenders[3]).crsgenResponse(expectedPreCrsId, crsId);

      // Check that the 4th response does not emit an event
      await expect(txResponse4).to.not.emit(kmsManagement, "CrsgenResponse");

      // Check that triggering a new preprocessing keygen request again gives a different preKeyId
      // (the counter is incremented by 1)
      const txRequest2 = await kmsManagement.connect(owner).crsgenRequest(fheParamsName);
      await expect(txRequest2)
        .to.emit(kmsManagement, "CrsgenRequest")
        .withArgs(expectedPreCrsId + 1, fheParamsDigest);
    });

    it("Should revert because the contract is paused", async function () {
      const { kmsManagement, owner, kmsTxSenders, fheParamsName } = await loadFixture(loadTestVariablesFixture);

      // Pause the contract
      await kmsManagement.connect(owner).pause();

      // Try calling paused crsgen request
      await expect(kmsManagement.connect(owner).crsgenRequest(fheParamsName)).to.be.revertedWithCustomError(
        kmsManagement,
        "EnforcedPause",
      );

      // Try calling paused crsgen response
      await expect(kmsManagement.connect(kmsTxSenders[0]).crsgenResponse(0, 0)).to.be.revertedWithCustomError(
        kmsManagement,
        "EnforcedPause",
      );
    });
  });

  describe("KSK generation", async function () {
    it("Should revert if the FHE params are not initialized", async function () {
      const { kmsManagement, owner } = await loadFixture(loadTestVariablesFixture);

      // Check that a preprocessing KSK generation request cannot be triggered if the fheParams are not initialized
      await expect(
        kmsManagement.connect(owner).preprocessKskgenRequest(fakeFheParamsName),
      ).to.be.revertedWithCustomError(kmsManagement, "FheParamsNotInitialized");
    });

    it("Should revert because of access controls", async function () {
      const { gatewayConfig, kmsManagement, fheParamsName } = await loadFixture(loadTestVariablesFixture);

      // Check that someone else than the owner cannot trigger a preprocessing KSK generation request
      await expect(kmsManagement.connect(fakeOwner).preprocessKskgenRequest(fheParamsName))
        .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
        .withArgs(fakeOwner.address);

      // Check that someone else than the KMS transaction sender cannot trigger a preprocessing KSK generation response
      await expect(kmsManagement.connect(fakeOwner).preprocessKskgenResponse(0, 0))
        .to.be.revertedWithCustomError(gatewayConfig, "NotKmsTxSender")
        .withArgs(fakeOwner.address);

      // Check that someone else than the owner cannot trigger a KSK generation request
      await expect(kmsManagement.connect(fakeOwner).kskgenRequest(0, 0, 0))
        .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
        .withArgs(fakeOwner.address);

      // Check that someone else than the KMS transaction sender cannot trigger a KSK generation response
      await expect(kmsManagement.connect(fakeOwner).kskgenResponse(0, 0))
        .to.be.revertedWithCustomError(gatewayConfig, "NotKmsTxSender")
        .withArgs(fakeOwner.address);
    });

    it("Should handle a preprocessed KSK generation", async function () {
      const { kmsManagement, owner, kmsTxSenders, fheParamsName, fheParamsDigest } =
        await loadFixture(loadTestVariablesFixture);

      // Define the expected preprocessing KSK ID
      const expectedPreKskRequestId = 1;

      // Trigger a preprocessing KSK generation request
      const txRequest = await kmsManagement.connect(owner).preprocessKskgenRequest(fheParamsName);

      // Check event
      await expect(txRequest)
        .to.emit(kmsManagement, "PreprocessKskgenRequest")
        .withArgs(expectedPreKskRequestId, fheParamsDigest);

      // Define a preKskRequestId for the preprocessing KSK generation request
      const preKskRequestId = 1;

      // Trigger a preprocessing KSK generation response with the first KMS node
      const txResponse1 = await kmsManagement
        .connect(kmsTxSenders[0])
        .preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(kmsManagement, "PreprocessKskgenResponse");

      // Check that a KMS node cannot respond twice to the same preprocessing KSK generation request
      await expect(
        kmsManagement.connect(kmsTxSenders[0]).preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId),
      )
        .to.be.revertedWithCustomError(kmsManagement, "PreprocessKskgenKmsNodeAlreadyResponded")
        .withArgs(preKskRequestId, kmsTxSenders[0]);

      // Trigger a second preprocessing KSK generation response with the first KMS node
      await kmsManagement.connect(kmsTxSenders[1]).preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId);

      // Trigger a third preprocessing KSK generation response with the second KMS node, which should reach
      // consensus (4 / 2 + 1 = 3) and thus emit an event
      const txResponse3 = await kmsManagement
        .connect(kmsTxSenders[2])
        .preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId);

      // Check event
      await expect(txResponse3)
        .to.emit(kmsManagement, "PreprocessKskgenResponse")
        .withArgs(expectedPreKskRequestId, preKskRequestId);

      // The 4th response should be ignored (not reverted) and not emit an event
      const txResponse4 = await kmsManagement
        .connect(kmsTxSenders[3])
        .preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId);

      // Check that the 4th response does not emit an event
      await expect(txResponse4).to.not.emit(kmsManagement, "PreprocessKskgenResponse");

      // Check that triggering a new preprocessing KSK generation request again gives a different preKskRequestId
      // (the counter is incremented by 1)
      const txRequest2 = await kmsManagement.connect(owner).preprocessKskgenRequest(fheParamsName);
      await expect(txRequest2)
        .to.emit(kmsManagement, "PreprocessKskgenRequest")
        .withArgs(expectedPreKskRequestId + 1, fheParamsDigest);
    });

    it("Should handle a KSK generation", async function () {
      const { kmsManagement, owner, kmsTxSenders, fheParamsDigest, keyId1, keyId2, preKskId } = await loadFixture(
        prepareKmsManagementPreKskgenFixture,
      );

      // Check that a KSK generation request cannot be triggered if the preprocessing KSK generation is not done,
      // using a preKskId different than the one given by the preprocessing KSK generation
      const fakePreKskId = preKskId + 1;
      await expect(kmsManagement.connect(owner).kskgenRequest(fakePreKskId, keyId1, keyId2))
        .to.be.revertedWithCustomError(kmsManagement, "KskgenPreprocessingRequired")
        .withArgs(fakePreKskId);

      // Check that the source key must be different from the destination key
      await expect(kmsManagement.connect(owner).kskgenRequest(preKskId, keyId1, keyId1))
        .to.be.revertedWithCustomError(kmsManagement, "KskgenSameSrcAndDestKeyIds")
        .withArgs(keyId1);

      // Check that the source key must be generated
      const fakeKeyId = keyId1 + keyId2;
      await expect(kmsManagement.connect(owner).kskgenRequest(preKskId, fakeKeyId, keyId2))
        .to.be.revertedWithCustomError(kmsManagement, "KskgenSourceKeyNotGenerated")
        .withArgs(fakeKeyId);

      // Check that the destination key must be generated
      await expect(kmsManagement.connect(owner).kskgenRequest(preKskId, keyId1, fakeKeyId))
        .to.be.revertedWithCustomError(kmsManagement, "KskgenDestKeyNotGenerated")
        .withArgs(fakeKeyId);

      // Trigger a KSK generation request
      const txRequest = await kmsManagement.connect(owner).kskgenRequest(preKskId, keyId1, keyId2);

      // Check event
      await expect(txRequest)
        .to.emit(kmsManagement, "KskgenRequest")
        .withArgs(preKskId, keyId1, keyId2, fheParamsDigest);

      // Check that a keygen request cannot be triggered again with the same preKeyId
      await expect(kmsManagement.connect(owner).kskgenRequest(preKskId, keyId1, keyId2))
        .to.be.revertedWithCustomError(kmsManagement, "KskgenRequestAlreadySent")
        .withArgs(preKskId);

      // Define a keyId for keygen responses
      const kskId = 1;

      // Trigger a KSK generation response
      const txResponse1 = await kmsManagement.connect(kmsTxSenders[0]).kskgenResponse(preKskId, kskId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(kmsManagement, "KskgenResponse");

      // Check that a KMS node cannot respond twice to the same KSK generation request
      await expect(kmsManagement.connect(kmsTxSenders[0]).kskgenResponse(preKskId, kskId))
        .to.be.revertedWithCustomError(kmsManagement, "KskgenKmsNodeAlreadyResponded")
        .withArgs(kskId, kmsTxSenders[0]);

      // Trigger a second KSK generation response with the first KMS node
      await kmsManagement.connect(kmsTxSenders[1]).kskgenResponse(preKskId, kskId);

      // Trigger a third KSK generation response with the second KMS node, which should reach
      // consensus (4 / 2 + 1 = 3) and thus emit an event
      const txResponse3 = await kmsManagement.connect(kmsTxSenders[2]).kskgenResponse(preKskId, kskId);

      // Check event
      await expect(txResponse3).to.emit(kmsManagement, "KskgenResponse").withArgs(preKskId, kskId, fheParamsDigest);

      // The 4th response should be ignored (not reverted) and not emit an event
      const txResponse4 = await kmsManagement.connect(kmsTxSenders[3]).kskgenResponse(preKskId, kskId);

      // Check that the 4th response does not emit an event
      await expect(txResponse4).to.not.emit(kmsManagement, "KskgenResponse");
    });

    it("Should revert because the contract is paused", async function () {
      const { kmsManagement, owner, kmsTxSenders, keyId1, keyId2, preKskId, fheParamsName } = await loadFixture(
        prepareKmsManagementPreKskgenFixture,
      );

      // Pause the contract
      await kmsManagement.connect(owner).pause();

      // Try calling paused preprocessing kskgen request
      await expect(kmsManagement.connect(owner).preprocessKskgenRequest(fheParamsName)).to.be.revertedWithCustomError(
        kmsManagement,
        "EnforcedPause",
      );

      // Try calling paused preprocessing kskgen response
      await expect(
        kmsManagement.connect(kmsTxSenders[0]).preprocessKskgenResponse(0, preKskId),
      ).to.be.revertedWithCustomError(kmsManagement, "EnforcedPause");

      // Try calling paused kskgen request
      await expect(kmsManagement.connect(owner).kskgenRequest(preKskId, keyId1, keyId2)).to.be.revertedWithCustomError(
        kmsManagement,
        "EnforcedPause",
      );

      // Try calling paused kskgen response
      await expect(kmsManagement.connect(kmsTxSenders[0]).kskgenResponse(preKskId, 1)).to.be.revertedWithCustomError(
        kmsManagement,
        "EnforcedPause",
      );
    });
  });

  describe("Key activation", async function () {
    it("Should revert because of access controls", async function () {
      const { gatewayConfig, kmsManagement } = await loadFixture(loadTestVariablesFixture);

      // Check that someone else than the owner cannot trigger a key activation request
      await expect(kmsManagement.connect(fakeOwner).activateKeyRequest(0))
        .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
        .withArgs(fakeOwner.address);
    });

    it("Should handle a first key activation (no KSK generation)", async function () {
      const { kmsManagement, owner, coprocessorTxSenders, keyId1, keyId2 } = await loadFixture(
        prepareKmsManagementKeygenFixture,
      );

      // Check that the key to activate must be generated
      const fakeKeyId = keyId1 + keyId2;
      await expect(kmsManagement.connect(owner).activateKeyRequest(fakeKeyId))
        .to.be.revertedWithCustomError(kmsManagement, "ActivateKeyRequiresKeygen")
        .withArgs(fakeKeyId);

      // Trigger a key activation request
      const txRequest1 = await kmsManagement.connect(owner).activateKeyRequest(keyId1);

      // Check event
      await expect(txRequest1).to.emit(kmsManagement, "ActivateKeyRequest").withArgs(keyId1);

      // Check that the key activation request cannot be triggered again with the same keyId
      await expect(kmsManagement.connect(owner).activateKeyRequest(keyId1))
        .to.be.revertedWithCustomError(kmsManagement, "ActivateKeyRequestAlreadySent")
        .withArgs(keyId1);

      // Trigger a key activation response
      const txResponse1 = await kmsManagement.connect(coprocessorTxSenders[0]).activateKeyResponse(keyId1);

      // Check that the first response emits an event
      // TODO: Currently, the consensus threshold is hardcoded to 0 until keygen is integrated in the contract
      // See https://github.com/zama-ai/fhevm/issues/33
      await expect(txResponse1).to.emit(kmsManagement, "ActivateKeyResponse").withArgs(keyId1);

      // Check that a coprocessor cannot respond twice to the same key activation request
      await expect(kmsManagement.connect(coprocessorTxSenders[0]).activateKeyResponse(keyId1))
        .to.be.revertedWithCustomError(kmsManagement, "ActivateKeyCoprocessorAlreadyResponded")
        .withArgs(keyId1, coprocessorTxSenders[0]);

      // Check that we cannot activate the 2nd key (which has been generated but for which a KSK key
      // has not been generated)
      await expect(kmsManagement.connect(owner).activateKeyRequest(keyId2))
        .to.be.revertedWithCustomError(kmsManagement, "ActivateKeyRequiresKskgen")
        .withArgs(keyId1, keyId2);

      // The 2nd and 3rd responses should be ignored (not reverted) and not emit an event
      const txResponse2 = await kmsManagement.connect(coprocessorTxSenders[1]).activateKeyResponse(keyId1);
      const txResponse3 = await kmsManagement.connect(coprocessorTxSenders[2]).activateKeyResponse(keyId1);

      // Check that the 2nd and 3rd responses do not emit an event
      await expect(txResponse2).to.not.emit(kmsManagement, "ActivateKeyResponse");
      await expect(txResponse3).to.not.emit(kmsManagement, "ActivateKeyResponse");
    });

    it("Should handle a second key activation (with KSK generation)", async function () {
      const { kmsManagement, owner, coprocessorTxSenders, keyId1, keyId2 } = await loadFixture(
        prepareKmsManagementActivateFixture,
      );

      // Activate the 2nd key
      await kmsManagement.connect(owner).activateKeyRequest(keyId2);
      await kmsManagement.connect(coprocessorTxSenders[0]).activateKeyResponse(keyId2);
      await kmsManagement.connect(coprocessorTxSenders[1]).activateKeyResponse(keyId2);
      await kmsManagement.connect(coprocessorTxSenders[2]).activateKeyResponse(keyId2);

      // Check that we can get both activated key ids
      expect(await kmsManagement.activatedKeyIds(0)).to.be.equal(keyId1);
      expect(await kmsManagement.activatedKeyIds(1)).to.be.equal(keyId2);
    });

    it("Should revert because the contract is paused", async function () {
      const { kmsManagement, owner, kmsTxSenders, coprocessorTxSenders, keyId1 } = await loadFixture(
        prepareKmsManagementActivateFixture,
      );

      // Pause the contract
      await kmsManagement.connect(owner).pause();

      // Try calling paused activate key request
      await expect(kmsManagement.connect(owner).activateKeyRequest(keyId1)).to.be.revertedWithCustomError(
        kmsManagement,
        "EnforcedPause",
      );

      // Try calling paused activate key response
      await expect(
        kmsManagement.connect(coprocessorTxSenders[0]).activateKeyResponse(keyId1),
      ).to.be.revertedWithCustomError(kmsManagement, "EnforcedPause");
    });
  });

  describe("FHE parameters", async function () {
    it("Should revert because of access controls", async function () {
      const { kmsManagement } = await loadFixture(loadTestVariablesFixture);

      // Get dummy FHE params
      const fheParamsName = "TEST";
      const fheParamsDigest = hre.ethers.randomBytes(32);

      // Check that only the owner can set the FHE params
      await expect(kmsManagement.connect(fakeOwner).addFheParams(fheParamsName, fheParamsDigest))
        .to.be.revertedWithCustomError(kmsManagement, "OwnableUnauthorizedAccount")
        .withArgs(fakeOwner.address);

      // Check that only the owner can update the FHE params
      await expect(kmsManagement.connect(fakeOwner).updateFheParams(fheParamsName, fheParamsDigest))
        .to.be.revertedWithCustomError(kmsManagement, "OwnableUnauthorizedAccount")
        .withArgs(fakeOwner.address);
    });

    it("Should add fheParams", async function () {
      const { kmsManagement, owner } = await loadFixture(loadTestVariablesFixture);

      // Get dummy FHE params
      const newFheParamsName = "DEFAULT";
      const newFheParamsDigest = hre.ethers.randomBytes(32);

      // Set the FHE params
      const txSetFheParams = await kmsManagement.connect(owner).addFheParams(newFheParamsName, newFheParamsDigest);

      // Check event
      await expect(txSetFheParams)
        .to.emit(kmsManagement, "AddFheParams")
        .withArgs(newFheParamsName, newFheParamsDigest);
    });

    it("Should revert when adding fheParams because they are initialized", async function () {
      const { kmsManagement, owner, fheParamsName } = await loadFixture(loadTestVariablesFixture);

      // Get dummy FHE params digest
      const newFheParamsDigest = hre.ethers.randomBytes(32);

      // Check that we can only set the FHE params once
      await expect(kmsManagement.connect(owner).addFheParams(fheParamsName, newFheParamsDigest))
        .to.be.revertedWithCustomError(kmsManagement, "FheParamsAlreadyInitialized")
        .withArgs(fheParamsName);
    });

    it("Should update fheParams", async function () {
      const { kmsManagement, owner, fheParamsName } = await loadFixture(loadTestVariablesFixture);

      // Get dummy FHE params
      const newFheParamsDigest = hre.ethers.randomBytes(32);

      // Update the FHE params
      const txUpdateFheParams = await kmsManagement.connect(owner).updateFheParams(fheParamsName, newFheParamsDigest);

      // Check event
      await expect(txUpdateFheParams)
        .to.emit(kmsManagement, "UpdateFheParams")
        .withArgs(fheParamsName, newFheParamsDigest);
    });

    it("Should revert when updating fheParams because they are not initialized", async function () {
      const { kmsManagement, owner } = await loadFixture(loadTestVariablesFixture);

      // Get dummy FHE params
      const newFheParamsDigest = hre.ethers.randomBytes(32);

      // Check that FHE params cannot be updated if they are not initialized
      await expect(
        kmsManagement.connect(owner).updateFheParams(fakeFheParamsName, newFheParamsDigest),
      ).to.be.revertedWithCustomError(kmsManagement, "FheParamsNotInitialized");
    });

    it("Should revert because the contract is paused", async function () {
      const { kmsManagement, owner, kmsTxSenders, fheParamsName, fheParamsDigest } =
        await loadFixture(loadTestVariablesFixture);

      // Pause the contract
      await kmsManagement.connect(owner).pause();

      // Try calling paused add FHE params
      await expect(
        kmsManagement.connect(owner).addFheParams(fheParamsName, fheParamsDigest),
      ).to.be.revertedWithCustomError(kmsManagement, "EnforcedPause");

      // Try calling paused update FHE params
      await expect(
        kmsManagement.connect(owner).updateFheParams(fheParamsName, fheParamsDigest),
      ).to.be.revertedWithCustomError(kmsManagement, "EnforcedPause");
    });
  });

  describe("Pause", async function () {
    let kmsManagement: KmsManagement;
    let owner: Wallet;
    let pauser: SignerWithAddress;

    beforeEach(async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      kmsManagement = fixtureData.kmsManagement;
      owner = fixtureData.owner;
      pauser = fixtureData.pauser;
    });

    it("Should pause and unpause contract with owner address", async function () {
      // Check that the contract is not paused
      expect(await kmsManagement.paused()).to.be.false;

      // Pause the contract with the owner address
      await expect(kmsManagement.connect(owner).pause()).to.emit(kmsManagement, "Paused").withArgs(owner);
      expect(await kmsManagement.paused()).to.be.true;

      // Unpause the contract with the owner address
      await expect(kmsManagement.connect(owner).unpause()).to.emit(kmsManagement, "Unpaused").withArgs(owner);
      expect(await kmsManagement.paused()).to.be.false;
    });

    it("Should pause contract with pauser address", async function () {
      // Check that the contract is not paused
      expect(await kmsManagement.paused()).to.be.false;

      // Pause the contract with the pauser address
      await expect(kmsManagement.connect(pauser).pause()).to.emit(kmsManagement, "Paused").withArgs(pauser);
      expect(await kmsManagement.paused()).to.be.true;
    });

    it("Should revert on pause because sender is not owner or pauser address", async function () {
      const notOwnerOrPauser = createRandomWallet();
      await expect(kmsManagement.connect(notOwnerOrPauser).pause())
        .to.be.revertedWithCustomError(kmsManagement, "NotOwnerOrPauser")
        .withArgs(notOwnerOrPauser.address);
    });
  });
});
