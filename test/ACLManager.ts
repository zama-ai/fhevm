import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { ACLManager } from "../typechain-types";
import { deployHTTPZFixture } from "./utils";

describe("ACLManager", function () {
  const keyId = 0; // Using exceptional first key (currentKeyId == 0). See {HTTPZ-activateKeyRequest}
  const ctHandle = 123;
  const chainId = 456;
  const ciphertext128 = "0x02";

  let aclManager: ACLManager;
  let coprocessorSigners: HardhatEthersSigner[];
  let fakeSigner: HardhatEthersSigner;

  async function deployACLManagerFixture() {
    const { httpz, coprocessorSigners, signers } = await deployHTTPZFixture();
    const ACLManager = await hre.ethers.getContractFactory("ACLManager");
    const CiphertextStorage = await hre.ethers.getContractFactory("CiphertextStorage");
    const ciphertextStorage = await CiphertextStorage.deploy(httpz);
    const aclManager = await ACLManager.deploy(httpz, ciphertextStorage);

    // Add the ciphertext to the CiphertextStorage contract state which will be used during the tests
    for (let i = 0; i < coprocessorSigners.length; i++) {
      await ciphertextStorage
        .connect(coprocessorSigners[i])
        .addCiphertext(ctHandle, keyId, chainId, "0x01", ciphertext128);
    }

    return { aclManager, ciphertextStorage, coprocessorSigners, signers };
  }

  beforeEach(async function () {
    // Initialize used global variables before each test
    const fixture = await loadFixture(deployACLManagerFixture);
    aclManager = fixture.aclManager;
    coprocessorSigners = fixture.coprocessorSigners;
    fakeSigner = fixture.signers[0];
  });

  describe("Allow user decrypt", async function () {
    const allowedAddress = "0x388C818CA8B9251b393131C08a736A67ccB19297";

    it("Should success", async function () {
      // When
      await aclManager.connect(coprocessorSigners[0]).allowUserDecrypt(chainId, ctHandle, allowedAddress);
      const txResponse = aclManager.connect(coprocessorSigners[1]).allowUserDecrypt(chainId, ctHandle, allowedAddress);

      // Then
      await expect(txResponse).to.emit(aclManager, "AllowUserDecrypt").withArgs(ctHandle, allowedAddress);
    });

    it("Should revert with CoprocessorHasAlreadyAllowed", async function () {
      // When
      await aclManager.connect(coprocessorSigners[0]).allowUserDecrypt(chainId, ctHandle, allowedAddress);
      const txResponse = aclManager.connect(coprocessorSigners[0]).allowUserDecrypt(chainId, ctHandle, allowedAddress);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "CoprocessorHasAlreadyAllowed")
        .withArgs(coprocessorSigners[0].address, ctHandle);
    });

    it("Should revert with InvalidCoprocessorSender", async function () {
      // When
      const txResponse = aclManager.connect(fakeSigner).allowUserDecrypt(chainId, ctHandle, allowedAddress);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "InvalidCoprocessorSender")
        .withArgs(fakeSigner.address);
    });

    it("Should revert with CiphertextHandleNotOnNetwork", async function () {
      // Given
      const fakeChainId = 12345;

      // When
      const txResponse = aclManager
        .connect(coprocessorSigners[0])
        .allowUserDecrypt(fakeChainId, ctHandle, allowedAddress);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "CiphertextHandleNotOnNetwork")
        .withArgs(ctHandle, fakeChainId);
    });
  });

  describe("Allow public decrypt", async function () {
    it("Should success", async function () {
      // When
      await aclManager.connect(coprocessorSigners[0]).allowPublicDecrypt(chainId, ctHandle);
      const txResponse = aclManager.connect(coprocessorSigners[1]).allowPublicDecrypt(chainId, ctHandle);

      // Then
      await expect(txResponse).to.emit(aclManager, "AllowPublicDecrypt").withArgs(ctHandle);
    });

    it("Should revert with CoprocessorHasAlreadyAllowed", async function () {
      // When
      await aclManager.connect(coprocessorSigners[0]).allowPublicDecrypt(chainId, ctHandle);
      const txResponse = aclManager.connect(coprocessorSigners[0]).allowPublicDecrypt(chainId, ctHandle);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "CoprocessorHasAlreadyAllowed")
        .withArgs(coprocessorSigners[0].address, ctHandle);
    });

    it("Should revert with InvalidCoprocessorSender", async function () {
      // When
      const txResponse = aclManager.connect(fakeSigner).allowPublicDecrypt(chainId, ctHandle);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "InvalidCoprocessorSender")
        .withArgs(fakeSigner.address);
    });

    it("Should revert with CiphertextHandleNotOnNetwork", async function () {
      // Given
      const fakeChainId = 12345;

      // When
      const txResponse = aclManager.connect(coprocessorSigners[0]).allowPublicDecrypt(fakeChainId, ctHandle);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "CiphertextHandleNotOnNetwork")
        .withArgs(ctHandle, fakeChainId);
    });
  });

  describe("Delegate account", async function () {
    // Given arbitrary delegator, delegatee and contract addresses
    const delegator = hre.ethers.Wallet.createRandom().address;
    const delegatee = hre.ethers.Wallet.createRandom().address;
    const allowedContract1 = hre.ethers.Wallet.createRandom().address;
    const allowedContract2 = hre.ethers.Wallet.createRandom().address;
    const allowedContract3 = hre.ethers.Wallet.createRandom().address;

    it("Should success", async function () {
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
        .withArgs(chainId, delegator, delegatee, allowedContract1);
      await expect(txResponse)
        .to.emit(aclManager, "DelegateAccount")
        .withArgs(chainId, delegator, delegatee, allowedContract2);
      await expect(txResponse)
        .to.emit(aclManager, "DelegateAccount")
        .withArgs(chainId, delegator, delegatee, allowedContract3);

      // Check that only three events were emitted: one for each allowed contract on the list
      const events = await aclManager.queryFilter(aclManager.filters.DelegateAccount());
      expect(events.length).to.equal(3);
    });

    it("Should revert with CoprocessorHasAlreadyDelegated", async function () {
      // When
      await aclManager
        .connect(coprocessorSigners[0])
        .delegateAccount(chainId, delegator, delegatee, [allowedContract1]);
      const txResponse = aclManager
        .connect(coprocessorSigners[0])
        .delegateAccount(chainId, delegator, delegatee, [allowedContract1]);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "CoprocessorHasAlreadyDelegated")
        .withArgs(coprocessorSigners[0].address);
    });

    it("Should revert with InvalidCoprocessorSender", async function () {
      // When
      const txResponse = aclManager
        .connect(fakeSigner)
        .delegateAccount(chainId, delegator, delegatee, [allowedContract1]);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "InvalidCoprocessorSender")
        .withArgs(fakeSigner.address);
    });
  });

  describe("Get user ciphertexts", async function () {
    const allowedUserAddress = hre.ethers.Wallet.createRandom().address;
    const allowedContractAddress = hre.ethers.Wallet.createRandom().address;
    const ctHandleContractPairs = [{ ctHandle, contractAddress: allowedContractAddress }];

    beforeEach(async function () {
      // Setup the user decrypt permission for the given chainId, ctHandle userAddress and contractAddress used during tests
      for (let i = 0; i < coprocessorSigners.length; i++) {
        await aclManager.connect(coprocessorSigners[i]).allowUserDecrypt(chainId, ctHandle, allowedUserAddress);
        await aclManager.connect(coprocessorSigners[i]).allowUserDecrypt(chainId, ctHandle, allowedContractAddress);
      }
    });

    it("Should success", async function () {
      // When
      const txResponse = await aclManager
        .connect(coprocessorSigners[0])
        .getUserCiphertexts(allowedUserAddress, ctHandleContractPairs);

      // Then
      expect(txResponse).to.deep.equal([[ctHandle, keyId, ciphertext128]]);
    });

    it("Should revert with UserDecryptNotAllowed", async function () {
      // Given
      const fakeUserAddress = hre.ethers.Wallet.createRandom().address;
      const fakeContractAddress = hre.ethers.Wallet.createRandom().address;
      const fakeCtHandleContractPairs = [{ ctHandle: ctHandle, contractAddress: fakeContractAddress }];

      // When
      const txResponse1 = aclManager
        .connect(coprocessorSigners[0])
        .getUserCiphertexts(fakeUserAddress, ctHandleContractPairs);
      const txResponse2 = aclManager
        .connect(coprocessorSigners[0])
        .getUserCiphertexts(allowedUserAddress, fakeCtHandleContractPairs);

      // Then, should revert as fakeUserAddress is not allowed to decrypt the ciphertext
      await expect(txResponse1)
        .revertedWithCustomError(aclManager, "UserDecryptNotAllowed")
        .withArgs(ctHandle, fakeUserAddress);
      // And fakeContractAddress (in fakeCtHandleContractPairs) is also not allowed to decrypt the ciphertext
      await expect(txResponse2)
        .revertedWithCustomError(aclManager, "UserDecryptNotAllowed")
        .withArgs(ctHandle, fakeContractAddress);
    });

    it("Should revert with TooManyContractsRequested", async function () {
      // Given an exceeded number of ctHandleContractPairs
      const exceededCtHandleContractPairs = [];
      for (let i = 0; i < 12; i++) {
        exceededCtHandleContractPairs.push({
          ctHandle: 123,
          contractAddress: hre.ethers.Wallet.createRandom().address,
        });
      }

      // When
      const txResponse = aclManager
        .connect(coprocessorSigners[0])
        .getUserCiphertexts(allowedUserAddress, exceededCtHandleContractPairs);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(aclManager, "TooManyContractsRequested")
        .withArgs(10, exceededCtHandleContractPairs.length);
    });
  });

  describe("Get public ciphertexts", async function () {
    beforeEach(async function () {
      // Setup the public decrypt permission for the given chainId and ctHandle used during tests
      for (let i = 0; i < coprocessorSigners.length; i++) {
        await aclManager.connect(coprocessorSigners[i]).allowPublicDecrypt(chainId, ctHandle);
      }
    });

    it("Should success", async function () {
      // When
      const txResponse = await aclManager.connect(coprocessorSigners[0]).getPublicCiphertexts([ctHandle]);

      // Then
      expect(txResponse).to.deep.equal([[ctHandle, keyId, ciphertext128]]);
    });

    it("Should revert with PublicDecryptNotAllowed", async function () {
      // Given
      const fakeCtHandle = 12345;

      // When
      const txResponse = aclManager.connect(coprocessorSigners[0]).getPublicCiphertexts([fakeCtHandle]);

      // Then
      await expect(txResponse).revertedWithCustomError(aclManager, "PublicDecryptNotAllowed").withArgs(fakeCtHandle);
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

    it("Should return true", async function () {
      // When
      const txResponse = await aclManager.isAccountDelegated(chainId, delegator, delegatee, allowedContracts);

      // Then
      expect(txResponse).to.equal(true);
    });

    it("Should return false", async function () {
      // Given
      const fakeChainId = 12345;
      const fakeDelegator = hre.ethers.Wallet.createRandom().address;
      const fakeDelegatee = hre.ethers.Wallet.createRandom().address;

      // When
      const txResponse1 = await aclManager.isAccountDelegated(fakeChainId, delegator, delegatee, allowedContracts);
      const txResponse2 = await aclManager.isAccountDelegated(chainId, fakeDelegator, delegatee, allowedContracts);
      const txResponse3 = await aclManager.isAccountDelegated(chainId, delegator, fakeDelegatee, allowedContracts);

      // Then
      expect(txResponse1).to.equal(false);
      expect(txResponse2).to.equal(false);
      expect(txResponse3).to.equal(false);
    });

    it("Should distinguish between differently ordered contract list", async function () {
      // Given
      const alteredAllowedContracts = [allowedContract2, allowedContract1];

      // When
      const txResponse = await aclManager.isAccountDelegated(chainId, delegator, delegatee, alteredAllowedContracts);

      // Then
      expect(txResponse).to.equal(false);
    });
  });
});
