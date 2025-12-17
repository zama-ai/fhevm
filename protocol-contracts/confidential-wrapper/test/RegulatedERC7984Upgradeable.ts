import { ethers, fhevm, upgrades } from "hardhat";
import { RegulatedERC7984Upgradeable, AdminProvider } from "../types";
import { expect } from "chai";
import { getMintEvent, getBurnEvent, getTransferInfoEvent, getConfidentialBalance, getTransferFeeInfoEvent, checkTotalSupply, getTokenRegulatorUpdatedEvent } from "./utils";
import { getSigners, Signers } from "./signers";
import { FhevmType } from "@fhevm/hardhat-plugin";
import { deployAdminProviderFixture, deployBurnableConfidentialErc20Fixture, deployConfidentialErc20Fixture, deploySanctionsListFixture } from "./fixtures";


describe("RegulatedERC7984Upgradeable", function () {
  let signers: Signers;
  let adminProvider: AdminProvider;
  let adminProviderAddress: string;
  let cErc20: RegulatedERC7984Upgradeable;
  let cErc20Address: string;

  before(async function () {
    signers = await getSigners();
  });

  beforeEach(async () => {
    ({ cErc20, cErc20Address, adminProvider, adminProviderAddress } = await deployConfidentialErc20Fixture(signers));
    await cErc20.grantRole(await cErc20.WRAPPER_ROLE(), signers.deployer);
  });

  describe("initializer", function () {
    it("should set correct attribute values at initialization", async function () {
      expect(await cErc20.name()).to.equal("Naraggara");
      expect(await cErc20.symbol()).to.equal("NARA");
      expect(await cErc20.decimals()).to.equal(6);
      expect(await cErc20.rate()).to.equal(1);
      expect(await cErc20.adminProvider()).to.equal(adminProvider);


      const totalSupplyHandle = await cErc20.confidentialTotalSupply();

      await checkTotalSupply(cErc20, 0);
    });

    it("should set the correct admin role", async function () {
      expect(await cErc20.hasRole(await cErc20.DEFAULT_ADMIN_ROLE(), signers.deployer)).to.equal(true);
    });

    it("cannot call initialize twice", async function () {
      await expect(
        cErc20.initialize("Naraggara", "NARA", 6, signers.deployer.address, 1, ethers.ZeroAddress, adminProviderAddress, signers.deployer.address),
      ).to.be.revertedWithCustomError(cErc20, "InvalidInitialization");
    });

    it("should revert when rate is 0", async function () {
      const RegulatedERC7984Factory = await ethers.getContractFactory("RegulatedERC7984Upgradeable");
      const initializerParams = [
          "Test Token",
          "TEST",
          6,
          signers.deployer.address,
          0, // Invalid rate: 0
          ethers.ZeroAddress,
          adminProviderAddress,
          signers.deployer.address // wrapperSetter
      ];
      await expect(
        upgrades.deployProxy(RegulatedERC7984Factory, initializerParams)
      ).to.be.revertedWithCustomError(RegulatedERC7984Factory, "InvalidRate");
    });

    it("should revert when rate exceeds maximum (10^24)", async function () {
      const RegulatedERC7984Factory = await ethers.getContractFactory("RegulatedERC7984Upgradeable");
      const maxRate = BigInt(10) ** BigInt(24);
      const invalidRate = maxRate + BigInt(1);
      const initializerParams = [
          "Test Token",
          "TEST",
          30,
          signers.deployer.address,
          invalidRate, // Invalid rate: exceeds 10^24
          ethers.ZeroAddress,
          adminProviderAddress,
          signers.deployer.address // wrapperSetter
      ];
      await expect(
        upgrades.deployProxy(RegulatedERC7984Factory, initializerParams)
      ).to.be.revertedWithCustomError(RegulatedERC7984Factory, "InvalidRate");
    });

    it("should accept rate of 1 (tokens with <= 6 decimals)", async function () {
      const RegulatedERC7984Factory = await ethers.getContractFactory("RegulatedERC7984Upgradeable");
      const initializerParams = [
          "USDC",
          "USDC",
          6,
          signers.deployer.address,
          1, // Valid rate for 6 decimals
          ethers.ZeroAddress,
          adminProviderAddress,
          signers.deployer.address // wrapperSetter
      ];
      const newCErc20 = await upgrades.deployProxy(RegulatedERC7984Factory, initializerParams)
      await newCErc20.waitForDeployment();
      expect(await newCErc20.rate()).to.equal(1);
    });

    it("should accept rate of 10^12 (18 decimal tokens)", async function () {
      const RegulatedERC7984Factory = await ethers.getContractFactory("RegulatedERC7984Upgradeable");
      const rate = BigInt(10) ** BigInt(12); // 18 decimals - 6 = 12
      const initializerParams = [
          "Ethereum",
          "ETH",
          6, // Confidential token has 6 decimals
          signers.deployer.address,
          rate, // Valid rate for 18 decimal original token
          ethers.ZeroAddress,
          adminProviderAddress,
          signers.deployer.address // wrapperSetter
      ];
      const newCErc20 = await upgrades.deployProxy(RegulatedERC7984Factory, initializerParams)
      await newCErc20.waitForDeployment();
      expect(await newCErc20.rate()).to.equal(rate);
    });

    it("should accept maximum rate of 10^24 (30 decimal tokens)", async function () {
      const RegulatedERC7984Factory = await ethers.getContractFactory("RegulatedERC7984Upgradeable");
      const maxRate = BigInt(10) ** BigInt(24); // 30 decimals - 6 = 24
      const rate = BigInt(10) ** BigInt(12); // 18 decimals - 6 = 12
      const initializerParams = [
          "High Decimal Token",
          "HDT",
          6, // Confidential token has 6 decimals
          signers.deployer.address,
          maxRate, // Maximum valid rate for 30 decimal original token
          ethers.ZeroAddress,
          adminProviderAddress,
          signers.deployer.address // wrapperSetter
      ];
      const newCErc20 = await upgrades.deployProxy(RegulatedERC7984Factory, initializerParams)
      await newCErc20.waitForDeployment();
      expect(await newCErc20.rate()).to.equal(maxRate);
    });

    it("should prevent implementation contract from initializing", async function () {
      const RegulatedERC7984Factory = await ethers.getContractFactory("RegulatedERC7984Upgradeable");
      const newCErc20 = await RegulatedERC7984Factory.deploy();
      await newCErc20.waitForDeployment();
      await expect(
        newCErc20.initialize(
          "High Decimal Token",
          "HDT",
          6,
          signers.deployer.address,
          1,
          ethers.ZeroAddress,
          adminProviderAddress,
          signers.deployer.address // wrapperSetter
        )
      ).to.be.revertedWithCustomError(RegulatedERC7984Factory, "InvalidInitialization");
    });
  })

  describe("setWrapper", function () {
    it("should allow WRAPPER_SETTER_ROLE to set wrapper once", async function () {
      // Deploy a fresh token without wrapper set
      const RegulatedERC7984Factory = await ethers.getContractFactory("RegulatedERC7984Upgradeable");
      const initializerParams = [
          "Test Token",
          "TEST",
          6,
          signers.deployer.address, // admin
          1,
          ethers.ZeroAddress,
          await adminProvider.getAddress(),
          signers.alice.address // alice gets WRAPPER_SETTER_ROLE
      ];
      const newCErc20 = await upgrades.deployProxy(RegulatedERC7984Factory, initializerParams);
      await newCErc20.waitForDeployment();

      // Alice (WRAPPER_SETTER_ROLE) should be able to set wrapper
      await newCErc20.connect(signers.alice).setWrapper(signers.bob.address);

      // Verify bob now has WRAPPER_ROLE
      expect(await newCErc20.hasRole(await newCErc20.WRAPPER_ROLE(), signers.bob.address)).to.equal(true);
    });

    it("should revert with WrapperAlreadySet when trying to set wrapper twice", async function () {
      // Deploy a fresh token without wrapper set
      const RegulatedERC7984Factory = await ethers.getContractFactory("RegulatedERC7984Upgradeable");
      const initializerParams = [
          "Test Token",
          "TEST",
          6,
          signers.deployer.address, // admin
          1,
          ethers.ZeroAddress,
          await adminProvider.getAddress(),
          signers.alice.address // alice gets WRAPPER_SETTER_ROLE
      ];
      const newCErc20 = await upgrades.deployProxy(RegulatedERC7984Factory, initializerParams);
      await newCErc20.waitForDeployment();

      // Alice sets wrapper first time
      await newCErc20.connect(signers.alice).setWrapper(signers.bob.address);

      // Try to set wrapper second time - should revert
      await expect(
        newCErc20.connect(signers.alice).setWrapper(signers.charlie.address)
      ).to.be.revertedWithCustomError(newCErc20, "WrapperAlreadySet");
    });

    it("should prevent non-WRAPPER_SETTER_ROLE from calling setWrapper", async function () {
      // Deploy a fresh token without wrapper set
      const RegulatedERC7984Factory = await ethers.getContractFactory("RegulatedERC7984Upgradeable");
      const initializerParams = [
          "Test Token",
          "TEST",
          6,
          signers.deployer.address, // admin
          1,
          ethers.ZeroAddress,
          await adminProvider.getAddress(),
          signers.alice.address // only alice gets WRAPPER_SETTER_ROLE
      ];
      const newCErc20 = await upgrades.deployProxy(RegulatedERC7984Factory, initializerParams);
      await newCErc20.waitForDeployment();

      // Bob (without WRAPPER_SETTER_ROLE) should not be able to set wrapper
      const WRAPPER_SETTER_ROLE = await newCErc20.WRAPPER_SETTER_ROLE();
      await expect(
        newCErc20.connect(signers.bob).setWrapper(signers.charlie.address)
      ).to.be.revertedWithCustomError(newCErc20, "AccessControlUnauthorizedAccount")
        .withArgs(signers.bob.address, WRAPPER_SETTER_ROLE);
    });
  });

  describe("mint", function () {
    it("should mint correct amount and emit event", async function () {
      const amount = 1000;
      const txId = await cErc20.nextTxId();
      const isRoyalty = true;
      const transaction = await cErc20.mint(signers.alice, amount);
      const receipt = await transaction.wait();

      await checkTotalSupply(cErc20, amount);

      const events = getMintEvent(receipt);

      expect(events.length).to.be.equal(1);
      const event = events[0];
      expect(event.args[0]).to.be.equal(signers.alice);
      expect(event.args[1]).to.be.equal(amount);
      expect(event.args[2]).to.be.equal(txId);

      // Reencrypt Alice's balance from Alice
      const balanceHandleAlice = await cErc20.confidentialBalanceOf(signers.alice);
      const balanceAliceFromAlice = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        balanceHandleAlice,
        cErc20Address,
        signers.alice,
      );

      expect(balanceAliceFromAlice).to.equal(amount);

      // Reencrypt Alice's balance from regulator
      const balanceAliceFromAdmin = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        balanceHandleAlice,
        cErc20Address,
        signers.regulator,
      );

      expect(balanceAliceFromAdmin).to.equal(amount);
    });

    it("should prevent minting to sanctioned address", async function () {
      const { sanctionsList } = await deploySanctionsListFixture();
      const { adminProvider } = await deployAdminProviderFixture(signers, sanctionsList);
      const { cErc20 } = await deployConfidentialErc20Fixture(signers, adminProvider);

      // Add alice to sanctions list
      await sanctionsList.addToSanctionsList([signers.alice.address]);

      // Try to mint to sanctioned address
      await expect(
        cErc20.mint(signers.alice.address, 1000)
      ).to.be.revertedWithCustomError(cErc20, "SanctionedAddress")
        .withArgs(signers.alice.address);
    });

    it("should allow minting to non-sanctioned address", async function () {
      const { sanctionsList } = await deploySanctionsListFixture();
      const { adminProvider } = await deployAdminProviderFixture(signers, sanctionsList);
      const { cErc20 } = await deployConfidentialErc20Fixture(signers, adminProvider);

      // Sanction someone else, not alice
      await sanctionsList.addToSanctionsList([signers.bob.address]);

      // Should be able to mint to alice (not sanctioned)
      await expect(
        cErc20.mint(signers.alice.address, 1000)
      ).to.not.be.reverted;
    });

    it("should work without sanctions list", async function () {
      // Use the regular fixture without sanctions list
      await expect(
        cErc20.mint(signers.alice.address, 1000)
      ).to.not.be.reverted;
    });

    it("should only allow WRAPPER_ROLE to mint tokens", async function () {
      await expect(
        cErc20.connect(signers.alice).mint(signers.alice, 1000)
      ).to.be.revertedWithCustomError(cErc20, "AccessControlUnauthorizedAccount")
        .withArgs(signers.alice.address, await cErc20.WRAPPER_ROLE());
    });
  });

  describe("burn", function () {
    it("should burn correct amount and emit event", async function () {
      const { burnableCErc20, burnableCErc20Address } = await deployBurnableConfidentialErc20Fixture(signers);

      const amount = 1000;
      const txId = await burnableCErc20.nextTxId();
      const isRoyalty = false;

      await burnableCErc20.mint(signers.deployer, amount);

      const encryptedAmount = await fhevm
        .createEncryptedInput(burnableCErc20Address, signers.deployer.address)
        .add64(amount)
        .encrypt();

      const transaction = await burnableCErc20
        .connect(signers.deployer)
        ["burn(bytes32,bytes)"]
        (encryptedAmount.handles[0], encryptedAmount.inputProof);
      const receipt = await transaction.wait();

      await checkTotalSupply(cErc20, 0);

      const events = getBurnEvent(receipt);
      expect(events.length).to.be.equal(1);
      const burnEvent = events[0];
      expect(burnEvent.args[0]).to.equal(signers.deployer);
      expect(burnEvent.args[2]).to.equal(txId + BigInt(1));
      const burnAmountHandle = burnEvent.args[1];

      const burnAmount = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        burnAmountHandle,
        burnableCErc20Address,
        signers.deployer,
      );
      expect(burnAmount).to.be.equal(amount);

      expect(await getConfidentialBalance(burnableCErc20, signers.deployer)).to.equal(0);
    });

    it("should only allow WRAPPER_ROLE to burn tokens", async function () {
      const { burnableCErc20, burnableCErc20Address } = await deployBurnableConfidentialErc20Fixture(signers);

      const encryptedAmount = await fhevm
        .createEncryptedInput(burnableCErc20Address, signers.alice.address)
        .add64(100)
        .encrypt();

      const transaction = burnableCErc20
        .connect(signers.alice)
        ["burn(bytes32,bytes)"]
        (encryptedAmount.handles[0], encryptedAmount.inputProof);

      await expect(transaction).to.be.revertedWithCustomError(cErc20, "AccessControlUnauthorizedAccount")
        .withArgs(signers.alice.address, await cErc20.WRAPPER_ROLE());
    });
  });

  describe("confidentialTransfer", function () {
    it("should transfer tokens between two users without fees", async function () {
      const aliceInitialAmount = 10_000;
      const transaction = await cErc20.mint(signers.alice, aliceInitialAmount);
      const t1 = await transaction.wait();
      expect(t1?.status).to.eq(1);

      const transferAmount = 1337;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const txId = await cErc20.nextTxId();

      const tx = await cErc20
        .connect(signers.alice)
        [
          "confidentialTransfer(address,bytes32,bytes)"
        ](signers.bob, encryptedTransferAmount.handles[0], encryptedTransferAmount.inputProof);
      const t2 = await tx.wait();
      expect(t2?.status).to.eq(1);

      await checkTotalSupply(cErc20, aliceInitialAmount);

      // admin, alice & bob should be able to decrypt amount in event
      const events = getTransferInfoEvent(t2);
      expect(events.length).to.be.equal(1);
      const event = events[0];
      expect(event.args[0]).to.be.equal(signers.alice);
      expect(event.args[1]).to.be.equal(signers.bob);
      expect(event.args[3]).to.be.equal(txId);
      const transferAmountHandle = event.args[2];

      // check reencrypt transfer amount handle
      for (const signer of [signers.regulator, signers.alice, signers.bob]) {
        const transferAmountFromSigner = await fhevm.userDecryptEuint(
          FhevmType.euint64,
          transferAmountHandle,
          cErc20Address,
          signer,
        );
        expect(transferAmountFromSigner).to.equal(transferAmount);
      }

      // charlie should NOT be able to decrypt amount in event
      await expect(
        fhevm.userDecryptEuint(FhevmType.euint64, transferAmountHandle, cErc20Address, signers.charlie),
      ).to.be.rejectedWith(
        `User ${signers.charlie.address} is not authorized to user decrypt handle ${transferAmountHandle}!`,
      );

      // Royalties balance is not initialized (ie is 0)
      await expect(
        getConfidentialBalance(cErc20, signers.royalties)
      ).to.be.rejectedWith("Handle is not initialized");

      // Reencrypt Alice's balance from Alice
      const balanceHandleAlice = await cErc20.confidentialBalanceOf(signers.alice);
      const balanceAlice = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        balanceHandleAlice,
        cErc20Address,
        signers.alice,
      );
      expect(balanceAlice).to.equal(aliceInitialAmount - transferAmount);

      // Reencrypt Bob's balance from Bob
      const balanceHandleBob = await cErc20.confidentialBalanceOf(signers.bob);
      const balanceBob = await fhevm.userDecryptEuint(FhevmType.euint64, balanceHandleBob, cErc20Address, signers.bob);
      expect(balanceBob).to.equal(transferAmount);

      // on the other hand, Bob should be unable to read Alice's balance
      await expect(
        fhevm.userDecryptEuint(FhevmType.euint64, balanceHandleAlice, cErc20Address, signers.bob),
      ).to.be.rejectedWith(
        `User ${signers.bob.address} is not authorized to user decrypt handle ${balanceHandleAlice}!`,
      );

      // and should be impossible to call reencrypt if contractAddress === userAddress
      await expect(
        fhevm.userDecryptEuint(FhevmType.euint64, balanceHandleAlice, signers.alice.address, signers.alice),
      ).to.be.rejectedWith(
        `userAddress ${signers.alice.address} should not be equal to contractAddress when requesting decryption!`,
      );

      const transferFeeInfoEvents = getTransferFeeInfoEvent(t2);
      expect(transferFeeInfoEvents.length).to.equal(0);

      expect(await cErc20.nextTxId()).to.equal(txId + BigInt(1));
    });

    it("should not transfer tokens between two users when sending more than balance", async function () {
      const transaction = await cErc20.mint(signers.alice, 1000);
      await transaction.wait();

      const transferAmount = 1337;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const txId = await cErc20.nextTxId();

      const tx = await cErc20
        .connect(signers.alice)
        [
          "confidentialTransfer(address,bytes32,bytes)"
        ](signers.bob, encryptedTransferAmount.handles[0], encryptedTransferAmount.inputProof);
      const receipt = await tx.wait();

      expect(await cErc20.nextTxId()).to.equal(txId + BigInt(1));

      const events = getTransferInfoEvent(receipt);
      expect(events.length).to.be.equal(1);
      const transferEvent = events[0];
      expect(transferEvent.args[0]).to.be.equal(signers.alice);
      expect(transferEvent.args[1]).to.be.equal(signers.bob);
      expect(transferEvent.args[3]).to.be.equal(txId);
      const transferAmountHandle = transferEvent.args[2];

      // check reencrypt transfer amount handle
      for (const signer of [signers.regulator, signers.alice, signers.bob]) {
        const transferAmountFromSigner = await fhevm.userDecryptEuint(
          FhevmType.euint64,
          transferAmountHandle,
          cErc20Address,
          signer,
        );
        expect(transferAmountFromSigner).to.equal(0);
      }

      const balanceHandleAlice = await cErc20.confidentialBalanceOf(signers.alice);
      const balanceAlice = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        balanceHandleAlice,
        cErc20Address,
        signers.alice,
      );
      expect(balanceAlice).to.equal(1000);

      // Reencrypt Bob's balance
      const balanceHandleBob = await cErc20.confidentialBalanceOf(signers.bob);
      const balanceBob = await fhevm.userDecryptEuint(FhevmType.euint64, balanceHandleBob, cErc20Address, signers.bob);
      expect(balanceBob).to.equal(0);
    });

    it("should prevent transfer from sanctioned address", async function () {
      const { sanctionsList } = await deploySanctionsListFixture();
      const { adminProvider } = await deployAdminProviderFixture(signers, sanctionsList);
      const { cErc20, cErc20Address } = await deployConfidentialErc20Fixture(signers, adminProvider);

      // Mint tokens to alice first
      await cErc20.mint(signers.alice.address, 10000);

      const txId = await cErc20.nextTxId();

      // Add alice to sanctions list
      await sanctionsList.addToSanctionsList([signers.alice.address]);

      const transferAmount = 100;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Try to transfer from sanctioned address
      await expect(
        cErc20
          .connect(signers.alice)
          ["confidentialTransfer(address,bytes32,bytes)"](
            signers.bob.address,
            encryptedTransferAmount.handles[0],
            encryptedTransferAmount.inputProof
          )
      ).to.be.revertedWithCustomError(cErc20, "SanctionedAddress")
        .withArgs(signers.alice.address);

      expect(await cErc20.nextTxId()).to.equal(txId);
    });

    it("should prevent transfer to sanctioned address", async function () {
      const { sanctionsList } = await deploySanctionsListFixture();
      const { adminProvider } = await deployAdminProviderFixture(signers, sanctionsList);
      const { cErc20, cErc20Address } = await deployConfidentialErc20Fixture(signers, adminProvider);

      // Mint tokens to alice first
      await cErc20.mint(signers.alice.address, 10000);

      // Add bob to sanctions list
      await sanctionsList.addToSanctionsList([signers.bob.address]);

      const txId = await cErc20.nextTxId();

      const transferAmount = 100;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Try to transfer to sanctioned address
      await expect(
        cErc20
          .connect(signers.alice)
          ["confidentialTransfer(address,bytes32,bytes)"](
            signers.bob.address,
            encryptedTransferAmount.handles[0],
            encryptedTransferAmount.inputProof
          )
      ).to.be.revertedWithCustomError(cErc20, "SanctionedAddress")
        .withArgs(signers.bob.address);

      expect(await cErc20.nextTxId()).to.equal(txId);
    });

    it("should allow transfer when no sanctions are in place", async function () {
      const { sanctionsList } = await deploySanctionsListFixture();
      const { adminProvider } = await deployAdminProviderFixture(signers, sanctionsList);
      const { cErc20, cErc20Address } = await deployConfidentialErc20Fixture(signers, adminProvider);

      // Mint tokens to alice first
      await cErc20.mint(signers.alice.address, 10000);

      const txId = await cErc20.nextTxId();

      // Sanction someone else, not alice or bob
      await sanctionsList.addToSanctionsList([signers.charlie.address]);

      const transferAmount = 100;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Should work fine
      await expect(
        cErc20
          .connect(signers.alice)
          ["confidentialTransfer(address,bytes32,bytes)"](
            signers.bob.address,
            encryptedTransferAmount.handles[0],
            encryptedTransferAmount.inputProof
          )
      ).to.not.be.reverted;

      expect(await cErc20.nextTxId()).to.equal(txId + BigInt(1));
    });

    it("should allow dynamic sanctions list updates", async function () {
      const { sanctionsList } = await deploySanctionsListFixture();
      const { adminProvider } = await deployAdminProviderFixture(signers, sanctionsList);
      const { cErc20, cErc20Address } = await deployConfidentialErc20Fixture(signers, adminProvider);

      // Mint tokens to alice first
      await cErc20.mint(signers.alice.address, 10000);

      const transferAmount = 100;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Initially should work
      await expect(
        cErc20
          .connect(signers.alice)
          ["confidentialTransfer(address,bytes32,bytes)"](
            signers.bob.address,
            encryptedTransferAmount.handles[0],
            encryptedTransferAmount.inputProof
          )
      ).to.not.be.reverted;

      // Add alice to sanctions list
      await sanctionsList.addToSanctionsList([signers.alice.address]);

      const encryptedTransferAmount2 = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      await expect(
        cErc20
          .connect(signers.alice)
          ["confidentialTransfer(address,bytes32,bytes)"](
            signers.bob.address,
            encryptedTransferAmount2.handles[0],
            encryptedTransferAmount2.inputProof
          )
      ).to.be.revertedWithCustomError(cErc20, "SanctionedAddress")
        .withArgs(signers.alice.address);

      // Remove alice from sanctions list
      await sanctionsList.removeFromSanctionsList([signers.alice.address]);

      const encryptedTransferAmount3 = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Should work again
      await expect(
        cErc20
          .connect(signers.alice)
          ["confidentialTransfer(address,bytes32,bytes)"](
            signers.bob.address,
            encryptedTransferAmount3.handles[0],
            encryptedTransferAmount3.inputProof
          )
      ).to.not.be.reverted;
    });

    it("should prevent transfer to zero address", async function () {
      const { sanctionsList } = await deploySanctionsListFixture();
      const { adminProvider } = await deployAdminProviderFixture(signers, sanctionsList);
      const { cErc20, cErc20Address } = await deployConfidentialErc20Fixture(signers, adminProvider);

      await cErc20.mint(signers.alice.address, 10000);

      const txId = await cErc20.nextTxId();

      const transferAmount = 100;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      await expect(
        cErc20
          .connect(signers.alice)
          ["confidentialTransfer(address,bytes32,bytes)"](
            ethers.ZeroAddress,
            encryptedTransferAmount.handles[0],
            encryptedTransferAmount.inputProof
          )
      ).to.be.revertedWithCustomError(cErc20, "ERC7984InvalidReceiver");

      expect(await cErc20.nextTxId()).to.equal(txId);
    });
  });

  describe("confidentialTransferFrom", function () {
    it("should prevent transferFrom when from address is sanctioned", async function () {
      const { sanctionsList } = await deploySanctionsListFixture();
      const { adminProvider } = await deployAdminProviderFixture(signers, sanctionsList);
      const { cErc20, cErc20Address } = await deployConfidentialErc20Fixture(signers, adminProvider);

      // Setup: mint tokens to alice and approve bob to spend them
      const aliceInitialBalance = 10_000;
      await cErc20.mint(signers.alice.address, aliceInitialBalance);

      const setOperator = await cErc20.connect(signers.alice).setOperator(signers.bob, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      // Sanction alice (the from address)
      await sanctionsList.addToSanctionsList([signers.alice.address]);

      const transferAmount = 500;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(cErc20Address, signers.bob.address)
        .add64(transferAmount)
        .encrypt();

      // Try transferFrom when from address is sanctioned
      await expect(
        cErc20
          .connect(signers.bob)
          ["confidentialTransferFrom(address,address,bytes32,bytes)"](
            signers.alice.address,
            signers.charlie.address,
            encryptedTransferAmount.handles[0],
            encryptedTransferAmount.inputProof
          )
      ).to.be.revertedWithCustomError(cErc20, "SanctionedAddress")
        .withArgs(signers.alice.address);
    });

    it("should prevent transferFrom when to address is sanctioned", async function () {
      const { sanctionsList } = await deploySanctionsListFixture();
      const { adminProvider } = await deployAdminProviderFixture(signers, sanctionsList);
      const { cErc20, cErc20Address } = await deployConfidentialErc20Fixture(signers, adminProvider);

      // Setup: mint tokens to alice and approve bob to spend them
      const aliceInitialBalance = 10_000;
      await cErc20.mint(signers.alice.address, aliceInitialBalance);

      const setOperator = await cErc20.connect(signers.alice).setOperator(signers.bob, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      // Sanction charlie (the to address)
      await sanctionsList.addToSanctionsList([signers.charlie.address]);

      const transferAmount = 500;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(cErc20Address, signers.bob.address)
        .add64(transferAmount)
        .encrypt();

      // Try transferFrom when to address is sanctioned
      await expect(
        cErc20
          .connect(signers.bob)
          ["confidentialTransferFrom(address,address,bytes32,bytes)"](
            signers.alice.address,
            signers.charlie.address,
            encryptedTransferAmount.handles[0],
            encryptedTransferAmount.inputProof
          )
      ).to.be.revertedWithCustomError(cErc20, "SanctionedAddress")
        .withArgs(signers.charlie.address);
    });

    it("should prevent transferFrom when operator (msg.sender) is sanctioned", async function () {
      const { sanctionsList } = await deploySanctionsListFixture();
      const { adminProvider } = await deployAdminProviderFixture(signers, sanctionsList);
      const { cErc20, cErc20Address } = await deployConfidentialErc20Fixture(signers, adminProvider);

      // Setup: mint tokens to alice and approve bob to spend them
      const aliceInitialBalance = 10_000;
      await cErc20.mint(signers.alice.address, aliceInitialBalance);

      const setOperator = await cErc20.connect(signers.alice).setOperator(signers.bob, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      // Sanction bob (the operator/msg.sender)
      await sanctionsList.addToSanctionsList([signers.bob.address]);

      const transferAmount = 500;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(cErc20Address, signers.bob.address)
        .add64(transferAmount)
        .encrypt();

      // Try transferFrom when operator is sanctioned - should revert
      await expect(
        cErc20
          .connect(signers.bob)
          ["confidentialTransferFrom(address,address,bytes32,bytes)"](
            signers.alice.address,
            signers.charlie.address,
            encryptedTransferAmount.handles[0],
            encryptedTransferAmount.inputProof
          )
      ).to.be.revertedWithCustomError(cErc20, "SanctionedAddress")
        .withArgs(signers.bob.address);
    });

    it("should revert if no operator", async function () {
      const aliceInitialBalance = 10_000;
      const transaction = await cErc20.mint(signers.alice, aliceInitialBalance);
      await transaction.wait();

      const encryptedTransferAmount1 = await fhevm
        .createEncryptedInput(cErc20Address, signers.bob.address)
        .add64(aliceInitialBalance)
        .encrypt();

      await expect(
        cErc20.connect(signers.bob)
        [
          "confidentialTransferFrom(address,address,bytes32,bytes)"
        ](signers.alice, signers.bob, encryptedTransferAmount1.handles[0], encryptedTransferAmount1.inputProof)
      ).to.be.revertedWithCustomError(cErc20, "ERC7984UnauthorizedSpender");

      const balanceHandleAlice = await cErc20.confidentialBalanceOf(signers.alice);
      const balanceAlice = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        balanceHandleAlice,
        cErc20Address,
        signers.alice,
      );
      expect(balanceAlice).to.equal(aliceInitialBalance);
    });
  });

  describe("tokenRegulator", function () {
    it("should initially be address(0)", async function () {
      expect(await cErc20.tokenRegulator()).to.equal(ethers.ZeroAddress);
    });

    it("should allow admin to set tokenRegulator", async function () {
      const newRegulator = signers.charlie.address;
      const tx = await cErc20.setTokenRegulator(newRegulator);
      const receipt = await tx.wait();

      expect(await cErc20.tokenRegulator()).to.equal(newRegulator);

      // Check event emission using helper
      const events = getTokenRegulatorUpdatedEvent(receipt);
      expect(events.length).to.equal(1);
      expect(events[0].args[0]).to.equal(ethers.ZeroAddress); // oldRegulator
      expect(events[0].args[1]).to.equal(newRegulator); // newRegulator
    });

    it("should emit TokenRegulatorUpdated event when updating tokenRegulator", async function () {
      const firstRegulator = signers.charlie.address;
      await cErc20.setTokenRegulator(firstRegulator);

      const secondRegulator = signers.bob.address;
      const tx = await cErc20.setTokenRegulator(secondRegulator);
      const receipt = await tx.wait();

      const events = getTokenRegulatorUpdatedEvent(receipt);
      expect(events.length).to.equal(1);
      expect(events[0].args[0]).to.equal(firstRegulator); // oldRegulator
      expect(events[0].args[1]).to.equal(secondRegulator); // newRegulator
    });

    it("should revert when non-admin tries to set tokenRegulator", async function () {
      await expect(
        cErc20.connect(signers.alice).setTokenRegulator(signers.charlie.address)
      ).to.be.revertedWithCustomError(cErc20, "AccessControlUnauthorizedAccount")
        .withArgs(signers.alice.address, await cErc20.DEFAULT_ADMIN_ROLE());
    });

    it("should allow tokenRegulator to decrypt balances after transfer when set", async function () {
      // Set charlie as the token regulator
      await cErc20.setTokenRegulator(signers.charlie.address);

      // Mint tokens to alice
      const aliceInitialAmount = 10_000;
      await cErc20.mint(signers.alice, aliceInitialAmount);

      // Transfer from alice to bob
      const transferAmount = 1337;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const tx = await cErc20
        .connect(signers.alice)
        ["confidentialTransfer(address,bytes32,bytes)"](
          signers.bob,
          encryptedTransferAmount.handles[0],
          encryptedTransferAmount.inputProof
        );
      await tx.wait();

      // Token regulator (charlie) should be able to decrypt alice's balance
      const balanceHandleAlice = await cErc20.confidentialBalanceOf(signers.alice);
      const balanceAliceFromTokenRegulator = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        balanceHandleAlice,
        cErc20Address,
        signers.charlie,
      );
      expect(balanceAliceFromTokenRegulator).to.equal(aliceInitialAmount - transferAmount);

      // Token regulator (charlie) should be able to decrypt bob's balance
      const balanceHandleBob = await cErc20.confidentialBalanceOf(signers.bob);
      const balanceBobFromTokenRegulator = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        balanceHandleBob,
        cErc20Address,
        signers.charlie,
      );
      expect(balanceBobFromTokenRegulator).to.equal(transferAmount);
    });

    it("should allow tokenRegulator to decrypt transfer amount", async function () {
      // Set charlie as the token regulator
      await cErc20.setTokenRegulator(signers.charlie.address);

      // Mint tokens to alice
      const aliceInitialAmount = 10_000;
      await cErc20.mint(signers.alice, aliceInitialAmount);

      // Transfer from alice to bob
      const transferAmount = 1337;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const tx = await cErc20
        .connect(signers.alice)
        ["confidentialTransfer(address,bytes32,bytes)"](
          signers.bob,
          encryptedTransferAmount.handles[0],
          encryptedTransferAmount.inputProof
        );
      const receipt = await tx.wait();

      // Get transfer event
      const events = getTransferInfoEvent(receipt);
      expect(events.length).to.be.equal(1);
      const transferAmountHandle = events[0].args[2];

      // Token regulator (charlie) should be able to decrypt transfer amount
      const transferAmountFromTokenRegulator = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        transferAmountHandle,
        cErc20Address,
        signers.charlie,
      );
      expect(transferAmountFromTokenRegulator).to.equal(transferAmount);
    });

    it("should allow both adminProvider regulator and tokenRegulator to decrypt simultaneously", async function () {
      // Set charlie as the token regulator
      await cErc20.setTokenRegulator(signers.charlie.address);

      // Mint tokens to alice
      const aliceInitialAmount = 10_000;
      await cErc20.mint(signers.alice, aliceInitialAmount);

      // Transfer from alice to bob
      const transferAmount = 1337;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const tx = await cErc20
        .connect(signers.alice)
        ["confidentialTransfer(address,bytes32,bytes)"](
          signers.bob,
          encryptedTransferAmount.handles[0],
          encryptedTransferAmount.inputProof
        );
      const receipt = await tx.wait();

      // Get transfer event
      const events = getTransferInfoEvent(receipt);
      const transferAmountHandle = events[0].args[2];

      // Both the adminProvider regulator and tokenRegulator should be able to decrypt
      const transferAmountFromAdminRegulator = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        transferAmountHandle,
        cErc20Address,
        signers.regulator,
      );
      expect(transferAmountFromAdminRegulator).to.equal(transferAmount);

      const transferAmountFromTokenRegulator = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        transferAmountHandle,
        cErc20Address,
        signers.charlie,
      );
      expect(transferAmountFromTokenRegulator).to.equal(transferAmount);

      // Both should be able to decrypt balances too
      const balanceHandleAlice = await cErc20.confidentialBalanceOf(signers.alice);

      const balanceAliceFromAdminRegulator = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        balanceHandleAlice,
        cErc20Address,
        signers.regulator,
      );
      expect(balanceAliceFromAdminRegulator).to.equal(aliceInitialAmount - transferAmount);

      const balanceAliceFromTokenRegulator = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        balanceHandleAlice,
        cErc20Address,
        signers.charlie,
      );
      expect(balanceAliceFromTokenRegulator).to.equal(aliceInitialAmount - transferAmount);
    });

    it("should not allow tokenRegulator to decrypt when set to address(0)", async function () {
      // Mint tokens to alice
      const aliceInitialAmount = 10_000;
      await cErc20.mint(signers.alice, aliceInitialAmount);

      // Transfer from alice to bob (with tokenRegulator still at address(0))
      const transferAmount = 1337;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const tx = await cErc20
        .connect(signers.alice)
        ["confidentialTransfer(address,bytes32,bytes)"](
          signers.bob,
          encryptedTransferAmount.handles[0],
          encryptedTransferAmount.inputProof
        );
      await tx.wait();

      // Charlie should NOT be able to decrypt alice's balance (not set as regulator)
      const balanceHandleAlice = await cErc20.confidentialBalanceOf(signers.alice);
      await expect(
        fhevm.userDecryptEuint(FhevmType.euint64, balanceHandleAlice, cErc20Address, signers.charlie),
      ).to.be.rejectedWith(
        `User ${signers.charlie.address} is not authorized to user decrypt handle ${balanceHandleAlice}!`,
      );
    });

    it("should revoke tokenRegulator access when set back to address(0)", async function () {
      // Set charlie as the token regulator
      await cErc20.setTokenRegulator(signers.charlie.address);

      // Mint tokens to alice
      const aliceInitialAmount = 10_000;
      await cErc20.mint(signers.alice, aliceInitialAmount);

      // First transfer - charlie should be able to decrypt
      const transferAmount1 = 1000;
      const encryptedTransferAmount1 = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount1)
        .encrypt();

      await cErc20
        .connect(signers.alice)
        ["confidentialTransfer(address,bytes32,bytes)"](
          signers.bob,
          encryptedTransferAmount1.handles[0],
          encryptedTransferAmount1.inputProof
        );

      // Verify charlie can decrypt
      const balanceHandleAlice1 = await cErc20.confidentialBalanceOf(signers.alice);
      const balanceAlice1 = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        balanceHandleAlice1,
        cErc20Address,
        signers.charlie,
      );
      expect(balanceAlice1).to.equal(aliceInitialAmount - transferAmount1);

      // Now revoke tokenRegulator by setting to address(0)
      await cErc20.setTokenRegulator(ethers.ZeroAddress);

      // Second transfer - charlie should NOT be able to decrypt new balances
      const transferAmount2 = 500;
      const encryptedTransferAmount2 = await fhevm
        .createEncryptedInput(cErc20Address, signers.alice.address)
        .add64(transferAmount2)
        .encrypt();

      await cErc20
        .connect(signers.alice)
        ["confidentialTransfer(address,bytes32,bytes)"](
          signers.bob,
          encryptedTransferAmount2.handles[0],
          encryptedTransferAmount2.inputProof
        );

      // Charlie should NOT be able to decrypt the new balance
      const balanceHandleAlice2 = await cErc20.confidentialBalanceOf(signers.alice);
      await expect(
        fhevm.userDecryptEuint(FhevmType.euint64, balanceHandleAlice2, cErc20Address, signers.charlie),
      ).to.be.rejectedWith(
        `User ${signers.charlie.address} is not authorized to user decrypt handle ${balanceHandleAlice2}!`,
      );
    });

    it("should allow tokenRegulator to decrypt mint event balances", async function () {
      // Set charlie as the token regulator
      await cErc20.setTokenRegulator(signers.charlie.address);

      // Mint tokens to alice
      const amount = 1000;
      await cErc20.mint(signers.alice, amount);

      // Token regulator (charlie) should be able to decrypt alice's balance
      const balanceHandleAlice = await cErc20.confidentialBalanceOf(signers.alice);
      const balanceAliceFromTokenRegulator = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        balanceHandleAlice,
        cErc20Address,
        signers.charlie,
      );
      expect(balanceAliceFromTokenRegulator).to.equal(amount);
    });
  });
});
