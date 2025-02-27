import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { deployHTTPZFixture } from "./utils/deploys";

describe("HTTPZ", function () {
  function getKmsNodes(signers: HardhatEthersSigner[]) {
    // Create dummy KMS nodes with the signers' addresses
    const kmsNodes = signers.map((signer) => ({
      connectorAddress: signer.address,
      identity: hre.ethers.randomBytes(32),
      ipAddress: "127.0.0.1",
      signedNodes: [hre.ethers.randomBytes(32)],
      daAddress: "0x1234567890abcdef1234567890abcdef12345678",
    }));

    return { kmsNodes };
  }

  function getCoprocessors(signers: HardhatEthersSigner[]) {
    // Create dummy Coprocessors with the signers' addresses
    const coprocessors = signers.map((signer) => ({
      connectorAddress: signer.address,
      identity: hre.ethers.randomBytes(32),
      daAddress: "0x1234567890abcdef1234567890abcdef12345678",
    }));

    return { coprocessors };
  }

  async function deployAndDefineInputsFixture() {
    const signers = await hre.ethers.getSigners();
    const [owner, admin, user] = signers.splice(0, 3);

    const HTTPZ = await hre.ethers.getContractFactory("HTTPZ", owner);
    const httpz = await HTTPZ.connect(owner).deploy();

    const protocolMetadata = { name: "Protocol", website: "https://protocol.com" };
    const admins = [admin.address];
    const kmsThreshold = 1;

    const kmsSigners = signers.splice(0, 4);
    const { kmsNodes } = getKmsNodes(kmsSigners);

    const coprocessorSigners = signers.splice(0, 3);
    const { coprocessors } = getCoprocessors(coprocessorSigners);

    return {
      httpz,
      owner,
      admin,
      user,
      protocolMetadata,
      admins,
      kmsThreshold,
      kmsNodes,
      coprocessors,
      kmsSigners,
      coprocessorSigners,
    };
  }

  describe("Initialization", function () {
    it("Should revert because of access controls", async function () {
      const { httpz, user, protocolMetadata, admins, kmsThreshold, kmsNodes, coprocessors } =
        await loadFixture(deployAndDefineInputsFixture);

      // Check that someone else than the owner cannot initialize
      await expect(httpz.connect(user).initialize(protocolMetadata, admins, kmsThreshold, kmsNodes, coprocessors))
        .to.be.revertedWithCustomError(httpz, "OwnableUnauthorizedAccount")
        .withArgs(user.address);
    });

    it("Should revert because of bad KMS threshold", async function () {
      const { httpz, owner, protocolMetadata, admins, kmsNodes, coprocessors } =
        await loadFixture(deployAndDefineInputsFixture);

      // Define a bad KMS threshold
      // The threshold must verify `3t < n`, with `n` the number of KMS nodes
      const badKmsThreshold = kmsNodes.length + 1;

      // Check that the initialization reverts
      await expect(httpz.connect(owner).initialize(protocolMetadata, admins, badKmsThreshold, kmsNodes, coprocessors))
        .to.be.revertedWithCustomError(httpz, "KmsThresholdTooHigh")
        .withArgs(badKmsThreshold, kmsNodes.length);
    });

    it("Should initialize", async function () {
      const { httpz, owner, protocolMetadata, admins, kmsThreshold, kmsNodes, coprocessors } =
        await loadFixture(deployAndDefineInputsFixture);

      // Initialize the contract
      const tx = await httpz.connect(owner).initialize(protocolMetadata, admins, kmsThreshold, kmsNodes, coprocessors);

      // Check event
      await expect(tx).to.emit(httpz, "Initialization");
    });

    it("Should be registered as an admin", async function () {
      const { httpz, admins } = await loadFixture(deployHTTPZFixture);

      // Loop over admins and check if they are properly registered
      for (const admin of admins) {
        await expect(httpz.checkIsAdmin(admin)).to.not.be.reverted;
      }
    });

    it("Should be registered as a KMS node", async function () {
      const { httpz, kmsSigners } = await loadFixture(deployHTTPZFixture);

      // Loop over kmsSigners and check if they are properly registered as KMS nodes
      for (const kmsSigner of kmsSigners) {
        await expect(httpz.checkIsKmsNode(kmsSigner.address)).to.not.be.reverted;
      }
    });

    it("Should be registered as a coprocessor", async function () {
      const { httpz, coprocessorSigners } = await loadFixture(deployHTTPZFixture);

      // Loop over coprocessorSigners and check if they are properly registered as coprocessors
      for (const coprocessorSigner of coprocessorSigners) {
        await expect(httpz.checkIsCoprocessor(coprocessorSigner.address)).to.not.be.reverted;
      }
    });
  });

  describe("Networks", function () {
    async function deployAndDefineNetworkFixture() {
      const { httpz, admins, user } = await loadFixture(deployHTTPZFixture);

      // Define the network to add
      const chainId = 2025;
      const network = {
        chainId: chainId,
        httpzLibrary: "0x1234567890abcdef1234567890abcdef12345678",
        acl: "0xabcdef1234567890abcdef1234567890abcdef12",
        name: "Network",
        website: "https://network.com",
      };
      return { httpz, user, admins, chainId, network };
    }

    it("Should revert because of access controls", async function () {
      const { httpz, user, network } = await loadFixture(deployAndDefineNetworkFixture);

      // Check that someone else than the admin cannot add a network
      await expect(httpz.connect(user).addNetwork(network))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());
    });

    it("Should add a network", async function () {
      const { httpz, admins, chainId, network } = await loadFixture(deployAndDefineNetworkFixture);

      // Add network
      const tx = await httpz.connect(admins[0]).addNetwork(network);

      // Check event
      await expect(tx).to.emit(httpz, "AddNetwork").withArgs(chainId);
    });

    it("Should be registered as a network", async function () {
      const { httpz, chainId } = await loadFixture(deployAndDefineNetworkFixture);

      // Check that the network is registered
      await expect(httpz.checkNetworkIsRegistered(chainId)).to.not.be.reverted;
    });
  });

  describe("KMS", function () {
    it("Should revert because of access controls", async function () {
      const { httpz, user } = await loadFixture(deployHTTPZFixture);

      // Check that someone else than the admin cannot update the KMS threshold
      await expect(httpz.connect(user).updateKmsThreshold(1))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());
    });

    it("Should update the KMS threshold", async function () {
      const { httpz, admins } = await loadFixture(deployHTTPZFixture);

      // Update the KMS threshold
      const newKmsThreshold = 0;
      const tx = await httpz.connect(admins[0]).updateKmsThreshold(newKmsThreshold);

      // Check event
      await expect(tx).to.emit(httpz, "UpdateKmsThreshold").withArgs(newKmsThreshold);

      // Check that the KMS threshold has been updated
      expect(await httpz.kmsThreshold()).to.equal(newKmsThreshold);
    });
  });
});
