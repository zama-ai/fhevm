import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { EventLog } from "ethers";
import hre from "hardhat";

import { deployHTTPZFixture, deployKeyManagerFixture } from "./utils/deploys";

describe("KeyManager", function () {
  // Define dummy FHE params to use for testing generation methods
  function getFheParams() {
    // TODO: Use proper fheParams format when implemented
    return {
      dummy: "dummy",
    };
  }

  // Deploy the KeyManager contract without setting the FHE params
  async function deployKeyManagerNoParamsFixture() {
    const { httpz, owner, admins, user, kmsSigners, coprocessorSigners, signers } =
      await loadFixture(deployHTTPZFixture);

    const KeyManager = await hre.ethers.getContractFactory("KeyManager", owner);
    const keyManager = await KeyManager.deploy(httpz);

    return { keyManager, owner, admins, user, kmsSigners, coprocessorSigners, signers };
  }

  // Deploy the keyManager contract and run a preprocessing keygen
  async function deployKeyManagerPreKeygenFixture() {
    const { keyManager, owner, admins, user, kmsSigners, coprocessorSigners, signers, fheParams } =
      await loadFixture(deployKeyManagerFixture);

    // Trigger a preprocessing keygen request
    const txRequest = await keyManager.connect(admins[0]).preprocessKeygenRequest();

    const receipt = await txRequest.wait();

    // Get the preKeyRequestId from the event in the transaction receipt (preKeyRequestId is the first argument of the event)
    const event = receipt?.logs[0] as EventLog;
    const preKeyRequestId = Number(event?.args[0]);

    // Define a preKeyId for the keygen response
    const preKeyId = 1;

    // Trigger preprocessing keygen responses for all KMS nodes
    // Note: not all responses are strictly needed (the consensus can be reached before the last response(s))
    for (let i = 0; i < kmsSigners.length; i++) {
      await keyManager.connect(kmsSigners[i]).preprocessKeygenResponse(preKeyRequestId, preKeyId);
    }

    return { keyManager, owner, admins, user, kmsSigners, coprocessorSigners, signers, fheParams, preKeyId };
  }

  // Deploy the keyManager contract and run a keygen
  async function deployKeyManagerKeygenFixture() {
    const { keyManager, owner, admins, user, kmsSigners, coprocessorSigners, signers, fheParams, preKeyId } =
      await loadFixture(deployKeyManagerPreKeygenFixture);

    // Trigger a keygen request
    await keyManager.connect(admins[0]).keygenRequest(preKeyId);

    // Define 2 keyIds for keygen responses
    const keyId1 = 1;
    const keyId2 = 2;

    // Trigger keygen responses for all KMS nodes
    // Note: not all responses are strictly needed (the consensus can be reached before the last response(s))
    for (let i = 0; i < kmsSigners.length; i++) {
      await keyManager.connect(kmsSigners[i]).keygenResponse(preKeyId, keyId1);
      await keyManager.connect(kmsSigners[i]).keygenResponse(preKeyId, keyId2);
    }

    return { keyManager, owner, admins, user, kmsSigners, coprocessorSigners, signers, fheParams, keyId1, keyId2 };
  }

  // Deploy the keyManager contract and run a preprocessing KSK generation
  async function deployKeyManagerPreKskgenFixture() {
    const { keyManager, owner, admins, user, kmsSigners, coprocessorSigners, signers, fheParams, keyId1, keyId2 } =
      await loadFixture(deployKeyManagerKeygenFixture);

    // Trigger a preprocessing KSK generation request
    const txRequest = await keyManager.connect(admins[0]).preprocessKskgenRequest();

    const receipt = await txRequest.wait();

    // Get the preKskRequestId from the event in the transaction receipt (preKskRequestId is the first argument of the event)
    const event = receipt?.logs[0] as EventLog;
    const preKskRequestId = Number(event?.args[0]);

    // Define a preKskId for the KSK generation response
    const preKskId = 1;

    // Trigger preprocessing KSK generation responses for all KMS nodes
    // Note: not all responses are strictly needed (the consensus can be reached before the last response(s))
    for (let i = 0; i < kmsSigners.length; i++) {
      await keyManager.connect(kmsSigners[i]).preprocessKskgenResponse(preKskRequestId, preKskId);
    }

    return {
      keyManager,
      owner,
      admins,
      user,
      kmsSigners,
      coprocessorSigners,
      signers,
      fheParams,
      keyId1,
      keyId2,
      preKskId,
    };
  }

  // Deploy the keyManager contract, run a KSK generation and activate the first key
  async function deployKeyManagerActivateFixture() {
    const {
      keyManager,
      owner,
      admins,
      user,
      kmsSigners,
      coprocessorSigners,
      signers,
      fheParams,
      keyId1,
      keyId2,
      preKskId,
    } = await loadFixture(deployKeyManagerPreKskgenFixture);

    // Trigger a KSK generation request
    await keyManager.connect(admins[0]).kskgenRequest(preKskId, keyId1, keyId2);

    // Define a kskId for KSK generation response
    const kskId = 1;

    // Trigger preprocessing KSK generation responses for all KMS nodes
    // Note: not all responses are strictly needed (the consensus can be reached before the last response(s))
    for (let i = 0; i < kmsSigners.length; i++) {
      await keyManager.connect(kmsSigners[i]).kskgenResponse(preKskId, kskId);
    }

    // Request activation of the first key
    await keyManager.connect(admins[0]).activateKeyRequest(keyId1);

    // Trigger activation responses for all coprocessors
    for (let i = 0; i < coprocessorSigners.length; i++) {
      await keyManager.connect(coprocessorSigners[i]).activateKeyResponse(keyId1);
    }

    return {
      keyManager,
      owner,
      admins,
      user,
      kmsSigners,
      coprocessorSigners,
      signers,
      fheParams,
      keyId1,
      keyId2,
      kskId,
    };
  }

  describe("Key generation", function () {
    it("Should revert if the FHE params are not initialized", async function () {
      const { keyManager, admins } = await loadFixture(deployKeyManagerNoParamsFixture);

      // Check that a preprocessing keygen request cannot be triggered if the fheParams are not initialized
      await expect(keyManager.connect(admins[0]).preprocessKeygenRequest()).to.be.revertedWithCustomError(
        keyManager,
        "FheParamsNotInitialized",
      );
    });

    it("Should revert because of access controls", async function () {
      const { keyManager, user } = await loadFixture(deployKeyManagerFixture);

      // Check that someone else than the admin cannot trigger a preprocessing keygen request
      await expect(keyManager.connect(user).preprocessKeygenRequest())
        .to.be.revertedWithCustomError(keyManager, "InvalidAdminSender")
        .withArgs(user.address);

      // Check that someone else than the admin cannot trigger a preprocessing keygen response
      await expect(keyManager.connect(user).preprocessKeygenResponse(0, 0))
        .to.be.revertedWithCustomError(keyManager, "InvalidKmsNodeSender")
        .withArgs(user.address);

      // Check that someone else than the admin cannot trigger a keygen request
      await expect(keyManager.connect(user).keygenRequest(0))
        .to.be.revertedWithCustomError(keyManager, "InvalidAdminSender")
        .withArgs(user.address);

      // Check that someone else than the KMS node cannot trigger a keygen response
      await expect(keyManager.connect(user).keygenResponse(0, 0))
        .to.be.revertedWithCustomError(keyManager, "InvalidKmsNodeSender")
        .withArgs(user.address);
    });

    it("Should handle a preprocessed keygen", async function () {
      const { keyManager, admins, kmsSigners, fheParams } = await loadFixture(deployKeyManagerFixture);

      // Define the expected preprocessing key request ID
      const expectedPreKeyRequestId = 1;

      // Trigger a preprocessing keygen request
      const txRequest = await keyManager.connect(admins[0]).preprocessKeygenRequest();

      // Check event
      await expect(txRequest)
        .to.emit(keyManager, "PreprocessKeygenRequest")
        .withArgs(expectedPreKeyRequestId, [fheParams.dummy]);

      // Define a preKeyId for the keygen response
      const preKeyId = 1;

      // Trigger a preprocessing keygen response with the first KMS node
      const txResponse1 = await keyManager
        .connect(kmsSigners[0])
        .preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(keyManager, "PreprocessKeygenResponse");

      // Check that a KMS node cannot respond twice to the same preprocessing keygen request
      await expect(
        keyManager.connect(kmsSigners[0]).preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId),
      ).to.be.revertedWithCustomError(keyManager, "PreprocessKeygenKmsNodeAlreadyResponded");

      // Trigger a second preprocessing keygen response with the second KMS node, which should reach
      // consensus (4 / 3 + 1 = 2) and thus emit an event
      const txResponse2 = await keyManager
        .connect(kmsSigners[1])
        .preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId);

      // Check event
      await expect(txResponse2)
        .to.emit(keyManager, "PreprocessKeygenResponse")
        .withArgs(expectedPreKeyRequestId, preKeyId);

      // The 3rd and 4th responses should be ignored (not reverted) and not emit an event
      const txResponse3 = await keyManager
        .connect(kmsSigners[2])
        .preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId);
      const txResponse4 = await keyManager
        .connect(kmsSigners[3])
        .preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId);

      // Check that the 3rd and 4th responses do not emit an event
      await expect(txResponse3).to.not.emit(keyManager, "PreprocessKeygenResponse");
      await expect(txResponse4).to.not.emit(keyManager, "PreprocessKeygenResponse");

      // Check that triggering a new preprocessing keygen request again gives a different preKeyRequestId
      // (the counter is incremented by 1)
      const txRequest2 = await keyManager.connect(admins[0]).preprocessKeygenRequest();
      await expect(txRequest2)
        .to.emit(keyManager, "PreprocessKeygenRequest")
        .withArgs(expectedPreKeyRequestId + 1, [fheParams.dummy]);
    });

    it("Should handle a keygen", async function () {
      const { keyManager, admins, kmsSigners, preKeyId, fheParams } = await loadFixture(
        deployKeyManagerPreKeygenFixture,
      );

      // Check that a keygen request cannot be triggered if the preprocessing keygen is not done,
      // using a preKeyId different than the one given by the preprocessing keygen
      const fakePreKeyId = preKeyId + 1;
      await expect(keyManager.connect(admins[0]).keygenRequest(fakePreKeyId))
        .to.be.revertedWithCustomError(keyManager, "KeygenPreprocessingRequired")
        .withArgs(fakePreKeyId);

      // Trigger a keygen request
      const txRequest = await keyManager.connect(admins[0]).keygenRequest(preKeyId);

      // Check event
      await expect(txRequest).to.emit(keyManager, "KeygenRequest").withArgs(preKeyId, [fheParams.dummy]);

      // Check that a keygen request cannot be triggered again with the same preKeyId
      await expect(keyManager.connect(admins[0]).keygenRequest(preKeyId))
        .to.be.revertedWithCustomError(keyManager, "KeygenRequestAlreadySent")
        .withArgs(preKeyId);

      // Define a keyId for keygen responses
      const keyId = 1;

      // Trigger a keygen response
      const txResponse1 = await keyManager.connect(kmsSigners[0]).keygenResponse(preKeyId, keyId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(keyManager, "KeygenResponse");

      // Check that a KMS node cannot respond twice to the same keygen request
      await expect(keyManager.connect(kmsSigners[0]).keygenResponse(preKeyId, keyId)).to.be.revertedWithCustomError(
        keyManager,
        "KeygenKmsNodeAlreadyResponded",
      );

      // Trigger a second keygen response with the second KMS node, which should reach
      // consensus (4 / 3 + 1 = 2) and thus emit an event
      const txResponse2 = await keyManager.connect(kmsSigners[1]).keygenResponse(preKeyId, keyId);

      // Check event
      await expect(txResponse2).to.emit(keyManager, "KeygenResponse").withArgs(preKeyId, keyId, [fheParams.dummy]);

      // The 3rd and 4th responses should be ignored (not reverted) and not emit an event
      const txResponse3 = await keyManager.connect(kmsSigners[2]).keygenResponse(preKeyId, keyId);
      const txResponse4 = await keyManager.connect(kmsSigners[3]).keygenResponse(preKeyId, keyId);

      // Check that the 3rd and 4th responses do not emit an event
      await expect(txResponse3).to.not.emit(keyManager, "KeygenResponse");
      await expect(txResponse4).to.not.emit(keyManager, "KeygenResponse");
    });
  });

  describe("CRS generation", async function () {
    it("Should revert if the FHE params are not initialized", async function () {
      const { keyManager, admins } = await loadFixture(deployKeyManagerNoParamsFixture);

      // Check that a CRS generation request cannot be triggered if the fheParams are not initialized
      await expect(keyManager.connect(admins[0]).crsgenRequest()).to.be.revertedWithCustomError(
        keyManager,
        "FheParamsNotInitialized",
      );
    });

    it("Should revert because of access controls", async function () {
      const { keyManager, user } = await loadFixture(deployKeyManagerFixture);

      // Check that someone else than the admin cannot trigger a CRS generation request
      await expect(keyManager.connect(user).crsgenRequest())
        .to.be.revertedWithCustomError(keyManager, "InvalidAdminSender")
        .withArgs(user.address);

      // Check that someone else than the KMS node cannot trigger a CRS generation response
      await expect(keyManager.connect(user).crsgenResponse(0, 0))
        .to.be.revertedWithCustomError(keyManager, "InvalidKmsNodeSender")
        .withArgs(user.address);
    });

    it("Should handle a CRS generation", async function () {
      const { keyManager, admins, kmsSigners, fheParams } = await loadFixture(deployKeyManagerFixture);

      // Define an expected preCrsId
      const expectedPreCrsId = 1;

      // Trigger a CRS generation request
      const txRequest = await keyManager.connect(admins[0]).crsgenRequest();

      // Check event
      await expect(txRequest).to.emit(keyManager, "CrsgenRequest").withArgs(expectedPreCrsId, [fheParams.dummy]);

      // Define a crsId for responses
      const crsId = 1;

      // Trigger a CRS generation response with the first KMS node
      const txResponse1 = await keyManager.connect(kmsSigners[0]).crsgenResponse(expectedPreCrsId, crsId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(keyManager, "CrsgenResponse");

      // Check that a KMS node cannot respond twice to the same CRS generation request
      await expect(
        keyManager.connect(kmsSigners[0]).crsgenResponse(expectedPreCrsId, crsId),
      ).to.be.revertedWithCustomError(keyManager, "CrsgenKmsNodeAlreadyResponded");

      // Trigger a second CRS generation response with the second KMS node, which should reach
      // consensus (4 / 3 + 1 = 2) and thus emit an event
      const txResponse2 = await keyManager.connect(kmsSigners[1]).crsgenResponse(expectedPreCrsId, crsId);

      // Check event
      await expect(txResponse2)
        .to.emit(keyManager, "CrsgenResponse")
        .withArgs(expectedPreCrsId, crsId, [fheParams.dummy]);

      // The 3rd and 4th responses should be ignored (not reverted) and not emit an event
      const txResponse3 = await keyManager.connect(kmsSigners[2]).crsgenResponse(expectedPreCrsId, crsId);
      const txResponse4 = await keyManager.connect(kmsSigners[3]).crsgenResponse(expectedPreCrsId, crsId);

      // Check that the 3rd and 4th responses do not emit an event
      await expect(txResponse3).to.not.emit(keyManager, "CrsgenResponse");
      await expect(txResponse4).to.not.emit(keyManager, "CrsgenResponse");

      // Check that triggering a new preprocessing keygen request again gives a different preKeyId
      // (the counter is incremented by 1)
      const txRequest2 = await keyManager.connect(admins[0]).crsgenRequest();
      await expect(txRequest2)
        .to.emit(keyManager, "CrsgenRequest")
        .withArgs(expectedPreCrsId + 1, [fheParams.dummy]);
    });
  });

  describe("KSK generation", async function () {
    it("Should revert if the FHE params are not initialized", async function () {
      const { keyManager, admins } = await loadFixture(deployKeyManagerNoParamsFixture);

      // Check that a preprocessing KSK generation request cannot be triggered if the fheParams are not initialized
      await expect(keyManager.connect(admins[0]).preprocessKskgenRequest()).to.be.revertedWithCustomError(
        keyManager,
        "FheParamsNotInitialized",
      );
    });

    it("Should revert because of access controls", async function () {
      const { httpz, keyManager, user } = await loadFixture(deployKeyManagerFixture);

      // Check that someone else than the admin cannot trigger a preprocessing KSK generation request
      await expect(keyManager.connect(user).preprocessKskgenRequest())
        .to.be.revertedWithCustomError(keyManager, "InvalidAdminSender")
        .withArgs(user.address);

      // Check that someone else than the KMS node cannot trigger a preprocessing KSK generation response
      await expect(keyManager.connect(user).preprocessKskgenResponse(0, 0))
        .to.be.revertedWithCustomError(keyManager, "InvalidKmsNodeSender")
        .withArgs(user.address);

      // Check that someone else than the admin cannot trigger a KSK generation request
      await expect(keyManager.connect(user).kskgenRequest(0, 0, 0))
        .to.be.revertedWithCustomError(keyManager, "InvalidAdminSender")
        .withArgs(user.address);

      // Check that someone else than the KMS node cannot trigger a KSK generation response
      await expect(keyManager.connect(user).kskgenResponse(0, 0))
        .to.be.revertedWithCustomError(keyManager, "InvalidKmsNodeSender")
        .withArgs(user.address);
    });

    it("Should handle a preprocessed KSK generation", async function () {
      const { keyManager, admins, kmsSigners, fheParams } = await loadFixture(deployKeyManagerFixture);

      // Define the expected preprocessing KSK ID
      const expectedPreKskRequestId = 1;

      // Trigger a preprocessing KSK generation request
      const txRequest = await keyManager.connect(admins[0]).preprocessKskgenRequest();

      // Check event
      await expect(txRequest)
        .to.emit(keyManager, "PreprocessKskgenRequest")
        .withArgs(expectedPreKskRequestId, [fheParams.dummy]);

      // Define a preKskRequestId for the preprocessing KSK generation request
      const preKskRequestId = 1;

      // Trigger a preprocessing KSK generation response with the first KMS node
      const txResponse1 = await keyManager
        .connect(kmsSigners[0])
        .preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(keyManager, "PreprocessKskgenResponse");

      // Check that a KMS node cannot respond twice to the same preprocessing KSK generation request
      await expect(
        keyManager.connect(kmsSigners[0]).preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId),
      ).to.be.revertedWithCustomError(keyManager, "PreprocessKskgenKmsNodeAlreadyResponded");

      // Trigger a second preprocessing KSK generation response with the second KMS node, which should reach
      // consensus (4 / 3 + 1 = 2) and thus emit an event
      const txResponse2 = await keyManager
        .connect(kmsSigners[1])
        .preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId);

      // Check event
      await expect(txResponse2)
        .to.emit(keyManager, "PreprocessKskgenResponse")
        .withArgs(expectedPreKskRequestId, preKskRequestId);

      // The 3rd and 4th responses should be ignored (not reverted) and not emit an event
      const txResponse3 = await keyManager
        .connect(kmsSigners[2])
        .preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId);
      const txResponse4 = await keyManager
        .connect(kmsSigners[3])
        .preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId);

      // Check that the 3rd and 4th responses do not emit an event
      await expect(txResponse3).to.not.emit(keyManager, "PreprocessKskgenResponse");
      await expect(txResponse4).to.not.emit(keyManager, "PreprocessKskgenResponse");

      // Check that triggering a new preprocessing KSK generation request again gives a different preKskRequestId
      // (the counter is incremented by 1)
      const txRequest2 = await keyManager.connect(admins[0]).preprocessKskgenRequest();
      await expect(txRequest2)
        .to.emit(keyManager, "PreprocessKskgenRequest")
        .withArgs(expectedPreKskRequestId + 1, [fheParams.dummy]);
    });

    it("Should handle a KSK generation", async function () {
      const { keyManager, admins, kmsSigners, fheParams, keyId1, keyId2, preKskId } = await loadFixture(
        deployKeyManagerPreKskgenFixture,
      );

      // Check that a KSK generation request cannot be triggered if the preprocessing KSK generation is not done,
      // using a preKskId different than the one given by the preprocessing KSK generation
      const fakePreKskId = preKskId + 1;
      await expect(keyManager.connect(admins[0]).kskgenRequest(fakePreKskId, keyId1, keyId2))
        .to.be.revertedWithCustomError(keyManager, "KskgenPreprocessingRequired")
        .withArgs(fakePreKskId);

      // Check that the source key must be different from the destination key
      await expect(keyManager.connect(admins[0]).kskgenRequest(preKskId, keyId1, keyId1))
        .to.be.revertedWithCustomError(keyManager, "KskgenSameSrcAndDestKeyIds")
        .withArgs(keyId1);

      // Check that the source key must be generated
      const fakeKeyId = keyId1 + keyId2;
      await expect(keyManager.connect(admins[0]).kskgenRequest(preKskId, fakeKeyId, keyId2))
        .to.be.revertedWithCustomError(keyManager, "KskgenSourceKeyNotGenerated")
        .withArgs(fakeKeyId);

      // Check that the destination key must be generated
      await expect(keyManager.connect(admins[0]).kskgenRequest(preKskId, keyId1, fakeKeyId))
        .to.be.revertedWithCustomError(keyManager, "KskgenDestKeyNotGenerated")
        .withArgs(fakeKeyId);

      // Trigger a KSK generation request
      const txRequest = await keyManager.connect(admins[0]).kskgenRequest(preKskId, keyId1, keyId2);

      // Check event
      await expect(txRequest)
        .to.emit(keyManager, "KskgenRequest")
        .withArgs(preKskId, keyId1, keyId2, [fheParams.dummy]);

      // Check that a keygen request cannot be triggered again with the same preKeyId
      await expect(keyManager.connect(admins[0]).kskgenRequest(preKskId, keyId1, keyId2))
        .to.be.revertedWithCustomError(keyManager, "KskgenRequestAlreadySent")
        .withArgs(preKskId);

      // Define a keyId for keygen responses
      const kskId = 1;

      // Trigger a KSK generation response
      const txResponse1 = await keyManager.connect(kmsSigners[0]).kskgenResponse(preKskId, kskId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(keyManager, "KskgenResponse");

      // Check that a KMS node cannot respond twice to the same KSK generation request
      await expect(keyManager.connect(kmsSigners[0]).kskgenResponse(preKskId, kskId)).to.be.revertedWithCustomError(
        keyManager,
        "KskgenKmsNodeAlreadyResponded",
      );

      // Trigger a second KSK generation response with the second KMS node, which should reach
      // consensus (4 / 3 + 1 = 2) and thus emit an event
      const txResponse2 = await keyManager.connect(kmsSigners[1]).kskgenResponse(preKskId, kskId);

      // Check event
      await expect(txResponse2).to.emit(keyManager, "KskgenResponse").withArgs(preKskId, kskId, [fheParams.dummy]);

      // The 3rd and 4th responses should be ignored (not reverted) and not emit an event
      const txResponse3 = await keyManager.connect(kmsSigners[2]).kskgenResponse(preKskId, kskId);
      const txResponse4 = await keyManager.connect(kmsSigners[3]).kskgenResponse(preKskId, kskId);

      // Check that the 3rd and 4th responses do not emit an event
      await expect(txResponse3).to.not.emit(keyManager, "KskgenResponse");
      await expect(txResponse4).to.not.emit(keyManager, "KskgenResponse");
    });
  });

  describe("Key activation", async function () {
    it("Should revert because of access controls", async function () {
      const { keyManager, user } = await loadFixture(deployKeyManagerFixture);

      // Check that someone else than the admin cannot trigger a key activation request
      await expect(keyManager.connect(user).activateKeyRequest(0))
        .to.be.revertedWithCustomError(keyManager, "InvalidAdminSender")
        .withArgs(user.address);

      // Check that someone else than the coprocessor cannot trigger a key activation response
      await expect(keyManager.connect(user).activateKeyResponse(0))
        .to.be.revertedWithCustomError(keyManager, "InvalidCoprocessorSender")
        .withArgs(user.address);
    });

    it("Should handle a first key activation (no KSK generation)", async function () {
      const { keyManager, admins, coprocessorSigners, keyId1, keyId2 } =
        await loadFixture(deployKeyManagerKeygenFixture);

      // Check that the key to activate must be generated
      const fakeKeyId = keyId1 + keyId2;
      await expect(keyManager.connect(admins[0]).activateKeyRequest(fakeKeyId))
        .to.be.revertedWithCustomError(keyManager, "ActivateKeyRequiresKeygen")
        .withArgs(fakeKeyId);

      // Trigger a key activation request
      const txRequest1 = await keyManager.connect(admins[0]).activateKeyRequest(keyId1);

      // Check event
      await expect(txRequest1).to.emit(keyManager, "ActivateKeyRequest").withArgs(keyId1);

      // Check that the key activation request cannot be triggered again with the same keyId
      await expect(keyManager.connect(admins[0]).activateKeyRequest(keyId1))
        .to.be.revertedWithCustomError(keyManager, "ActivateKeyRequestAlreadySent")
        .withArgs(keyId1);

      // Trigger a key activation response
      const txResponse1 = await keyManager.connect(coprocessorSigners[0]).activateKeyResponse(keyId1);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(keyManager, "ActivateKeyResponse");

      // Check that a coprocessor cannot respond twice to the same key activation request
      await expect(keyManager.connect(coprocessorSigners[0]).activateKeyResponse(keyId1)).to.be.revertedWithCustomError(
        keyManager,
        "ActivateKeyKmsNodeAlreadyResponded",
      );

      // Trigger a 2nd key activation response with the 2nd coprocessor, which should reach consensus
      // (tests use a total of 3 coprocessors) and thus emit an event
      const txResponse2 = await keyManager.connect(coprocessorSigners[1]).activateKeyResponse(keyId1);

      // Check that the 2nd response emits an event
      await expect(txResponse2).to.emit(keyManager, "ActivateKeyResponse").withArgs(keyId1);

      // Check that the key is activated
      expect(await keyManager.isCurrentKeyId(keyId1)).to.be.true;

      // Check that we cannot activate the 2nd key (which has been generated but for which a KSK key
      // has not been generated)
      await expect(keyManager.connect(admins[0]).activateKeyRequest(keyId2))
        .to.be.revertedWithCustomError(keyManager, "ActivateKeyRequiresKskgen")
        .withArgs(keyId1, keyId2);

      // The 3rd response should be ignored (not reverted) and not emit an event
      const txResponse3 = await keyManager.connect(coprocessorSigners[2]).activateKeyResponse(keyId1);

      // Check that the 3rd response does not emit an event
      await expect(txResponse3).to.not.emit(keyManager, "ActivateKeyResponse");
    });

    it("Should handle a second key activation (with KSK generation)", async function () {
      const { keyManager, admins, coprocessorSigners, keyId1, keyId2 } = await loadFixture(
        deployKeyManagerActivateFixture,
      );

      // Activate the 2nd key
      await keyManager.connect(admins[0]).activateKeyRequest(keyId2);
      await keyManager.connect(coprocessorSigners[0]).activateKeyResponse(keyId2);
      await keyManager.connect(coprocessorSigners[1]).activateKeyResponse(keyId2);
      await keyManager.connect(coprocessorSigners[2]).activateKeyResponse(keyId2);

      // Check that the 2nd key is activated and is now the current key, while the 1st key is not
      expect(await keyManager.isCurrentKeyId(keyId2)).to.be.true;
      expect(await keyManager.isCurrentKeyId(keyId1)).to.be.false;

      // Check that we can get both activated key ids
      expect(await keyManager.activatedKeyIds(0)).to.be.equal(keyId1);
      expect(await keyManager.activatedKeyIds(1)).to.be.equal(keyId2);
    });
  });

  describe("FHE parameters", async function () {
    it("Should revert because of access controls", async function () {
      const { keyManager, user } = await loadFixture(deployKeyManagerFixture);

      // Get dummy FHE params
      const fheParams = getFheParams();

      // Check that only the owner can set the FHE params
      await expect(keyManager.connect(user).setFheParams(fheParams))
        .to.be.revertedWithCustomError(keyManager, "OwnableUnauthorizedAccount")
        .withArgs(user.address);

      // Check that only the owner can update the FHE params
      await expect(keyManager.connect(user).updateFheParams(fheParams))
        .to.be.revertedWithCustomError(keyManager, "OwnableUnauthorizedAccount")
        .withArgs(user.address);
    });

    it("Should update fheParams", async function () {
      const { keyManager, owner } = await loadFixture(deployKeyManagerNoParamsFixture);

      // Get dummy FHE params
      const fheParams = getFheParams();

      // Check that FHE params cannot be updated if they are not initialized
      await expect(keyManager.connect(owner).updateFheParams(fheParams)).to.be.revertedWithCustomError(
        keyManager,
        "FheParamsNotInitialized",
      );

      // Set the FHE params
      const txSetFheParams = await keyManager.connect(owner).setFheParams(fheParams);

      // Check event
      await expect(txSetFheParams).to.emit(keyManager, "SetFheParams").withArgs([fheParams.dummy]);

      // Check that we can only set the FHE params once
      await expect(keyManager.setFheParams(fheParams)).to.be.revertedWithCustomError(
        keyManager,
        "FheParamsAlreadyInitialized",
      );

      // Update the FHE params
      const txUpdateFheParams = await keyManager.connect(owner).updateFheParams(fheParams);

      // Check event
      await expect(txUpdateFheParams).to.emit(keyManager, "UpdateFheParams").withArgs([fheParams.dummy]);
    });
  });
});
