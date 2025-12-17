import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { AdminProvider, DeploymentCoordinator, RegulatedERC7984Upgradeable, SwapV0, TestERC20, UniswapV2Factory, UniswapV2Pair, UniswapV2Router02, WETH9, WrapperUpgradeable } from "../types";
import { deploySwapV0Fixture, deployTestERC20Fixture, deployUniswapFactoryFixture, deployWrapperFixture } from "./fixtures";
import { getSigners, Signers } from "./signers";
import { ethers, fhevm } from "hardhat";
import { deployConfidentialToken, deployConfidentialETH, getConfidentialBalance, getFeeManager, getSwapEvent, getUnwrapFee, getUnwrapFinalizedEvent, getUnwrapStartedEvent, wrapERC20, wrapETH, finalizeUnwrapFromReceipt } from "./utils";
import { expect } from "chai";


async function addLiquidityETH(
  deployer: HardhatEthersSigner,
  router: UniswapV2Router02,
  token: TestERC20,
  amountToken: number | bigint,
  amountETH: number | bigint,
) {
  await token.connect(deployer).approve(router.target, 100_000_000_000);
  await router.connect(deployer).addLiquidityETH(
    token.target,
    amountToken,
    0,
    0,
    deployer,
    Math.floor(Date.now() / 1000) + 6000,
    { value: amountETH }
  );

  const factoryAddress = await router.factory();
  const factory = await ethers.getContractAt("UniswapV2Factory", factoryAddress);
  const wethAddress = await router.WETH();
  const pairAddress = await factory.getPair(token.target, wethAddress);
  return await ethers.getContractAt("UniswapV2Pair", pairAddress);
}

async function getSwapFinalizedReceipt(transferReceipt: any, wrapper: WrapperUpgradeable, signer: HardhatEthersSigner) {
    return await finalizeUnwrapFromReceipt(transferReceipt, wrapper, signer);
}


async function getCTokensFromPath(coordinator: DeploymentCoordinator, router: UniswapV2Router02, path: string[]) {
    const tokenInAddress = path[0] === await router.WETH() ? ethers.ZeroAddress : path[0];
    const cTokenInAddress = await coordinator.deployedConfidentialTokens(tokenInAddress);
    const cTokenIn = await ethers.getContractAt("RegulatedERC7984Upgradeable", cTokenInAddress);

    const tokenOutAddress = path[path.length - 1] === await router.WETH() ? ethers.ZeroAddress : path[path.length - 1];
    const cTokenOutAddress = await coordinator.deployedConfidentialTokens(tokenOutAddress);
    const cTokenOut = await ethers.getContractAt("RegulatedERC7984Upgradeable", cTokenOutAddress);

    return {cTokenIn, cTokenOut};
}


async function getAmountOut(coordinator: DeploymentCoordinator, router: UniswapV2Router02, path: string[], amount: bigint) {
    const { cTokenIn, cTokenOut } = await getCTokensFromPath(coordinator, router, path);

    const feeManager = await getFeeManager(coordinator);
    const unwrapFee = await feeManager.getUnwrapFee(amount, ethers.ZeroAddress, ethers.ZeroAddress);

    const swapInAmount = (amount - unwrapFee) * await cTokenIn.rate();
    const swapOutAmounts = await router.getAmountsOut(swapInAmount, path);
    const swapOutAmount = swapOutAmounts[swapOutAmounts.length - 1];

    const wrapFee = await feeManager.getWrapFee(swapOutAmount, ethers.ZeroAddress, ethers.ZeroAddress);

    return (swapOutAmount - wrapFee) / await cTokenOut.rate();
}


async function getRefund(coordinator: DeploymentCoordinator, router: UniswapV2Router02, path: string[], amount: bigint) {
    const { cTokenIn } = await getCTokensFromPath(coordinator, router, path);

    const feeManager = await getFeeManager(coordinator);
    const unwrapFee = await feeManager.getUnwrapFee(amount, ethers.ZeroAddress, ethers.ZeroAddress);

    const swapInAmount = (amount - unwrapFee) * await cTokenIn.rate();

    const wrapFee = await feeManager.getWrapFee(swapInAmount, ethers.ZeroAddress, ethers.ZeroAddress);

    return (swapInAmount - wrapFee) / await cTokenIn.rate();
}


describe("SwapV0 ETH Support", function () {
  let signers: Signers;
  let tokenA: TestERC20;
  let tokenB: TestERC20;
  let coordinator: DeploymentCoordinator;
  let cTokenA: RegulatedERC7984Upgradeable;
  let wrapperA: WrapperUpgradeable;
  let cTokenB: RegulatedERC7984Upgradeable;
  let wrapperB: WrapperUpgradeable;
  let cETH: RegulatedERC7984Upgradeable;
  let wrapperETH: WrapperUpgradeable;
  let router: UniswapV2Router02;
  let factory: UniswapV2Factory;
  let weth: WETH9;
  let pair: UniswapV2Pair;
  let swapV0: SwapV0;
  let adminProvider: AdminProvider;

  beforeEach(async function () {
    signers = await getSigners();
    ({ router, factory, weth } = await deployUniswapFactoryFixture(signers.admin));

    // Deploy test ERC20 tokens
    tokenA = await deployTestERC20Fixture("TOK_A", 6);
    await tokenA.mint(signers.alice, 100_000_000_000);

    tokenB = await deployTestERC20Fixture("TOK_B", 6);
    await tokenB.mint(signers.alice, 100_000_000_000);

    // Add liquidity: tokenA <-> ETH (via WETH)
    await addLiquidityETH(signers.alice, router, tokenA, 100_000_000, ethers.parseEther("10"));

    // Add liquidity: tokenB <-> ETH (via WETH)
    await addLiquidityETH(signers.alice, router, tokenB, 200_000_000, ethers.parseEther("10"));

    ({ coordinator, adminProvider } = await deployWrapperFixture(signers));

    // Deploy confidential tokenA
    ({ cToken: cTokenA, wrapper: wrapperA } = await deployConfidentialToken(coordinator, tokenA, signers.alice));

    // Deploy confidential tokenB
    ({ cToken: cTokenB, wrapper: wrapperB } = await deployConfidentialToken(coordinator, tokenB, signers.alice));

    // Deploy confidential ETH (originalToken = address(0))
    ({ cEth: cETH, wrapper: wrapperETH } = await deployConfidentialETH(coordinator, signers.alice));

    // Wrap some tokens for alice
    await wrapERC20(coordinator, tokenA, BigInt(100_000), signers.alice.address, signers.alice);

    // Wrap some ETH for alice
    const wrapAmount = ethers.parseEther("1");
    await wrapETH(coordinator, wrapAmount, signers.alice.address, signers.alice);

    ({ swapV0 } = await deploySwapV0Fixture(coordinator));

    // Whitelist the router
    await swapV0.addRouterToWhitelist(await router.getAddress());

    // Whitelist tokens (tokenA, tokenB, and WETH)
    await swapV0.addTokenToWhitelist(await tokenA.getAddress());
    await swapV0.addTokenToWhitelist(await tokenB.getAddress());
    await swapV0.addTokenToWhitelist(await weth.getAddress());
  });

  afterEach(async function () {
    if (this.currentTest.skipCleanup) {
      return; // Skip afterEach logic
    }

      // Verify no ETH stuck in SwapV0
      const swapBalance = await ethers.provider.getBalance(await swapV0.getAddress());
      expect(swapBalance).to.equal(0);

      // Verify no tokenA stuck in SwapV0
      const tokenABalance = await tokenA.balanceOf(await swapV0.getAddress());
      expect(tokenABalance).to.equal(0);

      // Verify no tokenB stuck in SwapV0
      const tokenBBalance = await tokenB.balanceOf(await swapV0.getAddress());
      expect(tokenBBalance).to.equal(0);
  });

  describe("ETH → Token Swaps (cETH → cToken)", function () {
    it("should swap cETH for cTokenA", async function () {
      const balanceCETHBefore = await getConfidentialBalance(cETH, signers.alice);
      const balanceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);

      // Transfer a small amount of cETH (remember: cETH uses internal representation)
      // Alice has 990000 cETH tokens, let's transfer 1000
      const transferAmount = BigInt(1000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cETH.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Path: WETH -> tokenA (Uniswap uses WETH for ETH swaps)
      const path = [await weth.getAddress(), await tokenA.getAddress()];
      const expectedAmountOut = await getAmountOut(coordinator, router, path, transferAmount);

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

      const transferTx = await cETH.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperETH.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      const finalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperETH, signers.alice);

      const unwrapFinalizedEvents = getUnwrapFinalizedEvent(finalizedReceipt);
      expect(unwrapFinalizedEvents).to.have.length(1);

      // Verify Swap event was emitted with success
      const swapEvents = getSwapEvent(finalizedReceipt);
      expect(swapEvents).to.have.length(1);
      const swapEvent = swapEvents[0];
      expect(swapEvent.args.success).to.equal(true);
      expect(swapEvent.args.path).to.deep.equal(path);
      expect(swapEvent.args.errorReasonString).to.equal("");
      expect(swapEvent.args.errorLowLevelData).to.equal("0x");

      const balanceCETHAfter = await getConfidentialBalance(cETH, signers.alice);
      const balanceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);


      // Verify balances changed (swap succeeded)
      expect(balanceCETHBefore - balanceCETHAfter).to.equal(transferAmount);
      expect(balanceCTokenAAfter - balanceCTokenABefore).to.equal(expectedAmountOut);
    });

    it("should refund cETH when ETH → Token swap fails", async function () {
      const balanceAliceCETHBefore = await getConfidentialBalance(cETH, signers.alice);
      const balanceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);

      const transferAmount = BigInt(1000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cETH.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await weth.getAddress(), await tokenA.getAddress()];
      const expectedRefund = await getRefund(coordinator, router, path, transferAmount);

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) - 1, // Expired deadline to force failure
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.bob.address, // Bob is refundTo
          callbackData,
        ]
      );

      const transferTx = await cETH.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperETH.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      // Bob finalizes the unwrap - refund should go to Bob
      const finalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperETH, signers.alice);

      // Verify Swap event shows failure
      const swapEvents = getSwapEvent(finalizedReceipt);
      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);
      expect(swapEvents[0].args.errorReasonString).to.equal("UniswapV2Router: EXPIRED");

      const balanceAliceCETHAfter = await getConfidentialBalance(cETH, signers.alice);
      const balanceBobCETHAfter = await getConfidentialBalance(cETH, signers.bob);
      const balanceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);

      // Alice's balance decreases by the transfer amount
      expect(balanceAliceCETHBefore - balanceAliceCETHAfter).to.equal(transferAmount);

      // Bob receives a refund (he finalized the unwrap)
      expect(balanceBobCETHAfter).to.equal(expectedRefund);

      // No cTokenA received (swap failed)
      expect(balanceCTokenAAfter).to.equal(balanceCTokenABefore);
    });

    it("should reject ETH → Token swap when path[0] is not WETH", async function () {
      const transferAmount = BigInt(1000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cETH.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Invalid path: should be WETH -> tokenA, but using tokenA -> tokenA
      const invalidPath = [await tokenA.getAddress(), await tokenA.getAddress()];

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
          signers.alice.address,
          callbackData,
        ]
      );

      const transferTx = await cETH.connect(signers.alice)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
        await wrapperETH.getAddress(),
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
        data,
      );
      const transferReceipt = await transferTx.wait();

      const finalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperETH, signers.alice);

      // Verify Swap event shows failure
      const swapEvents = getSwapEvent(finalizedReceipt);
      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);
      expect(swapEvents[0].args.errorLowLevelData).to.equal(await swapV0.INPUT_PATH_IS_NOT_UNDERLYING());
    });
  });

  describe("Token → ETH Swaps (cToken → cETH)", function () {
    it("should swap cTokenA for cETH", async function () {
      const balanceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCETHBefore = await getConfidentialBalance(cETH, signers.alice);

      const transferAmount = BigInt(10_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Path: tokenA -> WETH (Uniswap uses WETH for ETH swaps)
      const path = [await tokenA.getAddress(), await weth.getAddress()];
      const expectedAmountOut = await getAmountOut(coordinator, router, path, transferAmount);

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

      const unwrapFinalizedEvents = getUnwrapFinalizedEvent(finalizedReceipt);
      expect(unwrapFinalizedEvents).to.have.length(1);

      // Verify Swap event was emitted with success
      const swapEvents = getSwapEvent(finalizedReceipt);
      expect(swapEvents).to.have.length(1);
      const swapEvent = swapEvents[0];
      expect(swapEvent.args.success).to.equal(true);
      expect(swapEvent.args.path).to.deep.equal(path);
      expect(swapEvent.args.errorReasonString).to.equal("");
      expect(swapEvent.args.errorLowLevelData).to.equal("0x");

      const balanceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCETHAfter = await getConfidentialBalance(cETH, signers.alice);

      expect(balanceCTokenABefore - balanceCTokenAAfter).to.equal(transferAmount);
      expect(balanceCETHAfter - balanceCETHBefore).to.equal(expectedAmountOut);
    });

    it("should refund cTokenA when Token → ETH swap fails", async function () {
      const balanceAliceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceBobCTokenABefore = 0n;
      const balanceCETHBefore = await getConfidentialBalance(cETH, signers.alice);

      const transferAmount = BigInt(10_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const path = [await tokenA.getAddress(), await weth.getAddress()];
      const expectedRefund = await getRefund(coordinator, router, path, transferAmount);

      const abiCoder = new ethers.AbiCoder();
      const callbackData = abiCoder.encode(
        ["tuple(address, uint256, address[], uint256, address)"],
        [[
          await router.getAddress(),
          0,
          path,
          Math.floor(Date.now() / 1000) - 1, // Expired deadline to force failure
          signers.alice.address,
        ]]
      );
      const data = abiCoder.encode(
        ["address", "address", "bytes"],
        [
          await swapV0.getAddress(),
          signers.bob.address, // Bob is refundTo
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
      const finalizedReceipt = await getSwapFinalizedReceipt(transferReceipt, wrapperA, signers.alice);

      // Verify Swap event shows failure
      const swapEvents = getSwapEvent(finalizedReceipt);
      expect(swapEvents).to.have.length(1);
      expect(swapEvents[0].args.success).to.equal(false);
      expect(swapEvents[0].args.errorReasonString).to.equal("UniswapV2Router: EXPIRED");

      const balanceAliceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceBobCTokenAAfter = await getConfidentialBalance(cTokenA, signers.bob);
      const balanceCETHAfter = await getConfidentialBalance(cETH, signers.alice);

      // Alice's balance decreases by the transfer amount
      expect(balanceAliceCTokenABefore - balanceAliceCTokenAAfter).to.equal(transferAmount);

      // Bob receives the refund (he finalized the unwrap)
      expect(balanceBobCTokenAAfter - balanceBobCTokenABefore).to.equal(expectedRefund);

      // No cETH received (swap failed)
      expect(balanceCETHAfter - balanceCETHBefore).to.equal(0);
    });
  });

  describe("Path Validation for ETH Swaps", function () {
    it("should accept valid ETH input path (WETH as path[0])", async function () {
      const path = [await weth.getAddress(), await tokenA.getAddress()];

      const [isValid, errorString, errorData] = await swapV0.checkPath(
        wrapperETH,
        await router.getAddress(),
        path
      );

      expect(isValid).to.equal(true);
      expect(errorString).to.equal("");
      expect(errorData).to.equal("0x");
    });

    it("should accept valid ETH output path (WETH as path[last])", async function () {
      const path = [await tokenA.getAddress(), await weth.getAddress()];

      const [isValid, errorString, errorData] = await swapV0.checkPath(
        wrapperA,
        await router.getAddress(),
        path
      );

      expect(isValid).to.equal(true);
      expect(errorString).to.equal("");
      expect(errorData).to.equal("0x");
    });

    it("should reject ETH input path when path[0] is not WETH", async function () {
      // Try to use tokenA as path[0] when underlying is ETH (address(0))
      const path = [await tokenA.getAddress(), await weth.getAddress()];

      const [isValid, errorString, errorData] = await swapV0.checkPath(
        wrapperETH, // ETH wrapper
        await router.getAddress(),
        path
      );

      expect(isValid).to.equal(false);
      expect(errorData).to.equal(await swapV0.INPUT_PATH_IS_NOT_UNDERLYING());
    });

    it("should verify SwapV0 has receive() function for ETH", async function () {
      // Since we're sending funds directly to the swapper
      this.test.skipCleanup = true;

      // Send ETH directly to SwapV0 to verify it can receive ETH
      const sendAmount = ethers.parseEther("0.1");
      const balanceBefore = await ethers.provider.getBalance(await swapV0.getAddress());

      await signers.alice.sendTransaction({
        to: await swapV0.getAddress(),
        value: sendAmount
      });

      const balanceAfter = await ethers.provider.getBalance(await swapV0.getAddress());
      expect(balanceAfter - balanceBefore).to.equal(sendAmount);
    });
  });

  describe("Multi-Hop Swaps with ETH", function () {
    it("should swap cTokenA -> ETH -> cTokenB", async function () {
      const balanceCTokenABefore = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCTokenBBefore = 0n;

      const transferAmount = BigInt(10_000);
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(await cTokenA.getAddress(), signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Path: tokenA -> WETH -> tokenB (multi-hop through ETH)
      const path = [
        await tokenA.getAddress(),
        await weth.getAddress(),
        await tokenB.getAddress()
      ];
      const expectedAmountOut = await getAmountOut(coordinator, router, path, transferAmount);

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

      const unwrapFinalizedEvents = getUnwrapFinalizedEvent(finalizedReceipt);
      expect(unwrapFinalizedEvents).to.have.length(1);

      // Verify Swap event was emitted with success
      const swapEvents = getSwapEvent(finalizedReceipt);
      expect(swapEvents).to.have.length(1);
      const swapEvent = swapEvents[0];
      expect(swapEvent.args.success).to.equal(true);
      expect(swapEvent.args.path).to.deep.equal(path);
      expect(swapEvent.args.errorReasonString).to.equal("");
      expect(swapEvent.args.errorLowLevelData).to.equal("0x");

      const balanceCTokenAAfter = await getConfidentialBalance(cTokenA, signers.alice);
      const balanceCTokenBAfter = await getConfidentialBalance(cTokenB, signers.alice);

      // Verify balances changed correctly
      expect(balanceCTokenABefore - balanceCTokenAAfter).to.equal(transferAmount);
      expect(balanceCTokenBAfter - balanceCTokenBBefore).to.equal(expectedAmountOut);
    });
  });
});
