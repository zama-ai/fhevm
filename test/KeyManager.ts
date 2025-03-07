import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { EventLog } from "ethers";
import hre from "hardhat";

import { deployKeyManagerFixture } from "./utils/deploys";

describe("KeyManager", function () {
  const fakeFheParamsName = "FAKE_FHE_PARAMS_NAME";

  // Deploy the keyManager contract and run a preprocessing keygen
  async function deployKeyManagerPreKeygenFixture() {
    const { keyManager, owner, admins, user, kmsSigners, coprocessorSigners, signers, fheParamsName, fheParamsDigest } =
      await loadFixture(deployKeyManagerFixture);

    // Trigger a preprocessing keygen request
    const txRequest = await keyManager.connect(admins[0]).preprocessKeygenRequest(fheParamsName);

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

    return {
      keyManager,
      owner,
      admins,
      user,
      kmsSigners,
      coprocessorSigners,
      signers,
      preKeyId,
      fheParamsName,
      fheParamsDigest,
    };
  }

  // Deploy the keyManager contract and run a keygen
  async function deployKeyManagerKeygenFixture() {
    const {
      keyManager,
      owner,
      admins,
      user,
      kmsSigners,
      coprocessorSigners,
      signers,
      fheParamsName,
      fheParamsDigest,
      preKeyId,
    } = await loadFixture(deployKeyManagerPreKeygenFixture);

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

    return {
      keyManager,
      owner,
      admins,
      user,
      kmsSigners,
      coprocessorSigners,
      signers,
      fheParamsName,
      fheParamsDigest,
      keyId1,
      keyId2,
    };
  }

  // Deploy the keyManager contract and run a preprocessing KSK generation
  async function deployKeyManagerPreKskgenFixture() {
    const {
      keyManager,
      owner,
      admins,
      user,
      kmsSigners,
      coprocessorSigners,
      signers,
      fheParamsName,
      fheParamsDigest,
      keyId1,
      keyId2,
    } = await loadFixture(deployKeyManagerKeygenFixture);

    // Trigger a preprocessing KSK generation request
    const txRequest = await keyManager.connect(admins[0]).preprocessKskgenRequest(fheParamsName);

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
      fheParamsName,
      fheParamsDigest,
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
      fheParamsName,
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
      fheParamsName,
      keyId1,
      keyId2,
      kskId,
    };
  }

  describe("Key generation", function () {
    it("Should revert if the FHE params are not initialized", async function () {
      const { keyManager, admins } = await loadFixture(deployKeyManagerFixture);

      // Check that a preprocessing keygen request cannot be triggered if the fheParams are not initialized
      await expect(
        keyManager.connect(admins[0]).preprocessKeygenRequest(fakeFheParamsName),
      ).to.be.revertedWithCustomError(keyManager, "FheParamsNotInitialized");
    });

    it("Should revert because of access controls", async function () {
      const { httpz, keyManager, user, fheParamsName } = await loadFixture(deployKeyManagerFixture);

      // Check that someone else than the admin cannot trigger a preprocessing keygen request
      await expect(keyManager.connect(user).preprocessKeygenRequest(fheParamsName))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Check that someone else than the admin cannot trigger a preprocessing keygen response
      await expect(keyManager.connect(user).preprocessKeygenResponse(0, 0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.KMS_NODE_ROLE());

      // Check that someone else than the admin cannot trigger a keygen request
      await expect(keyManager.connect(user).keygenRequest(0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Check that someone else than the KMS node cannot trigger a keygen response
      await expect(keyManager.connect(user).keygenResponse(0, 0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.KMS_NODE_ROLE());
    });

    it("Should handle a preprocessed keygen", async function () {
      const { keyManager, admins, kmsSigners, fheParamsName, fheParamsDigest } =
        await loadFixture(deployKeyManagerFixture);

      // Define the expected preprocessing key request ID
      const expectedPreKeyRequestId = 1;

      // Trigger a preprocessing keygen request
      const txRequest = await keyManager.connect(admins[0]).preprocessKeygenRequest(fheParamsName);

      // Check event
      await expect(txRequest)
        .to.emit(keyManager, "PreprocessKeygenRequest")
        .withArgs(expectedPreKeyRequestId, fheParamsDigest);

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
      const txRequest2 = await keyManager.connect(admins[0]).preprocessKeygenRequest(fheParamsName);
      await expect(txRequest2)
        .to.emit(keyManager, "PreprocessKeygenRequest")
        .withArgs(expectedPreKeyRequestId + 1, fheParamsDigest);
    });

    it("Should handle a keygen", async function () {
      const { keyManager, admins, kmsSigners, preKeyId, fheParamsDigest } = await loadFixture(
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
      await expect(txRequest).to.emit(keyManager, "KeygenRequest").withArgs(preKeyId, fheParamsDigest);

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
      await expect(txResponse2).to.emit(keyManager, "KeygenResponse").withArgs(preKeyId, keyId, fheParamsDigest);

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
      const { keyManager, admins } = await loadFixture(deployKeyManagerFixture);

      // Check that a CRS generation request cannot be triggered if the fheParams are not initialized
      await expect(keyManager.connect(admins[0]).crsgenRequest(fakeFheParamsName)).to.be.revertedWithCustomError(
        keyManager,
        "FheParamsNotInitialized",
      );
    });

    it("Should revert because of access controls", async function () {
      const { httpz, keyManager, user, fheParamsName } = await loadFixture(deployKeyManagerFixture);

      // Check that someone else than the admin cannot trigger a CRS generation request
      await expect(keyManager.connect(user).crsgenRequest(fheParamsName))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Check that someone else than the KMS node cannot trigger a CRS generation response
      await expect(keyManager.connect(user).crsgenResponse(0, 0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.KMS_NODE_ROLE());
    });

    it("Should handle a CRS generation", async function () {
      const { keyManager, admins, kmsSigners, fheParamsName, fheParamsDigest } =
        await loadFixture(deployKeyManagerFixture);

      // Define an expected preCrsId
      const expectedPreCrsId = 1;

      // Trigger a CRS generation request
      const txRequest = await keyManager.connect(admins[0]).crsgenRequest(fheParamsName);

      // Check event
      await expect(txRequest).to.emit(keyManager, "CrsgenRequest").withArgs(expectedPreCrsId, fheParamsDigest);

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
        .withArgs(expectedPreCrsId, crsId, fheParamsDigest);

      // The 3rd and 4th responses should be ignored (not reverted) and not emit an event
      const txResponse3 = await keyManager.connect(kmsSigners[2]).crsgenResponse(expectedPreCrsId, crsId);
      const txResponse4 = await keyManager.connect(kmsSigners[3]).crsgenResponse(expectedPreCrsId, crsId);

      // Check that the 3rd and 4th responses do not emit an event
      await expect(txResponse3).to.not.emit(keyManager, "CrsgenResponse");
      await expect(txResponse4).to.not.emit(keyManager, "CrsgenResponse");

      // Check that triggering a new preprocessing keygen request again gives a different preKeyId
      // (the counter is incremented by 1)
      const txRequest2 = await keyManager.connect(admins[0]).crsgenRequest(fheParamsName);
      await expect(txRequest2)
        .to.emit(keyManager, "CrsgenRequest")
        .withArgs(expectedPreCrsId + 1, fheParamsDigest);
    });
  });

  describe("KSK generation", async function () {
    it("Should revert if the FHE params are not initialized", async function () {
      const { keyManager, admins } = await loadFixture(deployKeyManagerFixture);

      // Check that a preprocessing KSK generation request cannot be triggered if the fheParams are not initialized
      await expect(
        keyManager.connect(admins[0]).preprocessKskgenRequest(fakeFheParamsName),
      ).to.be.revertedWithCustomError(keyManager, "FheParamsNotInitialized");
    });

    it("Should revert because of access controls", async function () {
      const { httpz, keyManager, user, fheParamsName } = await loadFixture(deployKeyManagerFixture);

      // Check that someone else than the admin cannot trigger a preprocessing KSK generation request
      await expect(keyManager.connect(user).preprocessKskgenRequest(fheParamsName))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Check that someone else than the KMS node cannot trigger a preprocessing KSK generation response
      await expect(keyManager.connect(user).preprocessKskgenResponse(0, 0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.KMS_NODE_ROLE());

      // Check that someone else than the admin cannot trigger a KSK generation request
      await expect(keyManager.connect(user).kskgenRequest(0, 0, 0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Check that someone else than the KMS node cannot trigger a KSK generation response
      await expect(keyManager.connect(user).kskgenResponse(0, 0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.KMS_NODE_ROLE());
    });

    it("Should handle a preprocessed KSK generation", async function () {
      const { keyManager, admins, kmsSigners, fheParamsName, fheParamsDigest } =
        await loadFixture(deployKeyManagerFixture);

      // Define the expected preprocessing KSK ID
      const expectedPreKskRequestId = 1;

      // Trigger a preprocessing KSK generation request
      const txRequest = await keyManager.connect(admins[0]).preprocessKskgenRequest(fheParamsName);

      // Check event
      await expect(txRequest)
        .to.emit(keyManager, "PreprocessKskgenRequest")
        .withArgs(expectedPreKskRequestId, fheParamsDigest);

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
      const txRequest2 = await keyManager.connect(admins[0]).preprocessKskgenRequest(fheParamsName);
      await expect(txRequest2)
        .to.emit(keyManager, "PreprocessKskgenRequest")
        .withArgs(expectedPreKskRequestId + 1, fheParamsDigest);
    });

    it("Should handle a KSK generation", async function () {
      const { keyManager, admins, kmsSigners, fheParamsDigest, keyId1, keyId2, preKskId } = await loadFixture(
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
      await expect(txRequest).to.emit(keyManager, "KskgenRequest").withArgs(preKskId, keyId1, keyId2, fheParamsDigest);

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
      await expect(txResponse2).to.emit(keyManager, "KskgenResponse").withArgs(preKskId, kskId, fheParamsDigest);

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
      const { httpz, keyManager, user } = await loadFixture(deployKeyManagerFixture);

      // Check that someone else than the admin cannot trigger a key activation request
      await expect(keyManager.connect(user).activateKeyRequest(0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Check that someone else than the coprocessor cannot trigger a key activation response
      await expect(keyManager.connect(user).activateKeyResponse(0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.COPROCESSOR_ROLE());
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
      const fheParamsName = "TEST";
      const fheParamsDigest = hre.ethers.randomBytes(32);

      // Check that only the owner can set the FHE params
      await expect(keyManager.connect(user).addFheParams(fheParamsName, fheParamsDigest))
        .to.be.revertedWithCustomError(keyManager, "OwnableUnauthorizedAccount")
        .withArgs(user.address);

      // Check that only the owner can update the FHE params
      await expect(keyManager.connect(user).updateFheParams(fheParamsName, fheParamsDigest))
        .to.be.revertedWithCustomError(keyManager, "OwnableUnauthorizedAccount")
        .withArgs(user.address);
    });

    it("Should add fheParams", async function () {
      const { keyManager, owner } = await loadFixture(deployKeyManagerFixture);

      // Get dummy FHE params
      const newFheParamsName = "DEFAULT";
      const newFheParamsDigest = hre.ethers.randomBytes(32);

      // Set the FHE params
      const txSetFheParams = await keyManager.connect(owner).addFheParams(newFheParamsName, newFheParamsDigest);

      // Check event
      await expect(txSetFheParams).to.emit(keyManager, "AddFheParams").withArgs(newFheParamsName, newFheParamsDigest);
    });

    it("Should revert when adding fheParams because they are initialized", async function () {
      const { keyManager, owner, fheParamsName } = await loadFixture(deployKeyManagerFixture);

      // Get dummy FHE params digest
      const newFheParamsDigest = hre.ethers.randomBytes(32);

      // Check that we can only set the FHE params once
      await expect(keyManager.connect(owner).addFheParams(fheParamsName, newFheParamsDigest))
        .to.be.revertedWithCustomError(keyManager, "FheParamsAlreadyInitialized")
        .withArgs(fheParamsName);
    });

    it("Should update fheParams", async function () {
      const { keyManager, owner, fheParamsName } = await loadFixture(deployKeyManagerFixture);

      // Get dummy FHE params
      const newFheParamsDigest = hre.ethers.randomBytes(32);

      // Update the FHE params
      const txUpdateFheParams = await keyManager.connect(owner).updateFheParams(fheParamsName, newFheParamsDigest);

      // Check event
      await expect(txUpdateFheParams)
        .to.emit(keyManager, "UpdateFheParams")
        .withArgs(fheParamsName, newFheParamsDigest);
    });

    it("Should revert when updating fheParams because they are not initialized", async function () {
      const { keyManager, owner } = await loadFixture(deployKeyManagerFixture);

      // Get dummy FHE params
      const newFheParamsDigest = hre.ethers.randomBytes(32);

      // Check that FHE params cannot be updated if they are not initialized
      await expect(
        keyManager.connect(owner).updateFheParams(fakeFheParamsName, newFheParamsDigest),
      ).to.be.revertedWithCustomError(keyManager, "FheParamsNotInitialized");
    });
  });
});
