import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { deployHTTPZFixture, toValues } from "./utils/";

describe("HTTPZ", function () {
  function getKmsNodes(signers: HardhatEthersSigner[]) {
    // Create dummy KMS nodes with the signers' addresses
    const kmsNodes = signers.map((signer) => ({
      connectorAddress: signer.address,
      identity: hre.ethers.randomBytes(32),
      ipAddress: "127.0.0.1",
      daUrl: "https://da.com",
    }));

    return { kmsNodes };
  }

  function getCoprocessors(signers: HardhatEthersSigner[]) {
    // Create dummy Coprocessors with the signers' addresses
    const coprocessors = signers.map((signer) => ({
      transactionSenderAddress: signer.address,
      identity: hre.ethers.randomBytes(32),
      daUrl: "https://da.com",
      s3BucketUrl: "s3://bucket",
    }));

    return { coprocessors };
  }

  function getNetworks(chainIds: number[]) {
    // Create dummy Networks with the chain IDs
    const networks = chainIds.map((chainId) => ({
      chainId: chainId,
      httpzExecutor: hre.ethers.getAddress("0x1234567890AbcdEF1234567890aBcdef12345678"),
      aclAddress: hre.ethers.getAddress("0xabcdef1234567890abcdef1234567890abcdef12"),
      name: "Network",
      website: "https://network.com",
    }));

    return { networks };
  }

  async function getInputsForDeployFixture() {
    const signers = await hre.ethers.getSigners();
    const [owner, admin, user] = signers.splice(0, 3);

    const HTTPZ = await hre.ethers.getContractFactory("HTTPZ", owner);

    const protocolMetadata = { name: "Protocol", website: "https://protocol.com" };
    const admins = [admin.address];
    const kmsThreshold = 1;

    const kmsSigners = signers.splice(0, 4);
    const { kmsNodes } = getKmsNodes(kmsSigners);

    const coprocessorSigners = signers.splice(0, 3);
    const { coprocessors } = getCoprocessors(coprocessorSigners);

    const chainIds = [2025, 2026, 2027, 2028];
    const { networks } = getNetworks(chainIds);

    return {
      HTTPZ,
      owner,
      admin,
      user,
      protocolMetadata,
      admins,
      kmsThreshold,
      kmsNodes,
      coprocessors,
      networks,
      kmsSigners,
      coprocessorSigners,
    };
  }

  describe("Deployment", function () {
    it("Should revert because of bad KMS threshold", async function () {
      const { HTTPZ, owner, protocolMetadata, admins, kmsNodes, coprocessors, networks } =
        await loadFixture(getInputsForDeployFixture);

      // Define a bad KMS threshold
      // The threshold must verify `3t < n`, with `n` the number of KMS nodes
      const badKmsThreshold = kmsNodes.length + 1;

      // Check that the initialization reverts
      await expect(
        HTTPZ.connect(owner).deploy(protocolMetadata, admins, badKmsThreshold, kmsNodes, coprocessors, networks),
      )
        .to.be.revertedWithCustomError(HTTPZ, "KmsThresholdTooHigh")
        .withArgs(badKmsThreshold, kmsNodes.length);
    });

    it("Should deploy", async function () {
      const { HTTPZ, owner, protocolMetadata, admins, kmsThreshold, kmsNodes, coprocessors, networks } =
        await loadFixture(getInputsForDeployFixture);

      // Initialize the contract
      const httpz = await HTTPZ.connect(owner).deploy(
        protocolMetadata,
        admins,
        kmsThreshold,
        kmsNodes,
        coprocessors,
        networks,
      );

      // Check event
      await expect(httpz.deploymentTransaction())
        .to.emit(httpz, "Initialization")
        .withArgs(
          toValues(protocolMetadata),
          toValues(admins),
          kmsThreshold,
          toValues(kmsNodes),
          toValues(coprocessors),
          toValues(networks),
        );
    });

    it("Should be registered as an admin", async function () {
      const { httpz, admins } = await loadFixture(deployHTTPZFixture);

      // Loop over admins and check if they are properly registered
      for (const admin of admins) {
        await expect(httpz.checkIsAdmin(admin)).to.not.be.reverted;
      }
    });

    it("Should be registered as KMS nodes", async function () {
      const { httpz, kmsSigners } = await loadFixture(deployHTTPZFixture);

      // Loop over kmsSigners and check if they are properly registered as KMS nodes
      for (const kmsSigner of kmsSigners) {
        await expect(httpz.checkIsKmsNode(kmsSigner.address)).to.not.be.reverted;
      }
    });

    it("Should be registered as coprocessors", async function () {
      const { httpz, coprocessorSigners } = await loadFixture(deployHTTPZFixture);

      // Loop over coprocessorSigners and check if they are properly registered as coprocessors
      for (const coprocessorSigner of coprocessorSigners) {
        await expect(httpz.checkIsCoprocessor(coprocessorSigner.address)).to.not.be.reverted;
      }
    });

    it("Should be registered as networks", async function () {
      const { httpz, chainIds } = await loadFixture(deployHTTPZFixture);

      // Loop over chain IDs and check if they are properly registered as networks
      for (const chainId of chainIds) {
        await expect(httpz.checkNetworkIsRegistered(chainId)).to.not.be.reverted;
      }
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
