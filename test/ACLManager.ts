import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { HDNodeWallet } from "ethers";
import hre from "hardhat";

import { ACLManager, CiphertextManager, HTTPZ } from "../typechain-types";
// The type needs to be imported separately because it is not properly detected by the linter
// as this type is defined as a shared structs instead of directly in the IACLManager interface
import { DelegationAccountsStruct } from "../typechain-types/contracts/interfaces/IACLManager";
import {
  createAndFundRandomUser,
  createBytes32,
  createCtHandleWithChainId,
  loadChainIds,
  loadTestVariablesFixture,
  toValues,
} from "./utils";

describe("ACLManager", function () {
  // Define the host chainId(s)
  const hostChainIds = loadChainIds();
  const hostChainId = hostChainIds[0];

  // Create a ctHandle with the host chain ID
  const ctHandle = createCtHandleWithChainId(hostChainId);

  // Define input values
  const keyId = 0;
  const ciphertextDigest = createBytes32();
  const snsCiphertextDigest = createBytes32();

  // Fake values
  const fakeHostChainId = 123;
  const ctHandleFakeChainId = createCtHandleWithChainId(fakeHostChainId);
  const notAllowedCtHandle = createCtHandleWithChainId(hostChainId);

  let httpz: HTTPZ;
  let aclManager: ACLManager;
  let ciphertextManager: CiphertextManager;
  let coprocessorTxSenders: HardhatEthersSigner[];
  let fakeTxSender: HDNodeWallet;

  async function prepareACLManagerFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const { ciphertextManager, coprocessorTxSenders } = fixtureData;

    // Add the ciphertext to the CiphertextManager contract state which will be used during the tests
    for (let i = 0; i < coprocessorTxSenders.length; i++) {
      await ciphertextManager
        .connect(coprocessorTxSenders[i])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);
    }

    return fixtureData;
  }

  beforeEach(async function () {
    // Initialize used global variables before each test
    const fixture = await loadFixture(prepareACLManagerFixture);
    httpz = fixture.httpz;
    aclManager = fixture.aclManager;
    ciphertextManager = fixture.ciphertextManager;
    coprocessorTxSenders = fixture.coprocessorTxSenders;

    fakeTxSender = await createAndFundRandomUser();
  });

  describe("Allow account", async function () {
    const allowedAddress = "0x388C818CA8B9251b393131C08a736A67ccB19297";

    it("Should revert because the hostChainId is not registered in the HTTPZ contract", async function () {
      // Check that allowing an account to use a ciphertext on a fake chain ID reverts
      await expect(aclManager.connect(coprocessorTxSenders[0]).allowAccount(ctHandleFakeChainId, allowedAddress))
        .revertedWithCustomError(httpz, "NetworkNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should allow account to use the ciphertext", async function () {
      // Trigger two allow calls with different coprocessor transaction senders
      await aclManager.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, allowedAddress);
      const txResponse = aclManager.connect(coprocessorTxSenders[1]).allowAccount(ctHandle, allowedAddress);

      // Check that the right event is emitted
      await expect(txResponse).to.emit(aclManager, "AllowAccount").withArgs(ctHandle, allowedAddress);
    });

    it("Should revert with CoprocessorAlreadyAllowed", async function () {
      // Trigger an allow call with the first coprocessor transaction sender
      await aclManager.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, allowedAddress);

      // Check that triggering an allow call with the same coprocessor transaction sender reverts
      await expect(aclManager.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, allowedAddress))
        .revertedWithCustomError(aclManager, "CoprocessorAlreadyAllowed")
        .withArgs(coprocessorTxSenders[0].address, ctHandle);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(aclManager.connect(fakeTxSender).allowAccount(ctHandle, allowedAddress))
        .revertedWithCustomError(httpz, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
    });
  });

  describe("Allow public decrypt", async function () {
    it("Should revert because the hostChainId is not registered in the HTTPZ contract", async function () {
      await expect(aclManager.connect(coprocessorTxSenders[0]).allowPublicDecrypt(ctHandleFakeChainId))
        .revertedWithCustomError(httpz, "NetworkNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should allow for public decryption", async function () {
      // Trigger two allow calls with different coprocessor transaction senders
      await aclManager.connect(coprocessorTxSenders[0]).allowPublicDecrypt(ctHandle);
      const txResponse = aclManager.connect(coprocessorTxSenders[1]).allowPublicDecrypt(ctHandle);

      // Check that the right event is emitted
      await expect(txResponse).to.emit(aclManager, "AllowPublicDecrypt").withArgs(ctHandle);
    });

    it("Should revert with CoprocessorAlreadyAllowed", async function () {
      // Trigger an allow call with the first coprocessor transaction sender
      await aclManager.connect(coprocessorTxSenders[0]).allowPublicDecrypt(ctHandle);

      // Check that triggering an allow call with the same coprocessor transaction sender reverts
      await expect(aclManager.connect(coprocessorTxSenders[0]).allowPublicDecrypt(ctHandle))
        .revertedWithCustomError(aclManager, "CoprocessorAlreadyAllowed")
        .withArgs(coprocessorTxSenders[0].address, ctHandle);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(aclManager.connect(fakeTxSender).allowPublicDecrypt(ctHandle))
        .revertedWithCustomError(httpz, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
    });
  });

  describe("Delegate account", async function () {
    // Given arbitrary delegator, delegated and contract addresses
    const delegator = hre.ethers.Wallet.createRandom().address;
    const delegated = hre.ethers.Wallet.createRandom().address;
    const delegationAccounts: DelegationAccountsStruct = {
      delegatorAddress: delegator,
      delegatedAddress: delegated,
    };
    const allowedContract1 = hre.ethers.Wallet.createRandom().address;
    const allowedContract2 = hre.ethers.Wallet.createRandom().address;
    const allowedContract3 = hre.ethers.Wallet.createRandom().address;

    it("Should delegate account", async function () {
      // When
      await aclManager
        .connect(coprocessorTxSenders[0])
        .delegateAccount(hostChainId, delegationAccounts, [allowedContract1, allowedContract2, allowedContract3]);
      const txResponse = aclManager
        .connect(coprocessorTxSenders[1])
        .delegateAccount(hostChainId, delegationAccounts, [allowedContract1, allowedContract2, allowedContract3]);

      // Then
      await expect(txResponse)
        .to.emit(aclManager, "DelegateAccount")
        .withArgs(hostChainId, toValues(delegationAccounts), [allowedContract1, allowedContract2, allowedContract3]);
    });

    it("Should revert with CoprocessorAlreadyDelegated", async function () {
      // When
      await aclManager
        .connect(coprocessorTxSenders[0])
        .delegateAccount(hostChainId, delegationAccounts, [allowedContract1]);
      const txResponse = aclManager
        .connect(coprocessorTxSenders[0])
        .delegateAccount(hostChainId, delegationAccounts, [allowedContract1]);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "CoprocessorAlreadyDelegated")
        .withArgs(coprocessorTxSenders[0].address, hostChainId, toValues(delegationAccounts), [allowedContract1]);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      // When
      const txResponse = aclManager
        .connect(fakeTxSender)
        .delegateAccount(hostChainId, delegationAccounts, [allowedContract1]);

      // Then
      await expect(txResponse).revertedWithCustomError(httpz, "NotCoprocessorTxSender").withArgs(fakeTxSender.address);
    });

    it("Should revert because the contracts list exceeds the maximum length", async function () {
      // Given
      const exceededLength = 15;
      const exceededContracts = [];
      for (let i = 0; i < exceededLength; i++) {
        exceededContracts.push(hre.ethers.Wallet.createRandom().address);
      }
      // When
      const txResponse = aclManager
        .connect(coprocessorTxSenders[0])
        .delegateAccount(hostChainId, delegationAccounts, exceededContracts);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "ContractsMaxLengthExceeded")
        .withArgs(10, exceededLength);
    });
  });

  describe("Check account allowed", async function () {
    const allowedUserAddress = hre.ethers.Wallet.createRandom().address;
    const allowedContractAddress = hre.ethers.Wallet.createRandom().address;

    beforeEach(async function () {
      // Setup the account access permission
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await aclManager.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, allowedUserAddress);
        await aclManager.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, allowedContractAddress);
      }
    });

    it("Should check account is allowed to use the ciphertext", async function () {
      await aclManager.connect(coprocessorTxSenders[0]).checkAccountAllowed(allowedUserAddress, ctHandle);
    });

    it("Should revert because user is not allowed to use the ciphertext", async function () {
      const fakeUserAddress = hre.ethers.Wallet.createRandom().address;

      // Check that the fakeUserAddress is not allowed to use the ciphertext
      await expect(aclManager.connect(coprocessorTxSenders[0]).checkAccountAllowed(fakeUserAddress, ctHandle))
        .to.be.revertedWithCustomError(aclManager, "AccountNotAllowedToUseCiphertext")
        .withArgs(fakeUserAddress, ctHandle);
    });

    it("Should revert because contract is not allowed to use the ciphertext", async function () {
      const fakeContractAddress = hre.ethers.Wallet.createRandom().address;

      // Check that the fakeContractAddress is not allowed to use the ciphertext
      await expect(aclManager.connect(coprocessorTxSenders[0]).checkAccountAllowed(fakeContractAddress, ctHandle))
        .to.be.revertedWithCustomError(aclManager, "AccountNotAllowedToUseCiphertext")
        .withArgs(fakeContractAddress, ctHandle);
    });
  });

  describe("Check public decrypt allowed", async function () {
    beforeEach(async function () {
      // Setup the public decrypt permission for the given ctHandle used during tests
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await aclManager.connect(coprocessorTxSenders[i]).allowPublicDecrypt(ctHandle);
      }
    });

    it("Should check public decrypt is allowed", async function () {
      await aclManager.connect(coprocessorTxSenders[0]).checkPublicDecryptAllowed(ctHandle);
    });

    it("Should revert with PublicDecryptNotAllowed", async function () {
      // Check that the handle is not allowed for public decryption
      await expect(aclManager.connect(coprocessorTxSenders[0]).checkPublicDecryptAllowed(notAllowedCtHandle))
        .to.be.revertedWithCustomError(aclManager, "PublicDecryptNotAllowed")
        .withArgs(notAllowedCtHandle);
    });
  });

  describe("Is account delegated", async function () {
    // Given arbitrary delegator, delegated and contract addresses
    const delegator = hre.ethers.Wallet.createRandom().address;
    const delegated = hre.ethers.Wallet.createRandom().address;
    const delegationAccounts: DelegationAccountsStruct = {
      delegatorAddress: delegator,
      delegatedAddress: delegated,
    };
    const allowedContract1 = hre.ethers.Wallet.createRandom().address;
    const allowedContract2 = hre.ethers.Wallet.createRandom().address;
    const allowedContracts = [allowedContract1, allowedContract2];

    beforeEach(async function () {
      // Setup the account delegation for the given hostChainId, delegator, delegated and allowedContracts used during tests
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await aclManager
          .connect(coprocessorTxSenders[i])
          .delegateAccount(hostChainId, delegationAccounts, allowedContracts);
      }
    });

    it("Should check account is delegated", async function () {
      await aclManager.checkAccountDelegated(hostChainId, delegationAccounts, allowedContracts);
    });

    it("Should revert because none of the inputs has account delegation", async function () {
      // Given
      const fakeDelegator = hre.ethers.Wallet.createRandom().address;
      const fakeDelegated = hre.ethers.Wallet.createRandom().address;
      const fakeDelegationAccounts: DelegationAccountsStruct = {
        delegatorAddress: fakeDelegator,
        delegatedAddress: delegated,
      };
      const fakeDelegationAccounts2: DelegationAccountsStruct = {
        delegatorAddress: delegator,
        delegatedAddress: fakeDelegated,
      };

      // When
      const txResponse1 = aclManager.checkAccountDelegated(fakeHostChainId, delegationAccounts, allowedContracts);
      const txResponse2 = aclManager.checkAccountDelegated(hostChainId, fakeDelegationAccounts, allowedContracts);
      const txResponse3 = aclManager.checkAccountDelegated(hostChainId, fakeDelegationAccounts2, allowedContracts);

      // Then
      await expect(txResponse1)
        .revertedWithCustomError(aclManager, "AccountNotDelegated")
        .withArgs(fakeHostChainId, toValues(delegationAccounts), allowedContracts[0]);
      await expect(txResponse2)
        .revertedWithCustomError(aclManager, "AccountNotDelegated")
        .withArgs(hostChainId, toValues(fakeDelegationAccounts), allowedContracts[0]);
      await expect(txResponse3)
        .revertedWithCustomError(aclManager, "AccountNotDelegated")
        .withArgs(hostChainId, toValues(fakeDelegationAccounts2), allowedContracts[0]);
    });

    it("Should not distinguish between differently ordered contract list", async function () {
      // Given
      const alteredAllowedContracts = [allowedContract2, allowedContract1];

      await aclManager.checkAccountDelegated(hostChainId, delegationAccounts, alteredAllowedContracts);
    });
  });
});
