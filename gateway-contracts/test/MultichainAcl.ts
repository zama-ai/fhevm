import { HardhatEthersSigner, SignerWithAddress } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture, mine } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { HDNodeWallet, Wallet } from "ethers";
import hre from "hardhat";

import {
  CoprocessorContexts,
  GatewayConfig,
  InputVerification,
  MultichainAcl,
  MultichainAcl__factory,
} from "../typechain-types";
import { CoprocessorContextBlockPeriodsStruct } from "../typechain-types/contracts/interfaces/ICoprocessorContexts";
// The type needs to be imported separately because it is not properly detected by the linter
// as this type is defined as a shared structs instead of directly in the IMultichainAcl interface
import { DelegationAccountsStruct } from "../typechain-types/contracts/interfaces/IMultichainAcl";
import {
  ContextStatus,
  addNewCoprocessorContext,
  createCtHandle,
  createRandomAddress,
  createRandomAddresses,
  createRandomWallet,
  loadHostChainIds,
  loadTestVariablesFixture,
  refreshCoprocessorContextAfterBlockPeriod,
  toValues,
} from "./utils";

const MAX_CONTRACT_ADDRESSES = 10;

describe("MultichainAcl", function () {
  // Define the host chains' chain IDs
  const hostChainIds = loadHostChainIds();
  const hostChainId = hostChainIds[0];

  // Define the ctHandle (it will be allowed for public decryption or account access by default)
  const ctHandle = createCtHandle(hostChainId);

  // Define a new ctHandle (it won't be allowed for public decryption or account access by default)
  const newCtHandle = createCtHandle(hostChainId);

  // Define the first context ID
  const contextId = 1;

  // Define fake values
  const fakeHostChainId = 123;
  const ctHandleFakeChainId = createCtHandle(fakeHostChainId);
  const fakeTxSender = createRandomWallet();

  let gatewayConfig: GatewayConfig;
  let inputVerification: InputVerification;
  let coprocessorContexts: CoprocessorContexts;
  let multichainAcl: MultichainAcl;
  let coprocessorTxSenders: HardhatEthersSigner[];
  let owner: Wallet;
  let pauser: SignerWithAddress;

  beforeEach(async function () {
    // Initialize used global variables before each test
    const fixture = await loadFixture(loadTestVariablesFixture);
    gatewayConfig = fixture.gatewayConfig;
    inputVerification = fixture.inputVerification;
    coprocessorContexts = fixture.coprocessorContexts;
    multichainAcl = fixture.multichainAcl;
    coprocessorTxSenders = fixture.coprocessorTxSenders;
    owner = fixture.owner;
    pauser = fixture.pauser;
  });

  describe("Deployment", function () {
    let multichainAclFactory: MultichainAcl__factory;

    beforeEach(async function () {
      // Get the MultichainAcl contract factory
      multichainAclFactory = await hre.ethers.getContractFactory("MultichainAcl", owner);
    });

    it("Should revert because initialization is not from an empty proxy", async function () {
      await expect(
        hre.upgrades.upgradeProxy(multichainAcl, multichainAclFactory, {
          call: { fn: "initializeFromEmptyProxy" },
        }),
      ).to.be.revertedWithCustomError(multichainAcl, "NotInitializingFromEmptyProxy");
    });
  });

  describe("Allow account", async function () {
    // Define an account (it will be allowed to use the ciphertext by default)
    const accountAddress = createRandomAddress();

    // Define a new account (it will not be allowed to use the ciphertext by default)
    const newAccountAddress = createRandomAddress();

    beforeEach(async function () {
      // Allow the address to access the handle
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, accountAddress);
      }
    });

    it("Should revert because the hostChainId is not registered in the GatewayConfig contract", async function () {
      // Check that allowing an account to use a ciphertext on a fake chain ID reverts
      await expect(multichainAcl.connect(coprocessorTxSenders[0]).allowAccount(ctHandleFakeChainId, newAccountAddress))
        .revertedWithCustomError(gatewayConfig, "HostChainNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should allow account to use the ciphertext", async function () {
      // Trigger two allow calls with different coprocessor transaction senders
      await multichainAcl.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, newAccountAddress);
      const txResponse = multichainAcl.connect(coprocessorTxSenders[1]).allowAccount(ctHandle, newAccountAddress);

      // Check that the right event is emitted
      await expect(txResponse).to.emit(multichainAcl, "AllowAccount").withArgs(ctHandle, newAccountAddress);
    });

    it("Should revert because coprocessor tries to allow account twice", async function () {
      await expect(multichainAcl.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, accountAddress))
        .revertedWithCustomError(multichainAcl, "CoprocessorAlreadyAllowedAccount")
        .withArgs(ctHandle, accountAddress, coprocessorTxSenders[0].address);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(multichainAcl.connect(fakeTxSender).allowAccount(ctHandle, newAccountAddress))
        .revertedWithCustomError(coprocessorContexts, "NotCoprocessorTxSenderFromContext")
        .withArgs(contextId, fakeTxSender.address);
    });

    it("Should check account is allowed to use the ciphertext", async function () {
      await multichainAcl.connect(coprocessorTxSenders[0]).checkAccountAllowed(ctHandle, accountAddress);
    });

    it("Should revert because the account is not allowed to use the ciphertext", async function () {
      await expect(multichainAcl.connect(coprocessorTxSenders[0]).checkAccountAllowed(ctHandle, newAccountAddress))
        .to.be.revertedWithCustomError(multichainAcl, "AccountNotAllowedToUseCiphertext")
        .withArgs(ctHandle, newAccountAddress);
    });

    it("Should revert because the handle has not been allowed to be used by anyone", async function () {
      await expect(multichainAcl.connect(coprocessorTxSenders[0]).checkAccountAllowed(newCtHandle, accountAddress))
        .to.be.revertedWithCustomError(multichainAcl, "AccountNotAllowedToUseCiphertext")
        .withArgs(newCtHandle, accountAddress);
    });

    it("Should revert because the contract is paused", async function () {
      // Pause the contract
      await multichainAcl.connect(owner).pause();

      // Try calling paused allow account
      await expect(
        multichainAcl.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, newAccountAddress),
      ).to.be.revertedWithCustomError(multichainAcl, "EnforcedPause");
    });

    describe("Context changes", async function () {
      let blockPeriods: CoprocessorContextBlockPeriodsStruct;

      // Define the new expected context ID
      const newContextId = 2;

      beforeEach(async function () {
        // Allow the new account with the first coprocessor transaction sender. This should
        // register the request under the first active context (ID 1)
        await multichainAcl.connect(coprocessorTxSenders[0]).allowAccount(newCtHandle, accountAddress);

        // Add a new coprocessor context using a bigger set of coprocessors with different tx sender
        // and signer addresses
        const newCoprocessorContext = await addNewCoprocessorContext(10, coprocessorContexts, owner);
        blockPeriods = newCoprocessorContext.blockPeriods;
      });

      it("Should allow account with suspended context", async function () {
        // The second transaction should reach consensus and thus emit the expected event
        // This is because the consensus is reached amongst the suspended context (3 coprocessors)
        // and not the new one (10 coprocessors)
        const result = await multichainAcl.connect(coprocessorTxSenders[1]).allowAccount(newCtHandle, accountAddress);

        await expect(result).to.emit(multichainAcl, "AllowAccount").withArgs(newCtHandle, accountAddress);
      });

      it("Should revert because the context is no longer valid", async function () {
        // Wait for the pre activation period to pass
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.preActivationBlockPeriod, coprocessorContexts);

        // Wait for the suspended period to pass
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.suspendedBlockPeriod, coprocessorContexts);

        // Check that allow account request that has already been registered under an active context
        // reverts because this context is no longer valid
        await expect(multichainAcl.connect(coprocessorTxSenders[1]).allowAccount(newCtHandle, accountAddress))
          .revertedWithCustomError(multichainAcl, "InvalidCoprocessorContextAllowAccount")
          .withArgs(newCtHandle, accountAddress, contextId, ContextStatus.Deactivated);
      });

      it("Should revert because the transaction sender is a coprocessor from the suspended context", async function () {
        // Define another new handle
        // It is used to create a set of inputs different from the one used in the beforeEach block
        const newCtHandle2 = createCtHandle(hostChainId);

        // Wait for the pre activation period to pass
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.preActivationBlockPeriod, coprocessorContexts);

        // Make sure the old context has been suspended
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Suspended);

        // Make sure that a new account can't be allowed by a coprocessor from the suspended context
        await expect(multichainAcl.connect(coprocessorTxSenders[0]).allowAccount(newCtHandle2, accountAddress))
          .revertedWithCustomError(coprocessorContexts, "NotCoprocessorTxSenderFromContext")
          .withArgs(newContextId, coprocessorTxSenders[0].address);
      });
    });
  });

  describe("Allow public decrypt", async function () {
    beforeEach(async function () {
      // Allow the handle to be publicly decrypted
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await multichainAcl.connect(coprocessorTxSenders[i]).allowPublicDecrypt(ctHandle);
      }
    });

    it("Should revert because the hostChainId is not registered in the GatewayConfig contract", async function () {
      await expect(multichainAcl.connect(coprocessorTxSenders[0]).allowPublicDecrypt(ctHandleFakeChainId))
        .revertedWithCustomError(gatewayConfig, "HostChainNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should allow for public decryption", async function () {
      // Trigger two allow calls with different coprocessor transaction senders
      await multichainAcl.connect(coprocessorTxSenders[0]).allowPublicDecrypt(newCtHandle);
      const txResponse = multichainAcl.connect(coprocessorTxSenders[1]).allowPublicDecrypt(newCtHandle);

      // Check that the right event is emitted
      await expect(txResponse).to.emit(multichainAcl, "AllowPublicDecrypt").withArgs(newCtHandle);
    });

    it("Should revert because coprocessor tries to allow public decryption twice", async function () {
      await expect(multichainAcl.connect(coprocessorTxSenders[0]).allowPublicDecrypt(ctHandle))
        .revertedWithCustomError(multichainAcl, "CoprocessorAlreadyAllowedPublicDecrypt")
        .withArgs(ctHandle, coprocessorTxSenders[0].address);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(multichainAcl.connect(fakeTxSender).allowPublicDecrypt(newCtHandle))
        .revertedWithCustomError(coprocessorContexts, "NotCoprocessorTxSenderFromContext")
        .withArgs(contextId, fakeTxSender.address);
    });

    it("Should check public decrypt is allowed", async function () {
      await multichainAcl.connect(coprocessorTxSenders[0]).checkPublicDecryptAllowed(ctHandle);
    });

    it("Should revert because the handle is not allowed to be publicly decrypted", async function () {
      await expect(multichainAcl.connect(coprocessorTxSenders[0]).checkPublicDecryptAllowed(newCtHandle))
        .to.be.revertedWithCustomError(multichainAcl, "PublicDecryptNotAllowed")
        .withArgs(newCtHandle);
    });

    it("Should revert because the contract is paused", async function () {
      // Pause the contract
      await multichainAcl.connect(owner).pause();

      // Try calling paused allow public decrypt
      await expect(
        multichainAcl.connect(coprocessorTxSenders[0]).allowPublicDecrypt(ctHandle),
      ).to.be.revertedWithCustomError(multichainAcl, "EnforcedPause");
    });

    describe("Context changes", async function () {
      let blockPeriods: CoprocessorContextBlockPeriodsStruct;
      let newCoprocessorTxSenders: HDNodeWallet[];

      // Define the new expected context ID
      const newContextId = 2;

      beforeEach(async function () {
        // Allow a new handle for public decryption with the first coprocessor transaction sender. This should
        // register the request under the first active context (ID 1)
        await multichainAcl.connect(coprocessorTxSenders[0]).allowPublicDecrypt(newCtHandle);

        // Add a new coprocessor context using a bigger set of coprocessors with different tx sender
        // and signer addresses
        const newCoprocessorContext = await addNewCoprocessorContext(10, coprocessorContexts, owner, true);
        blockPeriods = newCoprocessorContext.blockPeriods;
        newCoprocessorTxSenders = newCoprocessorContext.coprocessorTxSenders;
      });

      it("Should activate the new context and suspend the old one", async function () {
        // Define another new handle. This is needed for the test to pass because `newCtHandle` is already
        // registered under the first active context (ID 1)
        const newCtHandle2 = createCtHandle(hostChainId);

        // Mine the number of blocks required for the pre-activation period to pass
        await mine(blockPeriods.preActivationBlockPeriod);

        // Allow a new handle with the first new coprocessor transaction sender
        await multichainAcl.connect(newCoprocessorTxSenders[0]).allowPublicDecrypt(newCtHandle2);

        // Make sure the old context has been suspended
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Suspended);

        // Make sure the new context has been activated
        expect(await coprocessorContexts.getCoprocessorContextStatus(newContextId)).to.equal(ContextStatus.Active);
      });

      it("Should deactivate the suspended context", async function () {
        // Define another new handle. This is needed for the test to pass because `newCtHandle` is already
        // registered under the first active context (ID 1)
        const newCtHandle2 = createCtHandle(hostChainId);

        // Mine the number of blocks required for the pre-activation period to pass
        await mine(blockPeriods.preActivationBlockPeriod);

        // Allow a new handle with the first new coprocessor transaction sender
        await multichainAcl.connect(newCoprocessorTxSenders[0]).allowPublicDecrypt(newCtHandle2);

        // Then mine the number of blocks required for the suspended period to pass
        await mine(blockPeriods.suspendedBlockPeriod);

        // Allow a new handle with the second new coprocessor transaction sender
        await multichainAcl.connect(newCoprocessorTxSenders[1]).allowPublicDecrypt(newCtHandle2);

        // Make sure the old context has been deactivated
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Deactivated);
      });

      it("Should allow public decryption with suspended context", async function () {
        // Wait for the pre activation period to pass
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.preActivationBlockPeriod, coprocessorContexts);

        // The second transaction should reach consensus and thus emit the expected event
        // This is because the consensus is reached amongst the suspended context (3 coprocessors)
        // and not the new one (10 coprocessors)
        const result = await multichainAcl.connect(coprocessorTxSenders[1]).allowPublicDecrypt(newCtHandle);

        await expect(result).to.emit(multichainAcl, "AllowPublicDecrypt").withArgs(newCtHandle);
      });

      it("Should revert because the context is no longer valid", async function () {
        // Wait for the pre activation period to pass
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.preActivationBlockPeriod, coprocessorContexts);

        // Wait for the suspended period to pass
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.suspendedBlockPeriod, coprocessorContexts);

        // Check that allow public decrypt request that has already been registered under an active context
        // reverts because this context is no longer valid
        await expect(multichainAcl.connect(coprocessorTxSenders[1]).allowPublicDecrypt(newCtHandle))
          .revertedWithCustomError(multichainAcl, "InvalidCoprocessorContextAllowPublicDecrypt")
          .withArgs(newCtHandle, contextId, ContextStatus.Deactivated);
      });

      it("Should revert because the transaction sender is a coprocessor from the suspended context", async function () {
        // Define another new handle
        // It is used to create a set of inputs different from the one used in the beforeEach block
        const newCtHandle2 = createCtHandle(hostChainId);

        // Wait for the pre activation period to pass
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.preActivationBlockPeriod, coprocessorContexts);

        // Make sure the old context has been suspended
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Suspended);

        // Make sure that a new handle can't be allowed for public decryption by a coprocessor from the suspended context
        await expect(multichainAcl.connect(coprocessorTxSenders[0]).allowPublicDecrypt(newCtHandle2))
          .revertedWithCustomError(coprocessorContexts, "NotCoprocessorTxSenderFromContext")
          .withArgs(newContextId, coprocessorTxSenders[0].address);
      });
    });
  });

  describe("Delegate account", async function () {
    // Define valid inputs (they will be used for delegation by default)
    const delegator = createRandomAddress();
    const delegated = createRandomAddress();
    const delegationAccounts: DelegationAccountsStruct = {
      delegatorAddress: delegator,
      delegatedAddress: delegated,
    };
    const allowedContracts = createRandomAddresses(3);

    // Define new delegation accounts (they will not be used for delegation by default)
    const newDelegator = createRandomAddress();
    const newDelegated = createRandomAddress();
    const newDelegationAccounts: DelegationAccountsStruct = {
      delegatorAddress: newDelegator,
      delegatedAddress: newDelegated,
    };

    beforeEach(async function () {
      // Delegate access to the the account and its contracts
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await multichainAcl
          .connect(coprocessorTxSenders[i])
          .delegateAccount(hostChainId, delegationAccounts, allowedContracts);
      }
    });

    it("Should delegate account", async function () {
      // Trigger two allow calls with different coprocessor transaction senders
      await multichainAcl
        .connect(coprocessorTxSenders[0])
        .delegateAccount(hostChainId, newDelegationAccounts, allowedContracts);
      const txResponse = multichainAcl
        .connect(coprocessorTxSenders[1])
        .delegateAccount(hostChainId, newDelegationAccounts, allowedContracts);

      // Then
      await expect(txResponse)
        .to.emit(multichainAcl, "DelegateAccount")
        .withArgs(hostChainId, toValues(newDelegationAccounts), allowedContracts);
    });

    it("Should revert because coprocessor tries to delegate account twice", async function () {
      await expect(
        multichainAcl
          .connect(coprocessorTxSenders[0])
          .delegateAccount(hostChainId, delegationAccounts, allowedContracts),
      )
        .revertedWithCustomError(multichainAcl, "CoprocessorAlreadyDelegated")
        .withArgs(hostChainId, toValues(delegationAccounts), allowedContracts, coprocessorTxSenders[0].address);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(
        multichainAcl.connect(fakeTxSender).delegateAccount(hostChainId, delegationAccounts, allowedContracts),
      )
        .revertedWithCustomError(coprocessorContexts, "NotCoprocessorTxSenderFromContext")
        .withArgs(contextId, fakeTxSender.address);
    });

    it("Should revert because the contracts list is empty", async function () {
      await expect(
        multichainAcl.connect(coprocessorTxSenders[0]).delegateAccount(hostChainId, delegationAccounts, []),
      ).revertedWithCustomError(multichainAcl, "EmptyContractAddresses");
    });

    it("Should revert because the contracts list exceeds the maximum length", async function () {
      // Define an invalid large list of contract addresses
      const largeContractAddresses = createRandomAddresses(15);

      await expect(
        multichainAcl
          .connect(coprocessorTxSenders[0])
          .delegateAccount(hostChainId, delegationAccounts, largeContractAddresses),
      )
        .revertedWithCustomError(multichainAcl, "ContractsMaxLengthExceeded")
        .withArgs(MAX_CONTRACT_ADDRESSES, largeContractAddresses.length);
    });

    it("Should check that the account is delegated", async function () {
      await multichainAcl.checkAccountDelegated(hostChainId, delegationAccounts, allowedContracts);
    });

    it("Should revert because the delegation has been made on a different host chain", async function () {
      await expect(multichainAcl.checkAccountDelegated(fakeHostChainId, delegationAccounts, allowedContracts))
        .revertedWithCustomError(multichainAcl, "AccountNotDelegated")
        .withArgs(fakeHostChainId, toValues(delegationAccounts), allowedContracts[0]);
    });

    it("Should revert because the contract addresses list is empty", async function () {
      await expect(multichainAcl.checkAccountDelegated(hostChainId, delegationAccounts, [])).revertedWithCustomError(
        multichainAcl,
        "EmptyContractAddresses",
      );
    });

    it("Should revert because the delegation has been made with a different delegator address", async function () {
      const fakeDelegationAccounts: DelegationAccountsStruct = {
        delegatorAddress: newDelegator,
        delegatedAddress: delegated,
      };

      await expect(multichainAcl.checkAccountDelegated(hostChainId, fakeDelegationAccounts, allowedContracts))
        .revertedWithCustomError(multichainAcl, "AccountNotDelegated")
        .withArgs(hostChainId, toValues(fakeDelegationAccounts), allowedContracts[0]);
    });

    it("Should revert because the delegation has been made with a different delegated address", async function () {
      const fakeDelegationAccounts: DelegationAccountsStruct = {
        delegatorAddress: delegator,
        delegatedAddress: newDelegated,
      };

      await expect(multichainAcl.checkAccountDelegated(hostChainId, fakeDelegationAccounts, allowedContracts))
        .revertedWithCustomError(multichainAcl, "AccountNotDelegated")
        .withArgs(hostChainId, toValues(fakeDelegationAccounts), allowedContracts[0]);
    });

    it("Should revert because the contract is paused", async function () {
      // Pause the contract
      await multichainAcl.connect(owner).pause();

      // Try calling paused delegate account
      await expect(
        multichainAcl
          .connect(coprocessorTxSenders[0])
          .delegateAccount(hostChainId, delegationAccounts, allowedContracts),
      ).to.be.revertedWithCustomError(multichainAcl, "EnforcedPause");
    });

    describe("Context changes", async function () {
      let blockPeriods: CoprocessorContextBlockPeriodsStruct;

      // Define the new expected context ID
      const newContextId = 2;

      beforeEach(async function () {
        // Delegate the new accounts with the first coprocessor transaction sender. This should
        // register the request under the first active context (ID 1)
        await multichainAcl
          .connect(coprocessorTxSenders[0])
          .delegateAccount(hostChainId, newDelegationAccounts, allowedContracts);

        // Add a new coprocessor context using a bigger set of coprocessors with different tx sender
        // and signer addresses
        const newCoprocessorContext = await addNewCoprocessorContext(10, coprocessorContexts, owner);
        blockPeriods = newCoprocessorContext.blockPeriods;
      });

      it("Should allow public decryption with suspended context", async function () {
        // The second transaction should reach consensus and thus emit the expected event
        // This is because the consensus is reached amongst the suspended context (3 coprocessors)
        // and not the new one (10 coprocessors)
        const result = await multichainAcl
          .connect(coprocessorTxSenders[1])
          .delegateAccount(hostChainId, newDelegationAccounts, allowedContracts);

        await expect(result)
          .to.emit(multichainAcl, "DelegateAccount")
          .withArgs(hostChainId, toValues(newDelegationAccounts), allowedContracts);
      });

      it("Should revert because the context is no longer valid", async function () {
        // Wait for the pre activation period to pass
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.preActivationBlockPeriod, coprocessorContexts);

        // Wait for the suspended period to pass
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.suspendedBlockPeriod, coprocessorContexts);

        // Check that delegate account request that has already been registered under an active context
        // reverts because this context is no longer valid
        await expect(
          multichainAcl
            .connect(coprocessorTxSenders[1])
            .delegateAccount(hostChainId, newDelegationAccounts, allowedContracts),
        )
          .revertedWithCustomError(multichainAcl, "InvalidCoprocessorContextDelegateAccount")
          .withArgs(
            hostChainId,
            toValues(newDelegationAccounts),
            allowedContracts,
            contextId,
            ContextStatus.Deactivated,
          );
      });

      it("Should revert because the transaction sender is a coprocessor from the suspended context", async function () {
        // Get a different host chain ID
        // It is used to create a set of inputs different from the one used in the beforeEach block
        const newHostChainId = hostChainIds[1];

        // Wait for the pre activation period to pass
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.preActivationBlockPeriod, coprocessorContexts);

        // Make sure the old context has been suspended
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Suspended);

        // Make sure that a new delegation can't be made by a coprocessor from the suspended context
        await expect(
          multichainAcl
            .connect(coprocessorTxSenders[0])
            .delegateAccount(newHostChainId, newDelegationAccounts, allowedContracts),
        )
          .revertedWithCustomError(coprocessorContexts, "NotCoprocessorTxSenderFromContext")
          .withArgs(newContextId, coprocessorTxSenders[0].address);
      });
    });
  });

  describe("Pause", async function () {
    it("Should pause and unpause contract with owner address", async function () {
      // Check that the contract is not paused
      expect(await multichainAcl.paused()).to.be.false;

      // Pause the contract with the owner address
      await expect(multichainAcl.connect(owner).pause()).to.emit(multichainAcl, "Paused").withArgs(owner);
      expect(await multichainAcl.paused()).to.be.true;

      // Unpause the contract with the owner address
      await expect(multichainAcl.connect(owner).unpause()).to.emit(multichainAcl, "Unpaused").withArgs(owner);
      expect(await multichainAcl.paused()).to.be.false;
    });

    it("Should pause contract with pauser address", async function () {
      // Check that the contract is not paused
      expect(await multichainAcl.paused()).to.be.false;

      // Pause the contract with the pauser address
      await expect(multichainAcl.connect(pauser).pause()).to.emit(multichainAcl, "Paused").withArgs(pauser);
      expect(await multichainAcl.paused()).to.be.true;
    });

    it("Should revert on pause because sender is not owner or pauser address", async function () {
      const notOwnerOrPauser = createRandomWallet();
      await expect(multichainAcl.connect(notOwnerOrPauser).pause())
        .to.be.revertedWithCustomError(multichainAcl, "NotOwnerOrPauser")
        .withArgs(notOwnerOrPauser.address);
    });
  });
});
