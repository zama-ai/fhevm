import { ethers } from "hardhat";
import { FeeManager } from "../types";
import { expect } from "chai";
import { getSigners, Signers } from "./signers";
import { deployFeeManagerFixture } from "./fixtures";

async function deployFixture(signers: Signers) {
  const { feeManager, feeManagerAddress } = await deployFeeManagerFixture(signers.royalties);

  return { feeManager, feeManagerAddress };
}

describe("FeeManager", function () {
  let signers: Signers;
  let feeManager: FeeManager;

  before(async function () {
    signers = await getSigners();
  });

  beforeEach(async () => {
    ({ feeManager } = await deployFixture(signers));
  });

  describe("Deployment", function () {
    it("should deploy with correct admin role", async function () {
      expect(await feeManager.hasRole(await feeManager.DEFAULT_ADMIN_ROLE(), signers.deployer.address)).to.be.true;
    });

    it("should set fees from constructor", async function () {
      expect(await feeManager.wrapFeeBasisPoints()).to.equal(100);
      expect(await feeManager.unwrapFeeBasisPoints()).to.equal(100);
    });

    it("should have set fee recipient from constructor", async function () {
      expect(await feeManager.getFeeRecipient()).to.equal(signers.royalties.address);
    });

    it("should revert deployment with zero address as fee recipient", async function () {
      const FeeManagerContract = await ethers.getContractFactory("FeeManager");

      await expect(
        FeeManagerContract.deploy(
          100,
          100,
          ethers.parseEther("0.01"),
          ethers.parseEther("0.001"),
          ethers.ZeroAddress,
        )
      ).to.be.revertedWithCustomError(FeeManagerContract, "ZeroAddressFeeRecipient");
    });

    it("should revert deployment when wrapFeeBasisPoints exceeds 100%", async function () {
      const FeeManagerContract = await ethers.getContractFactory("FeeManager");

      await expect(
        FeeManagerContract.deploy(
          10_001, // wrapFeeBasisPoints > 10_000
          100,
          ethers.parseEther("0.01"),
          ethers.parseEther("0.001"),
          signers.royalties.address,
        )
      ).to.be.revertedWithCustomError(FeeManagerContract, "FeeExceedsMaximum");
    });

    it("should revert deployment when unwrapFeeBasisPoints exceeds 100%", async function () {
      const FeeManagerContract = await ethers.getContractFactory("FeeManager");

      await expect(
        FeeManagerContract.deploy(
          100,
          10_001, // unwrapFeeBasisPoints > 10_000
          ethers.parseEther("0.01"),
          ethers.parseEther("0.001"),
          signers.royalties.address,
        )
      ).to.be.revertedWithCustomError(FeeManagerContract, "FeeExceedsMaximum");
    });

    it("should allow deployment when both feeBasisPoints are exactly 100%", async function () {
      const FeeManagerContract = await ethers.getContractFactory("FeeManager");

      const feeManager = await FeeManagerContract.deploy(
        10_000, // wrapFeeBasisPoints = 100%
        10_000, // unwrapFeeBasisPoints = 100%
        ethers.parseEther("0.01"),
        ethers.parseEther("0.001"),
        signers.royalties.address,
      );

      expect(await feeManager.wrapFeeBasisPoints()).to.equal(10_000);
      expect(await feeManager.unwrapFeeBasisPoints()).to.equal(10_000);
    });
  });

  describe("Wrap/Unwrap Fee Configuration", function () {
    it("should allow admin to set wrap fee basis points", async function () {
      const feeBasisPoints = 50; // 0.5%

      await expect(feeManager.setWrapFeeBasisPoints(feeBasisPoints))
        .to.emit(feeManager, "WrapFeeBasisPointsUpdated")
        .withArgs(100, feeBasisPoints);

      expect(await feeManager.wrapFeeBasisPoints()).to.equal(feeBasisPoints);
    });

    it("should allow admin to set unwrap fee basis points", async function () {
      const feeBasisPoints = 75; // 0.75%

      await expect(feeManager.setUnwrapFeeBasisPoints(feeBasisPoints))
        .to.emit(feeManager, "UnwrapFeeBasisPointsUpdated")
        .withArgs(100, feeBasisPoints);

      expect(await feeManager.unwrapFeeBasisPoints()).to.equal(feeBasisPoints);
    });

    it("should prevent setting wrap fee basis points above 100%", async function () {
      await expect(
        feeManager.setWrapFeeBasisPoints(10_001)
      ).to.be.revertedWithCustomError(feeManager, "FeeExceedsMaximum");
    });

    it("should prevent setting unwrap fee basis points above 100%", async function () {
      await expect(
        feeManager.setUnwrapFeeBasisPoints(10_001)
      ).to.be.revertedWithCustomError(feeManager, "FeeExceedsMaximum");
    });

    it("should prevent non-admin from setting wrap fee basis points", async function () {
      await expect(
        feeManager.connect(signers.alice).setWrapFeeBasisPoints(50)
      ).to.be.revertedWithCustomError(feeManager, "AccessControlUnauthorizedAccount");
    });

    it("should prevent non-admin from setting unwrap fee basis points", async function () {
      await expect(
        feeManager.connect(signers.alice).setUnwrapFeeBasisPoints(75)
      ).to.be.revertedWithCustomError(feeManager, "AccessControlUnauthorizedAccount");
    });
  });

  describe("Deploy Fee Configuration", function () {
    it("should allow admin to set deploy fee", async function () {
      const oldDeployFee = await feeManager.deployFee();
      const newDeployFee = oldDeployFee * BigInt(2);

      await expect(feeManager.setDeployFee(newDeployFee))
        .to.emit(feeManager, "DeployFeeUpdated")
        .withArgs(oldDeployFee, newDeployFee);

      expect(await feeManager.deployFee()).to.equal(newDeployFee);
    });

    it("should allow setting deploy fee to zero", async function () {
      // First set a non-zero fee
      const initialFee = ethers.parseEther("0.05");
      await feeManager.setDeployFee(initialFee);

      // Then set it to zero
      await expect(feeManager.setDeployFee(0))
        .to.emit(feeManager, "DeployFeeUpdated")
        .withArgs(initialFee, 0);

      expect(await feeManager.deployFee()).to.equal(0);
    });

    it("should prevent non-admin from setting deploy fee", async function () {
      await expect(
        feeManager.connect(signers.alice).setDeployFee(ethers.parseEther("0.1"))
      ).to.be.revertedWithCustomError(feeManager, "AccessControlUnauthorizedAccount");
    });
  });

  describe("Fee Recipient Management", function () {
    it("should allow admin to set fee recipient", async function () {
      await expect(feeManager.setFeeRecipient(signers.alice.address))
        .to.emit(feeManager, "FeeRecipientUpdated")
        .withArgs(signers.royalties.address, signers.alice.address);

      expect(await feeManager.getFeeRecipient()).to.equal(signers.alice.address);
    });

    it("should prevent setting zero address as fee recipient", async function () {
      await expect(
        feeManager.setFeeRecipient(ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(feeManager, "ZeroAddressFeeRecipient");
    });

    it("should prevent non-admin from setting fee recipient", async function () {
      await expect(
        feeManager.connect(signers.alice).setFeeRecipient(signers.royalties.address)
      ).to.be.revertedWithCustomError(feeManager, "AccessControlUnauthorizedAccount");
    });
  });

  describe("Wrap/Unwrap Fee Calculation", function () {
    beforeEach(async () => {
      await feeManager.setWrapFeeBasisPoints(100); // 1%
      await feeManager.setUnwrapFeeBasisPoints(50); // 0.5%
    });

    for (const [amount, expectedWrapFee, expectedUnwrapFee] of [[0, 0, 0], [10_000, 100, 50]]) {
      it(`should calculate wrap fee correctly (amount: ${amount})`, async function () {
        expect(await feeManager.getWrapFee(amount, ethers.ZeroAddress, ethers.ZeroAddress)).to.equal(expectedWrapFee);
      });

      it(`should calculate unwrap fee correctly (amount: ${amount})`, async function () {
        const fee = await feeManager.getUnwrapFee(amount, ethers.ZeroAddress, ethers.ZeroAddress);
        expect(fee).to.equal(expectedUnwrapFee);
      });
    }

    it("should handle maximum values", async function () {
      await feeManager.setWrapFeeBasisPoints(10_000); // 100%
      const amount = 1000;
      expect(await feeManager.getWrapFee(amount, ethers.ZeroAddress, ethers.ZeroAddress)).to.equal(amount);
    });
  });

  describe("Fee Rounding (Ceiling Division)", function () {
    it("should round up wrap fees to prevent fee leakage", async function () {
      await feeManager.setWrapFeeBasisPoints(10); // 0.1%

      // Amount that would round down to 0 with floor division
      // 99 * 10 / 10_000 = 0.099 -> floor = 0, ceil = 1
      expect(await feeManager.getWrapFee(99, ethers.ZeroAddress, ethers.ZeroAddress)).to.equal(1);

      // Amount that divides evenly should stay the same
      // 10_000 * 10 / 10_000 = 10
      expect(await feeManager.getWrapFee(10_000, ethers.ZeroAddress, ethers.ZeroAddress)).to.equal(10);

      // Amount with small remainder should round up
      // 10_001 * 10 / 10_000 = 10.001 -> floor = 10, ceil = 11
      expect(await feeManager.getWrapFee(10_001, ethers.ZeroAddress, ethers.ZeroAddress)).to.equal(11);
    });

    it("should round up unwrap fees to prevent fee leakage", async function () {
      await feeManager.setUnwrapFeeBasisPoints(10); // 0.1%

      // Amount that would round down to 0 with floor division
      // 99 * 10 / 10_000 = 0.099 -> floor = 0, ceil = 1
      expect(await feeManager.getUnwrapFee(99, ethers.ZeroAddress, ethers.ZeroAddress)).to.equal(1);

      // Amount that divides evenly should stay the same
      // 10_000 * 10 / 10_000 = 10
      expect(await feeManager.getUnwrapFee(10_000, ethers.ZeroAddress, ethers.ZeroAddress)).to.equal(10);

      // Amount with small remainder should round up
      // 10_001 * 10 / 10_000 = 10.001 -> floor = 10, ceil = 11
      expect(await feeManager.getUnwrapFee(10_001, ethers.ZeroAddress, ethers.ZeroAddress)).to.equal(11);
    });

    it("should prevent WBTC-like fee bypass attack through transaction splitting", async function () {
      // Simulate cWBTC scenario from the vulnerability description:
      // - WBTC has 8 decimals, cWBTC has 6 decimals (euint64 limitation)
      // - 0.1 cWBTC = 100,000 units (in 6 decimals)
      // - At 100k USD per BTC, this is 10k USD
      // - With 0.1% fee (10 basis points), expected fee is ~10 USD or ~100 units

      await feeManager.setUnwrapFeeBasisPoints(10); // 0.1%

      const totalAmount = 100_000; // 0.1 cWBTC
      const expectedTotalFee = 100; // 100,000 * 10 / 10_000 = 100 units

      // Calculate fee for the full amount
      const feeFullAmount = await feeManager.getUnwrapFee(totalAmount, ethers.ZeroAddress, ethers.ZeroAddress);
      expect(feeFullAmount).to.equal(expectedTotalFee);

      // Try to bypass fees by splitting into 101 chunks
      // With floor division: 990 * 10 / 10_000 = 0.99 -> 0 fee per chunk
      // With ceil division: 990 * 10 / 10_000 = 0.99 -> 1 fee per chunk
      const chunkSize = 990;
      const numChunks = 101;
      const feePerChunk = await feeManager.getUnwrapFee(chunkSize, ethers.ZeroAddress, ethers.ZeroAddress);

      // With ceiling division, each chunk should pay at least 1 unit fee
      expect(feePerChunk).to.equal(1);

      // Total fees from splitting should be >= original fee
      const totalFeesFromSplitting = feePerChunk * BigInt(numChunks);
      expect(totalFeesFromSplitting).to.be.gte(expectedTotalFee);
    });

    it("should round up swapper wrap fees when waiver is active", async function () {
      const swapperRole = await feeManager.SWAPPER_ROLE();
      await feeManager.grantRole(swapperRole, signers.alice.address);
      await feeManager.setSwapperWrapFeeBasisPoints(10); // 0.1%
      await feeManager.setSwapperFeeWaiverActive(true);

      // Amount that would round down to 0 with floor division
      expect(await feeManager.getWrapFee(99, signers.alice.address, ethers.ZeroAddress)).to.equal(1);
    });

    it("should round up swapper unwrap fees when waiver is active", async function () {
      const swapperRole = await feeManager.SWAPPER_ROLE();
      await feeManager.grantRole(swapperRole, signers.alice.address);
      await feeManager.setSwapperUnwrapFeeBasisPoints(10); // 0.1%
      await feeManager.setSwapperFeeWaiverActive(true);

      // Amount that would round down to 0 with floor division
      expect(await feeManager.getUnwrapFee(99, ethers.ZeroAddress, signers.alice.address)).to.equal(1);
    });

    it("should return 0 fee when amount is 0", async function () {
      await feeManager.setWrapFeeBasisPoints(100); // 1%
      await feeManager.setUnwrapFeeBasisPoints(100); // 1%

      expect(await feeManager.getWrapFee(0, ethers.ZeroAddress, ethers.ZeroAddress)).to.equal(0);
      expect(await feeManager.getUnwrapFee(0, ethers.ZeroAddress, ethers.ZeroAddress)).to.equal(0);
    });

    it("should return 0 fee when fee basis points is 0", async function () {
      await feeManager.setWrapFeeBasisPoints(0);
      await feeManager.setUnwrapFeeBasisPoints(0);

      expect(await feeManager.getWrapFee(10_000, ethers.ZeroAddress, ethers.ZeroAddress)).to.equal(0);
      expect(await feeManager.getUnwrapFee(10_000, ethers.ZeroAddress, ethers.ZeroAddress)).to.equal(0);
    });
  });

  describe("Access Control", function () {
    it("should allow admin to grant admin role to another account", async function () {
      const defaultAdminRole = await feeManager.DEFAULT_ADMIN_ROLE();

      await feeManager.grantRole(defaultAdminRole, signers.alice.address);
      expect(await feeManager.hasRole(defaultAdminRole, signers.alice.address)).to.be.true;
    });

    it("should allow admin to revoke admin role from another account", async function () {
      const defaultAdminRole = await feeManager.DEFAULT_ADMIN_ROLE();

      // Grant role first
      await feeManager.grantRole(defaultAdminRole, signers.alice.address);
      expect(await feeManager.hasRole(defaultAdminRole, signers.alice.address)).to.be.true;

      // Revoke role
      await feeManager.revokeRole(defaultAdminRole, signers.alice.address);
      expect(await feeManager.hasRole(defaultAdminRole, signers.alice.address)).to.be.false;
    });

    it("should prevent non-admin from granting roles", async function () {
      const defaultAdminRole = await feeManager.DEFAULT_ADMIN_ROLE();

      await expect(
        feeManager.connect(signers.alice).grantRole(defaultAdminRole, signers.bob.address)
      ).to.be.revertedWithCustomError(feeManager, "AccessControlUnauthorizedAccount");
    });
  });

  describe("Role Separation - FEE_MANAGER_ROLE", function () {
    it("should have FEE_MANAGER_ROLE constant defined", async function () {
      const feeManagerRole = await feeManager.FEE_MANAGER_ROLE();
      expect(feeManagerRole).to.equal(ethers.keccak256(ethers.toUtf8Bytes("FEE_MANAGER_ROLE")));
    });

    it("should grant FEE_MANAGER_ROLE to deployer in fixture", async function () {
      const feeManagerRole = await feeManager.FEE_MANAGER_ROLE();
      expect(await feeManager.hasRole(feeManagerRole, signers.deployer.address)).to.be.true;
    });

    it("should allow DEFAULT_ADMIN to grant FEE_MANAGER_ROLE to another account", async function () {
      const feeManagerRole = await feeManager.FEE_MANAGER_ROLE();

      await feeManager.grantRole(feeManagerRole, signers.bob.address);
      expect(await feeManager.hasRole(feeManagerRole, signers.bob.address)).to.be.true;
    });

    it("should allow FEE_MANAGER_ROLE to set wrap fee basis points", async function () {
      const feeManagerRole = await feeManager.FEE_MANAGER_ROLE();

      // Grant FEE_MANAGER_ROLE to alice
      await feeManager.grantRole(feeManagerRole, signers.alice.address);

      // Alice should be able to set wrap fee
      await expect(feeManager.connect(signers.alice).setWrapFeeBasisPoints(200))
        .to.emit(feeManager, "WrapFeeBasisPointsUpdated")
        .withArgs(100, 200);

      expect(await feeManager.wrapFeeBasisPoints()).to.equal(200);
    });

    it("should allow FEE_MANAGER_ROLE to set all fee parameters", async function () {
      const feeManagerRole = await feeManager.FEE_MANAGER_ROLE();

      // Grant FEE_MANAGER_ROLE to alice
      await feeManager.grantRole(feeManagerRole, signers.alice.address);

      // Test all fee setter functions
      await expect(feeManager.connect(signers.alice).setWrapFeeBasisPoints(200))
        .to.not.be.reverted;
      await expect(feeManager.connect(signers.alice).setUnwrapFeeBasisPoints(150))
        .to.not.be.reverted;
      await expect(feeManager.connect(signers.alice).setDeployFee(ethers.parseEther("0.02")))
        .to.not.be.reverted;
      await expect(feeManager.connect(signers.alice).setBatchTransferFee(ethers.parseEther("0.002")))
        .to.not.be.reverted;
      await expect(feeManager.connect(signers.alice).setFeeRecipient(signers.bob.address))
        .to.not.be.reverted;
      await expect(feeManager.connect(signers.alice).setSwapperFeeWaiverActive(true))
        .to.not.be.reverted;
    });

    it("should prevent accounts without FEE_MANAGER_ROLE from setting fees", async function () {
      const feeManagerRole = await feeManager.FEE_MANAGER_ROLE();

      // alice does not have FEE_MANAGER_ROLE
      await expect(
        feeManager.connect(signers.alice).setWrapFeeBasisPoints(200)
      ).to.be.revertedWithCustomError(feeManager, "AccessControlUnauthorizedAccount")
        .withArgs(signers.alice.address, feeManagerRole);
    });

    it("should prevent DEFAULT_ADMIN without FEE_MANAGER_ROLE from setting fees", async function () {
      const defaultAdminRole = await feeManager.DEFAULT_ADMIN_ROLE();
      const feeManagerRole = await feeManager.FEE_MANAGER_ROLE();

      // Grant DEFAULT_ADMIN to bob but NOT FEE_MANAGER_ROLE
      await feeManager.grantRole(defaultAdminRole, signers.bob.address);

      // Bob has DEFAULT_ADMIN but should NOT be able to set fees without FEE_MANAGER_ROLE
      await expect(
        feeManager.connect(signers.bob).setWrapFeeBasisPoints(200)
      ).to.be.revertedWithCustomError(feeManager, "AccessControlUnauthorizedAccount")
        .withArgs(signers.bob.address, feeManagerRole);
    });

    it("should allow DEFAULT_ADMIN to revoke FEE_MANAGER_ROLE", async function () {
      const feeManagerRole = await feeManager.FEE_MANAGER_ROLE();

      // Grant FEE_MANAGER_ROLE to alice
      await feeManager.grantRole(feeManagerRole, signers.alice.address);
      expect(await feeManager.hasRole(feeManagerRole, signers.alice.address)).to.be.true;

      // Alice can set fees
      await expect(feeManager.connect(signers.alice).setWrapFeeBasisPoints(200))
        .to.not.be.reverted;

      // Revoke the role
      await feeManager.revokeRole(feeManagerRole, signers.alice.address);
      expect(await feeManager.hasRole(feeManagerRole, signers.alice.address)).to.be.false;

      // Alice can no longer set fees
      await expect(
        feeManager.connect(signers.alice).setWrapFeeBasisPoints(300)
      ).to.be.revertedWithCustomError(feeManager, "AccessControlUnauthorizedAccount");
    });

    it("should isolate role management from fee operations", async function () {
      const defaultAdminRole = await feeManager.DEFAULT_ADMIN_ROLE();
      const feeManagerRole = await feeManager.FEE_MANAGER_ROLE();

      // Alice gets FEE_MANAGER_ROLE (can set fees)
      await feeManager.grantRole(feeManagerRole, signers.alice.address);

      // Bob gets DEFAULT_ADMIN (can manage roles)
      await feeManager.grantRole(defaultAdminRole, signers.bob.address);

      // Alice can set fees but NOT grant roles
      await expect(feeManager.connect(signers.alice).setWrapFeeBasisPoints(200))
        .to.not.be.reverted;
      await expect(
        feeManager.connect(signers.alice).grantRole(feeManagerRole, signers.charlie.address)
      ).to.be.revertedWithCustomError(feeManager, "AccessControlUnauthorizedAccount");

      // Bob can grant roles but NOT set fees
      await expect(feeManager.connect(signers.bob).grantRole(feeManagerRole, signers.charlie.address))
        .to.not.be.reverted;
      await expect(
        feeManager.connect(signers.bob).setWrapFeeBasisPoints(300)
      ).to.be.revertedWithCustomError(feeManager, "AccessControlUnauthorizedAccount");
    });
  });

  describe("Swapper Fee Configuration", function () {
    it("should initialize swapper fees to 0", async function () {
      expect(await feeManager.swapperWrapFeeBasisPoints()).to.equal(0);
      expect(await feeManager.swapperUnwrapFeeBasisPoints()).to.equal(0);
    });

    it("should allow admin to set swapper wrap fee basis points", async function () {
      const feeBasisPoints = 25; // 0.25%

      await expect(feeManager.setSwapperWrapFeeBasisPoints(feeBasisPoints))
        .to.emit(feeManager, "SwapperWrapFeeBasisPointsUpdated")
        .withArgs(0, feeBasisPoints);

      expect(await feeManager.swapperWrapFeeBasisPoints()).to.equal(feeBasisPoints);
    });

    it("should allow admin to set swapper unwrap fee basis points", async function () {
      const feeBasisPoints = 30; // 0.30%

      await expect(feeManager.setSwapperUnwrapFeeBasisPoints(feeBasisPoints))
        .to.emit(feeManager, "SwapperUnwrapFeeBasisPointsUpdated")
        .withArgs(0, feeBasisPoints);

      expect(await feeManager.swapperUnwrapFeeBasisPoints()).to.equal(feeBasisPoints);
    });

    it("should prevent setting swapper wrap fee basis points above 100%", async function () {
      await expect(
        feeManager.setSwapperWrapFeeBasisPoints(10_001)
      ).to.be.revertedWithCustomError(feeManager, "FeeExceedsMaximum");
    });

    it("should prevent setting swapper unwrap fee basis points above 100%", async function () {
      await expect(
        feeManager.setSwapperUnwrapFeeBasisPoints(10_001)
      ).to.be.revertedWithCustomError(feeManager, "FeeExceedsMaximum");
    });

    it("should prevent non-admin from setting swapper wrap fee basis points", async function () {
      await expect(
        feeManager.connect(signers.alice).setSwapperWrapFeeBasisPoints(25)
      ).to.be.revertedWithCustomError(feeManager, "AccessControlUnauthorizedAccount");
    });

    it("should prevent non-admin from setting swapper unwrap fee basis points", async function () {
      await expect(
        feeManager.connect(signers.alice).setSwapperUnwrapFeeBasisPoints(30)
      ).to.be.revertedWithCustomError(feeManager, "AccessControlUnauthorizedAccount");
    });

    it("should allow FEE_MANAGER_ROLE to set swapper fee basis points", async function () {
      const feeManagerRole = await feeManager.FEE_MANAGER_ROLE();
      await feeManager.grantRole(feeManagerRole, signers.alice.address);

      await expect(feeManager.connect(signers.alice).setSwapperWrapFeeBasisPoints(50))
        .to.emit(feeManager, "SwapperWrapFeeBasisPointsUpdated")
        .withArgs(0, 50);

      await expect(feeManager.connect(signers.alice).setSwapperUnwrapFeeBasisPoints(40))
        .to.emit(feeManager, "SwapperUnwrapFeeBasisPointsUpdated")
        .withArgs(0, 40);

      expect(await feeManager.swapperWrapFeeBasisPoints()).to.equal(50);
      expect(await feeManager.swapperUnwrapFeeBasisPoints()).to.equal(40);
    });
  });

  describe("Swapper Fee Waiver", function () {
    it("should allow admin to activate swapper fee waiver", async function () {
      await expect(feeManager.setSwapperFeeWaiverActive(true))
        .to.emit(feeManager, "SwapperFeeWaiverUpdated")
        .withArgs(true);

      expect(await feeManager.swapperFeeWaiverActive()).to.be.true;
    });

    it("should allow admin to deactivate swapper fee waiver", async function () {
      await feeManager.setSwapperFeeWaiverActive(true);

      await expect(feeManager.setSwapperFeeWaiverActive(false))
        .to.emit(feeManager, "SwapperFeeWaiverUpdated")
        .withArgs(false);

      expect(await feeManager.swapperFeeWaiverActive()).to.be.false;
    });

    it("should prevent non-admin from changing swapper fee waiver", async function () {
      await expect(
        feeManager.connect(signers.alice).setSwapperFeeWaiverActive(true)
      ).to.be.revertedWithCustomError(feeManager, "AccessControlUnauthorizedAccount");
    });

    it("should charge swapper-specific wrap fees for SWAPPER role when waiver is active", async function () {
      const swapperRole = await feeManager.SWAPPER_ROLE();

      // Grant SWAPPER role to alice
      await feeManager.grantRole(swapperRole, signers.alice.address);

      // Set fee basis points
      await feeManager.setWrapFeeBasisPoints(100); // 1% for regular users
      await feeManager.setSwapperWrapFeeBasisPoints(25); // 0.25% for swappers

      // Activate swapper fee waiver
      await feeManager.setSwapperFeeWaiverActive(true);

      // Test wrap fee for swapper should be swapperWrapFeeBasisPoints (25 basis points = 0.25%)
      expect(await feeManager.getWrapFee(10_000, signers.alice.address, ethers.ZeroAddress)).to.equal(25);

      // Test wrap fee for non-swapper should still use standard fee
      expect(await feeManager.getWrapFee(10_000, signers.bob.address, ethers.ZeroAddress)).to.equal(100);
    });

    it("should charge zero wrap fees for SWAPPER role when waiver is active and swapper fee is 0", async function () {
      const swapperRole = await feeManager.SWAPPER_ROLE();

      // Grant SWAPPER role to alice
      await feeManager.grantRole(swapperRole, signers.alice.address);

      // Set fee basis points
      await feeManager.setWrapFeeBasisPoints(100); // 1% for regular users
      // swapperWrapFeeBasisPoints is 0 by default

      // Activate swapper fee waiver
      await feeManager.setSwapperFeeWaiverActive(true);

      // Test wrap fee for swapper should be 0 (since swapperWrapFeeBasisPoints = 0)
      expect(await feeManager.getWrapFee(10_000, signers.alice.address, ethers.ZeroAddress)).to.equal(0);

      // Test wrap fee for non-swapper should still apply
      expect(await feeManager.getWrapFee(10_000, signers.bob.address, ethers.ZeroAddress)).to.equal(100);
    });

    it("should charge wrap fees for SWAPPER role when waiver is inactive", async function () {
      const swapperRole = await feeManager.SWAPPER_ROLE();

      // Grant SWAPPER role to alice
      await feeManager.grantRole(swapperRole, signers.alice.address);

      // Set fee basis points to non-zero
      await feeManager.setWrapFeeBasisPoints(100); // 1%
      await feeManager.setSwapperWrapFeeBasisPoints(25); // 0.25% for swappers

      // Keep swapper fee waiver inactive (default)
      expect(await feeManager.swapperFeeWaiverActive()).to.be.false;

      // Test wrap fee for swapper should use standard fee when waiver is inactive
      expect(await feeManager.getWrapFee(10_000, signers.alice.address, ethers.ZeroAddress)).to.equal(100);
    });

    it("should charge swapper-specific unwrap fees for SWAPPER role when waiver is active", async function () {
      const swapperRole = await feeManager.SWAPPER_ROLE();

      // Grant SWAPPER role to alice
      await feeManager.grantRole(swapperRole, signers.alice.address);

      // Set fee basis points
      await feeManager.setUnwrapFeeBasisPoints(50); // 0.5% for regular users
      await feeManager.setSwapperUnwrapFeeBasisPoints(10); // 0.1% for swappers

      // Activate swapper fee waiver
      await feeManager.setSwapperFeeWaiverActive(true);

      const amount = 10_000;

      // Test unwrap fee for swapper destination should be swapperUnwrapFeeBasisPoints (10 basis points = 0.1%)
      const swapperFee = await feeManager.getUnwrapFee(
        amount,
        ethers.ZeroAddress,
        signers.alice.address  // unwrapTo is the swapper
      );
      expect(swapperFee).to.equal(10);

      // Test unwrap fee for non-swapper destination should still apply standard fee
      const nonSwapperFee = await feeManager.getUnwrapFee(
        amount,
        ethers.ZeroAddress,
        signers.bob.address  // unwrapTo is not the swapper
      );
      expect(nonSwapperFee).to.equal(50); // 0.5% of 10,000
    });

    it("should charge zero unwrap fees for SWAPPER role when waiver is active and swapper fee is 0", async function () {
      const swapperRole = await feeManager.SWAPPER_ROLE();

      // Grant SWAPPER role to alice
      await feeManager.grantRole(swapperRole, signers.alice.address);

      // Set fee basis points
      await feeManager.setUnwrapFeeBasisPoints(50); // 0.5% for regular users
      // swapperUnwrapFeeBasisPoints is 0 by default

      // Activate swapper fee waiver
      await feeManager.setSwapperFeeWaiverActive(true);

      const amount = 10_000;

      // Test unwrap fee for swapper destination should be 0 (since swapperUnwrapFeeBasisPoints = 0)
      const swapperFee = await feeManager.getUnwrapFee(
        amount,
        ethers.ZeroAddress,
        signers.alice.address  // unwrapTo is the swapper
      );
      expect(swapperFee).to.equal(0);

      // Test unwrap fee for non-swapper destination should still apply
      const nonSwapperFee = await feeManager.getUnwrapFee(
        amount,
        ethers.ZeroAddress,
        signers.bob.address  // unwrapTo is not the swapper
      );
      expect(nonSwapperFee).to.equal(50); // 0.5% of 10,000
    });
  });
});
