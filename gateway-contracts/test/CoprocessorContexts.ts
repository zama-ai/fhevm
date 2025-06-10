import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture, time } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ContractFactory, Wallet } from "ethers";
import hre from "hardhat";

import { CoprocessorContexts, EmptyUUPSProxy } from "../typechain-types";
// The type needs to be imported separately because it is not properly detected by the linter
// as this type is defined as a shared structs instead of directly in the IDecryption interface
import {
  CoprocessorContextStruct,
  CoprocessorContextTimePeriodsStruct,
  CoprocessorStruct,
} from "../typechain-types/contracts/interfaces/ICoprocessorContexts";
import {
  ContextStatus,
  addNewCoprocessorContext,
  createCoprocessors,
  createRandomWallet,
  loadTestVariablesFixture,
  refreshCoprocessorContextAfterTimePeriod,
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

      it("Should get all coprocessor transaction sender addresses from the context", async function () {
        const coprocessorTxSenderAddresses = await coprocessorContexts.getCoprocessorTxSendersFromContext(contextId);

        // Check that the number of coprocessor transaction sender addresses is correct
        expect(coprocessorTxSenderAddresses.length).to.equal(coprocessorTxSenders.length);

        // Check that all coprocessor transaction sender addresses are in the list
        for (const coprocessorTxSender of coprocessorTxSenders) {
          expect(coprocessorTxSenderAddresses).to.include(coprocessorTxSender.address);
        }
      });

      it("Should get all coprocessor signer addresses from the context", async function () {
        const coprocessorSignerAddresses = await coprocessorContexts.getCoprocessorSignersFromContext(contextId);

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
        let timePeriods: CoprocessorContextTimePeriodsStruct;

        // Define the new expected context ID
        const newContextId = 2;

        beforeEach(async function () {
          // Add a new coprocessor context
          const newCoprocessorContext = await addNewCoprocessorContext(3, coprocessorContexts, owner);
          timePeriods = newCoprocessorContext.timePeriods;
        });

        it("Should get the pre-activation context ID", async function () {
          const preActivationContextId = await coprocessorContexts.getPreActivationCoprocessorContextId();

          // Check that the pre-activation context ID matches the new context ID
          expect(preActivationContextId).to.equal(newContextId);
        });

        it("Should get the suspended context ID", async function () {
          // Wait for the pre activation period to pass
          await refreshCoprocessorContextAfterTimePeriod(timePeriods.preActivationTimePeriod, coprocessorContexts);

          const suspendedContextId = await coprocessorContexts.getSuspendedCoprocessorContextId();

          // Check that the suspended context ID matches the old context ID
          expect(suspendedContextId).to.equal(contextId);
        });

        it("Should get the new active context ID", async function () {
          // Wait for the pre activation period to pass
          await refreshCoprocessorContextAfterTimePeriod(timePeriods.preActivationTimePeriod, coprocessorContexts);

          // Wait for the suspended period to pass
          await refreshCoprocessorContextAfterTimePeriod(timePeriods.suspendedTimePeriod, coprocessorContexts);

          const activeContextId = await coprocessorContexts.getActiveCoprocessorContextId();

          // Check that the active context ID matches the new context ID
          expect(activeContextId).to.equal(newContextId);
        });

        it("Should get the new context activation block timestamp", async function () {
          // Get the latest block timestamp
          const latestBlockTimestamp = await time.latest();

          const activationBlockTimestamp =
            await coprocessorContexts.getCoprocessorActivationBlockTimestampFromContext(newContextId);

          const expectedActivationBlockTimestamp =
            BigInt(latestBlockTimestamp) + BigInt(timePeriods.preActivationTimePeriod);

          expect(activationBlockTimestamp).to.equal(expectedActivationBlockTimestamp);
        });

        it("Should get the old context suspended block timestamp", async function () {
          // Wait for the pre activation period to pass
          await refreshCoprocessorContextAfterTimePeriod(timePeriods.preActivationTimePeriod, coprocessorContexts);

          // Get the latest block timestamp
          const latestBlockTimestamp = await time.latest();

          const deactivatedBlockTimestamp =
            await coprocessorContexts.getCoprocessorDeactivatedBlockTimestampFromContext(contextId);

          const expectedDeactivatedBlockTimestamp =
            BigInt(latestBlockTimestamp) + BigInt(timePeriods.suspendedTimePeriod);

          expect(deactivatedBlockTimestamp).to.equal(expectedDeactivatedBlockTimestamp);
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
        await refreshCoprocessorContextAfterTimePeriod(
          newCoprocessorContext.timePeriods.preActivationTimePeriod,
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

      // Define new time periods
      const newPreActivationTimePeriod = 100;
      const newSuspendedTimePeriod = 100;
      const newTimePeriods: CoprocessorContextTimePeriodsStruct = {
        preActivationTimePeriod: newPreActivationTimePeriod,
        suspendedTimePeriod: newSuspendedTimePeriod,
      };

      // Create a new set of coprocessors
      const { coprocessors: newCoprocessors } = createCoprocessors(7);

      it("Should add a new coprocessor context", async function () {
        // Get the latest block timestamp
        const latestBlockTimestamp = await time.latest();

        // Add a new coprocessor context
        const txResult = await coprocessorContexts
          .connect(owner)
          .addCoprocessorContext(newFeatureSet, newCoprocessors, newTimePeriods);

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

        const expectedActivationBlockTimestamp = BigInt(latestBlockTimestamp) + BigInt(newPreActivationTimePeriod);

        expect(txResult)
          .to.emit(coprocessorContexts, "NewCoprocessorContext")
          .withArgs(oldCoprocessorContext, newCoprocessorContext, newTimePeriods)
          .to.emit(coprocessorContexts, "PreActivateCoprocessorContext")
          .withArgs(newCoprocessorContext, expectedActivationBlockTimestamp);

        // Check that the new context is in the pre-activation state
        expect(await coprocessorContexts.getCoprocessorContextStatus(newContextId)).to.equal(
          ContextStatus.PreActivation,
        );
      });

      it("Should revert because there is a coprocessor context in pre-activation", async function () {
        // Add a new coprocessor context
        await addNewCoprocessorContext(3, coprocessorContexts, owner);

        await expect(
          coprocessorContexts.connect(owner).addCoprocessorContext(newFeatureSet, newCoprocessors, newTimePeriods),
        )
          .to.be.revertedWithCustomError(coprocessorContexts, "PreActivationContextOngoing")
          .withArgs(newContextId);
      });

      it("Should revert because there is a suspended coprocessor context ongoing", async function () {
        // Add a new coprocessor context
        const newCoprocessorContext = await addNewCoprocessorContext(3, coprocessorContexts, owner);

        // Wait for the pre activation period to pass
        await refreshCoprocessorContextAfterTimePeriod(
          newCoprocessorContext.timePeriods.preActivationTimePeriod,
          coprocessorContexts,
        );

        await expect(
          coprocessorContexts.connect(owner).addCoprocessorContext(newFeatureSet, newCoprocessors, newTimePeriods),
        )
          .to.be.revertedWithCustomError(coprocessorContexts, "SuspendedContextOngoing")
          .withArgs(contextId);
      });
    });

    describe("Context status changes", function () {
      let newTimePeriods: CoprocessorContextTimePeriodsStruct;

      // Define the new expected context ID
      const newContextId = 2;

      beforeEach(async function () {
        // Add a new coprocessor context
        const newCoprocessorContext = await addNewCoprocessorContext(3, coprocessorContexts, owner);
        newTimePeriods = newCoprocessorContext.timePeriods;
      });

      it("Should activate the new context and suspend the old one", async function () {
        // Increase the block timestamp to reach the end of the pre-activation period
        await time.increase(newTimePeriods.preActivationTimePeriod);

        // Get the latest block timestamp
        const latestBlockTimestamp = await time.latest();

        // Refresh the statuses of the coprocessor contexts: this suspends the old context and activates the new one
        const txResult = await coprocessorContexts.refreshCoprocessorContextStatuses();

        const expectedDeactivatedBlockTimestamp =
          BigInt(latestBlockTimestamp) + BigInt(newTimePeriods.suspendedTimePeriod);

        expect(txResult)
          .to.emit(coprocessorContexts, "SuspendCoprocessorContext")
          .withArgs(contextId, expectedDeactivatedBlockTimestamp)
          .to.emit(coprocessorContexts, "ActivateCoprocessorContext")
          .withArgs(newContextId);

        // Make sure the old context has been suspended
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Suspended);

        // Make sure the new context has been activated
        expect(await coprocessorContexts.getCoprocessorContextStatus(newContextId)).to.equal(ContextStatus.Active);
      });

      it("Should deactivate the suspended context", async function () {
        // Increase the block timestamp to reach the end of the pre-activation period
        await time.increase(newTimePeriods.preActivationTimePeriod);

        // Refresh the statuses of the coprocessor contexts: this suspends the old context
        await coprocessorContexts.refreshCoprocessorContextStatuses();

        // Increase the block timestamp to reach the end of the suspended period
        await time.increase(newTimePeriods.suspendedTimePeriod);

        // Refresh the statuses of the coprocessor contexts once again: this deactivates the old context
        const txResult = await coprocessorContexts.refreshCoprocessorContextStatuses();

        expect(txResult).to.emit(coprocessorContexts, "DeactivateCoprocessorContext").withArgs(contextId);

        // Make sure the old context has been deactivated
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Deactivated);
      });

      it("Should compromise the suspended context", async function () {
        // Increase the block timestamp to reach the end of the pre-activation period
        await time.increase(newTimePeriods.preActivationTimePeriod);

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
        // Increase the block timestamp to reach the end of the pre-activation period
        await time.increase(newTimePeriods.preActivationTimePeriod);

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
        // Increase the block timestamp to reach the end of the pre-activation period
        await time.increase(newTimePeriods.preActivationTimePeriod);

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
