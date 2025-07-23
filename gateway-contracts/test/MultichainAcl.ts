import { HardhatEthersSigner, SignerWithAddress } from "@nomicfoundation/hardhat-ethers/signers";
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

  let gatewayConfig: GatewayConfig;
  let multichainAcl: MultichainAcl;
  let coprocessorTxSenders: HardhatEthersSigner[];
  let owner: Wallet;

  beforeEach(async function () {
    // Initialize used global variables before each test
    const fixture = await loadFixture(loadTestVariablesFixture);
    gatewayConfig = fixture.gatewayConfig;
    multichainAcl = fixture.multichainAcl;
    coprocessorTxSenders = fixture.coprocessorTxSenders;
    owner = fixture.owner;
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

    beforeEach(async function () {
      // Delegate access to the the account and its contracts
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await multichainAcl
          .connect(coprocessorTxSenders[i])
          .delegateAccount(hostChainId, delegationAccounts, allowedContracts);
      }
    });

    it("Should delegate account", async function () {
      // Define new accounts to use for delegation
      const newDelegationAccounts: DelegationAccountsStruct = {
        delegatorAddress: newDelegator,
        delegatedAddress: newDelegated,
      };

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
});
