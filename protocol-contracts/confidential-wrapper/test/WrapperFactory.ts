import { ethers } from "hardhat";
import { expect } from "chai";
import { getSigners, Signers } from "./signers";
import { WrapperFactory, AdminProvider, RegulatedERC7984Upgradeable, WrapperUpgradeable, DeploymentCoordinator } from "../types";
import { deployWrapperFactoryFixture, deployAdminProviderFixture, deployConfidentialErc20Fixture, deployTestERC20Fixture,  } from "./fixtures";

describe("WrapperFactory", function () {
  let signers: Signers;
  let wrapperFactory: WrapperFactory;
  let adminProvider: AdminProvider;
  let cUsdc: RegulatedERC7984Upgradeable;
  let wrapperImplementation: WrapperUpgradeable;

  before(async function () {
    signers = await getSigners();
  });

  beforeEach(async function () {
    const { wrapperFactory: deployedFactory } = await deployWrapperFactoryFixture();
    wrapperFactory = deployedFactory;

    const { adminProvider: deployedProvider } = await deployAdminProviderFixture(signers);
    adminProvider = deployedProvider;

    ({ cErc20: cUsdc } = await deployConfidentialErc20Fixture(signers));

    // Deploy WrapperUpgradeable implementation
    const wrapperUpgradeableFactory = await ethers.getContractFactory("WrapperUpgradeable");
    wrapperImplementation = await wrapperUpgradeableFactory.deploy() as WrapperUpgradeable;
    await wrapperImplementation.waitForDeployment();
  });

  describe("deployWrapper", function () {
    it("should prevent non-owner from calling deployWrapper", async function () {
      const originalToken = ethers.ZeroAddress; // ETH wrapper

      await expect(
        wrapperFactory.connect(signers.alice).deployWrapper(
          await wrapperImplementation.getAddress(),
          originalToken,
          await cUsdc.getAddress(),
          await adminProvider.getAddress(),
          signers.deployer.address
        )
      ).to.be.revertedWithCustomError(wrapperFactory, "OwnableUnauthorizedAccount")
        .withArgs(signers.alice.address);
    });

    it("should allow owner to call deployWrapper", async function () {
      const originalToken = ethers.ZeroAddress; // ETH wrapper

      const tx = await wrapperFactory.deployWrapper(
        await wrapperImplementation.getAddress(),
        originalToken,
        await cUsdc.getAddress(),
        await adminProvider.getAddress(),
        signers.deployer.address
      );

      const receipt = await tx.wait();
      expect(receipt?.status).to.equal(1);
    });

    it("should prevent non-owner from calling deployWrapper with ERC20 token", async function () {
      const testToken = await deployTestERC20Fixture("Test Token", 6);
      const testTokenAddress = await testToken.getAddress();

      await expect(
        wrapperFactory.connect(signers.bob).deployWrapper(
          await wrapperImplementation.getAddress(),
          testTokenAddress,
          await cUsdc.getAddress(),
          await adminProvider.getAddress(),
          signers.deployer.address
        )
      ).to.be.revertedWithCustomError(wrapperFactory, "OwnableUnauthorizedAccount")
        .withArgs(signers.bob.address);
    });

    describe("Parameter Validation", function () {
      it("should revert when implementation_ is zero address", async function () {
        const originalToken = ethers.ZeroAddress;

        await expect(
          wrapperFactory.deployWrapper(
            ethers.ZeroAddress, // implementation = zero address
            originalToken,
            await cUsdc.getAddress(),
            await adminProvider.getAddress(),
            signers.deployer.address
          )
        ).to.be.revertedWithCustomError(wrapperFactory, "ZeroAddressImplementation");
      });

      it("should revert when implementation_ is not a contract (EOA)", async function () {
        const originalToken = ethers.ZeroAddress;
        const eoaAddress = signers.alice.address; // EOA, not a contract

        await expect(
          wrapperFactory.deployWrapper(
            eoaAddress, // EOA address instead of contract
            originalToken,
            await cUsdc.getAddress(),
            await adminProvider.getAddress(),
            signers.deployer.address
          )
        ).to.be.revertedWithCustomError(wrapperFactory, "ImplementationNotContract");
      });

      it("should revert when admin_ is zero address", async function () {
        const originalToken = ethers.ZeroAddress;

        await expect(
          wrapperFactory.deployWrapper(
            await wrapperImplementation.getAddress(),
            originalToken,
            await cUsdc.getAddress(),
            await adminProvider.getAddress(),
            ethers.ZeroAddress // admin = zero address
          )
        ).to.be.revertedWithCustomError(wrapperFactory, "ZeroAddressAdmin");
      });

      it("should allow originalToken_ to be zero address (for ETH wrapper)", async function () {
        const tx = await wrapperFactory.deployWrapper(
          await wrapperImplementation.getAddress(),
          ethers.ZeroAddress, // ETH wrapper - valid
          await cUsdc.getAddress(),
          await adminProvider.getAddress(),
          signers.deployer.address
        );

        const receipt = await tx.wait();
        expect(receipt?.status).to.equal(1);
      });
    });

    describe("Event Emission", function () {
      it("should emit WrapperDeployed event with correct parameters for ETH wrapper", async function () {
        const originalToken = ethers.ZeroAddress;
        const implementationAddress = await wrapperImplementation.getAddress();
        const cUsdcAddress = await cUsdc.getAddress();
        const coordinator = signers.deployer.address;

        const tx = await wrapperFactory.deployWrapper(
          implementationAddress,
          originalToken,
          cUsdcAddress,
          coordinator,
          signers.deployer.address
        );

        const receipt = await tx.wait();
        const event = receipt?.logs.find(
          (log: any) => {
            try {
              const parsed = wrapperFactory.interface.parseLog({
                topics: log.topics as string[],
                data: log.data
              });
              return parsed?.name === "WrapperDeployed";
            } catch {
              return false;
            }
          }
        );

        expect(event).to.not.be.undefined;

        const parsedEvent = wrapperFactory.interface.parseLog({
          topics: event!.topics as string[],
          data: event!.data
        });

        expect(parsedEvent!.args.wrapper).to.be.properAddress;
        expect(parsedEvent!.args.originalToken).to.equal(originalToken);
        expect(parsedEvent!.args.confidentialToken).to.equal(cUsdcAddress);
        expect(parsedEvent!.args.implementation).to.equal(implementationAddress);
        expect(parsedEvent!.args.deploymentCoordinator).to.equal(coordinator);
        expect(parsedEvent!.args.admin).to.equal(signers.deployer.address);
      });

      it("should emit WrapperDeployed event with correct parameters for ERC20 wrapper", async function () {
        const testToken = await deployTestERC20Fixture("Test Token", 6);
        const testTokenAddress = await testToken.getAddress();
        const implementationAddress = await wrapperImplementation.getAddress();
        const cUsdcAddress = await cUsdc.getAddress();
        const coordinator = signers.deployer.address;

        const tx = await wrapperFactory.deployWrapper(
          implementationAddress,
          testTokenAddress,
          cUsdcAddress,
          coordinator,
          signers.alice.address // Use alice as admin for variety
        );

        const receipt = await tx.wait();
        const event = receipt?.logs.find(
          (log: any) => {
            try {
              const parsed = wrapperFactory.interface.parseLog({
                topics: log.topics as string[],
                data: log.data
              });
              return parsed?.name === "WrapperDeployed";
            } catch {
              return false;
            }
          }
        );

        expect(event).to.not.be.undefined;

        const parsedEvent = wrapperFactory.interface.parseLog({
          topics: event!.topics as string[],
          data: event!.data
        });

        expect(parsedEvent!.args.wrapper).to.be.properAddress;
        expect(parsedEvent!.args.originalToken).to.equal(testTokenAddress);
        expect(parsedEvent!.args.confidentialToken).to.equal(cUsdcAddress);
        expect(parsedEvent!.args.implementation).to.equal(implementationAddress);
        expect(parsedEvent!.args.deploymentCoordinator).to.equal(coordinator);
        expect(parsedEvent!.args.admin).to.equal(signers.alice.address);
      });
    });
  });

  describe("Ownership Management", function () {
    it("should support two-step ownership transfer", async function () {
      await wrapperFactory.transferOwnership(signers.alice.address);

      expect(await wrapperFactory.pendingOwner()).to.equal(signers.alice.address);

      expect(await wrapperFactory.owner()).to.equal(signers.deployer.address);

      await wrapperFactory.connect(signers.alice).acceptOwnership();

      expect(await wrapperFactory.owner()).to.equal(signers.alice.address);
      expect(await wrapperFactory.pendingOwner()).to.equal(ethers.ZeroAddress);
    });

    it("should prevent non-pending owner from accepting ownership", async function () {
      await wrapperFactory.transferOwnership(signers.alice.address);

      await expect(
        wrapperFactory.connect(signers.bob).acceptOwnership()
      ).to.be.revertedWithCustomError(wrapperFactory, "OwnableUnauthorizedAccount")
        .withArgs(signers.bob.address);
    });
  });
});
