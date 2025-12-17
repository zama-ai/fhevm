import { ethers, fhevm } from "hardhat";
import { expect } from "chai";
import { FhevmType } from "@fhevm/hardhat-plugin";
import {
  deployConfidentialToken,
  wrapERC20,
  getConfidentialBalance,
  getUnwrapFee,
  getUnwrapStartedEvent,
  getUnwrapFinalizedEvent,
} from "./utils";
import { getSigners, Signers } from "./signers";
import type { TestERC20, AdminProvider, DeploymentCoordinator, FeeManager, ERC20FailOnAddressMock } from "../types";
import { deployWrapperFixture, deployTestERC20Fixture, deployTestUnsafeERC20Fixture } from "./fixtures";

async function deployFixture(signers: Signers) {
  const { coordinator, adminProvider } = await deployWrapperFixture(signers);

  const usdc = await deployTestERC20Fixture("USDC", 6);
  await usdc.mint(signers.alice, ethers.parseUnits("100000", 6));

  // Deploy a mock that can be configured to fail transfers
  const rejectMock = await deployTestUnsafeERC20Fixture("ERC20FailOnAddressMock", "REJECT", 6);

  return { coordinator, adminProvider, usdc, rejectMock };
}

describe("Wrapper Refund Address Tests", function () {
  let signers: Signers;
  let coordinator: DeploymentCoordinator;
  let adminProvider: AdminProvider;
  let usdc: TestERC20;
  let rejectMock: ERC20FailOnAddressMock;
  let feeManager: FeeManager;

  before(async function () {
    signers = await getSigners();
  });

  beforeEach(async function () {
    ({ coordinator, adminProvider, usdc, rejectMock } = await deployFixture(signers));

    const feeManagerAddress = await adminProvider.feeManager();
    feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
  });

  describe("Unwrap with different to/refund addresses - Transfer fails", function () {
    it("should mint refund to `refund` address when `to` rejects tokens", async function () {
      // Use rejectMock as the underlying token so we can control transfer failures
      await rejectMock.mint(signers.alice, ethers.parseUnits("100000", 6));
      const { cToken, wrapper } = await deployConfidentialToken(coordinator, rejectMock, signers.alice);

      // Wrap tokens for alice using rejectMock
      const wrapAmount = 100_000n;
      await wrapERC20(coordinator, rejectMock, wrapAmount, signers.alice.address, signers.alice);

      const aliceCTokenBalanceBefore = await getConfidentialBalance(cToken, signers.alice);
      // Bob starts with 0 balance (no handle yet)
      const bobCTokenBalanceBefore = 0n;
      const rate = await cToken.rate();

      // Configure rejectMock to reject transfers to itself
      const rejectAddress = await rejectMock.getAddress();
      await rejectMock.setFailOnTransferTo(rejectAddress, true);

      // Unwrap: to = rejectMock (will fail), refund = bob (should receive cTokens)
      const unwrapAmount = 50_000n;
      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(await cToken.getAddress(), signers.alice.address)
        .add64(unwrapAmount)
        .encrypt();

      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [rejectAddress, signers.bob.address, "0x"] // to=reject, refund=bob
      );

      const unwrapTx = await cToken
        .connect(signers.alice)
        ["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
          await wrapper.getAddress(),
          encryptedUnwrapAmount.handles[0],
          encryptedUnwrapAmount.inputProof,
          data
        );
      const unwrapReceipt = await unwrapTx.wait();

      // Verify event
      const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
      expect(unwrapStartedEvents.length).to.equal(1);
      const unwrapStartedEvent = unwrapStartedEvents[0];
      expect(unwrapStartedEvent.args.to).to.equal(rejectAddress);
      expect(unwrapStartedEvent.args.refund).to.equal(signers.bob.address);

      // Finalize unwrap
      const publicDecryptResults = await fhevm.publicDecrypt([
        unwrapStartedEvent.args[5],
        unwrapStartedEvent.args[6],
      ]);

      const unwrapFinalizedTx = await wrapper.connect(signers.alice).finalizeUnwrap(
        unwrapStartedEvent.args.requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof
      );
      const finalizeReceipt = await unwrapFinalizedTx.wait();

      // Verify UnwrappedFinalized event shows failure
      const finalizedEvents = getUnwrapFinalizedEvent(finalizeReceipt);
      expect(finalizedEvents.length).to.equal(1);
      expect(finalizedEvents[0].args.finalizeSuccess).to.equal(false); // Transfer failed
      expect(finalizedEvents[0].args.feeTransferSuccess).to.equal(true); // Fee succeeded

      // Verify balances
      const aliceCTokenBalanceAfter = await getConfidentialBalance(cToken, signers.alice);
      const bobCTokenBalanceAfter = await getConfidentialBalance(cToken, signers.bob);
      const rejectBalance = await rejectMock.balanceOf(rejectAddress);

      const unwrapFee = await getUnwrapFee(wrapper, unwrapAmount);

      // Alice lost unwrapAmount
      expect(aliceCTokenBalanceBefore - aliceCTokenBalanceAfter).to.equal(unwrapAmount);

      // Bob received (unwrapAmount - fee) back as cTokens
      expect(bobCTokenBalanceAfter - bobCTokenBalanceBefore).to.equal(unwrapAmount - unwrapFee);

      // rejectMock received nothing
      expect(rejectBalance).to.equal(0n);
    });

    it("should mint full amount to `refund` when both transfers fail", async function () {
      // Use rejectMock as the underlying token so we can control transfer failures
      await rejectMock.mint(signers.alice, ethers.parseUnits("100000", 6));
      const { cToken, wrapper } = await deployConfidentialToken(coordinator, rejectMock, signers.alice);

      // Wrap tokens using rejectMock
      const wrapAmount = 100_000n;
      await wrapERC20(coordinator, rejectMock, wrapAmount, signers.alice.address, signers.alice);

      const aliceCTokenBalanceBefore = await getConfidentialBalance(cToken, signers.alice);
      // Bob starts with 0 balance (no handle yet)
      const bobCTokenBalanceBefore = 0n;

      // Configure both recipient and fee recipient to reject
      const rejectAddress = await rejectMock.getAddress();
      await rejectMock.setFailOnTransferTo(rejectAddress, true);
      await rejectMock.setFailOnTransferTo(signers.royalties.address, true);

      // Unwrap: to = rejectMock (will fail), refund = bob
      const unwrapAmount = 50_000n;
      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(await cToken.getAddress(), signers.alice.address)
        .add64(unwrapAmount)
        .encrypt();

      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [rejectAddress, signers.bob.address, "0x"]
      );

      const unwrapTx = await cToken
        .connect(signers.alice)
        ["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
          await wrapper.getAddress(),
          encryptedUnwrapAmount.handles[0],
          encryptedUnwrapAmount.inputProof,
          data
        );
      const unwrapReceipt = await unwrapTx.wait();

      // Finalize
      const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
      const unwrapStartedEvent = unwrapStartedEvents[0];

      const publicDecryptResults = await fhevm.publicDecrypt([
        unwrapStartedEvent.args[5],
        unwrapStartedEvent.args[6],
      ]);

      const unwrapFinalizedTx = await wrapper.connect(signers.alice).finalizeUnwrap(
        unwrapStartedEvent.args.requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof
      );
      const finalizeReceipt = await unwrapFinalizedTx.wait();

      // Verify event shows both transfers failed
      const finalizedEvents = getUnwrapFinalizedEvent(finalizeReceipt);
      expect(finalizedEvents[0].args.finalizeSuccess).to.equal(false);
      expect(finalizedEvents[0].args.feeTransferSuccess).to.equal(false);

      // Verify bob received FULL amount back (no fees deducted)
      const aliceCTokenBalanceAfter = await getConfidentialBalance(cToken, signers.alice);
      const bobCTokenBalanceAfter = await getConfidentialBalance(cToken, signers.bob);

      expect(aliceCTokenBalanceBefore - aliceCTokenBalanceAfter).to.equal(unwrapAmount);
      expect(bobCTokenBalanceAfter - bobCTokenBalanceBefore).to.equal(unwrapAmount); // Full amount
    });
  });

  describe("Zero address validation", function () {
    it("should revert when `to` is address(0)", async function () {
      const { cToken, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      // Wrap tokens
      const wrapAmount = 100_000n;
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      const unwrapAmount = 50_000n;
      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(await cToken.getAddress(), signers.alice.address)
        .add64(unwrapAmount)
        .encrypt();

      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [ethers.ZeroAddress, signers.alice.address, "0x"] // to = zero address
      );

      await expect(
        cToken.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
          await wrapper.getAddress(),
          encryptedUnwrapAmount.handles[0],
          encryptedUnwrapAmount.inputProof,
          data
        )
      ).to.be.revertedWithCustomError(wrapper, "CannotSendToZeroAddress");
    });

    it("should revert when `refund` is address(0)", async function () {
      const { cToken, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      // Wrap tokens
      const wrapAmount = 100_000n;
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      const unwrapAmount = 50_000n;
      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(await cToken.getAddress(), signers.alice.address)
        .add64(unwrapAmount)
        .encrypt();

      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [signers.alice.address, ethers.ZeroAddress, "0x"] // refund = zero address
      );

      await expect(
        cToken.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
          await wrapper.getAddress(),
          encryptedUnwrapAmount.handles[0],
          encryptedUnwrapAmount.inputProof,
          data
        )
      ).to.be.revertedWithCustomError(wrapper, "CannotSendToZeroAddress");
    });
  });

  describe("Failed unwrap path (actualBurnAmount mismatch)", function () {
    it("should mint to refund address when actualBurnAmount != expectedBurnAmount", async function () {
      const { cToken, wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      // Wrap a small amount for alice
      const wrapAmount = 100n;
      await wrapERC20(coordinator, usdc, wrapAmount, signers.alice.address, signers.alice);

      const aliceCTokenBalanceBefore = await getConfidentialBalance(cToken, signers.alice);

      // Try to unwrap MORE than alice has - this will cause actualBurnAmount < expectedBurnAmount
      const unwrapAmount = aliceCTokenBalanceBefore + 100n;
      const encryptedUnwrapAmount = await fhevm
        .createEncryptedInput(await cToken.getAddress(), signers.alice.address)
        .add64(unwrapAmount)
        .encrypt();

      const data = ethers.AbiCoder.defaultAbiCoder().encode(
        ["address", "address", "bytes"],
        [signers.alice.address, signers.bob.address, "0x"] // refund = bob
      );

      const unwrapTx = await cToken
        .connect(signers.alice)
        ["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
          await wrapper.getAddress(),
          encryptedUnwrapAmount.handles[0],
          encryptedUnwrapAmount.inputProof,
          data
        );
      const unwrapReceipt = await unwrapTx.wait();

      // Finalize
      const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
      const unwrapStartedEvent = unwrapStartedEvents[0];

      const publicDecryptResults = await fhevm.publicDecrypt([
        unwrapStartedEvent.args[5],
        unwrapStartedEvent.args[6],
      ]);

      const unwrapFinalizedTx = await wrapper.connect(signers.alice).finalizeUnwrap(
        unwrapStartedEvent.args.requestId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof
      );
      await unwrapFinalizedTx.wait();

      // Verify: alice balance should remain the same (nothing burned)
      const aliceCTokenBalanceAfter = await getConfidentialBalance(cToken, signers.alice);
      expect(aliceCTokenBalanceAfter).to.equal(aliceCTokenBalanceBefore);

      // Bob should receive the refund, but the transfer is skipped since the actualBurnAmount was 0
      expect(await cToken.confidentialBalanceOf(signers.bob)).to.equal(
        "0x0000000000000000000000000000000000000000000000000000000000000000",
      );
    });
  });
});
