import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture, mine } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ContractFactory, Wallet } from "ethers";
import hre from "hardhat";

import { CoprocessorContexts, EmptyUUPSProxy } from "../typechain-types";
// The type needs to be imported separately because it is not properly detected by the linter
// as this type is defined as a shared structs instead of directly in the IDecryption interface
import {
  CoprocessorContextBlockPeriodsStruct,
  CoprocessorContextStruct,
  CoprocessorStruct,
} from "../typechain-types/contracts/interfaces/ICoprocessorContexts";
import {
  ContextStatus,
  addNewCoprocessorContext,
  createCoprocessors,
  createRandomWallet,
  loadTestVariablesFixture,
  refreshCoprocessorContextAfterBlockPeriod,
  toValues,
} from "./utils";

describe("CoprocessorContexts", function () {
  // Define input values
  const featureSet = 1;

  // Define fake values
  const fakeTxSender = createRandomWallet();
  const fakeSigner = createRandomWallet();
  const nullAddress = hre.ethers.ZeroAddress;

  let coprocessorContexts: CoprocessorContexts;
  let owner: Wallet;
  let coprocessors: CoprocessorStruct[];
  let coprocessorTxSenders: HardhatEthersSigner[];
  let coprocessorSigners: HardhatEthersSigner[];

  async function getInputsForDeployFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    return fixtureData;
  }

  before(async function () {
    // Initialize globally used variables before each test
    const fixtureData = await loadFixture(getInputsForDeployFixture);
    coprocessorContexts = fixtureData.coprocessorContexts;
    owner = fixtureData.owner;
    coprocessors = fixtureData.coprocessors;
    coprocessorTxSenders = fixtureData.coprocessorTxSenders;
    coprocessorSigners = fixtureData.coprocessorSigners;
  });

  describe("Deployment", function () {
    let proxyContract: EmptyUUPSProxy;
    let newCoprocessorContextsFactory: ContractFactory;

    beforeEach(async function () {
      // Deploy a new proxy contract
      const proxyImplementation = await hre.ethers.getContractFactory("EmptyUUPSProxy", owner);
      proxyContract = await hre.upgrades.deployProxy(proxyImplementation, [owner.address], {
        initializer: "initialize",
        kind: "uups",
      });
      await proxyContract.waitForDeployment();

      // Get the CoprocessorContexts contract factory
      newCoprocessorContextsFactory = await hre.ethers.getContractFactory("CoprocessorContexts", owner);
    });

    it("Should revert because the coprocessors list is empty", async function () {
      const emptyCoprocessors: CoprocessorStruct[] = [];
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newCoprocessorContextsFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [featureSet, emptyCoprocessors],
          },
        }),
      ).to.be.revertedWithCustomError(coprocessorContexts, "EmptyCoprocessors");
    });

    it("Should revert because a coprocessor has a null transaction sender address", async function () {
      const coprocessorsWithNullTxSender: CoprocessorStruct[] = [
        {
          name: "Coprocessor 1",
          txSenderAddress: nullAddress,
          signerAddress: coprocessorSigners[0].address,
          s3BucketUrl: "https://s3.amazonaws.com/bucket/key",
        },
      ];
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newCoprocessorContextsFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [featureSet, coprocessorsWithNullTxSender],
          },
        }),
      ).to.be.revertedWithCustomError(coprocessorContexts, "NullCoprocessorTxSenderAddress");
    });

    it("Should revert because a coprocessor has a null signer address", async function () {
      const coprocessorsWithNullSigner: CoprocessorStruct[] = [
        {
          name: "Coprocessor 1",
          txSenderAddress: coprocessorTxSenders[0].address,
          signerAddress: nullAddress,
          s3BucketUrl: "https://s3.amazonaws.com/bucket/key",
        },
      ];
      await expect(
        hre.upgrades.upgradeProxy(proxyContract, newCoprocessorContextsFactory, {
          call: {
            fn: "initializeFromEmptyProxy",
            args: [featureSet, coprocessorsWithNullSigner],
          },
        }),
      ).to.be.revertedWithCustomError(coprocessorContexts, "NullCoprocessorSignerAddress");
    });
  });

  describe("After deployment", function () {
    // Define the first context ID
    const contextId = 1;

    beforeEach(async function () {
      const fixture = await loadFixture(loadTestVariablesFixture);
      coprocessorContexts = fixture.coprocessorContexts;
      coprocessorTxSenders = fixture.coprocessorTxSenders;
    });

    describe("Getters", function () {
      it("Should revert because there is no pre-activation context ID", async function () {
        await expect(coprocessorContexts.getPreActivationCoprocessorContextId()).to.be.revertedWithCustomError(
          coprocessorContexts,
          "NoPreActivationCoprocessorContext",
        );
      });

      it("Should revert because there is no suspended context ID", async function () {
        await expect(coprocessorContexts.getSuspendedCoprocessorContextId()).to.be.revertedWithCustomError(
          coprocessorContexts,
          "NoSuspendedCoprocessorContext",
        );
      });

      it("Should get the active context ID", async function () {
        const activeContextId = await coprocessorContexts.getActiveCoprocessorContextId();

        // Check that the active context ID matches the first context ID
        expect(activeContextId).to.equal(contextId);
      });

      it("Should get the active coprocessor context", async function () {
        const activeCoprocessorContext = await coprocessorContexts.getActiveCoprocessorContext();

        const expectedActiveCoprocessorContext: CoprocessorContextStruct = {
          contextId,
          previousContextId: 0,
          featureSet,
          coprocessors: coprocessors,
        };

        expect(activeCoprocessorContext).to.deep.equal(toValues(expectedActiveCoprocessorContext));
      });

      it("Should get the coprocessor from the context", async function () {
        const coprocessor = await coprocessorContexts.getCoprocessorFromContext(
          contextId,
          coprocessorTxSenders[0].address,
        );

        expect(coprocessor).to.deep.equal(toValues(coprocessors[0]));
      });

      it("Should revert because coprocessor is not from the context", async function () {
        await expect(
          coprocessorContexts.getCoprocessorFromContext(contextId, fakeTxSender.address),
        ).to.be.revertedWithCustomError(coprocessorContexts, "NotCoprocessorFromContext");
      });

      it("Should revert because the context has not been initialized", async function () {
        // Define a fake context ID
        const fakeContextId = 1000;

        await expect(coprocessorContexts.getCoprocessorFromContext(fakeContextId, coprocessorTxSenders[0].address))
          .to.be.revertedWithCustomError(coprocessorContexts, "CoprocessorContextNotInitialized")
          .withArgs(fakeContextId);
      });

      it("Should get the coprocessor majority threshold from the context", async function () {
        const coprocessorMajorityThreshold =
          await coprocessorContexts.getCoprocessorMajorityThresholdFromContext(contextId);

        // The coprocessor majority threshold currently directly depends on the number of coprocessors
        expect(coprocessorMajorityThreshold).to.equal((coprocessorTxSenders.length >> 1) + 1);
      });

      it("Should get a coprocessor from the active context", async function () {
        const coprocessor = await coprocessorContexts.getCoprocessor(coprocessorTxSenders[0].address);

        expect(coprocessor).to.deep.equal(toValues(coprocessors[0]));
      });

      it("Should get all coprocessor transaction sender addresses", async function () {
        const coprocessorTxSenderAddresses = await coprocessorContexts.getCoprocessorTxSenders();

        // Check that the number of coprocessor transaction sender addresses is correct
        expect(coprocessorTxSenderAddresses.length).to.equal(coprocessorTxSenders.length);

        // Check that all coprocessor transaction sender addresses are in the list
        for (const coprocessorTxSender of coprocessorTxSenders) {
          expect(coprocessorTxSenderAddresses).to.include(coprocessorTxSender.address);
        }
      });

      it("Should get all coprocessor signer addresses", async function () {
        const coprocessorSignerAddresses = await coprocessorContexts.getCoprocessorSigners();

        // Check that the number of coprocessor signer addresses is correct
        expect(coprocessorSignerAddresses.length).to.equal(coprocessorSigners.length);

        // Check that all coprocessor signer addresses are in the list
        for (const coprocessorSigner of coprocessorSigners) {
          expect(coprocessorSignerAddresses).to.include(coprocessorSigner.address);
        }
      });

      it("Should get the coprocessor context status", async function () {
        const coprocessorContextStatus = await coprocessorContexts.getCoprocessorContextStatus(contextId);

        expect(coprocessorContextStatus).to.equal(ContextStatus.Active);
      });

      describe("Context changes", function () {
        let blockPeriods: CoprocessorContextBlockPeriodsStruct;

        // Define the new expected context ID
        const newContextId = 2;

        beforeEach(async function () {
          // Add a new coprocessor context
          const newCoprocessorContext = await addNewCoprocessorContext(3, coprocessorContexts, owner);
          blockPeriods = newCoprocessorContext.blockPeriods;
        });

        it("Should get the pre-activation context ID", async function () {
          const preActivationContextId = await coprocessorContexts.getPreActivationCoprocessorContextId();

          // Check that the pre-activation context ID matches the new context ID
          expect(preActivationContextId).to.equal(newContextId);
        });

        it("Should get the suspended context ID", async function () {
          // Wait for the pre activation period to pass
          await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.preActivationBlockPeriod, coprocessorContexts);

          const suspendedContextId = await coprocessorContexts.getSuspendedCoprocessorContextId();

          // Check that the suspended context ID matches the old context ID
          expect(suspendedContextId).to.equal(contextId);
        });

        it("Should get the new active context ID", async function () {
          // Wait for the pre activation period to pass
          await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.preActivationBlockPeriod, coprocessorContexts);

          // Wait for the suspended period to pass
          await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.suspendedBlockPeriod, coprocessorContexts);

          const activeContextId = await coprocessorContexts.getActiveCoprocessorContextId();

          // Check that the active context ID matches the new context ID
          expect(activeContextId).to.equal(newContextId);
        });

        it("Should get the new context activation block number", async function () {
          // Get the current block number
          const currentBlockNumber = await hre.ethers.provider.getBlockNumber();

          const activationBlockNumber =
            await coprocessorContexts.getCoprocessorContextActivationBlockNumber(newContextId);

          const expectedActivationBlockNumber =
            BigInt(currentBlockNumber) + BigInt(blockPeriods.preActivationBlockPeriod);

          expect(activationBlockNumber).to.equal(expectedActivationBlockNumber);
        });

        it("Should get the old context suspended block number", async function () {
          // Wait for the pre activation period to pass
          await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.preActivationBlockPeriod, coprocessorContexts);

          // Get the current block number
          const currentBlockNumber = await hre.ethers.provider.getBlockNumber();

          const deactivatedBlockNumber =
            await coprocessorContexts.getCoprocessorContextDeactivatedBlockNumber(contextId);

          const expectedDeactivatedBlockNumber = BigInt(currentBlockNumber) + BigInt(blockPeriods.suspendedBlockPeriod);

          expect(deactivatedBlockNumber).to.equal(expectedDeactivatedBlockNumber);
        });
      });
    });

    describe("Checks", function () {
      it("Should be registered as coprocessors transaction senders", async function () {
        for (const coprocessorTxSender of coprocessorTxSenders) {
          await expect(
            coprocessorContexts.checkIsCoprocessorTxSenderFromContext(contextId, coprocessorTxSender.address),
          ).to.not.be.reverted;
        }
      });

      it("Should revert because the address is not registered as a coprocessor transaction sender", async function () {
        await expect(coprocessorContexts.checkIsCoprocessorTxSenderFromContext(contextId, fakeTxSender.address))
          .to.be.revertedWithCustomError(coprocessorContexts, "NotCoprocessorTxSenderFromContext")
          .withArgs(contextId, fakeTxSender.address);
      });

      it("Should be registered as coprocessors signers", async function () {
        for (const coprocessorSigner of coprocessorSigners) {
          await expect(coprocessorContexts.checkIsCoprocessorSignerFromContext(contextId, coprocessorSigner.address)).to
            .not.be.reverted;
        }
      });

      it("Should revert because the address is not registered as a coprocessor signer", async function () {
        await expect(coprocessorContexts.checkIsCoprocessorSignerFromContext(contextId, fakeSigner.address))
          .to.be.revertedWithCustomError(coprocessorContexts, "NotCoprocessorSignerFromContext")
          .withArgs(contextId, fakeSigner.address);
      });
    });

    describe("Predicates", function () {
      it("Should return true as the context is active", async function () {
        // Make sure the context is active
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Active);

        const isActiveOrSuspended = await coprocessorContexts.isCoprocessorContextActiveOrSuspended(contextId);

        expect(isActiveOrSuspended).to.be.true;
      });

      it("Should return true as the context is suspended", async function () {
        // Add a new coprocessor context
        const newCoprocessorContext = await addNewCoprocessorContext(3, coprocessorContexts, owner);

        // Wait for the pre activation period to pass
        await refreshCoprocessorContextAfterBlockPeriod(
          newCoprocessorContext.blockPeriods.preActivationBlockPeriod,
          coprocessorContexts,
        );

        // Make sure the old context has been suspended
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Suspended);

        const isActiveOrSuspended = await coprocessorContexts.isCoprocessorContextActiveOrSuspended(contextId);

        expect(isActiveOrSuspended).to.be.true;
      });
    });

    describe("Add coprocessor context", function () {
      // Define new context ID
      const newContextId = 2;

      // Define new coprocessor context fields
      const newFeatureSet = 2030;

      // Define new block periods
      const newPreActivationBlockPeriod = 100;
      const newSuspendedBlockPeriod = 100;
      const newBlockPeriods: CoprocessorContextBlockPeriodsStruct = {
        preActivationBlockPeriod: newPreActivationBlockPeriod,
        suspendedBlockPeriod: newSuspendedBlockPeriod,
      };

      // Create a new set of coprocessors
      const { coprocessors: newCoprocessors } = createCoprocessors(7);

      it("Should add a new coprocessor context", async function () {
        // Get the current block number
        const currentBlockNumber = await hre.ethers.provider.getBlockNumber();

        // Add a new coprocessor context
        const txResult = await coprocessorContexts
          .connect(owner)
          .addCoprocessorContext(newFeatureSet, newCoprocessors, newBlockPeriods);

        const oldCoprocessorContext: CoprocessorContextStruct = {
          contextId,
          previousContextId: 0,
          featureSet,
          coprocessors: coprocessors,
        };

        const newCoprocessorContext: CoprocessorContextStruct = {
          contextId: newContextId,
          previousContextId: contextId,
          featureSet: newFeatureSet,
          coprocessors: newCoprocessors,
        };

        const expectedActivationBlockNumber = BigInt(currentBlockNumber) + BigInt(newPreActivationBlockPeriod);

        expect(txResult)
          .to.emit(coprocessorContexts, "NewCoprocessorContext")
          .withArgs(oldCoprocessorContext, newCoprocessorContext, newBlockPeriods)
          .to.emit(coprocessorContexts, "PreActivateCoprocessorContext")
          .withArgs(newCoprocessorContext, expectedActivationBlockNumber);

        // Check that the new context is in the pre-activation state
        expect(await coprocessorContexts.getCoprocessorContextStatus(newContextId)).to.equal(
          ContextStatus.PreActivation,
        );
      });

      it("Should revert because there is a coprocessor context in pre-activation", async function () {
        // Add a new coprocessor context
        await addNewCoprocessorContext(3, coprocessorContexts, owner);

        await expect(
          coprocessorContexts.connect(owner).addCoprocessorContext(newFeatureSet, newCoprocessors, newBlockPeriods),
        )
          .to.be.revertedWithCustomError(coprocessorContexts, "PreActivationContextOngoing")
          .withArgs(newContextId);
      });

      it("Should revert because there is a suspended coprocessor context ongoing", async function () {
        // Add a new coprocessor context
        const newCoprocessorContext = await addNewCoprocessorContext(3, coprocessorContexts, owner);

        // Wait for the pre activation period to pass
        await refreshCoprocessorContextAfterBlockPeriod(
          newCoprocessorContext.blockPeriods.preActivationBlockPeriod,
          coprocessorContexts,
        );

        await expect(
          coprocessorContexts.connect(owner).addCoprocessorContext(newFeatureSet, newCoprocessors, newBlockPeriods),
        )
          .to.be.revertedWithCustomError(coprocessorContexts, "SuspendedContextOngoing")
          .withArgs(contextId);
      });
    });

    describe("Context status changes", function () {
      let newBlockPeriods: CoprocessorContextBlockPeriodsStruct;

      // Define the new expected context ID
      const newContextId = 2;

      beforeEach(async function () {
        // Add a new coprocessor context
        const newCoprocessorContext = await addNewCoprocessorContext(3, coprocessorContexts, owner);
        newBlockPeriods = newCoprocessorContext.blockPeriods;
      });

      it("Should activate the new context and suspend the old one", async function () {
        // Mine the number of blocks required for the pre-activation period to pass
        await mine(newBlockPeriods.preActivationBlockPeriod);

        // Get the current block number
        const currentBlockNumber = await hre.ethers.provider.getBlockNumber();

        // Refresh the statuses of the coprocessor contexts: this suspends the old context and activates the new one
        const txResult = await coprocessorContexts.refreshCoprocessorContextStatuses();

        const expectedDeactivatedBlockNumber =
          BigInt(currentBlockNumber) + BigInt(newBlockPeriods.suspendedBlockPeriod);

        expect(txResult)
          .to.emit(coprocessorContexts, "SuspendCoprocessorContext")
          .withArgs(contextId, expectedDeactivatedBlockNumber)
          .to.emit(coprocessorContexts, "ActivateCoprocessorContext")
          .withArgs(newContextId);

        // Make sure the old context has been suspended
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Suspended);

        // Make sure the new context has been activated
        expect(await coprocessorContexts.getCoprocessorContextStatus(newContextId)).to.equal(ContextStatus.Active);
      });

      it("Should deactivate the suspended context", async function () {
        // Mine the number of blocks required for the pre-activation period to pass
        await mine(newBlockPeriods.preActivationBlockPeriod);

        // Refresh the statuses of the coprocessor contexts: this suspends the old context
        await coprocessorContexts.refreshCoprocessorContextStatuses();

        // Then mine the number of blocks required for the suspended period to pass
        await mine(newBlockPeriods.suspendedBlockPeriod);

        // Refresh the statuses of the coprocessor contexts once again: this deactivates the old context
        const txResult = await coprocessorContexts.refreshCoprocessorContextStatuses();

        expect(txResult).to.emit(coprocessorContexts, "DeactivateCoprocessorContext").withArgs(contextId);

        // Make sure the old context has been deactivated
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Deactivated);
      });

      it("Should compromise the suspended context", async function () {
        // Mine the number of blocks required for the pre-activation period to pass
        await mine(newBlockPeriods.preActivationBlockPeriod);

        // Refresh the statuses of the coprocessor contexts: this suspends the old context
        await coprocessorContexts.refreshCoprocessorContextStatuses();

        const txResult = await coprocessorContexts.connect(owner).compromiseCoprocessorContext(contextId);

        expect(txResult).to.emit(coprocessorContexts, "CompromiseCoprocessorContext").withArgs(contextId);

        // Make sure the context is compromised
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Compromised);
      });

      it("Should revert because an active context cannot be compromised", async function () {
        await expect(coprocessorContexts.connect(owner).compromiseCoprocessorContext(contextId))
          .to.be.revertedWithCustomError(coprocessorContexts, "ContextIsActive")
          .withArgs(contextId);
      });

      it("Should destroy the suspended context", async function () {
        // Mine the number of blocks required for the pre-activation period to pass
        await mine(newBlockPeriods.preActivationBlockPeriod);

        // Refresh the statuses of the coprocessor contexts: this suspends the old context
        await coprocessorContexts.refreshCoprocessorContextStatuses();

        const txResult = await coprocessorContexts.connect(owner).destroyCoprocessorContext(contextId);

        expect(txResult).to.emit(coprocessorContexts, "DestroyCoprocessorContext").withArgs(contextId);

        // Make sure the context is destroyed
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Destroyed);
      });

      it("Should revert because an active context cannot be destroyed", async function () {
        await expect(coprocessorContexts.connect(owner).destroyCoprocessorContext(contextId))
          .to.be.revertedWithCustomError(coprocessorContexts, "ContextIsActive")
          .withArgs(contextId);
      });

      it("Should move the suspended context to the active context", async function () {
        // Mine the number of blocks required for the pre-activation period to pass
        await mine(newBlockPeriods.preActivationBlockPeriod);

        // Refresh the statuses of the coprocessor contexts: this suspends the old context
        await coprocessorContexts.refreshCoprocessorContextStatuses();

        const txResult = await coprocessorContexts.connect(owner).moveSuspendedCoprocessorContextToActive();

        expect(txResult)
          .to.emit(coprocessorContexts, "DeactivateCoprocessorContext")
          .withArgs(newContextId)
          .to.emit(coprocessorContexts, "ActivateCoprocessorContext")
          .withArgs(contextId);

        // Make sure the old context has been reactivated
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Active);

        // Make sure the new context has been deactivated
        expect(await coprocessorContexts.getCoprocessorContextStatus(newContextId)).to.equal(ContextStatus.Deactivated);
      });

      it("Should revert because there is no suspended context to move to active", async function () {
        await expect(
          coprocessorContexts.connect(owner).moveSuspendedCoprocessorContextToActive(),
        ).to.be.revertedWithCustomError(coprocessorContexts, "NoSuspendedCoprocessorContext");
      });
    });
  });
});
