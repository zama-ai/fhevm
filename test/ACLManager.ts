import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { HDNodeWallet } from "ethers";
import hre from "hardhat";

import { ACLManager, CiphertextManager, HTTPZ } from "../typechain-types";
import { createAndFundRandomUser, createBytes32, createCtHandle, loadTestVariablesFixture } from "./utils";

describe("ACLManager", function () {
  const keyId = 0;
  const ctHandle = createCtHandle();
  const ciphertextDigest = createBytes32();
  const snsCiphertextDigest = createBytes32();

  // Fake values
  const fakeCtHandle = createCtHandle();
  const fakeHostChainId = 123;

  let httpz: HTTPZ;
  let aclManager: ACLManager;
  let ciphertextManager: CiphertextManager;
  let coprocessorTxSenders: HardhatEthersSigner[];
  let hostChainId: number;
  let fakeTxSender: HDNodeWallet;

  async function prepareACLManagerFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const { ciphertextManager, coprocessorTxSenders } = fixtureData;

    // Define the hostChainId
    hostChainId = fixtureData.chainIds[0];

    // Add the ciphertext to the CiphertextManager contract state which will be used during the tests
    for (let i = 0; i < coprocessorTxSenders.length; i++) {
      await ciphertextManager
        .connect(coprocessorTxSenders[i])
        .addCiphertextMaterial(ctHandle, keyId, hostChainId, ciphertextDigest, snsCiphertextDigest);
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
      await expect(aclManager.connect(coprocessorTxSenders[0]).allowAccount(fakeHostChainId, ctHandle, allowedAddress))
        .revertedWithCustomError(httpz, "NetworkNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should allow account to use the ciphertext", async function () {
      // When
      await aclManager.connect(coprocessorTxSenders[0]).allowAccount(hostChainId, ctHandle, allowedAddress);
      const txResponse = aclManager
        .connect(coprocessorTxSenders[1])
        .allowAccount(hostChainId, ctHandle, allowedAddress);

      // Then
      await expect(txResponse).to.emit(aclManager, "AllowAccount").withArgs(ctHandle, allowedAddress);
    });

    it("Should revert with CoprocessorAlreadyAllowed", async function () {
      // When
      await aclManager.connect(coprocessorTxSenders[0]).allowAccount(hostChainId, ctHandle, allowedAddress);
      const txResponse = aclManager
        .connect(coprocessorTxSenders[0])
        .allowAccount(hostChainId, ctHandle, allowedAddress);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "CoprocessorAlreadyAllowed")
        .withArgs(coprocessorTxSenders[0].address, ctHandle);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      // When
      const txResponse = aclManager.connect(fakeTxSender).allowAccount(hostChainId, ctHandle, allowedAddress);

      // Then
      await expect(txResponse).revertedWithCustomError(httpz, "NotCoprocessorTxSender").withArgs(fakeTxSender.address);
    });
  });

  describe("Allow public decrypt", async function () {
    it("Should revert because the hostChainId is not registered in the HTTPZ contract", async function () {
      // Check that allowing public decryption on a fake chain ID reverts
      await expect(aclManager.connect(coprocessorTxSenders[0]).allowPublicDecrypt(fakeHostChainId, ctHandle))
        .revertedWithCustomError(httpz, "NetworkNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should allow for public decryption", async function () {
      // When
      await aclManager.connect(coprocessorTxSenders[0]).allowPublicDecrypt(hostChainId, ctHandle);
      const txResponse = aclManager.connect(coprocessorTxSenders[1]).allowPublicDecrypt(hostChainId, ctHandle);

      // Then
      await expect(txResponse).to.emit(aclManager, "AllowPublicDecrypt").withArgs(ctHandle);
    });

    it("Should revert with CoprocessorAlreadyAllowed", async function () {
      // When
      await aclManager.connect(coprocessorTxSenders[0]).allowPublicDecrypt(hostChainId, ctHandle);
      const txResponse = aclManager.connect(coprocessorTxSenders[0]).allowPublicDecrypt(hostChainId, ctHandle);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "CoprocessorAlreadyAllowed")
        .withArgs(coprocessorTxSenders[0].address, ctHandle);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      // When
      const txResponse = aclManager.connect(fakeTxSender).allowPublicDecrypt(hostChainId, ctHandle);

      // Then
      await expect(txResponse).revertedWithCustomError(httpz, "NotCoprocessorTxSender").withArgs(fakeTxSender.address);
    });
  });

  describe("Delegate account", async function () {
    // Given arbitrary delegator, delegatee and contract addresses
    const delegator = hre.ethers.Wallet.createRandom().address;
    const delegatee = hre.ethers.Wallet.createRandom().address;
    const allowedContract1 = hre.ethers.Wallet.createRandom().address;
    const allowedContract2 = hre.ethers.Wallet.createRandom().address;
    const allowedContract3 = hre.ethers.Wallet.createRandom().address;

    it("Should delegate account", async function () {
      // When
      await aclManager
        .connect(coprocessorTxSenders[0])
        .delegateAccount(hostChainId, delegator, delegatee, [allowedContract1, allowedContract2, allowedContract3]);
      const txResponse = aclManager
        .connect(coprocessorTxSenders[1])
        .delegateAccount(hostChainId, delegator, delegatee, [allowedContract1, allowedContract2, allowedContract3]);

      // Then
      await expect(txResponse)
        .to.emit(aclManager, "DelegateAccount")
        .withArgs(hostChainId, delegator, delegatee, [allowedContract1, allowedContract2, allowedContract3]);
    });

    it("Should revert with CoprocessorAlreadyDelegated", async function () {
      // When
      await aclManager
        .connect(coprocessorTxSenders[0])
        .delegateAccount(hostChainId, delegator, delegatee, [allowedContract1]);
      const txResponse = aclManager
        .connect(coprocessorTxSenders[0])
        .delegateAccount(hostChainId, delegator, delegatee, [allowedContract1]);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "CoprocessorAlreadyDelegated")
        .withArgs(coprocessorTxSenders[0].address, hostChainId, delegator, delegatee, [allowedContract1]);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      // When
      const txResponse = aclManager
        .connect(fakeTxSender)
        .delegateAccount(hostChainId, delegator, delegatee, [allowedContract1]);

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
        .delegateAccount(hostChainId, delegator, delegatee, exceededContracts);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "ContractsMaxLengthExceeded")
        .withArgs(10, exceededLength);
    });
  });

  describe("Check account allowed", async function () {
    const allowedUserAddress = hre.ethers.Wallet.createRandom().address;
    const allowedContractAddress = hre.ethers.Wallet.createRandom().address;
    const ctHandleContractPairs = [{ ctHandle, contractAddress: allowedContractAddress }];

    beforeEach(async function () {
      // Setup the account access permission
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await aclManager.connect(coprocessorTxSenders[i]).allowAccount(hostChainId, ctHandle, allowedUserAddress);
        await aclManager.connect(coprocessorTxSenders[i]).allowAccount(hostChainId, ctHandle, allowedContractAddress);
      }
    });

    it("Should check account is allowed to use the ciphertext", async function () {
      await aclManager.connect(coprocessorTxSenders[0]).checkAccountAllowed(allowedUserAddress, ctHandleContractPairs);
    });

    it("Should revert because user part of the contract addresses", async function () {
      const fakeCtHandleContractPairs = [{ ctHandle: ctHandle, contractAddress: allowedUserAddress }];

      // Check that the fakeUserAddress is not allowed to use the ciphertext
      await expect(
        aclManager.connect(coprocessorTxSenders[0]).checkAccountAllowed(allowedUserAddress, fakeCtHandleContractPairs),
      )
        .to.be.revertedWithCustomError(aclManager, "AccountAddressInContractAddresses")
        .withArgs(allowedUserAddress);
    });

    it("Should revert because user is not allowed to use the ciphertext", async function () {
      const fakeUserAddress = hre.ethers.Wallet.createRandom().address;

      // Check that the fakeUserAddress is not allowed to use the ciphertext
      await expect(
        aclManager.connect(coprocessorTxSenders[0]).checkAccountAllowed(fakeUserAddress, ctHandleContractPairs),
      )
        .to.be.revertedWithCustomError(aclManager, "AccountNotAllowedToUseCiphertext")
        .withArgs(ctHandle, fakeUserAddress);
    });

    it("Should revert because contract is not allowed to use the ciphertext", async function () {
      const fakeContractAddress = hre.ethers.Wallet.createRandom().address;
      const fakeCtHandleContractPairs = [{ ctHandle: ctHandle, contractAddress: fakeContractAddress }];

      // Check that the fakeContractAddress is not allowed to use the ciphertext
      await expect(
        aclManager.connect(coprocessorTxSenders[0]).checkAccountAllowed(allowedUserAddress, fakeCtHandleContractPairs),
      )
        .to.be.revertedWithCustomError(aclManager, "ContractNotAllowedToUseCiphertext")
        .withArgs(ctHandle, fakeContractAddress);
    });
  });

  describe("Check public decrypt allowed", async function () {
    beforeEach(async function () {
      // Setup the public decrypt permission for the given hostChainId and ctHandle used during tests
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await aclManager.connect(coprocessorTxSenders[i]).allowPublicDecrypt(hostChainId, ctHandle);
      }
    });

    it("Should check public decrypt is allowed", async function () {
      await aclManager.connect(coprocessorTxSenders[0]).checkPublicDecryptAllowed([ctHandle]);
    });

    it("Should revert with PublicDecryptNotAllowed", async function () {
      // Check that the fakeCtHandle is not allowed for public decryption
      await expect(aclManager.connect(coprocessorTxSenders[0]).checkPublicDecryptAllowed([fakeCtHandle]))
        .to.be.revertedWithCustomError(aclManager, "PublicDecryptNotAllowed")
        .withArgs(fakeCtHandle);
    });
  });

  describe("Is account delegated", async function () {
    // Given arbitrary delegator, delegatee and contract addresses
    const delegator = hre.ethers.Wallet.createRandom().address;
    const delegatee = hre.ethers.Wallet.createRandom().address;
    const allowedContract1 = hre.ethers.Wallet.createRandom().address;
    const allowedContract2 = hre.ethers.Wallet.createRandom().address;
    const allowedContracts = [allowedContract1, allowedContract2];

    beforeEach(async function () {
      // Setup the account delegation for the given hostChainId, delegator, delegatee and allowedContracts used during tests
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await aclManager
          .connect(coprocessorTxSenders[i])
          .delegateAccount(hostChainId, delegator, delegatee, allowedContracts);
      }
    });

    it("Should check account is delegated", async function () {
      await aclManager.checkAccountDelegated(hostChainId, delegator, delegatee, allowedContracts);
    });

    it("Should revert because none of the inputs has account delegation", async function () {
      // Given
      const fakeDelegator = hre.ethers.Wallet.createRandom().address;
      const fakeDelegatee = hre.ethers.Wallet.createRandom().address;

      // When
      const txResponse1 = aclManager.checkAccountDelegated(fakeHostChainId, delegator, delegatee, allowedContracts);
      const txResponse2 = aclManager.checkAccountDelegated(hostChainId, fakeDelegator, delegatee, allowedContracts);
      const txResponse3 = aclManager.checkAccountDelegated(hostChainId, delegator, fakeDelegatee, allowedContracts);

      // Then
      await expect(txResponse1)
        .revertedWithCustomError(aclManager, "AccountNotDelegated")
        .withArgs(fakeHostChainId, delegator, delegatee, allowedContracts[0]);
      await expect(txResponse2)
        .revertedWithCustomError(aclManager, "AccountNotDelegated")
        .withArgs(hostChainId, fakeDelegator, delegatee, allowedContracts[0]);
      await expect(txResponse3)
        .revertedWithCustomError(aclManager, "AccountNotDelegated")
        .withArgs(hostChainId, delegator, fakeDelegatee, allowedContracts[0]);
    });

    it("Should not distinguish between differently ordered contract list", async function () {
      // Given
      const alteredAllowedContracts = [allowedContract2, allowedContract1];

      await aclManager.checkAccountDelegated(hostChainId, delegator, delegatee, alteredAllowedContracts);
    });
  });
});
