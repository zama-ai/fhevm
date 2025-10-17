import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import hre from "hardhat";

import { GatewayConfig, MultichainACL, MultichainACL__factory } from "../typechain-types";
import {
  createCtHandle,
  createRandomAddress,
  createRandomWallet,
  loadHostChainIds,
  loadTestVariablesFixture,
} from "./utils";

describe("MultichainACL", function () {
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

  // Define user decryption delegation parameters
  const delegator = createRandomAddress();
  const delegate = createRandomAddress();
  const contractAddress = createRandomAddress();
  const expiryDate = Date.now();
  const delegationCounter = 1;

  let gatewayConfig: GatewayConfig;
  let multichainACL: MultichainACL;
  let coprocessorTxSenders: HardhatEthersSigner[];
  let owner: Wallet;
  let pauser: Wallet;

  beforeEach(async function () {
    // Initialize used global variables before each test
    const fixture = await loadFixture(loadTestVariablesFixture);
    gatewayConfig = fixture.gatewayConfig;
    multichainACL = fixture.multichainACL;
    coprocessorTxSenders = fixture.coprocessorTxSenders;
    owner = fixture.owner;
    pauser = fixture.pauser;
  });

  describe("Deployment", function () {
    let multichainACLFactory: MultichainACL__factory;

    beforeEach(async function () {
      // Get the multichainACL contract factory
      multichainACLFactory = await hre.ethers.getContractFactory("MultichainACL", owner);
    });

    it("Should revert because initialization is not from an empty proxy", async function () {
      await expect(
        hre.upgrades.upgradeProxy(multichainACL, multichainACLFactory, {
          call: { fn: "initializeFromEmptyProxy" },
        }),
      ).to.be.revertedWithCustomError(multichainACL, "NotInitializingFromEmptyProxy");
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
        await multichainACL.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, accountAddress, extraDataV0);
      }
    });

    it("Should revert because the hostChainId is not registered in the GatewayConfig contract", async function () {
      // Check that allowing an account to use a ciphertext on a fake chain ID reverts
      await expect(
        multichainACL
          .connect(coprocessorTxSenders[0])
          .allowAccount(ctHandleFakeChainId, newAccountAddress, extraDataV0),
      )
        .revertedWithCustomError(multichainACL, "HostChainNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should allow account with 2 valid responses", async function () {
      // Trigger 2 allow calls with different coprocessor transaction senders
      await multichainACL.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, newAccountAddress, extraDataV0);
      const txResponse = multichainACL
        .connect(coprocessorTxSenders[1])
        .allowAccount(ctHandle, newAccountAddress, extraDataV0);

      // Consensus should be reached at the second response
      await expect(txResponse).to.emit(multichainACL, "AllowAccount").withArgs(ctHandle, newAccountAddress);
    });

    it("Should allow account with 2 valid responses and ignore the other valid one", async function () {
      // Trigger 3 allow account calls with different coprocessor transaction senders
      const txResponse1 = await multichainACL
        .connect(coprocessorTxSenders[0])
        .allowAccount(ctHandle, newAccountAddress, extraDataV0);
      await multichainACL.connect(coprocessorTxSenders[1]).allowAccount(ctHandle, newAccountAddress, extraDataV0);
      const txResponse3 = await multichainACL
        .connect(coprocessorTxSenders[2])
        .allowAccount(ctHandle, newAccountAddress, extraDataV0);

      // Check that the 1st and 3rd responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet
      // - 3rd response is ignored (not reverted) even though it is late
      await expect(txResponse1).to.not.emit(multichainACL, "AllowAccount");
      await expect(txResponse3).to.not.emit(multichainACL, "AllowAccount");
    });

    it("Should get all valid coprocessor transaction senders from allow account consensus", async function () {
      // Trigger an allow account calls using the first coprocessor transaction sender
      await multichainACL.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, newAccountAddress, extraDataV0);

      const expectedCoprocessorTxSenders1 = coprocessorTxSenders.slice(0, 1).map((s) => s.address);

      // Get the coprocessor transaction sender that answered first, before the consensus is reached
      // Since the consensus is directly made in the "request" call, the list represents the coprocessor
      // transaction sender that answered, and is accessible before the consensus is reached
      const proofRejectionConsensusTxSenders1 = await multichainACL.getAllowAccountConsensusTxSenders(
        ctHandle,
        newAccountAddress,
      );
      expect(proofRejectionConsensusTxSenders1).to.deep.equal(expectedCoprocessorTxSenders1);

      // Trigger an allow account calls using the second coprocessor transaction sender
      await multichainACL.connect(coprocessorTxSenders[1]).allowAccount(ctHandle, newAccountAddress, extraDataV0);

      const expectedCoprocessorTxSenders2 = coprocessorTxSenders.slice(0, 2).map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // first 2 coprocessor transaction senders, at the moment the consensus is reached
      const proofRejectionConsensusTxSenders2 = await multichainACL.getAllowAccountConsensusTxSenders(
        ctHandle,
        newAccountAddress,
      );
      expect(proofRejectionConsensusTxSenders2).to.deep.equal(expectedCoprocessorTxSenders2);

      // Trigger an allow account calls using the third coprocessor transaction sender
      await multichainACL.connect(coprocessorTxSenders[2]).allowAccount(ctHandle, newAccountAddress, extraDataV0);

      const expectedCoprocessorTxSenders3 = coprocessorTxSenders.map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the 3
      // coprocessor transaction senders, after the consensus is reached
      const proofRejectionConsensusTxSenders3 = await multichainACL.getAllowAccountConsensusTxSenders(
        ctHandle,
        newAccountAddress,
      );
      expect(proofRejectionConsensusTxSenders3).to.deep.equal(expectedCoprocessorTxSenders3);
    });

    it("Should revert because coprocessor tries to allow account twice", async function () {
      await expect(multichainACL.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, accountAddress, extraDataV0))
        .revertedWithCustomError(multichainACL, "CoprocessorAlreadyAllowedAccount")
        .withArgs(ctHandle, accountAddress, coprocessorTxSenders[0].address);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(multichainACL.connect(fakeTxSender).allowAccount(ctHandle, newAccountAddress, extraDataV0))
        .revertedWithCustomError(multichainACL, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should be true because the account is allowed to use the ciphertext", async function () {
      expect(await multichainACL.connect(coprocessorTxSenders[0]).isAccountAllowed(ctHandle, accountAddress)).to.be
        .true;
    });

    it("Should be false because the account is not allowed to use the ciphertext", async function () {
      expect(await multichainACL.connect(coprocessorTxSenders[0]).isAccountAllowed(ctHandle, newAccountAddress)).to.be
        .false;
    });

    it("Should be false because the handle has not been allowed to be used by anyone", async function () {
      expect(await multichainACL.connect(coprocessorTxSenders[0]).isAccountAllowed(newCtHandle, accountAddress)).to.be
        .false;
    });
  });

  describe("Allow public decrypt", async function () {
    beforeEach(async function () {
      // Allow the handle to be publicly decrypted
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await multichainACL.connect(coprocessorTxSenders[i]).allowPublicDecrypt(ctHandle, extraDataV0);
      }
    });

    it("Should revert because the hostChainId is not registered in the GatewayConfig contract", async function () {
      await expect(multichainACL.connect(coprocessorTxSenders[0]).allowPublicDecrypt(ctHandleFakeChainId, extraDataV0))
        .revertedWithCustomError(multichainACL, "HostChainNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should allow for public decryption with 2 valid responses", async function () {
      // Trigger 2 allow calls with different coprocessor transaction senders
      await multichainACL.connect(coprocessorTxSenders[0]).allowPublicDecrypt(newCtHandle, extraDataV0);
      const txResponse = multichainACL.connect(coprocessorTxSenders[1]).allowPublicDecrypt(newCtHandle, extraDataV0);

      // Consensus should be reached at the second response
      await expect(txResponse).to.emit(multichainACL, "AllowPublicDecrypt").withArgs(newCtHandle);
    });

    it("Should allow public decryption with 2 valid responses and ignore the other valid one", async function () {
      // Trigger 3 allow public decryption calls with different coprocessor transaction senders
      const txResponse1 = await multichainACL
        .connect(coprocessorTxSenders[0])
        .allowPublicDecrypt(newCtHandle, extraDataV0);
      await multichainACL.connect(coprocessorTxSenders[1]).allowPublicDecrypt(newCtHandle, extraDataV0);
      const txResponse3 = await multichainACL
        .connect(coprocessorTxSenders[2])
        .allowPublicDecrypt(newCtHandle, extraDataV0);

      // Check that the 1st and 3rd responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet
      // - 3rd response is ignored (not reverted) even though it is late
      await expect(txResponse1).to.not.emit(multichainACL, "AllowPublicDecrypt");
      await expect(txResponse3).to.not.emit(multichainACL, "AllowPublicDecrypt");
    });

    it("Should get all valid coprocessor transaction senders from allow public decryption consensus", async function () {
      // Trigger an allow public decryption calls using the first coprocessor transaction sender
      await multichainACL.connect(coprocessorTxSenders[0]).allowPublicDecrypt(newCtHandle, extraDataV0);

      const expectedCoprocessorTxSenders1 = coprocessorTxSenders.slice(0, 1).map((s) => s.address);

      // Get the coprocessor transaction sender that answered first, before the consensus is reached
      // Since the consensus is directly made in the "request" call, the list represents the coprocessor
      // transaction sender that answered, and is accessible before the consensus is reached
      const proofRejectionConsensusTxSenders1 =
        await multichainACL.getAllowPublicDecryptConsensusTxSenders(newCtHandle);
      expect(proofRejectionConsensusTxSenders1).to.deep.equal(expectedCoprocessorTxSenders1);

      // Trigger an allow public decryption calls using the second coprocessor transaction sender
      await multichainACL.connect(coprocessorTxSenders[1]).allowPublicDecrypt(newCtHandle, extraDataV0);

      const expectedCoprocessorTxSenders2 = coprocessorTxSenders.slice(0, 2).map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // first 2 coprocessor transaction senders, at the moment the consensus is reached
      const proofRejectionConsensusTxSenders2 =
        await multichainACL.getAllowPublicDecryptConsensusTxSenders(newCtHandle);
      expect(proofRejectionConsensusTxSenders2).to.deep.equal(expectedCoprocessorTxSenders2);

      // Trigger an allow public decryption calls using the third coprocessor transaction sender
      await multichainACL.connect(coprocessorTxSenders[2]).allowPublicDecrypt(newCtHandle, extraDataV0);

      const expectedCoprocessorTxSenders3 = coprocessorTxSenders.map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the 3
      // coprocessor transaction senders, after the consensus is reached
      const proofRejectionConsensusTxSenders3 =
        await multichainACL.getAllowPublicDecryptConsensusTxSenders(newCtHandle);
      expect(proofRejectionConsensusTxSenders3).to.deep.equal(expectedCoprocessorTxSenders3);
    });

    it("Should revert because coprocessor tries to allow public decryption twice", async function () {
      await expect(multichainACL.connect(coprocessorTxSenders[0]).allowPublicDecrypt(ctHandle, extraDataV0))
        .revertedWithCustomError(multichainACL, "CoprocessorAlreadyAllowedPublicDecrypt")
        .withArgs(ctHandle, coprocessorTxSenders[0].address);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(multichainACL.connect(fakeTxSender).allowPublicDecrypt(newCtHandle, extraDataV0))
        .revertedWithCustomError(multichainACL, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should be true because the public decrypt is allowed", async function () {
      expect(await multichainACL.connect(coprocessorTxSenders[0]).isPublicDecryptAllowed(ctHandle)).to.be.true;
    });

    it("Should be false because the handle is not allowed to be publicly decrypted", async function () {
      expect(await multichainACL.connect(coprocessorTxSenders[0]).isPublicDecryptAllowed(newCtHandle)).to.be.false;
    });
  });

  describe("Delegate user decryption", async function () {
    it("Should delegate user decryption with 2 valid calls", async function () {
      // Trigger 2 delegate calls with different coprocessor transaction senders.
      await multichainACL
        .connect(coprocessorTxSenders[0])
        .delegateUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, delegationCounter);
      const txResponse = multichainACL
        .connect(coprocessorTxSenders[1])
        .delegateUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, delegationCounter);

      // Consensus should be reached at the second response.
      await expect(txResponse)
        .to.emit(multichainACL, "DelegateUserDecryption")
        .withArgs(hostChainId, delegator, delegate, contractAddress);
    });

    it("Should delegate user decryption with 2 valid calls and ignore the other valid one", async function () {
      // Trigger 3 delegate calls with different coprocessor transaction senders.
      const txResponse1 = await multichainACL
        .connect(coprocessorTxSenders[0])
        .delegateUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, delegationCounter);
      await multichainACL
        .connect(coprocessorTxSenders[1])
        .delegateUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, delegationCounter);
      const txResponse3 = await multichainACL
        .connect(coprocessorTxSenders[2])
        .delegateUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, delegationCounter);

      // Check that the 1st and 3rd responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet.
      // - 3rd response is ignored (not reverted) even though it is late.
      await expect(txResponse1).to.not.emit(multichainACL, "DelegateUserDecryption");
      await expect(txResponse3).to.not.emit(multichainACL, "DelegateUserDecryption");
    });

    it("Should revert because coprocessor tries to delegate user decryption twice", async function () {
      await multichainACL
        .connect(coprocessorTxSenders[0])
        .delegateUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, delegationCounter);
      await expect(
        multichainACL
          .connect(coprocessorTxSenders[0])
          .delegateUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, delegationCounter),
      )
        .revertedWithCustomError(multichainACL, "CoprocessorAlreadyDelegatedOrRevokedUserDecryption")
        .withArgs(
          hostChainId,
          delegator,
          delegate,
          contractAddress,
          expiryDate,
          delegationCounter,
          coprocessorTxSenders[0].address,
        );
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(
        multichainACL
          .connect(fakeTxSender)
          .delegateUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, delegationCounter),
      )
        .revertedWithCustomError(multichainACL, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should revert because delegation counter is too low", async function () {
      const tooLowDelegationCounter = 0;
      await multichainACL
        .connect(coprocessorTxSenders[0])
        .delegateUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, tooLowDelegationCounter);
      await expect(
        multichainACL
          .connect(coprocessorTxSenders[1])
          .delegateUserDecryption(
            hostChainId,
            delegator,
            delegate,
            contractAddress,
            expiryDate,
            tooLowDelegationCounter,
          ),
      )
        .revertedWithCustomError(multichainACL, "UserDecryptionDelegationCounterTooLow")
        .withArgs(tooLowDelegationCounter);
    });

    it("Should be false because the user decryption is not delegated", async function () {
      expect(await multichainACL.isUserDecryptionDelegated(hostChainId, delegator, delegate, contractAddress)).to.be
        .false;
    });

    it("Should be true because the user decryption is delegated", async function () {
      // Delegate the user decryption.
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await multichainACL
          .connect(coprocessorTxSenders[i])
          .delegateUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, delegationCounter);
      }

      expect(await multichainACL.isUserDecryptionDelegated(hostChainId, delegator, delegate, contractAddress)).to.be
        .true;
    });
  });

  describe("Revoke user decryption delegation", async function () {
    const revokeDelegationCounter = delegationCounter + 1;

    beforeEach(async function () {
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await multichainACL
          .connect(coprocessorTxSenders[i])
          .delegateUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, delegationCounter);
      }
    });

    it("Should revoke user decryption delegation with 2 valid calls", async function () {
      // Trigger 2 revoke calls with different coprocessor transaction senders.
      await multichainACL
        .connect(coprocessorTxSenders[0])
        .revokeUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, revokeDelegationCounter);
      const txResponse = multichainACL
        .connect(coprocessorTxSenders[1])
        .revokeUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, revokeDelegationCounter);

      // Consensus should be reached at the second response.
      await expect(txResponse)
        .to.emit(multichainACL, "RevokeUserDecryption")
        .withArgs(hostChainId, delegator, delegate, contractAddress);
    });

    it("Should revoke user decryption delegation with 2 valid calls and ignore the other valid one", async function () {
      // Trigger 3 revoke calls with different coprocessor transaction senders.
      const txResponse1 = await multichainACL
        .connect(coprocessorTxSenders[0])
        .revokeUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, revokeDelegationCounter);
      await multichainACL
        .connect(coprocessorTxSenders[1])
        .revokeUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, revokeDelegationCounter);
      const txResponse3 = await multichainACL
        .connect(coprocessorTxSenders[2])
        .revokeUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, revokeDelegationCounter);

      // Check that the 1st and 3rd responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet.
      // - 3rd response is ignored (not reverted) even though it is late.
      await expect(txResponse1).to.not.emit(multichainACL, "RevokeUserDecryption");
      await expect(txResponse3).to.not.emit(multichainACL, "RevokeUserDecryption");
    });

    it("Should revert because coprocessor tries to revoke user decryption delegation twice", async function () {
      await multichainACL
        .connect(coprocessorTxSenders[0])
        .revokeUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, revokeDelegationCounter);
      await expect(
        multichainACL
          .connect(coprocessorTxSenders[0])
          .revokeUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, revokeDelegationCounter),
      )
        .revertedWithCustomError(multichainACL, "CoprocessorAlreadyDelegatedOrRevokedUserDecryption")
        .withArgs(
          hostChainId,
          delegator,
          delegate,
          contractAddress,
          expiryDate,
          revokeDelegationCounter,
          coprocessorTxSenders[0].address,
        );
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(
        multichainACL
          .connect(fakeTxSender)
          .revokeUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, revokeDelegationCounter),
      )
        .revertedWithCustomError(multichainACL, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should revert because delegation counter is too low", async function () {
      // Execute a delegation with a higher counter first.
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await multichainACL
          .connect(coprocessorTxSenders[i])
          .delegateUserDecryption(
            hostChainId,
            delegator,
            delegate,
            contractAddress,
            expiryDate,
            revokeDelegationCounter + 1,
          );
      }

      // Now try to revoke with a too low counter.
      await multichainACL
        .connect(coprocessorTxSenders[0])
        .revokeUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, revokeDelegationCounter);
      await expect(
        multichainACL
          .connect(coprocessorTxSenders[1])
          .revokeUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, revokeDelegationCounter),
      )
        .revertedWithCustomError(multichainACL, "UserDecryptionDelegationCounterTooLow")
        .withArgs(revokeDelegationCounter);
    });

    it("Should be true because the user decryption is delegated", async function () {
      expect(await multichainACL.isUserDecryptionDelegated(hostChainId, delegator, delegate, contractAddress)).to.be
        .true;
    });

    it("Should be false because the user decryption is revoked", async function () {
      // Revoke the user decryption delegation.
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await multichainACL
          .connect(coprocessorTxSenders[i])
          .revokeUserDecryption(hostChainId, delegator, delegate, contractAddress, expiryDate, revokeDelegationCounter);
      }

      expect(await multichainACL.isUserDecryptionDelegated(hostChainId, delegator, delegate, contractAddress)).to.be
        .false;
    });
  });
});
