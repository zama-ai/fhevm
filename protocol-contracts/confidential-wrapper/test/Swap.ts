import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { AdminProvider, MaliciousWrapper, MaliciousWrapperAttacker, DeploymentCoordinator, FeeOnTransferERC20, RegulatedERC7984Upgradeable, SwapV0, TestERC20, UniswapV2Factory, UniswapV2Pair, UniswapV2Router02, WETH9, Wrapper } from "../types";
import { deployConfidentialErc20Fixture, deploySwapV0Fixture, deployTestERC20Fixture, deployUniswapFactoryFixture, deployWrapperFixture } from "./fixtures";
import { getSigners, Signers } from "./signers";
import { ethers, fhevm } from "hardhat";
import { deployConfidentialToken, getConfidentialBalance, getFeeManager, getSwapEvent, getSwapStartedEvent, getTxHashes, getUnwrapFee, getUnwrapFinalizedEvent, wrapERC20, finalizeUnwrapFromReceipt } from "./utils";
import { expect } from "chai";


async function addLiquidity(
  deployer: HardhatEthersSigner,
  router: UniswapV2Router02,
  tokenA: TestERC20,
  amountA: number,
  tokenB: TestERC20,
  amountB: number,
) {
  await tokenA.connect(deployer).approve(router.target, 100_000_000);
  await tokenB.connect(deployer).approve(router.target, 100_000_000);
  await router.connect(deployer).addLiquidity(
    tokenA.target,
    tokenB.target,
    amountA,
    amountB,
    0,
    0,
    deployer,
    Math.floor(Date.now() / 1000) + 6000
  );

  const factoryAddress = await router.factory();
  const factory = await ethers.getContractAt("UniswapV2Factory", factoryAddress);
  const pairAddress = await factory.getPair(tokenA.target, tokenB.target);
  return await ethers.getContractAt("UniswapV2Pair", pairAddress);
}


async function getSwapFinalizedReceipt(transferReceipt: any, wrapper: Wrapper, signer: HardhatEthersSigner) {
    return await finalizeUnwrapFromReceipt(transferReceipt, wrapper, signer);
}


describe("Uniswap V2 Swap Test", function () {
  let signers: Signers;
  let tokenA: TestERC20;
  let coordinator: DeploymentCoordinator;
  let cTokenA: RegulatedERC7984Upgradeable;
  let wrapperA: Wrapper;
  let tokenB: TestERC20;
  let cTokenB: RegulatedERC7984Upgradeable;
  let wrapperB: Wrapper;
  let router: UniswapV2Router02;
  let factory: UniswapV2Factory;
  let weth: WETH9;
  let pair: UniswapV2Pair;
  let swapV0: SwapV0;
  let adminProvider: AdminProvider;

  beforeEach(async function () {
    signers = await getSigners();
    ({ router, factory, weth } = await deployUniswapFactoryFixture(signers.admin));

    tokenA = await deployTestERC20Fixture("TOK_A", 6);
    tokenB = await deployTestERC20Fixture("TOK_B", 6);
    await tokenA.mint(signers.alice, 100_000_000_000);
    await tokenB.mint(signers.alice, 100_000_000_000);

    pair = await addLiquidity(signers.alice, router, tokenA, 100_000_000, tokenB, 100_000_000);

    ({ coordinator, adminProvider } = await deployWrapperFixture(signers));
    ({ cToken: cTokenA, wrapper: wrapperA } = await deployConfidentialToken(coordinator, tokenA, signers.alice));
    ({ cToken: cTokenB, wrapper: wrapperB } = await deployConfidentialToken(coordinator, tokenB, signers.alice));

    await wrapERC20(coordinator, tokenA, BigInt(100_000), signers.alice.address, signers.alice);
    await wrapERC20(coordinator, tokenB, BigInt(100_000), signers.alice.address, signers.alice);

    ({ swapV0 } = await deploySwapV0Fixture(coordinator));

    // Whitelist the router for all tests (except the specific test that checks non-whitelisted behavior)
    await swapV0.addRouterToWhitelist(await router.getAddress());

    // Whitelist the tokens for all tests (except the specific tests that check non-whitelisted behavior)
    await swapV0.addTokenToWhitelist(await tokenA.getAddress());
    await swapV0.addTokenToWhitelist(await tokenB.getAddress());
  });

  it("should swap cTokenA for cTokenB", async function () {
    const balanceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
    const balanceCTokenBBefore = await getConfidentialBalance(cTokenB, signers.alice);

    const transferAmount = BigInt(1_000);
    const encryptedTransferAmount = await fhevm
      .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
      .add64(transferAmount)
      .encrypt();

    const path = [await tokenA.getAddress(), await tokenB.getAddress()];

    const feeManager = await getFeeManager(coordinator);
    const unwrapFee = await getUnwrapFee(wrapperA, transferAmount);

    const swapInAmount = transferAmount - unwrapFee;
    const swapOutAmounts = await router.getAmountsOut(swapInAmount, path);
    const swapOutAmount = swapOutAmounts[swapOutAmounts.length - 1];

    const wrapFee = await feeManager.getWrapFee(swapOutAmount, ethers.ZeroAddress, ethers.ZeroAddress);
    const amountCTokenBOut = swapOutAmount - wrapFee;

    // Get the next transaction IDs before the swap for event verification
    const expectedWrapTxId = await cTokenB.nextTxId();

    const abiCoder = new ethers.AbiCoder();
    const callbackData = abiCoder.encode(
      ["tuple(address, uint256, address[], uint256, address)"],
      [[
        await router.getAddress(),
        0,
        path,
        Math.floor(Date.now() / 1000) + 6000,
        signers.alice.address,
      ]]
    )
    const data = abiCoder.encode(
      ["address", "address", "bytes"],
      [
        await swapV0.getAddress(),
        signers.alice.address,
        callbackData,
      ]
    )

    const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
      await wrapperA.getAddress(),
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
      data,
    )
    const transferReceipt = await transferTx.wait();

    const finalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);

    const unwrapFinalizedEvents = getUnwrapFinalizedEvent(finalizedReceipt);
    expect(unwrapFinalizedEvents).to.have.length(1);

    // Verify Swap event was emitted with correct parameters
    const swapEvents = getSwapEvent(finalizedReceipt);
    expect(swapEvents).to.have.length(1);
    const swapEvent = swapEvents[0];
    expect(swapEvent.args.success).to.equal(true);
    expect(swapEvent.args.path).to.deep.equal(path);
    expect(swapEvent.args.unwrapTxId).to.equal(unwrapFinalizedEvents[0].args.requestId);
    expect(swapEvent.args.wrapTxId).to.equal(expectedWrapTxId);
    expect(swapEvent.args.errorReasonString).to.equal("");
    expect(swapEvent.args.errorLowLevelData).to.equal("0x");

    const balanceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
    const balanceCTokenBAfter = await getConfidentialBalance(cTokenB, signers.alice);

    expect(balanceCTokenABefore - balanceCTokenAAfter).to.equal(transferAmount);
    expect(balanceCTokenBAfter - balanceCTokenBBefore).to.equal(amountCTokenBOut);
  });

  it("should return cTokenA when swapping fails", async function () {
    const balanceAliceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
    const balanceBobCTokenABefore = 0n;
    const balanceCTokenBBefore = await getConfidentialBalance(cTokenB, signers.alice);

    const transferAmount = BigInt(1_000);
    const encryptedTransferAmount = await fhevm
      .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
      .add64(transferAmount)
      .encrypt();

    const path = [await tokenA.getAddress(), await tokenB.getAddress()];

    const feeManager = await getFeeManager(coordinator);
    const unwrapFee = await getUnwrapFee(wrapperA, transferAmount);

    const swapInAmount = transferAmount - unwrapFee;

    const wrapFee = await feeManager.getWrapFee(swapInAmount, ethers.ZeroAddress, ethers.ZeroAddress);

    const abiCoder = new ethers.AbiCoder();
    const callbackData = abiCoder.encode(
      ["tuple(address, uint256, address[], uint256, address)"],
      [[
        await router.getAddress(),
        0,
        path,
        Math.floor(Date.now() / 1000) - 1,
        signers.alice.address,
      ]]
    )
    const data = abiCoder.encode(
      ["address", "address", "bytes"],
      [
        await swapV0.getAddress(),
        signers.bob.address,
        callbackData,
      ]
    )

    const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
      await wrapperA.getAddress(),
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
      data,
    )
    const transferReceipt = await transferTx.wait();

    // Bob finalizes the unwrap (not Alice) - this tests that refunds go to the finalizer
    const finalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);

    const unwrapFinalizedEvents = getUnwrapFinalizedEvent(finalizedReceipt);
    expect(unwrapFinalizedEvents).to.have.length(1);

    // Verify Swap event was emitted with failure parameters
    const swapEvents = getSwapEvent(finalizedReceipt);
    expect(swapEvents).to.have.length(1);
    const swapEvent = swapEvents[0];
    expect(swapEvent.args.success).to.equal(false);
    expect(swapEvent.args.path).to.deep.equal(path);
    expect(swapEvent.args.unwrapTxId).to.equal(unwrapFinalizedEvents[0].args.requestId);
    expect(swapEvent.args.wrapTxId).to.equal(5); // wrap transaction for reimbursement
    expect(swapEvent.args.errorReasonString).to.equal("UniswapV2Router: EXPIRED");
    expect(swapEvent.args.errorLowLevelData).to.equal("0x");

    const balanceAliceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
    const balanceBobCTokenAAfter = await getConfidentialBalance(cTokenA, signers.bob);
    const balanceCTokenBAfter = await getConfidentialBalance(cTokenB, signers.alice);

    // Alice's balance decreases by the transfer amount (she initiated the swap)
    expect(balanceAliceCTokenABefore - balanceAliceCTokenAAfter).to.equal(transferAmount);

    // Bob receives the refund minus fees (he finalized the unwrap, so refund goes to him)
    expect(balanceBobCTokenAAfter - balanceBobCTokenABefore).to.equal(transferAmount - unwrapFee - wrapFee);

    // No cTokenB received (swap failed)
    expect(balanceCTokenBAfter - balanceCTokenBBefore).to.equal(0);
  });

  it("should swap TokenA for TokenB", async function () {
    const amountIn = 10_000_000;

    await tokenA.connect(signers.alice).approve(router.target, amountIn);

    const path = [tokenA.target, tokenB.target];
    const deadline = Math.floor(Date.now() / 1000) + 6000;

    const beforeBalance = await tokenB.balanceOf(signers.alice);

    await router.connect(signers.alice).swapExactTokensForTokens(
      amountIn,
      0,
      path,
      signers.alice,
      deadline
    );

    const afterBalance = await tokenB.balanceOf(signers.alice);
  });

  it("should not charge wrap/unwrap fees for SWAPPER role when waiver is active", async function () {
    const balanceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
    const balanceCTokenBBefore = await getConfidentialBalance(cTokenB, signers.alice);

    const feeManager = await getFeeManager(coordinator);
    const swapperRole = await feeManager.SWAPPER_ROLE();

    // Grant SWAPPER role to the SwapV0 contract
    await feeManager.grantRole(swapperRole, await swapV0.getAddress());

    // Activate swapper fee waiver
    await feeManager.setSwapperFeeWaiverActive(true);

    // Set non-zero fees to verify they are waived
    await feeManager.setWrapFeeBasisPoints(100); // 1%
    await feeManager.setUnwrapFeeBasisPoints(100); // 1%

    const transferAmount = BigInt(10_000);
    const encryptedTransferAmount = await fhevm
      .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
      .add64(transferAmount)
      .encrypt();

    const path = [await tokenA.getAddress(), await tokenB.getAddress()];

    // Calculate expected amounts without fees since SwapV0 has SWAPPER role
    // Since SwapV0 has SWAPPER role, full transfer amount goes to swap (no unwrap fee)
    const swapInAmountWithFeeWaiver = transferAmount;
    const swapOutAmountsWithFeeWaiver = await router.getAmountsOut(swapInAmountWithFeeWaiver, path);
    const swapOutAmountWithFeeWaiver = swapOutAmountsWithFeeWaiver[swapOutAmountsWithFeeWaiver.length - 1];
    const expectedAmountCTokenBOut = swapOutAmountWithFeeWaiver; // No wrap fee either

    const abiCoder = new ethers.AbiCoder();
    const callbackData = abiCoder.encode(
      ["tuple(address, uint256, address[], uint256, address)"],
      [[
        await router.getAddress(),
        0,
        path,
        Math.floor(Date.now() / 1000) + 6000,
        signers.alice.address,
      ]]
    );
    const data = abiCoder.encode(
      ["address", "address", "bytes"],
      [
        await swapV0.getAddress(),
        signers.alice.address,
        callbackData,
      ]
    );

    const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
      await wrapperA.getAddress(),
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
      data,
    );
    const transferReceipt = await transferTx.wait();

    await finalizeUnwrapFromReceipt(transferReceipt, wrapperA, signers.alice);

    const balanceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
    const balanceCTokenBAfter = await getConfidentialBalance(cTokenB, signers.alice);

    // Verify full transfer amount was used (no unwrap fee)
    expect(balanceCTokenABefore - balanceCTokenAAfter).to.equal(transferAmount);

    // Verify full transfer amount was used (no unwrap fee) and full swap output was received (no wrap fee)
    expect(balanceCTokenBAfter - balanceCTokenBBefore).to.equal(expectedAmountCTokenBOut);

    // Verify that the FeeManager correctly returns 0 fees for the SwapV0 contract
    const wrapFee = await feeManager.getWrapFee(10000, await swapV0.getAddress(), ethers.ZeroAddress);
    expect(wrapFee).to.equal(0);

    // Verify that SwapV0 has the SWAPPER role and waiver is active
    expect(await feeManager.hasRole(swapperRole, await swapV0.getAddress())).to.be.true;
    expect(await feeManager.swapperFeeWaiverActive()).to.be.true;
  });

  describe("forceApprove with USDT-like tokens", function () {
    it("should handle forceApprove for router approval (line 75) with USDT-like token", async function () {
      // Deploy USDT-like tokenA that requires 0-approval before changing
      const USDTMockFactory = await ethers.getContractFactory("ERC20USDTApprovalMock");
      const usdtLikeTokenA = await USDTMockFactory.deploy("USDT-like TokenA", "USDTA", 6);
      await usdtLikeTokenA.waitForDeployment();
      await usdtLikeTokenA.mint(signers.alice, 100_000_000_000);

      // Add liquidity with USDT-like token
      await addLiquidity(signers.alice, router, usdtLikeTokenA, 100_000_000, tokenB, 100_000_000);

      // Deploy wrapper for USDT-like token
      const { cToken: cUsdtLikeTokenA, wrapper: cUsdtLikeWrapperA } = await deployConfidentialToken(coordinator, usdtLikeTokenA, signers.alice);
      await wrapERC20(coordinator, usdtLikeTokenA, BigInt(100_000), signers.alice.address, signers.alice);

      // Use test helper to set non-zero approval from SwapV0 to router
      // This simulates a scenario where SwapV0 already has approval set
      await usdtLikeTokenA.testSetApproval(await swapV0.getAddress(), await router.getAddress(), 1000);

      // Verify the approval is set
      const approvalBefore = await usdtLikeTokenA.allowance(await swapV0.getAddress(), await router.getAddress());
      expect(approvalBefore).to.equal(1000);

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cUsdtLikeTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await usdtLikeTokenA.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) + 6000,
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.alice.address,
          callbackData,
        ]
      );

      const transferTx = await cUsdtLikeTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await cUsdtLikeWrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      // Should succeed despite existing approval (forceApprove handles it)
      await finalizeUnwrapFromReceipt(transferReceipt, cUsdtLikeWrapperA, signers.alice);
    });

    it("should handle forceApprove for wrapper approval after successful swap (line 105) with USDT-like tokenOut", async function () {
      // Deploy USDT-like tokenB as output
      const USDTMockFactory = await ethers.getContractFactory("ERC20USDTApprovalMock");
      const usdtLikeTokenB = await USDTMockFactory.deploy("USDT-like TokenB", "USDTB", 6);
      await usdtLikeTokenB.waitForDeployment();
      await usdtLikeTokenB.mint(signers.alice, 100_000_000_000);

      // Add liquidity
      await addLiquidity(signers.alice, router, tokenA, 100_000_000, usdtLikeTokenB, 100_000_000);

      // Deploy wrapper for USDT-like tokenB
      const { wrapper: wrapperUsdtB } = await deployConfidentialToken(coordinator, usdtLikeTokenB, signers.alice);

      // Use test helper to set non-zero approval from SwapV0 to wrapperUsdtB
      // This simulates a scenario where SwapV0 already has approval set
      await usdtLikeTokenB.testSetApproval(await swapV0.getAddress(), await wrapperUsdtB.getAddress(), 1000);

      // Verify the approval is set
      const approvalBefore = await usdtLikeTokenB.allowance(await swapV0.getAddress(), await wrapperUsdtB.getAddress());
      expect(approvalBefore).to.equal(1000);

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await tokenA.getAddress(), await usdtLikeTokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) + 6000,
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.alice.address,
          callbackData,
        ]
      );

      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      // Should succeed - forceApprove handles the non-zero approval by resetting to 0 first
      // Without forceApprove, this would revert with ApprovalFromNonZeroToNonZero error
      await finalizeUnwrapFromReceipt(transferReceipt, wrapperA, signers.alice);
    });

    it("should handle forceApprove for wrapper approval after failed swap (line 116) with USDT-like tokenIn", async function () {
      // Deploy USDT-like tokenA
      const USDTMockFactory = await ethers.getContractFactory("ERC20USDTApprovalMock");
      const usdtLikeTokenA = await USDTMockFactory.deploy("USDT-like TokenA", "USDTA", 6);
      await usdtLikeTokenA.waitForDeployment();
      await usdtLikeTokenA.mint(signers.alice, 100_000_000_000);

      // Add liquidity
      await addLiquidity(signers.alice, router, usdtLikeTokenA, 100_000_000, tokenB, 100_000_000);

      // Deploy wrapper for USDT-like token
      const { cToken: cUsdtLikeTokenA, wrapper: wrapperUsdtA } = await deployConfidentialToken(coordinator, usdtLikeTokenA, signers.alice);
      await wrapERC20(coordinator, usdtLikeTokenA, BigInt(100_000), signers.alice.address, signers.alice);

      // Use test helper to set non-zero approval from SwapV0 to wrapperUsdtA
      // This simulates a scenario where SwapV0 already has approval set
      await usdtLikeTokenA.testSetApproval(await swapV0.getAddress(), await wrapperUsdtA.getAddress(), 1000);

      // Verify the approval is set
      const approvalBefore = await usdtLikeTokenA.allowance(await swapV0.getAddress(), await wrapperUsdtA.getAddress());
      expect(approvalBefore).to.equal(1000);

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cUsdtLikeTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await usdtLikeTokenA.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) - 1, // Expired deadline to force swap failure
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.alice.address,
          callbackData,
        ]
      );

      const transferTx = await cUsdtLikeTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperUsdtA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      // Should succeed - forceApprove handles the non-zero approval by resetting to 0 first
      // Without forceApprove, this would revert with ApprovalFromNonZeroToNonZero error
      await finalizeUnwrapFromReceipt(transferReceipt, wrapperUsdtA, signers.alice);
    });
  });

  describe("Path Validation", function () {
    it("should reject swap when cToken.underlying() does not match path[0]", async function () {
      const feeManager = await getFeeManager(coordinator);
      const swapperRole = await feeManager.SWAPPER_ROLE();
      await feeManager.grantRole(swapperRole, await swapV0.getAddress());
      await feeManager.setSwapperFeeWaiverActive(true);

      const balanceAliceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceBobCTokenABefore = 0n;
      const balanceCTokenBBefore = await getConfidentialBalance(cTokenB, signers.alice);

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Create path that starts with tokenB instead of tokenA (cTokenA.underlying())
      const invalidPath = [await tokenB.getAddress(), await tokenA.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          invalidPath,
          Math.floor(Date.now() / 1000) + 6000,
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.bob.address,
          callbackData,
        ]
      );

      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      // Bob finalizes the unwrap - refund should go to Bob
      const swapFinalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);
      const swapEvents = getSwapEvent(swapFinalizedReceipt);

      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);
      expect(swapEvents[0].args.errorReasonString).to.equal("");
      expect(swapEvents[0].args.errorLowLevelData).to.equal(await swapV0.INPUT_PATH_IS_NOT_UNDERLYING());

      // Verify balances: Alice loses the transfer, Bob gets refunded
      const balanceAliceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceBobCTokenAAfter = await getConfidentialBalance(cTokenA, signers.bob);
      const balanceCTokenBAfter = await getConfidentialBalance(cTokenB, signers.alice);

      // Alice's balance decreased (she initiated the swap)
      expect(balanceAliceCTokenABefore - balanceAliceCTokenAAfter).to.equal(transferAmount);

      // Bob receives the refund (he finalized, so refund goes to him)
      expect(balanceBobCTokenAAfter - balanceBobCTokenABefore).to.equal(transferAmount);

      expect(balanceCTokenBAfter).to.equal(balanceCTokenBBefore);
    });

    it("should reject swap when no coordinator wrapper exists for input token (path[0])", async function () {
      // create an alternate coordinator that is unknown to the swapper
      // we'll deploy a tokenC on this alternat coordinator so that all interfaces match
      // and we can call the swapper. Any swap from cTokenC with a path starting at tokenC
      // should thus fail.
      const { coordinator: coordinatorAlt } = await deployWrapperFixture(signers);

      const feeManager = await getFeeManager(coordinatorAlt);
      const swapperRole = await feeManager.SWAPPER_ROLE();
      await feeManager.grantRole(swapperRole, await swapV0.getAddress());
      await feeManager.setSwapperFeeWaiverActive(true);


      // deploy tokenC wrapper in different coordinator than that of swap
      const tokenC = await deployTestERC20Fixture("TOK_C", 6);
      await tokenC.mint(signers.alice, 100_000_000_000);
      const {cToken: cTokenC, wrapper: wrapperC} = await deployConfidentialToken(coordinatorAlt, tokenC, signers.alice);
      await wrapERC20(coordinatorAlt, tokenC, BigInt(100_000_000), signers.alice.address, signers.alice);

      // Whitelist tokenC so we can test the wrapper validation (not token whitelist validation)
      await swapV0.addTokenToWhitelist(await tokenC.getAddress());

      // Add liquidity for tokenC -> tokenB swap
      await addLiquidity(signers.alice, router, tokenC, 100_000_000, tokenB, 100_000_000);

      const balanceAliceCTokenCBefore = await getConfidentialBalance(cTokenC, signers.alice);
      const balanceBobCTokenCBefore = 0n;

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenC.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Create path with tokenC (no wrapper at coordinator level) as input
      const invalidPath = [await tokenC.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          invalidPath,
          Math.floor(Date.now() / 1000) + 6000,
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.bob.address,
          callbackData,
        ]
      );

      // Try to swap cTokenC with path that starts with tokenC
      // This should fail because cTokenC does not have a wrapper with
      // the configured coordinator
      const transferTx = await cTokenC.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperC.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      // Bob finalizes the unwrap - refund should go to Bob
      await expect(getSwapFinalizedReceipt(transferReceipt, wrapperC, signers.alice)).to.be.revertedWithCustomError(swapV0, "UnknownWrapper");
    });

    it("should reject swap when no wrapper exists for output token (path[path.length-1])", async function () {
      const feeManager = await getFeeManager(coordinator);
      const swapperRole = await feeManager.SWAPPER_ROLE();
      await feeManager.grantRole(swapperRole, await swapV0.getAddress());
      await feeManager.setSwapperFeeWaiverActive(true);

      // Deploy a new token without creating a wrapper for it
      const tokenC = await deployTestERC20Fixture("TOK_C", 6);
      await tokenC.mint(signers.alice, 100_000_000_000);

      // deploy a confidential token without a wrapper. Athough path[-1] matches the cToken's
      // underlying, the lack of wrapper will make the swap fail and refund.
      const { cErc20: cTokenC } = await deployConfidentialErc20Fixture(signers, adminProvider, tokenC);
      await cTokenC.mint(signers.alice, 100_000_000_000);

      // Whitelist tokenC so we can test the wrapper validation (not token whitelist validation)
      await swapV0.addTokenToWhitelist(await tokenC.getAddress());

      // Add liquidity for tokenA -> tokenC swap
      await addLiquidity(signers.alice, router, tokenA, 100_000_000, tokenC, 100_000_000);

      const balanceAliceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceBobCTokenABefore = 0n;

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Create path with tokenC (no wrapper) as output
      const invalidPath = [await tokenA.getAddress(), await tokenC.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          invalidPath,
          Math.floor(Date.now() / 1000) + 6000,
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.bob.address,
          callbackData,
        ]
      );

      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      // Bob finalizes the unwrap - refund should go to Bob
      const swapFinalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);
      const swapEvents = getSwapEvent(swapFinalizedReceipt);

      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);
      expect(swapEvents[0].args.errorReasonString).to.equal("");
      expect(swapEvents[0].args.errorLowLevelData).to.equal(await swapV0.OUTPUT_PATH_HAS_NO_WRAPPER());

      // Verify balances: Alice loses the transfer, Bob gets refunded
      const balanceAliceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceBobCTokenAAfter = await getConfidentialBalance(cTokenA, signers.bob);

      // Alice's balance decreased
      expect(balanceAliceCTokenABefore - balanceAliceCTokenAAfter).to.equal(transferAmount);

      // Bob receives the refund
      expect(balanceBobCTokenAAfter - balanceBobCTokenABefore).to.equal(transferAmount);
    });

    for (let createPool of [true, false]) {
      it("should reject swap when path does not exist at Uniswap router level", async function () {
        const feeManager = await getFeeManager(coordinator);
        const swapperRole = await feeManager.SWAPPER_ROLE();
        await feeManager.grantRole(swapperRole, await swapV0.getAddress());
        await feeManager.setSwapperFeeWaiverActive(true);

        // Deploy a new token and create wrapper but don't add liquidity
        const tokenC = await deployTestERC20Fixture("TOK_C", 6);
        await tokenC.mint(signers.alice, 100_000_000_000);

        // Create wrapper for tokenC so it passes the wrapper checks
        const { cToken: cTokenC } = await deployConfidentialToken(coordinator, tokenC, signers.alice);

        // Whitelist tokenC so we can test the Uniswap path validation (not token whitelist validation)
        await swapV0.addTokenToWhitelist(await tokenC.getAddress());

        const balanceAliceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
        const balanceBobCTokenABefore = 0n;

        const transferAmount = BigInt(1_000);
        const encryptedTransferAmount = await fhevm
          .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
          .add64(transferAmount)
          .encrypt();

        await router.factory()

        if (createPool) {
          const factory = await ethers.getContractAt("UniswapV2Factory", await router.factory()) as UniswapV2Factory;
          await factory.createPair(await tokenA.getAddress(), await tokenC.getAddress());
        }

        // Create path tokenA -> tokenC (no liquidity, so getAmountsOut will fail)
        const invalidPath = [await tokenA.getAddress(), await tokenC.getAddress()];

        const abiCoder = new ethers.AbiCoder();
        const callbackData = abiCoder.encode(
          ["tuple(address, uint256, address[], uint256, address)"],
          [[
            await router.getAddress(),
            0,
            invalidPath,
            Math.floor(Date.now() / 1000) + 6000,
            signers.alice.address,
          ]]
        );
        const data = abiCoder.encode(
          ["address", "address", "bytes"],
          [
            await swapV0.getAddress(),
            signers.bob.address,
            callbackData,
          ]
        );

        const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
          await wrapperA.getAddress(),
          encryptedTransferAmount.handles[0],
          encryptedTransferAmount.inputProof,
          data,
        );
        const receipt = await transferTx.wait();

        // Bob finalizes the unwrap - refund should go to Bob
        const swapFinalizedReceipt = await getSwapFinalizedReceipt(receipt, wrapperA, signers.alice);
        const swapEvents = getSwapEvent(swapFinalizedReceipt);

        expect(swapEvents).to.have.length(1);
        expect(swapEvents[0].args.success).to.equal(false);
        if (createPool) {
          expect(swapEvents[0].args.errorReasonString).to.equal("UniswapV2Library: INSUFFICIENT_LIQUIDITY");
        } else {
          expect(swapEvents[0].args.errorReasonString).to.equal("");
        }
        expect(swapEvents[0].args.errorLowLevelData).to.equal("0x");

        // Verify balances: Alice loses the transfer, Bob gets refunded
        const balanceAliceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
        const balanceBobCTokenAAfter = await getConfidentialBalance(cTokenA, signers.bob);

        // Alice's balance decreased
        expect(balanceAliceCTokenABefore - balanceAliceCTokenAAfter).to.equal(transferAmount);

        // Bob receives the refund
        expect(balanceBobCTokenAAfter - balanceBobCTokenABefore).to.equal(transferAmount);
      });
    }

    it("should expose checkPath as a public view function", async function () {
      // Simple test to verify checkPath is accessible as a public view
      const path = [await tokenA.getAddress(), await tokenB.getAddress()];

      // Call checkPath - should not revert and return expected structure
      const [isValid, errorString, errorData] = await swapV0.checkPath(
        wrapperA,
        await router.getAddress(),
        path
      );

      expect(isValid).to.equal(true);
      expect(errorString).to.equal("");
      expect(errorData).to.equal("0x");
    });

    it("should handle valid path that fails getAmountsOut(1) due to rounding", async function () {
      const feeManager = await getFeeManager(coordinator);
      const swapperRole = await feeManager.SWAPPER_ROLE();
      await feeManager.grantRole(swapperRole, await swapV0.getAddress());
      await feeManager.setSwapperFeeWaiverActive(true);

      // Deploy tokenC and create a pool with very low liquidity
      const tokenC = await deployTestERC20Fixture("TOK_C", 6);
      await tokenC.mint(signers.alice, 100_000_000_000);

      // Create wrapper for tokenC
      const { cToken: cTokenC, wrapper: wrapperC } = await deployConfidentialToken(coordinator, tokenC, signers.alice);

      // Whitelist tokenC so we can test the rounding behavior (not token whitelist validation)
      await swapV0.addTokenToWhitelist(await tokenC.getAddress());

      // Wrap some cTokenA for Alice to swap
      await wrapERC20(coordinator, tokenA, BigInt(100_000), signers.alice.address, signers.alice);

      // Add liquidity with very imbalanced reserves to cause rounding issues
      // Pool: 1 tokenA : 1,000,000 tokenB (tokenA is very valuable)
      await addLiquidity(signers.alice, router, tokenA, 1, tokenB, 1_000_000);

      // Add liquidity for second hop: 1,000,000 tokenB : 1,000,000 tokenC
      await addLiquidity(signers.alice, router, tokenB, 1_000_000, tokenC, 1_000_000);

      // Create a valid path: tokenA -> tokenB -> tokenC
      const path = [
        await tokenA.getAddress(),
        await tokenB.getAddress(),
        await tokenC.getAddress()
      ];

      // Verify that getAmountsOut(1) FAILS due to rounding
      await expect(
        router.getAmountsOut(1, path)
      ).to.be.reverted;

      // But verify that larger amounts work fine (proving the path is actually valid)
      const amounts = await router.getAmountsOut(1000, path);
      expect(amounts.length).to.equal(3);
      expect(amounts[0]).to.equal(1000);
      expect(amounts[2]).to.be.greaterThan(0);

      const [isValid, errorString, errorData] = await swapV0.checkPath(
        wrapperA,
        await router.getAddress(),
        path
      );

      // checkPath returns true although getAmountsOut fails
      expect(isValid).to.equal(true);

      // Now attempt an actual swap to ensure proper refund
      const balanceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCTokenCBefore = 0n;

      const transferAmount = BigInt(1);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) + 6000,
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.alice.address,
          callbackData,
        ]
      );

      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      const finalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);

      // Verify Swap event still failed
      const swapEvents = getSwapEvent(finalizedReceipt);
      expect(swapEvents).to.have.length(1);

      expect(swapEvents[0].args.success).to.equal(false);

      // Verify Alice gets refunded (swap failed)
      const balanceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCTokenCAfter = 0n;

      // Alice should have received her tokens back
      expect(balanceCTokenABefore - balanceCTokenAAfter).to.equal(0);
      expect(balanceCTokenCAfter).to.equal(balanceCTokenCBefore); // No tokenC received
    });
  });

  describe("Security: Zero Address Recipient Protection", function () {
    it("should reject swap with address(0) recipient and refund to refundTo address", async function () {
      const feeManager = await getFeeManager(coordinator);
      const swapperRole = await feeManager.SWAPPER_ROLE();
      await feeManager.grantRole(swapperRole, await swapV0.getAddress());
      await feeManager.setSwapperFeeWaiverActive(true);

      const balanceAliceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceBobCTokenABefore = 0n;

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await tokenA.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) + 6000,
          ethers.ZeroAddress, // address(0) as recipient - this is the vulnerability
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.bob.address, // Bob is the refundTo address
          callbackData,
        ]
      );

      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      // Bob finalizes the unwrap - refund should go to Bob
      const swapFinalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);
      const swapEvents = getSwapEvent(swapFinalizedReceipt);

      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);
      expect(swapEvents[0].args.errorReasonString).to.equal("");
      expect(swapEvents[0].args.errorLowLevelData).to.equal(await swapV0.RECIPIENT_CANNOT_BE_ZERO_ADDRESS());

      // Verify balances: Alice loses the transfer, Bob gets refunded
      const balanceAliceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceBobCTokenAAfter = await getConfidentialBalance(cTokenA, signers.bob);

      // Alice's balance decreased (she initiated the swap)
      expect(balanceAliceCTokenABefore - balanceAliceCTokenAAfter).to.equal(transferAmount);

      // Bob receives the refund (he is the refundTo address)
      expect(balanceBobCTokenAAfter - balanceBobCTokenABefore).to.equal(transferAmount);

      // Verify no funds stuck in SwapV0
      expect(await tokenA.balanceOf(await swapV0.getAddress())).to.equal(0);
    });
  });

  describe("Security: Router Whitelist in Swaps", function () {
    it("should reject swap when router is not whitelisted", async function () {
      const feeManager = await getFeeManager(coordinator);
      const swapperRole = await feeManager.SWAPPER_ROLE();
      await feeManager.grantRole(swapperRole, await swapV0.getAddress());
      await feeManager.setSwapperFeeWaiverActive(true);

      // Use a fake/malicious router address (not whitelisted)
      const fakeRouter = await ethers.Wallet.createRandom().getAddress();

      const balanceAliceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceBobCTokenABefore = 0n;

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await tokenA.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          fakeRouter, // Use non-whitelisted router
          0,
          path,
          Math.floor(Date.now() / 1000) + 6000,
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.bob.address,
          callbackData,
        ]
      );

      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      // Bob finalizes the unwrap - should receive refund because router not whitelisted
      const swapFinalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);
      const swapEvents = getSwapEvent(swapFinalizedReceipt);

      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);
      expect(swapEvents[0].args.errorReasonString).to.equal("");
      expect(swapEvents[0].args.errorLowLevelData).to.equal(await swapV0.ROUTER_NOT_WHITELISTED());

      // Verify balances: Alice loses the transfer, Bob gets refunded
      const balanceAliceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceBobCTokenAAfter = await getConfidentialBalance(cTokenA, signers.bob);

      // Alice's balance decreased (she initiated the swap)
      expect(balanceAliceCTokenABefore - balanceAliceCTokenAAfter).to.equal(transferAmount);

      // Bob receives the refund (he finalized, so refund goes to him)
      expect(balanceBobCTokenAAfter - balanceBobCTokenABefore).to.equal(transferAmount);
    });
  });

  describe("Security: Unauthorized Wrapper Attack", function () {
    it("should block malicious wrapper from calling onUnwrapFinalizedReceived directly", async function () {
      // Deploy the MaliciousWrapper contract that attempts to bypass fees
      const MaliciousWrapperFactory = await ethers.getContractFactory("MaliciousWrapper");
      const maliciousWrapper = await MaliciousWrapperFactory.deploy(
        await tokenA.getAddress(),
        await swapV0.getAddress()
      );
      await maliciousWrapper.waitForDeployment();

      // Set fees to demonstrate what the attack would have bypassed
      const feeManager = await getFeeManager(coordinator);
      await feeManager.setWrapFeeBasisPoints(100); // 1% wrap fee
      await feeManager.setUnwrapFeeBasisPoints(100); // 1% unwrap fee

      const swapAmount = BigInt(10_000);

      // Approve malicious wrapper to spend alice's tokens
      await tokenA.connect(signers.alice).approve(await maliciousWrapper.getAddress(), swapAmount);

      const path = [await tokenA.getAddress(), await tokenB.getAddress()];
      const deadline = Math.floor(Date.now() / 1000) + 6000;

      // Attempt the attack - should revert because SwapV0 validates msg.sender
      // The malicious wrapper tries to call onUnwrapFinalizedReceived directly to bypass unwrap fees
      // SwapV0's checkPath function verifies that msg.sender matches the legitimate wrapper
      // registered in the coordinator for the underlying token
      await expect(
        maliciousWrapper.connect(signers.alice).attemptExploit(
          swapAmount,
          await router.getAddress(),
          0, // amountOutMin
          path,
          deadline,
          signers.alice.address
        )
      ).to.be.revertedWithCustomError(swapV0, "UnknownWrapper");

      // Verify no tokens were transferred (attack was blocked before any state changes)
      expect(await tokenA.balanceOf(await maliciousWrapper.getAddress())).to.equal(0);
      expect(await tokenA.balanceOf(await swapV0.getAddress())).to.equal(0);
    });
  });

  describe("Fee-on-Transfer (FOT) Token Support", function () {
    it("should handle FOT swap fails and refunds user", async function () {
      // The issue was that SwapV0 would attempt to wrap amountIn tokens during refund,
      // but only held actualBalance < amountIn due to fee-on-transfer, causing revert and stuck funds.
      // The fix checks actual balance and only wraps what the contract actually holds.

      // Deploy FOT token with 2% transfer fee (200 basis points)
      const FeeOnTransferERC20Factory = await ethers.getContractFactory("FeeOnTransferERC20");
      const feeToken = await FeeOnTransferERC20Factory.deploy("FeeToken", "FEE", 6, 200);
      await feeToken.waitForDeployment();
      await feeToken.mint(signers.alice, 100_000_000_000);

      // Deploy confidential FOT token and wrapper
      const { cToken: cFOT, wrapper: wrapperFOT } = await deployConfidentialToken(coordinator, feeToken, signers.alice);

      // Whitelist feeToken so we can test the FOT behavior (not token whitelist validation)
      await swapV0.addTokenToWhitelist(await feeToken.getAddress());

      // Wrap FOT tokens
      await wrapERC20(coordinator, feeToken, BigInt(100_000), signers.alice.address, signers.alice);

      // Add liquidity with FOT token
      await addLiquidity(signers.alice, router, feeToken, 100_000_000, tokenB, 100_000_000);

      const balanceAliceCFOTBefore = await getConfidentialBalance(cFOT, signers.alice);
      const balanceBobCFOTBefore = 0n;
      const balanceCTokenBBefore = await getConfidentialBalance(cTokenB, signers.alice);

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cFOT.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await feeToken.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) - 1, // Expired deadline to force swap failure
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(["address", "address", "bytes"], [await swapV0.getAddress(), signers.bob.address, callbackData]);

      const transferTx = await cFOT.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperFOT.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data
      );
      const transferReceipt = await transferTx.wait();

      // KEY TEST: Bob finalizes the unwrap - refund should go to Bob (not Alice)
      // This should succeed now (with the fix) - SwapV0 checks actual balance and only wraps what it has
      // Without the fix, this would revert with ERC20InsufficientBalance
      const finalizedReceipt = await finalizeUnwrapFromReceipt(transferReceipt, wrapperFOT, signers.alice);

      // Verify Swap event shows failure
      const swapEvents = getSwapEvent(finalizedReceipt);
      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);
      expect(swapEvents[0].args.errorReasonString).to.equal("UniswapV2Router: EXPIRED");

      const balanceAliceCFOTAfter = await getConfidentialBalance(cFOT, signers.alice);
      const balanceBobCFOTAfter = await getConfidentialBalance(cFOT, signers.bob);
      const balanceCTokenBAfter = await getConfidentialBalance(cTokenB, signers.alice);

      // Alice's balance decreased (she initiated the swap)
      expect(balanceAliceCFOTBefore - balanceAliceCFOTAfter).to.equal(transferAmount);

      // Bob receives the refund (he finalized the unwrap)
      // Due to FOT fees, Bob gets less than the original transfer amount
      const bobRefund = balanceBobCFOTAfter - balanceBobCFOTBefore;
      expect(bobRefund).to.be.lessThan(transferAmount); // Bob gets less due to FOT fees
      expect(bobRefund).to.be.greaterThan(0); // Bob got a partial refund

      // Verify no cTokenB was received (swap failed)
      expect(balanceCTokenBAfter).to.equal(balanceCTokenBBefore);

      // Verify SwapV0 has no leftover tokens (all refunded to user)
      expect(await feeToken.balanceOf(await swapV0.getAddress())).to.equal(0);
    });
  });

  describe("SwapV0 Router Whitelist Management", function () {
    it("should allow owner to add router to whitelist", async function () {
      const testRouter = await ethers.Wallet.createRandom().getAddress();

      // Verify router not whitelisted initially
      expect(await swapV0.whitelistedRouters(testRouter)).to.equal(false);

      // Add router to whitelist
      await expect(swapV0.addRouterToWhitelist(testRouter))
        .to.emit(swapV0, "RouterAddedToWhitelist")
        .withArgs(testRouter);

      // Verify router is now whitelisted
      expect(await swapV0.whitelistedRouters(testRouter)).to.equal(true);
    });

    it("should allow owner to remove router from whitelist", async function () {
      const testRouter = await ethers.Wallet.createRandom().getAddress();

      // Add router first
      await swapV0.addRouterToWhitelist(testRouter);
      expect(await swapV0.whitelistedRouters(testRouter)).to.equal(true);

      // Remove router from whitelist
      await expect(swapV0.removeRouterFromWhitelist(testRouter))
        .to.emit(swapV0, "RouterRemovedFromWhitelist")
        .withArgs(testRouter);

      // Verify router is no longer whitelisted
      expect(await swapV0.whitelistedRouters(testRouter)).to.equal(false);
    });

    it("should revert when adding zero address to whitelist", async function () {
      await expect(
        swapV0.addRouterToWhitelist(ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(swapV0, "ZeroAddressRouter");
    });

    it("should revert when adding already whitelisted router", async function () {
      const testRouter = await ethers.Wallet.createRandom().getAddress();
      await swapV0.addRouterToWhitelist(testRouter);

      await expect(
        swapV0.addRouterToWhitelist(testRouter)
      ).to.be.revertedWithCustomError(swapV0, "RouterAlreadyWhitelisted");
    });

    it("should revert when removing zero address from whitelist", async function () {
      await expect(
        swapV0.removeRouterFromWhitelist(ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(swapV0, "ZeroAddressRouter");
    });

    it("should revert when removing router that is not whitelisted", async function () {
      const testRouter = await ethers.Wallet.createRandom().getAddress();

      await expect(
        swapV0.removeRouterFromWhitelist(testRouter)
      ).to.be.revertedWithCustomError(swapV0, "RouterNotWhitelisted");
    });

    it("should revert when non-owner tries to add router", async function () {
      const testRouter = await ethers.Wallet.createRandom().getAddress();

      await expect(
        swapV0.connect(signers.alice).addRouterToWhitelist(testRouter)
      ).to.be.revertedWithCustomError(swapV0, "OwnableUnauthorizedAccount");
    });

    it("should revert when non-owner tries to remove router", async function () {
      const testRouter = await ethers.Wallet.createRandom().getAddress();
      await swapV0.addRouterToWhitelist(testRouter);

      await expect(
        swapV0.connect(signers.alice).removeRouterFromWhitelist(testRouter)
      ).to.be.revertedWithCustomError(swapV0, "OwnableUnauthorizedAccount");
    });

    it("should allow adding multiple different routers", async function () {
      const router1 = await ethers.Wallet.createRandom().getAddress();
      const router2 = await ethers.Wallet.createRandom().getAddress();
      const router3 = await ethers.Wallet.createRandom().getAddress();

      await swapV0.addRouterToWhitelist(router1);
      await swapV0.addRouterToWhitelist(router2);
      await swapV0.addRouterToWhitelist(router3);

      expect(await swapV0.whitelistedRouters(router1)).to.equal(true);
      expect(await swapV0.whitelistedRouters(router2)).to.equal(true);
      expect(await swapV0.whitelistedRouters(router3)).to.equal(true);
    });

    it("should allow re-adding a router after removal", async function () {
      const testRouter = await ethers.Wallet.createRandom().getAddress();

      // Add, remove, then re-add
      await swapV0.addRouterToWhitelist(testRouter);
      await swapV0.removeRouterFromWhitelist(testRouter);
      await swapV0.addRouterToWhitelist(testRouter);

      expect(await swapV0.whitelistedRouters(testRouter)).to.equal(true);
    });

    it("should support two-step ownership transfer", async function () {
      // SwapV0 is deployed by the default signer (first from ethers.getSigners())
      const deployer = (await ethers.getSigners())[0];

      await swapV0.connect(deployer).transferOwnership(signers.alice.address);
      expect(await swapV0.pendingOwner()).to.equal(signers.alice.address);
      expect(await swapV0.owner()).to.equal(deployer.address);

      await swapV0.connect(signers.alice).acceptOwnership();
      expect(await swapV0.owner()).to.equal(signers.alice.address);
      expect(await swapV0.pendingOwner()).to.equal(ethers.ZeroAddress);
    });
  });

  describe("Sanctioned Address Handling", function () {
    it("should gracefully handle swap when recipient (to) is sanctioned - fallback to refundTo", async function () {
      const balanceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCTokenBBefore = await getConfidentialBalance(cTokenB, signers.alice);

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await tokenA.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) + 6000,
          signers.bob.address, // Bob is the recipient
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.alice.address, // Alice is refundTo
          callbackData,
        ]
      );

      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      // Get sanctions list and sanction Bob BEFORE finalization
      const adminProviderAddress = await coordinator.adminProvider();
      const adminProviderContract = await ethers.getContractAt("AdminProvider", adminProviderAddress);
      const sanctionsListAddress = await adminProviderContract.sanctionsList();
      const sanctionsList = await ethers.getContractAt("SanctionsList", sanctionsListAddress);

      await sanctionsList.addToSanctionsList([signers.bob.address]);

      // Finalize the unwrap - should succeed because of fallback to refundTo (alice)
      const finalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);

      // Verify Swap event shows success
      const swapEvents = getSwapEvent(finalizedReceipt);
      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);
      expect(swapEvents[0].args.errorReasonString).to.equal("");
      expect(swapEvents[0].args.errorLowLevelData).to.equal(await swapV0.RECIPIENT_IS_SANCTIONED());

      const balanceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCTokenBAfter = await getConfidentialBalance(cTokenB, signers.alice);

      // Alice's cTokenA balance decreased
      const feeManager = await getFeeManager(coordinator);
      const wrapFee = await feeManager.getWrapFee(transferAmount, ethers.ZeroAddress, ethers.ZeroAddress);
      const unwrapFee = await getUnwrapFee(wrapperA, transferAmount - wrapFee);
      expect(balanceCTokenABefore - balanceCTokenAAfter).to.equal(wrapFee + unwrapFee);

      // Alice received no cTokenB
      expect(balanceCTokenBBefore - balanceCTokenBAfter).to.equal(0);

      // Bob should have 0 cTokenB (sanctioned, so fallback kicked in)
      const balanceBobCTokenBAfterHandle = await cTokenB.confidentialBalanceOf(signers.bob);
      expect(Number(balanceBobCTokenBAfterHandle)).to.equal(0);
    });

    it("should gracefully handle swap when both recipient (to) and refundTo are sanctioned - fallback to feeRecipient", async function () {
      const balanceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await tokenA.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) + 6000,
          signers.bob.address, // Bob is the recipient
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.charlie.address, // Charlie is refundTo
          callbackData,
        ]
      );

      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      // Get sanctions list and sanction BOTH Bob and Charlie BEFORE finalization
      const adminProviderAddress = await coordinator.adminProvider();
      const adminProviderContract = await ethers.getContractAt("AdminProvider", adminProviderAddress);
      const sanctionsListAddress = await adminProviderContract.sanctionsList();
      const sanctionsList = await ethers.getContractAt("SanctionsList", sanctionsListAddress);

      await sanctionsList.addToSanctionsList([signers.bob.address, signers.charlie.address]);

      // Get fee recipient before swap
      const feeManager = await getFeeManager(coordinator);
      const feeRecipient = await feeManager.feeRecipient();
      const balanceFeeRecipientCTokenBBefore = 0n;

      // Finalize the unwrap - should succeed because of final fallback to feeRecipient
      const finalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);

      // Verify Swap event shows success
      const swapEvents = getSwapEvent(finalizedReceipt);
      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);
      expect(swapEvents[0].args.errorReasonString).to.equal("");
      expect(swapEvents[0].args.errorLowLevelData).to.equal(await swapV0.REFUND_ADDRESS_IS_SANCTIONED());

      const balanceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);

      // Alice's cTokenA balance decreased
      expect(balanceCTokenABefore - balanceCTokenAAfter).to.equal(transferAmount);

      // Fee recipient receives the cTokenA (final fallback)
      const balanceFeeRecipientCTokenAAfter = await getConfidentialBalance(cTokenA, feeRecipient, signers.royalties);
      const wrapFee = await feeManager.getWrapFee(transferAmount, ethers.ZeroAddress, ethers.ZeroAddress);
      const unwrapFee = await getUnwrapFee(wrapperA, transferAmount - wrapFee);
      expect(balanceFeeRecipientCTokenAAfter).to.equal(transferAmount - wrapFee - unwrapFee);

      // Bob and Charlie should have 0 cTokenB (both sanctioned)
      const balanceBobCTokenBAfterHandle = await cTokenB.confidentialBalanceOf(signers.bob);
      expect(Number(balanceBobCTokenBAfterHandle)).to.equal(0);
      const balanceCharlieCTokenBAfterHandle = await cTokenB.confidentialBalanceOf(signers.charlie);
      expect(Number(balanceCharlieCTokenBAfterHandle)).to.equal(0);
    });

    it("should prevent fund loss when recipient gets sanctioned between phase 1 and phase 2", async function () {
      // This test reproduces the vulnerability scenario from the bug report
      const balanceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await tokenA.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) + 6000,
          signers.bob.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.alice.address, // Alice is refundTo
          callbackData,
        ]
      );

      // Phase 1: confidentialTransferAndCall (burns tokens, stores callback data)
      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      // Sanction Bob AFTER phase 1 but BEFORE phase 2
      const adminProviderAddress = await coordinator.adminProvider();
      const adminProviderContract = await ethers.getContractAt("AdminProvider", adminProviderAddress);
      const sanctionsListAddress = await adminProviderContract.sanctionsList();
      const sanctionsList = await ethers.getContractAt("SanctionsList", sanctionsListAddress);

      await sanctionsList.addToSanctionsList([signers.bob.address]);

      // Phase 2: finalizeUnwrap (should NOT revert thanks to the fix)
      const finalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);

      // Verify swap succeeded with fallback to refundTo
      const swapEvents = getSwapEvent(finalizedReceipt);
      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);

      const balanceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCTokenBAfterAlice = await getConfidentialBalance(cTokenB, signers.alice);
      const balanceCTokenBAfterBob = await cTokenB.confidentialBalanceOf(signers.bob);

      // Alice's cTokenA decreased
      const feeManager = await getFeeManager(coordinator);
      const wrapFee = await feeManager.getWrapFee(transferAmount, ethers.ZeroAddress, ethers.ZeroAddress);
      const unwrapFee = await getUnwrapFee(wrapperA, transferAmount - wrapFee);
      expect(balanceCTokenABefore - balanceCTokenAAfter).to.equal(wrapFee + unwrapFee);

      // Bob receives nothing (sanctioned)
      expect(Number(balanceCTokenBAfterBob)).to.equal(0);
    });
  });

  describe("Token Whitelist", function () {
    it("should allow owner to add token to whitelist", async function () {
      // Create a new token that's not whitelisted in beforeEach
      const tokenC = await deployTestERC20Fixture("TOK_C", 6);
      const tokenAddress = await tokenC.getAddress();

      // Verify token is not whitelisted initially
      expect(await swapV0.whitelistedTokens(tokenAddress)).to.be.false;

      // Add token to whitelist
      const tx = await swapV0.addTokenToWhitelist(tokenAddress);
      const receipt = await tx.wait();

      // Verify event was emitted
      const events = receipt!.logs
        .map((log: any) => {
          try {
            return swapV0.interface.parseLog({ topics: log.topics, data: log.data });
          } catch {
            return null;
          }
        })
        .filter((parsed) => parsed?.name === "TokenAddedToWhitelist");

      expect(events).to.have.length(1);
      expect(events[0]!.args.token).to.equal(tokenAddress);

      // Verify token is now whitelisted
      expect(await swapV0.whitelistedTokens(tokenAddress)).to.be.true;
    });

    it("should allow owner to remove token from whitelist", async function () {
      // Use tokenA which is already whitelisted in beforeEach
      const tokenAddress = await tokenA.getAddress();
      expect(await swapV0.whitelistedTokens(tokenAddress)).to.be.true;

      // Remove token from whitelist
      const tx = await swapV0.removeTokenFromWhitelist(tokenAddress);
      const receipt = await tx.wait();

      // Verify event was emitted
      const events = receipt!.logs
        .map((log: any) => {
          try {
            return swapV0.interface.parseLog({ topics: log.topics, data: log.data });
          } catch {
            return null;
          }
        })
        .filter((parsed) => parsed?.name === "TokenRemovedFromWhitelist");

      expect(events).to.have.length(1);
      expect(events[0]!.args.token).to.equal(tokenAddress);

      // Verify token is no longer whitelisted
      expect(await swapV0.whitelistedTokens(tokenAddress)).to.be.false;
    });

    it("should revert when adding zero address to whitelist", async function () {
      await expect(
        swapV0.addTokenToWhitelist(ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(swapV0, "ZeroAddressToken");
    });

    it("should revert when adding already whitelisted token", async function () {
      // Use tokenA which is already whitelisted in beforeEach
      const tokenAddress = await tokenA.getAddress();

      await expect(
        swapV0.addTokenToWhitelist(tokenAddress)
      ).to.be.revertedWithCustomError(swapV0, "TokenAlreadyWhitelisted");
    });

    it("should revert when removing zero address from whitelist", async function () {
      await expect(
        swapV0.removeTokenFromWhitelist(ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(swapV0, "ZeroAddressToken");
    });

    it("should revert when removing non-whitelisted token", async function () {
      // Create a new token that's not whitelisted
      const tokenC = await deployTestERC20Fixture("TOK_C", 6);
      const tokenAddress = await tokenC.getAddress();

      await expect(
        swapV0.removeTokenFromWhitelist(tokenAddress)
      ).to.be.revertedWithCustomError(swapV0, "TokenNotWhitelisted");
    });

    it("should revert when non-owner tries to add token to whitelist", async function () {
      // Create a new token that's not whitelisted
      const tokenC = await deployTestERC20Fixture("TOK_C", 6);
      const tokenAddress = await tokenC.getAddress();

      await expect(
        swapV0.connect(signers.alice).addTokenToWhitelist(tokenAddress)
      ).to.be.revertedWithCustomError(swapV0, "OwnableUnauthorizedAccount");
    });

    it("should revert when non-owner tries to remove token from whitelist", async function () {
      // Use tokenA which is already whitelisted in beforeEach
      const tokenAddress = await tokenA.getAddress();

      await expect(
        swapV0.connect(signers.alice).removeTokenFromWhitelist(tokenAddress)
      ).to.be.revertedWithCustomError(swapV0, "OwnableUnauthorizedAccount");
    });

    it("should reject swap when tokens in path are not whitelisted", async function () {
      // Remove tokens from whitelist (they were added in beforeEach)
      await swapV0.removeTokenFromWhitelist(await tokenA.getAddress());
      await swapV0.removeTokenFromWhitelist(await tokenB.getAddress());

      const balanceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCTokenBBefore = await getConfidentialBalance(cTokenB, signers.alice);

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await tokenA.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) + 6000,
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.alice.address,
          callbackData,
        ]
      );

      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      const swapFinalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);
      const swapEvents = getSwapEvent(swapFinalizedReceipt);

      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);
      expect(swapEvents[0].args.errorReasonString).to.equal("");
      expect(swapEvents[0].args.errorLowLevelData).to.equal(await swapV0.TOKEN_NOT_WHITELISTED());

      // Verify user was refunded (balance unchanged after fees)
      const balanceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCTokenBAfter = await getConfidentialBalance(cTokenB, signers.alice);

      // Alice loses transferAmount but gets refund minus fees
      expect(balanceCTokenABefore - balanceCTokenAAfter).to.be.lessThan(transferAmount);
      expect(balanceCTokenBAfter).to.equal(balanceCTokenBBefore);
    });

    it("should reject swap when only first token in path is whitelisted", async function () {
      // Remove tokens from whitelist (they were added in beforeEach), then whitelist only tokenA
      await swapV0.removeTokenFromWhitelist(await tokenA.getAddress());
      await swapV0.removeTokenFromWhitelist(await tokenB.getAddress());
      await swapV0.addTokenToWhitelist(await tokenA.getAddress());

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await tokenA.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) + 6000,
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.alice.address,
          callbackData,
        ]
      );

      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      const swapFinalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);
      const swapEvents = getSwapEvent(swapFinalizedReceipt);

      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);
      expect(swapEvents[0].args.errorLowLevelData).to.equal(await swapV0.TOKEN_NOT_WHITELISTED());
    });

    it("should reject swap when only last token in path is whitelisted", async function () {
      // Remove tokens from whitelist (they were added in beforeEach), then whitelist only tokenB
      await swapV0.removeTokenFromWhitelist(await tokenA.getAddress());
      await swapV0.removeTokenFromWhitelist(await tokenB.getAddress());
      await swapV0.addTokenToWhitelist(await tokenB.getAddress());

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await tokenA.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) + 6000,
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.alice.address,
          callbackData,
        ]
      );

      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      const swapFinalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);
      const swapEvents = getSwapEvent(swapFinalizedReceipt);

      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);
      expect(swapEvents[0].args.errorLowLevelData).to.equal(await swapV0.TOKEN_NOT_WHITELISTED());
    });

    it("should reject swap with multi-hop path when middle token is not whitelisted", async function () {
      // Deploy a third token for multi-hop path
      const tokenC = await deployTestERC20Fixture("TOK_C", 6);
      await tokenC.mint(signers.alice, 100_000_000_000);

      // Create liquidity pools: A-C and C-B
      await addLiquidity(signers.alice, router, tokenA, 100_000_000, tokenC, 100_000_000);
      await addLiquidity(signers.alice, router, tokenC, 100_000_000, tokenB, 100_000_000);

      const { cToken: cTokenC, wrapper: wrapperC } = await deployConfidentialToken(coordinator, tokenC, signers.alice);

      // TokenA and tokenB are already whitelisted from beforeEach, just don't whitelist tokenC

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Multi-hop path: A -> C -> B (C is not whitelisted)
      const path = [await tokenA.getAddress(), await tokenC.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) + 6000,
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.alice.address,
          callbackData,
        ]
      );

      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      const swapFinalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);
      const swapEvents = getSwapEvent(swapFinalizedReceipt);

      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);
      expect(swapEvents[0].args.errorLowLevelData).to.equal(await swapV0.TOKEN_NOT_WHITELISTED());
    });

    it("should allow swap when all tokens in path are whitelisted", async function () {
      // Both tokens are already whitelisted in beforeEach

      const balanceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCTokenBBefore = await getConfidentialBalance(cTokenB, signers.alice);

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await tokenA.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) + 6000,
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.alice.address,
          callbackData,
        ]
      );

      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      const swapFinalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);
      const swapEvents = getSwapEvent(swapFinalizedReceipt);

      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(true);

      // Verify balances changed (swap succeeded)
      const balanceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCTokenBAfter = await getConfidentialBalance(cTokenB, signers.alice);

      expect(balanceCTokenABefore - balanceCTokenAAfter).to.equal(transferAmount);
      expect(balanceCTokenBAfter).to.be.greaterThan(balanceCTokenBBefore);
    });

    it("should allow multi-hop swap when all tokens in path are whitelisted", async function () {
      // Deploy a third token for multi-hop path
      const tokenC = await deployTestERC20Fixture("TOK_C", 6);
      await tokenC.mint(signers.alice, 100_000_000_000);

      // Create liquidity pools: A-C and C-B
      await addLiquidity(signers.alice, router, tokenA, 100_000_000, tokenC, 100_000_000);
      await addLiquidity(signers.alice, router, tokenC, 100_000_000, tokenB, 100_000_000);

      const { cToken: cTokenC, wrapper: wrapperC } = await deployConfidentialToken(coordinator, tokenC, signers.alice);

      // Whitelist tokenC (tokenA and tokenB are already whitelisted in beforeEach)
      await swapV0.addTokenToWhitelist(await tokenC.getAddress());

      const balanceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCTokenBBefore = await getConfidentialBalance(cTokenB, signers.alice);

      const transferAmount = BigInt(1_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Multi-hop path: A -> C -> B
      const path = [await tokenA.getAddress(), await tokenC.getAddress(), await tokenB.getAddress()];

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) + 6000,
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.alice.address,
          callbackData,
        ]
      );

      const transferTx = await cTokenA.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperA.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      const swapFinalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);
      const swapEvents = getSwapEvent(swapFinalizedReceipt);

      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(true);

      // Verify balances changed (swap succeeded)
      const balanceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCTokenBAfter = await getConfidentialBalance(cTokenB, signers.alice);

      expect(balanceCTokenABefore - balanceCTokenAAfter).to.equal(transferAmount);
      expect(balanceCTokenBAfter).to.be.greaterThan(balanceCTokenBBefore);
    });

    describe.skip("Attack Prevention", function () {
      it("should prevent fee bypass attack with fake token (Example 1: avoid wrap fees)", async function () {
        // Bob deploys a fake token and creates pools
        const fakeToken = await deployTestERC20Fixture("FAKE", 6);
        await fakeToken.mint(signers.bob, 100_000_000_000);

        // Create pools: USDC<>fake and WETH<>fake with Bob controlling liquidity
        const tokenUSDC = tokenA; // reuse tokenA as USDC
        const tokenWETH = tokenB; // reuse tokenB as WETH

        // Mint tokens to Bob so he can provide liquidity
        await tokenUSDC.mint(signers.bob, 100_000_000_000);
        await tokenWETH.mint(signers.bob, 100_000_000_000);

        await addLiquidity(signers.bob, router, tokenUSDC, 1, fakeToken, 1_000_000);
        await addLiquidity(signers.bob, router, fakeToken, 1_000_000, tokenWETH, 10_000_000); // favorable rate for attacker

        // Whitelist legitimate tokens but NOT fake token
        await swapV0.addTokenToWhitelist(await tokenUSDC.getAddress());
        await swapV0.addTokenToWhitelist(await tokenWETH.getAddress());
        // fake token is NOT whitelisted

        // Bob wraps 1 wei of USDC
        await wrapERC20(coordinator, tokenUSDC, BigInt(1), signers.bob.address, signers.bob);

        const transferAmount = BigInt(1);
        const encryptedTransferAmount = await fhevm
          .createEncryptedInput(await cTokenA.getAddress(), signers.bob.address)
          .add64(transferAmount)
          .encrypt();

        // Bob tries to use path [USDC, fake, WETH] to swap 1 wei USDC -> 10 WETH
        const path = [await tokenUSDC.getAddress(), await fakeToken.getAddress(), await tokenWETH.getAddress()];

        const abiCoder = new ethers.AbiCoder();
        const callbackData = abiCoder.encode(
          ["tuple(address, uint256, address[], uint256, address)"],
          [[
            await router.getAddress(),
            0,
            path,
            Math.floor(Date.now() / 1000) + 6000,
            signers.bob.address,
          ]]
        );
        const data = abiCoder.encode(
          ["address", "address", "bytes"],
          [
            await swapV0.getAddress(),
            signers.bob.address,
            callbackData,
          ]
        );

        const transferTx = await cTokenA.connect(signers.bob)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
          await wrapperA.getAddress(),
          encryptedTransferAmount.handles[0],
          encryptedTransferAmount.inputProof,
          data,
        );
        const transferReceipt = await transferTx.wait();

        const swapFinalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.bob);
        const swapEvents = getSwapEvent(swapFinalizedReceipt);

        // Verify swap was rejected due to fake token not being whitelisted
        expect(swapEvents).to.have.length(1);
        expect(swapEvents[0].args.success).to.equal(false);
        expect(swapEvents[0].args.errorLowLevelData).to.equal(await swapV0.TOKEN_NOT_WHITELISTED());
      });

      it("should prevent fee bypass attack with fake token (Example 2: avoid unwrap fees)", async function () {
        // Bob deploys a fake token
        const fakeToken = await deployTestERC20Fixture("FAKE", 6);
        await fakeToken.mint(signers.bob, 100_000_000_000);

        // Create pools: WETH<>fake and fake<>USDC with Bob controlling liquidity
        const tokenWETH = tokenA; // reuse tokenA as WETH
        const tokenUSDC = tokenB; // reuse tokenB as USDC

        // Mint tokens to Bob so he can provide liquidity
        await tokenWETH.mint(signers.bob, 100_000_000_000);
        await tokenUSDC.mint(signers.bob, 100_000_000_000);

        await addLiquidity(signers.bob, router, tokenWETH, 10_000_000, fakeToken, 1_000_000);
        await addLiquidity(signers.bob, router, fakeToken, 1_000_000, tokenUSDC, 1); // 1 wei USDC out

        // Whitelist legitimate tokens but NOT fake token
        await swapV0.addTokenToWhitelist(await tokenWETH.getAddress());
        await swapV0.addTokenToWhitelist(await tokenUSDC.getAddress());
        // fake token is NOT whitelisted

        // Bob wraps 10 WETH
        await wrapERC20(coordinator, tokenWETH, BigInt(10_000_000), signers.bob.address, signers.bob);

        const transferAmount = BigInt(10_000_000);
        const encryptedTransferAmount = await fhevm
          .createEncryptedInput(await cTokenA.getAddress(), signers.bob.address)
          .add64(transferAmount)
          .encrypt();

        // Bob tries to use path [WETH, fake, USDC] to swap 10 WETH -> 1 wei USDC
        // This would let him unwrap 10 WETH with reduced unwrap fee, then recover WETH from liquidity
        const path = [await tokenWETH.getAddress(), await fakeToken.getAddress(), await tokenUSDC.getAddress()];

        const abiCoder = new ethers.AbiCoder();
        const callbackData = abiCoder.encode(
          ["tuple(address, uint256, address[], uint256, address)"],
          [[
            await router.getAddress(),
            0,
            path,
            Math.floor(Date.now() / 1000) + 6000,
            signers.bob.address,
          ]]
        );
        const data = abiCoder.encode(
          ["address", "address", "bytes"],
          [
            await swapV0.getAddress(),
            signers.bob.address,
            callbackData,
          ]
        );

        const transferTx = await cTokenA.connect(signers.bob)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
          await wrapperA.getAddress(),
          encryptedTransferAmount.handles[0],
          encryptedTransferAmount.inputProof,
          data,
        );
        const transferReceipt = await transferTx.wait();

        const swapFinalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.bob);
        const swapEvents = getSwapEvent(swapFinalizedReceipt);

        // Verify swap was rejected due to fake token not being whitelisted
        expect(swapEvents).to.have.length(1);
        expect(swapEvents[0].args.success).to.equal(false);
        expect(swapEvents[0].args.errorLowLevelData).to.equal(await swapV0.TOKEN_NOT_WHITELISTED());
      });
    });
  });

  describe("Security - Malicious Wrapper Attack Prevention", function () {
    it("should revert when malicious contract attempts to call onUnwrapFinalizedReceived directly", async function () {
      // This test verifies the fix for the vulnerability where an attacker could
      // call onUnwrapFinalizedReceived with a malicious contract to steal tokens

      // Setup: Send some tokenA to SwapV0 (simulating leftover tokens)
      const swapV0Balance = BigInt(50_000);
      await tokenA.connect(signers.alice).transfer(await swapV0.getAddress(), swapV0Balance);

      // Verify SwapV0 has tokens
      const balanceBefore = await tokenA.balanceOf(await swapV0.getAddress());
      expect(balanceBefore).to.equal(swapV0Balance);

      // Deploy malicious wrapper attacker
      const MaliciousWrapperAttackerFactory = await ethers.getContractFactory("MaliciousWrapperAttacker");
      const maliciousWrapper = await MaliciousWrapperAttackerFactory.connect(signers.bob).deploy(
        await tokenA.getAddress(),
        await swapV0.getAddress()
      );

      // Prepare attack parameters
      // Path with tokenB as first element (doesn't match tokenA from originalToken())
      // This would trigger checkPath to return false BEFORE wrapper verification in vulnerable code
      const path = [await tokenB.getAddress(), await tokenA.getAddress()];
      const amountIn = BigInt(10_000);

      // Attempt the attack - should revert with UnknownWrapper error
      // The fix ensures wrapper validation happens at the START of checkPath
      await expect(
        maliciousWrapper.connect(signers.bob).executeAttack(
          amountIn,
          await router.getAddress(),
          path
        )
      ).to.be.revertedWithCustomError(swapV0, "UnknownWrapper");

      // Verify no tokens were stolen
      const swapV0BalanceAfter = await tokenA.balanceOf(await swapV0.getAddress());
      expect(swapV0BalanceAfter).to.equal(swapV0Balance);

      const stolenAmount = await maliciousWrapper.stolenAmount();
      expect(stolenAmount).to.equal(0);
    });
  });

  describe("Governance - Rescue Tokens", function () {
    it("should allow owner to rescue stuck ERC20 tokens", async function () {
      // Get the current owner
      const ownerAddress = await swapV0.owner();
      const owner = await ethers.getSigner(ownerAddress);

      // Send some tokens to SwapV0 (simulating stuck tokens)
      const stuckAmount = BigInt(100_000);
      await tokenA.connect(signers.alice).transfer(await swapV0.getAddress(), stuckAmount);

      // Verify SwapV0 has tokens
      const swapV0BalanceBefore = await tokenA.balanceOf(await swapV0.getAddress());
      expect(swapV0BalanceBefore).to.equal(stuckAmount);

      // Owner rescues tokens to alice
      const aliceBalanceBefore = await tokenA.balanceOf(signers.alice.address);
      const tx = await swapV0.connect(owner).rescueTokens(
        await tokenA.getAddress(),
        signers.alice.address,
        stuckAmount
      );

      // Verify event was emitted
      await expect(tx)
        .to.emit(swapV0, "TokensRescued")
        .withArgs(await tokenA.getAddress(), signers.alice.address, stuckAmount);

      // Verify tokens were transferred
      const swapV0BalanceAfter = await tokenA.balanceOf(await swapV0.getAddress());
      expect(swapV0BalanceAfter).to.equal(0);

      const aliceBalanceAfter = await tokenA.balanceOf(signers.alice.address);
      expect(aliceBalanceAfter - aliceBalanceBefore).to.equal(stuckAmount);
    });

    it("should allow owner to rescue stuck ETH", async function () {
      // Get the current owner
      const ownerAddress = await swapV0.owner();
      const owner = await ethers.getSigner(ownerAddress);

      // Send some ETH to SwapV0 (simulating stuck ETH)
      const stuckAmount = ethers.parseEther("1.0");
      await signers.alice.sendTransaction({
        to: await swapV0.getAddress(),
        value: stuckAmount,
      });

      // Verify SwapV0 has ETH
      const swapV0BalanceBefore = await ethers.provider.getBalance(await swapV0.getAddress());
      expect(swapV0BalanceBefore).to.be.gte(stuckAmount);

      // Owner rescues ETH to alice
      const aliceBalanceBefore = await ethers.provider.getBalance(signers.alice.address);
      const tx = await swapV0.connect(owner).rescueTokens(
        ethers.ZeroAddress,
        signers.alice.address,
        stuckAmount
      );

      // Verify event was emitted
      await expect(tx)
        .to.emit(swapV0, "TokensRescued")
        .withArgs(ethers.ZeroAddress, signers.alice.address, stuckAmount);

      // Verify ETH was transferred
      const swapV0BalanceAfter = await ethers.provider.getBalance(await swapV0.getAddress());
      expect(swapV0BalanceBefore - swapV0BalanceAfter).to.equal(stuckAmount);

      const aliceBalanceAfter = await ethers.provider.getBalance(signers.alice.address);
      // Alice balance should increase by stuckAmount
      expect(aliceBalanceAfter - aliceBalanceBefore).to.equal(stuckAmount);
    });

    it("should revert when non-owner tries to rescue tokens", async function () {
      // Send tokens to SwapV0
      await tokenA.connect(signers.alice).transfer(await swapV0.getAddress(), BigInt(10_000));

      // Non-owner (bob) tries to rescue tokens - should revert
      await expect(
        swapV0.connect(signers.bob).rescueTokens(
          await tokenA.getAddress(),
          signers.bob.address,
          BigInt(10_000)
        )
      ).to.be.revertedWithCustomError(swapV0, "OwnableUnauthorizedAccount");
    });

    it("should revert when rescuing to zero address", async function () {
      // Get the current owner
      const ownerAddress = await swapV0.owner();
      const owner = await ethers.getSigner(ownerAddress);

      // Send tokens to SwapV0
      await tokenA.connect(signers.alice).transfer(await swapV0.getAddress(), BigInt(10_000));

      // Try to rescue to zero address - should revert
      await expect(
        swapV0.connect(owner).rescueTokens(
          await tokenA.getAddress(),
          ethers.ZeroAddress,
          BigInt(10_000)
        )
      ).to.be.revertedWithCustomError(swapV0, "ZeroAddressRecipient");
    });

    it("should allow rescuing partial amount", async function () {
      // Get the current owner
      const ownerAddress = await swapV0.owner();
      const owner = await ethers.getSigner(ownerAddress);

      // Send tokens to SwapV0
      const stuckAmount = BigInt(100_000);
      await tokenA.connect(signers.alice).transfer(await swapV0.getAddress(), stuckAmount);

      // Rescue only half
      const rescueAmount = stuckAmount / BigInt(2);
      await swapV0.connect(owner).rescueTokens(
        await tokenA.getAddress(),
        signers.alice.address,
        rescueAmount
      );

      // Verify partial rescue
      const swapV0BalanceAfter = await tokenA.balanceOf(await swapV0.getAddress());
      expect(swapV0BalanceAfter).to.equal(stuckAmount - rescueAmount);
    });
  });
});
