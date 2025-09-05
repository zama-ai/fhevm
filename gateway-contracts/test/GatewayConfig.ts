import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ContractFactory, EventLog, Wallet } from "ethers";
import hre from "hardhat";

import {
  CiphertextCommits,
  Decryption,
  EmptyUUPSProxy,
  GatewayConfig,
  InputVerification,
  KmsManagement,
  MultichainAcl,
} from "../typechain-types";
// The type needs to be imported separately because it is not properly detected by the linter
// as this type is defined as a shared structs instead of directly in the IDecryption interface
import {
  CoprocessorStruct,
  CustodianStruct,
  KmsNodeStruct,
} from "../typechain-types/contracts/interfaces/IGatewayConfig";
import { UINT64_MAX, createRandomWallet, loadHostChainIds, loadTestVariablesFixture, toValues } from "./utils";

describe("GatewayConfig", function () {
  // Get the registered host chains' chainIds
  const hostChainIds = loadHostChainIds();

  // Define input values
  const protocolMetadata = { name: "Protocol", website: "https://protocol.com" };
  const mpcThreshold = 1;
  const publicDecryptionThreshold = 3;
  const userDecryptionThreshold = 3;

  // Define fake values
  const fakeOwner = createRandomWallet();
  const fakeTxSender = createRandomWallet();
  const fakeSigner = createRandomWallet();

  let gatewayConfig: GatewayConfig;
  let owner: Wallet;
  let pauser: HardhatEthersSigner;
  let nKmsNodes: number;
  let kmsNodes: KmsNodeV2Struct[];
  let kmsTxSenders: HardhatEthersSigner[];
  let kmsSigners: HardhatEthersSigner[];
  let coprocessors: CoprocessorStruct[];
  let custodians: CustodianStruct[];
  let coprocessorTxSenders: HardhatEthersSigner[];
  let coprocessorSigners: HardhatEthersSigner[];
  let custodianTxSenders: HardhatEthersSigner[];
  let custodianSigners: HardhatEthersSigner[];

  async function getInputsForDeployFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const {
      kmsTxSenders,
      kmsSigners,
      kmsNodeIps,
      kmsNodeS3BucketUrls,
      nKmsNodes,
      coprocessorTxSenders,
      coprocessorSigners,
      coprocessorS3Buckets,
      nCoprocessors,
      custodianTxSenders,
      custodianSigners,
      custodianEncryptionKeys,
      nCustodians,
    } = fixtureData;

    // Create KMS nodes with the tx sender and signer addresses
    kmsNodes = [];
    for (let i = 0; i < nKmsNodes; i++) {
      kmsNodes.push({
        txSenderAddress: kmsTxSenders[i].address,
        signerAddress: kmsSigners[i].address,
        ipAddress: kmsNodeIps[i],
        s3BucketUrl: kmsNodeS3BucketUrls[i],
      });
    }

    // Create coprocessors with the tx sender and signer addresses
    coprocessors = [];
    for (let i = 0; i < nCoprocessors; i++) {
      coprocessors.push({
        txSenderAddress: coprocessorTxSenders[i].address,
        signerAddress: coprocessorSigners[i].address,
        s3BucketUrl: coprocessorS3Buckets[i],
      });
    }

    // Create custodians with the tx sender addresses
    custodians = [];
    for (let i = 0; i < nCustodians; i++) {
      custodians.push({
        txSenderAddress: custodianTxSenders[i].address,
        signerAddress: custodianSigners[i].address,
        encryptionKey: custodianEncryptionKeys[i],
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
          fn: "initializeFromEmptyProxy",
          args: [
            pauser.address,
            protocolMetadata,
            mpcThreshold,
            publicDecryptionThreshold,
            userDecryptionThreshold,
            kmsNodes,
            coprocessors,
            custodians,
          ],
        },
      });

      // Extract event args and convert to strings. This is needed as the "upgradeProxy()" method above
      // returns an GatewayConfig instance instead of a ContractTransactionResponse, so the expect() function
      // from chaijs fails on the evaluation of the transaction events.
      const initializeGatewayConfigEvents = await upgradeTx.queryFilter(upgradeTx.filters.InitializeGatewayConfig);
      const stringifiedEventArgs = (initializeGatewayConfigEvents[0] as EventLog).args.map((arg: any) =>
        arg.toString(),
      );

      // It should emit one event containing the initialization parameters
      expect(initializeGatewayConfigEvents.length).to.equal(1);
      expect(stringifiedEventArgs).to.deep.equal([
        pauser.address,
        toValues(protocolMetadata).toString(),
        mpcThreshold,
        toValues(kmsNodes).toString(),
        toValues(coprocessors).toString(),
        toValues(custodians).toString(),
      ]);
    });

    it("Should revert because the pauser is the null address", async function () {
      const nullPauser = hre.ethers.ZeroAddress;

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              nullPauser,
              protocolMetadata,
              mpcThreshold,
              publicDecryptionThreshold,
              userDecryptionThreshold,
              kmsNodes,
              coprocessors,
              custodians,
            ],
          },
        }),
      ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullPauser");
    });

    it("Should revert because the KMS nodes list is empty", async function () {
      const emptyKmsNodes: KmsNodeStruct[] = [];

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              pauser.address,
              protocolMetadata,
              mpcThreshold,
              publicDecryptionThreshold,
              userDecryptionThreshold,
              emptyKmsNodes,
              coprocessors,
              custodians,
            ],
          },
        }),
      ).to.be.revertedWithCustomError(gatewayConfig, "EmptyKmsNodes");
    });

    it("Should revert because the coprocessors list is empty", async function () {
      const emptyCoprocessors: CoprocessorStruct[] = [];

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              pauser.address,
              protocolMetadata,
              mpcThreshold,
              publicDecryptionThreshold,
              userDecryptionThreshold,
              kmsNodes,
              emptyCoprocessors,
              custodians,
            ],
          },
        }),
      ).to.be.revertedWithCustomError(gatewayConfig, "EmptyCoprocessors");
    });

    it("Should revert because the custodians list is empty", async function () {
      const emptyCustodians: CustodianStruct[] = [];

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              pauser.address,
              protocolMetadata,
              mpcThreshold,
              publicDecryptionThreshold,
              userDecryptionThreshold,
              kmsNodes,
              coprocessors,
              emptyCustodians,
            ],
          },
        }),
      ).to.be.revertedWithCustomError(gatewayConfig, "EmptyCustodians");
    });

    it("Should revert because the MPC threshold is too high", async function () {
      // The MPC threshold must be strictly less than the number of KMS nodes
      const highMpcThreshold = nKmsNodes;

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              pauser.address,
              protocolMetadata,
              highMpcThreshold,
              publicDecryptionThreshold,
              userDecryptionThreshold,
              kmsNodes,
              coprocessors,
              custodians,
            ],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighMpcThreshold")
        .withArgs(highMpcThreshold, nKmsNodes);
    });

    it("Should revert because the public decryption threshold is null", async function () {
      // The public decryption threshold must be greater than 0
      const nullPublicDecryptionThreshold = 0;

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              pauser.address,
              protocolMetadata,
              mpcThreshold,
              nullPublicDecryptionThreshold,
              userDecryptionThreshold,
              kmsNodes,
              coprocessors,
              custodians,
            ],
          },
        }),
      ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullPublicDecryptionThreshold");
    });

    it("Should revert because the public decryption threshold is too high", async function () {
      // The public decryption threshold must be less or equal to the number of KMS nodes
      const highPublicDecryptionThreshold = nKmsNodes + 1;

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              pauser.address,
              protocolMetadata,
              mpcThreshold,
              highPublicDecryptionThreshold,
              userDecryptionThreshold,
              kmsNodes,
              coprocessors,
              custodians,
            ],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighPublicDecryptionThreshold")
        .withArgs(highPublicDecryptionThreshold, nKmsNodes);
    });

    it("Should revert because the user decryption threshold is null", async function () {
      // The user decryption threshold must be greater than 0
      const nullUserDecryptionThreshold = 0;

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              pauser.address,
              protocolMetadata,
              mpcThreshold,
              publicDecryptionThreshold,
              nullUserDecryptionThreshold,
              kmsNodes,
              coprocessors,
              custodians,
            ],
          },
        }),
      ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullUserDecryptionThreshold");
    });

    it("Should revert because the user decryption threshold is too high", async function () {
      // The user decryption threshold must be less or equal to the number of KMS nodes
      const highUserDecryptionThreshold = nKmsNodes + 1;

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              pauser.address,
              protocolMetadata,
              mpcThreshold,
              publicDecryptionThreshold,
              highUserDecryptionThreshold,
              kmsNodes,
              coprocessors,
              custodians,
            ],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighUserDecryptionThreshold")
        .withArgs(highUserDecryptionThreshold, nKmsNodes);
    });

    it("Should revert because initialization is not from an empty proxy", async function () {
      await expect(
        hre.upgrades.upgradeProxy(gatewayConfig, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              pauser.address,
              protocolMetadata,
              mpcThreshold,
              publicDecryptionThreshold,
              userDecryptionThreshold,
              kmsNodes,
              coprocessors,
              custodians,
            ],
          },
        }),
      ).to.be.revertedWithCustomError(gatewayConfig, "NotInitializingFromEmptyProxy");
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
      custodianTxSenders = fixture.custodianTxSenders;
      custodianSigners = fixture.custodianSigners;
    });

    describe("GatewayConfig initialization checks and getters", function () {
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

      it("Should not be registered as coprocessors transaction senders", async function () {
        await expect(gatewayConfig.checkIsCoprocessorTxSender(fakeTxSender))
          .to.be.revertedWithCustomError(gatewayConfig, "NotCoprocessorTxSender")
          .withArgs(fakeTxSender);
      });

      it("Should be registered as coprocessors signers", async function () {
        for (const coprocessorSigner of coprocessorSigners) {
          await expect(gatewayConfig.checkIsCoprocessorSigner(coprocessorSigner.address)).to.not.be.reverted;
        }
      });

      it("Should not be registered as coprocessors signers", async function () {
        await expect(gatewayConfig.checkIsCoprocessorSigner(fakeSigner))
          .to.be.revertedWithCustomError(gatewayConfig, "NotCoprocessorSigner")
          .withArgs(fakeSigner);
      });

      it("Should be registered as custodian transaction senders", async function () {
        for (const custodianTxSender of custodianTxSenders) {
          await expect(gatewayConfig.checkIsCustodianTxSender(custodianTxSender.address)).to.not.be.reverted;
        }
      });

      it("Should not be registered as custodian transaction senders", async function () {
        await expect(gatewayConfig.checkIsCustodianTxSender(fakeTxSender))
          .to.be.revertedWithCustomError(gatewayConfig, "NotCustodianTxSender")
          .withArgs(fakeTxSender);
      });

      it("Should be registered as custodian signers", async function () {
        for (const custodianSigner of custodianSigners) {
          await expect(gatewayConfig.checkIsCustodianSigner(custodianSigner.address)).to.not.be.reverted;
        }
      });

      it("Should be registered as custodian signers", async function () {
        await expect(gatewayConfig.checkIsCustodianSigner(fakeSigner))
          .to.be.revertedWithCustomError(gatewayConfig, "NotCustodianSigner")
          .withArgs(fakeSigner);
      });

      it("Should be registered as host chains", async function () {
        for (const hostChainId of hostChainIds) {
          await expect(gatewayConfig.checkHostChainIsRegistered(hostChainId)).to.not.be.reverted;
        }
      });

      it("Should get the protocol metadata", async function () {
        const metadata = await gatewayConfig.getProtocolMetadata();

        // Check that the protocol metadata is correct
        expect(metadata).to.deep.equal(toValues(protocolMetadata));
      });

      it("Should get the KMS node metadata by its transaction sender address", async function () {
        const kmsNode = await gatewayConfig.getKmsNode(kmsNodes[0].txSenderAddress);

        // Check that KMS node metadata for the given transaction sender addresses is correct
        expect(kmsNode).to.deep.equal(toValues(kmsNodes[0]));
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

      it("Should get custodian metadata from transaction sender addresses", async function () {
        for (let i = 0; i < custodianTxSenders.length; i++) {
          const custodian = await gatewayConfig.getCustodian(custodianTxSenders[i].address);
          expect(custodian).to.deep.equal(toValues(custodians[i]));
        }
      });

      it("Should get all custodian transaction sender addresses", async function () {
        const custodianTxSenderAddresses = await gatewayConfig.getCustodianTxSenders();

        // Check that the number of custodian transaction sender addresses is correct
        expect(custodianTxSenderAddresses.length).to.equal(custodianTxSenders.length);

        // Check that all custodian transaction sender addresses are in the list
        for (const custodianTxSender of custodianTxSenders) {
          expect(custodianTxSenderAddresses).to.include(custodianTxSender.address);
        }
      });

      it("Should get all custodian signer addresses", async function () {
        const custodianSignerAddresses = await gatewayConfig.getCustodianSigners();

        // Check that the number of custodian signer addresses is correct
        expect(custodianSignerAddresses.length).to.equal(custodianSigners.length);

        // Check that all custodian signer addresses are in the list
        for (const custodianSigner of custodianSigners) {
          expect(custodianSignerAddresses).to.include(custodianSigner.address);
        }
      });

      it("Should get all host chains' metadata", async function () {
        const hostChains = await gatewayConfig.getHostChains();

        // Check that the number of host chains is correct
        expect(hostChains.length).to.equal(hostChainIds.length);

        // Check that all host chains' chainIds are in the list
        for (const hostChain of hostChains) {
          expect(hostChainIds).to.include(Number(hostChain.chainId));
        }
      });

      it("Should get host chain's metadata", async function () {
        const hostChains = await gatewayConfig.getHostChains();

        for (let i = 0; i < hostChainIds.length; i++) {
          const hostChain = await gatewayConfig.getHostChain(i);
          expect(hostChain).to.deep.equal(hostChains[i]);
        }
      });
    });

    describe("Pauser", function () {
      it("Should return the initialized pauser address", async function () {
        expect(await gatewayConfig.getPauser()).to.equal(pauser.address);
      });

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
        const nullPauser = hre.ethers.ZeroAddress;

        await expect(gatewayConfig.connect(owner).updatePauser(nullPauser)).to.be.revertedWithCustomError(
          gatewayConfig,
          "InvalidNullPauser",
        );
      });
    });

    describe("Update MPC threshold", function () {
      it("Should revert because the sender is not the owner", async function () {
        await expect(gatewayConfig.connect(fakeOwner).updateMpcThreshold(1))
          .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
          .withArgs(fakeOwner.address);
      });

      it("Should update the MPC threshold", async function () {
        const newMpcThreshold = 0;

        const tx = await gatewayConfig.connect(owner).updateMpcThreshold(newMpcThreshold);

        await expect(tx).to.emit(gatewayConfig, "UpdateMpcThreshold").withArgs(newMpcThreshold);

        // Check that the MPC threshold has been updated
        expect(await gatewayConfig.getMpcThreshold()).to.equal(newMpcThreshold);
      });

      it("Should revert because the MPC threshold is too high", async function () {
        // The MPC threshold must be strictly less than the number of KMS nodes
        const highMpcThreshold = nKmsNodes;

        await expect(gatewayConfig.connect(owner).updateMpcThreshold(highMpcThreshold))
          .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighMpcThreshold")
          .withArgs(highMpcThreshold, nKmsNodes);
      });
    });

    describe("Update public decryption threshold", function () {
      it("Should revert because the sender is not the owner", async function () {
        await expect(gatewayConfig.connect(fakeOwner).updatePublicDecryptionThreshold(1))
          .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
          .withArgs(fakeOwner.address);
      });

      it("Should update the public decryption threshold", async function () {
        // The public decryption threshold must be greater than 0
        const newPublicDecryptionThreshold = 1;

        const tx = await gatewayConfig.connect(owner).updatePublicDecryptionThreshold(newPublicDecryptionThreshold);

        await expect(tx)
          .to.emit(gatewayConfig, "UpdatePublicDecryptionThreshold")
          .withArgs(newPublicDecryptionThreshold);

        // Check that the public decryption threshold has been updated
        expect(await gatewayConfig.getPublicDecryptionThreshold()).to.equal(newPublicDecryptionThreshold);
      });

      it("Should revert because the public decryption threshold is null", async function () {
        // The public decryption threshold must be greater than 0
        const nullPublicDecryptionThreshold = 0;

        await expect(
          gatewayConfig.connect(owner).updatePublicDecryptionThreshold(nullPublicDecryptionThreshold),
        ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullPublicDecryptionThreshold");
      });

      it("Should revert because the public decryption threshold is too high", async function () {
        // The public decryption threshold must be less or equal to the number of KMS nodes
        const highPublicDecryptionThreshold = nKmsNodes + 1;

        await expect(gatewayConfig.connect(owner).updatePublicDecryptionThreshold(highPublicDecryptionThreshold))
          .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighPublicDecryptionThreshold")
          .withArgs(highPublicDecryptionThreshold, nKmsNodes);
      });
    });

    describe("Update user decryption threshold", function () {
      it("Should revert because the sender is not the owner", async function () {
        await expect(gatewayConfig.connect(fakeOwner).updateUserDecryptionThreshold(1))
          .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
          .withArgs(fakeOwner.address);
      });

      it("Should update the user decryption threshold", async function () {
        // The user decryption threshold must be greater than 0
        const newUserDecryptionThreshold = 1;

        const tx = await gatewayConfig.connect(owner).updateUserDecryptionThreshold(newUserDecryptionThreshold);

        await expect(tx).to.emit(gatewayConfig, "UpdateUserDecryptionThreshold").withArgs(newUserDecryptionThreshold);

        // Check that the user decryption threshold has been updated
        expect(await gatewayConfig.getUserDecryptionThreshold()).to.equal(newUserDecryptionThreshold);
      });

      it("Should revert because the user decryption threshold is null", async function () {
        // The user decryption threshold must be greater than 0
        const nullUserDecryptionThreshold = 0;

        await expect(
          gatewayConfig.connect(owner).updateUserDecryptionThreshold(nullUserDecryptionThreshold),
        ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullUserDecryptionThreshold");
      });

      it("Should revert because the user decryption threshold is too high", async function () {
        // The user decryption threshold must be less or equal to the number of KMS nodes
        const highUserDecryptionThreshold = nKmsNodes + 1;

        await expect(gatewayConfig.connect(owner).updateUserDecryptionThreshold(highUserDecryptionThreshold))
          .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighUserDecryptionThreshold")
          .withArgs(highUserDecryptionThreshold, nKmsNodes);
      });
    });

    describe("Add host chain", function () {
      // Define a new chain ID that does not correspond to an already registered host chain
      // (since the GatewayConfig contract has already been deployed and host chains have been
      // registered)
      const newHostChainId = Math.max(...hostChainIds) + 1;

      const fhevmExecutorAddress = hre.ethers.getAddress("0x1234567890AbcdEF1234567890aBcdef12345678");
      const aclAddress = hre.ethers.getAddress("0xabcdef1234567890abcdef1234567890abcdef12");
      const name = "Host chain";
      const website = "https://host-chain-test.com";

      const newHostChain = {
        chainId: newHostChainId,
        fhevmExecutorAddress,
        aclAddress,
        name,
        website,
      };

      it("Should add a new host chain", async function () {
        const txResponse = gatewayConfig.connect(owner).addHostChain(newHostChain);

        await expect(txResponse).to.emit(gatewayConfig, "AddHostChain").withArgs(toValues(newHostChain));
      });

      it("Should revert because the sender is not the owner", async function () {
        await expect(gatewayConfig.connect(fakeOwner).addHostChain(newHostChain)).to.revertedWithCustomError(
          gatewayConfig,
          "OwnableUnauthorizedAccount",
        );
      });

      it("Should revert because the host chain's chain ID is null", async function () {
        const nullChainIdHostChain = {
          chainId: 0,
          fhevmExecutorAddress,
          aclAddress,
          name,
          website,
        };

        await expect(gatewayConfig.connect(owner).addHostChain(nullChainIdHostChain)).to.revertedWithCustomError(
          gatewayConfig,
          "InvalidNullChainId",
        );
      });

      it("Should revert because the host chain's chain ID is not representable by a uint64", async function () {
        // Define a chain ID that is not representable by a uint64
        const chainIdTooLarge = UINT64_MAX + 1n;

        const chainIdTooLargeHostChain = {
          chainId: chainIdTooLarge,
          fhevmExecutorAddress,
          aclAddress,
          name,
          website,
        };

        await expect(gatewayConfig.connect(owner).addHostChain(chainIdTooLargeHostChain))
          .to.revertedWithCustomError(gatewayConfig, "ChainIdNotUint64")
          .withArgs(chainIdTooLarge);
      });

      it("Should revert because another host chain with the same chain ID already has been registered", async function () {
        // Get the first host chain ID that has already been registered
        const alreadyAddedHostChainId = hostChainIds[0];

        const alreadyAddedHostChain = {
          chainId: alreadyAddedHostChainId,
          fhevmExecutorAddress,
          aclAddress,
          name,
          website,
        };

        await expect(gatewayConfig.connect(owner).addHostChain(alreadyAddedHostChain))
          .to.revertedWithCustomError(gatewayConfig, "HostChainAlreadyRegistered")
          .withArgs(alreadyAddedHostChainId);
      });
    });
  });

  describe("Pause", async function () {
    const fakeOwner = createRandomWallet();
    const fakePauser = createRandomWallet();

    beforeEach(async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      gatewayConfig = fixtureData.gatewayConfig;
      owner = fixtureData.owner;
      pauser = fixtureData.pauser;
    });

    it("Should pause the contract with the pauser and unpause with the owner", async function () {
      // Check that the contract is not paused
      expect(await gatewayConfig.paused()).to.be.false;

      // Pause the contract with the pauser address
      await expect(gatewayConfig.connect(pauser).pause()).to.emit(gatewayConfig, "Paused").withArgs(pauser);
      expect(await gatewayConfig.paused()).to.be.true;

      // Unpause the contract with the owner address
      await expect(gatewayConfig.connect(owner).unpause()).to.emit(gatewayConfig, "Unpaused").withArgs(owner);
      expect(await gatewayConfig.paused()).to.be.false;
    });

    it("Should revert on pause because sender is not the pauser", async function () {
      await expect(gatewayConfig.connect(fakePauser).pause())
        .to.be.revertedWithCustomError(gatewayConfig, "NotPauserOrGatewayConfig")
        .withArgs(fakePauser.address);
    });

    it("Should revert on unpause because sender is not the owner", async function () {
      // Pause the contract with the pauser address
      await gatewayConfig.connect(pauser).pause();

      await expect(gatewayConfig.connect(fakeOwner).unpause())
        .to.be.revertedWithCustomError(gatewayConfig, "NotOwnerOrGatewayConfig")
        .withArgs(fakeOwner.address);
    });

    describe("Pause all gateway contracts", function () {
      let ciphertextCommits: CiphertextCommits;
      let decryption: Decryption;
      let inputVerification: InputVerification;
      let kmsManagement: KmsManagement;
      let multichainAcl: MultichainAcl;

      before(async function () {
        const fixtureData = await loadFixture(loadTestVariablesFixture);
        ciphertextCommits = fixtureData.ciphertextCommits;
        decryption = fixtureData.decryption;
        inputVerification = fixtureData.inputVerification;
        kmsManagement = fixtureData.kmsManagement;
        multichainAcl = fixtureData.multichainAcl;
      });

      it("Should pause all the Gateway contracts with the pauser", async function () {
        // Check that the contracts are not paused
        expect(await ciphertextCommits.paused()).to.be.false;
        expect(await decryption.paused()).to.be.false;
        expect(await gatewayConfig.paused()).to.be.false;
        expect(await inputVerification.paused()).to.be.false;
        expect(await kmsManagement.paused()).to.be.false;
        expect(await multichainAcl.paused()).to.be.false;

        const txResponse = await gatewayConfig.connect(pauser).pauseAllGatewayContracts();

        await expect(txResponse).to.emit(gatewayConfig, "PauseAllGatewayContracts");

        // Check that the pausable contracts are paused
        expect(await decryption.paused()).to.be.true;
        expect(await inputVerification.paused()).to.be.true;

        // Check that the non-pausable contracts are not paused
        // The following contracts are pausable but don't have pausable functions yet, so they are
        // not paused by the `pauseAllGatewayContracts()` function for now:
        // - CiphertextCommits
        // - MultichainAcl
        // - GatewayConfig
        // In addition, the `KmsManagement` contract is not used yet, so we don't need to pause it for now.
        // See https://github.com/zama-ai/fhevm-internal/issues/180
        expect(await ciphertextCommits.paused()).to.be.false;
        expect(await multichainAcl.paused()).to.be.false;
        expect(await gatewayConfig.paused()).to.be.false;
        expect(await kmsManagement.paused()).to.be.false;
      });

      it("Should revert on pause all gateway contracts because the sender is not the pauser", async function () {
        await expect(gatewayConfig.connect(fakePauser).pauseAllGatewayContracts()).to.be.revertedWithCustomError(
          gatewayConfig,
          "NotPauser",
        );
      });

      it("Should unpause all the gateway contracts with the owner", async function () {
        // Pause the contract with the pauser address
        await gatewayConfig.connect(pauser).pauseAllGatewayContracts();

        // Unpause the contract with the owner address
        const txResponse = await gatewayConfig.connect(owner).unpauseAllGatewayContracts();

        await expect(txResponse).to.emit(gatewayConfig, "UnpauseAllGatewayContracts");

        // Check that the contracts are not paused anymore
        expect(await ciphertextCommits.paused()).to.be.false;
        expect(await decryption.paused()).to.be.false;
        expect(await gatewayConfig.paused()).to.be.false;
        expect(await inputVerification.paused()).to.be.false;
        expect(await kmsManagement.paused()).to.be.false;
        expect(await multichainAcl.paused()).to.be.false;
      });

      it("Should revert on unpause all gateway contracts because the sender is not the owner", async function () {
        await expect(gatewayConfig.connect(fakeOwner).unpauseAllGatewayContracts()).to.be.revertedWithCustomError(
          gatewayConfig,
          "OwnableUnauthorizedAccount",
        );
      });
    });
  });
});
