import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import hre from "hardhat";

import { GatewayConfig, MultichainACL, MultichainACL__factory } from "../typechain-types";
// The type needs to be imported separately because it is not properly detected by the linter
// as this type is defined as a shared structs instead of directly in the IMultichainACL interface
import { DelegationAccountsStruct } from "../typechain-types/contracts/interfaces/IMultichainACL";
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

  let gatewayConfig: GatewayConfig;
  let MultichainACL: MultichainACL;
  let coprocessorTxSenders: HardhatEthersSigner[];
  let owner: Wallet;
  let pauser: Wallet;

  beforeEach(async function () {
    // Initialize used global variables before each test
    const fixture = await loadFixture(loadTestVariablesFixture);
    gatewayConfig = fixture.gatewayConfig;
    MultichainACL = fixture.MultichainACL;
    coprocessorTxSenders = fixture.coprocessorTxSenders;
    owner = fixture.owner;
    pauser = fixture.pauser;
  });

  describe("Deployment", function () {
    let MultichainACLFactory: MultichainACL__factory;

    beforeEach(async function () {
      // Get the MultichainACL contract factory
      MultichainACLFactory = await hre.ethers.getContractFactory("MultichainACL", owner);
    });

    it("Should revert because initialization is not from an empty proxy", async function () {
      await expect(
        hre.upgrades.upgradeProxy(MultichainACL, MultichainACLFactory, {
          call: { fn: "initializeFromEmptyProxy" },
        }),
      ).to.be.revertedWithCustomError(MultichainACL, "NotInitializingFromEmptyProxy");
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
        await MultichainACL.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, accountAddress, extraDataV0);
      }
    });

    it("Should revert because the hostChainId is not registered in the GatewayConfig contract", async function () {
      // Check that allowing an account to use a ciphertext on a fake chain ID reverts
      await expect(
        MultichainACL.connect(coprocessorTxSenders[0]).allowAccount(
          ctHandleFakeChainId,
          newAccountAddress,
          extraDataV0,
        ),
      )
        .revertedWithCustomError(MultichainACL, "HostChainNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should allow account with 2 valid responses", async function () {
      // Trigger 2 allow calls with different coprocessor transaction senders
      await MultichainACL.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, newAccountAddress, extraDataV0);
      const txResponse = MultichainACL.connect(coprocessorTxSenders[1]).allowAccount(
        ctHandle,
        newAccountAddress,
        extraDataV0,
      );

      // Consensus should be reached at the second response
      await expect(txResponse).to.emit(MultichainACL, "AllowAccount").withArgs(ctHandle, newAccountAddress);
    });

    it("Should allow account with 2 valid responses and ignore the other valid one", async function () {
      // Trigger 3 allow account calls with different coprocessor transaction senders
      const txResponse1 = await MultichainACL.connect(coprocessorTxSenders[0]).allowAccount(
        ctHandle,
        newAccountAddress,
        extraDataV0,
      );
      await MultichainACL.connect(coprocessorTxSenders[1]).allowAccount(ctHandle, newAccountAddress, extraDataV0);
      const txResponse3 = await MultichainACL.connect(coprocessorTxSenders[2]).allowAccount(
        ctHandle,
        newAccountAddress,
        extraDataV0,
      );

      // Check that the 1st and 3rd responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet
      // - 3rd response is ignored (not reverted) even though it is late
      await expect(txResponse1).to.not.emit(MultichainACL, "AllowAccount");
      await expect(txResponse3).to.not.emit(MultichainACL, "AllowAccount");
    });

    it("Should get all valid coprocessor transaction senders from allow account consensus", async function () {
      // Trigger an allow account calls using the first coprocessor transaction sender
      await MultichainACL.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, newAccountAddress, extraDataV0);

      const expectedCoprocessorTxSenders1 = coprocessorTxSenders.slice(0, 1).map((s) => s.address);

      // Get the coprocessor transaction sender that answered first, before the consensus is reached
      // Since the consensus is directly made in the "request" call, the list represents the coprocessor
      // transaction sender that answered, and is accessible before the consensus is reached
      const proofRejectionConsensusTxSenders1 = await MultichainACL.getAllowAccountConsensusTxSenders(
        ctHandle,
        newAccountAddress,
      );
      expect(proofRejectionConsensusTxSenders1).to.deep.equal(expectedCoprocessorTxSenders1);

      // Trigger an allow account calls using the second coprocessor transaction sender
      await MultichainACL.connect(coprocessorTxSenders[1]).allowAccount(ctHandle, newAccountAddress, extraDataV0);

      const expectedCoprocessorTxSenders2 = coprocessorTxSenders.slice(0, 2).map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // first 2 coprocessor transaction senders, at the moment the consensus is reached
      const proofRejectionConsensusTxSenders2 = await MultichainACL.getAllowAccountConsensusTxSenders(
        ctHandle,
        newAccountAddress,
      );
      expect(proofRejectionConsensusTxSenders2).to.deep.equal(expectedCoprocessorTxSenders2);

      // Trigger an allow account calls using the third coprocessor transaction sender
      await MultichainACL.connect(coprocessorTxSenders[2]).allowAccount(ctHandle, newAccountAddress, extraDataV0);

      const expectedCoprocessorTxSenders3 = coprocessorTxSenders.map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the 3
      // coprocessor transaction senders, after the consensus is reached
      const proofRejectionConsensusTxSenders3 = await MultichainACL.getAllowAccountConsensusTxSenders(
        ctHandle,
        newAccountAddress,
      );
      expect(proofRejectionConsensusTxSenders3).to.deep.equal(expectedCoprocessorTxSenders3);
    });

    it("Should revert because coprocessor tries to allow account twice", async function () {
      await expect(MultichainACL.connect(coprocessorTxSenders[0]).allowAccount(ctHandle, accountAddress, extraDataV0))
        .revertedWithCustomError(MultichainACL, "CoprocessorAlreadyAllowedAccount")
        .withArgs(ctHandle, accountAddress, coprocessorTxSenders[0].address);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(MultichainACL.connect(fakeTxSender).allowAccount(ctHandle, newAccountAddress, extraDataV0))
        .revertedWithCustomError(MultichainACL, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should be true because the account is allowed to use the ciphertext", async function () {
      expect(await MultichainACL.connect(coprocessorTxSenders[0]).isAccountAllowed(ctHandle, accountAddress)).to.be
        .true;
    });

    it("Should be false because the account is not allowed to use the ciphertext", async function () {
      expect(await MultichainACL.connect(coprocessorTxSenders[0]).isAccountAllowed(ctHandle, newAccountAddress)).to.be
        .false;
    });

    it("Should be false because the handle has not been allowed to be used by anyone", async function () {
      expect(await MultichainACL.connect(coprocessorTxSenders[0]).isAccountAllowed(newCtHandle, accountAddress)).to.be
        .false;
    });
  });

  describe("Allow public decrypt", async function () {
    beforeEach(async function () {
      // Allow the handle to be publicly decrypted
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await MultichainACL.connect(coprocessorTxSenders[i]).allowPublicDecrypt(ctHandle, extraDataV0);
      }
    });

    it("Should revert because the hostChainId is not registered in the GatewayConfig contract", async function () {
      await expect(MultichainACL.connect(coprocessorTxSenders[0]).allowPublicDecrypt(ctHandleFakeChainId, extraDataV0))
        .revertedWithCustomError(MultichainACL, "HostChainNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should allow for public decryption with 2 valid responses", async function () {
      // Trigger 2 allow calls with different coprocessor transaction senders
      await MultichainACL.connect(coprocessorTxSenders[0]).allowPublicDecrypt(newCtHandle, extraDataV0);
      const txResponse = MultichainACL.connect(coprocessorTxSenders[1]).allowPublicDecrypt(newCtHandle, extraDataV0);

      // Consensus should be reached at the second response
      await expect(txResponse).to.emit(MultichainACL, "AllowPublicDecrypt").withArgs(newCtHandle);
    });

    it("Should allow public decryption with 2 valid responses and ignore the other valid one", async function () {
      // Trigger 3 allow public decryption calls with different coprocessor transaction senders
      const txResponse1 = await MultichainACL.connect(coprocessorTxSenders[0]).allowPublicDecrypt(
        newCtHandle,
        extraDataV0,
      );
      await MultichainACL.connect(coprocessorTxSenders[1]).allowPublicDecrypt(newCtHandle, extraDataV0);
      const txResponse3 = await MultichainACL.connect(coprocessorTxSenders[2]).allowPublicDecrypt(
        newCtHandle,
        extraDataV0,
      );

      // Check that the 1st and 3rd responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet
      // - 3rd response is ignored (not reverted) even though it is late
      await expect(txResponse1).to.not.emit(MultichainACL, "AllowPublicDecrypt");
      await expect(txResponse3).to.not.emit(MultichainACL, "AllowPublicDecrypt");
    });

    it("Should get all valid coprocessor transaction senders from allow public decryption consensus", async function () {
      // Trigger an allow public decryption calls using the first coprocessor transaction sender
      await MultichainACL.connect(coprocessorTxSenders[0]).allowPublicDecrypt(newCtHandle, extraDataV0);

      const expectedCoprocessorTxSenders1 = coprocessorTxSenders.slice(0, 1).map((s) => s.address);

      // Get the coprocessor transaction sender that answered first, before the consensus is reached
      // Since the consensus is directly made in the "request" call, the list represents the coprocessor
      // transaction sender that answered, and is accessible before the consensus is reached
      const proofRejectionConsensusTxSenders1 =
        await MultichainACL.getAllowPublicDecryptConsensusTxSenders(newCtHandle);
      expect(proofRejectionConsensusTxSenders1).to.deep.equal(expectedCoprocessorTxSenders1);

      // Trigger an allow public decryption calls using the second coprocessor transaction sender
      await MultichainACL.connect(coprocessorTxSenders[1]).allowPublicDecrypt(newCtHandle, extraDataV0);

      const expectedCoprocessorTxSenders2 = coprocessorTxSenders.slice(0, 2).map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // first 2 coprocessor transaction senders, at the moment the consensus is reached
      const proofRejectionConsensusTxSenders2 =
        await MultichainACL.getAllowPublicDecryptConsensusTxSenders(newCtHandle);
      expect(proofRejectionConsensusTxSenders2).to.deep.equal(expectedCoprocessorTxSenders2);

      // Trigger an allow public decryption calls using the third coprocessor transaction sender
      await MultichainACL.connect(coprocessorTxSenders[2]).allowPublicDecrypt(newCtHandle, extraDataV0);

      const expectedCoprocessorTxSenders3 = coprocessorTxSenders.map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the 3
      // coprocessor transaction senders, after the consensus is reached
      const proofRejectionConsensusTxSenders3 =
        await MultichainACL.getAllowPublicDecryptConsensusTxSenders(newCtHandle);
      expect(proofRejectionConsensusTxSenders3).to.deep.equal(expectedCoprocessorTxSenders3);
    });

    it("Should revert because coprocessor tries to allow public decryption twice", async function () {
      await expect(MultichainACL.connect(coprocessorTxSenders[0]).allowPublicDecrypt(ctHandle, extraDataV0))
        .revertedWithCustomError(MultichainACL, "CoprocessorAlreadyAllowedPublicDecrypt")
        .withArgs(ctHandle, coprocessorTxSenders[0].address);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(MultichainACL.connect(fakeTxSender).allowPublicDecrypt(newCtHandle, extraDataV0))
        .revertedWithCustomError(MultichainACL, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should be true because the public decrypt is allowed", async function () {
      expect(await MultichainACL.connect(coprocessorTxSenders[0]).isPublicDecryptAllowed(ctHandle)).to.be.true;
    });

    it("Should be false because the handle is not allowed to be publicly decrypted", async function () {
      expect(await MultichainACL.connect(coprocessorTxSenders[0]).isPublicDecryptAllowed(newCtHandle)).to.be.false;
    });
  });
});
