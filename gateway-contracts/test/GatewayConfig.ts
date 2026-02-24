import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ContractFactory, EventLog, Wallet, ZeroAddress } from "ethers";
import hre from "hardhat";

import { Decryption, EmptyUUPSProxyGatewayConfig, GatewayConfig, InputVerification } from "../typechain-types";
// The type needs to be imported separately because it is not properly detected by the linter
// as this type is defined as a shared structs instead of directly in the IDecryption interface
import {
  CoprocessorStruct,
  CustodianStruct,
  KmsNodeStruct,
} from "../typechain-types/contracts/interfaces/IGatewayConfig";
import {
  UINT64_MAX,
  createByteInput,
  createRandomAddress,
  createRandomWallet,
  loadHostChainIds,
  loadTestVariablesFixture,
  toValues,
} from "./utils";

describe("GatewayConfig", function () {
  // Get the registered host chains' chainIds
  const hostChainIds = loadHostChainIds();

  // Define input values
  // KMS context ID format: [0x07 | counter_1..31]
  const initialKmsContextId = (7n << 248n) | 1n;
  const nextKmsContextId = (7n << 248n) | 2n;
  const protocolMetadata = { name: "Protocol", website: "https://protocol.com" };
  const mpcThreshold = 1;
  const publicDecryptionThreshold = 3;
  const userDecryptionThreshold = 3;
  const kmsGenThreshold = 3;
  const coprocessorThreshold = 2;
  const thresholds = {
    mpcThreshold,
    publicDecryptionThreshold,
    userDecryptionThreshold,
    kmsGenThreshold,
    coprocessorThreshold,
  };

  // Define bad values
  const emptyKmsNodes: KmsNodeStruct[] = [];
  const emptyCoprocessors: CoprocessorStruct[] = [];
  const emptyCustodians: CustodianStruct[] = [];
  const nullPublicDecryptionThreshold = 0;
  const nullUserDecryptionThreshold = 0;
  const nullKmsGenThreshold = 0;
  const nullCoprocessorThreshold = 0;

  // Define fake values
  const fakeOwner = createRandomWallet();
  const fakeTxSender = createRandomWallet();
  const fakeSigner = createRandomWallet();

  let gatewayConfig: GatewayConfig;
  let owner: Wallet;
  let pauser: Wallet;
  let nKmsNodes: number;
  let kmsNodes: KmsNodeStruct[];
  let kmsTxSenders: HardhatEthersSigner[];
  let kmsSigners: HardhatEthersSigner[];
  let coprocessors: CoprocessorStruct[];
  let nCoprocessors: number;
  let coprocessorTxSenders: HardhatEthersSigner[];
  let coprocessorSigners: HardhatEthersSigner[];
  let custodians: CustodianStruct[];
  let custodianTxSenders: HardhatEthersSigner[];
  let custodianSigners: HardhatEthersSigner[];
  let highMpcThreshold: number;
  let highPublicDecryptionThreshold: number;
  let highUserDecryptionThreshold: number;
  let highKmsGenThreshold: number;
  let highCoprocessorThreshold: number;

  async function getInputsForDeployFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const {
      kmsTxSenders,
      kmsSigners,
      kmsNodeIps,
      kmsNodeStorageUrls,
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
        storageUrl: kmsNodeStorageUrls[i],
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
    nCoprocessors = fixtureData.nCoprocessors;
    coprocessorTxSenders = fixtureData.coprocessorTxSenders;
    coprocessorSigners = fixtureData.coprocessorSigners;

    highMpcThreshold = nKmsNodes;
    highPublicDecryptionThreshold = nKmsNodes + 1;
    highUserDecryptionThreshold = nKmsNodes + 1;
    highKmsGenThreshold = nKmsNodes + 1;
    highCoprocessorThreshold = nCoprocessors + 1;
  });

  describe("Deployment", function () {
    let proxyContract: EmptyUUPSProxyGatewayConfig;
    let newGatewayConfigFactory: ContractFactory;

    beforeEach(async function () {
      // Deploy a new proxy contract for the GatewayConfig contract
      const proxyImplementation = await hre.ethers.getContractFactory("EmptyUUPSProxyGatewayConfig", owner);
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
          args: [initialKmsContextId, protocolMetadata, thresholds, kmsNodes, coprocessors, custodians],
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
        initialKmsContextId.toString(),
        toValues(protocolMetadata).toString(),
        toValues(thresholds).toString(),
        toValues(kmsNodes).toString(),
        toValues(coprocessors).toString(),
        toValues(custodians).toString(),
      ]);
    });

    it("Should revert because the KMS nodes list is empty", async function () {
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [initialKmsContextId, protocolMetadata, thresholds, emptyKmsNodes, coprocessors, custodians],
          },
        }),
      ).to.be.revertedWithCustomError(gatewayConfig, "EmptyKmsNodes");
    });

    it("Should revert because the KMS transaction sender is already registered", async function () {
      // Deep copy the KMS nodes and duplicate the first KMS node's transaction sender address
      const duplicatedTxSenderKmsNode = kmsNodes.map((node) => ({ ...node }));
      duplicatedTxSenderKmsNode[0].txSenderAddress = duplicatedTxSenderKmsNode[1].txSenderAddress;

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              initialKmsContextId,
              protocolMetadata,
              thresholds,
              duplicatedTxSenderKmsNode,
              coprocessors,
              custodians,
            ],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "KmsTxSenderAlreadyRegistered")
        .withArgs(duplicatedTxSenderKmsNode[0].txSenderAddress);
    });

    it("Should revert because the KMS signer is already registered", async function () {
      // Deep copy the KMS nodes and duplicate the first KMS node's signer address
      const duplicatedSignerKmsNode = kmsNodes.map((node) => ({ ...node }));
      duplicatedSignerKmsNode[0].signerAddress = duplicatedSignerKmsNode[1].signerAddress;

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              initialKmsContextId,
              protocolMetadata,
              thresholds,
              duplicatedSignerKmsNode,
              coprocessors,
              custodians,
            ],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "KmsSignerAlreadyRegistered")
        .withArgs(duplicatedSignerKmsNode[0].signerAddress);
    });

    it("Should revert because the coprocessors list is empty", async function () {
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [initialKmsContextId, protocolMetadata, thresholds, kmsNodes, emptyCoprocessors, custodians],
          },
        }),
      ).to.be.revertedWithCustomError(gatewayConfig, "EmptyCoprocessors");
    });

    it("Should revert because the coprocessor transaction sender is already registered", async function () {
      // Deep copy the coprocessors and duplicate the first coprocessor's transaction sender address
      const duplicatedTxSenderCoprocessor = coprocessors.map((processor) => ({ ...processor }));
      duplicatedTxSenderCoprocessor[0].txSenderAddress = duplicatedTxSenderCoprocessor[1].txSenderAddress;

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              initialKmsContextId,
              protocolMetadata,
              thresholds,
              kmsNodes,
              duplicatedTxSenderCoprocessor,
              custodians,
            ],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "CoprocessorTxSenderAlreadyRegistered")
        .withArgs(duplicatedTxSenderCoprocessor[0].txSenderAddress);
    });

    it("Should revert because the coprocessor signer is already registered", async function () {
      // Deep copy the coprocessors and duplicate the first coprocessor's signer address
      const duplicatedSignerCoprocessor = coprocessors.map((processor) => ({ ...processor }));
      duplicatedSignerCoprocessor[0].signerAddress = duplicatedSignerCoprocessor[1].signerAddress;

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              initialKmsContextId,
              protocolMetadata,
              thresholds,
              kmsNodes,
              duplicatedSignerCoprocessor,
              custodians,
            ],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "CoprocessorSignerAlreadyRegistered")
        .withArgs(duplicatedSignerCoprocessor[0].signerAddress);
    });

    it("Should revert because the custodians list is empty", async function () {
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [initialKmsContextId, protocolMetadata, thresholds, kmsNodes, coprocessors, emptyCustodians],
          },
        }),
      ).to.be.revertedWithCustomError(gatewayConfig, "EmptyCustodians");
    });

    it("Should revert because the custodian transaction sender is already registered", async function () {
      // Deep copy the custodians and duplicate the first custodian's transaction sender address
      const duplicatedTxSenderCustodian = custodians.map((custodian) => ({ ...custodian }));
      duplicatedTxSenderCustodian[0].txSenderAddress = duplicatedTxSenderCustodian[1].txSenderAddress;

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              initialKmsContextId,
              protocolMetadata,
              thresholds,
              kmsNodes,
              coprocessors,
              duplicatedTxSenderCustodian,
            ],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "CustodianTxSenderAlreadyRegistered")
        .withArgs(duplicatedTxSenderCustodian[0].txSenderAddress);
    });

    it("Should revert because the custodian signer is already registered", async function () {
      // Deep copy the custodians and duplicate the first custodian's signer address
      const duplicatedSignerCustodian = custodians.map((custodian) => ({ ...custodian }));
      duplicatedSignerCustodian[0].signerAddress = duplicatedSignerCustodian[1].signerAddress;

      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [
              initialKmsContextId,
              protocolMetadata,
              thresholds,
              kmsNodes,
              coprocessors,
              duplicatedSignerCustodian,
            ],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "CustodianSignerAlreadyRegistered")
        .withArgs(duplicatedSignerCustodian[0].signerAddress);
    });

    it("Should revert because the MPC threshold is too high", async function () {
      const badThresholds = {
        mpcThreshold: highMpcThreshold,
        publicDecryptionThreshold,
        userDecryptionThreshold,
        kmsGenThreshold,
        coprocessorThreshold,
      };
      // The MPC threshold must be strictly less than the number of KMS nodes
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [initialKmsContextId, protocolMetadata, badThresholds, kmsNodes, coprocessors, custodians],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighMpcThreshold")
        .withArgs(highMpcThreshold, nKmsNodes);
    });

    it("Should revert because the public decryption threshold is null", async function () {
      const badThresholds = {
        mpcThreshold,
        publicDecryptionThreshold: nullPublicDecryptionThreshold,
        userDecryptionThreshold,
        kmsGenThreshold,
        coprocessorThreshold,
      };
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [initialKmsContextId, protocolMetadata, badThresholds, kmsNodes, coprocessors, custodians],
          },
        }),
      ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullPublicDecryptionThreshold");
    });

    it("Should revert because the public decryption threshold is too high", async function () {
      const badThresholds = {
        mpcThreshold,
        publicDecryptionThreshold: highPublicDecryptionThreshold,
        userDecryptionThreshold,
        kmsGenThreshold,
        coprocessorThreshold,
      };

      // The public decryption threshold must be less or equal to the number of KMS nodes
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [initialKmsContextId, protocolMetadata, badThresholds, kmsNodes, coprocessors, custodians],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighPublicDecryptionThreshold")
        .withArgs(highPublicDecryptionThreshold, nKmsNodes);
    });

    it("Should revert because the user decryption threshold is null", async function () {
      const badThresholds = {
        mpcThreshold,
        publicDecryptionThreshold,
        userDecryptionThreshold: nullUserDecryptionThreshold,
        kmsGenThreshold,
        coprocessorThreshold,
      };
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [initialKmsContextId, protocolMetadata, badThresholds, kmsNodes, coprocessors, custodians],
          },
        }),
      ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullUserDecryptionThreshold");
    });

    it("Should revert because the user decryption threshold is too high", async function () {
      const badThresholds = {
        mpcThreshold,
        publicDecryptionThreshold,
        userDecryptionThreshold: highUserDecryptionThreshold,
        kmsGenThreshold,
        coprocessorThreshold,
      };

      // The user decryption threshold must be less or equal to the number of KMS nodes
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [initialKmsContextId, protocolMetadata, badThresholds, kmsNodes, coprocessors, custodians],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighUserDecryptionThreshold")
        .withArgs(highUserDecryptionThreshold, nKmsNodes);
    });

    it("Should revert because the KMS generation threshold is null", async function () {
      const badThresholds = {
        mpcThreshold,
        publicDecryptionThreshold,
        userDecryptionThreshold,
        kmsGenThreshold: nullKmsGenThreshold,
        coprocessorThreshold,
      };
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [initialKmsContextId, protocolMetadata, badThresholds, kmsNodes, coprocessors, custodians],
          },
        }),
      ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullKmsGenThreshold");
    });

    it("Should revert because the KMS generation threshold is too high", async function () {
      const badThresholds = {
        mpcThreshold,
        publicDecryptionThreshold,
        userDecryptionThreshold,
        kmsGenThreshold: highKmsGenThreshold,
        coprocessorThreshold,
      };

      // The KMS generation threshold must be less or equal to the number of KMS nodes
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [initialKmsContextId, protocolMetadata, badThresholds, kmsNodes, coprocessors, custodians],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighKmsGenThreshold")
        .withArgs(highKmsGenThreshold, nKmsNodes);
    });

    it("Should revert because the coprocessor threshold is null", async function () {
      const badThresholds = {
        mpcThreshold,
        publicDecryptionThreshold,
        userDecryptionThreshold,
        kmsGenThreshold,
        coprocessorThreshold: nullCoprocessorThreshold,
      };
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [initialKmsContextId, protocolMetadata, badThresholds, kmsNodes, coprocessors, custodians],
          },
        }),
      ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullCoprocessorThreshold");
    });

    it("Should revert because the coprocessor threshold is too high", async function () {
      const badThresholds = {
        mpcThreshold,
        publicDecryptionThreshold,
        userDecryptionThreshold,
        kmsGenThreshold,
        coprocessorThreshold: highCoprocessorThreshold,
      };

      // The coprocessor threshold must be less or equal to the number of coprocessors
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [initialKmsContextId, protocolMetadata, badThresholds, kmsNodes, coprocessors, custodians],
          },
        }),
      )
        .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighCoprocessorThreshold")
        .withArgs(highCoprocessorThreshold, nCoprocessors);
    });

    it("Should revert because initialization is not from an empty proxy", async function () {
      await expect(
        hre.upgrades.upgradeProxy(gatewayConfig, newGatewayConfigFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [initialKmsContextId, protocolMetadata, thresholds, kmsNodes, coprocessors, custodians],
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

    describe("Operators updates", function () {
      // Define new addresses
      const newTxSenderAddress = createRandomAddress();
      const newSignerAddress = createRandomAddress();

      describe("KMS nodes updates", function () {
        it("Should update the KMS nodes", async function () {
          const newKmsNode: KmsNodeStruct = {
            txSenderAddress: newTxSenderAddress,
            signerAddress: newSignerAddress,
            ipAddress: "127.0.0.1000",
            storageUrl: "s3://kms-bucket-1000",
          };
          const newKmsNodes: KmsNodeStruct[] = [newKmsNode];
          const newMpcThreshold = 0;
          const newPublicDecryptionThreshold = 1;
          const newUserDecryptionThreshold = 1;
          const newKmsGenThreshold = 1;
          const newContextId = nextKmsContextId;

          const tx = await gatewayConfig
            .connect(owner)
            .updateKmsContext(
              newContextId,
              newKmsNodes,
              newMpcThreshold,
              newPublicDecryptionThreshold,
              newUserDecryptionThreshold,
              newKmsGenThreshold,
            );

          await expect(tx)
            .to.emit(gatewayConfig, "UpdateKmsContext")
            .withArgs(
              newContextId,
              toValues(newKmsNodes),
              newMpcThreshold,
              newPublicDecryptionThreshold,
              newUserDecryptionThreshold,
              newKmsGenThreshold,
            );

          // Check that the KMS nodes have been updated (global getters)
          expect(await gatewayConfig.isKmsTxSender(newTxSenderAddress)).to.be.true;
          expect(await gatewayConfig.isKmsSigner(newSignerAddress)).to.be.true;
          expect(await gatewayConfig.getKmsNode(newTxSenderAddress)).to.deep.equal(toValues(newKmsNode));
          expect(await gatewayConfig.getKmsTxSenders()).to.deep.equal([newTxSenderAddress]);
          expect(await gatewayConfig.getKmsSigners()).to.deep.equal([newSignerAddress]);

          // Check that the context-indexed state has been updated
          expect(await gatewayConfig.isKmsContextTxSender(newContextId, newTxSenderAddress)).to.be.true;
          expect(await gatewayConfig.isKmsContextSigner(newContextId, newSignerAddress)).to.be.true;
          expect(await gatewayConfig.getKmsContextNode(newContextId, newTxSenderAddress)).to.deep.equal(
            toValues(newKmsNode),
          );
          expect(await gatewayConfig.getKmsContextTxSenders(newContextId)).to.deep.equal([newTxSenderAddress]);
          expect(await gatewayConfig.getKmsContextSigners(newContextId)).to.deep.equal([newSignerAddress]);

          // Check that the active context ID has been updated
          expect(await gatewayConfig.getCurrentKmsContextId()).to.equal(newContextId);

          // Check that the thresholds have been updated
          expect(await gatewayConfig.getMpcThreshold()).to.equal(newMpcThreshold);
          expect(await gatewayConfig.getKmsContextPublicDecryptionThreshold(newContextId)).to.equal(
            newPublicDecryptionThreshold,
          );
          expect(await gatewayConfig.getKmsContextUserDecryptionThreshold(newContextId)).to.equal(
            newUserDecryptionThreshold,
          );
          expect(await gatewayConfig.getKmsGenThreshold()).to.equal(newKmsGenThreshold);

          // Define the null KMS node
          const nullKmsNode: KmsNodeStruct = {
            txSenderAddress: ZeroAddress,
            signerAddress: ZeroAddress,
            ipAddress: "",
            storageUrl: "",
          };

          // Check that old KMS nodes have been removed
          for (const kmsSigner of kmsSigners) {
            expect(await gatewayConfig.isKmsSigner(kmsSigner)).to.be.false;
          }
          for (const kmsTxSender of kmsTxSenders) {
            expect(await gatewayConfig.isKmsTxSender(kmsTxSender)).to.be.false;
            expect(await gatewayConfig.getKmsNode(kmsTxSender)).to.deep.equal(toValues(nullKmsNode));
          }
        });

        it("Should revert because the sender is not the owner", async function () {
          await expect(
            gatewayConfig
              .connect(fakeOwner)
              .updateKmsContext(
                nextKmsContextId,
                kmsNodes,
                mpcThreshold,
                publicDecryptionThreshold,
                userDecryptionThreshold,
                kmsGenThreshold,
              ),
          )
            .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
            .withArgs(fakeOwner.address);
        });

        it("Should revert because the KMS nodes are empty", async function () {
          await expect(
            gatewayConfig
              .connect(owner)
              .updateKmsContext(
                nextKmsContextId,
                emptyKmsNodes,
                mpcThreshold,
                publicDecryptionThreshold,
                userDecryptionThreshold,
                kmsGenThreshold,
              ),
          ).to.be.revertedWithCustomError(gatewayConfig, "EmptyKmsNodes");
        });

        it("Should revert because the MPC threshold is too high", async function () {
          await expect(
            gatewayConfig
              .connect(owner)
              .updateKmsContext(
                nextKmsContextId,
                kmsNodes,
                highMpcThreshold,
                publicDecryptionThreshold,
                userDecryptionThreshold,
                kmsGenThreshold,
              ),
          )
            .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighMpcThreshold")
            .withArgs(highMpcThreshold, nKmsNodes);
        });

        it("Should revert because the public decryption threshold is null", async function () {
          await expect(
            gatewayConfig
              .connect(owner)
              .updateKmsContext(
                nextKmsContextId,
                kmsNodes,
                mpcThreshold,
                nullPublicDecryptionThreshold,
                userDecryptionThreshold,
                kmsGenThreshold,
              ),
          ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullPublicDecryptionThreshold");
        });

        it("Should revert because the public decryption threshold is too high", async function () {
          // The public decryption threshold must be less or equal to the number of KMS nodes
          await expect(
            gatewayConfig
              .connect(owner)
              .updateKmsContext(
                nextKmsContextId,
                kmsNodes,
                mpcThreshold,
                highPublicDecryptionThreshold,
                userDecryptionThreshold,
                kmsGenThreshold,
              ),
          )
            .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighPublicDecryptionThreshold")
            .withArgs(highPublicDecryptionThreshold, nKmsNodes);
        });

        it("Should revert because the user decryption threshold is null", async function () {
          await expect(
            gatewayConfig
              .connect(owner)
              .updateKmsContext(
                nextKmsContextId,
                kmsNodes,
                mpcThreshold,
                publicDecryptionThreshold,
                nullUserDecryptionThreshold,
                kmsGenThreshold,
              ),
          ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullUserDecryptionThreshold");
        });

        it("Should revert because the user decryption threshold is too high", async function () {
          // The user decryption threshold must be less or equal to the number of KMS nodes
          await expect(
            gatewayConfig
              .connect(owner)
              .updateKmsContext(
                nextKmsContextId,
                kmsNodes,
                mpcThreshold,
                publicDecryptionThreshold,
                highUserDecryptionThreshold,
                kmsGenThreshold,
              ),
          )
            .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighUserDecryptionThreshold")
            .withArgs(highUserDecryptionThreshold, nKmsNodes);
        });

        it("Should revert because the KMS generation threshold is null", async function () {
          await expect(
            gatewayConfig
              .connect(owner)
              .updateKmsContext(
                nextKmsContextId,
                kmsNodes,
                mpcThreshold,
                publicDecryptionThreshold,
                userDecryptionThreshold,
                nullKmsGenThreshold,
              ),
          ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullKmsGenThreshold");
        });

        it("Should revert because the KMS generation threshold is too high", async function () {
          // The KMS generation threshold must be less or equal to the number of KMS nodes
          await expect(
            gatewayConfig
              .connect(owner)
              .updateKmsContext(
                nextKmsContextId,
                kmsNodes,
                mpcThreshold,
                publicDecryptionThreshold,
                userDecryptionThreshold,
                highKmsGenThreshold,
              ),
          )
            .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighKmsGenThreshold")
            .withArgs(highKmsGenThreshold, nKmsNodes);
        });

        it("Should revert because the contextId is already registered", async function () {
          // initialKmsContextId is already registered from initialization
          const newKmsNode: KmsNodeStruct = {
            txSenderAddress: newTxSenderAddress,
            signerAddress: newSignerAddress,
            ipAddress: "127.0.0.100",
            storageUrl: "s3://kms-bucket-100",
          };

          await expect(gatewayConfig.connect(owner).updateKmsContext(initialKmsContextId, [newKmsNode], 0, 1, 1, 1))
            .to.be.revertedWithCustomError(gatewayConfig, "KmsContextAlreadyRegistered")
            .withArgs(initialKmsContextId, initialKmsContextId);
        });

        it("Should revert because the contextId is zero", async function () {
          const newKmsNode: KmsNodeStruct = {
            txSenderAddress: newTxSenderAddress,
            signerAddress: newSignerAddress,
            ipAddress: "127.0.0.100",
            storageUrl: "s3://kms-bucket-100",
          };

          await expect(
            gatewayConfig.connect(owner).updateKmsContext(0, [newKmsNode], 0, 1, 1, 1),
          ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullKmsContextId");
        });

        it("Should preserve initial context after registering a new one", async function () {
          const newKmsNode: KmsNodeStruct = {
            txSenderAddress: newTxSenderAddress,
            signerAddress: newSignerAddress,
            ipAddress: "127.0.0.100",
            storageUrl: "s3://kms-bucket-100",
          };

          await gatewayConfig.connect(owner).updateKmsContext(nextKmsContextId, [newKmsNode], 0, 1, 1, 1);

          // Initial context should still have the original nodes
          expect(await gatewayConfig.getKmsContextSigners(initialKmsContextId)).to.have.lengthOf(kmsSigners.length);
          for (const kmsSigner of kmsSigners) {
            expect(await gatewayConfig.isKmsContextSigner(initialKmsContextId, kmsSigner.address)).to.be.true;
          }
          for (const kmsTxSender of kmsTxSenders) {
            expect(await gatewayConfig.isKmsContextTxSender(initialKmsContextId, kmsTxSender.address)).to.be.true;
          }

          // New context should have the new node
          expect(await gatewayConfig.isKmsContextSigner(nextKmsContextId, newSignerAddress)).to.be.true;
          expect(await gatewayConfig.isKmsContextTxSender(nextKmsContextId, newTxSenderAddress)).to.be.true;

          // Active context should be updated
          expect(await gatewayConfig.getCurrentKmsContextId()).to.equal(nextKmsContextId);

          // Initial context should have original thresholds
          expect(await gatewayConfig.getKmsContextPublicDecryptionThreshold(initialKmsContextId)).to.equal(
            publicDecryptionThreshold,
          );
          expect(await gatewayConfig.getKmsContextUserDecryptionThreshold(initialKmsContextId)).to.equal(
            userDecryptionThreshold,
          );

          // New context should have the new thresholds
          expect(await gatewayConfig.getKmsContextPublicDecryptionThreshold(nextKmsContextId)).to.equal(1);
          expect(await gatewayConfig.getKmsContextUserDecryptionThreshold(nextKmsContextId)).to.equal(1);
        });
      });

      describe("Coprocessors updates", function () {
        it("Should update the coprocessors", async function () {
          const newCoprocessor: CoprocessorStruct = {
            txSenderAddress: newTxSenderAddress,
            signerAddress: newSignerAddress,
            s3BucketUrl: "s3://coprocessor-bucket-1000",
          };
          const newCoprocessors: CoprocessorStruct[] = [newCoprocessor];
          const newCoprocessorThreshold = 1;

          const tx = await gatewayConfig.connect(owner).updateCoprocessors(newCoprocessors, newCoprocessorThreshold);

          await expect(tx)
            .to.emit(gatewayConfig, "UpdateCoprocessors")
            .withArgs(toValues(newCoprocessors), newCoprocessorThreshold);

          // Check that the KMS nodes have been updated
          expect(await gatewayConfig.isCoprocessorTxSender(newTxSenderAddress)).to.be.true;
          expect(await gatewayConfig.isCoprocessorSigner(newSignerAddress)).to.be.true;
          expect(await gatewayConfig.getCoprocessor(newTxSenderAddress)).to.deep.equal(toValues(newCoprocessor));
          expect(await gatewayConfig.getCoprocessorTxSenders()).to.deep.equal([newTxSenderAddress]);
          expect(await gatewayConfig.getCoprocessorSigners()).to.deep.equal([newSignerAddress]);

          // Check that the threshold have been updated
          expect(await gatewayConfig.getCoprocessorMajorityThreshold()).to.equal(newCoprocessorThreshold);

          // Define the null coprocessor
          const nullCoprocessor: CoprocessorStruct = {
            txSenderAddress: ZeroAddress,
            signerAddress: ZeroAddress,
            s3BucketUrl: "",
          };

          // Check that old coprocessors have been removed
          for (const coprocessorSigner of coprocessorSigners) {
            expect(await gatewayConfig.isCoprocessorSigner(coprocessorSigner)).to.be.false;
          }
          for (const coprocessorTxSender of coprocessorTxSenders) {
            expect(await gatewayConfig.isCoprocessorTxSender(coprocessorTxSender)).to.be.false;
            expect(await gatewayConfig.getCoprocessor(coprocessorTxSender)).to.deep.equal(toValues(nullCoprocessor));
          }
        });

        it("Should revert because the sender is not the owner", async function () {
          await expect(gatewayConfig.connect(fakeOwner).updateCoprocessors(emptyCoprocessors, nullCoprocessorThreshold))
            .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
            .withArgs(fakeOwner.address);
        });

        it("Should revert because the coprocessors are empty", async function () {
          await expect(
            gatewayConfig.connect(owner).updateCoprocessors(emptyCoprocessors, nullCoprocessorThreshold),
          ).to.be.revertedWithCustomError(gatewayConfig, "EmptyCoprocessors");
        });

        it("Should revert because the coprocessor threshold is null", async function () {
          await expect(
            gatewayConfig.connect(owner).updateCoprocessors(coprocessors, nullCoprocessorThreshold),
          ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullCoprocessorThreshold");
        });

        it("Should revert because the coprocessor threshold is too high", async function () {
          // The coprocessor threshold must be less or equal to the number of coprocessors
          await expect(gatewayConfig.connect(owner).updateCoprocessors(coprocessors, highCoprocessorThreshold))
            .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighCoprocessorThreshold")
            .withArgs(highCoprocessorThreshold, nCoprocessors);
        });
      });

      describe("Custodians updates", function () {
        it("Should update the custodians", async function () {
          const newCustodian: CustodianStruct = {
            txSenderAddress: newTxSenderAddress,
            signerAddress: newSignerAddress,
            encryptionKey: createByteInput(),
          };
          const newCustodians: CustodianStruct[] = [newCustodian];

          const tx = await gatewayConfig.connect(owner).updateCustodians(newCustodians);

          await expect(tx).to.emit(gatewayConfig, "UpdateCustodians").withArgs(toValues(newCustodians));

          // Check that the custodians have been updated
          expect(await gatewayConfig.isCustodianTxSender(newTxSenderAddress)).to.be.true;
          expect(await gatewayConfig.isCustodianSigner(newSignerAddress)).to.be.true;
          expect(await gatewayConfig.getCustodian(newTxSenderAddress)).to.deep.equal(toValues(newCustodian));
          expect(await gatewayConfig.getCustodianTxSenders()).to.deep.equal([newTxSenderAddress]);
          expect(await gatewayConfig.getCustodianSigners()).to.deep.equal([newSignerAddress]);

          // Define the null custodian
          const nullCustodian: CustodianStruct = {
            txSenderAddress: ZeroAddress,
            signerAddress: ZeroAddress,
            encryptionKey: "0x",
          };

          // Check that old custodians have been removed
          for (const custodianSigner of custodianSigners) {
            expect(await gatewayConfig.isCustodianSigner(custodianSigner)).to.be.false;
          }

          for (const custodianTxSender of custodianTxSenders) {
            expect(await gatewayConfig.isCustodianTxSender(custodianTxSender)).to.be.false;
            expect(await gatewayConfig.getCustodian(custodianTxSender)).to.deep.equal(toValues(nullCustodian));
          }
        });

        it("Should revert because the sender is not the owner", async function () {
          await expect(gatewayConfig.connect(fakeOwner).updateCustodians(emptyCustodians))
            .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
            .withArgs(fakeOwner.address);
        });

        it("Should revert because the custodians are empty", async function () {
          await expect(gatewayConfig.connect(owner).updateCustodians(emptyCustodians));
        });
      });
    });

    describe("GatewayConfig initialization getters", function () {
      it("Should be registered as KMS nodes transaction senders", async function () {
        for (const kmsTxSender of kmsTxSenders) {
          expect(await gatewayConfig.isKmsTxSender(kmsTxSender.address)).to.be.true;
        }
      });

      it("Should be registered as KMS nodes signers", async function () {
        for (const kmsSigner of kmsSigners) {
          expect(await gatewayConfig.isKmsSigner(kmsSigner.address)).to.be.true;
        }
      });

      it("Should be registered as coprocessors transaction senders", async function () {
        for (const coprocessorTxSender of coprocessorTxSenders) {
          expect(await gatewayConfig.isCoprocessorTxSender(coprocessorTxSender.address)).to.be.true;
        }
      });

      it("Should not be registered as coprocessors transaction senders", async function () {
        expect(await gatewayConfig.isCoprocessorTxSender(fakeTxSender)).to.be.false;
      });

      it("Should be registered as coprocessors signers", async function () {
        for (const coprocessorSigner of coprocessorSigners) {
          expect(await gatewayConfig.isCoprocessorSigner(coprocessorSigner.address)).to.be.true;
        }
      });

      it("Should not be registered as coprocessors signers", async function () {
        expect(await gatewayConfig.isCoprocessorSigner(fakeSigner)).to.be.false;
      });

      it("Should be registered as custodian transaction senders", async function () {
        for (const custodianTxSender of custodianTxSenders) {
          expect(await gatewayConfig.isCustodianTxSender(custodianTxSender.address)).to.be.true;
        }
      });

      it("Should not be registered as custodian transaction senders", async function () {
        expect(await gatewayConfig.isCustodianTxSender(fakeTxSender)).to.be.false;
      });

      it("Should be registered as custodian signers", async function () {
        for (const custodianSigner of custodianSigners) {
          expect(await gatewayConfig.isCustodianSigner(custodianSigner.address)).to.be.true;
        }
      });

      it("Should be registered as custodian signers", async function () {
        expect(await gatewayConfig.isCustodianSigner(fakeSigner)).to.be.false;
      });

      it("Should be registered as host chains", async function () {
        for (const hostChainId of hostChainIds) {
          expect(await gatewayConfig.isHostChainRegistered(hostChainId)).to.be.true;
        }
      });

      it("Should be registered as pauser", async function () {
        expect(await gatewayConfig.isPauser(pauser.address)).to.be.true;
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
        const currentContextId = await gatewayConfig.getCurrentKmsContextId();
        expect(await gatewayConfig.getKmsContextPublicDecryptionThreshold(currentContextId)).to.equal(
          newPublicDecryptionThreshold,
        );
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
        const currentContextId = await gatewayConfig.getCurrentKmsContextId();
        expect(await gatewayConfig.getKmsContextUserDecryptionThreshold(currentContextId)).to.equal(
          newUserDecryptionThreshold,
        );
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

    describe("Update KMS generation threshold", function () {
      it("Should revert because the sender is not the owner", async function () {
        await expect(gatewayConfig.connect(fakeOwner).updateKmsGenThreshold(1))
          .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
          .withArgs(fakeOwner.address);
      });

      it("Should update the KMS generation threshold", async function () {
        // The KMS generation threshold must be greater than 0
        const newKmsGenThreshold = 1;

        const tx = await gatewayConfig.connect(owner).updateKmsGenThreshold(newKmsGenThreshold);

        await expect(tx).to.emit(gatewayConfig, "UpdateKmsGenThreshold").withArgs(newKmsGenThreshold);

        // Check that the KMS generation threshold has been updated
        expect(await gatewayConfig.getKmsGenThreshold()).to.equal(newKmsGenThreshold);
      });

      it("Should revert because the KMS generation threshold is null", async function () {
        // The KMS generation threshold must be greater than 0
        const nullKmsGenThreshold = 0;

        await expect(
          gatewayConfig.connect(owner).updateKmsGenThreshold(nullKmsGenThreshold),
        ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullKmsGenThreshold");
      });

      it("Should revert because the KMS generation threshold is too high", async function () {
        // The KMS generation threshold must be less or equal to the number of KMS nodes
        const highKmsGenThreshold = nKmsNodes + 1;

        await expect(gatewayConfig.connect(owner).updateKmsGenThreshold(highKmsGenThreshold))
          .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighKmsGenThreshold")
          .withArgs(highKmsGenThreshold, nKmsNodes);
      });
    });

    describe("Update coprocessor threshold", function () {
      it("Should revert because the sender is not the owner", async function () {
        await expect(gatewayConfig.connect(fakeOwner).updateCoprocessorThreshold(1))
          .to.be.revertedWithCustomError(gatewayConfig, "OwnableUnauthorizedAccount")
          .withArgs(fakeOwner.address);
      });

      it("Should update the coprocessor threshold", async function () {
        // The coprocessor threshold must be greater than 0
        const newCoprocessorThreshold = 1;

        const tx = await gatewayConfig.connect(owner).updateCoprocessorThreshold(newCoprocessorThreshold);

        await expect(tx).to.emit(gatewayConfig, "UpdateCoprocessorThreshold").withArgs(newCoprocessorThreshold);

        // Check that the coprocessor threshold has been updated
        expect(await gatewayConfig.getCoprocessorMajorityThreshold()).to.equal(newCoprocessorThreshold);
      });

      it("Should revert because the coprocessor threshold is null", async function () {
        // The coprocessor threshold must be greater than 0
        const nullCoprocessorThreshold = 0;

        await expect(
          gatewayConfig.connect(owner).updateCoprocessorThreshold(nullCoprocessorThreshold),
        ).to.be.revertedWithCustomError(gatewayConfig, "InvalidNullCoprocessorThreshold");
      });

      it("Should revert because the coprocessor threshold is too high", async function () {
        // The coprocessor threshold must be less or equal to the number of coprocessors
        const highCoprocessorThreshold = nCoprocessors + 1;

        await expect(gatewayConfig.connect(owner).updateCoprocessorThreshold(highCoprocessorThreshold))
          .to.be.revertedWithCustomError(gatewayConfig, "InvalidHighCoprocessorThreshold")
          .withArgs(highCoprocessorThreshold, nCoprocessors);
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

    describe("Pause all gateway contracts", function () {
      let decryption: Decryption;
      let inputVerification: InputVerification;

      before(async function () {
        const fixtureData = await loadFixture(loadTestVariablesFixture);
        decryption = fixtureData.decryption;
        inputVerification = fixtureData.inputVerification;
      });

      it("Should pause all the Gateway contracts with the pauser", async function () {
        // Check that the contracts are not paused
        expect(await decryption.paused()).to.be.false;
        expect(await inputVerification.paused()).to.be.false;

        const txResponse = await gatewayConfig.connect(pauser).pauseAllGatewayContracts();

        await expect(txResponse).to.emit(gatewayConfig, "PauseAllGatewayContracts");

        // Check that the pausable contracts are paused
        expect(await decryption.paused()).to.be.true;
        expect(await inputVerification.paused()).to.be.true;
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
        expect(await decryption.paused()).to.be.false;
        expect(await inputVerification.paused()).to.be.false;
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
