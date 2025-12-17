import { ethers, fhevm } from "hardhat";
import { expect } from "chai";
import {
  getMintEvent,
  deployConfidentialToken,
  deployConfidentialETH,
  getEncryptedBalance,
  getWrappedEvent,
  getERC20TransferEvent,
  wrapETH,
  wrapERC20,
  getWrapFeeBasisPoints,
  verifyWrapperBacking,
} from "./utils";
import { getSigners, Signers } from "./signers";
import type { TestERC20, AdminProvider, DeploymentCoordinator, WrapperFactory, RegulatedERC7984UpgradeableFactory, FeeOnTransferERC20, FeeManager, ERC20FailOnAddressMock } from "../types";
import { deployWrapperFixture, deployTestERC20Fixture, deploySanctionsListFixture, deployTestUnsafeERC20Fixture } from "./fixtures";
import { FhevmType } from "@fhevm/hardhat-plugin";

async function deployFixture(signers: Signers) {
  const { coordinator, coordinatorAddress, adminProvider, confidentialTokenFactory, wrapperFactory } = await deployWrapperFixture(signers);

  const usdc = await deployTestERC20Fixture("USDC");
  const usdcAddress = await usdc.getAddress();

  let transaction = await usdc.mint(signers.alice, ethers.parseUnits("10000", 6));
  await transaction.wait();

  return { coordinator, coordinatorAddress, adminProvider, confidentialTokenFactory, wrapperFactory, usdc, usdcAddress };
}

async function deployFeeOnTransferToken(signers: Signers, transferFeeBasisPoints: number = 100) {
  const FeeOnTransferERC20Factory = await ethers.getContractFactory("FeeOnTransferERC20");
  const feeToken = await FeeOnTransferERC20Factory.deploy("FeeToken", "FEE", 6, transferFeeBasisPoints);
  await feeToken.waitForDeployment();

  // Mint to alice
  const transaction = await feeToken.mint(signers.alice, ethers.parseUnits("10000", 6));
  await transaction.wait();

  return feeToken;
}

describe("Wrapper Fee Handling", function () {
  let signers: Signers;
  let coordinator: DeploymentCoordinator;
  let coordinatorAddress: string;
  let adminProvider: AdminProvider;
  let confidentialTokenFactory: RegulatedERC7984UpgradeableFactory;
  let wrapperFactory: WrapperFactory;
  let usdc: TestERC20;
  let usdcAddress: string;
  let feeManager: FeeManager;

  before(async function () {
    signers = await getSigners();
  });

  beforeEach(async function () {
    ({
      coordinator,
      coordinatorAddress,
      adminProvider,
      confidentialTokenFactory,
      wrapperFactory,
      usdc,
      usdcAddress,
    } = await deployFixture(signers));

    const feeManagerAddress = await adminProvider.feeManager();
    feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
  });

  describe("ETH Wrapping with Fees", function () {
    it("should revert when fee recipient rejects ETH", async function () {
      // Deploy cETH first with normal fee recipient
      const { wrapper } = await deployConfidentialETH(coordinator, signers.alice);

      // Deploy RejectEth contract
      const RejectEthContract = await ethers.getContractFactory("RejectEth");
      const rejectEth = await RejectEthContract.deploy();
      await rejectEth.waitForDeployment();

      // Now set RejectEth as fee recipient AFTER deployment
      await feeManager.setFeeRecipient(await rejectEth.getAddress());

      // Try to wrap ETH - should fail because fee recipient rejects ETH
      const amount = ethers.parseEther("1.0");

      await expect(
        wrapper.connect(signers.alice).wrap(signers.alice.address, amount, { value: amount })
      ).to.be.revertedWithCustomError(wrapper, "EthFeeTransferFailed");
    });

    it("should wrap ETH with fee - comprehensive balance checks", async function () {
      // DEPLOY: cETH
      const { cEth, cEthAddress, wrapperAddress, wrapper } = await deployConfidentialETH(coordinator, signers.alice);

      // Get initial balances
      const aliceEthBefore = await ethers.provider.getBalance(signers.alice.address);
      const royaltiesEthBefore = await ethers.provider.getBalance(signers.royalties.address);
      const wrapperEthBefore = await ethers.provider.getBalance(wrapperAddress);

      // WRAP: ETH -> cETH
      const amount = ethers.parseEther("1.0");
      const rate = await cEth.rate();
      const wrapFeeBasisPoints = await feeManager.wrapFeeBasisPoints();

      // Calculate expected values based on contract logic
      const baseFee = (amount * wrapFeeBasisPoints) / BigInt(10_000);
      const baseAmount = amount - baseFee;
      const wrapDust = baseAmount % rate;
      const transferAmount = baseAmount - wrapDust;
      const totalFee = baseFee + wrapDust;
      const expectedMintAmount = transferAmount / rate;

      // Verify accounting
      expect(amount).to.equal(transferAmount + totalFee);

      const wrapTx = await wrapper.connect(signers.alice).wrap(signers.alice.address, amount, { value: amount });
      const wrapReceipt = await wrapTx.wait();

      // CHECK: alice cETH balance
      const balanceAlice = await getEncryptedBalance(cEth, signers.alice, cEthAddress);
      expect(balanceAlice).to.equal(expectedMintAmount, "Alice should receive exactly expectedMintAmount");

      // CHECK: royalties received NO cETH (fees are in ETH now)
      await expect(
        getEncryptedBalance(cEth, signers.royalties, cEthAddress),
      ).to.be.rejectedWith("Handle is not initialized");

      // CHECK: ETH balances
      const aliceEthAfter = await ethers.provider.getBalance(signers.alice.address);
      const royaltiesEthAfter = await ethers.provider.getBalance(signers.royalties.address);
      const wrapperEthAfter = await ethers.provider.getBalance(wrapperAddress);

      // Alice paid amount + gas
      const gasUsed = (wrapReceipt?.gasUsed ?? 0n) * (wrapReceipt?.gasPrice ?? 0n);
      expect(aliceEthBefore - aliceEthAfter).to.be.closeTo(amount + gasUsed, ethers.parseEther("0.001"));

      // Royalties received totalFee in ETH
      expect(royaltiesEthAfter - royaltiesEthBefore).to.equal(totalFee, "Royalties should receive totalFee in ETH");

      // Wrapper received transferAmount in ETH
      expect(wrapperEthAfter - wrapperEthBefore).to.equal(transferAmount, "Wrapper should hold transferAmount in ETH");

      // CHECK: Mint events
      const mintEvents = getMintEvent(wrapReceipt);
      expect(mintEvents.length).to.equal(1, "Should have 1 mint event (no fee mint in cETH)");
      expect(mintEvents[0].args[1]).to.equal(expectedMintAmount);

      // CHECK: Wrapped event (for ETH, actualFeeReceived = totalFee)
      const wrappedEvents = getWrappedEvent(wrapReceipt);

      expect(wrappedEvents.length).to.equal(1);
      expect(wrappedEvents[0]?.args[0]).to.equal(expectedMintAmount, "Wrapped event mintAmount");
      expect(wrappedEvents[0]?.args[1]).to.equal(amount, "Wrapped event amountIn");
      expect(wrappedEvents[0]?.args[2]).to.equal(totalFee, "Wrapped event feeAmount (ETH)");
      expect(wrappedEvents[0]?.args[3]).to.equal(signers.alice.address, "Wrapped event recipient");

      // VERIFY: Total accounting
      const totalEthIn = amount;
      const totalEthOut = transferAmount + totalFee;
      expect(totalEthIn).to.equal(totalEthOut, "Total ETH in should equal total ETH out");

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("should wrap ETH without fee - comprehensive balance checks", async function () {
      // SETTINGS: remove wrap fee
      await feeManager.setWrapFeeBasisPoints(0);

      // DEPLOY: cETH
      const { cEth, cEthAddress, wrapperAddress, wrapper } = await deployConfidentialETH(coordinator, signers.alice);

      // Get initial balances
      const aliceEthBefore = await ethers.provider.getBalance(signers.alice.address);
      const royaltiesEthBefore = await ethers.provider.getBalance(signers.royalties.address);
      const wrapperEthBefore = await ethers.provider.getBalance(wrapperAddress);

      // WRAP: ETH -> cETH
      const amount = ethers.parseEther("1.0");
      const rate = await cEth.rate();

      // Calculate expected values (no base fee)
      const baseFee = BigInt(0);
      const baseAmount = amount;
      const wrapDust = baseAmount % rate;
      const transferAmount = baseAmount - wrapDust;
      const totalFee = wrapDust; // Only dust, no baseFee
      const expectedMintAmount = transferAmount / rate;

      const wrapTx = await wrapper.connect(signers.alice).wrap(signers.alice.address, amount, { value: amount });
      const wrapReceipt = await wrapTx.wait();

      // CHECK: alice cETH balance
      const balanceAlice = await getEncryptedBalance(cEth, signers.alice, cEthAddress);
      expect(balanceAlice).to.equal(expectedMintAmount);

      // CHECK: royalties received NO cETH
      await expect(
        getEncryptedBalance(cEth, signers.royalties, cEthAddress),
      ).to.be.rejectedWith("Handle is not initialized");

      // CHECK: ETH balances
      const aliceEthAfter = await ethers.provider.getBalance(signers.alice.address);
      const royaltiesEthAfter = await ethers.provider.getBalance(signers.royalties.address);
      const wrapperEthAfter = await ethers.provider.getBalance(wrapperAddress);

      // Royalties received only dust in ETH
      expect(royaltiesEthAfter - royaltiesEthBefore).to.equal(totalFee);

      // Wrapper received transferAmount in ETH
      expect(wrapperEthAfter - wrapperEthBefore).to.equal(transferAmount);

      // VERIFY: Total accounting
      expect(amount).to.equal(transferAmount + totalFee);

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("should handle dust correctly for various amounts", async function () {
      const { cEth, cEthAddress, wrapper, wrapperAddress } = await deployConfidentialETH(coordinator, signers.alice);
      const rate = await cEth.rate();
      const wrapFeeBasisPoints = await feeManager.wrapFeeBasisPoints();

      // Test various amounts with different dust values
      const testAmounts = [
        ethers.parseEther("0.1"),
        ethers.parseEther("0.123456789"),
        ethers.parseEther("1.0"),
        ethers.parseEther("1.999999999"),
      ];

      for (const amount of testAmounts) {
        const royaltiesEthBefore = await ethers.provider.getBalance(signers.royalties.address);
        const wrapperEthBefore = await ethers.provider.getBalance(wrapperAddress);

        const baseFee = (amount * wrapFeeBasisPoints) / BigInt(10_000);
        const baseAmount = amount - baseFee;
        const wrapDust = baseAmount % rate;
        const transferAmount = baseAmount - wrapDust;
        const totalFee = baseFee + wrapDust;
        const expectedMintAmount = transferAmount / rate;

        await wrapper.connect(signers.alice).wrap(signers.alice.address, amount, { value: amount });

        const royaltiesEthAfter = await ethers.provider.getBalance(signers.royalties.address);
        const wrapperEthAfter = await ethers.provider.getBalance(wrapperAddress);

        expect(royaltiesEthAfter - royaltiesEthBefore).to.equal(totalFee, `Royalties should receive totalFee for amount ${amount}`);
        expect(wrapperEthAfter - wrapperEthBefore).to.equal(transferAmount, `Wrapper should hold transferAmount for amount ${amount}`);
        expect(amount).to.equal(transferAmount + totalFee, `Accounting should balance for amount ${amount}`);
      }
    });
  });

  describe("ERC20 Wrapping with Fees", function () {
    it("should wrap USDC with fee - comprehensive balance checks", async function () {
      // DEPLOY: cUSDC
      const { cToken, cTokenAddress, wrapperAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      // Get initial balances
      const aliceUsdcBefore = await usdc.balanceOf(signers.alice.address);
      const royaltiesUsdcBefore = await usdc.balanceOf(signers.royalties.address);
      const wrapperUsdcBefore = await usdc.balanceOf(wrapperAddress);

      // WRAP: USDC -> cUSDC
      const amount = BigInt(1000);
      const rate = await cToken.rate();
      const wrapFeeBasisPoints = await feeManager.wrapFeeBasisPoints();

      // Calculate expected values
      const baseFee = (amount * wrapFeeBasisPoints) / BigInt(10_000);
      const baseAmount = amount - baseFee;
      const wrapDust = baseAmount % rate;
      const transferAmount = baseAmount - wrapDust;
      const totalFee = baseFee + wrapDust;
      const expectedMintAmount = transferAmount / rate;

      // Approve full amount (contract needs to transfer transferAmount + totalFee)
      await usdc.connect(signers.alice).approve(wrapperAddress, amount);

      const wrapTx = await wrapper.connect(signers.alice).wrap(signers.alice.address, amount);
      const wrapReceipt = await wrapTx.wait();

      // CHECK: alice cUSDC balance
      const balanceAlice = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);
      expect(balanceAlice).to.equal(expectedMintAmount, "Alice should receive exactly expectedMintAmount");

      // CHECK: royalties received NO cUSDC (fees are in USDC now)
      await expect(
        getEncryptedBalance(cToken, signers.royalties, cTokenAddress),
      ).to.be.rejectedWith("Handle is not initialized");

      // CHECK: USDC balances
      const aliceUsdcAfter = await usdc.balanceOf(signers.alice.address);
      const royaltiesUsdcAfter = await usdc.balanceOf(signers.royalties.address);
      const wrapperUsdcAfter = await usdc.balanceOf(wrapperAddress);

      // Alice paid full amount
      expect(aliceUsdcBefore - aliceUsdcAfter).to.equal(amount, "Alice should pay full amount");

      // Royalties received totalFee in USDC
      expect(royaltiesUsdcAfter - royaltiesUsdcBefore).to.equal(totalFee, "Royalties should receive totalFee in USDC");

      // Wrapper received transferAmount in USDC
      expect(wrapperUsdcAfter - wrapperUsdcBefore).to.equal(transferAmount, "Wrapper should hold transferAmount in USDC");

      // VERIFY: Total accounting
      expect(amount).to.equal(transferAmount + totalFee, "Total USDC should balance");

      // CHECK: No more and no less tokens
      const totalUsdcMoved = (aliceUsdcBefore - aliceUsdcAfter);
      const totalUsdcReceived = (royaltiesUsdcAfter - royaltiesUsdcBefore) + (wrapperUsdcAfter - wrapperUsdcBefore);
      expect(totalUsdcMoved).to.equal(totalUsdcReceived, "All USDC should be accounted for");

      // CHECK: Mint events
      const mintEvents = getMintEvent(wrapReceipt);
      expect(mintEvents.length).to.equal(1, "Should have 1 mint event");
      expect(mintEvents[0].args[1]).to.equal(expectedMintAmount);

      // CHECK: Wrapped event (for standard ERC20, actualFeeReceived = totalFee)
      const wrappedEvents = getWrappedEvent(wrapReceipt);
      expect(wrappedEvents.length).to.equal(1);
      expect(wrappedEvents[0]?.args[0]).to.equal(expectedMintAmount, "Wrapped event mintAmount");
      expect(wrappedEvents[0]?.args[1]).to.equal(amount, "Wrapped event amountIn");
      expect(wrappedEvents[0]?.args[2]).to.equal(totalFee, "Wrapped event feeAmount (standard ERC20)");
      expect(wrappedEvents[0]?.args[3]).to.equal(signers.alice.address, "Wrapped event recipient");

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("should handle dust correctly for various amounts", async function () {
      const { cToken, cTokenAddress, wrapper, wrapperAddress } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const rate = await cToken.rate();
      const wrapFeeBasisPoints = await feeManager.wrapFeeBasisPoints();

      // Test various amounts with different dust values
      const testAmounts = [
        BigInt(100),
        BigInt(999),
        BigInt(1000),
        BigInt(1234567),
        BigInt(9999999),
      ];

      for (const amount of testAmounts) {
        const royaltiesUsdcBefore = await usdc.balanceOf(signers.royalties.address);
        const wrapperUsdcBefore = await usdc.balanceOf(wrapperAddress);
        const aliceUsdcBefore = await usdc.balanceOf(signers.alice.address);

        const baseFee = (amount * wrapFeeBasisPoints + 10_000n - 1n) / BigInt(10_000);
        const baseAmount = amount - baseFee;
        const wrapDust = baseAmount % rate;
        const transferAmount = baseAmount - wrapDust;
        const totalFee = baseFee + wrapDust;

        await usdc.connect(signers.alice).approve(wrapperAddress, amount);
        await wrapper.connect(signers.alice).wrap(signers.alice.address, amount);

        const royaltiesUsdcAfter = await usdc.balanceOf(signers.royalties.address);
        const wrapperUsdcAfter = await usdc.balanceOf(wrapperAddress);
        const aliceUsdcAfter = await usdc.balanceOf(signers.alice.address);

        expect(royaltiesUsdcAfter - royaltiesUsdcBefore).to.equal(totalFee, `Royalties fee for ${amount}`);
        expect(wrapperUsdcAfter - wrapperUsdcBefore).to.equal(transferAmount, `Wrapper balance for ${amount}`);
        expect(aliceUsdcBefore - aliceUsdcAfter).to.equal(amount, `Alice payment for ${amount}`);
        expect(amount).to.equal(transferAmount + totalFee, `Accounting for ${amount}`);
      }
    });
  });

  it("should use safeTransferFrom for principal transfer", async function () {
    const unsafeUsdc = await deployTestUnsafeERC20Fixture("ERC20ReturnFalseMock", "Unsafe USDC", 6)
    const mintTransaction = await unsafeUsdc.mint(signers.alice, 2_000);
    await mintTransaction.wait();

    const { wrapper } = await deployConfidentialToken(coordinator, unsafeUsdc, signers.alice);

    const amount = BigInt(100);

    await unsafeUsdc.connect(signers.alice).approve(await wrapper.getAddress(), amount);

    await expect(
      wrapper.connect(signers.alice).wrap(signers.alice.address, amount)
    ).to.be.revertedWithCustomError(wrapper, "SafeERC20FailedOperation");
  });

  it("should use safeTransferFrom for fee transfer", async function () {
    // Deploy mock that fails on transfers to fee recipient
    const unsafeUsdcAddress = await (await deployTestUnsafeERC20Fixture("ERC20FailOnAddressMock", "Unsafe USDC", 6)).getAddress();
    const unsafeUsdc = (await ethers.getContractAt("ERC20FailOnAddressMock", unsafeUsdcAddress)) as ERC20FailOnAddressMock;
    const mintTransaction = await unsafeUsdc.mint(signers.alice, 2_000);
    await mintTransaction.wait();

    const { wrapper } = await deployConfidentialToken(coordinator, unsafeUsdc, signers.alice);

    // Configure mock to fail on transfers to fee recipient
    const feeRecipient = await feeManager.getFeeRecipient();
    await unsafeUsdc.setFailOnTransferTo(feeRecipient, true);

    const amount = BigInt(100);

    await unsafeUsdc.connect(signers.alice).approve(await wrapper.getAddress(), amount);

    // This should fail on the second safeTransferFrom (fee transfer to fee recipient)
    await expect(
      wrapper.connect(signers.alice).wrap(signers.alice.address, amount)
    ).to.be.revertedWithCustomError(wrapper, "SafeERC20FailedOperation");
  });

  describe("Fee-on-Transfer Token Support", function () {
    it("should handle fee-on-transfer tokens and transferDust correctly", async function () {
      const transferFeeBasisPoints = 100; // 1% transfer fee
      const feeToken = await deployFeeOnTransferToken(signers, transferFeeBasisPoints);
      const feeTokenAddress = await feeToken.getAddress();

      // DEPLOY: cFEE
      const { cToken, cTokenAddress, wrapperAddress, wrapper } = await deployConfidentialToken(coordinator, feeToken as any, signers.alice);

      // Get initial balances
      const aliceBefore = await feeToken.balanceOf(signers.alice.address);
      const royaltiesBefore = await feeToken.balanceOf(signers.royalties.address);
      const wrapperBefore = await feeToken.balanceOf(wrapperAddress);
      const feeCollectorBefore = await feeToken.balanceOf(await feeToken.feeCollector());

      // WRAP
      const amount = BigInt(10000);
      const rate = await cToken.rate();
      const wrapFeeBasisPoints = await feeManager.wrapFeeBasisPoints();

      // Calculate expected values BEFORE fee-on-transfer
      const baseFee = (amount * wrapFeeBasisPoints) / BigInt(10_000);
      const baseAmount = amount - baseFee;
      const wrapDust = baseAmount % rate;
      const transferAmount = baseAmount - wrapDust;
      const totalFee = baseFee + wrapDust;

      // Calculate actual received after fee-on-transfer
      const transferFee = (transferAmount * BigInt(transferFeeBasisPoints)) / BigInt(10_000);
      const actualReceived = transferAmount - transferFee;
      const transferDust = actualReceived % rate;
      const expectedMintAmount = (actualReceived - transferDust) / rate;

      // Approve full amount
      await feeToken.connect(signers.alice).approve(wrapperAddress, amount);

      const wrapTx = await wrapper.connect(signers.alice).wrap(signers.alice.address, amount);
      const wrapReceipt = await wrapTx.wait();

      // CHECK: balances after
      const aliceAfter = await feeToken.balanceOf(signers.alice.address);
      const royaltiesAfter = await feeToken.balanceOf(signers.royalties.address);
      const wrapperAfter = await feeToken.balanceOf(wrapperAddress);
      const feeCollectorAfter = await feeToken.balanceOf(await feeToken.feeCollector());

      // Alice paid full amount
      expect(aliceBefore - aliceAfter).to.equal(amount, "Alice should pay full amount");

      // Wrapper received less due to fee-on-transfer, minus dust sent back
      expect(wrapperAfter - wrapperBefore).to.equal(actualReceived - transferDust, "Wrapper balance after dust transfer");

      // Royalties received (baseFee + wrapDust) minus the fee-on-transfer, plus transferDust
      // Note: The totalFee transfer from alice to royalties is also subject to fee-on-transfer!
      const totalFeeFee = (totalFee * BigInt(transferFeeBasisPoints)) / BigInt(10_000);
      const totalFeeReceived = totalFee - totalFeeFee;
      const transferDustFee = transferDust > 0 ? (transferDust * BigInt(transferFeeBasisPoints)) / BigInt(10_000) : BigInt(0);
      const transferDustReceived = transferDust - transferDustFee;
      const royaltiesReceived = royaltiesAfter - royaltiesBefore;
      expect(royaltiesReceived).to.equal(totalFeeReceived + transferDustReceived, "Royalties should receive fees and dust after fee-on-transfer");

      // Fee collector received fees from both transfers (transferAmount and totalFee, plus transferDust if any)
      const expectedFeeCollectorTotal = transferFee + totalFeeFee + transferDustFee;
      expect(feeCollectorAfter - feeCollectorBefore).to.equal(expectedFeeCollectorTotal, "Fee collector should receive all transfer fees");

      // Verify alice's cToken balance
      const balanceAlice = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);
      expect(balanceAlice).to.equal(expectedMintAmount, "Alice cToken balance");

      // VERIFY: Total accounting (considering fee-on-transfer loss)
      const totalOut = (wrapperAfter - wrapperBefore) + (royaltiesAfter - royaltiesBefore) + (feeCollectorAfter - feeCollectorBefore);
      expect(aliceBefore - aliceAfter).to.equal(totalOut, "All tokens should be accounted for");

      // CHECK: Mint events
      const mintEvents = getMintEvent(wrapReceipt);
      expect(mintEvents.length).to.equal(1, "Should have 1 mint event");
      expect(mintEvents[0].args[1]).to.equal(expectedMintAmount);

      // CHECK: Wrapped event (for fee-on-transfer tokens, actualFeeReceived < totalFee)
      const wrappedEvents = getWrappedEvent(wrapReceipt);
      expect(wrappedEvents.length).to.equal(1);
      expect(wrappedEvents[0]?.args[0]).to.equal(expectedMintAmount, "Wrapped event mintAmount");
      expect(wrappedEvents[0]?.args[1]).to.equal(amount, "Wrapped event amountIn");
      // The event should emit the ACTUAL fee received by royalties, not the totalFee sent
      expect(wrappedEvents[0]?.args[2]).to.equal(totalFeeReceived + transferDustReceived, "Wrapped event feeAmount (fee-on-transfer)");
      expect(wrappedEvents[0]?.args[3]).to.equal(signers.alice.address, "Wrapped event recipient");

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("should emit Wrapped event with fee amount including transferDust (OZ2-L01 regression test)", async function () {
      // Use 18 decimals (like ETH/DAI) so rate = 10^12, which will produce transferDust
      // Use 1.37% transfer fee (prime-ish) to create remainder when dividing by rate
      const transferFeeBasisPoints = 137;
      const FeeOnTransferERC20Factory = await ethers.getContractFactory("FeeOnTransferERC20");
      const feeToken = await FeeOnTransferERC20Factory.deploy("FeeToken", "FEE", 18, transferFeeBasisPoints);
      await feeToken.waitForDeployment();
      await feeToken.mint(signers.alice, ethers.parseUnits("10000", 18));
      const feeTokenAddress = await feeToken.getAddress();

      const { cToken, cTokenAddress, wrapperAddress, wrapper } = await deployConfidentialToken(coordinator, feeToken as any, signers.alice);

      const rate = await cToken.rate();
      const wrapFeeBasisPoints = await feeManager.wrapFeeBasisPoints();

      // Choose non-round amount that will produce transferDust with fee-on-transfer
      // With 18 decimals and 1.37% transfer fee, this creates transferDust
      const amount = ethers.parseUnits("1.001", 18);

      // Calculate protocol fee and dust
      const baseFee = (amount * wrapFeeBasisPoints) / BigInt(10_000);
      const baseAmount = amount - baseFee;
      const wrapDust = baseAmount % rate;
      const transferAmount = baseAmount - wrapDust;
      const totalFee = baseFee + wrapDust;

      // Calculate actual amounts after fee-on-transfer deductions
      const transferFee = (transferAmount * BigInt(transferFeeBasisPoints)) / BigInt(10_000);
      const actualReceived = transferAmount - transferFee;
      const transferDust = actualReceived % rate;

      // Calculate what the fee recipient actually receives
      const totalFeeFee = (totalFee * BigInt(transferFeeBasisPoints)) / BigInt(10_000);
      const totalFeeReceived = totalFee - totalFeeFee;
      const transferDustFee = transferDust > 0 ? (transferDust * BigInt(transferFeeBasisPoints)) / BigInt(10_000) : BigInt(0);
      const transferDustReceived = transferDust - transferDustFee;

      // Track fee recipient balance to verify emitted amount matches actual received
      const royaltiesBefore = await feeToken.balanceOf(signers.royalties.address);

      await feeToken.connect(signers.alice).approve(wrapperAddress, amount);
      const wrapTx = await wrapper.connect(signers.alice).wrap(signers.alice.address, amount);
      const wrapReceipt = await wrapTx.wait();

      const royaltiesAfter = await feeToken.balanceOf(signers.royalties.address);
      const actualFeeRecipientReceived = royaltiesAfter - royaltiesBefore;

      // Verify transferDust exists (required for this test to be meaningful)
      expect(transferDust).to.be.greaterThan(0, "transferDust must be non-zero for this test");

      // CRITICAL: Verify Wrapped event emits the ACTUAL fee received (including dust)
      const wrappedEvents = getWrappedEvent(wrapReceipt);
      expect(wrappedEvents.length).to.equal(1);

      const emittedFeeAmount = wrappedEvents[0]?.args[2];
      const expectedFeeAmount = totalFeeReceived + transferDustReceived;

      expect(emittedFeeAmount).to.equal(expectedFeeAmount,
        "Wrapped event feeAmount must include both protocol fee and transferDust");

      expect(emittedFeeAmount).to.equal(actualFeeRecipientReceived,
        "Wrapped event feeAmount must match actual amount received by fee recipient");

      // Verify the fix: emittedFeeAmount should be GREATER than just the protocol fee
      expect(emittedFeeAmount).to.be.greaterThan(totalFeeReceived,
        "Emitted fee must include transferDust (old bug would fail this check)");

      // VERIFY: Event order (OZ2-L01 also mentioned transfer order changed)
      // The fix moves feeBalBefore tracking before both transfers, ensuring correct order:
      // 1. Transfer from user to wrapper (for principal)
      // 2. Transfer from user to feeRecipient (for fees)
      // 3. Transfer from wrapper to feeRecipient (for dust)
      const transferEvents = getERC20TransferEvent(wrapReceipt);
      expect(transferEvents.length).to.be.greaterThanOrEqual(3, "Should have at least 3 Transfer events");

      // Find the fee transfer and dust transfer (both to fee recipient)
      const toFeeRecipient = transferEvents.filter((e: any) => e.args.to === signers.royalties.address);
      expect(toFeeRecipient.length).to.equal(2, "Should have exactly 2 transfers to fee recipient");

      // First transfer to fee recipient: from user (totalFee)
      expect(toFeeRecipient[0].args.from).to.equal(signers.alice.address, "First fee transfer from user");

      // Second transfer to fee recipient: from wrapper (transferDust)
      expect(toFeeRecipient[1].args.from).to.equal(wrapperAddress, "Second fee transfer (dust) from wrapper");

      // Verify order: user fee transfer should come BEFORE wrapper dust transfer
      const feeTransferIndex = transferEvents.indexOf(toFeeRecipient[0]);
      const dustTransferIndex = transferEvents.indexOf(toFeeRecipient[1]);
      expect(feeTransferIndex).to.be.lessThan(dustTransferIndex,
        "Fee transfer must occur before dust transfer (OZ2-L01 fix)");
    });

    it("should handle multiple fee-on-transfer percentages", async function () {
      const testFees = [50, 100, 500, 1000]; // 0.5%, 1%, 5%, 10%

      for (const transferFeeBps of testFees) {
        const feeToken = await deployFeeOnTransferToken(signers, transferFeeBps);
        const { cToken, cTokenAddress, wrapperAddress, wrapper } = await deployConfidentialToken(coordinator, feeToken as any, signers.alice);

        const amount = BigInt(10000);
        const rate = await cToken.rate();
        const wrapFeeBasisPoints = await feeManager.wrapFeeBasisPoints();

        const baseFee = (amount * wrapFeeBasisPoints) / BigInt(10_000);
        const baseAmount = amount - baseFee;
        const wrapDust = baseAmount % rate;
        const transferAmount = baseAmount - wrapDust;

        const transferFee = (transferAmount * BigInt(transferFeeBps)) / BigInt(10_000);
        const actualReceived = transferAmount - transferFee;
        const transferDust = actualReceived % rate;
        const expectedMintAmount = (actualReceived - transferDust) / rate;

        await feeToken.connect(signers.alice).approve(wrapperAddress, amount);
        await wrapper.connect(signers.alice).wrap(signers.alice.address, amount);

        const balanceAlice = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);
        expect(balanceAlice).to.equal(expectedMintAmount, `Mint amount for ${transferFeeBps} bps fee`);
      }
    });

    it("should verify no tokens are lost in fee-on-transfer scenario", async function () {
      const feeToken = await deployFeeOnTransferToken(signers, 100);
      const { cToken, cTokenAddress, wrapperAddress, wrapper } = await deployConfidentialToken(coordinator, feeToken as any, signers.alice);

      const totalSupplyBefore = await feeToken.totalSupply();
      const amount = BigInt(10000);

      await feeToken.connect(signers.alice).approve(wrapperAddress, amount);
      await wrapper.connect(signers.alice).wrap(signers.alice.address, amount);

      const totalSupplyAfter = await feeToken.totalSupply();

      // Total supply should not change (no tokens burned/minted)
      expect(totalSupplyAfter).to.equal(totalSupplyBefore, "Total supply should remain constant");
    });
  });

  describe("Edge Cases", function () {
    it("should handle minimum wrap amount", async function () {
      const { cToken, cTokenAddress, wrapper, wrapperAddress } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const rate = await cToken.rate();

      // Minimum amount that results in at least 1 token after fees and dust
      const wrapFeeBasisPoints = await feeManager.wrapFeeBasisPoints();
      const minAmount = rate + (rate * wrapFeeBasisPoints) / BigInt(10_000) + BigInt(1);

      await usdc.connect(signers.alice).approve(wrapperAddress, minAmount);
      await wrapper.connect(signers.alice).wrap(signers.alice.address, minAmount);

      const balance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);
      expect(balance).to.be.gte(1n, "Should mint at least 1 token");
    });

    it("should handle maximum uint64 amount", async function () {
      const { cToken, wrapper, wrapperAddress } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const rate = await cToken.rate();
      const wrapFeeBasisPoints = await feeManager.wrapFeeBasisPoints();

      const maxUint64 = BigInt("18446744073709551615");
      //const maxAmount = maxUint64 * rate;

      // We want: transferAmount = (maxUint64 + 1) * rate
      const targetTransferAmount = (maxUint64) * rate;

      // Working backwards from: transferAmount ≈ amount * (10_000 - wrapFeeBasisPoints) / 10_000
      // (ignoring dust for simplicity, which only makes the issue worse)
      // amount = transferAmount * 10_000 / (10_000 - wrapFeeBasisPoints)
      const maxAmount = (targetTransferAmount * BigInt(10_000)) / (BigInt(10_000) - wrapFeeBasisPoints);

      // Mint enough tokens
      await usdc.mint(signers.alice, maxAmount);
      await usdc.connect(signers.alice).approve(wrapperAddress, maxAmount);

      await expect(
        wrapper.connect(signers.alice).wrap(signers.alice.address, maxAmount)
      ).to.not.be.reverted;
    });

    it("should revert on amount too large causing overflow", async function () {
      const { cToken, wrapper, wrapperAddress } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const rate = await cToken.rate();
      const wrapFeeBasisPoints = await feeManager.wrapFeeBasisPoints();

      // We need to calculate an amount such that after fees are taken,
      // transferAmount / rate will exceed maxUint64
      const maxUint64 = BigInt("18446744073709551615");

      // We want: transferAmount = (maxUint64 + 1) * rate
      const targetTransferAmount = (maxUint64 + BigInt(1)) * rate;

      // Working backwards from: baseAmount = amount - baseFee
      // With ceiling division: baseFee = ceil(amount * wrapFeeBasisPoints / 10_000)
      // For worst case, assume: baseFee ≈ (amount * wrapFeeBasisPoints + 9_999) / 10_000
      // So: baseAmount ≈ amount - (amount * wrapFeeBasisPoints + 9_999) / 10_000
      //     baseAmount ≈ (amount * 10_000 - amount * wrapFeeBasisPoints - 9_999) / 10_000
      //     baseAmount ≈ (amount * (10_000 - wrapFeeBasisPoints) - 9_999) / 10_000
      // Ignoring dust (wrapDust): transferAmount ≈ baseAmount
      // We need: transferAmount ≥ targetTransferAmount
      // So: (amount * (10_000 - wrapFeeBasisPoints) - 9_999) / 10_000 ≥ targetTransferAmount
      //     amount * (10_000 - wrapFeeBasisPoints) - 9_999 ≥ targetTransferAmount * 10_000
      //     amount ≥ (targetTransferAmount * 10_000 + 9_999) / (10_000 - wrapFeeBasisPoints)
      const MAX_BASIS_POINTS = BigInt(10_000);
      const overflowAmount = (targetTransferAmount * MAX_BASIS_POINTS + MAX_BASIS_POINTS - BigInt(1)) / (MAX_BASIS_POINTS - wrapFeeBasisPoints);

      await usdc.mint(signers.alice, overflowAmount);
      await usdc.connect(signers.alice).approve(wrapperAddress, overflowAmount);

      await expect(
        wrapper.connect(signers.alice).wrap(signers.alice.address, overflowAmount)
      ).to.be.reverted;

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });
  });

  describe("Sanctions List", function () {
    it("should prevent ETH wrapping when sender is sanctioned", async function () {
      const { cEth } = await deployConfidentialETH(coordinator, signers.alice);
      const { sanctionsList } = await deploySanctionsListFixture();
      await adminProvider.setSanctionsList(sanctionsList);

      // Sanction alice
      await sanctionsList.addToSanctionsList([signers.alice.address]);

      const amount = ethers.parseEther("0.1");

      // Try to wrap ETH when sender is sanctioned
      await expect(
        wrapETH(coordinator, amount, signers.alice.address, signers.alice),
      ).to.be.revertedWithCustomError(cEth, "SanctionedAddress");
    });

    it("should prevent ETH wrapping when recipient (to_) is sanctioned", async function () {
      const { cEth } = await deployConfidentialETH(coordinator, signers.alice);
      const { sanctionsList } = await deploySanctionsListFixture();
      await adminProvider.setSanctionsList(sanctionsList);

      // Sanction bob (the recipient)
      await sanctionsList.addToSanctionsList([signers.bob.address]);

      const amount = ethers.parseEther("0.1");

      // Try to wrap ETH when recipient is sanctioned
      await expect(
        wrapETH(coordinator, amount, signers.bob.address, signers.alice),
      ).to.be.revertedWithCustomError(cEth, "SanctionedAddress");
    });

    it("should prevent ERC20 wrapping when sender is sanctioned", async function () {
      const { cToken } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const { sanctionsList } = await deploySanctionsListFixture();
      await adminProvider.setSanctionsList(sanctionsList);

      // Sanction alice
      await sanctionsList.addToSanctionsList([signers.alice.address]);

      const amount = BigInt(100);

      // Try to wrap when sender is sanctioned
      await expect(
        wrapERC20(coordinator, usdc, amount, signers.alice.address, signers.alice),
      ).to.be.revertedWithCustomError(cToken, "SanctionedAddress");
    });

    it("should prevent ERC20 wrapping when recipient (to_) is sanctioned", async function () {
      const { cToken } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const { sanctionsList } = await deploySanctionsListFixture();
      await adminProvider.setSanctionsList(sanctionsList);

      // Sanction bob (the recipient)
      await sanctionsList.addToSanctionsList([signers.bob.address]);

      const amount = BigInt(100);

      // Try to wrap when recipient is sanctioned
      await expect(
        wrapERC20(coordinator, usdc, amount, signers.bob.address, signers.alice),
      ).to.be.revertedWithCustomError(cToken, "SanctionedAddress");
    });

    it("should allow wrapping when no sanctions are in place", async function () {
      await deployConfidentialETH(coordinator, signers.alice);
      const { sanctionsList } = await deploySanctionsListFixture();
      await adminProvider.setSanctionsList(sanctionsList);

      // Sanction someone else, not involved in the transaction
      await sanctionsList.addToSanctionsList([signers.charlie.address]);

      const amount = ethers.parseEther("0.1");

      // Should work fine when neither sender nor recipient is sanctioned
      await expect(
        wrapETH(coordinator, amount, signers.bob.address, signers.alice),
      ).to.not.be.reverted;
    });

    it("should wrap when wrapper has no sanctions list", async function () {
      // Deploy token without setting sanctions list
      await deployConfidentialETH(coordinator, signers.alice);

      const amount = ethers.parseEther("0.1");

      // Should work fine without any sanctions list
      await expect(
        wrapETH(coordinator, amount, signers.bob.address, signers.alice),
      ).to.not.be.reverted;
    });

    it("should allow dynamic sanctions updates affecting wrap operations", async function () {
      const { cEth } = await deployConfidentialETH(coordinator, signers.alice);
      const { sanctionsList } = await deploySanctionsListFixture();
      await adminProvider.setSanctionsList(sanctionsList);

      const amount = ethers.parseEther("0.1");

      // Initially should work
      await expect(
        wrapETH(coordinator, amount, signers.bob.address, signers.alice),
      ).to.not.be.reverted;

      // Add bob to sanctions list
      await sanctionsList.addToSanctionsList([signers.bob.address]);

      await expect(
        wrapETH(coordinator, amount, signers.bob.address, signers.alice),
      ).to.be.revertedWithCustomError(cEth, "SanctionedAddress");

      // Remove bob from sanctions list
      await sanctionsList.removeFromSanctionsList([signers.bob.address]);

      // Should work again
      await expect(
        wrapETH(coordinator, amount, signers.bob.address, signers.alice),
      ).to.not.be.reverted;
    });
  });

  describe("Input Validation", function () {
    it("should revert on incorrect ETH amount for ETH wrapping", async function () {
      const { wrapper } = await deployConfidentialETH(coordinator, signers.alice);

      const amount = ethers.parseEther("1.0");
      const incorrectValue = ethers.parseEther("0.5");

      await expect(
        wrapper.connect(signers.alice).wrap(signers.alice.address, amount, { value: incorrectValue })
      ).to.be.revertedWithCustomError(wrapper, "IncorrectEthAmount");
    });

    it("should revert when sending ETH for token wrapping", async function () {
      const { wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      const amount = BigInt(100);
      const ethValue = ethers.parseEther("0.1");

      // Approve first
      await usdc.connect(signers.alice).approve(await wrapper.getAddress(), amount);

      await expect(
        wrapper.connect(signers.alice).wrap(signers.alice.address, amount, { value: ethValue })
      ).to.be.revertedWithCustomError(wrapper, "CannotReceiveEthForTokenWrap");
    });
  });

  describe("Fee Calculation", function () {
    it("should return correct wrap fee", async function () {
      const { wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      const amount = BigInt(100);

      const wrapFeeBasisPoints = await getWrapFeeBasisPoints(wrapper);
      const adminProviderAddress = await wrapper.adminProvider();
      const adminProvider = await ethers.getContractAt("AdminProvider", adminProviderAddress);
      const feeManagerAddress = await adminProvider.feeManager();
      const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
      const wrapFee = await feeManager.getWrapFee(amount, ethers.ZeroAddress, ethers.ZeroAddress);
      const expectedWrapPrice = (amount * wrapFeeBasisPoints) / BigInt(10_000);

      expect(wrapFee).to.equal(expectedWrapPrice);
    });
  });

  describe("Overflow Protection", function () {
    it("ETH should revert when passing boundary: wrapping up to exactly type(uint64).max + 1", async function () {
      const { cEth, wrapperAddress, wrapper } = await deployConfidentialETH(coordinator, signers.alice);

      const maxUint64 = 18446744073709551615n;

      const feeManagerAddress = await adminProvider.feeManager();
      const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
      const wrapFeeBasisPoints = await feeManager.wrapFeeBasisPoints();

      // Calculate amount that results in exactly maxUint64 in wrapper (accounting for fees)
      const rate = await cEth.rate();
      const amount = ((maxUint64 * 10000n) / (10000n - wrapFeeBasisPoints) + 1n) * rate;

      await ethers.provider.send("hardhat_setBalance", [
        signers.alice.address,
        ethers.toBeHex(ethers.parseEther("18446744073709551615"))
      ]);

      // This should succeed (at the limit)
      await expect(
        wrapper.connect(signers.alice).wrap(signers.alice.address, amount, { value: amount })
      ).to.not.be.reverted;

      const wrapperBalance = await usdc.balanceOf(wrapperAddress);
      expect(wrapperBalance).to.be.lte(maxUint64);

      // But two more units should fail since it'll overflow after fee
      const newAmount = 2n * rate
      await expect(
        wrapper.connect(signers.alice).wrap(signers.alice.address, newAmount, { value: newAmount })
      ).to.be.revertedWithCustomError(wrapper, "WrapperBalanceExceedsMaxSupply");

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("ERC20 should revert when passing boundary: wrapping up to exactly type(uint64).max + 1", async function () {
      const { wrapper, wrapperAddress } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      const maxUint64 = 18446744073709551615n;

      const feeManagerAddress = await adminProvider.feeManager();
      const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
      const wrapFeeBasisPoints = await feeManager.wrapFeeBasisPoints();

      // Calculate amount that results in exactly maxUint64 in wrapper (accounting for ceiling division fees)
      // With ceiling division: baseFee = ceil(amount * wrapFeeBasisPoints / 10000)
      // We want: amount - baseFee ≈ maxUint64
      // So: amount - ceil(amount * wrapFeeBasisPoints / 10000) = maxUint64
      // Solving: amount ≈ (maxUint64 * 10000 + 10000 - 1) / (10000 - wrapFeeBasisPoints)
      // This accounts for the ceiling division rounding up the fee
      const MAX_BASIS_POINTS = 10000n;
      const amount = (maxUint64 * MAX_BASIS_POINTS + MAX_BASIS_POINTS - 1n) / (MAX_BASIS_POINTS - wrapFeeBasisPoints);

      await usdc.mint(signers.alice.address, amount + 10000n);
      await usdc.connect(signers.alice).approve(wrapperAddress, ethers.MaxUint256);

      // This should succeed (at the limit)
      await expect(
        wrapper.connect(signers.alice).wrap(signers.alice.address, amount)
      ).to.not.be.reverted;

      const wrapperBalance = await usdc.balanceOf(wrapperAddress);
      expect(wrapperBalance).to.be.lte(maxUint64);

      // But wrapping more should fail
      // With ceiling division, we need to wrap enough that at least 1 unit reaches the wrapper
      // For wrapFeeBasisPoints = 100 (1%): ceil(101 * 100 / 10000) = 2, so 101 - 2 = 99 units reach wrapper
      const additionalAmount = 101n;
      await expect(
        wrapper.connect(signers.alice).wrap(signers.alice.address, additionalAmount)
      ).to.be.revertedWithCustomError(wrapper, "WrapperBalanceExceedsMaxSupply");

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("ETH should not brick wrapper when cTokens are sent directly to wrapper", async function () {
      const { wrapper, wrapperAddress, cEth } = await deployConfidentialETH(coordinator, signers.alice);

      await ethers.provider.send("hardhat_setBalance", [
        wrapperAddress,
        ethers.toBeHex(ethers.parseEther("18446744073709551615"))
      ]);

      expect(await ethers.provider.getBalance(wrapperAddress)).to.equal(ethers.parseEther("18446744073709551615"))

      const rate = await cEth.rate();

      // Should still be able to wrap more tokens without being bricked
      // If the direct transfer bricked the wrapper, this would fail
      const wrapAmount = 10000n * rate;
      await expect(
        wrapper.connect(signers.alice).wrap(signers.alice.address, wrapAmount, { value: wrapAmount })
      ).to.not.be.reverted;

      // VERIFY: Wrapper total supply matches cToken total supply
      const totalSupplyHandle = await cEth.confidentialTotalSupply();
      const totalSupply = await fhevm.publicDecryptEuint(FhevmType.euint64, totalSupplyHandle);
      expect(await wrapper.mintedSupply()).to.equal(totalSupply);
    });

    it("ERC20 should not brick wrapper when cTokens are sent directly to wrapper", async function () {
      const { wrapper, wrapperAddress, cToken } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      const rate = await cToken.rate();

      // Mint enough tokens for testing
      const maxUint64 = 18446744073709551615n;
      await usdc.mint(wrapperAddress, maxUint64 * rate);
      await usdc.mint(signers.alice.address, 100000n * rate);
      await usdc.connect(signers.alice).approve(wrapperAddress, ethers.MaxUint256);

      const wrapAmount = 10000n * rate;
      await expect(
        wrapper.connect(signers.alice).wrap(signers.alice.address, wrapAmount)
      ).to.not.be.reverted;

      // VERIFY: Wrapper total supply matches cToken total supply
      const totalSupplyHandle = await cToken.confidentialTotalSupply();
      const totalSupply = await fhevm.publicDecryptEuint(FhevmType.euint64, totalSupplyHandle);
      expect(await wrapper.mintedSupply()).to.equal(totalSupply);
    });
  });
});
