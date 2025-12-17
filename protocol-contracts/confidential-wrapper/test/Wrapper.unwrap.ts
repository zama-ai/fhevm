import { ethers, fhevm } from "hardhat";
import { expect } from "chai";
import { FhevmType } from "@fhevm/hardhat-plugin";
import {
  deployConfidentialToken,
  deployConfidentialETH,
  getEncryptedBalance,
  unwrapToken,
  verifyWrapperBacking,
  wrapETH,
  wrapERC20,
  getUnwrapFinalizedEvent,
  getBurnEvent,
  getUnwrapStartedEvent,
  checkTotalSupply,
} from "./utils";
import { getSigners, Signers } from "./signers";
import type { TestERC20, AdminProvider, DeploymentCoordinator, FeeManager, ERC20FailOnAddressMock, WrapperReceiverMock } from "../types";
import { deployWrapperFixture, deployTestERC20Fixture, deployTestUnsafeERC20Fixture } from "./fixtures";

async function deployFixture(signers: Signers) {
  const { coordinator, coordinatorAddress, adminProvider, confidentialTokenFactory, wrapperFactory } = await deployWrapperFixture(signers);

  const usdc = await deployTestERC20Fixture("USDC");
  const usdcAddress = await usdc.getAddress();

  const transaction = await usdc.mint(signers.alice, ethers.parseUnits("100000", 6));
  await transaction.wait();

  return { coordinator, coordinatorAddress, adminProvider, confidentialTokenFactory, wrapperFactory, usdc, usdcAddress };
}

describe("Wrapper Unwrap Flow", function () {
  let signers: Signers;
  let coordinator: DeploymentCoordinator;
  let adminProvider: AdminProvider;
  let usdc: TestERC20;
  let feeManager: FeeManager;

  before(async function () {
    signers = await getSigners();
  });

  beforeEach(async function () {
    ({
      coordinator,
      adminProvider,
      usdc,
    } = await deployFixture(signers));

    const feeManagerAddress = await adminProvider.feeManager();
    feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
  });

  describe("ETH Unwrapping with Fees", function () {
    it("should unwrap ETH with fee - comprehensive balance checks", async function () {
      // DEPLOY & WRAP
      const { cEth, cEthAddress, wrapper, wrapperAddress } = await deployConfidentialETH(coordinator, signers.alice);
      const wrapAmount = ethers.parseEther("1.0");
      await wrapETH(coordinator, wrapAmount, signers.alice.address, signers.alice);

      // Verify initial backing
      await verifyWrapperBacking(wrapper);

      // Get initial balances
      const aliceEthBefore = await ethers.provider.getBalance(signers.alice.address);
      const royaltiesEthBefore = await ethers.provider.getBalance(signers.royalties.address);
      const wrapperEthBefore = await ethers.provider.getBalance(wrapperAddress);

      // Get alice's cETH balance
      const aliceCEthBalance = await getEncryptedBalance(cEth, signers.alice, cEthAddress);
      const rate = await cEth.rate();

      // CHECK: requestId before unwrap
      const requestIdBefore = await wrapper.requestId();

      // UNWRAP
      const { unwrapReceipt, unwrapFinalizedReceipt, unwrapFee } = await unwrapToken(
        wrapper,
        signers.alice.address,
        aliceCEthBalance,
        signers.alice
      );

      // CHECK: requestId was incremented after onConfidentialTransferReceived
      const requestIdAfter = await wrapper.requestId();
      expect(requestIdAfter).to.equal(requestIdBefore + 1n);

      // Calculate expected amounts
      const expectedUnwrapAmount = (aliceCEthBalance - unwrapFee) * rate;
      const expectedFeeAmount = unwrapFee * rate;

      // CHECK: alice ETH balance increased
      const aliceEthAfter = await ethers.provider.getBalance(signers.alice.address);
      const gasUsed =  (
        (unwrapReceipt?.gasPrice ?? 0n) * (unwrapReceipt?.gasUsed ?? 0n)
        + (unwrapFinalizedReceipt?.gasPrice ?? 0n) * (unwrapFinalizedReceipt?.gasUsed ?? 0n)
      );

      // CHECK: burn event
      const burnEvents = getBurnEvent(unwrapReceipt);
      expect(burnEvents.length).to.be.equal(1);
      expect(burnEvents[0].args[0]).to.equal(signers.alice.address);
      const burnAmountHandle = burnEvents[0].args[1];
      const burnAmount = await fhevm.userDecryptEuint(
        FhevmType.euint64,
        burnAmountHandle,
        cEth,
        signers.alice,
      );
      expect(burnAmount).to.be.equal(aliceCEthBalance);

      // CHECK: unwrap started event
      const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
      expect(unwrapStartedEvents.length).to.be.equal(1);
      const unwrapStartedEvent = unwrapStartedEvents[0];
      expect(unwrapStartedEvent.args[0]).to.be.equal(true);
      const actualRequestId = unwrapStartedEvent.args[1];
      const expectedRequestId = 0;
      expect(actualRequestId).to.be.equal(expectedRequestId);

      // CHECK: unwrap finalized event
      const unwrapFinalizedEvents = getUnwrapFinalizedEvent(unwrapFinalizedReceipt);
      expect(unwrapFinalizedEvents.length).to.be.equal(1);
      const unwrapFinalizedEvent = unwrapFinalizedEvents[0];
      expect(unwrapFinalizedEvent.args[0]).to.be.equal(actualRequestId);
      expect(unwrapFinalizedEvent.args[1]).to.be.equal(true);
      expect(unwrapFinalizedEvent.args[2]).to.be.equal(true);
      expect(unwrapFinalizedEvent.args[3]).to.be.equal(aliceCEthBalance);
      expect(unwrapFinalizedEvent.args[4]).to.be.equal(expectedUnwrapAmount);
      expect(unwrapFinalizedEvent.args[5]).to.be.equal(unwrapFee * rate);
      expect(unwrapFinalizedEvent.args[6]).to.be.equal(await cEth.nextTxId());

      // Alice should receive unwrapAmount minus gas
      expect(aliceEthAfter - aliceEthBefore).to.equal(expectedUnwrapAmount - gasUsed);

      // CHECK: royalties received fee in ETH
      const royaltiesEthAfter = await ethers.provider.getBalance(signers.royalties.address);
      expect(royaltiesEthAfter - royaltiesEthBefore).to.equal(expectedFeeAmount);

      // CHECK: wrapper balance decreased
      const wrapperEthAfter = await ethers.provider.getBalance(wrapperAddress);
      expect(wrapperEthBefore - wrapperEthAfter).to.equal(expectedUnwrapAmount + expectedFeeAmount);

      // CHECK: alice cETH balance is zero
      expect(await getEncryptedBalance(cEth, signers.alice, cEthAddress)).to.equal(0);

      // VERIFY: Wrapper backing invariant (should be zero now)
      await verifyWrapperBacking(wrapper);

      // VERIFY: Total accounting
      const totalEthOut = expectedUnwrapAmount + expectedFeeAmount;
      expect(wrapperEthBefore - wrapperEthAfter).to.equal(totalEthOut);
    });

    it("should unwrap ETH without fee - comprehensive balance checks", async function () {
      // SETTINGS: remove unwrap fee
      await feeManager.setUnwrapFeeBasisPoints(0);

      // DEPLOY & WRAP
      const { cEth, cEthAddress, wrapper } = await deployConfidentialETH(coordinator, signers.alice);
      const wrapAmount = ethers.parseEther("1.0");
      await wrapETH(coordinator, wrapAmount, signers.alice.address, signers.alice);

      // Get initial balances
      const aliceEthBefore = await ethers.provider.getBalance(signers.alice.address);
      const royaltiesEthBefore = await ethers.provider.getBalance(signers.royalties.address);

      const aliceCEthBalance = await getEncryptedBalance(cEth, signers.alice, cEthAddress);
      const rate = await cEth.rate();

      // UNWRAP
      const { unwrapReceipt, unwrapFinalizedReceipt, unwrapFee } = await unwrapToken(
        wrapper,
        signers.alice.address,
        aliceCEthBalance,
        signers.alice
      );

      expect(unwrapFee).to.equal(0n);

      const expectedUnwrapAmount = aliceCEthBalance * rate;

      // CHECK: alice ETH balance increased by full amount
      const aliceEthAfter = await ethers.provider.getBalance(signers.alice.address);
      const gasUsed =  (
        (unwrapReceipt?.gasPrice ?? 0n) * (unwrapReceipt?.gasUsed ?? 0n)
        + (unwrapFinalizedReceipt?.gasPrice ?? 0n) * (unwrapFinalizedReceipt?.gasUsed ?? 0n)
      );

      expect(aliceEthAfter - aliceEthBefore).to.equal(expectedUnwrapAmount - gasUsed);

      // CHECK: royalties received no fee
      const royaltiesEthAfter = await ethers.provider.getBalance(signers.royalties.address);
      expect(royaltiesEthAfter - royaltiesEthBefore).to.equal(0n);

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("should handle ETH unwrap when fee recipient rejects ETH", async function () {
      // DEPLOY & WRAP
      const { cEth, cEthAddress, wrapper } = await deployConfidentialETH(coordinator, signers.alice);
      const wrapAmount = ethers.parseEther("1.0");
      await wrapETH(coordinator, wrapAmount, signers.alice.address, signers.alice);

      // Deploy RejectEth and set as fee recipient
      const RejectEthContract = await ethers.getContractFactory("RejectEth");
      const rejectEth = await RejectEthContract.deploy();
      await rejectEth.waitForDeployment();
      await feeManager.setFeeRecipient(await rejectEth.getAddress());

      // Get balances
      const aliceEthBefore = await ethers.provider.getBalance(signers.alice.address);

      const aliceCEthBalance = await getEncryptedBalance(cEth, signers.alice, cEthAddress);
      const rate = await cEth.rate();

      // UNWRAP - should succeed, alice gets fee + unwrap amount
      const { unwrapReceipt, unwrapFinalizedReceipt } = await unwrapToken(
        wrapper,
        signers.alice.address,
        aliceCEthBalance,
        signers.alice
      );

      // When fee transfer fails, unwrapAmount should include the fee
      // Protocol takes the hit to maintain parity
      const expectedTotalAmount = aliceCEthBalance * rate;

      const aliceEthAfter = await ethers.provider.getBalance(signers.alice.address);
      const gasUsed =  (
        (unwrapReceipt?.gasPrice ?? 0n) * (unwrapReceipt?.gasUsed ?? 0n)
        + (unwrapFinalizedReceipt?.gasPrice ?? 0n) * (unwrapFinalizedReceipt?.gasUsed ?? 0n)
      );

      expect(aliceEthAfter - aliceEthBefore).to.equal(expectedTotalAmount - gasUsed);

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);

      // VERIFY: RejectEth received nothing
      expect(await ethers.provider.getBalance(await rejectEth.getAddress())).to.equal(0n);
    });

    it.skip("should handle ETH unwrap when unwrap destination rejects ETH (fee succeeds)", async function () {
      // DEPLOY & WRAP
      const { cEth, cEthAddress, wrapper, wrapperAddress } = await deployConfidentialETH(coordinator, signers.alice);
      const wrapAmount = ethers.parseEther("1.0");
      await wrapETH(coordinator, wrapAmount, signers.alice.address, signers.alice);

      // Deploy RejectEth contract as unwrap destination
      const RejectEthContract = await ethers.getContractFactory("RejectEth");
      const rejectEth = await RejectEthContract.deploy();
      await rejectEth.waitForDeployment();
      const rejectEthAddress = await rejectEth.getAddress();

      // Get initial balances
      const royaltiesEthBefore = await ethers.provider.getBalance(signers.royalties.address);
      const wrapperEthBefore = await ethers.provider.getBalance(wrapperAddress);

      const aliceCEthBalanceBefore = await getEncryptedBalance(cEth, signers.alice, cEthAddress);
      const rate = await cEth.rate();

      // UNWRAP to RejectEth address - fee succeeds, unwrap fails
      const { unwrapFinalizedReceipt, unwrapFee } = await unwrapToken(
        wrapper,
        rejectEthAddress,  // This address rejects ETH
        aliceCEthBalanceBefore,
        signers.alice
      );

      const expectedFeeAmount = unwrapFee * rate;

      // CHECK: Fee recipient received fee
      const royaltiesEthAfter = await ethers.provider.getBalance(signers.royalties.address);
      expect(royaltiesEthAfter - royaltiesEthBefore).to.equal(expectedFeeAmount);

      // CHECK: Wrapper lost only the fee amount
      const wrapperEthAfter = await ethers.provider.getBalance(wrapperAddress);
      expect(wrapperEthBefore - wrapperEthAfter).to.equal(expectedFeeAmount);

      // CHECK: RejectEth received nothing
      expect(await ethers.provider.getBalance(rejectEthAddress)).to.equal(0n);

      // CHECK: Alice got reminted (actualBurnAmount - feeAmount64) cTokens
      const aliceCEthBalanceAfter = await getEncryptedBalance(cEth, signers.alice, cEthAddress);
      expect(aliceCEthBalanceAfter).to.equal(aliceCEthBalanceBefore - unwrapFee);

      // CHECK: UnwrapFinalized event shows partial failure
      const unwrapFinalizedEvents = getUnwrapFinalizedEvent(unwrapFinalizedReceipt);
      expect(unwrapFinalizedEvents.length).to.equal(1);
      const event = unwrapFinalizedEvents[0];
      expect(event.args[1]).to.equal(false, "ethUnwrapSuccess should be false");
      expect(event.args[2]).to.equal(true, "ethFeeSuccess should be true");
      expect(event.args[3]).to.be.equal(aliceCEthBalanceBefore);
      expect(event.args[4]).to.be.equal(0);
      expect(event.args[5]).to.be.equal(unwrapFee * rate);
      expect(event.args[6]).to.be.equal((await cEth.nextTxId()) - BigInt(1));

      // VERIFY: Wrapper backing invariant (parity maintained)
      await verifyWrapperBacking(wrapper);
    });

    it.skip("should handle ETH unwrap when both fee and unwrap destinations reject ETH", async function () {
      // DEPLOY & WRAP
      const { cEth, cEthAddress, wrapper, wrapperAddress } = await deployConfidentialETH(coordinator, signers.alice);
      const wrapAmount = ethers.parseEther("1.0");
      await wrapETH(coordinator, wrapAmount, signers.alice.address, signers.alice);

      // Deploy RejectEth contracts
      const RejectEthContract = await ethers.getContractFactory("RejectEth");
      const rejectEthFee = await RejectEthContract.deploy();
      await rejectEthFee.waitForDeployment();
      const rejectEthUnwrap = await RejectEthContract.deploy();
      await rejectEthUnwrap.waitForDeployment();

      // Set RejectEth as fee recipient
      await feeManager.setFeeRecipient(await rejectEthFee.getAddress());

      // Get initial balances
      const wrapperEthBefore = await ethers.provider.getBalance(wrapperAddress);
      const aliceCEthBalanceBefore = await getEncryptedBalance(cEth, signers.alice, cEthAddress);

      // UNWRAP to RejectEth address - both fail
      const { unwrapFinalizedReceipt } = await unwrapToken(
        wrapper,
        await rejectEthUnwrap.getAddress(),  // This address rejects ETH
        aliceCEthBalanceBefore,
        signers.alice
      );

      // CHECK: Wrapper ETH unchanged (both transfers failed)
      const wrapperEthAfter = await ethers.provider.getBalance(wrapperAddress);
      expect(wrapperEthAfter).to.equal(wrapperEthBefore);

      // CHECK: Both RejectEth contracts received nothing
      expect(await ethers.provider.getBalance(await rejectEthFee.getAddress())).to.equal(0n);
      expect(await ethers.provider.getBalance(await rejectEthUnwrap.getAddress())).to.equal(0n);

      // CHECK: Alice got ALL cTokens reminted (actualBurnAmount)
      const aliceCEthBalanceAfter = await getEncryptedBalance(cEth, signers.alice, cEthAddress);
      expect(aliceCEthBalanceAfter).to.equal(aliceCEthBalanceBefore);

      // CHECK: UnwrapFinalized event shows both failed
      const unwrapFinalizedEvents = getUnwrapFinalizedEvent(unwrapFinalizedReceipt);
      expect(unwrapFinalizedEvents.length).to.equal(1);
      const event = unwrapFinalizedEvents[0];
      expect(event.args[1]).to.equal(false, "ethUnwrapSuccess should be false");
      expect(event.args[2]).to.equal(false, "ethFeeSuccess should be false");
      expect(event.args[3]).to.equal(aliceCEthBalanceBefore);
      expect(event.args[4]).to.equal(0n, "unwrapAmount should be 0");
      expect(event.args[5]).to.equal(0n, "feeAmount should be 0");
      expect(event.args[6]).to.be.equal((await cEth.nextTxId()) - BigInt(1));

      // VERIFY: Wrapper backing invariant (parity maintained)
      await verifyWrapperBacking(wrapper);
    });

    it("should handle various unwrap amounts", async function () {
      const { cEth, cEthAddress, wrapper } = await deployConfidentialETH(coordinator, signers.alice);

      const testAmounts = [
        ethers.parseEther("0.001"),
        ethers.parseEther("0.1"),
        ethers.parseEther("1.0"),
        ethers.parseEther("10.0"),
      ];

      for (const wrapAmount of testAmounts) {
        await wrapETH(coordinator, wrapAmount, signers.alice.address, signers.alice);

        const aliceCEthBalance = await getEncryptedBalance(cEth, signers.alice, cEthAddress);

        await unwrapToken(
          wrapper,
          signers.alice.address,
          aliceCEthBalance,
          signers.alice
        );

        // Verify parity after each unwrap
        await verifyWrapperBacking(wrapper);
        await checkTotalSupply(cEth, 0);
      }
    });
  });

  describe("ERC20 Unwrapping with Fees", function () {
    it("should unwrap USDC with fee - comprehensive balance checks", async function () {
      // DEPLOY & WRAP
      const { cToken, cTokenAddress, wrapper, wrapperAddress } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      // Verify initial backing
      await verifyWrapperBacking(wrapper);

      // Get initial balances
      const aliceUsdcBefore = await usdc.balanceOf(signers.alice.address);
      const royaltiesUsdcBefore = await usdc.balanceOf(signers.royalties.address);
      const wrapperUsdcBefore = await usdc.balanceOf(wrapperAddress);

      const aliceCUsdcBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);
      const rate = await cToken.rate();

      // CHECK: requestId before unwrap
      const requestIdBefore = await wrapper.requestId();

      // UNWRAP
      const { unwrapFee } = await unwrapToken(
        wrapper,
        signers.alice.address,
        aliceCUsdcBalance,
        signers.alice
      );

      // CHECK: requestId was incremented after onConfidentialTransferReceived
      const requestIdAfter = await wrapper.requestId();
      expect(requestIdAfter).to.equal(requestIdBefore + 1n);

      // Calculate expected amounts
      const expectedUnwrapAmount = (aliceCUsdcBalance - unwrapFee) * rate;
      const expectedFeeAmount = unwrapFee * rate;

      // CHECK: alice USDC balance increased
      const aliceUsdcAfter = await usdc.balanceOf(signers.alice.address);
      expect(aliceUsdcAfter - aliceUsdcBefore).to.equal(expectedUnwrapAmount);

      // CHECK: royalties received fee in USDC
      const royaltiesUsdcAfter = await usdc.balanceOf(signers.royalties.address);
      expect(royaltiesUsdcAfter - royaltiesUsdcBefore).to.equal(expectedFeeAmount);

      // CHECK: wrapper balance decreased
      const wrapperUsdcAfter = await usdc.balanceOf(wrapperAddress);
      expect(wrapperUsdcBefore - wrapperUsdcAfter).to.equal(expectedUnwrapAmount + expectedFeeAmount);

      // CHECK: alice cUSDC balance is zero
      expect(
        await getEncryptedBalance(cToken, signers.alice, cTokenAddress)
      ).to.equal(0);

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);

      // VERIFY: No tokens created or destroyed
      const totalUsdcOut = (aliceUsdcAfter - aliceUsdcBefore) + (royaltiesUsdcAfter - royaltiesUsdcBefore);
      expect(wrapperUsdcBefore - wrapperUsdcAfter).to.equal(totalUsdcOut);
    });

    it("should unwrap USDC without fee", async function () {
      // SETTINGS: remove unwrap fee
      await feeManager.setUnwrapFeeBasisPoints(0);

      // DEPLOY & WRAP
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      const aliceUsdcBefore = await usdc.balanceOf(signers.alice.address);
      const aliceCUsdcBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);
      const rate = await cToken.rate();

      // UNWRAP
      const { unwrapFee } = await unwrapToken(
        wrapper,
        signers.alice.address,
        aliceCUsdcBalance,
        signers.alice
      );

      expect(unwrapFee).to.equal(0n);

      const expectedUnwrapAmount = aliceCUsdcBalance * rate;

      // CHECK: alice USDC balance increased by full amount
      const aliceUsdcAfter = await usdc.balanceOf(signers.alice.address);
      expect(aliceUsdcAfter - aliceUsdcBefore).to.equal(expectedUnwrapAmount);

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("should handle various unwrap amounts", async function () {
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      const testAmounts = [
        BigInt(100),
        BigInt(1000),
        BigInt(10000),
        BigInt(100000),
      ];

      for (const wrapAmount of testAmounts) {
        await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

        const aliceCUsdcBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

        await unwrapToken(
          wrapper,
          signers.alice.address,
          aliceCUsdcBalance,
          signers.alice
        );

        // Verify parity after each unwrap
        await verifyWrapperBacking(wrapper);
      }
    });

    for (let unsafeERC20Name of ["ERC20FailOnAddressMock", "ERC20RevertOnAddressMock"]) {
      it(`should handle ERC20 unwrap when fee recipient rejects tokens with ${unsafeERC20Name}`, async function () {
        // Deploy mock token that fails on transfers to fee recipient
        const mockUsdcAddress = await (await deployTestUnsafeERC20Fixture(unsafeERC20Name, "Mock USDC", 6)).getAddress();
        const mockUsdc = (await ethers.getContractAt(unsafeERC20Name, mockUsdcAddress)) as ERC20FailOnAddressMock;
        await mockUsdc.mint(signers.alice, 200_000);

        // DEPLOY & WRAP
        const { cToken, cTokenAddress, wrapper, wrapperAddress } = await deployConfidentialToken(coordinator, mockUsdc, signers.alice);
        const wrapAmount = BigInt(100000);
        await wrapERC20(coordinator, mockUsdc, wrapAmount, signers.alice.address, signers.alice);

        // Configure mock to fail on transfers to fee recipient
        const feeRecipient = await feeManager.getFeeRecipient();
        await mockUsdc.setFailOnTransferTo(feeRecipient, true);

        // Get initial balances
        const aliceUsdcBefore = await mockUsdc.balanceOf(signers.alice.address);
        const wrapperUsdcBefore = await mockUsdc.balanceOf(wrapperAddress);
        const royaltiesUsdcBefore = await mockUsdc.balanceOf(feeRecipient);
        const aliceCUsdcBalanceBefore = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);
        const rate = await cToken.rate();

        // UNWRAP - fee transfer fails, but unwrap succeeds (protocol absorbs fee loss)
        const { unwrapFinalizedReceipt } = await unwrapToken(
          wrapper,
          signers.alice.address,
          aliceCUsdcBalanceBefore,
          signers.alice
        );

        const expectedTotalAmount = aliceCUsdcBalanceBefore * rate;

        // CHECK: Alice received total amount (unwrap + fee, protocol absorbed loss)
        const aliceUsdcAfter = await mockUsdc.balanceOf(signers.alice.address);
        expect(aliceUsdcAfter - aliceUsdcBefore).to.equal(expectedTotalAmount);

        // CHECK: Wrapper lost everything
        const wrapperUsdcAfter = await mockUsdc.balanceOf(wrapperAddress);
        expect(wrapperUsdcBefore - wrapperUsdcAfter).to.equal(expectedTotalAmount);

        // CHECK: Fee recipient received nothing (transfer failed)
        const royaltiesUsdcAfter = await mockUsdc.balanceOf(feeRecipient);
        expect(royaltiesUsdcAfter).to.equal(royaltiesUsdcBefore);

        // CHECK: Alice has no cTokens left (not reminted, since unwrap succeeded)
        const aliceCUsdcBalanceAfter = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);
        expect(aliceCUsdcBalanceAfter).to.equal(0n);

        // CHECK: UnwrapFinalized event
        const unwrapFinalizedEvents = getUnwrapFinalizedEvent(unwrapFinalizedReceipt);
        expect(unwrapFinalizedEvents.length).to.equal(1);
        const event = unwrapFinalizedEvents[0];
        expect(event.args[1]).to.equal(true, "unwrapSuccess should be true");
        expect(event.args[2]).to.equal(false, "feeSuccess should be false");
        expect(event.args[4]).to.equal(expectedTotalAmount, "unwrapAmount includes fee");
        expect(event.args[5]).to.equal(0n, "feeAmount should be 0");
        expect(event.args[6]).to.be.equal((await cToken.nextTxId()), "no reimbursement");

        // VERIFY: Wrapper backing invariant maintained
        await verifyWrapperBacking(wrapper);
      });

      it(`should handle ERC20 unwrap when unwrap destination rejects tokens (fee succeeds) with ${unsafeERC20Name}`, async function () {
        // Deploy mock token that fails on transfers to alice
        const mockUsdcAddress = await (await deployTestUnsafeERC20Fixture("ERC20FailOnAddressMock", "Mock USDC", 6)).getAddress();
        const mockUsdc = (await ethers.getContractAt("ERC20FailOnAddressMock", mockUsdcAddress)) as ERC20FailOnAddressMock;
        await mockUsdc.mint(signers.alice, 200_000);

        // DEPLOY & WRAP
        const { cToken, cTokenAddress, wrapper, wrapperAddress } = await deployConfidentialToken(coordinator, mockUsdc, signers.alice);
        const wrapAmount = BigInt(100000);
        await wrapERC20(coordinator, mockUsdc, wrapAmount, signers.alice.address, signers.alice);

        // Configure mock to fail on transfers to alice
        await mockUsdc.setFailOnTransferTo(signers.alice.address, true);

        // Get initial balances
        const aliceUsdcBefore = await mockUsdc.balanceOf(signers.alice.address);
        const wrapperUsdcBefore = await mockUsdc.balanceOf(wrapperAddress);
        const royaltiesUsdcBefore = await mockUsdc.balanceOf(signers.royalties.address);
        const aliceCUsdcBalanceBefore = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);
        const rate = await cToken.rate();

        // UNWRAP - fee succeeds, unwrap fails
        const { unwrapFinalizedReceipt, unwrapFee } = await unwrapToken(
          wrapper,
          signers.alice.address,
          aliceCUsdcBalanceBefore,
          signers.alice
        );

        const expectedFeeAmount = unwrapFee * rate;

        // CHECK: Alice received nothing (transfer failed)
        const aliceUsdcAfter = await mockUsdc.balanceOf(signers.alice.address);
        expect(aliceUsdcAfter).to.equal(aliceUsdcBefore);

        // CHECK: Wrapper lost only fee amount
        const wrapperUsdcAfter = await mockUsdc.balanceOf(wrapperAddress);
        expect(wrapperUsdcBefore - wrapperUsdcAfter).to.equal(expectedFeeAmount);

        // CHECK: Fee recipient received fee
        const royaltiesUsdcAfter = await mockUsdc.balanceOf(signers.royalties.address);
        expect(royaltiesUsdcAfter - royaltiesUsdcBefore).to.equal(expectedFeeAmount);

        // CHECK: Alice got reminted principal only (actualBurnAmount - feeAmount64)
        const aliceCUsdcBalanceAfter = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);
        expect(aliceCUsdcBalanceAfter).to.equal(aliceCUsdcBalanceBefore - unwrapFee);

        // CHECK: UnwrapFinalized event
        const unwrapFinalizedEvents = getUnwrapFinalizedEvent(unwrapFinalizedReceipt);
        expect(unwrapFinalizedEvents.length).to.equal(1);
        const event = unwrapFinalizedEvents[0];
        expect(event.args[1]).to.equal(false, "unwrapSuccess should be false");
        expect(event.args[2]).to.equal(true, "feeSuccess should be true");
        expect(event.args[4]).to.equal(0n, "unwrapAmount should be 0");
        expect(event.args[5]).to.equal(expectedFeeAmount, "feeAmount");
        expect(event.args[6]).to.be.equal((await cToken.nextTxId()) - BigInt(1));

        // VERIFY: Wrapper backing invariant maintained
        await verifyWrapperBacking(wrapper);
      });

      it(`should handle ERC20 unwrap when both transfers fail with ${unsafeERC20Name}`, async function () {
        // Deploy mock token that fails on transfers to both fee recipient and alice
        const mockUsdcAddress = await (await deployTestUnsafeERC20Fixture("ERC20FailOnAddressMock", "Mock USDC", 6)).getAddress();
        const mockUsdc = (await ethers.getContractAt("ERC20FailOnAddressMock", mockUsdcAddress)) as ERC20FailOnAddressMock;
        await mockUsdc.mint(signers.alice, 200_000);

        // DEPLOY & WRAP
        const { cToken, cTokenAddress, wrapper, wrapperAddress } = await deployConfidentialToken(coordinator, mockUsdc, signers.alice);
        const wrapAmount = BigInt(100000);
        await wrapERC20(coordinator, mockUsdc, wrapAmount, signers.alice.address, signers.alice);

        // Configure mock to fail on transfers to both
        const feeRecipient = await feeManager.getFeeRecipient();
        await mockUsdc.setFailOnTransferTo(feeRecipient, true);
        await mockUsdc.setFailOnTransferTo(signers.alice.address, true);

        // Get initial balances
        const aliceUsdcBefore = await mockUsdc.balanceOf(signers.alice.address);
        const wrapperUsdcBefore = await mockUsdc.balanceOf(wrapperAddress);
        const royaltiesUsdcBefore = await mockUsdc.balanceOf(feeRecipient);
        const aliceCUsdcBalanceBefore = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

        // UNWRAP - both fail
        const { unwrapFinalizedReceipt } = await unwrapToken(
          wrapper,
          signers.alice.address,
          aliceCUsdcBalanceBefore,
          signers.alice
        );

        // CHECK: Alice USDC unchanged
        const aliceUsdcAfter = await mockUsdc.balanceOf(signers.alice.address);
        expect(aliceUsdcAfter).to.equal(aliceUsdcBefore);

        // CHECK: Wrapper USDC unchanged (both transfers failed)
        const wrapperUsdcAfter = await mockUsdc.balanceOf(wrapperAddress);
        expect(wrapperUsdcAfter).to.equal(wrapperUsdcBefore);

        // CHECK: Fee recipient USDC unchanged
        const royaltiesUsdcAfter = await mockUsdc.balanceOf(feeRecipient);
        expect(royaltiesUsdcAfter).to.equal(royaltiesUsdcBefore);

        // CHECK: Alice got ALL cTokens reminted (actualBurnAmount)
        const aliceCUsdcBalanceAfter = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);
        expect(aliceCUsdcBalanceAfter).to.equal(aliceCUsdcBalanceBefore);

        // CHECK: UnwrapFinalized event shows both failed
        const unwrapFinalizedEvents = getUnwrapFinalizedEvent(unwrapFinalizedReceipt);
        expect(unwrapFinalizedEvents.length).to.equal(1);
        const event = unwrapFinalizedEvents[0];
        expect(event.args[1]).to.equal(false, "unwrapSuccess should be false");
        expect(event.args[2]).to.equal(false, "feeSuccess should be false");
        expect(event.args[4]).to.equal(0n, "unwrapAmount should be 0");
        expect(event.args[5]).to.equal(0n, "feeAmount should be 0");
        expect(event.args[6]).to.be.equal((await cToken.nextTxId()) - BigInt(1));

        // VERIFY: Wrapper backing invariant maintained
        await verifyWrapperBacking(wrapper);
      });
    }
  });

  describe("Unwrap to Different Recipient", function () {
    it("should unwrap ETH to bob", async function () {
      // DEPLOY & WRAP
      const { cEth, cEthAddress, wrapper } = await deployConfidentialETH(coordinator, signers.alice);
      const wrapAmount = ethers.parseEther("1.0");
      await wrapETH(coordinator, wrapAmount, signers.alice.address, signers.alice);

      const bobEthBefore = await ethers.provider.getBalance(signers.bob.address);
      const aliceCEthBalance = await getEncryptedBalance(cEth, signers.alice, cEthAddress);
      const rate = await cEth.rate();

      // UNWRAP to bob
      const { unwrapFee } = await unwrapToken(
        wrapper,
        signers.bob.address,  // Different recipient
        aliceCEthBalance,
        signers.alice
      );

      const expectedUnwrapAmount = (aliceCEthBalance - unwrapFee) * rate;

      // CHECK: bob received the ETH
      const bobEthAfter = await ethers.provider.getBalance(signers.bob.address);
      expect(bobEthAfter - bobEthBefore).to.equal(expectedUnwrapAmount);

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("should unwrap USDC to bob", async function () {
      // DEPLOY & WRAP
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      const bobUsdcBefore = await usdc.balanceOf(signers.bob.address);
      const aliceCUsdcBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);
      const rate = await cToken.rate();

      // UNWRAP to bob
      const { unwrapFee } = await unwrapToken(
        wrapper,
        signers.bob.address,  // Different recipient
        aliceCUsdcBalance,
        signers.alice
      );

      const expectedUnwrapAmount = (aliceCUsdcBalance - unwrapFee) * rate;

      // CHECK: bob received the USDC
      const bobUsdcAfter = await usdc.balanceOf(signers.bob.address);
      expect(bobUsdcAfter - bobUsdcBefore).to.equal(expectedUnwrapAmount);

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });
  });

  describe("Edge Cases", function () {
    it("should handle unwrap of 0 tokens", async function () {
      // DEPLOY & WRAP
      const { cToken, cTokenAddress, wrapper, wrapperAddress } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      // Get initial balances
      const aliceUsdcBefore = await usdc.balanceOf(signers.alice.address);
      const royaltiesUsdcBefore = await usdc.balanceOf(signers.royalties.address);
      const wrapperUsdcBefore = await usdc.balanceOf(wrapperAddress);
      const aliceCTokenBefore = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

      // UNWRAP 0 tokens
      const { unwrapReceipt, unwrapFinalizedReceipt } = await unwrapToken(
        wrapper,
        signers.alice.address,
        0n, // Try to unwrap 0
        signers.alice
      );

      // CHECK: UnwrapStarted event shows success (transfer of 0 succeeds)
      const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
      expect(unwrapStartedEvents.length).to.equal(1);
      expect(unwrapStartedEvents[0].args[0]).to.equal(true, "returnVal should be true");

      // CHECK: UnwrapFinalized event shows failure path (actualBurnAmount is 0)
      const unwrapFinalizedEvents = getUnwrapFinalizedEvent(unwrapFinalizedReceipt);
      expect(unwrapFinalizedEvents.length).to.equal(1);
      const event = unwrapFinalizedEvents[0];
      expect(event.args[1]).to.equal(false, "ethUnwrapSuccess should be false (failure path)");
      expect(event.args[2]).to.equal(false, "ethFeeSuccess should be false (failure path)");
      expect(event.args[3]).to.equal(0n, "actualBurnAmount should be 0");
      expect(event.args[4]).to.equal(0n, "unwrapAmount should be 0");
      expect(event.args[5]).to.equal(0n, "feeAmount should be 0");

      // CHECK: All balances unchanged
      const aliceUsdcAfter = await usdc.balanceOf(signers.alice.address);
      const royaltiesUsdcAfter = await usdc.balanceOf(signers.royalties.address);
      const wrapperUsdcAfter = await usdc.balanceOf(wrapperAddress);
      const aliceCTokenAfter = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

      expect(aliceUsdcAfter).to.equal(aliceUsdcBefore, "Alice USDC should be unchanged");
      expect(royaltiesUsdcAfter).to.equal(royaltiesUsdcBefore, "Royalties USDC should be unchanged");
      expect(wrapperUsdcAfter).to.equal(wrapperUsdcBefore, "Wrapper USDC should be unchanged");
      expect(aliceCTokenAfter).to.equal(aliceCTokenBefore, "Alice cToken should be unchanged");

      // VERIFY: Wrapper backing invariant maintained
      await verifyWrapperBacking(wrapper);
    });

    it("should revert unwrap to zero address", async function () {
      const { cEth, cEthAddress, wrapper } = await deployConfidentialETH(coordinator, signers.alice);
      const wrapAmount = ethers.parseEther("1.0");
      await wrapETH(coordinator, wrapAmount, signers.alice.address, signers.alice);

      const aliceCEthBalance = await getEncryptedBalance(cEth, signers.alice, cEthAddress);

      // Try to unwrap to zero address
      await expect(
        unwrapToken(
          wrapper,
          ethers.ZeroAddress,
          aliceCEthBalance,
          signers.alice
        )
      ).to.be.revertedWithCustomError(wrapper, "CannotSendToZeroAddress");
    });

    it("should handle minimum unwrap amount", async function () {
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const rate = await cToken.rate();

      // Wrap minimum amount
      const minWrapAmount = rate * BigInt(10); // Enough for at least 1 token after fees
      await wrapERC20(coordinator, usdc, minWrapAmount, signers.alice.address, signers.alice);

      const aliceCUsdcBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

      // CHECK: requestId before unwrap
      const requestIdBefore = await wrapper.requestId();

      // UNWRAP
      await unwrapToken(
        wrapper,
        signers.alice.address,
        aliceCUsdcBalance,
        signers.alice
      );

      // CHECK: requestId was incremented after onConfidentialTransferReceived
      const requestIdAfter = await wrapper.requestId();
      expect(requestIdAfter).to.equal(requestIdBefore + 1n);

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("should handle partial unwrap", async function () {
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      const aliceCUsdcBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

      // Unwrap only half
      const unwrapAmount = aliceCUsdcBalance / BigInt(2);

      // CHECK: requestId before unwrap
      const requestIdBefore = await wrapper.requestId();

      await unwrapToken(
        wrapper,
        signers.alice.address,
        unwrapAmount,
        signers.alice
      );

      // CHECK: requestId was incremented after onConfidentialTransferReceived
      const requestIdAfter = await wrapper.requestId();
      expect(requestIdAfter).to.equal(requestIdBefore + 1n);

      // CHECK: alice still has remaining cUSDC
      const remainingBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);
      expect(remainingBalance).to.be.gt(0n);

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("should handle multiple unwraps", async function () {
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      const aliceCUsdcBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

      // CHECK: requestId before unwraps
      const requestIdStart = await wrapper.requestId();

      // Unwrap in 4 parts
      for (let i = 0; i < 4; i++) {
        const unwrapAmount = aliceCUsdcBalance / BigInt(4);

        const requestIdBefore = await wrapper.requestId();

        await unwrapToken(
          wrapper,
          signers.alice.address,
          unwrapAmount,
          signers.alice
        );

        // CHECK: requestId was incremented after each onConfidentialTransferReceived
        const requestIdAfter = await wrapper.requestId();
        expect(requestIdAfter).to.equal(requestIdBefore + 1n);

        // Verify parity after each unwrap
        await verifyWrapperBacking(wrapper);
      }

      // CHECK: requestId incremented 4 times total
      const requestIdEnd = await wrapper.requestId();
      expect(requestIdEnd).to.equal(requestIdStart + 4n);

      // After 4 unwraps, balance should be very small or zero
      expect(
        await getEncryptedBalance(cToken, signers.alice, cTokenAddress)
      ).to.equal(0);
    });
  });

  describe("Fee Calculation", function () {
    it("should return correct unwrap fee", async function () {
      const { wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      const amount = BigInt(100);

      const adminProviderAddress = await wrapper.adminProvider();
      const adminProvider = await ethers.getContractAt("AdminProvider", adminProviderAddress);
      const feeManagerAddress = await adminProvider.feeManager();
      const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
      const unwrapFeeBasisPoints = await feeManager.unwrapFeeBasisPoints();
      const unwrapFee = await feeManager.getUnwrapFee(amount, ethers.ZeroAddress, ethers.ZeroAddress);
      const expectedUnwrapFee = (amount * unwrapFeeBasisPoints) / BigInt(10_000);

      expect(unwrapFee).to.equal(expectedUnwrapFee);
    });

    it("should commit fee basis points at unwrap start and not be affected by fee changes", async function () {
      // This test verifies the fix for OZ2-L02: Fee parameters are committed when unwrap starts
      // and the committed values are used in finalizeUnwrap, regardless of subsequent fee changes

      // DEPLOY & WRAP
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      // Get alice's balance
      const aliceCTokenBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);
      const rate = await cToken.rate();

      // Get initial unwrap fee basis points
      const initialUnwrapFeeBps = await feeManager.unwrapFeeBasisPoints();
      expect(initialUnwrapFeeBps).to.equal(100n, "Initial fee should be 100 bps (1%)");

      // Calculate expected fee with INITIAL rate
      const expectedFeeAmount = (aliceCTokenBalance * initialUnwrapFeeBps) / BigInt(10_000);

      // START unwrap (onConfidentialTransferReceived) - this commits the fee
      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [signers.alice.address, signers.alice.address, "0x"]
      );

      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(cTokenAddress, signers.alice.address)
        .add64(aliceCTokenBalance)
        .encrypt();

      const unwrapTx = await cToken.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapper.getAddress(),
        encryptedUnwrapAmount.handles[0],
        encryptedUnwrapAmount.inputProof,
        data,
      );
      const unwrapReceipt = await unwrapTx.wait();

      // Get requestId from UnwrapStarted event
      const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
      expect(unwrapStartedEvents.length).to.equal(1);
      const unwrapStartedEvent = unwrapStartedEvents[0];
      const requestId = unwrapStartedEvent.args[1];

      // CHANGE FEE PARAMETERS before finalization (this should NOT affect the unwrap)
      const newUnwrapFeeBps = BigInt(500); // 5% - much higher!
      await feeManager.setUnwrapFeeBasisPoints(newUnwrapFeeBps);

      // Verify fee was actually changed
      const currentUnwrapFeeBps = await feeManager.unwrapFeeBasisPoints();
      expect(currentUnwrapFeeBps).to.equal(newUnwrapFeeBps, "Fee should be changed to 500 bps");

      // Get balances before finalization
      const aliceUsdcBefore = await usdc.balanceOf(signers.alice.address);
      const royaltiesUsdcBefore = await usdc.balanceOf(signers.royalties.address);

      // FINALIZE unwrap - should use COMMITTED fee (100 bps), not current fee (500 bps)
      const publicDecryptResults = await fhevm.publicDecrypt([
        unwrapStartedEvent.args[5],
        unwrapStartedEvent.args[6],
      ]);
      const abiEncodedClearBurnResults = publicDecryptResults.abiEncodedClearValues;
      const decryptionProof = publicDecryptResults.decryptionProof;

      const unwrapFinalizedTx = await wrapper.connect(signers.alice).finalizeUnwrap(
        unwrapStartedEvent.args.requestId,
        abiEncodedClearBurnResults,
        decryptionProof,
      );
      const finalizeReceipt = await unwrapFinalizedTx.wait();

      // Get balances after finalization
      const aliceUsdcAfter = await usdc.balanceOf(signers.alice.address);
      const royaltiesUsdcAfter = await usdc.balanceOf(signers.royalties.address);

      // Calculate actual fee paid
      const actualFeeAmount = royaltiesUsdcAfter - royaltiesUsdcBefore;
      const actualUnwrapAmount = aliceUsdcAfter - aliceUsdcBefore;

      // VERIFY: Fee was calculated using INITIAL (committed) basis points, not the changed ones
      const expectedFeeAmount256 = expectedFeeAmount * rate;
      const expectedUnwrapAmount = aliceCTokenBalance * rate - expectedFeeAmount256;

      expect(actualFeeAmount).to.equal(expectedFeeAmount256,
        "Fee should be calculated with committed rate (100 bps), not changed rate (500 bps)");
      expect(actualUnwrapAmount).to.equal(expectedUnwrapAmount,
        "Unwrap amount should reflect committed fee");

      // VERIFY: Event shows correct fee (with committed rate)
      const unwrapFinalizedEvents = getUnwrapFinalizedEvent(finalizeReceipt);
      expect(unwrapFinalizedEvents.length).to.equal(1);
      expect(unwrapFinalizedEvents[0].args[5]).to.equal(expectedFeeAmount256,
        "Event should show fee with committed basis points");

      // VERIFY: If fee was calculated with NEW rate (500 bps), it would be 5x higher
      const wrongFeeAmount = (aliceCTokenBalance * newUnwrapFeeBps) / BigInt(10_000) * rate;
      expect(actualFeeAmount).to.not.equal(wrongFeeAmount,
        "Fee should NOT be calculated with the changed rate");
      expect(actualFeeAmount).to.be.lt(wrongFeeAmount,
        "Committed fee should be less than what new rate would charge");

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });
  });

  describe("Security Checks", function () {
    it("should prevent finalizeUnwrap replay", async function () {
      const { wrapper, wrapperAddress } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      // set wrap fees to 0 to simplify accounting
      await feeManager.setWrapFeeBasisPoints(0);

      const unwrapAmount = BigInt(100000);
      const wrapAmount = unwrapAmount * BigInt(2);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      // Unwrap half of what was wrapped
      const { unwrapFinalizedReceipt } = await unwrapToken(
        wrapper,
        signers.alice.address,
        unwrapAmount,
        signers.alice
      );

      const wrapperBalanceBeforeReplay = await usdc.balanceOf(wrapperAddress);
      // CHECK: first unwrap was successful
      expect(wrapperBalanceBeforeReplay).to.equal(unwrapAmount);

      if (unwrapFinalizedReceipt === null) {
        throw Error("unwrapFinalizedReceipt is null");
      }

      const finalizeUnwrapTx = await ethers.provider.getTransaction(unwrapFinalizedReceipt.hash);

      if (finalizeUnwrapTx === null) {
        throw Error("unwrapFinalizedReceipt is null");
      }

      const parsedTx = wrapper.interface.parseTransaction({data: finalizeUnwrapTx.data, value: finalizeUnwrapTx.value});
      if (!parsedTx) {
        throw Error("Failed to parse transaction");
      }

      // Extract args - need to copy arrays to avoid read-only issues
      const requestId = parsedTx.args[0];
      const abiEncodedClearBurnAmounts = parsedTx.args[1];
      const decryptionProof = parsedTx.args[2];

      // CHECK: replay raises
      await expect(wrapper.finalizeUnwrap(requestId, abiEncodedClearBurnAmounts, decryptionProof))
        .to.be.revertedWithCustomError(wrapper, "ERC7984InvalidGatewayRequest")
        .withArgs(requestId);

      // CHECK: wrapper underlying balance did not change
      expect(await usdc.balanceOf(wrapperAddress)).to.equal(wrapperBalanceBeforeReplay);

      // CHECK: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    })

    it("should reject unwrap from wrong confidential token", async function () {
      const usdt = await deployTestERC20Fixture("USDT");
      const usdtMintTx = await usdt.mint(signers.alice, ethers.parseUnits("100000", 6));
      await usdtMintTx.wait();

      // DEPLOY first confidential token + wrapper
      const { cToken: cToken1, cTokenAddress: cToken1Address, wrapper: wrapper1, wrapperAddress: wrapper1Address } =
        await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      // DEPLOY second confidential token (different wrapper)
      const { cToken: cToken2, cTokenAddress: cToken2Address } =
        await deployConfidentialToken(coordinator, usdt, signers.alice);
      await wrapERC20(coordinator, usdt, wrapAmount, signers.alice.address, signers.alice);

      // Get initial balances
      const alice1BalanceBefore = await getEncryptedBalance(cToken1, signers.alice, cToken1Address);
      const alice2BalanceBefore = await getEncryptedBalance(cToken2, signers.alice, cToken2Address);
      const wrapper1UsdcBefore = await usdc.balanceOf(wrapper1Address);

      // Attempt to unwrap cToken2 to wrapper1 (wrong wrapper)
      // This should fail the msg.sender check since msg.sender will be cToken2, not cToken1
      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [signers.alice.address, signers.alice.address, "0x"]
      );

      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(await cToken2.getAddress(), signers.alice.address)
        .add64(42)
        .encrypt();

      const tx = await cToken2.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        wrapper1Address,
        encryptedUnwrapAmount.handles[0],
        encryptedUnwrapAmount.inputProof,
        data,
      );
      const receipt = await tx.wait();

      // CHECK: UnwrappedStarted event shows rejection (returnVal = false, requestId = 0)
      const unwrapStartedEvents = getUnwrapStartedEvent(receipt);
      expect(unwrapStartedEvents.length).to.equal(1);
      expect(unwrapStartedEvents[0].args[0]).to.equal(false, "returnVal should be false");
      expect(unwrapStartedEvents[0].args[1]).to.equal(0n, "requestId should be 0");

      // CHECK: Alice's balances unchanged (transfer was rejected)
      const alice1BalanceAfter = await getEncryptedBalance(cToken1, signers.alice, cToken1Address);
      const alice2BalanceAfter = await getEncryptedBalance(cToken2, signers.alice, cToken2Address);
      expect(alice1BalanceAfter).to.equal(alice1BalanceBefore, "cToken1 balance should be unchanged");
      expect(alice2BalanceAfter).to.equal(alice2BalanceBefore, "cToken2 balance should be unchanged");

      // CHECK: Wrapper USDC balance unchanged
      const wrapper1UsdcAfter = await usdc.balanceOf(wrapper1Address);
      expect(wrapper1UsdcAfter).to.equal(wrapper1UsdcBefore, "Wrapper USDC balance should be unchanged");

      // VERIFY: Wrapper backing invariant maintained for both wrappers
      await verifyWrapperBacking(wrapper1);
    });
  });

  describe("Callback Return Value Handling", function () {
    it("should return true from finalizeUnwrap when callback returns true", async function () {
      // Deploy mock receiver
      const WrapperReceiverMockFactory = await ethers.getContractFactory("WrapperReceiverMock");
      const receiverMock = await WrapperReceiverMockFactory.deploy() as WrapperReceiverMock;
      await receiverMock.waitForDeployment();
      const receiverMockAddress = await receiverMock.getAddress();

      // DEPLOY & WRAP
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      // Fund the mock receiver with some USDC for the test
      await usdc.mint(receiverMockAddress, wrapAmount);

      // Get alice's balance
      const aliceCTokenBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

      // Set callback to return true (default, but being explicit)
      await receiverMock.setReturnValue(true);

      // START unwrap to the receiver mock
      const callbackData = ethers.AbiCoder.defaultAbiCoder().encode(["string"], ["test-data"]);
      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [receiverMockAddress, signers.alice.address, callbackData]
      );

      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(cTokenAddress, signers.alice.address)
        .add64(aliceCTokenBalance)
        .encrypt();

      const unwrapTx = await cToken.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapper.getAddress(),
        encryptedUnwrapAmount.handles[0],
        encryptedUnwrapAmount.inputProof,
        data,
      );
      const unwrapReceipt = await unwrapTx.wait();

      // Get requestId from UnwrapStarted event
      const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
      expect(unwrapStartedEvents.length).to.equal(1);
      const unwrapStartedEvent = unwrapStartedEvents[0];

      // FINALIZE unwrap - use staticCall to check return value
      const publicDecryptResults = await fhevm.publicDecrypt([
        unwrapStartedEvent.args[5],
        unwrapStartedEvent.args[6],
      ]);

      // Use staticCall to check return value without executing
      const returnValue = await wrapper.connect(signers.alice).finalizeUnwrap.staticCall(
        unwrapStartedEvent.args.requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof,
      );

      // CHECK: finalizeUnwrap should return true
      expect(returnValue).to.equal(true, "finalizeUnwrap should return true when callback returns true");

      // Now actually execute the transaction
      const unwrapFinalizedTx = await wrapper.connect(signers.alice).finalizeUnwrap(
        unwrapStartedEvent.args.requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof,
      );
      await unwrapFinalizedTx.wait();

      // CHECK: callback was called with expected params
      expect(await receiverMock.callbackCount()).to.equal(1n);
      expect(await receiverMock.lastUnwrapRequestId()).to.equal(unwrapStartedEvent.args.requestId);

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("should return false from finalizeUnwrap when callback returns false", async function () {
      // Deploy mock receiver
      const WrapperReceiverMockFactory = await ethers.getContractFactory("WrapperReceiverMock");
      const receiverMock = await WrapperReceiverMockFactory.deploy() as WrapperReceiverMock;
      await receiverMock.waitForDeployment();
      const receiverMockAddress = await receiverMock.getAddress();

      // DEPLOY & WRAP
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      // Fund the mock receiver with some USDC for the test
      await usdc.mint(receiverMockAddress, wrapAmount);

      // Get alice's balance
      const aliceCTokenBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

      // Set callback to return false
      await receiverMock.setReturnValue(false);

      // START unwrap to the receiver mock
      const callbackData = ethers.AbiCoder.defaultAbiCoder().encode(["string"], ["test-data"]);
      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [receiverMockAddress, signers.alice.address, callbackData]
      );

      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(cTokenAddress, signers.alice.address)
        .add64(aliceCTokenBalance)
        .encrypt();

      const unwrapTx = await cToken.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapper.getAddress(),
        encryptedUnwrapAmount.handles[0],
        encryptedUnwrapAmount.inputProof,
        data,
      );
      const unwrapReceipt = await unwrapTx.wait();

      // Get requestId from UnwrapStarted event
      const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
      expect(unwrapStartedEvents.length).to.equal(1);
      const unwrapStartedEvent = unwrapStartedEvents[0];

      // FINALIZE unwrap - use staticCall to check return value
      const publicDecryptResults = await fhevm.publicDecrypt([
        unwrapStartedEvent.args[5],
        unwrapStartedEvent.args[6],
      ]);

      // Use staticCall to check return value without executing
      const returnValue = await wrapper.connect(signers.alice).finalizeUnwrap.staticCall(
        unwrapStartedEvent.args.requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof,
      );

      // CHECK: finalizeUnwrap should return false
      expect(returnValue).to.equal(false, "finalizeUnwrap should return false when callback returns false");

      // Now actually execute the transaction
      const unwrapFinalizedTx = await wrapper.connect(signers.alice).finalizeUnwrap(
        unwrapStartedEvent.args.requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof,
      );
      await unwrapFinalizedTx.wait();

      // CHECK: callback was called
      expect(await receiverMock.callbackCount()).to.equal(1n);
      expect(await receiverMock.lastUnwrapRequestId()).to.equal(unwrapStartedEvent.args.requestId);

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("should return true from finalizeUnwrap when recipient is not a contract", async function () {
      // DEPLOY & WRAP
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      // Get alice's balance
      const aliceCTokenBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

      // START unwrap to EOA (alice)
      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [signers.alice.address, signers.alice.address, "0x"]
      );

      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(cTokenAddress, signers.alice.address)
        .add64(aliceCTokenBalance)
        .encrypt();

      const unwrapTx = await cToken.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapper.getAddress(),
        encryptedUnwrapAmount.handles[0],
        encryptedUnwrapAmount.inputProof,
        data,
      );
      const unwrapReceipt = await unwrapTx.wait();

      // Get requestId from UnwrapStarted event
      const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
      expect(unwrapStartedEvents.length).to.equal(1);
      const unwrapStartedEvent = unwrapStartedEvents[0];

      // FINALIZE unwrap - use staticCall to check return value
      const publicDecryptResults = await fhevm.publicDecrypt([
        unwrapStartedEvent.args[5],
        unwrapStartedEvent.args[6],
      ]);

      // Use staticCall to check return value without executing
      const returnValue = await wrapper.connect(signers.alice).finalizeUnwrap.staticCall(
        unwrapStartedEvent.args.requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof,
      );

      // CHECK: should return true when no callback is made (EOA recipient)
      expect(returnValue).to.equal(true, "finalizeUnwrap should return true for EOA recipient");

      // Now actually execute the transaction
      await wrapper.connect(signers.alice).finalizeUnwrap(
        unwrapStartedEvent.args.requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof,
      );

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("should return false from finalizeUnwrap when unwrap fails", async function () {
      // DEPLOY & WRAP
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(1);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      // Get alice's balance
      const aliceCTokenBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

      // START unwrap with 0 amount (will fail)
      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [signers.alice.address, signers.alice.address, "0x"]
      );

      // Unwrap more than balance
      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(cTokenAddress, signers.alice.address)
        .add64(10)
        .encrypt();

      const unwrapTx = await cToken.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapper.getAddress(),
        encryptedUnwrapAmount.handles[0],
        encryptedUnwrapAmount.inputProof,
        data,
      );
      const unwrapReceipt = await unwrapTx.wait();

      // Get requestId from UnwrapStarted event
      const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
      expect(unwrapStartedEvents.length).to.equal(1);
      const unwrapStartedEvent = unwrapStartedEvents[0];

      // FINALIZE unwrap - use staticCall to check return value
      const publicDecryptResults = await fhevm.publicDecrypt([
        unwrapStartedEvent.args[5],
        unwrapStartedEvent.args[6],
      ]);

      // Use staticCall to check return value without executing
      const returnValue = await wrapper.connect(signers.alice).finalizeUnwrap.staticCall(
        unwrapStartedEvent.args.requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof,
      );

      // CHECK: should return false when unwrap fails
      expect(returnValue).to.equal(false, "finalizeUnwrap should return false when unwrap fails");

      // Now actually execute the transaction
      const unwrapFinalizedTx = await wrapper.connect(signers.alice).finalizeUnwrap(
        unwrapStartedEvent.args.requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof,
      );
      const finalizeReceipt = await unwrapFinalizedTx.wait();

      // CHECK: UnwrapFinalized event shows failure
      const unwrapFinalizedEvents = getUnwrapFinalizedEvent(finalizeReceipt);
      expect(unwrapFinalizedEvents.length).to.equal(1);
      expect(unwrapFinalizedEvents[0].args[1]).to.equal(false, "unwrap should fail");

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });
  });

  describe("Finalize Unwrap Permissions", function () {
    it("should allow unwrap initiator to finalize their own unwrap", async function () {
      // This test validates existing behavior still works - initiator can finalize
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      const aliceCUsdcBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

      // Alice initiates and finalizes unwrap (standard flow)
      const { unwrapFinalizedReceipt } = await unwrapToken(
        wrapper,
        signers.alice.address,
        aliceCUsdcBalance,
        signers.alice
      );

      // CHECK: Unwrap succeeded
      const unwrapFinalizedEvents = getUnwrapFinalizedEvent(unwrapFinalizedReceipt);
      expect(unwrapFinalizedEvents.length).to.equal(1);
      expect(unwrapFinalizedEvents[0].args[1]).to.equal(true, "unwrap should succeed");

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("should reject finalization by unauthorized address", async function () {
      // Alice initiates unwrap, Bob (unauthorized) tries to finalize
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      const aliceCUsdcBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

      // START unwrap (alice initiates)
      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [signers.alice.address, signers.alice.address, "0x"]
      );

      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(cTokenAddress, signers.alice.address)
        .add64(aliceCUsdcBalance)
        .encrypt();

      const unwrapTx = await cToken.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapper.getAddress(),
        encryptedUnwrapAmount.handles[0],
        encryptedUnwrapAmount.inputProof,
        data,
      );
      const unwrapReceipt = await unwrapTx.wait();

      // Get requestId from UnwrapStarted event
      const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
      const requestId = unwrapStartedEvents[0].args[1];

      // Get decryption proofs
      const publicDecryptResults = await fhevm.publicDecrypt([
        unwrapStartedEvents[0].args[5],
        unwrapStartedEvents[0].args[6],
      ]);

      // TRY to finalize with bob (unauthorized)
      await expect(
        wrapper.connect(signers.bob).finalizeUnwrap(
          requestId,
          publicDecryptResults.abiEncodedClearValues,
          publicDecryptResults.decryptionProof,
        )
      ).to.be.revertedWithCustomError(wrapper, "UnauthorizedFinalizeUnwrapCaller")
        .withArgs(requestId, signers.bob.address, signers.alice.address);

      // Now alice (authorized) finalizes to complete the unwrap and maintain backing invariant
      await wrapper.connect(signers.alice).finalizeUnwrap(
        requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof,
      );

      // VERIFY: Wrapper backing invariant (after finalization)
      await verifyWrapperBacking(wrapper);
    });

    it("should allow authorized operator to finalize unwrap", async function () {
      // Alice sets bob as operator, then bob finalizes alice's unwrap
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      // Alice sets bob as operator (valid until far future)
      const futureTimestamp = Math.floor(Date.now() / 1000) + 86400; // +24 hours
      const setOperatorTx = await wrapper.connect(signers.alice).setFinalizeUnwrapOperator(
        signers.bob.address,
        futureTimestamp
      );
      const setOperatorReceipt = await setOperatorTx.wait();

      // CHECK: FinalizeUnwrapOperatorSet event
      const iface = new ethers.Interface(["event FinalizeUnwrapOperatorSet(address indexed holder, address indexed operator, uint48 until)"]);
      const events = setOperatorReceipt?.logs
        .map(log => {
          try {
            return iface.parseLog({ topics: log.topics as string[], data: log.data });
          } catch {
            return null;
          }
        })
        .filter(e => e !== null);
      expect(events?.length).to.equal(1);
      expect(events?.[0]?.args[0]).to.equal(signers.alice.address);
      expect(events?.[0]?.args[1]).to.equal(signers.bob.address);
      expect(events?.[0]?.args[2]).to.equal(futureTimestamp);

      // CHECK: isFinalizeUnwrapOperator returns true
      expect(await wrapper.isFinalizeUnwrapOperator(signers.alice.address, signers.bob.address)).to.equal(true);

      const aliceCUsdcBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

      // START unwrap (alice initiates)
      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [signers.alice.address, signers.alice.address, "0x"]
      );

      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(cTokenAddress, signers.alice.address)
        .add64(aliceCUsdcBalance)
        .encrypt();

      const unwrapTx = await cToken.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapper.getAddress(),
        encryptedUnwrapAmount.handles[0],
        encryptedUnwrapAmount.inputProof,
        data,
      );
      const unwrapReceipt = await unwrapTx.wait();

      // Get requestId and decryption proofs
      const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
      const requestId = unwrapStartedEvents[0].args[1];
      const publicDecryptResults = await fhevm.publicDecrypt([
        unwrapStartedEvents[0].args[5],
        unwrapStartedEvents[0].args[6],
      ]);

      // Bob (authorized operator) finalizes alice's unwrap - should succeed
      const finalizeReceipt = await (await wrapper.connect(signers.bob).finalizeUnwrap(
        requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof,
      )).wait();

      // CHECK: Unwrap succeeded
      const unwrapFinalizedEvents = getUnwrapFinalizedEvent(finalizeReceipt);
      expect(unwrapFinalizedEvents.length).to.equal(1);
      expect(unwrapFinalizedEvents[0].args[1]).to.equal(true, "unwrap should succeed");

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });

    it("should reject finalization by expired operator", async function () {
      // Alice sets bob as operator with past expiration, bob tries to finalize
      const { cToken, cTokenAddress, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapAmount = BigInt(100000);
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      // Alice sets bob as operator (already expired)
      const pastTimestamp = Math.floor(Date.now() / 1000) - 86400; // -24 hours
      await wrapper.connect(signers.alice).setFinalizeUnwrapOperator(
        signers.bob.address,
        pastTimestamp
      );

      // CHECK: isFinalizeUnwrapOperator returns false
      expect(await wrapper.isFinalizeUnwrapOperator(signers.alice.address, signers.bob.address)).to.equal(false);

      const aliceCUsdcBalance = await getEncryptedBalance(cToken, signers.alice, cTokenAddress);

      // START unwrap (alice initiates)
      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [signers.alice.address, signers.alice.address, "0x"]
      );

      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(cTokenAddress, signers.alice.address)
        .add64(aliceCUsdcBalance)
        .encrypt();

      const unwrapTx = await cToken.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapper.getAddress(),
        encryptedUnwrapAmount.handles[0],
        encryptedUnwrapAmount.inputProof,
        data,
      );
      const unwrapReceipt = await unwrapTx.wait();

      // Get requestId and decryption proofs
      const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
      const requestId = unwrapStartedEvents[0].args[1];
      const publicDecryptResults = await fhevm.publicDecrypt([
        unwrapStartedEvents[0].args[5],
        unwrapStartedEvents[0].args[6],
      ]);

      // Bob (expired operator) tries to finalize - should fail
      await expect(
        wrapper.connect(signers.bob).finalizeUnwrap(
          requestId,
          publicDecryptResults.abiEncodedClearValues,
          publicDecryptResults.decryptionProof,
        )
      ).to.be.revertedWithCustomError(wrapper, "UnauthorizedFinalizeUnwrapCaller")
        .withArgs(requestId, signers.bob.address, signers.alice.address);

      // Now alice (authorized) finalizes to complete the unwrap and maintain backing invariant
      await wrapper.connect(signers.alice).finalizeUnwrap(
        requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof,
      );

      // VERIFY: Wrapper backing invariant (after finalization)
      await verifyWrapperBacking(wrapper);
    });

    it("should allow operator to be updated and revoked", async function () {
      const { wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      // Set bob as operator
      const futureTimestamp1 = Math.floor(Date.now() / 1000) + 86400;
      await wrapper.connect(signers.alice).setFinalizeUnwrapOperator(
        signers.bob.address,
        futureTimestamp1
      );
      expect(await wrapper.isFinalizeUnwrapOperator(signers.alice.address, signers.bob.address)).to.equal(true);

      // Update bob's expiration (extend)
      const futureTimestamp2 = Math.floor(Date.now() / 1000) + 172800; // +48 hours
      await wrapper.connect(signers.alice).setFinalizeUnwrapOperator(
        signers.bob.address,
        futureTimestamp2
      );
      expect(await wrapper.isFinalizeUnwrapOperator(signers.alice.address, signers.bob.address)).to.equal(true);

      // Revoke bob immediately (set to 0)
      await wrapper.connect(signers.alice).setFinalizeUnwrapOperator(
        signers.bob.address,
        0
      );
      expect(await wrapper.isFinalizeUnwrapOperator(signers.alice.address, signers.bob.address)).to.equal(false);

      // Set multiple operators
      const futureTimestamp3 = Math.floor(Date.now() / 1000) + 86400;
      await wrapper.connect(signers.alice).setFinalizeUnwrapOperator(signers.bob.address, futureTimestamp3);
      await wrapper.connect(signers.alice).setFinalizeUnwrapOperator(signers.royalties.address, futureTimestamp3);
      expect(await wrapper.isFinalizeUnwrapOperator(signers.alice.address, signers.bob.address)).to.equal(true);
      expect(await wrapper.isFinalizeUnwrapOperator(signers.alice.address, signers.royalties.address)).to.equal(true);
    });

    it("should handle holder setting themselves as operator (redundant but allowed)", async function () {
      const { wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      // Alice can set herself as operator (redundant)
      const futureTimestamp = Math.floor(Date.now() / 1000) + 86400;
      await wrapper.connect(signers.alice).setFinalizeUnwrapOperator(
        signers.alice.address,
        futureTimestamp
      );

      // isFinalizeUnwrapOperator returns true (because holder == operator)
      expect(await wrapper.isFinalizeUnwrapOperator(signers.alice.address, signers.alice.address)).to.equal(true);

      // Even with timestamp 0, holder can always finalize their own
      await wrapper.connect(signers.alice).setFinalizeUnwrapOperator(signers.alice.address, 0);
      expect(await wrapper.isFinalizeUnwrapOperator(signers.alice.address, signers.alice.address)).to.equal(true);
    });

    it("should handle ETH unwrap with operator finalization", async function () {
      // Test operator can finalize ETH unwraps too
      const { cEth, cEthAddress, wrapper } = await deployConfidentialETH(coordinator, signers.alice);
      const wrapAmount = ethers.parseEther("1.0");
      await wrapETH(coordinator, wrapAmount, signers.alice.address, signers.alice);

      // Alice sets bob as operator
      const futureTimestamp = Math.floor(Date.now() / 1000) + 86400;
      await wrapper.connect(signers.alice).setFinalizeUnwrapOperator(signers.bob.address, futureTimestamp);

      const aliceCEthBalance = await getEncryptedBalance(cEth, signers.alice, cEthAddress);

      // START unwrap (alice initiates)
      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [signers.alice.address, signers.alice.address, "0x"]
      );

      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(cEthAddress, signers.alice.address)
        .add64(aliceCEthBalance)
        .encrypt();

      const unwrapTx = await cEth.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapper.getAddress(),
        encryptedUnwrapAmount.handles[0],
        encryptedUnwrapAmount.inputProof,
        data,
      );
      const unwrapReceipt = await unwrapTx.wait();

      // Get requestId and decryption proofs
      const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
      const requestId = unwrapStartedEvents[0].args[1];
      const publicDecryptResults = await fhevm.publicDecrypt([
        unwrapStartedEvents[0].args[5],
        unwrapStartedEvents[0].args[6],
      ]);

      // Bob finalizes alice's ETH unwrap
      const finalizeReceipt = await (await wrapper.connect(signers.bob).finalizeUnwrap(
        requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof,
      )).wait();

      // CHECK: Unwrap succeeded
      const unwrapFinalizedEvents = getUnwrapFinalizedEvent(finalizeReceipt);
      expect(unwrapFinalizedEvents.length).to.equal(1);
      expect(unwrapFinalizedEvents[0].args[1]).to.equal(true, "unwrap should succeed");

      // VERIFY: Wrapper backing invariant
      await verifyWrapperBacking(wrapper);
    });
  });
});
