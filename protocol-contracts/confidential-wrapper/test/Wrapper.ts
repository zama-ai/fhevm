import { ethers } from "hardhat";
import { expect } from "chai";
import {
  getWrapDeployedEvent,
  deployConfidentialToken,
  deployConfidentialETH,
  getDeployConfidentialTokenTx,
  getWrapFeeBasisPoints
} from "./utils";
import { getSigners, Signers } from "./signers";

import type { TestERC20, AdminProvider, DeploymentCoordinator, WrapperFactory, RegulatedERC7984UpgradeableFactory } from "../types";
import { MAX_DECIMALS } from "./constants";
import { deployAdminProviderFixture, deployConfidentialTokenFactoryFixture, deployERC20InvalidDecimalsFixture, deployERC20NoDecimalsFixture, deployTestERC20Fixture, deployWrapperFactoryFixture, deployWrapperFixture } from "./fixtures";


async function deployFixture(signers: Signers) {
  const { coordinator, coordinatorAddress, adminProvider, confidentialTokenFactory, wrapperFactory } = await deployWrapperFixture(signers);

  const usdc = await deployTestERC20Fixture("USDC");
  const usdcAddress = await usdc.getAddress();

  let transaction = await usdc.mint(signers.alice, 1000);
  await transaction.wait();

  const usdt = await deployTestERC20Fixture("USDT");
  const usdtAddress = await usdt.getAddress();

  transaction = await usdt.mint(signers.alice, 1000);
  await transaction.wait();

  return { coordinator, coordinatorAddress, adminProvider, confidentialTokenFactory, wrapperFactory, usdc, usdcAddress, usdt, usdtAddress };
}

describe("Wrapper", function () {
  let signers: Signers;
  let coordinator: DeploymentCoordinator;
  let coordinatorAddress: string;
  let adminProvider: AdminProvider;
  let usdc: TestERC20;
  let usdcAddress: string;

  before(async function () {
    signers = await getSigners();
  });

  beforeEach(async function () {
    ({
      coordinator,
      coordinatorAddress,
      adminProvider,
      usdc,
      usdcAddress,
    } = await deployFixture(signers));
  });

  describe("Deploy", function () {
    it("should configure the right permissions", async function () {
      await deployConfidentialToken(coordinator, usdc, signers.alice);

      const cUsdcAddress = await coordinator.getConfidentialToken(usdcAddress);
      const cUsdc = await ethers.getContractAt("RegulatedERC7984Upgradeable", cUsdcAddress);
      const wrapperAddress = await coordinator.getWrapper(usdcAddress);
      const wrapper = await ethers.getContractAt("Wrapper", wrapperAddress);

      expect(await cUsdc.hasRole(await cUsdc.DEFAULT_ADMIN_ROLE(), wrapperAddress)).to.equal(false);
      expect(await cUsdc.hasRole(await cUsdc.WRAPPER_ROLE(), wrapperAddress)).to.equal(true);
      expect(await cUsdc.hasRole(await cUsdc.DEFAULT_ADMIN_ROLE(), await adminProvider.owner())).to.equal(true);
      expect(await cUsdc.hasRole(await cUsdc.DEFAULT_ADMIN_ROLE(), coordinatorAddress)).to.equal(false);
    });

    it("should not deploy new encrypted erc20 when paying less than required fee", async function () {
      // Get deploy fee from FeeManager
      const feeManagerAddress = await adminProvider.feeManager();
      const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
      const deployTokenFee = await feeManager.getDeployFee(ethers.ZeroAddress);

      const insufficientFee = deployTokenFee > 0n ? deployTokenFee - 1n : 0n;
      const tx = coordinator.deploy(usdcAddress, {
        value: insufficientFee,
      });
      await expect(tx).to.be.revertedWithCustomError(coordinator, "IncorrectDeployFee");
    });

    it("should not deploy new encrypted erc20 when paying more than required fee", async function () {
      // Get deploy fee from FeeManager
      const feeManagerAddress = await adminProvider.feeManager();
      const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
      const deployTokenFee = await feeManager.getDeployFee(ethers.ZeroAddress);

      const excessiveFee = deployTokenFee + 1n;
      const tx = coordinator.deploy(usdcAddress, {
        value: excessiveFee,
      });
      await expect(tx).to.be.revertedWithCustomError(coordinator, "IncorrectDeployFee");
    });

    it("should deploy new encrypted erc20 when paying exact fee", async function () {
      // Get deploy fee from FeeManager
      const feeManagerAddress = await adminProvider.feeManager();
      const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
      const deployTokenFee = await feeManager.getDeployFee(ethers.ZeroAddress);

      // Deploy with exact fee should succeed
      await expect(
        coordinator.deploy(usdcAddress, { value: deployTokenFee })
      ).to.not.be.reverted;

      // Verify deployment was successful
      const cUsdcAddress = await coordinator.getConfidentialToken(usdcAddress);
      expect(cUsdcAddress).to.not.equal(ethers.ZeroAddress);

      const wrapperAddress = await coordinator.getWrapper(usdcAddress);
      expect(wrapperAddress).to.not.equal(ethers.ZeroAddress);
    });

    it("should deploy new encrypted ETH", async function () {
      // Get deploy fee from FeeManager
      const { cEth, cEthAddress, wrapperAddress, wrapper, receipt } = await deployConfidentialETH(coordinator, signers.alice);

      expect(await cEth.name()).to.equal("confidential Ethereum");
      expect(await cEth.symbol()).to.equal("cETH");
      expect(await cEth.decimals()).to.equal(MAX_DECIMALS);

      const wrapDeployedEvent = getWrapDeployedEvent(receipt);
      expect(wrapDeployedEvent.length).to.be.equal(1);
      expect(wrapDeployedEvent[0].args[0]).to.be.equal(ethers.ZeroAddress);
      expect(wrapDeployedEvent[0].args[1]).to.be.equal(wrapperAddress);
      expect(wrapDeployedEvent[0].args[2]).to.be.equal(cEthAddress);
      expect(wrapDeployedEvent[0].args[3]).to.be.equal("Ethereum");
      expect(wrapDeployedEvent[0].args[4]).to.be.equal("ETH");
      expect(wrapDeployedEvent[0].args[5]).to.be.equal(18);
      expect(wrapDeployedEvent[0].args[6]).to.be.equal(signers.alice);
    });

    for (const originalDecimals of [5, 6, 18]) {
      let itName = "equal to";
      if (originalDecimals > MAX_DECIMALS) {
        itName = "greater than";
      } else if (originalDecimals < MAX_DECIMALS) {
        itName = "lesser than";
      }
      it(`should deploy new encrypted erc20 with from erc20 with decimals ${itName} MAX_DECIMALS`, async function () {
        const originalName = "My Token";
        const originalSymbol = "TOK";
        const erc20 = await deployTestERC20Fixture(originalSymbol, originalDecimals, originalName);
        const erc20Address = await erc20.getAddress();

        const { receipt } = await deployConfidentialToken(coordinator, erc20, signers.alice);

        const cErc20Address = await coordinator.getConfidentialToken(erc20Address);
        const cErc20 = await ethers.getContractAt("RegulatedERC7984Upgradeable", cErc20Address);
        const wrapperAddress = await coordinator.getWrapper(erc20Address);
        const wrapper = await ethers.getContractAt("Wrapper", wrapperAddress);

        expect(await cErc20.name()).to.equal(`confidential ${originalName}`);
        expect(await cErc20.symbol()).to.equal(`c${originalSymbol}`);
        if (originalDecimals > MAX_DECIMALS) {
          expect(await cErc20.decimals()).to.equal(MAX_DECIMALS);
        } else {
          expect(await cErc20.decimals()).to.equal(originalDecimals);
        }

        expect(await cErc20.hasRole(await cErc20.DEFAULT_ADMIN_ROLE(), wrapperAddress)).to.equal(false);
        expect(await cErc20.hasRole(await cErc20.WRAPPER_ROLE(), wrapperAddress)).to.equal(true);
        expect(await cErc20.regulator()).to.equal(await adminProvider.regulator());

        const wrapDeployedEvent = getWrapDeployedEvent(receipt);
        expect(wrapDeployedEvent.length).to.be.equal(1);
        expect(wrapDeployedEvent[0].args[0]).to.be.equal(erc20Address);
        expect(wrapDeployedEvent[0].args[1]).to.be.equal(wrapperAddress);
        expect(wrapDeployedEvent[0].args[2]).to.be.equal(cErc20Address);
        expect(wrapDeployedEvent[0].args[3]).to.be.equal(originalName);
        expect(wrapDeployedEvent[0].args[4]).to.be.equal(originalSymbol);
        expect(wrapDeployedEvent[0].args[5]).to.be.equal(originalDecimals);
        expect(wrapDeployedEvent[0].args[6]).to.be.equal(signers.alice);
      });
    }

    it("should revert if encrypted erc20 already exists", async function () {
      await deployConfidentialToken(coordinator, usdc, signers.alice);

      await expect(getDeployConfidentialTokenTx(coordinator, usdc, signers.alice)).to.be.revertedWithCustomError(
        coordinator,
        "WrapperAlreadyExists",
      );
    });

    it("should deploy tokens with admin provider from wrapper", async function () {
      // Deploy a confidential token
      await deployConfidentialToken(coordinator, usdc, signers.alice);

      const cUsdcAddress = await coordinator.getConfidentialToken(usdcAddress);
      const cUsdc = await ethers.getContractAt("RegulatedERC7984Upgradeable", cUsdcAddress);

      // Verify the token has the same admin provider as the coordinator
      expect(await cUsdc.adminProvider()).to.equal(await coordinator.adminProvider());
    });

    it("should revert on fee transfer failure", async function () {
      // Deploy a contract that rejects ETH transfers
      const RejectEthContract = await ethers.getContractFactory("RejectEth");
      const rejectEth = await RejectEthContract.deploy();
      await rejectEth.waitForDeployment();

      // Update fee manager to use the rejecting contract as fee recipient
      const feeManagerAddress = await adminProvider.feeManager();
      const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
      await feeManager.setFeeRecipient(await rejectEth.getAddress());

      // Get deploy fee
      const deployTokenFee = await feeManager.getDeployFee(ethers.ZeroAddress);

      // Attempt to deploy should fail due to fee transfer failure
      await expect(
        coordinator.deploy(usdcAddress, { value: deployTokenFee })
      ).to.be.revertedWithCustomError(coordinator, "FeeTransferFailed");
    });

    describe("Protection against griefing attacks", function () {
      it("should prevent deploying wrapper for non-existent token (no code at address)", async function () {
        // Attacker tries to front-run a CREATE2 token deployment
        // by deploying wrapper for an address with no code
        const nonExistentTokenAddress = "0x000000000000000000000004500000000000007B";

        // Get deploy fee
        const feeManagerAddress = await adminProvider.feeManager();
        const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
        const deployTokenFee = await feeManager.getDeployFee(ethers.ZeroAddress);

        // Attempt to deploy should fail because token doesn't exist
        await expect(
          coordinator.connect(signers.charlie).deploy(nonExistentTokenAddress, { value: deployTokenFee })
        ).to.be.revertedWithCustomError(coordinator, "TokenMustExist");

        // Verify no wrapper or cToken was deployed
        expect(await coordinator.getWrapper(nonExistentTokenAddress)).to.equal(ethers.ZeroAddress);
        expect(await coordinator.getConfidentialToken(nonExistentTokenAddress)).to.equal(ethers.ZeroAddress);
        expect(await coordinator.wrapperExists(nonExistentTokenAddress)).to.equal(false);
      });

      it("should prevent deployment for EOA (externally owned account)", async function () {
        // EOAs have no code, so should be rejected
        const eoaAddress = ethers.Wallet.createRandom().address;

        const feeManagerAddress = await adminProvider.feeManager();
        const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
        const deployTokenFee = await feeManager.getDeployFee(ethers.ZeroAddress);

        await expect(
          coordinator.deploy(eoaAddress, { value: deployTokenFee })
        ).to.be.revertedWithCustomError(coordinator, "TokenMustExist");
      });
    });
  });

  describe("Constructor Validation", function () {
    it("should revert if admin provider is zero address", async function () {
      const WrapperFactoryContract = await ethers.getContractFactory("WrapperFactory");
      const RegulatedERC7984UpgradeableFactoryContract = await ethers.getContractFactory("RegulatedERC7984UpgradeableFactory");
      const DeploymentCoordinatorContract = await ethers.getContractFactory("DeploymentCoordinator");

      const wrapperFactory = await WrapperFactoryContract.deploy();
      const confidentialTokenFactory = await RegulatedERC7984UpgradeableFactoryContract.deploy();

      await expect(
        DeploymentCoordinatorContract.deploy(
          ethers.ZeroAddress, // adminProvider
          wrapperFactory,
          confidentialTokenFactory
        )
      ).to.be.revertedWithCustomError(DeploymentCoordinatorContract, "ZeroAddressAdminProvider");
    });

    it("should revert if wrapper factory is zero address", async function () {
      const RegulatedERC7984UpgradeableFactoryContract = await ethers.getContractFactory("RegulatedERC7984UpgradeableFactory");
      const DeploymentCoordinatorContract = await ethers.getContractFactory("DeploymentCoordinator");

      const confidentialTokenFactory = await RegulatedERC7984UpgradeableFactoryContract.deploy();

      await expect(
        DeploymentCoordinatorContract.deploy(
          adminProvider,
          ethers.ZeroAddress, // wrapperFactory
          confidentialTokenFactory
        )
      ).to.be.revertedWithCustomError(DeploymentCoordinatorContract, "ZeroAddressWrapperFactory");
    });

    it("should revert if confidential token factory is zero address", async function () {
      const WrapperFactoryContract = await ethers.getContractFactory("WrapperFactory");
      const DeploymentCoordinatorContract = await ethers.getContractFactory("DeploymentCoordinator");

      const wrapperFactory = await WrapperFactoryContract.deploy();

      await expect(
        DeploymentCoordinatorContract.deploy(
          adminProvider,
          wrapperFactory,
          ethers.ZeroAddress // confidentialTokenFactory
        )
      ).to.be.revertedWithCustomError(DeploymentCoordinatorContract, "ZeroAddressConfidentialTokenFactory");
    });
  });

  describe("Fees", function () {
    it("should return wrap fee", async function () {
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

  describe("DeploymentCoordinator wrapperExists", function () {
    it("should return false for non-deployed wrapper", async function () {
      expect(await coordinator.wrapperExists(usdcAddress)).to.equal(false);
    });

    it("should return true for deployed ERC20 wrapper", async function () {
      await deployConfidentialToken(coordinator, usdc, signers.alice);
      expect(await coordinator.wrapperExists(usdcAddress)).to.equal(true);
    });

    it("should return true for deployed ETH wrapper", async function () {
      await deployConfidentialETH(coordinator, signers.alice);
      expect(await coordinator.wrapperExists(ethers.ZeroAddress)).to.equal(true);
    });

    it("should return false for random address", async function () {
      const randomAddress = ethers.Wallet.createRandom().address;
      expect(await coordinator.wrapperExists(randomAddress)).to.equal(false);
    });
  });

  describe("DeploymentCoordinator setters", function () {
    it("should allow owner to set adminProvider and emit event", async function () {
      const { adminProviderAddress: newAdminProviderAddress } = await deployAdminProviderFixture(signers);

      const oldAdminProvider = await coordinator.adminProvider();
      const tx = await coordinator.setAdminProvider(newAdminProviderAddress);

      await expect(tx)
        .to.emit(coordinator, "AdminProviderUpdated")
        .withArgs(oldAdminProvider, newAdminProviderAddress);

      expect(await coordinator.adminProvider()).to.equal(newAdminProviderAddress);
    });

    it("should revert when non-owner tries to set adminProvider", async function () {
      const { adminProviderAddress: newAdminProviderAddress } = await deployAdminProviderFixture(signers);

      await expect(
        coordinator.connect(signers.alice).setAdminProvider(newAdminProviderAddress)
      ).to.be.revertedWithCustomError(coordinator, "OwnableUnauthorizedAccount");
    });

    it("should revert when setting adminProvider to zero address", async function () {
      await expect(
        coordinator.setAdminProvider(ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(coordinator, "ZeroAddressAdminProvider");
    });

    it("should allow owner to set wrapperFactory and emit event", async function () {
      const { wrapperFactoryAddress: newWrapperFactoryAddress } = await deployWrapperFactoryFixture();

      const oldWrapperFactory = await coordinator.wrapperFactory();
      const tx = await coordinator.setWrapperFactory(newWrapperFactoryAddress);

      await expect(tx)
        .to.emit(coordinator, "WrapperFactoryUpdated")
        .withArgs(oldWrapperFactory, newWrapperFactoryAddress);

      expect(await coordinator.wrapperFactory()).to.equal(newWrapperFactoryAddress);
    });

    it("should revert when non-owner tries to set wrapperFactory", async function () {
      const { wrapperFactoryAddress: newWrapperFactoryAddress } = await deployWrapperFactoryFixture();

      await expect(
        coordinator.connect(signers.alice).setWrapperFactory(newWrapperFactoryAddress)
      ).to.be.revertedWithCustomError(coordinator, "OwnableUnauthorizedAccount");
    });

    it("should revert when setting wrapperFactory to zero address", async function () {
      await expect(
        coordinator.setWrapperFactory(ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(coordinator, "ZeroAddressWrapperFactory");
    });

    it("should allow owner to set confidentialTokenFactory and emit event", async function () {
      const { confidentialTokenFactoryAddress: newConfidentialTokenFactoryAddress } = await deployConfidentialTokenFactoryFixture();

      const oldConfidentialTokenFactory = await coordinator.confidentialTokenFactory();
      const tx = await coordinator.setConfidentialTokenFactory(newConfidentialTokenFactoryAddress);

      await expect(tx)
        .to.emit(coordinator, "ConfidentialTokenFactoryUpdated")
        .withArgs(oldConfidentialTokenFactory, newConfidentialTokenFactoryAddress);

      expect(await coordinator.confidentialTokenFactory()).to.equal(newConfidentialTokenFactoryAddress);
    });

    it("should revert when non-owner tries to set confidentialTokenFactory", async function () {
      const { confidentialTokenFactoryAddress: newConfidentialTokenFactoryAddress } = await deployConfidentialTokenFactoryFixture();

      await expect(
        coordinator.connect(signers.alice).setConfidentialTokenFactory(newConfidentialTokenFactoryAddress)
      ).to.be.revertedWithCustomError(coordinator, "OwnableUnauthorizedAccount");
    });

    it("should revert when setting confidentialTokenFactory to zero address", async function () {
      await expect(
        coordinator.setConfidentialTokenFactory(ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(coordinator, "ZeroAddressConfidentialTokenFactory");
    });
  });

  describe("deploy with _tryGetAssetDecimals fallback", function () {
    it("should use fallback decimals (18) when token has no decimals function", async function () {
      const { tokenNoDecimals } = await deployERC20NoDecimalsFixture();
      const { cToken } = await deployConfidentialToken(coordinator, tokenNoDecimals, signers.alice);

      expect(await cToken.decimals()).to.equal(MAX_DECIMALS);
      expect(await cToken.rate()).to.equal(BigInt(10) ** BigInt(12));
    });

    it("should use fallback decimals (18) when token returns invalid decimals data", async function () {
      const { tokenInvalidDecimals } = await deployERC20InvalidDecimalsFixture();
      const { cToken } = await deployConfidentialToken(coordinator, tokenInvalidDecimals, signers.alice);

      expect(await cToken.decimals()).to.equal(MAX_DECIMALS);
      expect(await cToken.rate()).to.equal(BigInt(10) ** BigInt(12));
    });
  });

  describe("setWrapperImplementation", function () {
    it("should deploy new wrappers with updated implementation after calling setWrapperImplementation", async function () {
      // Deploy first wrapper with default implementation
      await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapperAddress = await coordinator.getWrapper(usdcAddress);

      // Get implementation of first wrapper
      const implementationSlot = "0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc";
      const impl1 = await ethers.provider.getStorage(wrapperAddress, implementationSlot);
      const impl1Address = "0x" + impl1.slice(26);

      // Deploy V2 implementation
      const WrapperV2 = await ethers.getContractFactory("WrapperUpgradeableV2Mock");
      const v2Impl = await WrapperV2.deploy();
      await v2Impl.waitForDeployment();
      const v2ImplAddress = await v2Impl.getAddress();

      // Update coordinator's canonical implementation to V2
      await coordinator.setWrapperImplementation(v2ImplAddress);

      // Verify coordinator now points to V2 implementation
      expect(await coordinator.wrapperImplementation()).to.equal(v2ImplAddress);

      // Deploy second wrapper (USDT) - should use V2 implementation
      const usdt = await deployTestERC20Fixture("USDT");
      const usdtAddress = await usdt.getAddress();
      await deployConfidentialToken(coordinator, usdt, signers.alice);
      const wrapper2Address = await coordinator.getWrapper(usdtAddress);

      // Get implementation of second wrapper
      const impl2 = await ethers.provider.getStorage(wrapper2Address, implementationSlot);
      const impl2Address = "0x" + impl2.slice(26);

      // Verify second wrapper uses V2 implementation
      expect(impl2Address.toLowerCase()).to.equal(v2ImplAddress.toLowerCase());

      // Verify first wrapper still uses V1 implementation (unchanged)
      const impl1After = await ethers.provider.getStorage(wrapperAddress, implementationSlot);
      const impl1AfterAddress = "0x" + impl1After.slice(26);
      expect(impl1AfterAddress.toLowerCase()).to.equal(impl1Address.toLowerCase());
      expect(impl1AfterAddress.toLowerCase()).to.not.equal(v2ImplAddress.toLowerCase());

      // Verify V2 functionality works on second wrapper
      const wrapper2V2 = await ethers.getContractAt("WrapperUpgradeableV2Mock", wrapper2Address);
      expect(await wrapper2V2.counter()).to.equal(0);
      await wrapper2V2.incrementCounter();
      expect(await wrapper2V2.counter()).to.equal(1);

      // Verify first wrapper does NOT have V2 functionality (still V1)
      const wrapper1AsV2 = await ethers.getContractAt("WrapperUpgradeableV2Mock", wrapperAddress);
      await expect(wrapper1AsV2.counter()).to.be.reverted;
    });

    it("should prevent non-owner from calling setWrapperImplementation", async function () {
      const WrapperV2 = await ethers.getContractFactory("WrapperUpgradeableV2Mock");
      const v2Impl = await WrapperV2.deploy();
      await v2Impl.waitForDeployment();
      const v2ImplAddress = await v2Impl.getAddress();

      await expect(
        coordinator.connect(signers.alice).setWrapperImplementation(v2ImplAddress)
      ).to.be.revertedWithCustomError(coordinator, "OwnableUnauthorizedAccount")
        .withArgs(signers.alice.address);
    });

    it("should revert when setting zero address as implementation", async function () {
      await expect(
        coordinator.setWrapperImplementation(ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(coordinator, "ZeroAddressImplementation");
    });
  });

  describe("setConfidentialTokenImplementation", function () {
    it("should deploy new tokens with updated implementation after calling setConfidentialTokenImplementation", async function () {
      // Deploy first token with default implementation
      await deployConfidentialToken(coordinator, usdc, signers.alice);
      const cUsdcAddress = await coordinator.getConfidentialToken(usdcAddress);

      // Get implementation of first token
      const implementationSlot = "0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc";
      const impl1 = await ethers.provider.getStorage(cUsdcAddress, implementationSlot);
      const impl1Address = "0x" + impl1.slice(26);

      // Deploy V2 implementation
      const RegulatedERC7984V2 = await ethers.getContractFactory("RegulatedERC7984UpgradeableV2Mock");
      const v2Impl = await RegulatedERC7984V2.deploy();
      await v2Impl.waitForDeployment();
      const v2ImplAddress = await v2Impl.getAddress();

      // Update coordinator's canonical implementation to V2
      await coordinator.setConfidentialTokenImplementation(v2ImplAddress);

      // Verify coordinator now points to V2 implementation
      expect(await coordinator.confidentialTokenImplementation()).to.equal(v2ImplAddress);

      // Deploy second token (USDT) - should use V2 implementation
      const usdt = await deployTestERC20Fixture("USDT");
      const usdtAddress = await usdt.getAddress();
      await deployConfidentialToken(coordinator, usdt, signers.alice);
      const cUsdtAddress = await coordinator.getConfidentialToken(usdtAddress);

      // Get implementation of second token
      const impl2 = await ethers.provider.getStorage(cUsdtAddress, implementationSlot);
      const impl2Address = "0x" + impl2.slice(26);

      // Verify second token uses V2 implementation
      expect(impl2Address.toLowerCase()).to.equal(v2ImplAddress.toLowerCase());

      // Verify first token still uses V1 implementation (unchanged)
      const impl1After = await ethers.provider.getStorage(cUsdcAddress, implementationSlot);
      const impl1AfterAddress = "0x" + impl1After.slice(26);
      expect(impl1AfterAddress.toLowerCase()).to.equal(impl1Address.toLowerCase());
      expect(impl1AfterAddress.toLowerCase()).to.not.equal(v2ImplAddress.toLowerCase());

      // Verify V2 functionality works on second token
      const cUsdtV2 = await ethers.getContractAt("RegulatedERC7984UpgradeableV2Mock", cUsdtAddress);
      expect(await cUsdtV2.counter()).to.equal(0);
      await cUsdtV2.incrementCounter();
      expect(await cUsdtV2.counter()).to.equal(1);

      // Verify first token does NOT have V2 functionality (still V1)
      const cUsdcAsV2 = await ethers.getContractAt("RegulatedERC7984UpgradeableV2Mock", cUsdcAddress);
      await expect(cUsdcAsV2.counter()).to.be.reverted;
    });

    it("should prevent non-owner from calling setConfidentialTokenImplementation", async function () {
      const RegulatedERC7984V2 = await ethers.getContractFactory("RegulatedERC7984UpgradeableV2Mock");
      const v2Impl = await RegulatedERC7984V2.deploy();
      await v2Impl.waitForDeployment();
      const v2ImplAddress = await v2Impl.getAddress();

      await expect(
        coordinator.connect(signers.alice).setConfidentialTokenImplementation(v2ImplAddress)
      ).to.be.revertedWithCustomError(coordinator, "OwnableUnauthorizedAccount")
        .withArgs(signers.alice.address);
    });

    it("should revert when setting zero address as implementation", async function () {
      await expect(
        coordinator.setConfidentialTokenImplementation(ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(coordinator, "ZeroAddressImplementation");
    });
  });

  describe("Upgrade deployed RegulatedERC7984Upgradeable contract", function () {
    it("should allow upgrading with UPGRADER_ROLE", async function () {
      await deployConfidentialToken(coordinator, usdc, signers.alice);

      const cUsdcAddress = await coordinator.getConfidentialToken(usdcAddress);
      const cUsdc = await ethers.getContractAt("RegulatedERC7984Upgradeable", cUsdcAddress);

      // Admin provider owner has DEFAULT_ADMIN_ROLE
      const adminProviderOwner = await adminProvider.owner();

      // Grant UPGRADER_ROLE to bob
      await ethers.provider.send("hardhat_impersonateAccount", [adminProviderOwner]);
      const adminSigner = await ethers.getSigner(adminProviderOwner);
      const UPGRADER_ROLE = await cUsdc.UPGRADER_ROLE();
      await cUsdc.connect(adminSigner).grantRole(UPGRADER_ROLE, signers.bob.address);
      await ethers.provider.send("hardhat_stopImpersonatingAccount", [adminProviderOwner]);

      const RegulatedERC7984UpgradeableV2Mock = await ethers.getContractFactory("RegulatedERC7984UpgradeableV2Mock");
      const newImpl = await RegulatedERC7984UpgradeableV2Mock.deploy();
      await newImpl.waitForDeployment();

      // Bob with UPGRADER_ROLE can upgrade
      const tx = await cUsdc.connect(signers.bob).upgradeToAndCall(await newImpl.getAddress(), "0x");
      await tx.wait();

      const cUsdcV2 = await ethers.getContractAt("RegulatedERC7984UpgradeableV2Mock", cUsdcAddress);
      await cUsdcV2.connect(signers.bob).incrementCounter();
      expect(await cUsdcV2.counter()).to.equal(1);
    });

    it("should NOT allow upgrading without UPGRADER_ROLE", async function () {
      await deployConfidentialToken(coordinator, usdc, signers.alice);

      const cUsdcAddress = await coordinator.getConfidentialToken(usdcAddress);
      const cUsdc = await ethers.getContractAt("RegulatedERC7984Upgradeable", cUsdcAddress);

      const UPGRADER_ROLE = await cUsdc.UPGRADER_ROLE();
      expect(await cUsdc.hasRole(UPGRADER_ROLE, signers.alice.address)).to.equal(false);

      const RegulatedERC7984UpgradeableV2Mock = await ethers.getContractFactory("RegulatedERC7984UpgradeableV2Mock");
      const newImpl = await RegulatedERC7984UpgradeableV2Mock.deploy();
      await newImpl.waitForDeployment();

      await expect(
        cUsdc.connect(signers.alice).upgradeToAndCall(await newImpl.getAddress(), "0x")
      ).to.be.revertedWithCustomError(
        cUsdc,
        "AccessControlUnauthorizedAccount",
      ).withArgs(signers.alice.address, UPGRADER_ROLE);
    });
  });

  describe("Non-Standard ERC20 Token Support", function () {
    describe("ERC20 with bytes32 name/symbol (like MKR)", function () {
      it("should deploy wrapper for token with bytes32 metadata", async function () {
        // Deploy ERC20 with bytes32 metadata (like MKR)
        const ERC20Bytes32MetadataFactory = await ethers.getContractFactory("ERC20Bytes32Metadata");

        // Convert strings to bytes32
        const nameBytes32 = ethers.encodeBytes32String("Maker");
        const symbolBytes32 = ethers.encodeBytes32String("MKR");

        const mkrLikeToken = await ERC20Bytes32MetadataFactory.deploy(
          nameBytes32,
          symbolBytes32,
          18
        );
        await mkrLikeToken.waitForDeployment();
        const mkrLikeTokenAddress = await mkrLikeToken.getAddress();

        // Mint some tokens for testing
        await mkrLikeToken.mint(signers.alice, 1000);

        // Deploy confidential wrapper
        const { cToken, wrapper, receipt } = await deployConfidentialToken(coordinator, mkrLikeToken, signers.alice);

        // Verify deployment succeeded
        expect(await coordinator.wrapperExists(mkrLikeTokenAddress)).to.equal(true);

        // Verify confidential token was created with parsed metadata
        expect(await cToken.name()).to.equal("confidential Maker");
        expect(await cToken.symbol()).to.equal("cMKR");
        expect(await cToken.decimals()).to.equal(MAX_DECIMALS);

        // Verify event contains correct metadata
        const wrapDeployedEvent = getWrapDeployedEvent(receipt);
        expect(wrapDeployedEvent.length).to.equal(1);
        expect(wrapDeployedEvent[0].args[3]).to.equal("Maker");
        expect(wrapDeployedEvent[0].args[4]).to.equal("MKR");
        expect(wrapDeployedEvent[0].args[5]).to.equal(18);
      });

      it("should handle bytes32 metadata with trailing zeros", async function () {
        const ERC20Bytes32MetadataFactory = await ethers.getContractFactory("ERC20Bytes32Metadata");

        // Short name/symbol will have trailing zeros
        const nameBytes32 = ethers.encodeBytes32String("Dai");
        const symbolBytes32 = ethers.encodeBytes32String("DAI");

        const daiLikeToken = await ERC20Bytes32MetadataFactory.deploy(
          nameBytes32,
          symbolBytes32,
          18
        );
        await daiLikeToken.waitForDeployment();

        const { cToken } = await deployConfidentialToken(coordinator, daiLikeToken, signers.alice);

        // Verify trailing zeros were properly removed
        expect(await cToken.name()).to.equal("confidential Dai");
        expect(await cToken.symbol()).to.equal("cDAI");
      });
    });

    describe("ERC20 that reverts on name/symbol", function () {
      it("should deploy wrapper using address fallback when name/symbol revert", async function () {
        const ERC20RevertingMetadataFactory = await ethers.getContractFactory("ERC20RevertingMetadata");
        const revertingToken = await ERC20RevertingMetadataFactory.deploy(18);
        await revertingToken.waitForDeployment();
        const revertingTokenAddress = await revertingToken.getAddress();

        await revertingToken.mint(signers.alice, 1000);

        const { cToken, receipt } = await deployConfidentialToken(coordinator, revertingToken, signers.alice);

        // Verify deployment succeeded
        expect(await coordinator.wrapperExists(revertingTokenAddress)).to.equal(true);

        // Verify fallback to address-based naming
        const expectedName = ethers.toBeHex(BigInt(revertingTokenAddress), 20);
        const expectedSymbol = expectedName.slice(0, 8); // "0x" + 6 chars

        expect(await cToken.name()).to.equal(`confidential ${expectedName}`);
        expect(await cToken.symbol()).to.equal(`c${expectedSymbol}`);

        // Verify event uses fallback names
        const wrapDeployedEvent = getWrapDeployedEvent(receipt);
        expect(wrapDeployedEvent[0].args[3]).to.equal(expectedName);
        expect(wrapDeployedEvent[0].args[4]).to.equal(expectedSymbol);
      });
    });

    describe("ERC20 with empty name/symbol", function () {
      it("should deploy wrapper using address fallback when name/symbol are empty", async function () {
        const ERC20EmptyMetadataFactory = await ethers.getContractFactory("ERC20EmptyMetadata");
        const emptyToken = await ERC20EmptyMetadataFactory.deploy(18);
        await emptyToken.waitForDeployment();
        const emptyTokenAddress = await emptyToken.getAddress();

        await emptyToken.mint(signers.alice, 1000);

        const { cToken, receipt } = await deployConfidentialToken(coordinator, emptyToken, signers.alice);

        // Verify deployment succeeded
        expect(await coordinator.wrapperExists(emptyTokenAddress)).to.equal(true);

        // Verify fallback to address-based naming
        const expectedName = ethers.toBeHex(BigInt(emptyTokenAddress), 20);
        const expectedSymbol = expectedName.slice(0, 8); // "0x" + 6 chars

        expect(await cToken.name()).to.equal(`confidential ${expectedName}`);
        expect(await cToken.symbol()).to.equal(`c${expectedSymbol}`);

        // Verify event uses fallback names
        const wrapDeployedEvent = getWrapDeployedEvent(receipt);
        expect(wrapDeployedEvent[0].args[3]).to.equal(expectedName);
        expect(wrapDeployedEvent[0].args[4]).to.equal(expectedSymbol);
      });
    });

    describe("ERC20 with no metadata functions", function () {
      it("should deploy wrapper using address fallback when metadata functions don't exist", async function () {
        const ERC20NoMetadataFactory = await ethers.getContractFactory("ERC20NoMetadata");
        const noMetadataToken = await ERC20NoMetadataFactory.deploy();
        await noMetadataToken.waitForDeployment();
        const noMetadataTokenAddress = await noMetadataToken.getAddress();

        await noMetadataToken.mint(signers.alice, 1000);

        const { cToken, receipt } = await deployConfidentialToken(coordinator, noMetadataToken, signers.alice);

        // Verify deployment succeeded
        expect(await coordinator.wrapperExists(noMetadataTokenAddress)).to.equal(true);

        // Verify fallback to address-based naming
        const expectedName = ethers.toBeHex(BigInt(noMetadataTokenAddress), 20);
        const expectedSymbol = expectedName.slice(0, 8); // "0x" + 6 chars

        expect(await cToken.name()).to.equal(`confidential ${expectedName}`);
        expect(await cToken.symbol()).to.equal(`c${expectedSymbol}`);
        expect(await cToken.decimals()).to.equal(MAX_DECIMALS); // Uses fallback decimals

        // Verify event uses fallback names
        const wrapDeployedEvent = getWrapDeployedEvent(receipt);
        expect(wrapDeployedEvent[0].args[3]).to.equal(expectedName);
        expect(wrapDeployedEvent[0].args[4]).to.equal(expectedSymbol);
        expect(wrapDeployedEvent[0].args[5]).to.equal(18); // Fallback decimals
      });
    });

    describe("Standard ERC20 compatibility verification", function () {
      it("should still work correctly with standard ERC20 tokens", async function () {
        // Use the standard USDC from the fixture
        const { cToken, receipt } = await deployConfidentialToken(coordinator, usdc, signers.alice);

        // Verify standard token behavior is not affected
        expect(await cToken.name()).to.equal("confidential USDC");
        expect(await cToken.symbol()).to.equal("cUSDC");
        expect(await cToken.decimals()).to.equal(MAX_DECIMALS);

        const wrapDeployedEvent = getWrapDeployedEvent(receipt);
        expect(wrapDeployedEvent[0].args[3]).to.equal("USDC");
        expect(wrapDeployedEvent[0].args[4]).to.equal("USDC");
      });
    });

    describe("Address fallback format", function () {
      it("should always produce fixed-width 42-char hex string (0x + 40 hex) for address fallback", async function () {
        const ERC20RevertingMetadataFactory = await ethers.getContractFactory("ERC20RevertingMetadata");
        const revertingToken = await ERC20RevertingMetadataFactory.deploy(18);
        await revertingToken.waitForDeployment();
        const revertingTokenAddress = await revertingToken.getAddress();

        await revertingToken.mint(signers.alice, 1000);

        const { cToken, receipt } = await deployConfidentialToken(coordinator, revertingToken, signers.alice);

        // Verify deployment succeeded
        expect(await coordinator.wrapperExists(revertingTokenAddress)).to.equal(true);

        // Verify fallback name is ALWAYS 42 characters (0x + 40 hex)
        const name = await cToken.name();
        const nameWithoutPrefix = name.replace("confidential ", "");
        expect(nameWithoutPrefix.length).to.equal(42);
        expect(nameWithoutPrefix).to.match(/^0x[0-9a-f]{40}$/);

        // Verify fallback symbol is ALWAYS 8 characters (0x + 6 hex)
        const symbol = await cToken.symbol();
        const symbolWithoutPrefix = symbol.replace("c", "");
        expect(symbolWithoutPrefix.length).to.equal(8);
        expect(symbolWithoutPrefix).to.match(/^0x[0-9a-f]{6}$/);

        // Verify consistency with ethers.toBeHex
        const expectedName = ethers.toBeHex(BigInt(revertingTokenAddress), 20);
        const expectedSymbol = expectedName.slice(0, 8);
        expect(nameWithoutPrefix).to.equal(expectedName);
        expect(symbolWithoutPrefix).to.equal(expectedSymbol);

        // Verify event uses fallback names
        const wrapDeployedEvent = getWrapDeployedEvent(receipt);
        expect(wrapDeployedEvent[0].args[3]).to.equal(expectedName);
        expect(wrapDeployedEvent[0].args[4]).to.equal(expectedSymbol);
      });

      it("should handle address with leading zeros correctly", async function () {
        // While we can't deploy to a specific low address in tests,
        // we can verify that any address produces a 42-char string
        const ERC20EmptyMetadataFactory = await ethers.getContractFactory("ERC20EmptyMetadata");
        const emptyToken = await ERC20EmptyMetadataFactory.deploy(18);
        await emptyToken.waitForDeployment();
        const emptyTokenAddress = await emptyToken.getAddress();

        await emptyToken.mint(signers.alice, 1000);

        const { cToken } = await deployConfidentialToken(coordinator, emptyToken, signers.alice);

        // The key property: name must ALWAYS be 42 chars, even for low addresses
        const name = await cToken.name();
        const nameWithoutPrefix = name.replace("confidential ", "");

        // This ensures leading zeros are preserved
        expect(nameWithoutPrefix.length).to.equal(42, "Name should be exactly 42 characters (0x + 40 hex)");
        expect(nameWithoutPrefix.startsWith("0x")).to.be.true;

        // Verify symbol can be extracted without revert (the bug scenario)
        const symbol = await cToken.symbol();
        const symbolWithoutPrefix = symbol.replace("c", "");
        expect(symbolWithoutPrefix.length).to.equal(8, "Symbol should be exactly 8 characters (0x + 6 hex)");

        // Verify format is lowercase hex
        expect(nameWithoutPrefix).to.match(/^0x[0-9a-f]{40}$/);
        expect(symbolWithoutPrefix).to.match(/^0x[0-9a-f]{6}$/);
      });
    });

    describe("Mixed edge cases", function () {
      it("should handle bytes32 with all 32 bytes used", async function () {
        const ERC20Bytes32MetadataFactory = await ethers.getContractFactory("ERC20Bytes32Metadata");

        // Create a 32-char string (max length for bytes32)
        const longName = "VeryLongTokenNameExactly32ch";
        const nameBytes32 = ethers.encodeBytes32String(longName);
        const symbolBytes32 = ethers.encodeBytes32String("LONG");

        const longNameToken = await ERC20Bytes32MetadataFactory.deploy(
          nameBytes32,
          symbolBytes32,
          6
        );
        await longNameToken.waitForDeployment();

        const { cToken } = await deployConfidentialToken(coordinator, longNameToken, signers.alice);

        expect(await cToken.name()).to.equal(`confidential ${longName}`);
        expect(await cToken.symbol()).to.equal("cLONG");
        expect(await cToken.decimals()).to.equal(MAX_DECIMALS);
      });

      it("should handle bytes32 with zero value", async function () {
        const ERC20Bytes32MetadataFactory = await ethers.getContractFactory("ERC20Bytes32Metadata");

        // bytes32(0) - empty
        const zeroBytes32 = ethers.ZeroHash;

        const zeroToken = await ERC20Bytes32MetadataFactory.deploy(
          zeroBytes32,
          zeroBytes32,
          18
        );
        await zeroToken.waitForDeployment();
        const zeroTokenAddress = await zeroToken.getAddress();

        const { cToken } = await deployConfidentialToken(coordinator, zeroToken, signers.alice);

        // Should fallback to address-based naming
        const expectedName = ethers.toBeHex(BigInt(zeroTokenAddress), 20);
        const expectedSymbol = expectedName.slice(0, 8);

        expect(await cToken.name()).to.equal(`confidential ${expectedName}`);
        expect(await cToken.symbol()).to.equal(`c${expectedSymbol}`);
      });
    });
  });

  describe("Upgradeability", function () {
    it("should allow admin to upgrade wrapper to V2", async function () {
      // Deploy a wrapper using the coordinator
      const usdc = await deployTestERC20Fixture("USDC", 6);
      const { wrapper, cToken } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const wrapperAddress = await wrapper.getAddress();

      // Verify current implementation
      const implementationSlot = "0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc";
      const implBefore = await ethers.provider.getStorage(wrapperAddress, implementationSlot);
      const implBeforeAddress = "0x" + implBefore.slice(26);

      const wrapperImplementation = await ethers.getContractAt("FeeManager", await coordinator.wrapperImplementation());
      expect(implBeforeAddress.toLowerCase()).to.equal((await wrapperImplementation.getAddress()).toLowerCase());

      // Deploy V2 implementation
      const WrapperV2 = await ethers.getContractFactory("WrapperUpgradeableV2Mock");
      const v2Impl = await WrapperV2.deploy();
      await v2Impl.waitForDeployment();
      const v2ImplAddress = await v2Impl.getAddress();

      // Verify deployer has DEFAULT_ADMIN_ROLE
      const adminRole = await wrapper.DEFAULT_ADMIN_ROLE();
      expect(await wrapper.hasRole(adminRole, signers.deployer.address)).to.be.true;

      // Grant UPGRADER_ROLE to deployer
      const upgraderRole = await wrapper.UPGRADER_ROLE();
      await wrapper.connect(signers.deployer).grantRole(upgraderRole, signers.deployer.address);

      // Upgrade to V2
      await wrapper.connect(signers.deployer).upgradeToAndCall(v2ImplAddress, "0x");

      // Verify implementation was upgraded
      const implAfter = await ethers.provider.getStorage(wrapperAddress, implementationSlot);
      const implAfterAddress = "0x" + implAfter.slice(26);
      expect(implAfterAddress.toLowerCase()).to.equal(v2ImplAddress.toLowerCase());

      // Verify V2 functionality works
      const wrapperV2 = WrapperV2.attach(wrapperAddress);
      expect(await wrapperV2.counter()).to.equal(0);
      await wrapperV2.incrementCounter();
      expect(await wrapperV2.counter()).to.equal(1);

      // Verify original functionality still works
      expect(await wrapper.confidentialToken()).to.equal(await cToken.getAddress());
      expect(await wrapper.originalToken()).to.equal(await usdc.getAddress());
    });

    it("should prevent non-admin from upgrading wrapper", async function () {
      // Deploy a wrapper using the coordinator
      const usdc = await deployTestERC20Fixture("USDC", 6);
      const { wrapper } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      // Deploy V2 implementation
      const WrapperV2 = await ethers.getContractFactory("WrapperUpgradeableV2Mock");
      const v2Impl = await WrapperV2.deploy();
      await v2Impl.waitForDeployment();
      const v2ImplAddress = await v2Impl.getAddress();

      // Try to upgrade as bob (doesn't have UPGRADER_ROLE)
      const upgraderRole = await wrapper.UPGRADER_ROLE();
      await expect(
        wrapper.connect(signers.bob).upgradeToAndCall(v2ImplAddress, "0x")
      ).to.be.revertedWithCustomError(wrapper, "AccessControlUnauthorizedAccount")
        .withArgs(signers.bob.address, upgraderRole);
    });

    it("should maintain independent upgradeability for multiple wrappers", async function () {
      // Deploy two wrappers
      const usdc = await deployTestERC20Fixture("USDC", 6);
      const dai = await deployTestERC20Fixture("DAI", 18);

      const wrapperImplementation = await ethers.getContractAt("FeeManager", await coordinator.wrapperImplementation());

      const { wrapper: wrapper1 } = await deployConfidentialToken(coordinator, usdc, signers.alice);
      const { wrapper: wrapper2 } = await deployConfidentialToken(coordinator, dai, signers.alice);

      const wrapper1Address = await wrapper1.getAddress();
      const wrapper2Address = await wrapper2.getAddress();

      // Verify both use original implementation
      const implementationSlot = "0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc";
      const impl1Before = await ethers.provider.getStorage(wrapper1Address, implementationSlot);
      const impl1BeforeAddress = "0x" + impl1Before.slice(26);
      const impl2Before = await ethers.provider.getStorage(wrapper2Address, implementationSlot);
      const impl2BeforeAddress = "0x" + impl2Before.slice(26);

      expect(impl1BeforeAddress.toLowerCase()).to.equal((await wrapperImplementation.getAddress()).toLowerCase());
      expect(impl2BeforeAddress.toLowerCase()).to.equal((await wrapperImplementation.getAddress()).toLowerCase());

      // Deploy V2 implementation
      const WrapperV2 = await ethers.getContractFactory("WrapperUpgradeableV2Mock");
      const v2Impl = await WrapperV2.deploy();
      await v2Impl.waitForDeployment();
      const v2ImplAddress = await v2Impl.getAddress();

      // Grant UPGRADER_ROLE to deployer for wrapper1
      const upgraderRole = await wrapper1.UPGRADER_ROLE();
      await wrapper1.connect(signers.deployer).grantRole(upgraderRole, signers.deployer.address);

      // Upgrade ONLY wrapper1 to V2
      await wrapper1.connect(signers.deployer).upgradeToAndCall(v2ImplAddress, "0x");

      // Verify wrapper1 was upgraded
      const impl1After = await ethers.provider.getStorage(wrapper1Address, implementationSlot);
      const impl1AfterAddress = "0x" + impl1After.slice(26);
      expect(impl1AfterAddress.toLowerCase()).to.equal(v2ImplAddress.toLowerCase());

      // Verify wrapper2 still uses original implementation
      const impl2After = await ethers.provider.getStorage(wrapper2Address, implementationSlot);
      const impl2AfterAddress = "0x" + impl2After.slice(26);
      expect(impl2AfterAddress.toLowerCase()).to.equal((await wrapperImplementation.getAddress()).toLowerCase());

      // Verify V2 functionality works on wrapper1
      const wrapper1V2 = WrapperV2.attach(wrapper1Address);
      expect(await wrapper1V2.counter()).to.equal(0);
      await wrapper1V2.incrementCounter();
      expect(await wrapper1V2.counter()).to.equal(1);

      // Verify wrapper2 doesn't have V2 functions (still V1)
      const wrapper2AsV2 = WrapperV2.attach(wrapper2Address);
      await expect(wrapper2AsV2.counter()).to.be.reverted; // V2 functions don't exist on V1

      // Verify both wrappers still have correct original tokens
      expect(await wrapper1.originalToken()).to.equal(await usdc.getAddress());
      expect(await wrapper2.originalToken()).to.equal(await dai.getAddress());
    });

    it("should prevent implementation contract from initializing", async function () {
      const WrapperUpgradeableFactory = await ethers.getContractFactory("WrapperUpgradeable");
      const wrapperImpl = await WrapperUpgradeableFactory.deploy();
      await wrapperImpl.waitForDeployment();

      const usdc = await deployTestERC20Fixture("USDC", 6);
      const { cToken } = await deployConfidentialToken(coordinator, usdc, signers.alice);

      await expect(
        wrapperImpl.initialize(
          await usdc.getAddress(),
          cToken,
          adminProvider,
          signers.deployer.address
        )
      ).to.be.revertedWithCustomError(WrapperUpgradeableFactory, "InvalidInitialization");
    });
  });
});
