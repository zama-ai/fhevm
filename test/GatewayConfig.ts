import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { ContractFactory, EventLog, Wallet } from "ethers";
import hre from "hardhat";

import { EmptyUUPSProxy, GatewayConfig } from "../typechain-types";
// The type needs to be imported separately because it is not properly detected by the linter
// as this type is defined as a shared structs instead of directly in the IDecryption interface
import { CoprocessorStruct, KmsNodeStruct } from "../typechain-types/contracts/interfaces/IGatewayConfig";
import { UINT64_MAX, createRandomWallet, loadChainIds, loadTestVariablesFixture, toValues } from "./utils";

describe("GatewayConfig", function () {
  // Get the registered host chainId(s)
  const hostChainIds = loadChainIds();

  // Define input values
  const protocolMetadata = { name: "Protocol", website: "https://protocol.com" };
  const kmsThreshold = 1;

  // Define fake values
  const fakeOwner = createRandomWallet();

  let gatewayConfig: GatewayConfig;
  let owner: Wallet;
  let pauser: HardhatEthersSigner;
  let nKmsNodes: number;
  let kmsNodes: KmsNodeStruct[];
  let kmsTxSenders: HardhatEthersSigner[];
  let kmsSigners: HardhatEthersSigner[];
  let coprocessors: CoprocessorStruct[];
  let coprocessorTxSenders: HardhatEthersSigner[];
  let coprocessorSigners: HardhatEthersSigner[];

  async function getInputsForDeployFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const { kmsTxSenders, kmsSigners, nKmsNodes, coprocessorTxSenders, coprocessorSigners, nCoprocessors } =
      fixtureData;

    // Create KMS nodes with the tx sender and signer addresses
    kmsNodes = [];
    for (let i = 0; i < nKmsNodes; i++) {
      kmsNodes.push({
        txSenderAddress: kmsTxSenders[i].address,
        signerAddress: kmsSigners[i].address,
        ipAddress: `127.0.0.${i}`,
      });
    }

    // Create coprocessors with the tx sender and signer addresses
    coprocessors = [];
    for (let i = 0; i < nCoprocessors; i++) {
      coprocessors.push({
        txSenderAddress: coprocessorTxSenders[i].address,
        signerAddress: coprocessorSigners[i].address,
        s3BucketUrl: `s3://bucket-${i}`,
      });
    }

    return fixtureData;
  }

  before(async function () {
    // Initialize globally used variables before each test
    const fixtureData = await loadFixture(getInputsForDeployFixture);
    gatewayConfig = fixtureData.gatewayConfig;
    owner = fixtureData.owner;
    pauser = fixtureData.pauser;
    nKmsNodes = fixtureData.nKmsNodes;
    kmsTxSenders = fixtureData.kmsTxSenders;
    kmsSigners = fixtureData.kmsSigners;
    coprocessorTxSenders = fixtureData.coprocessorTxSenders;
    coprocessorSigners = fixtureData.coprocessorSigners;
  });

  describe("Deployment", function () {
    let proxyContract: EmptyUUPSProxy;
    let newGatewayConfigFactory: ContractFactory;

    beforeEach(async function () {
      // Deploy a new proxy contract
      const proxyImplementation = await hre.ethers.getContractFactory("EmptyUUPSProxy", owner);
      proxyContract = await hre.upgrades.deployProxy(proxyImplementation, [owner.address], {
        initializer: "initialize",
        kind: "uups",
      });
      await proxyContract.waitForDeployment();

      // Get the GatewayConfig contract factory
      newGatewayConfigFactory = await hre.ethers.getContractFactory("GatewayConfig", owner);
    });

    // This test is not here for making sure the deployment works, as all contracts are deployed in the
    // hardhat "test" pre-hook, but rather to verify that the event is emitted correctly (since it
    // contains several parameters).
    it("Should deploy the GatewayConfig contract", async function () {
      // Upgrade the proxy contract to the GatewayConfig contract
      const upgradeTx = await hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
        call: {
          fn: "initialize",
          args: [pauser.address, protocolMetadata, kmsThreshold, kmsNodes, coprocessors],
        },
      });

      // Extract event args and convert to strings. This is needed as the "upgradeProxy()" method above
      // returns an GatewayConfig instance instead of a ContractTransactionResponse, so the expect() function
      // from chaijs fails on the evaluation of the transaction events.
      const initializationEvents = await upgradeTx.queryFilter(upgradeTx.filters.Initialization);
      const stringifiedEventArgs = (initializationEvents[0] as EventLog).args.map((arg: any) => arg.toString());

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

    it("Should revert because the KMS threshold is too high", async function () {
      // The KMS threshold must be between 0 and the number of KMS nodes
      const highKmsThreshold = nKmsNodes + 1;

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initialize",
            args: [pauser.address, protocolMetadata, highKmsThreshold, kmsNodes, coprocessors],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "KmsThresholdTooHigh")
        .withArgs(highKmsThreshold, nKmsNodes);
    });
  });

  describe("After deployment", function () {
    beforeEach(async function () {
      const fixture = await loadFixture(loadTestVariablesFixture);
      gatewayConfig = fixture.gatewayConfig;
      pauser = fixture.pauser;
      kmsTxSenders = fixture.kmsTxSenders;
      kmsSigners = fixture.kmsSigners;
      coprocessorTxSenders = fixture.coprocessorTxSenders;
    });

    describe("GatewayConfig initialization checks and getters", function () {
      it("Should be registered as an pauser", async function () {
        await expect(gatewayConfig.checkIsPauser(pauser)).to.not.be.reverted;
      });

      it("Should be registered as KMS nodes transaction senders", async function () {
        for (const kmsTxSender of kmsTxSenders) {
          await expect(gatewayConfig.checkIsKmsTxSender(kmsTxSender.address)).to.not.be.reverted;
        }
      });

      it("Should be registered as KMS nodes signers", async function () {
        for (const kmsSigner of kmsSigners) {
          await expect(gatewayConfig.checkIsKmsSigner(kmsSigner.address)).to.not.be.reverted;
        }
      });

      it("Should be registered as coprocessors transaction senders", async function () {
        for (const coprocessorTxSender of coprocessorTxSenders) {
          await expect(gatewayConfig.checkIsCoprocessorTxSender(coprocessorTxSender.address)).to.not.be.reverted;
        }
      });

      it("Should be registered as coprocessors signers", async function () {
        for (const coprocessorSigner of coprocessorSigners) {
          await expect(gatewayConfig.checkIsCoprocessorSigner(coprocessorSigner.address)).to.not.be.reverted;
        }
      });

      it("Should be registered as networks", async function () {
        for (const hostChainId of hostChainIds) {
          await expect(gatewayConfig.checkNetworkIsRegistered(hostChainId)).to.not.be.reverted;
        }
      });

      it("Should get all KMS node transaction sender addresses", async function () {
        const kmsTxSenderAddresses = await gatewayConfig.getKmsTxSenders();

        // Check that the number of KMS node transaction sender addresses is correct
        expect(kmsTxSenderAddresses.length).to.equal(kmsTxSenders.length);

        // Check that all KMS node transaction sender addresses are in the list
        for (const kmsTxSender of kmsTxSenders) {
          expect(kmsTxSenderAddresses).to.include(kmsTxSender.address);
        }
      });

      it("Should get all KMS node signer addresses", async function () {
        const kmsSignerAddresses = await gatewayConfig.getKmsSigners();

        // Check that the number of KMS node signer addresses is correct
        expect(kmsSignerAddresses.length).to.equal(kmsSigners.length);

        // Check that all KMS node signer addresses are in the list
        for (const kmsSigner of kmsSigners) {
          expect(kmsSignerAddresses).to.include(kmsSigner.address);
        }
      });

      it("Should get all coprocessor transaction sender addresses", async function () {
        const coprocessorTxSenderAddresses = await gatewayConfig.getCoprocessorTxSenders();

        // Check that the number of coprocessor transaction sender addresses is correct
        expect(coprocessorTxSenderAddresses.length).to.equal(coprocessorTxSenders.length);

        // Check that all coprocessor transaction sender addresses are in the list
        for (const coprocessorTxSender of coprocessorTxSenders) {
          expect(coprocessorTxSenderAddresses).to.include(coprocessorTxSender.address);
        }
      });

      it("Should get all coprocessor signer addresses", async function () {
        const coprocessorSignerAddresses = await gatewayConfig.getCoprocessorSigners();

        // Check that the number of coprocessor signer addresses is correct
        expect(coprocessorSignerAddresses.length).to.equal(coprocessorSigners.length);

        // Check that all coprocessor signer addresses are in the list
        for (const coprocessorSigner of coprocessorSigners) {
          expect(coprocessorSignerAddresses).to.include(coprocessorSigner.address);
        }
      });

      it("Should get all networks' metadata", async function () {
        const networks = await gatewayConfig.getNetworks();

        // Check that the number of networks is correct
        expect(networks.length).to.equal(hostChainIds.length);

        // Check that all networks' chainIds are in the list
        for (const network of networks) {
          expect(hostChainIds).to.include(Number(network.chainId));
        }
      });
    });

    describe("Pauser", function () {
      it("Should revert because the sender is not the owner", async function () {
        await expect(gatewayConfig.connect(fakeOwner).updatePauser(fakeOwner.address))
          .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
          .withArgs(fakeOwner.address);
      });

      it("Should update the pauser", async function () {
        const newPauser = createRandomWallet();

        const tx = await gatewayConfig.connect(owner).updatePauser(newPauser.address);

        await expect(tx).to.emit(gatewayConfig, "UpdatePauser").withArgs(newPauser.address);
      });

      it("Should revert because the pauser is the null address", async function () {
        const nullAddress = hre.ethers.ZeroAddress;

        await expect(gatewayConfig.connect(owner).updatePauser(nullAddress)).to.be.revertedWithCustomError(
          gatewayConfig,
          "InvalidNullPauser",
        );
      });
    });

    describe("Update KMS threshold", function () {
      it("Should revert because the sender is not the owner", async function () {
        await expect(gatewayConfig.connect(fakeOwner).updateKmsThreshold(1))
          .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
          .withArgs(fakeOwner.address);
      });

      it("Should update the KMS threshold", async function () {
        // The KMS threshold must be between 0 and the number of KMS nodes
        const newKmsThreshold = 0;

        const tx = await gatewayConfig.connect(owner).updateKmsThreshold(newKmsThreshold);

        await expect(tx).to.emit(gatewayConfig, "UpdateKmsThreshold").withArgs(newKmsThreshold);

        // Check that the KMS threshold has been updated
        expect(await gatewayConfig.getKmsThreshold()).to.equal(newKmsThreshold);
      });

      it("Should revert because the KMS threshold is too high", async function () {
        // The KMS threshold must be between 0 and the number of KMS nodes
        const highKmsThreshold = nKmsNodes + 1;

        await expect(gatewayConfig.connect(owner).updateKmsThreshold(highKmsThreshold))
          .to.be.revertedWithCustomError(gatewayConfig, "KmsThresholdTooHigh")
          .withArgs(highKmsThreshold, nKmsNodes);
      });
    });

    describe("Add network", function () {
      // Define a new chainId that is not already registered (since the GatewayConfig contract has already
      // been deployed and networks have been registered)
      const newChainId = Math.max(...hostChainIds) + 1;

      const fhevmExecutorAddress = hre.ethers.getAddress("0x1234567890AbcdEF1234567890aBcdef12345678");
      const aclAddress = hre.ethers.getAddress("0xabcdef1234567890abcdef1234567890abcdef12");
      const name = "Network";
      const website = "https://network.com";

      const newNetwork = {
        chainId: newChainId,
        fhevmExecutor: fhevmExecutorAddress,
        aclAddress,
        name,
        website,
      };

      it("Should add a new network", async function () {
        const txResponse = gatewayConfig.connect(owner).addNetwork(newNetwork);

        await expect(txResponse).to.emit(gatewayConfig, "AddNetwork").withArgs(toValues(newNetwork));
      });

      it("Should revert because the sender is not the owner", async function () {
        await expect(gatewayConfig.connect(fakeOwner).addNetwork(newNetwork)).to.revertedWithCustomError(
          gatewayConfig,
          "OwnableUnauthorizedAccount",
        );
      });

      it("Should revert because the network's chainId is null", async function () {
        const nullChainIdNetwork = {
          chainId: 0,
          fhevmExecutor: fhevmExecutorAddress,
          aclAddress,
          name,
          website,
        };

        await expect(gatewayConfig.connect(owner).addNetwork(nullChainIdNetwork)).to.revertedWithCustomError(
          gatewayConfig,
          "InvalidNullChainId",
        );
      });

      it("Should revert because the network's chainId is not representable by a uint64", async function () {
        // Define a chainId that is not representable by a uint64
        const chainIdTooLarge = UINT64_MAX + 1n;

        const chainIdTooLargeNetwork = {
          chainId: chainIdTooLarge,
          fhevmExecutor: fhevmExecutorAddress,
          aclAddress,
          name,
          website,
        };

        await expect(gatewayConfig.connect(owner).addNetwork(chainIdTooLargeNetwork))
          .to.revertedWithCustomError(gatewayConfig, "ChainIdNotUint64")
          .withArgs(chainIdTooLarge);
      });

      it("Should revert because another network with the same chainId already has been registered", async function () {
        // Get the first host chainId that has already been registered
        const alreadyAddedHostChainId = hostChainIds[0];

        const alreadyAddedNetwork = {
          chainId: alreadyAddedHostChainId,
          fhevmExecutor: fhevmExecutorAddress,
          aclAddress,
          name,
          website,
        };

        await expect(gatewayConfig.connect(owner).addNetwork(alreadyAddedNetwork))
          .to.revertedWithCustomError(gatewayConfig, "NetworkAlreadyRegistered")
          .withArgs(alreadyAddedHostChainId);
      });
    });
  });
});
