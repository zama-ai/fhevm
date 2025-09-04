import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import hre from "hardhat";

import { GatewayConfig, MultichainAcl, MultichainAcl__factory } from "../typechain-types";
// The type needs to be imported separately because it is not properly detected by the linter
// as this type is defined as a shared structs instead of directly in the IMultichainAcl interface
import { DelegationAccountsStruct } from "../typechain-types/contracts/interfaces/IMultichainAcl";
import {
  createCtHandle,
  createRandomAddress,
  createRandomAddresses,
  createRandomWallet,
  loadHostChainIds,
  loadTestVariablesFixture,
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

  // Define fake values
  const fakeHostChainId = 123;
  const ctHandleFakeChainId = createCtHandle(fakeHostChainId);
  const fakeTxSender = createRandomWallet();

  // Define extra data for version 0
  const extraDataV0 = hre.ethers.solidityPacked(["uint8"], [0]);

  let gatewayConfig: GatewayConfig;
  let multichainAcl: MultichainAcl;
  let coprocessorTxSenders: HardhatEthersSigner[];
  let owner: Wallet;
  let pauser: HardhatEthersSigner;

  beforeEach(async function () {
    // Initialize used global variables before each test
    const fixture = await loadFixture(loadTestVariablesFixture);
    gatewayConfig = fixture.gatewayConfig;
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
        await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, accountAddress, extraDataV0);
      }
    });

    it("Should revert because the hostChainId is not registered in the GatewayConfig contract", async function () {
      // Check that allowing an account to use a ciphertext on a fake chain ID reverts
      await expect(
        multichainAcl
          .connect(coprocessorTxSenders[0])
          .allowAccount(ctHandleFakeChainId, newAccountAddress, extraDataV0),
      )
        .revertedWithCustomError(gatewayConfig, "HostChainNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should allow account with 2 valid responses", async function () {
      // Trigger 2 allow calls with different coprocessor transaction senders
      await multichainAcl.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, newAccountAddress, extraDataV0);
      const txResponse = multichainAcl
        .connect(coprocessorTxSenders[1])
        .allowAccount(ctHandle, newAccountAddress, extraDataV0);

      // Consensus should be reached at the second response
      await expect(txResponse).to.emit(multichainAcl, "AllowAccount").withArgs(ctHandle, newAccountAddress);
    });

    it("Should allow account with 2 valid responses and ignore the other valid one", async function () {
      // Trigger 3 allow account calls with different coprocessor transaction senders
      const txResponse1 = await multichainAcl
        .connect(coprocessorTxSenders[0])
        .allowAccount(ctHandle, newAccountAddress, extraDataV0);
      await multichainAcl.connect(coprocessorTxSenders[1]).allowAccount(ctHandle, newAccountAddress, extraDataV0);
      const txResponse3 = await multichainAcl
        .connect(coprocessorTxSenders[2])
        .allowAccount(ctHandle, newAccountAddress, extraDataV0);

      // Check that the 1st and 3rd responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet
      // - 3rd response is ignored (not reverted) even though it is late
      await expect(txResponse1).to.not.emit(multichainAcl, "AllowAccount");
      await expect(txResponse3).to.not.emit(multichainAcl, "AllowAccount");
    });

    it("Should get all valid coprocessor transaction senders from allow account consensus", async function () {
      // Trigger an allow account calls using the first coprocessor transaction sender
      await multichainAcl.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, newAccountAddress, extraDataV0);

      const expectedCoprocessorTxSenders1 = coprocessorTxSenders.slice(0, 1).map((s) => s.address);

      // Get the coprocessor transaction sender that answered first, before the consensus is reached
      // Since the consensus is directly made in the "request" call, the list represents the coprocessor
      // transaction sender that answered, and is accessible before the consensus is reached
      const proofRejectionConsensusTxSenders1 = await multichainAcl.getAllowAccountConsensusTxSenders(
        ctHandle,
        newAccountAddress,
      );
      expect(proofRejectionConsensusTxSenders1).to.deep.equal(expectedCoprocessorTxSenders1);

      // Trigger an allow account calls using the second coprocessor transaction sender
      await multichainAcl.connect(coprocessorTxSenders[1]).allowAccount(ctHandle, newAccountAddress, extraDataV0);

      const expectedCoprocessorTxSenders2 = coprocessorTxSenders.slice(0, 2).map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // first 2 coprocessor transaction senders, at the moment the consensus is reached
      const proofRejectionConsensusTxSenders2 = await multichainAcl.getAllowAccountConsensusTxSenders(
        ctHandle,
        newAccountAddress,
      );
      expect(proofRejectionConsensusTxSenders2).to.deep.equal(expectedCoprocessorTxSenders2);

      // Trigger an allow account calls using the third coprocessor transaction sender
      await multichainAcl.connect(coprocessorTxSenders[2]).allowAccount(ctHandle, newAccountAddress, extraDataV0);

      const expectedCoprocessorTxSenders3 = coprocessorTxSenders.map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the 3
      // coprocessor transaction senders, after the consensus is reached
      const proofRejectionConsensusTxSenders3 = await multichainAcl.getAllowAccountConsensusTxSenders(
        ctHandle,
        newAccountAddress,
      );
      expect(proofRejectionConsensusTxSenders3).to.deep.equal(expectedCoprocessorTxSenders3);
    });

    it("Should revert because coprocessor tries to allow account twice", async function () {
      await expect(multichainAcl.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, accountAddress, extraDataV0))
        .revertedWithCustomError(multichainAcl, "CoprocessorAlreadyAllowedAccount")
        .withArgs(ctHandle, accountAddress, coprocessorTxSenders[0].address);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(multichainAcl.connect(fakeTxSender).allowAccount(ctHandle, newAccountAddress, extraDataV0))
        .revertedWithCustomError(gatewayConfig, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
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
  });

  describe("Allow public decrypt", async function () {
    beforeEach(async function () {
      // Allow the handle to be publicly decrypted
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await multichainAcl.connect(coprocessorTxSenders[i]).allowPublicDecrypt(ctHandle, extraDataV0);
      }
    });

    it("Should revert because the hostChainId is not registered in the GatewayConfig contract", async function () {
      await expect(multichainAcl.connect(coprocessorTxSenders[0]).allowPublicDecrypt(ctHandleFakeChainId, extraDataV0))
        .revertedWithCustomError(gatewayConfig, "HostChainNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should allow for public decryption with 2 valid responses", async function () {
      // Trigger 2 allow calls with different coprocessor transaction senders
      await multichainAcl.connect(coprocessorTxSenders[0]).allowPublicDecrypt(newCtHandle, extraDataV0);
      const txResponse = multichainAcl.connect(coprocessorTxSenders[1]).allowPublicDecrypt(newCtHandle, extraDataV0);

      // Consensus should be reached at the second response
      await expect(txResponse).to.emit(multichainAcl, "AllowPublicDecrypt").withArgs(newCtHandle);
    });

    it("Should allow public decryption with 2 valid responses and ignore the other valid one", async function () {
      // Trigger 3 allow public decryption calls with different coprocessor transaction senders
      const txResponse1 = await multichainAcl
        .connect(coprocessorTxSenders[0])
        .allowPublicDecrypt(newCtHandle, extraDataV0);
      await multichainAcl.connect(coprocessorTxSenders[1]).allowPublicDecrypt(newCtHandle, extraDataV0);
      const txResponse3 = await multichainAcl
        .connect(coprocessorTxSenders[2])
        .allowPublicDecrypt(newCtHandle, extraDataV0);

      // Check that the 1st and 3rd responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet
      // - 3rd response is ignored (not reverted) even though it is late
      await expect(txResponse1).to.not.emit(multichainAcl, "AllowPublicDecrypt");
      await expect(txResponse3).to.not.emit(multichainAcl, "AllowPublicDecrypt");
    });

    it("Should get all valid coprocessor transaction senders from allow public decryption consensus", async function () {
      // Trigger an allow public decryption calls using the first coprocessor transaction sender
      await multichainAcl.connect(coprocessorTxSenders[0]).allowPublicDecrypt(newCtHandle, extraDataV0);

      const expectedCoprocessorTxSenders1 = coprocessorTxSenders.slice(0, 1).map((s) => s.address);

      // Get the coprocessor transaction sender that answered first, before the consensus is reached
      // Since the consensus is directly made in the "request" call, the list represents the coprocessor
      // transaction sender that answered, and is accessible before the consensus is reached
      const proofRejectionConsensusTxSenders1 =
        await multichainAcl.getAllowPublicDecryptConsensusTxSenders(newCtHandle);
      expect(proofRejectionConsensusTxSenders1).to.deep.equal(expectedCoprocessorTxSenders1);

      // Trigger an allow public decryption calls using the second coprocessor transaction sender
      await multichainAcl.connect(coprocessorTxSenders[1]).allowPublicDecrypt(newCtHandle, extraDataV0);

      const expectedCoprocessorTxSenders2 = coprocessorTxSenders.slice(0, 2).map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // first 2 coprocessor transaction senders, at the moment the consensus is reached
      const proofRejectionConsensusTxSenders2 =
        await multichainAcl.getAllowPublicDecryptConsensusTxSenders(newCtHandle);
      expect(proofRejectionConsensusTxSenders2).to.deep.equal(expectedCoprocessorTxSenders2);

      // Trigger an allow public decryption calls using the third coprocessor transaction sender
      await multichainAcl.connect(coprocessorTxSenders[2]).allowPublicDecrypt(newCtHandle, extraDataV0);

      const expectedCoprocessorTxSenders3 = coprocessorTxSenders.map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the 3
      // coprocessor transaction senders, after the consensus is reached
      const proofRejectionConsensusTxSenders3 =
        await multichainAcl.getAllowPublicDecryptConsensusTxSenders(newCtHandle);
      expect(proofRejectionConsensusTxSenders3).to.deep.equal(expectedCoprocessorTxSenders3);
    });

    it("Should revert because coprocessor tries to allow public decryption twice", async function () {
      await expect(multichainAcl.connect(coprocessorTxSenders[0]).allowPublicDecrypt(ctHandle, extraDataV0))
        .revertedWithCustomError(multichainAcl, "CoprocessorAlreadyAllowedPublicDecrypt")
        .withArgs(ctHandle, coprocessorTxSenders[0].address);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(multichainAcl.connect(fakeTxSender).allowPublicDecrypt(newCtHandle, extraDataV0))
        .revertedWithCustomError(gatewayConfig, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should check public decrypt is allowed", async function () {
      await multichainAcl.connect(coprocessorTxSenders[0]).checkPublicDecryptAllowed(ctHandle);
    });

    it("Should revert because the handle is not allowed to be publicly decrypted", async function () {
      await expect(multichainAcl.connect(coprocessorTxSenders[0]).checkPublicDecryptAllowed(newCtHandle))
        .to.be.revertedWithCustomError(multichainAcl, "PublicDecryptNotAllowed")
        .withArgs(newCtHandle);
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

    it("Should delegate account with 2 valid responses", async function () {
      // Trigger 2 delegate calls with different coprocessor transaction senders
      await multichainAcl
        .connect(coprocessorTxSenders[0])
        .delegateAccount(hostChainId, newDelegationAccounts, allowedContracts);
      const txResponse = multichainAcl
        .connect(coprocessorTxSenders[1])
        .delegateAccount(hostChainId, newDelegationAccounts, allowedContracts);

      // Consensus should be reached at the second response
      await expect(txResponse)
        .to.emit(multichainAcl, "DelegateAccount")
        .withArgs(hostChainId, toValues(newDelegationAccounts), allowedContracts);
    });

    it("Should delegate account with 2 valid responses and ignore the other valid one", async function () {
      // Trigger 3 delegate account calls with different coprocessor transaction senders
      const txResponse1 = await multichainAcl
        .connect(coprocessorTxSenders[0])
        .delegateAccount(hostChainId, newDelegationAccounts, allowedContracts);
      await multichainAcl
        .connect(coprocessorTxSenders[1])
        .delegateAccount(hostChainId, newDelegationAccounts, allowedContracts);
      const txResponse3 = await multichainAcl
        .connect(coprocessorTxSenders[2])
        .delegateAccount(hostChainId, newDelegationAccounts, allowedContracts);

      // Check that the 1st and 3rd responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet
      // - 3rd response is ignored (not reverted) even though it is late
      await expect(txResponse1).to.not.emit(multichainAcl, "DelegateAccount");
      await expect(txResponse3).to.not.emit(multichainAcl, "DelegateAccount");
    });

    it("Should get all valid coprocessor transaction senders from delegate account consensus", async function () {
      // Trigger a delegate account calls using the first coprocessor transaction sender
      await multichainAcl
        .connect(coprocessorTxSenders[0])
        .delegateAccount(hostChainId, newDelegationAccounts, allowedContracts);

      const expectedCoprocessorTxSenders1 = coprocessorTxSenders.slice(0, 1).map((s) => s.address);

      // Get the coprocessor transaction sender that answered first, before the consensus is reached
      // Since the consensus is directly made in the "request" call, the list represents the coprocessor
      // transaction sender that answered, and is accessible before the consensus is reached
      const proofRejectionConsensusTxSenders1 = await multichainAcl.getDelegateAccountConsensusTxSenders(
        hostChainId,
        newDelegationAccounts,
        allowedContracts,
      );
      expect(proofRejectionConsensusTxSenders1).to.deep.equal(expectedCoprocessorTxSenders1);

      // Trigger a delegate account calls using the second coprocessor transaction sender
      await multichainAcl
        .connect(coprocessorTxSenders[1])
        .delegateAccount(hostChainId, newDelegationAccounts, allowedContracts);

      const expectedCoprocessorTxSenders2 = coprocessorTxSenders.slice(0, 2).map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // first 2 coprocessor transaction senders, at the moment the consensus is reached
      const proofRejectionConsensusTxSenders2 = await multichainAcl.getDelegateAccountConsensusTxSenders(
        hostChainId,
        newDelegationAccounts,
        allowedContracts,
      );
      expect(proofRejectionConsensusTxSenders2).to.deep.equal(expectedCoprocessorTxSenders2);

      // Trigger a delegate account calls using the third coprocessor transaction sender
      await multichainAcl
        .connect(coprocessorTxSenders[2])
        .delegateAccount(hostChainId, newDelegationAccounts, allowedContracts);

      const expectedCoprocessorTxSenders3 = coprocessorTxSenders.map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the 3
      // coprocessor transaction senders, after the consensus is reached
      const proofRejectionConsensusTxSenders3 = await multichainAcl.getDelegateAccountConsensusTxSenders(
        hostChainId,
        newDelegationAccounts,
        allowedContracts,
      );
      expect(proofRejectionConsensusTxSenders3).to.deep.equal(expectedCoprocessorTxSenders3);
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
        .revertedWithCustomError(gatewayConfig, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
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
  });

  describe("Pause", async function () {
    it("Should pause the contract with the pauser and unpause with the owner", async function () {
      // Check that the contract is not paused
      expect(await multichainAcl.paused()).to.be.false;

      // Pause the contract with the pauser address
      await expect(multichainAcl.connect(pauser).pause()).to.emit(multichainAcl, "Paused").withArgs(pauser);
      expect(await multichainAcl.paused()).to.be.true;

      // Unpause the contract with the owner address
      await expect(multichainAcl.connect(owner).unpause()).to.emit(multichainAcl, "Unpaused").withArgs(owner);
      expect(await multichainAcl.paused()).to.be.false;
    });

    it("Should revert on pause because sender is not the pauser", async function () {
      const fakePauser = createRandomWallet();

      await expect(multichainAcl.connect(fakePauser).pause())
        .to.be.revertedWithCustomError(multichainAcl, "NotPauserOrGatewayConfig")
        .withArgs(fakePauser.address);
    });

    it("Should revert on unpause because sender is not the owner", async function () {
      // Pause the contract with the pauser address
      await multichainAcl.connect(pauser).pause();

      const fakeOwner = createRandomWallet();

      await expect(multichainAcl.connect(fakeOwner).unpause())
        .to.be.revertedWithCustomError(multichainAcl, "NotOwnerOrGatewayConfig")
        .withArgs(fakeOwner.address);
    });
  });
});
