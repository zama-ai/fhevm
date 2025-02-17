import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { EventLog } from "ethers";
import hre from "hardhat";

import { deployHTTPZFixture } from "./utils/deploys";

describe("HTTPZ", function () {
  // Deploy the HTTPZ contract
  async function deployHTTPZOnlyFixture() {
    const allSigners = await hre.ethers.getSigners();
    const owner = allSigners[0];

    const HTTPZ = await hre.ethers.getContractFactory("HTTPZ", owner);
    const httpz = await HTTPZ.connect(owner).deploy();

    // The first signer is the owner
    const signers = allSigners.slice(1);
    return { httpz, owner, signers };
  }

  // Deploy the HTTPZ contract and initialize the protocol
  async function deployInitHTTPZFixture() {
    const { httpz, owner, signers } = await loadFixture(deployHTTPZOnlyFixture);

    // Define the admin and a user
    const [admin, user] = signers;

    const protocolMetadata = { name: "Protocol", website: "https://protocol.com" };
    await httpz.initialize(protocolMetadata, [admin.address]);

    const allSigners = signers.slice(2);
    return { httpz, owner, admin, user, signers: allSigners };
  }

  // Define dummy FHE params to use for testing generation methods
  function getFheParams() {
    // TODO: Use proper fheParams format when implemented
    return {
      dummy: "dummy",
    };
  }

  // Deploy the HTTPZ contract, initialize the protocol, add KMS nodes and set the FHE params
  // for key/CRS/KSK generations
  async function deployHTTPZParamsFixture() {
    const { httpz, owner, admin, user, kmsSigners, coprocessorSigners, signers } =
      await loadFixture(deployHTTPZFixture);

    // TODO: Use proper fheParams format when implemented
    const fheParams = getFheParams();

    await httpz.setFheParams(fheParams);

    return { httpz, owner, admin, user, kmsSigners, coprocessorSigners, signers, fheParams };
  }

  // Deploy the HTTPZ contract and run a preprocessing keygen
  async function deployHTTPZPreKeygenFixture() {
    const { httpz, owner, admin, user, kmsSigners, coprocessorSigners, signers, fheParams } =
      await loadFixture(deployHTTPZParamsFixture);

    // Trigger a preprocessing keygen request
    const txRequest = await httpz.connect(admin).preprocessKeygenRequest();

    const receipt = await txRequest.wait();

    // Get the preKeyRequestId from the event in the transaction receipt (preKeyRequestId is the first argument of the event)
    const event = receipt?.logs[0] as EventLog;
    const preKeyRequestId = Number(event?.args[0]);

    // Define a preKeyId for the keygen response
    const preKeyId = 1;

    // Trigger preprocessing keygen responses for all KMS nodes
    // Note: not all responses are strictly needed (the consensus can be reached before the last response(s))
    for (let i = 0; i < kmsSigners.length; i++) {
      await httpz.connect(kmsSigners[i]).preprocessKeygenResponse(preKeyRequestId, preKeyId);
    }

    return { httpz, owner, admin, user, kmsSigners, coprocessorSigners, signers, fheParams, preKeyId };
  }

  // Deploy the HTTPZ contract and run a keygen
  async function deployHTTPZKeygenFixture() {
    const { httpz, owner, admin, user, kmsSigners, coprocessorSigners, signers, fheParams, preKeyId } =
      await loadFixture(deployHTTPZPreKeygenFixture);

    // Trigger a keygen request
    await httpz.connect(admin).keygenRequest(preKeyId);

    // Define 2 keyIds for keygen responses
    const keyId1 = 1;
    const keyId2 = 2;

    // Trigger keygen responses for all KMS nodes
    // Note: not all responses are strictly needed (the consensus can be reached before the last response(s))
    for (let i = 0; i < kmsSigners.length; i++) {
      await httpz.connect(kmsSigners[i]).keygenResponse(preKeyId, keyId1);
      await httpz.connect(kmsSigners[i]).keygenResponse(preKeyId, keyId2);
    }

    return { httpz, owner, admin, user, kmsSigners, coprocessorSigners, signers, fheParams, keyId1, keyId2 };
  }

  // Deploy the HTTPZ contract and run a preprocessing KSK generation
  async function deployHTTPZPreKskgenFixture() {
    const { httpz, owner, admin, user, kmsSigners, coprocessorSigners, signers, fheParams, keyId1, keyId2 } =
      await loadFixture(deployHTTPZKeygenFixture);

    // Trigger a preprocessing KSK generation request
    const txRequest = await httpz.connect(admin).preprocessKskgenRequest();

    const receipt = await txRequest.wait();

    // Get the preKskRequestId from the event in the transaction receipt (preKskRequestId is the first argument of the event)
    const event = receipt?.logs[0] as EventLog;
    const preKskRequestId = Number(event?.args[0]);

    // Define a preKskId for the KSK generation response
    const preKskId = 1;

    // Trigger preprocessing KSK generation responses for all KMS nodes
    // Note: not all responses are strictly needed (the consensus can be reached before the last response(s))
    for (let i = 0; i < kmsSigners.length; i++) {
      await httpz.connect(kmsSigners[i]).preprocessKskgenResponse(preKskRequestId, preKskId);
    }

    return { httpz, owner, admin, user, kmsSigners, coprocessorSigners, signers, fheParams, keyId1, keyId2, preKskId };
  }

  // Deploy the HTTPZ contract, run a KSK generation and activate the first key
  async function deployHTTPZActivateFixture() {
    const { httpz, owner, admin, user, kmsSigners, coprocessorSigners, signers, fheParams, keyId1, keyId2, preKskId } =
      await loadFixture(deployHTTPZPreKskgenFixture);

    // Trigger a KSK generation request
    await httpz.connect(admin).kskgenRequest(preKskId, keyId1, keyId2);

    // Define a kskId for KSK generation response
    const kskId = 1;

    // Trigger preprocessing KSK generation responses for all KMS nodes
    // Note: not all responses are strictly needed (the consensus can be reached before the last response(s))
    for (let i = 0; i < kmsSigners.length; i++) {
      await httpz.connect(kmsSigners[i]).kskgenResponse(preKskId, kskId);
    }

    // Activate the first key
    await httpz.connect(admin).activateKeyRequest(keyId1);

    // Trigger preprocessing KSK generation responses for all coprocessors
    for (let i = 0; i < coprocessorSigners.length; i++) {
      await httpz.connect(coprocessorSigners[i]).activateKeyResponse(keyId1);
    }

    return { httpz, owner, admin, user, kmsSigners, coprocessorSigners, signers, fheParams, keyId1, keyId2, kskId };
  }

  describe("Initialization", function () {
    it("Should initialize", async function () {
      const { httpz, owner, signers } = await loadFixture(deployHTTPZOnlyFixture);

      const protocolMetadata = { name: "Protocol", website: "https://protocol.com" };
      const admins = [signers[0].address, signers[1].address];

      // Check that someone else than the owner cannot initialize
      await expect(httpz.connect(signers[0]).initialize(protocolMetadata, admins))
        .to.be.revertedWithCustomError(httpz, "OwnableUnauthorizedAccount")
        .withArgs(signers[0].address);

      // Initialize the protocol
      const tx = await httpz.connect(owner).initialize(protocolMetadata, admins);

      // Check event
      await expect(tx)
        .to.emit(httpz, "Initialization")
        .withArgs([protocolMetadata.name, protocolMetadata.website], admins);
    });
  });

  describe("KMS Nodes", function () {
    it("Should add KMS nodes amd mark them as ready", async function () {
      const { httpz, admin, signers } = await loadFixture(deployInitHTTPZFixture);

      // KMS Node 1
      const connector1 = signers[0];
      const identity1 = hre.ethers.randomBytes(32);

      // KMS Node 2
      const connector2 = signers[1];
      const identity2 = hre.ethers.randomBytes(32);

      const kmsNodes = [
        { connectorAddress: connector1.address, identity: identity1, ipAddress: "127.0.0.1" },
        { connectorAddress: connector2.address, identity: identity2, ipAddress: "127.0.0.2" },
      ];

      // Response values
      const signedNodes1 = hre.ethers.randomBytes(32);
      const signedNodes2 = hre.ethers.randomBytes(32);
      const keychainDaAddress1 = "0x1234567890abcdef1234567890abcdef12345678";
      const keychainDaAddress2 = "0xabcdef1234567890abcdef1234567890abcdef12";

      // Define a user
      const user = signers[2];

      // Check that someone else than the admin cannot add KMS nodes
      await expect(httpz.connect(user).addKmsNodes(kmsNodes))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Add KMS nodes
      const addTx = await httpz.connect(admin).addKmsNodes(kmsNodes);

      // Check init event
      await expect(addTx).to.emit(httpz, "KmsNodesInit").withArgs([identity1, identity2]);

      // Check that someone else than the pending KMS connector cannot mark a KMS node as ready
      await expect(httpz.connect(user).kmsNodeReady(signedNodes1, keychainDaAddress1))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.PENDING_KMS_NODE_ROLE());

      // Mark KMS first node as ready
      await httpz.connect(connector1).kmsNodeReady(signedNodes1, keychainDaAddress1);

      // Check that a connector cannot mark KMS node as ready twice
      await expect(httpz.connect(connector1).kmsNodeReady(signedNodes1, keychainDaAddress1))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(connector1.address, httpz.PENDING_KMS_NODE_ROLE());

      // Mark second KMS node as ready (this should emit the KmsServiceReady event)
      const readyTx2 = await httpz.connect(connector2).kmsNodeReady(signedNodes2, keychainDaAddress2);

      // Check ready event
      await expect(readyTx2).to.emit(httpz, "KmsServiceReady").withArgs([identity1, identity2]);

      // Check that both KMS nodes are ready
      const isKmsNode1 = await httpz.isKmsNode(connector1.address);
      const isKmsNode2 = await httpz.isKmsNode(connector2.address);
      expect(isKmsNode1).to.be.true;
      expect(isKmsNode2).to.be.true;
    });
  });

  describe("Coprocessors", function () {
    it("Should add coprocessors and mark them as ready", async function () {
      const { httpz, admin, signers } = await loadFixture(deployInitHTTPZFixture);

      // Coprocessor 1
      const connector1 = signers[0];
      const identity1 = hre.ethers.randomBytes(32);

      // Coprocessor 2
      const connector2 = signers[1];
      const identity2 = hre.ethers.randomBytes(32);

      const coprocessors = [
        { connectorAddress: connector1.address, identity: identity1 },
        { connectorAddress: connector2.address, identity: identity2 },
      ];

      // Response values
      const coprocessorDaAddress1 = "0x1234567890abcdef1234567890abcdef12345678";
      const coprocessorDaAddress2 = "0xabcdef1234567890abcdef1234567890abcdef12";

      // Define a user
      const user = signers[2];

      // Check that someone else than the admin cannot add coprocessors
      await expect(httpz.connect(user).addCoprocessors(coprocessors))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Add coprocessors
      const addTx = await httpz.connect(admin).addCoprocessors(coprocessors);

      // Check init event
      await expect(addTx).to.emit(httpz, "CoprocessorsInit").withArgs([identity1, identity2]);

      // Check that someone else than the connector cannot mark coprocessor as ready
      await expect(httpz.connect(user).coprocessorReady(coprocessorDaAddress1))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.PENDING_COPROCESSOR_ROLE());

      // Mark first coprocessor as ready
      await httpz.connect(connector1).coprocessorReady(coprocessorDaAddress1);

      // Check that a connector cannot mark coprocessor as ready twice
      await expect(httpz.connect(connector1).coprocessorReady(coprocessorDaAddress1))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(connector1.address, httpz.PENDING_COPROCESSOR_ROLE());

      // Mark second coprocessor as ready (this should emit the CoprocessorServiceReady event)
      const readyTx2 = await httpz.connect(connector2).coprocessorReady(coprocessorDaAddress2);

      // Check ready event
      await expect(readyTx2).to.emit(httpz, "CoprocessorServiceReady").withArgs([identity1, identity2]);

      // Check that both coprocessors are ready
      const isCoprocessor1 = await httpz.isCoprocessor(connector1.address);
      const isCoprocessor2 = await httpz.isCoprocessor(connector2.address);
      expect(isCoprocessor1).to.be.true;
      expect(isCoprocessor2).to.be.true;
    });
  });

  describe("Networks", function () {
    it("Should add a network", async function () {
      const { httpz, admin, signers } = await loadFixture(deployInitHTTPZFixture);

      const chainId = 2025;
      const network = {
        chainId: chainId,
        httpzLibrary: "0x1234567890abcdef1234567890abcdef12345678",
        acl: "0xabcdef1234567890abcdef1234567890abcdef12",
        name: "Network",
        website: "https://network.com",
      };

      // Define a user
      const user = signers[0];

      // Check that someone else than the admin cannot add a network
      await expect(httpz.connect(user).addNetwork(network))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Add network
      const tx = await httpz.connect(admin).addNetwork(network);

      // Check event
      await expect(tx).to.emit(httpz, "AddNetwork").withArgs(chainId);

      // Check that the network is registered
      const isNetwork = await httpz.isNetwork(chainId);
      expect(isNetwork).to.be.true;
    });
  });

  describe("Key generation", function () {
    it("Should revert if the FHE params are not initialized", async function () {
      const { httpz, admin } = await loadFixture(deployHTTPZFixture);

      // Check that a preprocessing keygen request cannot be triggered if the fheParams are not initialized
      await expect(httpz.connect(admin).preprocessKeygenRequest()).to.be.revertedWithCustomError(
        httpz,
        "FheParamsNotInitialized",
      );
    });

    it("Should revert if the KMS nodes are not set", async function () {
      const { httpz, admin } = await loadFixture(deployInitHTTPZFixture);

      // Check that a preprocessing keygen request cannot be triggered if the KMS nodes are not set
      await expect(httpz.connect(admin).preprocessKeygenRequest()).to.be.revertedWithCustomError(
        httpz,
        "KmsNodesNotSet",
      );
    });

    it("Should revert because of access controls", async function () {
      const { httpz, user } = await loadFixture(deployHTTPZFixture);

      // Check that someone else than the admin cannot trigger a preprocessing keygen request
      await expect(httpz.connect(user).preprocessKeygenRequest())
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Check that someone else than the admin cannot trigger a preprocessing keygen response
      await expect(httpz.connect(user).preprocessKeygenResponse(0, 0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.KMS_NODE_ROLE());

      // Check that someone else than the admin cannot trigger a keygen request
      await expect(httpz.connect(user).keygenRequest(0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Check that someone else than the KMS node cannot trigger a keygen response
      await expect(httpz.connect(user).keygenResponse(0, 0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.KMS_NODE_ROLE());
    });

    it("Should handle a preprocessed keygen", async function () {
      const { httpz, admin, kmsSigners, fheParams } = await loadFixture(deployHTTPZParamsFixture);

      // Define the expected preprocessing key request ID
      const expectedPreKeyRequestId = 1;

      // Trigger a preprocessing keygen request
      const txRequest = await httpz.connect(admin).preprocessKeygenRequest();

      // Check event
      await expect(txRequest)
        .to.emit(httpz, "PreprocessKeygenRequest")
        .withArgs(expectedPreKeyRequestId, [fheParams.dummy]);

      // Define a preKeyId for the keygen response
      const preKeyId = 1;

      // Trigger a preprocessing keygen response with the first KMS node
      const txResponse1 = await httpz
        .connect(kmsSigners[0])
        .preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(httpz, "PreprocessKeygenResponse");

      // Check that a KMS node cannot respond twice to the same preprocessing keygen request
      await expect(
        httpz.connect(kmsSigners[0]).preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId),
      ).to.be.revertedWithCustomError(httpz, "PreprocessKeygenKmsNodeAlreadyResponded");

      // Trigger a second preprocessing keygen response with the second KMS node, which should reach
      // consensus (4 / 3 + 1 = 2) and thus emit an event
      const txResponse2 = await httpz
        .connect(kmsSigners[1])
        .preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId);

      // Check event
      await expect(txResponse2).to.emit(httpz, "PreprocessKeygenResponse").withArgs(expectedPreKeyRequestId, preKeyId);

      // The 3rd and 4th responses should be ignored (not reverted) and not emit an event
      const txResponse3 = await httpz
        .connect(kmsSigners[2])
        .preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId);
      const txResponse4 = await httpz
        .connect(kmsSigners[3])
        .preprocessKeygenResponse(expectedPreKeyRequestId, preKeyId);

      // Check that the 3rd and 4th responses do not emit an event
      await expect(txResponse3).to.not.emit(httpz, "PreprocessKeygenResponse");
      await expect(txResponse4).to.not.emit(httpz, "PreprocessKeygenResponse");

      // Check that triggering a new preprocessing keygen request again gives a different preKeyRequestId
      // (the counter is incremented by 1)
      const txRequest2 = await httpz.connect(admin).preprocessKeygenRequest();
      await expect(txRequest2)
        .to.emit(httpz, "PreprocessKeygenRequest")
        .withArgs(expectedPreKeyRequestId + 1, [fheParams.dummy]);
    });

    it("Should handle a keygen", async function () {
      const { httpz, admin, kmsSigners, preKeyId, fheParams } = await loadFixture(deployHTTPZPreKeygenFixture);

      // Check that a keygen request cannot be triggered if the preprocessing keygen is not done,
      // using a preKeyId different than the one given by the preprocessing keygen
      const fakePreKeyId = preKeyId + 1;
      await expect(httpz.connect(admin).keygenRequest(fakePreKeyId))
        .to.be.revertedWithCustomError(httpz, "KeygenPreprocessingRequired")
        .withArgs(fakePreKeyId);

      // Trigger a keygen request
      const txRequest = await httpz.connect(admin).keygenRequest(preKeyId);

      // Check event
      await expect(txRequest).to.emit(httpz, "KeygenRequest").withArgs(preKeyId, [fheParams.dummy]);

      // Check that a keygen request cannot be triggered again with the same preKeyId
      await expect(httpz.connect(admin).keygenRequest(preKeyId))
        .to.be.revertedWithCustomError(httpz, "KeygenRequestAlreadySent")
        .withArgs(preKeyId);

      // Define a keyId for keygen responses
      const keyId = 1;

      // Trigger a keygen response
      const txResponse1 = await httpz.connect(kmsSigners[0]).keygenResponse(preKeyId, keyId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(httpz, "KeygenResponse");

      // Check that a KMS node cannot respond twice to the same keygen request
      await expect(httpz.connect(kmsSigners[0]).keygenResponse(preKeyId, keyId)).to.be.revertedWithCustomError(
        httpz,
        "KeygenKmsNodeAlreadyResponded",
      );

      // Trigger a second keygen response with the second KMS node, which should reach
      // consensus (4 / 3 + 1 = 2) and thus emit an event
      const txResponse2 = await httpz.connect(kmsSigners[1]).keygenResponse(preKeyId, keyId);

      // Check event
      await expect(txResponse2).to.emit(httpz, "KeygenResponse").withArgs(preKeyId, keyId, [fheParams.dummy]);

      // The 3rd and 4th responses should be ignored (not reverted) and not emit an event
      const txResponse3 = await httpz.connect(kmsSigners[2]).keygenResponse(preKeyId, keyId);
      const txResponse4 = await httpz.connect(kmsSigners[3]).keygenResponse(preKeyId, keyId);

      // Check that the 3rd and 4th responses do not emit an event
      await expect(txResponse3).to.not.emit(httpz, "KeygenResponse");
      await expect(txResponse4).to.not.emit(httpz, "KeygenResponse");
    });
  });

  describe("CRS generation", async function () {
    it("Should revert if the FHE params are not initialized", async function () {
      const { httpz, admin } = await loadFixture(deployHTTPZFixture);

      // Check that a CRS generation request cannot be triggered if the fheParams are not initialized
      await expect(httpz.connect(admin).crsgenRequest()).to.be.revertedWithCustomError(
        httpz,
        "FheParamsNotInitialized",
      );
    });

    it("Should revert if the KMS nodes are not set", async function () {
      const { httpz, admin } = await loadFixture(deployInitHTTPZFixture);

      // Check that a CRS generation request cannot be triggered if the KMS nodes are not set
      await expect(httpz.connect(admin).crsgenRequest()).to.be.revertedWithCustomError(httpz, "KmsNodesNotSet");
    });

    it("Should revert because of access controls", async function () {
      const { httpz, user } = await loadFixture(deployHTTPZFixture);

      // Check that someone else than the admin cannot trigger a CRS generation request
      await expect(httpz.connect(user).crsgenRequest())
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Check that someone else than the KMS node cannot trigger a CRS generation response
      await expect(httpz.connect(user).crsgenResponse(0, 0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.KMS_NODE_ROLE());
    });

    it("Should handle a CRS generation", async function () {
      const { httpz, admin, kmsSigners, fheParams } = await loadFixture(deployHTTPZParamsFixture);

      // Define an expected preCrsId
      const expectedPreCrsId = 1;

      // Trigger a CRS generation request
      const txRequest = await httpz.connect(admin).crsgenRequest();

      // Check event
      await expect(txRequest).to.emit(httpz, "CrsgenRequest").withArgs(expectedPreCrsId, [fheParams.dummy]);

      // Define a crsId for responses
      const crsId = 1;

      // Trigger a CRS generation response with the first KMS node
      const txResponse1 = await httpz.connect(kmsSigners[0]).crsgenResponse(expectedPreCrsId, crsId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(httpz, "CrsgenResponse");

      // Check that a KMS node cannot respond twice to the same CRS generation request
      await expect(httpz.connect(kmsSigners[0]).crsgenResponse(expectedPreCrsId, crsId)).to.be.revertedWithCustomError(
        httpz,
        "CrsgenKmsNodeAlreadyResponded",
      );

      // Trigger a second CRS generation response with the second KMS node, which should reach
      // consensus (4 / 3 + 1 = 2) and thus emit an event
      const txResponse2 = await httpz.connect(kmsSigners[1]).crsgenResponse(expectedPreCrsId, crsId);

      // Check event
      await expect(txResponse2).to.emit(httpz, "CrsgenResponse").withArgs(expectedPreCrsId, crsId, [fheParams.dummy]);

      // The 3rd and 4th responses should be ignored (not reverted) and not emit an event
      const txResponse3 = await httpz.connect(kmsSigners[2]).crsgenResponse(expectedPreCrsId, crsId);
      const txResponse4 = await httpz.connect(kmsSigners[3]).crsgenResponse(expectedPreCrsId, crsId);

      // Check that the 3rd and 4th responses do not emit an event
      await expect(txResponse3).to.not.emit(httpz, "CrsgenResponse");
      await expect(txResponse4).to.not.emit(httpz, "CrsgenResponse");

      // Check that triggering a new preprocessing keygen request again gives a different preKeyId
      // (the counter is incremented by 1)
      const txRequest2 = await httpz.connect(admin).crsgenRequest();
      await expect(txRequest2)
        .to.emit(httpz, "CrsgenRequest")
        .withArgs(expectedPreCrsId + 1, [fheParams.dummy]);
    });
  });

  describe("KSK generation", async function () {
    it("Should revert if the FHE params are not initialized", async function () {
      const { httpz, admin } = await loadFixture(deployHTTPZFixture);

      // Check that a preprocessing KSK generation request cannot be triggered if the fheParams are not initialized
      await expect(httpz.connect(admin).preprocessKskgenRequest()).to.be.revertedWithCustomError(
        httpz,
        "FheParamsNotInitialized",
      );
    });

    it("Should revert if the KMS nodes are not set", async function () {
      const { httpz, admin } = await loadFixture(deployInitHTTPZFixture);

      // Check that a preprocessing KSK generation request cannot be triggered if the KMS nodes are not set
      await expect(httpz.connect(admin).preprocessKskgenRequest()).to.be.revertedWithCustomError(
        httpz,
        "KmsNodesNotSet",
      );
    });

    it("Should revert because of access controls", async function () {
      const { httpz, user } = await loadFixture(deployHTTPZFixture);

      // Check that someone else than the admin cannot trigger a preprocessing KSK generation request
      await expect(httpz.connect(user).preprocessKskgenRequest())
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Check that someone else than the KMS node cannot trigger a preprocessing KSK generation response
      await expect(httpz.connect(user).preprocessKskgenResponse(0, 0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.KMS_NODE_ROLE());

      // Check that someone else than the admin cannot trigger a KSK generation request
      await expect(httpz.connect(user).kskgenRequest(0, 0, 0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Check that someone else than the KMS node cannot trigger a KSK generation response
      await expect(httpz.connect(user).kskgenResponse(0, 0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.KMS_NODE_ROLE());
    });

    it("Should handle a preprocessed KSK generation", async function () {
      const { httpz, admin, kmsSigners, fheParams } = await loadFixture(deployHTTPZParamsFixture);

      // Define the expected preprocessing KSK ID
      const expectedPreKskRequestId = 1;

      // Trigger a preprocessing KSK generation request
      const txRequest = await httpz.connect(admin).preprocessKskgenRequest();

      // Check event
      await expect(txRequest)
        .to.emit(httpz, "PreprocessKskgenRequest")
        .withArgs(expectedPreKskRequestId, [fheParams.dummy]);

      // Define a preKskRequestId for the preprocessing KSK generation request
      const preKskRequestId = 1;

      // Trigger a preprocessing KSK generation response with the first KMS node
      const txResponse1 = await httpz
        .connect(kmsSigners[0])
        .preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(httpz, "PreprocessKskgenResponse");

      // Check that a KMS node cannot respond twice to the same preprocessing KSK generation request
      await expect(
        httpz.connect(kmsSigners[0]).preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId),
      ).to.be.revertedWithCustomError(httpz, "PreprocessKskgenKmsNodeAlreadyResponded");

      // Trigger a second preprocessing KSK generation response with the second KMS node, which should reach
      // consensus (4 / 3 + 1 = 2) and thus emit an event
      const txResponse2 = await httpz
        .connect(kmsSigners[1])
        .preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId);

      // Check event
      await expect(txResponse2)
        .to.emit(httpz, "PreprocessKskgenResponse")
        .withArgs(expectedPreKskRequestId, preKskRequestId);

      // The 3rd and 4th responses should be ignored (not reverted) and not emit an event
      const txResponse3 = await httpz
        .connect(kmsSigners[2])
        .preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId);
      const txResponse4 = await httpz
        .connect(kmsSigners[3])
        .preprocessKskgenResponse(expectedPreKskRequestId, preKskRequestId);

      // Check that the 3rd and 4th responses do not emit an event
      await expect(txResponse3).to.not.emit(httpz, "PreprocessKskgenResponse");
      await expect(txResponse4).to.not.emit(httpz, "PreprocessKskgenResponse");

      // Check that triggering a new preprocessing KSK generation request again gives a different preKskRequestId
      // (the counter is incremented by 1)
      const txRequest2 = await httpz.connect(admin).preprocessKskgenRequest();
      await expect(txRequest2)
        .to.emit(httpz, "PreprocessKskgenRequest")
        .withArgs(expectedPreKskRequestId + 1, [fheParams.dummy]);
    });

    it("Should handle a KSK generation", async function () {
      const { httpz, admin, kmsSigners, fheParams, keyId1, keyId2, preKskId } =
        await loadFixture(deployHTTPZPreKskgenFixture);

      // Check that a KSK generation request cannot be triggered if the preprocessing KSK generation is not done,
      // using a preKskId different than the one given by the preprocessing KSK generation
      const fakePreKskId = preKskId + 1;
      await expect(httpz.connect(admin).kskgenRequest(fakePreKskId, keyId1, keyId2))
        .to.be.revertedWithCustomError(httpz, "KskgenPreprocessingRequired")
        .withArgs(fakePreKskId);

      // Check that the source key must be different from the destination key
      await expect(httpz.connect(admin).kskgenRequest(preKskId, keyId1, keyId1))
        .to.be.revertedWithCustomError(httpz, "KskgenSameSrcAndDestKeyIds")
        .withArgs(keyId1);

      // Check that the source key must be generated
      const fakeKeyId = keyId1 + keyId2;
      await expect(httpz.connect(admin).kskgenRequest(preKskId, fakeKeyId, keyId2))
        .to.be.revertedWithCustomError(httpz, "KskgenSourceKeyNotGenerated")
        .withArgs(fakeKeyId);

      // Check that the destination key must be generated
      await expect(httpz.connect(admin).kskgenRequest(preKskId, keyId1, fakeKeyId))
        .to.be.revertedWithCustomError(httpz, "KskgenDestKeyNotGenerated")
        .withArgs(fakeKeyId);

      // Trigger a KSK generation request
      const txRequest = await httpz.connect(admin).kskgenRequest(preKskId, keyId1, keyId2);

      // Check event
      await expect(txRequest).to.emit(httpz, "KskgenRequest").withArgs(preKskId, keyId1, keyId2, [fheParams.dummy]);

      // Check that a keygen request cannot be triggered again with the same preKeyId
      await expect(httpz.connect(admin).kskgenRequest(preKskId, keyId1, keyId2))
        .to.be.revertedWithCustomError(httpz, "KskgenRequestAlreadySent")
        .withArgs(preKskId);

      // Define a keyId for keygen responses
      const kskId = 1;

      // Trigger a KSK generation response
      const txResponse1 = await httpz.connect(kmsSigners[0]).kskgenResponse(preKskId, kskId);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(httpz, "KskgenResponse");

      // Check that a KMS node cannot respond twice to the same KSK generation request
      await expect(httpz.connect(kmsSigners[0]).kskgenResponse(preKskId, kskId)).to.be.revertedWithCustomError(
        httpz,
        "KskgenKmsNodeAlreadyResponded",
      );

      // Trigger a second KSK generation response with the second KMS node, which should reach
      // consensus (4 / 3 + 1 = 2) and thus emit an event
      const txResponse2 = await httpz.connect(kmsSigners[1]).kskgenResponse(preKskId, kskId);

      // Check event
      await expect(txResponse2).to.emit(httpz, "KskgenResponse").withArgs(preKskId, kskId, [fheParams.dummy]);

      // The 3rd and 4th responses should be ignored (not reverted) and not emit an event
      const txResponse3 = await httpz.connect(kmsSigners[2]).kskgenResponse(preKskId, kskId);
      const txResponse4 = await httpz.connect(kmsSigners[3]).kskgenResponse(preKskId, kskId);

      // Check that the 3rd and 4th responses do not emit an event
      await expect(txResponse3).to.not.emit(httpz, "KskgenResponse");
      await expect(txResponse4).to.not.emit(httpz, "KskgenResponse");
    });
  });

  describe("Key activation", async function () {
    it("Should revert if the coprocessors are not set", async function () {
      const { httpz, admin } = await loadFixture(deployInitHTTPZFixture);

      // Check that a key activation request cannot be triggered if the coprocessors are not set
      await expect(httpz.connect(admin).activateKeyRequest(0)).to.be.revertedWithCustomError(
        httpz,
        "CoprocessorsNotSet",
      );
    });

    it("Should revert because of access controls", async function () {
      const { httpz, user } = await loadFixture(deployHTTPZFixture);

      // Check that someone else than the admin cannot trigger a key activation request
      await expect(httpz.connect(user).activateKeyRequest(0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());

      // Check that someone else than the coprocessor cannot trigger a key activation response
      await expect(httpz.connect(user).activateKeyResponse(0))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.COPROCESSOR_ROLE());
    });

    it("Should handle a first key activation (no KSK generation)", async function () {
      const { httpz, admin, coprocessorSigners, keyId1, keyId2 } = await loadFixture(deployHTTPZKeygenFixture);

      // Check that the key to activate must be generated
      const fakeKeyId = keyId1 + keyId2;
      await expect(httpz.connect(admin).activateKeyRequest(fakeKeyId))
        .to.be.revertedWithCustomError(httpz, "ActivateKeyRequiresKeygen")
        .withArgs(fakeKeyId);

      // Trigger a key activation request
      const txRequest1 = await httpz.connect(admin).activateKeyRequest(keyId1);

      // Check event
      await expect(txRequest1).to.emit(httpz, "ActivateKeyRequest").withArgs(keyId1);

      // Check that the key activation request cannot be triggered again with the same keyId
      await expect(httpz.connect(admin).activateKeyRequest(keyId1))
        .to.be.revertedWithCustomError(httpz, "ActivateKeyRequestAlreadySent")
        .withArgs(keyId1);

      // Trigger a key activation response
      const txResponse1 = await httpz.connect(coprocessorSigners[0]).activateKeyResponse(keyId1);

      // Check that the first response does not emit an event (consensus is not reached yet)
      await expect(txResponse1).to.not.emit(httpz, "ActivateKeyResponse");

      // Check that a coprocessor cannot respond twice to the same key activation request
      await expect(httpz.connect(coprocessorSigners[0]).activateKeyResponse(keyId1)).to.be.revertedWithCustomError(
        httpz,
        "ActivateKeyKmsNodeAlreadyResponded",
      );

      // TODO: Check if this threshold is correct
      // Trigger a 2nd key activation response with the 2nd coprocessor, which should
      // not reach consensus as well
      const txResponse2 = await httpz.connect(coprocessorSigners[1]).activateKeyResponse(keyId1);

      // Check that the 2nd response does not emit an event
      await expect(txResponse2).to.not.emit(httpz, "ActivateKeyResponse");

      // Trigger a 3rd key activation response with the 3rd coprocessor, which should reach consensus
      // (tests use a total of 3 coprocessors) and thus emit an event
      const txResponse3 = await httpz.connect(coprocessorSigners[2]).activateKeyResponse(keyId1);

      // Check that the 3rd response emits an event
      await expect(txResponse3).to.emit(httpz, "ActivateKeyResponse").withArgs(keyId1);

      // Check that the key is activated
      expect(await httpz.isCurrentKeyId(keyId1)).to.be.true;

      // Check that we cannot activate the 2nd key (which has been generated but for which a KSK key
      // has not been generated)
      await expect(httpz.connect(admin).activateKeyRequest(keyId2))
        .to.be.revertedWithCustomError(httpz, "ActivateKeyRequiresKskgen")
        .withArgs(keyId1, keyId2);
    });

    it("Should handle a second key activation (with KSK generation)", async function () {
      const { httpz, admin, coprocessorSigners, keyId1, keyId2 } = await loadFixture(deployHTTPZActivateFixture);

      // Activate the 2nd key
      await httpz.connect(admin).activateKeyRequest(keyId2);
      await httpz.connect(coprocessorSigners[0]).activateKeyResponse(keyId2);
      await httpz.connect(coprocessorSigners[1]).activateKeyResponse(keyId2);
      await httpz.connect(coprocessorSigners[2]).activateKeyResponse(keyId2);

      // Check that the 2nd key is activated and is now the current key, while the 1st key is not
      expect(await httpz.isCurrentKeyId(keyId2)).to.be.true;
      expect(await httpz.isCurrentKeyId(keyId1)).to.be.false;

      // Check that we can get both activated key ids
      expect(await httpz.activatedKeyIds(0)).to.be.equal(keyId1);
      expect(await httpz.activatedKeyIds(1)).to.be.equal(keyId2);
    });
  });

  describe("FHE parameters", async function () {
    it("Should revert because of access controls", async function () {
      const { httpz, user } = await loadFixture(deployHTTPZFixture);

      // Get dummy FHE params
      const fheParams = getFheParams();

      // Check that only the owner can set the FHE params
      await expect(httpz.connect(user).setFheParams(fheParams))
        .to.be.revertedWithCustomError(httpz, "OwnableUnauthorizedAccount")
        .withArgs(user.address);

      // Check that only the owner can update the FHE params
      await expect(httpz.connect(user).updateFheParams(fheParams))
        .to.be.revertedWithCustomError(httpz, "OwnableUnauthorizedAccount")
        .withArgs(user.address);
    });

    it("Should update fheParams", async function () {
      const { httpz, owner } = await loadFixture(deployInitHTTPZFixture);

      // Get dummy FHE params
      const fheParams = getFheParams();

      // Check that FHE params cannot be updated if they are not initialized
      await expect(httpz.connect(owner).updateFheParams(fheParams)).to.be.revertedWithCustomError(
        httpz,
        "FheParamsNotInitialized",
      );

      // Set the FHE params
      const txSetFheParams = await httpz.connect(owner).setFheParams(fheParams);

      // Check event
      await expect(txSetFheParams).to.emit(httpz, "SetFheParams").withArgs([fheParams.dummy]);

      // Check that we can only set the FHE params once
      await expect(httpz.setFheParams(fheParams)).to.be.revertedWithCustomError(httpz, "FheParamsAlreadyInitialized");

      // Update the FHE params
      const txUpdateFheParams = await httpz.connect(owner).updateFheParams(fheParams);

      // Check event
      await expect(txUpdateFheParams).to.emit(httpz, "UpdateFheParams").withArgs([fheParams.dummy]);
    });
  });
});
