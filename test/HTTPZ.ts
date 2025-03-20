import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre, { ethers } from "hardhat";

import { deployHTTPZFixture, toValues } from "./utils/";

describe("HTTPZ", function () {
  function getKmsNodes(signers: HardhatEthersSigner[]) {
    // Create dummy KMS nodes with the signers' addresses
    const kmsNodes = signers.map((signer) => ({
      connectorAddress: signer.address,
      identity: hre.ethers.hexlify(hre.ethers.randomBytes(32)),
      ipAddress: "127.0.0.1",
      daUrl: "https://da.com",
    }));

    return { kmsNodes };
  }

  function getCoprocessors(signers: HardhatEthersSigner[]) {
    // Create dummy Coprocessors with the signers' addresses
    const coprocessors = signers.map((signer) => ({
      transactionSenderAddress: signer.address,
      identity: hre.ethers.hexlify(hre.ethers.randomBytes(32)),
      daUrl: "https://da.com",
      s3BucketUrl: "s3://bucket",
    }));

    return { coprocessors };
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
      kmsSigners,
      coprocessorSigners,
    };
  }

  async function deployEmptyProxy(owner: HardhatEthersSigner) {
    const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", owner);
    const proxyContract = await hre.upgrades.deployProxy(proxyImplementation, [owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    await proxyContract.waitForDeployment();
    return proxyContract;
  }

  describe("Deployment", function () {
    it("Should revert because of bad KMS threshold", async function () {
      const { HTTPZ, protocolMetadata, owner, admins, kmsNodes, coprocessors } =
        await loadFixture(getInputsForDeployFixture);
      const proxyContract = await deployEmptyProxy(owner);

      // Define a bad KMS threshold
      // The threshold must verify `3t < n`, with `n` the number of KMS nodes
      const badKmsThreshold = kmsNodes.length + 1;

      // Check that the initialization reverts
      const upgradeTx = hre.upgrades.upgradeProxy(proxyContract, HTTPZ, {
        call: {
          fn: "initialize",
          args: [protocolMetadata, admins, badKmsThreshold, kmsNodes, coprocessors],
        },
      });
      await expect(upgradeTx)
        .to.be.revertedWithCustomError(HTTPZ, "KmsThresholdTooHigh")
        .withArgs(badKmsThreshold, kmsNodes.length);
    });

    it("Should deploy", async function () {
      const { HTTPZ, protocolMetadata, owner, admins, kmsThreshold, kmsNodes, coprocessors } =
        await loadFixture(getInputsForDeployFixture);
      const proxyContract = await deployEmptyProxy(owner);

      const upgradeTx = await hre.upgrades.upgradeProxy(proxyContract, HTTPZ, {
        call: {
          fn: "initialize",
          args: [protocolMetadata, admins, kmsThreshold, kmsNodes, coprocessors],
        },
      });

      /**
       * Extract event args and convert to strings. This is needed as the "upgradeProxy()" method above
       * returns an HTTPZ instance instead of a ContractTransactionResponse, so the expect() function ç
       * from chaijs fails on the evaluation of the transaction events.
       */
      const initializationEvents = await upgradeTx.queryFilter(upgradeTx.filters.Initialization);
      const stringifiedEventArgs = initializationEvents[0].args.map((arg) => arg.toString());

      // It should emit one event containing the initialization parameters
      expect(initializationEvents.length).to.equal(1);
      expect(stringifiedEventArgs).to.deep.equal([
        toValues(protocolMetadata).toString(),
        admins.toString(),
        kmsThreshold,
        toValues(kmsNodes).toString(),
        toValues(coprocessors).toString(),
      ]);
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

    it("Should get all KMS node addresses", async function () {
      const { httpz, kmsSigners } = await loadFixture(deployHTTPZFixture);

      // Get all KMS node addresses
      const kmsNodeAddresses = await httpz.getAllKmsNodeAddresses();

      // Check that the number of KMS node addresses is correct
      expect(kmsNodeAddresses.length).to.equal(kmsSigners.length);

      // Check that all KMS node addresses are in the list
      for (const kmsSigner of kmsSigners) {
        expect(kmsNodeAddresses).to.include(kmsSigner.address);
      }
    });

    it("Should get all coprocessor addresses", async function () {
      const { httpz, coprocessorSigners } = await loadFixture(deployHTTPZFixture);

      // Get all coprocessor addresses
      const coprocessorAddresses = await httpz.getAllCoprocessorAddresses();

      // Check that the number of coprocessor addresses is correct
      expect(coprocessorAddresses.length).to.equal(coprocessorSigners.length);

      // Check that all coprocessor addresses are in the list
      for (const coprocessorSigner of coprocessorSigners) {
        expect(coprocessorAddresses).to.include(coprocessorSigner.address);
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
      expect(await httpz.getKmsThreshold()).to.equal(newKmsThreshold);
    });
  });

  describe("Add network", function () {
    it("Should add a new network metadata", async function () {
      const { httpz, admins } = await loadFixture(deployHTTPZFixture);

      const newNetwork = {
        chainId: hre.ethers.toNumber(hre.ethers.randomBytes(2)),
        httpzExecutor: hre.ethers.getAddress("0x1234567890AbcdEF1234567890aBcdef12345678"),
        aclAddress: hre.ethers.getAddress("0xabcdef1234567890abcdef1234567890abcdef12"),
        name: "Network",
        website: "https://network.com",
      };

      const txResponse = httpz.connect(admins[0]).addNetwork(newNetwork);

      // Check AddNetwork event has been emitted
      await expect(txResponse).to.emit(httpz, "AddNetwork").withArgs(toValues(newNetwork));
    });

    it("Should revert because the network's chainId is null", async function () {
      const { httpz, admins } = await loadFixture(deployHTTPZFixture);

      const txResponse = httpz.connect(admins[0]).addNetwork({
        chainId: 0,
        httpzExecutor: hre.ethers.getAddress("0x1234567890AbcdEF1234567890aBcdef12345678"),
        aclAddress: hre.ethers.getAddress("0xabcdef1234567890abcdef1234567890abcdef12"),
        name: "Network",
        website: "https://network.com",
      });

      // Check AddNetwork event has been emitted
      await expect(txResponse).to.revertedWithCustomError(httpz, "InvalidNullChainId");
    });

    it("Should revert because a network with the same chainId already exists", async function () {
      const { httpz, admins } = await loadFixture(deployHTTPZFixture);

      const alreadyAddedNetwork = await httpz.networks(0);

      const txResponse = httpz.connect(admins[0]).addNetwork({
        chainId: alreadyAddedNetwork.chainId,
        httpzExecutor: hre.ethers.getAddress("0x1234567890AbcdEF1234567890aBcdef12345678"),
        aclAddress: hre.ethers.getAddress("0xabcdef1234567890abcdef1234567890abcdef12"),
        name: "Network",
        website: "https://network.com",
      });

      // Check AddNetwork event has been emitted
      await expect(txResponse)
        .to.revertedWithCustomError(httpz, "NetworkAlreadyRegistered")
        .withArgs(alreadyAddedNetwork.chainId);
    });
  });
});
