import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre, { ethers } from "hardhat";

import { loadTestVariablesFixture, toValues } from "./utils/";

describe("HTTPZ", function () {
  async function getInputsForDeployFixture() {
    const signers = await hre.ethers.getSigners();
    const [owner, admin, user] = signers.splice(0, 3);

    const HTTPZ = await hre.ethers.getContractFactory("HTTPZ", owner);

    const protocolMetadata = { name: "Protocol", website: "https://protocol.com" };
    const kmsThreshold = 1;

    const nKmsNodes = 4;
    const kmsTxSenders = signers.splice(0, nKmsNodes);
    const kmsSigners = signers.splice(0, nKmsNodes);

    // Create dummy KMS nodes with the tx sender and signer addresses
    const kmsNodes = [];
    for (let i = 0; i < nKmsNodes; i++) {
      kmsNodes.push({
        txSenderAddress: kmsTxSenders[i].address,
        signerAddress: kmsSigners[i].address,
        ipAddress: `127.0.0.${i}`,
      });
    }

    const nCoprocessors = 3;
    const coprocessorTxSenders = signers.splice(0, nCoprocessors);
    const coprocessorSigners = signers.splice(0, nCoprocessors);

    // Create dummy Coprocessors with the tx sender and signer addresses
    const coprocessors = [];
    for (let i = 0; i < nCoprocessors; i++) {
      coprocessors.push({
        txSenderAddress: coprocessorTxSenders[i].address,
        signerAddress: coprocessorSigners[i].address,
        s3BucketUrl: `s3://bucket-${i}`,
      });
    }

    return {
      HTTPZ,
      owner,
      admin,
      user,
      protocolMetadata,
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
      const { HTTPZ, protocolMetadata, owner, admin, kmsNodes, coprocessors } =
        await loadFixture(getInputsForDeployFixture);
      const proxyContract = await deployEmptyProxy(owner);

      // Define a bad KMS threshold
      // The threshold must verify `t <= n`, with `n` the number of KMS nodes
      const badKmsThreshold = kmsNodes.length + 1;

      // Check that the initialization reverts
      const upgradeTx = hre.upgrades.upgradeProxy(proxyContract, HTTPZ, {
        call: {
          fn: "initialize",
          args: [protocolMetadata, admin.address, badKmsThreshold, kmsNodes, coprocessors],
        },
      });
      await expect(upgradeTx)
        .to.be.revertedWithCustomError(HTTPZ, "KmsThresholdTooHigh")
        .withArgs(badKmsThreshold, kmsNodes.length);
    });

    it("Should deploy", async function () {
      const { HTTPZ, protocolMetadata, owner, admin, kmsThreshold, kmsNodes, coprocessors } =
        await loadFixture(getInputsForDeployFixture);
      const proxyContract = await deployEmptyProxy(owner);

      const upgradeTx = await hre.upgrades.upgradeProxy(proxyContract, HTTPZ, {
        call: {
          fn: "initialize",
          args: [protocolMetadata, admin.address, kmsThreshold, kmsNodes, coprocessors],
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
        admin.address,
        kmsThreshold,
        toValues(kmsNodes).toString(),
        toValues(coprocessors).toString(),
      ]);
    });

    it("Should be registered as an admin", async function () {
      const { httpz, admin } = await loadFixture(loadTestVariablesFixture);

      // Check if the admin is properly registered
      await expect(httpz.checkIsAdmin(admin)).to.not.be.reverted;
    });

    it("Should be registered as KMS nodes transaction senders", async function () {
      const { httpz, kmsTxSenders } = await loadFixture(loadTestVariablesFixture);

      // Loop over kmsTxSenders and check if they are properly registered as KMS transaction senders
      for (const kmsTxSender of kmsTxSenders) {
        await expect(httpz.checkIsKmsTxSender(kmsTxSender.address)).to.not.be.reverted;
      }
    });

    it("Should be registered as KMS nodes signers", async function () {
      const { httpz, kmsSigners } = await loadFixture(loadTestVariablesFixture);

      // Loop over kmsSigners and check if they are properly registered as KMS signers
      for (const kmsSigner of kmsSigners) {
        await expect(httpz.checkIsKmsSigner(kmsSigner.address)).to.not.be.reverted;
      }
    });

    it("Should be registered as coprocessors transaction senders", async function () {
      const { httpz, coprocessorTxSenders } = await loadFixture(loadTestVariablesFixture);

      // Loop over coprocessorTxSenders and check if they are properly registered as coprocessor transaction senders
      for (const coprocessorTxSender of coprocessorTxSenders) {
        await expect(httpz.checkIsCoprocessorTxSender(coprocessorTxSender.address)).to.not.be.reverted;
      }
    });

    it("Should be registered as coprocessors signers", async function () {
      const { httpz, coprocessorSigners } = await loadFixture(loadTestVariablesFixture);

      // Loop over coprocessorSigners and check if they are properly registered as coprocessor signers
      for (const coprocessorSigner of coprocessorSigners) {
        await expect(httpz.checkIsCoprocessorSigner(coprocessorSigner.address)).to.not.be.reverted;
      }
    });

    it("Should be registered as networks", async function () {
      const { httpz, chainIds } = await loadFixture(loadTestVariablesFixture);

      // Loop over chain IDs and check if they are properly registered as networks
      for (const chainId of chainIds) {
        await expect(httpz.checkNetworkIsRegistered(chainId)).to.not.be.reverted;
      }
    });

    it("Should get all KMS node transaction sender addresses", async function () {
      const { httpz, kmsTxSenders } = await loadFixture(loadTestVariablesFixture);

      // Get all KMS node transaction sender addresses
      const kmsTxSenderAddresses = await httpz.getAllKmsTxSenderAddresses();

      // Check that the number of KMS node transaction sender addresses is correct
      expect(kmsTxSenderAddresses.length).to.equal(kmsTxSenders.length);

      // Check that all KMS node addresses are in the list
      for (const kmsTxSender of kmsTxSenders) {
        expect(kmsTxSenderAddresses).to.include(kmsTxSender.address);
      }
    });

    it("Should get all coprocessor transaction sender addresses", async function () {
      const { httpz, coprocessorTxSenders } = await loadFixture(loadTestVariablesFixture);

      // Get all coprocessor transaction sender addresses
      const coprocessorTxSenderAddresses = await httpz.getAllCoprocessorTxSenderAddresses();

      // Check that the number of coprocessor transaction sender addresses is correct
      expect(coprocessorTxSenderAddresses.length).to.equal(coprocessorTxSenders.length);

      // Check that all coprocessor transaction sender addresses are in the list
      for (const coprocessorTxSender of coprocessorTxSenders) {
        expect(coprocessorTxSenderAddresses).to.include(coprocessorTxSender.address);
      }
    });
  });

  describe("KMS", function () {
    it("Should revert because of access controls", async function () {
      const { httpz, user } = await loadFixture(loadTestVariablesFixture);

      // Check that someone else than the admin cannot update the KMS threshold
      await expect(httpz.connect(user).updateKmsThreshold(1))
        .to.be.revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.ADMIN_ROLE());
    });

    it("Should update the KMS threshold", async function () {
      const { httpz, admin } = await loadFixture(loadTestVariablesFixture);

      // Update the KMS threshold
      const newKmsThreshold = 0;
      const tx = await httpz.connect(admin).updateKmsThreshold(newKmsThreshold);

      // Check event
      await expect(tx).to.emit(httpz, "UpdateKmsThreshold").withArgs(newKmsThreshold);

      // Check that the KMS threshold has been updated
      expect(await httpz.getKmsThreshold()).to.equal(newKmsThreshold);
    });

    it("Should revert because the KMS threshold is too high", async function () {
      const { httpz, admin, kmsSigners } = await loadFixture(loadTestVariablesFixture);

      // Define a KMS threshold that is too high (greater than the number of KMS nodes)
      const badKmsThreshold = kmsSigners.length + 1;

      // Check that updating with a KMS threshold that is too high reverts
      await expect(httpz.connect(admin).updateKmsThreshold(badKmsThreshold))
        .to.be.revertedWithCustomError(httpz, "KmsThresholdTooHigh")
        .withArgs(badKmsThreshold, kmsSigners.length);
    });
  });

  describe("Add network", function () {
    it("Should add a new network metadata", async function () {
      const { httpz, admin } = await loadFixture(loadTestVariablesFixture);

      const newNetwork = {
        chainId: hre.ethers.toNumber(hre.ethers.randomBytes(2)),
        httpzExecutor: hre.ethers.getAddress("0x1234567890AbcdEF1234567890aBcdef12345678"),
        aclAddress: hre.ethers.getAddress("0xabcdef1234567890abcdef1234567890abcdef12"),
        name: "Network",
        website: "https://network.com",
      };

      const txResponse = httpz.connect(admin).addNetwork(newNetwork);

      // Check AddNetwork event has been emitted
      await expect(txResponse).to.emit(httpz, "AddNetwork").withArgs(toValues(newNetwork));
    });

    it("Should revert because the network's chainId is null", async function () {
      const { httpz, admin } = await loadFixture(loadTestVariablesFixture);

      const txResponse = httpz.connect(admin).addNetwork({
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
      const { httpz, admin } = await loadFixture(loadTestVariablesFixture);

      const alreadyAddedNetwork = await httpz.networks(0);

      const txResponse = httpz.connect(admin).addNetwork({
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
