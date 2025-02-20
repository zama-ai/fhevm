import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

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
});
