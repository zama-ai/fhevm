import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { HDNodeWallet, Wallet } from "ethers";
import hre from "hardhat";

import { ACLManager, CiphertextManager, HTTPZ } from "../typechain-types";
import { createAndFundRandomUser, loadTestVariablesFixture } from "./utils";

describe("ACLManager", function () {
  const keyId = 0; // Using exceptional first key (currentKeyId == 0). See {HTTPZ-activateKeyRequest}
  const ctHandle = 2025;
  const chainId = 1;
  const ciphertextDigest = hre.ethers.hexlify(hre.ethers.randomBytes(32));
  const snsCiphertextDigest = hre.ethers.hexlify(hre.ethers.randomBytes(32));

  // Fake values
  const fakeCtHandle = 11111;
  const fakeChainId = 123;

  let httpz: HTTPZ;
  let aclManager: ACLManager;
  let ciphertextManager: CiphertextManager;
  let coprocessorSigners: Wallet[];
  let fakeSigner: HDNodeWallet;

  async function prepareACLManagerFixture() {
    const { httpz, aclManager, ciphertextManager, coprocessorSigners } = await loadFixture(loadTestVariablesFixture);

    // Add the ciphertext to the CiphertextManager contract state which will be used during the tests
    for (let i = 0; i < coprocessorSigners.length; i++) {
      await ciphertextManager
        .connect(coprocessorSigners[i])
        .addCiphertextMaterial(ctHandle, keyId, chainId, ciphertextDigest, snsCiphertextDigest);
    }

    const fakeSigner = await createAndFundRandomUser();
    return { httpz, aclManager, ciphertextManager, coprocessorSigners, fakeSigner };
  }

  beforeEach(async function () {
    // Initialize used global variables before each test
    const fixture = await loadFixture(prepareACLManagerFixture);
    httpz = fixture.httpz;
    aclManager = fixture.aclManager;
    ciphertextManager = fixture.ciphertextManager;
    coprocessorSigners = fixture.coprocessorSigners;
    fakeSigner = fixture.fakeSigner;
  });

  describe("Allow account", async function () {
    const allowedAddress = "0x388C818CA8B9251b393131C08a736A67ccB19297";

    it("Should allow account to use the ciphertext", async function () {
      // When
      await aclManager.connect(coprocessorSigners[0]).allowAccount(chainId, ctHandle, allowedAddress);
      const txResponse = aclManager.connect(coprocessorSigners[1]).allowAccount(chainId, ctHandle, allowedAddress);

      // Then
      await expect(txResponse).to.emit(aclManager, "AllowAccount").withArgs(ctHandle, allowedAddress);
    });

    it("Should revert with CoprocessorAlreadyAllowed", async function () {
      // When
      await aclManager.connect(coprocessorSigners[0]).allowAccount(chainId, ctHandle, allowedAddress);
      const txResponse = aclManager.connect(coprocessorSigners[0]).allowAccount(chainId, ctHandle, allowedAddress);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "CoprocessorAlreadyAllowed")
        .withArgs(coprocessorSigners[0].address, ctHandle);
    });

    it("Should revert because the signer is not a Coprocessor", async function () {
      // When
      const txResponse = aclManager.connect(fakeSigner).allowAccount(chainId, ctHandle, allowedAddress);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(fakeSigner.address, httpz.COPROCESSOR_ROLE());
    });

    // TODO: Replace test with pending allow logic tests
    // https://github.com/zama-ai/gateway-l2/issues/171
    // it("Should revert because the ciphertext is not on the network", async function () {
    //   // When
    //   const txResponse = aclManager
    //     .connect(coprocessorSigners[0])
    //     .allowAccount(fakeChainId, ctHandle, allowedAddress);

    //   // Then
    //   await expect(txResponse)
    //     .revertedWithCustomError(ciphertextManager, "CiphertextNotOnNetwork")
    //     .withArgs(ctHandle, fakeChainId);
    // });
  });

  describe("Allow public decrypt", async function () {
    it("Should allow for public decryption", async function () {
      // When
      await aclManager.connect(coprocessorSigners[0]).allowPublicDecrypt(chainId, ctHandle);
      const txResponse = aclManager.connect(coprocessorSigners[1]).allowPublicDecrypt(chainId, ctHandle);

      // Then
      await expect(txResponse).to.emit(aclManager, "AllowPublicDecrypt").withArgs(ctHandle);
    });

    it("Should revert with CoprocessorAlreadyAllowed", async function () {
      // When
      await aclManager.connect(coprocessorSigners[0]).allowPublicDecrypt(chainId, ctHandle);
      const txResponse = aclManager.connect(coprocessorSigners[0]).allowPublicDecrypt(chainId, ctHandle);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "CoprocessorAlreadyAllowed")
        .withArgs(coprocessorSigners[0].address, ctHandle);
    });

    it("Should revert because the signer is not a Coprocessor", async function () {
      // When
      const txResponse = aclManager.connect(fakeSigner).allowPublicDecrypt(chainId, ctHandle);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(fakeSigner.address, httpz.COPROCESSOR_ROLE());
    });

    // TODO: Replace test with pending allow logic tests
    // https://github.com/zama-ai/gateway-l2/issues/171
    // it("Should revert because the ciphertext is not on the network", async function () {
    //   // When
    //   const txResponse = aclManager.connect(coprocessorSigners[0]).allowPublicDecrypt(fakeChainId, ctHandle);

    //   // Then
    //   await expect(txResponse)
    //     .revertedWithCustomError(ciphertextManager, "CiphertextNotOnNetwork")
    //     .withArgs(ctHandle, fakeChainId);
    // });
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
        .connect(coprocessorSigners[0])
        .delegateAccount(chainId, delegator, delegatee, [allowedContract1, allowedContract2, allowedContract3]);
      const txResponse = aclManager
        .connect(coprocessorSigners[1])
        .delegateAccount(chainId, delegator, delegatee, [allowedContract1, allowedContract2, allowedContract3]);

      // Then
      await expect(txResponse)
        .to.emit(aclManager, "DelegateAccount")
        .withArgs(chainId, delegator, delegatee, [allowedContract1, allowedContract2, allowedContract3]);
    });

    it("Should revert with CoprocessorAlreadyDelegated", async function () {
      // When
      await aclManager
        .connect(coprocessorSigners[0])
        .delegateAccount(chainId, delegator, delegatee, [allowedContract1]);
      const txResponse = aclManager
        .connect(coprocessorSigners[0])
        .delegateAccount(chainId, delegator, delegatee, [allowedContract1]);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "CoprocessorAlreadyDelegated")
        .withArgs(coprocessorSigners[0].address, chainId, delegator, delegatee, [allowedContract1]);
    });

    it("Should revert because the signer is not a Coprocessor", async function () {
      // When
      const txResponse = aclManager
        .connect(fakeSigner)
        .delegateAccount(chainId, delegator, delegatee, [allowedContract1]);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(fakeSigner.address, httpz.COPROCESSOR_ROLE());
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
        .connect(coprocessorSigners[0])
        .delegateAccount(chainId, delegator, delegatee, exceededContracts);

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
      for (let i = 0; i < coprocessorSigners.length; i++) {
        await aclManager.connect(coprocessorSigners[i]).allowAccount(chainId, ctHandle, allowedUserAddress);
        await aclManager.connect(coprocessorSigners[i]).allowAccount(chainId, ctHandle, allowedContractAddress);
      }
    });

    it("Should check account is allowed to use the ciphertext", async function () {
      await aclManager.connect(coprocessorSigners[0]).checkAccountAllowed(allowedUserAddress, ctHandleContractPairs);
    });

    it("Should revert because user part of the contract addresses", async function () {
      const fakeCtHandleContractPairs = [{ ctHandle: ctHandle, contractAddress: allowedUserAddress }];

      // Check that the fakeUserAddress is not allowed to use the ciphertext
      await expect(
        aclManager.connect(coprocessorSigners[0]).checkAccountAllowed(allowedUserAddress, fakeCtHandleContractPairs),
      )
        .to.be.revertedWithCustomError(aclManager, "AccountAddressInContractAddresses")
        .withArgs(allowedUserAddress);
    });

    it("Should revert because user is not allowed to use the ciphertext", async function () {
      const fakeUserAddress = hre.ethers.Wallet.createRandom().address;

      // Check that the fakeUserAddress is not allowed to use the ciphertext
      await expect(
        aclManager.connect(coprocessorSigners[0]).checkAccountAllowed(fakeUserAddress, ctHandleContractPairs),
      )
        .to.be.revertedWithCustomError(aclManager, "AccountNotAllowedToUseCiphertext")
        .withArgs(ctHandle, fakeUserAddress);
    });

    it("Should revert because contract is not allowed to use the ciphertext", async function () {
      const fakeContractAddress = hre.ethers.Wallet.createRandom().address;
      const fakeCtHandleContractPairs = [{ ctHandle: ctHandle, contractAddress: fakeContractAddress }];

      // Check that the fakeContractAddress is not allowed to use the ciphertext
      await expect(
        aclManager.connect(coprocessorSigners[0]).checkAccountAllowed(allowedUserAddress, fakeCtHandleContractPairs),
      )
        .to.be.revertedWithCustomError(aclManager, "ContractNotAllowedToUseCiphertext")
        .withArgs(ctHandle, fakeContractAddress);
    });
  });

  describe("Check public decrypt allowed", async function () {
    beforeEach(async function () {
      // Setup the public decrypt permission for the given chainId and ctHandle used during tests
      for (let i = 0; i < coprocessorSigners.length; i++) {
        await aclManager.connect(coprocessorSigners[i]).allowPublicDecrypt(chainId, ctHandle);
      }
    });

    it("Should check public decrypt is allowed", async function () {
      await aclManager.connect(coprocessorSigners[0]).checkPublicDecryptAllowed([ctHandle]);
    });

    it("Should revert with PublicDecryptNotAllowed", async function () {
      // Check that the fakeCtHandle is not allowed for public decryption
      await expect(aclManager.connect(coprocessorSigners[0]).checkPublicDecryptAllowed([fakeCtHandle]))
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
      // Setup the account delegation for the given chainId, delegator, delegatee and allowedContracts used during tests
      for (let i = 0; i < coprocessorSigners.length; i++) {
        await aclManager
          .connect(coprocessorSigners[i])
          .delegateAccount(chainId, delegator, delegatee, allowedContracts);
      }
    });

    it("Should check account is delegated", async function () {
      await aclManager.checkAccountDelegated(chainId, delegator, delegatee, allowedContracts);
    });

    it("Should revert because none of the inputs has account delegation", async function () {
      // Given
      const fakeDelegator = hre.ethers.Wallet.createRandom().address;
      const fakeDelegatee = hre.ethers.Wallet.createRandom().address;

      // When
      const txResponse1 = aclManager.checkAccountDelegated(fakeChainId, delegator, delegatee, allowedContracts);
      const txResponse2 = aclManager.checkAccountDelegated(chainId, fakeDelegator, delegatee, allowedContracts);
      const txResponse3 = aclManager.checkAccountDelegated(chainId, delegator, fakeDelegatee, allowedContracts);

      // Then
      await expect(txResponse1)
        .revertedWithCustomError(aclManager, "AccountNotDelegated")
        .withArgs(fakeChainId, delegator, delegatee, allowedContracts[0]);
      await expect(txResponse2)
        .revertedWithCustomError(aclManager, "AccountNotDelegated")
        .withArgs(chainId, fakeDelegator, delegatee, allowedContracts[0]);
      await expect(txResponse3)
        .revertedWithCustomError(aclManager, "AccountNotDelegated")
        .withArgs(chainId, delegator, fakeDelegatee, allowedContracts[0]);
    });

    it("Should not distinguish between differently ordered contract list", async function () {
      // Given
      const alteredAllowedContracts = [allowedContract2, allowedContract1];

      await aclManager.checkAccountDelegated(chainId, delegator, delegatee, alteredAllowedContracts);
    });
  });
});
