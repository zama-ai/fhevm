import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre, { ethers } from "hardhat";

import { UINT64_MAX, createRandomWallet, loadTestVariablesFixture, toValues } from "./utils";

describe("GatewayConfig", function () {
  // Define fake values
  const fakeOwner = createRandomWallet();

  async function getInputsForDeployFixture() {
    const signers = await hre.ethers.getSigners();
    const [owner, pauser, user] = signers.splice(0, 3);

    const gatewayConfig = await hre.ethers.getContractFactory("GatewayConfig", owner);

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
      gatewayConfig,
      owner,
      pauser,
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
    it("Should deploy", async function () {
      const { gatewayConfig, protocolMetadata, owner, pauser, kmsThreshold, kmsNodes, coprocessors } =
        await loadFixture(getInputsForDeployFixture);
      const proxyContract = await deployEmptyProxy(owner);

      const upgradeTx = await hre.upgrades.upgradeProxy(proxyContract, gatewayConfig, {
        call: {
          fn: "initialize",
          args: [pauser.address, protocolMetadata, kmsThreshold, kmsNodes, coprocessors],
        },
      });

      /**
       * Extract event args and convert to strings. This is needed as the "upgradeProxy()" method above
       * returns an GatewayConfig instance instead of a ContractTransactionResponse, so the expect() function ç
       * from chaijs fails on the evaluation of the transaction events.
       */
      const initializationEvents = await upgradeTx.queryFilter(upgradeTx.filters.Initialization);
      const stringifiedEventArgs = initializationEvents[0].args.map((arg) => arg.toString());

      // It should emit one event containing the initialization parameters
      expect(initializationEvents.length).to.equal(1);
      expect(stringifiedEventArgs).to.deep.equal([
        pauser.address,
        toValues(protocolMetadata).toString(),
        kmsThreshold,
        toValues(kmsNodes).toString(),
        toValues(coprocessors).toString(),
      ]);
    });

    it("Should revert because of bad KMS threshold", async function () {
      const { gatewayConfig, protocolMetadata, owner, pauser, kmsNodes, coprocessors } =
        await loadFixture(getInputsForDeployFixture);
      const proxyContract = await deployEmptyProxy(owner);

      // Define a bad KMS threshold
      // The threshold must verify `t <= n`, with `n` the number of KMS nodes
      const badKmsThreshold = kmsNodes.length + 1;

      // Check that the initialization reverts
      const upgradeTx = hre.upgrades.upgradeProxy(proxyContract, gatewayConfig, {
        call: {
          fn: "initialize",
          args: [pauser.address, protocolMetadata, badKmsThreshold, kmsNodes, coprocessors],
        },
      });
      await expect(upgradeTx)
        .to.be.revertedWithCustomError(gatewayConfig, "KmsThresholdTooHigh")
        .withArgs(badKmsThreshold, kmsNodes.length);
    });

    it("Should be registered as an pauser", async function () {
      const { gatewayConfig, pauser } = await loadFixture(loadTestVariablesFixture);

      // Check if the pauser is properly registered
      await expect(gatewayConfig.checkIsPauser(pauser)).to.not.be.reverted;
    });

    it("Should be registered as KMS nodes transaction senders", async function () {
      const { gatewayConfig, kmsTxSenders } = await loadFixture(loadTestVariablesFixture);

      // Loop over kmsTxSenders and check if they are properly registered as KMS transaction senders
      for (const kmsTxSender of kmsTxSenders) {
        await expect(gatewayConfig.checkIsKmsTxSender(kmsTxSender.address)).to.not.be.reverted;
      }
    });

    it("Should be registered as KMS nodes signers", async function () {
      const { gatewayConfig, kmsSigners } = await loadFixture(loadTestVariablesFixture);

      // Loop over kmsSigners and check if they are properly registered as KMS signers
      for (const kmsSigner of kmsSigners) {
        await expect(gatewayConfig.checkIsKmsSigner(kmsSigner.address)).to.not.be.reverted;
      }
    });

    it("Should be registered as coprocessors transaction senders", async function () {
      const { gatewayConfig, coprocessorTxSenders } = await loadFixture(loadTestVariablesFixture);

      // Loop over coprocessorTxSenders and check if they are properly registered as coprocessor transaction senders
      for (const coprocessorTxSender of coprocessorTxSenders) {
        await expect(gatewayConfig.checkIsCoprocessorTxSender(coprocessorTxSender.address)).to.not.be.reverted;
      }
    });

    it("Should be registered as coprocessors signers", async function () {
      const { gatewayConfig, coprocessorSigners } = await loadFixture(loadTestVariablesFixture);

      // Loop over coprocessorSigners and check if they are properly registered as coprocessor signers
      for (const coprocessorSigner of coprocessorSigners) {
        await expect(gatewayConfig.checkIsCoprocessorSigner(coprocessorSigner.address)).to.not.be.reverted;
      }
    });

    it("Should be registered as networks", async function () {
      const { gatewayConfig, chainIds } = await loadFixture(loadTestVariablesFixture);

      // Loop over chain IDs and check if they are properly registered as networks
      for (const chainId of chainIds) {
        await expect(gatewayConfig.checkNetworkIsRegistered(chainId)).to.not.be.reverted;
      }
    });

    it("Should get all KMS node transaction sender addresses", async function () {
      const { gatewayConfig, kmsTxSenders } = await loadFixture(loadTestVariablesFixture);

      // Get all KMS node transaction sender addresses
      const kmsTxSenderAddresses = await gatewayConfig.getKmsTxSenders();

      // Check that the number of KMS node transaction sender addresses is correct
      expect(kmsTxSenderAddresses.length).to.equal(kmsTxSenders.length);

      // Check that all KMS node transaction sender addresses are in the list
      for (const kmsTxSender of kmsTxSenders) {
        expect(kmsTxSenderAddresses).to.include(kmsTxSender.address);
      }
    });

    it("Should get all coprocessor transaction sender addresses", async function () {
      const { gatewayConfig, coprocessorTxSenders } = await loadFixture(loadTestVariablesFixture);

      // Get all coprocessor transaction sender addresses
      const coprocessorTxSenderAddresses = await gatewayConfig.getCoprocessorTxSenders();

      // Check that the number of coprocessor transaction sender addresses is correct
      expect(coprocessorTxSenderAddresses.length).to.equal(coprocessorTxSenders.length);

      // Check that all coprocessor transaction sender addresses are in the list
      for (const coprocessorTxSender of coprocessorTxSenders) {
        expect(coprocessorTxSenderAddresses).to.include(coprocessorTxSender.address);
      }
    });

    it("Should get all KMS node signer addresses", async function () {
      const { gatewayConfig, kmsSigners } = await loadFixture(loadTestVariablesFixture);

      // Get all KMS node signer addresses
      const kmsSignerAddresses = await gatewayConfig.getKmsSigners();

      // Check that the number of KMS node signer addresses is correct
      expect(kmsSignerAddresses.length).to.equal(kmsSigners.length);

      // Check that all KMS node signer addresses are in the list
      for (const kmsSigner of kmsSigners) {
        expect(kmsSignerAddresses).to.include(kmsSigner.address);
      }
    });

    it("Should get all coprocessor signer addresses", async function () {
      const { gatewayConfig, coprocessorSigners } = await loadFixture(loadTestVariablesFixture);

      // Get all coprocessor signer addresses
      const coprocessorSignerAddresses = await gatewayConfig.getCoprocessorSigners();

      // Check that the number of coprocessor signer addresses is correct
      expect(coprocessorSignerAddresses.length).to.equal(coprocessorSigners.length);

      // Check that all coprocessor signer addresses are in the list
      for (const coprocessorTxSender of coprocessorSigners) {
        expect(coprocessorSignerAddresses).to.include(coprocessorTxSender.address);
      }
    });

    it("Should get all networks' metadata", async function () {
      const { gatewayConfig, chainIds } = await loadFixture(loadTestVariablesFixture);

      // Get all coprocessor signer addresses
      const networks = await gatewayConfig.getNetworks();

      // Check that the number of coprocessor signer addresses is correct
      expect(networks.length).to.equal(chainIds.length);

      // Check that all coprocessor signer addresses are in the list
      for (const network of networks) {
        expect(chainIds).to.include(Number(network.chainId));
      }
    });
  });

  describe("Pauser", function () {
    it("Should revert because of access controls", async function () {
      const { gatewayConfig } = await loadFixture(loadTestVariablesFixture);

      // Check that someone else than the owner cannot update the pauser
      await expect(gatewayConfig.connect(fakeOwner).updatePauser(fakeOwner.address))
        .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
        .withArgs(fakeOwner.address);
    });

    it("Should update the pauser", async function () {
      const { gatewayConfig, owner } = await loadFixture(loadTestVariablesFixture);

      const newPauser = createRandomWallet();
      // Update the pauser
      const tx = await gatewayConfig.connect(owner).updatePauser(newPauser.address);

      // Check the event
      await expect(tx).to.emit(gatewayConfig, "UpdatePauser").withArgs(newPauser.address);
    });

    it("Should revert because the pauser is the null address", async function () {
      const { gatewayConfig, owner } = await loadFixture(loadTestVariablesFixture);

      // Check that updating with the null address reverts
      await expect(gatewayConfig.connect(owner).updatePauser(hre.ethers.ZeroAddress)).to.be.revertedWithCustomError(
        gatewayConfig,
        "InvalidNullPauser",
      );
    });
  });

  describe("KMS threshold", function () {
    it("Should revert because of access controls", async function () {
      const { gatewayConfig } = await loadFixture(loadTestVariablesFixture);

      // Check that someone else than the owner cannot update the KMS threshold
      await expect(gatewayConfig.connect(fakeOwner).updateKmsThreshold(1))
        .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
        .withArgs(fakeOwner.address);
    });

    it("Should update the KMS threshold", async function () {
      const { gatewayConfig, owner } = await loadFixture(loadTestVariablesFixture);

      // Update the KMS threshold
      const newKmsThreshold = 0;
      const tx = await gatewayConfig.connect(owner).updateKmsThreshold(newKmsThreshold);

      // Check event
      await expect(tx).to.emit(gatewayConfig, "UpdateKmsThreshold").withArgs(newKmsThreshold);

      // Check that the KMS threshold has been updated
      expect(await gatewayConfig.getKmsThreshold()).to.equal(newKmsThreshold);
    });

    it("Should revert because the KMS threshold is too high", async function () {
      const { gatewayConfig, owner, kmsSigners } = await loadFixture(loadTestVariablesFixture);

      // Define a KMS threshold that is too high (greater than the number of KMS nodes)
      const badKmsThreshold = kmsSigners.length + 1;

      // Check that updating with a KMS threshold that is too high reverts
      await expect(gatewayConfig.connect(owner).updateKmsThreshold(badKmsThreshold))
        .to.be.revertedWithCustomError(gatewayConfig, "KmsThresholdTooHigh")
        .withArgs(badKmsThreshold, kmsSigners.length);
    });
  });

  describe("Add network", function () {
    it("Should add a new network metadata", async function () {
      const { gatewayConfig, owner, chainIds } = await loadFixture(loadTestVariablesFixture);

      // Define a new chainId that is not already registered (since the GatewayConfig contract has already
      // been deployed and networks have been registered)
      const newChainId = chainIds[0] - 1;

      const newNetwork = {
        chainId: newChainId,
        fhevmExecutor: hre.ethers.getAddress("0x1234567890AbcdEF1234567890aBcdef12345678"),
        aclAddress: hre.ethers.getAddress("0xabcdef1234567890abcdef1234567890abcdef12"),
        name: "Network",
        website: "https://network.com",
      };

      const txResponse = gatewayConfig.connect(owner).addNetwork(newNetwork);

      // Check AddNetwork event has been emitted
      await expect(txResponse).to.emit(gatewayConfig, "AddNetwork").withArgs(toValues(newNetwork));
    });

    it("Should revert because the network's chainId is null", async function () {
      const { gatewayConfig, owner } = await loadFixture(loadTestVariablesFixture);

      const fakeNetwork = {
        chainId: 0,
        fhevmExecutor: hre.ethers.getAddress("0x1234567890AbcdEF1234567890aBcdef12345678"),
        aclAddress: hre.ethers.getAddress("0xabcdef1234567890abcdef1234567890abcdef12"),
        name: "Network",
        website: "https://network.com",
      };

      // Check that registering a network with a null chainId reverts
      await expect(gatewayConfig.connect(owner).addNetwork(fakeNetwork)).to.revertedWithCustomError(
        gatewayConfig,
        "InvalidNullChainId",
      );
    });

    it("Should revert because the network's chainId is not representable by a uint64", async function () {
      const { gatewayConfig, owner } = await loadFixture(loadTestVariablesFixture);

      // Define a chainId that is not representable by a uint64
      const chainIdTooLarge = UINT64_MAX + 1n;

      const fakeNetwork = {
        chainId: chainIdTooLarge,
        fhevmExecutor: hre.ethers.getAddress("0x1234567890AbcdEF1234567890aBcdef12345678"),
        aclAddress: hre.ethers.getAddress("0xabcdef1234567890abcdef1234567890abcdef12"),
        name: "Network",
        website: "https://network.com",
      };

      // Check that registering a network with a chainId that is not representable by a uint64 reverts
      await expect(gatewayConfig.connect(owner).addNetwork(fakeNetwork))
        .to.revertedWithCustomError(gatewayConfig, "ChainIdNotUint64")
        .withArgs(chainIdTooLarge);
    });

    it("Should revert because a network with the same chainId already exists", async function () {
      const { gatewayConfig, owner, chainIds } = await loadFixture(loadTestVariablesFixture);

      // Get a chainId that has been registered
      const alreadyAddedChainId = chainIds[0];

      const fakeNetwork = {
        chainId: alreadyAddedChainId,
        fhevmExecutor: hre.ethers.getAddress("0x1234567890AbcdEF1234567890aBcdef12345678"),
        aclAddress: hre.ethers.getAddress("0xabcdef1234567890abcdef1234567890abcdef12"),
        name: "Network",
        website: "https://network.com",
      };

      // Check that registering a network whose chainId is already registered reverts
      await expect(gatewayConfig.connect(owner).addNetwork(fakeNetwork))
        .to.revertedWithCustomError(gatewayConfig, "NetworkAlreadyRegistered")
        .withArgs(alreadyAddedChainId);
    });
  });
});
