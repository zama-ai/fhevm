import { ethers } from "hardhat";
import { AdminProvider, FeeManager, SanctionsList } from "../types";
import { expect } from "chai";
import { getSigners, Signers } from "./signers";
import { deployAdminProviderFixture, deployFeeManagerFixture, deploySanctionsListFixture } from "./fixtures";


describe("AdminProvider", function () {
  let signers: Signers;
  let adminProvider: AdminProvider;
  let adminProviderAddress: string;
  let feeManager: FeeManager;
  let feeManagerAddress: string;
  let sanctionsList: SanctionsList;
  let sanctionsListAddress: string;

  before(async function () {
    signers = await getSigners();
  });

  beforeEach(async () => {
    ({ adminProvider, adminProviderAddress, feeManager, feeManagerAddress, sanctionsList, sanctionsListAddress } = await deployAdminProviderFixture(signers));
  });

  describe("Deployment", function () {
    it("should deploy with correct owner", async function () {
      expect(await adminProvider.owner()).to.equal(signers.deployer.address);
    });

    it("should have sanctions list from constructor", async function () {
      expect(await adminProvider.sanctionsList()).to.equal(sanctionsListAddress);
    });

    it("should have fee manager from constructor", async function () {
      expect(await adminProvider.feeManager()).to.equal(feeManagerAddress);
    });

    it("should revert if fee manager is zero address", async function () {
      const AdminProviderFactory = await ethers.getContractFactory("AdminProvider");

      await expect(
        AdminProviderFactory.deploy(ethers.ZeroAddress, sanctionsList, signers.regulator.address)
      ).to.be.revertedWithCustomError(AdminProviderFactory, "ZeroAddressFeeManager");
    });

    it("should revert if sanctions list is zero address", async function () {
      const AdminProviderFactory = await ethers.getContractFactory("AdminProvider");

      await expect(
        AdminProviderFactory.deploy(feeManager, ethers.ZeroAddress, signers.regulator.address)
      ).to.be.revertedWithCustomError(AdminProviderFactory, "ZeroAddressSanctionsList");
    });

    it("should revert if regulator is zero address", async function () {
      const AdminProviderFactory = await ethers.getContractFactory("AdminProvider");

      await expect(
        AdminProviderFactory.deploy(feeManager, sanctionsList, ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(AdminProviderFactory, "ZeroAddressRegulator");
    });
  });

  describe("Sanctions List Management", function () {
    it("should allow owner to set sanctions list", async function () {
      const { sanctionsList: newSanctionsList, sanctionsListAddress: newSanctionsListAddress } = await deploySanctionsListFixture();

      await expect(adminProvider.setSanctionsList(newSanctionsList))
        .to.emit(adminProvider, "SanctionsListUpdated")
        .withArgs(sanctionsListAddress, newSanctionsListAddress);

      expect(await adminProvider.sanctionsList()).to.equal(newSanctionsListAddress);
    });

    it("should prevent non-owner from setting sanctions list", async function () {
      const { sanctionsList } = await deploySanctionsListFixture();

      await expect(
        adminProvider.connect(signers.alice).setSanctionsList(sanctionsList)
      ).to.be.revertedWithCustomError(adminProvider, "OwnableUnauthorizedAccount");
    });

    it("should emit event when updating existing sanctions list", async function () {
      const { sanctionsList: sanctionsList1, sanctionsListAddress: address1 } = await deploySanctionsListFixture();
      const { sanctionsList: sanctionsList2, sanctionsListAddress: address2 } = await deploySanctionsListFixture();

      await adminProvider.setSanctionsList(sanctionsList1);

      await expect(adminProvider.setSanctionsList(sanctionsList2))
        .to.emit(adminProvider, "SanctionsListUpdated")
        .withArgs(address1, address2);
    });

    it("should revert if sanctions list is zero address", async function () {
      await expect(
        adminProvider.setSanctionsList(ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(adminProvider, "ZeroAddressSanctionsList");
    });
  });

  describe("Fee Manager Management", function () {
    it("should allow owner to set fee manager", async function () {
      const oldFeeManager = await adminProvider.feeManager();
      const { feeManager, feeManagerAddress } = await deployFeeManagerFixture(signers.royalties);

      await expect(adminProvider.setFeeManager(feeManager))
        .to.emit(adminProvider, "FeeManagerUpdated")
        .withArgs(oldFeeManager, feeManagerAddress);

      expect(await adminProvider.feeManager()).to.equal(feeManagerAddress);
    });

    it("should prevent non-owner from setting fee manager", async function () {
      const { feeManagerAddress } = await deployFeeManagerFixture(signers.royalties);

      await expect(
        adminProvider.connect(signers.alice).setFeeManager(feeManagerAddress)
      ).to.be.revertedWithCustomError(adminProvider, "OwnableUnauthorizedAccount");
    });

    it("should revert if fee manager is zero address", async function () {
      await expect(
        adminProvider.setFeeManager(ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(adminProvider, "ZeroAddressFeeManager");
    });
  });

  describe("Regulator Management", function () {
    it("should allow owner to set regulator", async function () {
      const oldRegulator = await adminProvider.regulator();
      const newRegulator = signers.alice.address;

      await expect(adminProvider.setRegulator(newRegulator))
        .to.emit(adminProvider, "RegulatorUpdated")
        .withArgs(oldRegulator, newRegulator);

      expect(await adminProvider.regulator()).to.equal(newRegulator);
    });

    it("should prevent non-owner from setting regulator", async function () {
      await expect(
        adminProvider.connect(signers.alice).setRegulator(signers.bob.address)
      ).to.be.revertedWithCustomError(adminProvider, "OwnableUnauthorizedAccount");
    });

    it("should revert if regulator is zero address", async function () {
      await expect(
        adminProvider.setRegulator(ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(adminProvider, "ZeroAddressRegulator");
    });
  });

  describe("Ownership", function () {
    it("should support two-step ownership transfer", async function () {
      await adminProvider.transferOwnership(signers.alice.address);
      expect(await adminProvider.pendingOwner()).to.equal(signers.alice.address);
      expect(await adminProvider.owner()).to.equal(signers.deployer.address);

      await adminProvider.connect(signers.alice).acceptOwnership();
      expect(await adminProvider.owner()).to.equal(signers.alice.address);
      expect(await adminProvider.pendingOwner()).to.equal(ethers.ZeroAddress);
    });
  });
});
