import { ethers } from "hardhat";
import { expect } from "chai";
import { getSigners, Signers } from "./signers";
import { WrapperUpgradeable, AdminProvider, RegulatedERC7984Upgradeable } from "../types";
import { deployAdminProviderFixture, deployConfidentialErc20Fixture } from "./fixtures";
import { ERC1967Proxy__factory } from "../types";

describe("WrapperUpgradeable - Initialize Validation", function () {
  let signers: Signers;
  let wrapperImplementation: WrapperUpgradeable;
  let adminProvider: AdminProvider;
  let cToken: RegulatedERC7984Upgradeable;

  before(async function () {
    signers = await getSigners();
  });

  beforeEach(async function () {
    // Deploy WrapperUpgradeable implementation
    const wrapperUpgradeableFactory = await ethers.getContractFactory("WrapperUpgradeable");
    wrapperImplementation = await wrapperUpgradeableFactory.deploy() as WrapperUpgradeable;
    await wrapperImplementation.waitForDeployment();

    // Deploy admin provider
    ({ adminProvider } = await deployAdminProviderFixture(signers));

    // Deploy confidential token
    ({ cErc20: cToken } = await deployConfidentialErc20Fixture(signers, adminProvider));
  });

  describe("Zero Address Validation", function () {
    it("should revert when confidentialToken_ is zero address", async function () {
      const originalToken = ethers.ZeroAddress; // ETH wrapper
      const implAddress = await wrapperImplementation.getAddress();

      // Create initialize call data with zero address for confidentialToken
      const initData = wrapperImplementation.interface.encodeFunctionData("initialize", [
        originalToken,
        ethers.ZeroAddress, // confidentialToken_ = zero address
        await adminProvider.getAddress(),
        signers.deployer.address
      ]);

      // Deploy proxy with initialization
      const proxyFactory = await ethers.getContractFactory("ERC1967Proxy");

      await expect(
        proxyFactory.deploy(implAddress, initData)
      ).to.be.revertedWithCustomError(wrapperImplementation, "ZeroAddressConfidentialToken");
    });

    it("should revert when deploymentCoordinator_ is zero address", async function () {
      const originalToken = ethers.ZeroAddress; // ETH wrapper
      const implAddress = await wrapperImplementation.getAddress();

      // Create initialize call data with zero address for adminProvider
      const initData = wrapperImplementation.interface.encodeFunctionData("initialize", [
        originalToken,
        await cToken.getAddress(),
        ethers.ZeroAddress, // deploymentCoordinator_ = zero address
        signers.deployer.address
      ]);

      // Deploy proxy with initialization
      const proxyFactory = await ethers.getContractFactory("ERC1967Proxy");

      await expect(
        proxyFactory.deploy(implAddress, initData)
      ).to.be.revertedWithCustomError(wrapperImplementation, "ZeroAddressDeploymentCoordinator");
    });

    it("should revert when both confidentialToken_ and adminProvider_ are zero addresses", async function () {
      const originalToken = ethers.ZeroAddress; // ETH wrapper
      const implAddress = await wrapperImplementation.getAddress();

      // Create initialize call data with zero addresses for both
      const initData = wrapperImplementation.interface.encodeFunctionData("initialize", [
        originalToken,
        ethers.ZeroAddress, // confidentialToken_ = zero address
        ethers.ZeroAddress, // adminProvider_ = zero address
        signers.deployer.address
      ]);

      // Deploy proxy with initialization
      const proxyFactory = await ethers.getContractFactory("ERC1967Proxy");

      // Should revert with the first check (confidentialToken)
      await expect(
        proxyFactory.deploy(implAddress, initData)
      ).to.be.revertedWithCustomError(wrapperImplementation, "ZeroAddressConfidentialToken");
    });

    it("should allow originalToken_ to be zero address (ETH wrapper)", async function () {
      const implAddress = await wrapperImplementation.getAddress();

      const coordinator = signers.deployer.address;

      // Create initialize call data with zero address for originalToken (valid for ETH)
      const initData = wrapperImplementation.interface.encodeFunctionData("initialize", [
        ethers.ZeroAddress, // originalToken_ = zero address (ETH)
        await cToken.getAddress(),
        coordinator,
        signers.deployer.address
      ]);

      // Deploy proxy with initialization
      const proxyFactory = await ethers.getContractFactory("ERC1967Proxy");
      const proxy = await proxyFactory.deploy(implAddress, initData);
      await proxy.waitForDeployment();

      // Attach wrapper interface to proxy
      const wrapper = await ethers.getContractAt("WrapperUpgradeable", await proxy.getAddress());

      // Verify initialization succeeded
      expect(await wrapper.originalToken()).to.equal(ethers.ZeroAddress);
      expect(await wrapper.confidentialToken()).to.equal(await cToken.getAddress());
      expect(await wrapper.deploymentCoordinator()).to.equal(coordinator);
    });
  });

  describe("Prevent Re-initialization", function () {
    it("should prevent calling initialize twice on the same proxy", async function () {
      const implAddress = await wrapperImplementation.getAddress();

      const initData = wrapperImplementation.interface.encodeFunctionData("initialize", [
        ethers.ZeroAddress,
        await cToken.getAddress(),
        await adminProvider.getAddress(),
        signers.deployer.address
      ]);

      const proxyFactory = await ethers.getContractFactory("ERC1967Proxy");
      const proxy = await proxyFactory.deploy(implAddress, initData);
      await proxy.waitForDeployment();

      const wrapper = await ethers.getContractAt("WrapperUpgradeable", await proxy.getAddress());

      // Try to initialize again
      await expect(
        wrapper.initialize(
          ethers.ZeroAddress,
          await cToken.getAddress(),
          await adminProvider.getAddress(),
          signers.deployer.address
        )
      ).to.be.revertedWithCustomError(wrapper, "InvalidInitialization");
    });
  });

  describe("Receiver Entry Getter", function () {
    it("should return empty ReceiverEntry for non-existent requestId", async function () {
      const implAddress = await wrapperImplementation.getAddress();

      const initData = wrapperImplementation.interface.encodeFunctionData("initialize", [
        ethers.ZeroAddress,
        await cToken.getAddress(),
        await adminProvider.getAddress(),
        signers.deployer.address
      ]);

      const proxyFactory = await ethers.getContractFactory("ERC1967Proxy");
      const proxy = await proxyFactory.deploy(implAddress, initData);
      await proxy.waitForDeployment();

      const wrapper = await ethers.getContractAt("WrapperUpgradeable", await proxy.getAddress());

      // Query non-existent requestId
      const receiverEntry = await wrapper.getReceiverEntry(999);

      // Verify all fields are zero/empty
      expect(receiverEntry.to).to.equal(ethers.ZeroAddress);
      expect(receiverEntry.refund).to.equal(ethers.ZeroAddress);
      expect(receiverEntry.callbackData).to.equal("0x");
      expect(receiverEntry.committedFeeBasisPoints).to.equal(0);
    });
  });
});
