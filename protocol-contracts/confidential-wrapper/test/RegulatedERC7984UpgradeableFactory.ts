import { ethers } from "hardhat";
import { expect } from "chai";
import { getSigners, Signers } from "./signers";
import { RegulatedERC7984UpgradeableFactory, AdminProvider, RegulatedERC7984Upgradeable, RegulatedERC7984UpgradeableV2Mock } from "../types";
import { deployConfidentialTokenFactoryFixture, deployAdminProviderFixture } from "./fixtures";

describe("RegulatedERC7984UpgradeableFactory", function () {
  let signers: Signers;
  let confidentialTokenFactory: RegulatedERC7984UpgradeableFactory;
  let adminProvider: AdminProvider;
  let implementation: string;

  before(async function () {
    signers = await getSigners();
  });

  beforeEach(async function () {
    const { confidentialTokenFactory: deployedFactory } = await deployConfidentialTokenFactoryFixture();
    confidentialTokenFactory = deployedFactory;

    const { adminProvider: deployedProvider } = await deployAdminProviderFixture(signers);
    adminProvider = deployedProvider;

    // Deploy canonical implementation for testing
    const RegulatedERC7984Impl = await ethers.getContractFactory("RegulatedERC7984Upgradeable");
    const implContract = await RegulatedERC7984Impl.deploy();
    await implContract.waitForDeployment();
    implementation = await implContract.getAddress();
  });

  describe("deployConfidentialToken", function () {
    it("should prevent non-owner from calling deployConfidentialToken", async function () {
      await expect(
        confidentialTokenFactory.connect(signers.alice).deployConfidentialToken(
          implementation,
          "Confidential USDC",
          "cUSDC",
          6,
          1,
          ethers.ZeroAddress,
          await adminProvider.getAddress(),
          signers.alice.address, // admin
          signers.alice.address  // wrapperSetter
        )
      ).to.be.revertedWithCustomError(confidentialTokenFactory, "OwnableUnauthorizedAccount")
        .withArgs(signers.alice.address);
    });

    it("should allow owner to call deployConfidentialToken", async function () {
      const tx = await confidentialTokenFactory.deployConfidentialToken(
        implementation,
        "Confidential USDC",
        "cUSDC",
        6,
        1,
        ethers.ZeroAddress,
        await adminProvider.getAddress(),
        signers.deployer.address, // admin
        signers.deployer.address  // wrapperSetter
      );

      const receipt = await tx.wait();
      expect(receipt?.status).to.equal(1);
    });

    it("should emit ConfidentialTokenDeployed event when deploying", async function () {
      const name = "Confidential USDC";
      const symbol = "cUSDC";
      const decimals = 6;
      const underlying = ethers.ZeroAddress;

      await expect(
        confidentialTokenFactory.deployConfidentialToken(
          implementation,
          name,
          symbol,
          decimals,
          1,
          underlying,
          await adminProvider.getAddress(),
          signers.deployer.address,
          signers.deployer.address
        )
      ).to.emit(confidentialTokenFactory, "ConfidentialTokenDeployed")
        .withArgs(
          implementation,
          (confidentialToken: string) => ethers.isAddress(confidentialToken), // Check it's a valid address
          name,
          symbol,
          decimals,
          underlying
        );
    });

    it("should reuse the same implementation for multiple deployments", async function () {
      // Deploy first token - use staticCall to get return value
      const token1Address = await confidentialTokenFactory.deployConfidentialToken.staticCall(
        implementation,
        "Confidential USDC",
        "cUSDC",
        6,
        1,
        ethers.ZeroAddress,
        await adminProvider.getAddress(),
        signers.deployer.address, // admin
        signers.deployer.address  // wrapperSetter
      );
      await confidentialTokenFactory.deployConfidentialToken(
        implementation,
        "Confidential USDC",
        "cUSDC",
        6,
        1,
        ethers.ZeroAddress,
        await adminProvider.getAddress(),
        signers.deployer.address, // admin
        signers.deployer.address  // wrapperSetter
      );

      // Deploy second token - use staticCall to get return value
      const token2Address = await confidentialTokenFactory.deployConfidentialToken.staticCall(
        implementation,
        "Confidential DAI",
        "cDAI",
        18,
        1,
        ethers.ZeroAddress,
        await adminProvider.getAddress(),
        signers.deployer.address, // admin
        signers.deployer.address  // wrapperSetter
      );
      await confidentialTokenFactory.deployConfidentialToken(
        implementation,
        "Confidential DAI",
        "cDAI",
        18,
        1,
        ethers.ZeroAddress,
        await adminProvider.getAddress(),
        signers.deployer.address, // admin
        signers.deployer.address  // wrapperSetter
      );

      // Verify proxies are different
      expect(token1Address).to.not.equal(token2Address);

      // Get implementation addresses using ERC1967 storage slot
      const implementationSlot = "0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc";
      const impl1 = await ethers.provider.getStorage(token1Address, implementationSlot);
      const impl2 = await ethers.provider.getStorage(token2Address, implementationSlot);

      // Convert to addresses and verify they match
      const impl1Address = "0x" + impl1.slice(26);
      const impl2Address = "0x" + impl2.slice(26);

      expect(impl1Address.toLowerCase()).to.equal(implementation.toLowerCase());
      expect(impl2Address.toLowerCase()).to.equal(implementation.toLowerCase());
    });

    it("should deploy functional tokens with shared implementation", async function () {
      // Deploy two tokens with the same implementation using staticCall
      const token1Address = await confidentialTokenFactory.deployConfidentialToken.staticCall(
        implementation,
        "Confidential USDC",
        "cUSDC",
        6,
        1,
        ethers.ZeroAddress,
        await adminProvider.getAddress(),
        signers.deployer.address, // admin
        signers.deployer.address  // wrapperSetter
      );
      await confidentialTokenFactory.deployConfidentialToken(
        implementation,
        "Confidential USDC",
        "cUSDC",
        6,
        1,
        ethers.ZeroAddress,
        await adminProvider.getAddress(),
        signers.deployer.address, // admin
        signers.deployer.address  // wrapperSetter
      );

      const token2Address = await confidentialTokenFactory.deployConfidentialToken.staticCall(
        implementation,
        "Confidential DAI",
        "cDAI",
        18,
        1,
        ethers.ZeroAddress,
        await adminProvider.getAddress(),
        signers.deployer.address, // admin
        signers.deployer.address  // wrapperSetter
      );
      await confidentialTokenFactory.deployConfidentialToken(
        implementation,
        "Confidential DAI",
        "cDAI",
        18,
        1,
        ethers.ZeroAddress,
        await adminProvider.getAddress(),
        signers.deployer.address, // admin
        signers.deployer.address  // wrapperSetter
      );

      // Get contract instances
      const RegulatedERC7984 = await ethers.getContractFactory("RegulatedERC7984Upgradeable");
      const token1 = RegulatedERC7984.attach(token1Address) as RegulatedERC7984Upgradeable;
      const token2 = RegulatedERC7984.attach(token2Address) as RegulatedERC7984Upgradeable;

      // Verify tokens have correct metadata
      expect(await token1.name()).to.equal("Confidential USDC");
      expect(await token1.symbol()).to.equal("cUSDC");
      expect(await token1.decimals()).to.equal(6);

      expect(await token2.name()).to.equal("Confidential DAI");
      expect(await token2.symbol()).to.equal("cDAI");
      expect(await token2.decimals()).to.equal(18);

      // Verify underlying addresses
      expect(await token1.underlying()).to.equal(ethers.ZeroAddress);
      expect(await token2.underlying()).to.equal(ethers.ZeroAddress);
    });
  });

  describe("Individual Token Upgrades (UUPS)", function () {
    it("should allow upgrading individual tokens independently", async function () {
      // Deploy two tokens with same implementation
      const adminProviderOwner = await adminProvider.owner();

      const token1Address = await confidentialTokenFactory.deployConfidentialToken.staticCall(
        implementation,
        "Confidential USDC",
        "cUSDC",
        6,
        1,
        ethers.ZeroAddress,
        await adminProvider.getAddress(),
        adminProviderOwner, // admin goes to adminProvider owner
        signers.deployer.address  // wrapperSetter
      );
      await confidentialTokenFactory.deployConfidentialToken(
        implementation,
        "Confidential USDC",
        "cUSDC",
        6,
        1,
        ethers.ZeroAddress,
        await adminProvider.getAddress(),
        adminProviderOwner, // admin goes to adminProvider owner
        signers.deployer.address  // wrapperSetter
      );

      const token2Address = await confidentialTokenFactory.deployConfidentialToken.staticCall(
        implementation,
        "Confidential DAI",
        "cDAI",
        18,
        1,
        ethers.ZeroAddress,
        await adminProvider.getAddress(),
        adminProviderOwner, // admin goes to adminProvider owner
        signers.deployer.address  // wrapperSetter
      );
      await confidentialTokenFactory.deployConfidentialToken(
        implementation,
        "Confidential DAI",
        "cDAI",
        18,
        1,
        ethers.ZeroAddress,
        await adminProvider.getAddress(),
        adminProviderOwner, // admin goes to adminProvider owner
        signers.deployer.address  // wrapperSetter
      );

      // Get token contracts
      const RegulatedERC7984 = await ethers.getContractFactory("RegulatedERC7984Upgradeable");
      const token1 = RegulatedERC7984.attach(token1Address) as RegulatedERC7984Upgradeable;
      const token2 = RegulatedERC7984.attach(token2Address) as RegulatedERC7984Upgradeable;

      // Verify both use original implementation
      const implementationSlot = "0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc";
      const impl1Before = await ethers.provider.getStorage(token1Address, implementationSlot);
      const impl1BeforeAddress = "0x" + impl1Before.slice(26);
      const impl2Before = await ethers.provider.getStorage(token2Address, implementationSlot);
      const impl2BeforeAddress = "0x" + impl2Before.slice(26);

      expect(impl1BeforeAddress.toLowerCase()).to.equal(implementation.toLowerCase());
      expect(impl2BeforeAddress.toLowerCase()).to.equal(implementation.toLowerCase());

      // Deploy V2 implementation
      const RegulatedERC7984V2 = await ethers.getContractFactory("RegulatedERC7984UpgradeableV2Mock");
      const v2Impl = await RegulatedERC7984V2.deploy();
      await v2Impl.waitForDeployment();
      const v2ImplAddress = await v2Impl.getAddress();

      // Grant UPGRADER_ROLE to alice for testing
      await ethers.provider.send("hardhat_impersonateAccount", [adminProviderOwner]);
      const adminSigner = await ethers.getSigner(adminProviderOwner);
      const UPGRADER_ROLE = await token1.UPGRADER_ROLE();
      await token1.connect(adminSigner).grantRole(UPGRADER_ROLE, signers.alice.address);
      await ethers.provider.send("hardhat_stopImpersonatingAccount", [adminProviderOwner]);

      // Upgrade ONLY token1 to V2 (as alice with UPGRADER_ROLE)
      await token1.connect(signers.alice).upgradeToAndCall(v2ImplAddress, "0x");

      // Verify token1 was upgraded
      const impl1After = await ethers.provider.getStorage(token1Address, implementationSlot);
      const impl1AfterAddress = "0x" + impl1After.slice(26);
      expect(impl1AfterAddress.toLowerCase()).to.equal(v2ImplAddress.toLowerCase());

      // Verify token2 still uses original implementation
      const impl2After = await ethers.provider.getStorage(token2Address, implementationSlot);
      const impl2AfterAddress = "0x" + impl2After.slice(26);
      expect(impl2AfterAddress.toLowerCase()).to.equal(implementation.toLowerCase());

      // Verify V2 functionality works on token1
      const token1V2 = RegulatedERC7984V2.attach(token1Address) as RegulatedERC7984UpgradeableV2Mock;
      expect(await token1V2.counter()).to.equal(0);
      await token1V2.incrementCounter();
      expect(await token1V2.counter()).to.equal(1);

      // Verify token2 doesn't have V2 functions (still V1)
      const token2AsV2 = RegulatedERC7984V2.attach(token2Address) as RegulatedERC7984UpgradeableV2Mock;
      await expect(token2AsV2.counter()).to.be.reverted; // V2 functions don't exist on V1

      // Verify both tokens still have correct metadata
      expect(await token1.name()).to.equal("Confidential USDC");
      expect(await token2.name()).to.equal("Confidential DAI");
    });
  });

  describe("Ownership Management", function () {
    it("should support two-step ownership transfer", async function () {
      await confidentialTokenFactory.transferOwnership(signers.alice.address);
      
      expect(await confidentialTokenFactory.pendingOwner()).to.equal(signers.alice.address);
      
      expect(await confidentialTokenFactory.owner()).to.equal(signers.deployer.address);
      
      await confidentialTokenFactory.connect(signers.alice).acceptOwnership();
      
      expect(await confidentialTokenFactory.owner()).to.equal(signers.alice.address);
      expect(await confidentialTokenFactory.pendingOwner()).to.equal(ethers.ZeroAddress);
    });

    it("should prevent non-pending owner from accepting ownership", async function () {
      await confidentialTokenFactory.transferOwnership(signers.alice.address);
      
      await expect(
        confidentialTokenFactory.connect(signers.bob).acceptOwnership()
      ).to.be.revertedWithCustomError(confidentialTokenFactory, "OwnableUnauthorizedAccount")
        .withArgs(signers.bob.address);
    });

    it("should allow new owner to deploy after ownership transfer", async function () {
      await confidentialTokenFactory.transferOwnership(signers.alice.address);
      await confidentialTokenFactory.connect(signers.alice).acceptOwnership();

      const tx = await confidentialTokenFactory.connect(signers.alice).deployConfidentialToken(
        implementation,
        "Alice Token",
        "ALICE",
        18,
        1,
        ethers.ZeroAddress,
        await adminProvider.getAddress(),
        signers.alice.address, // admin
        signers.alice.address  // wrapperSetter
      );

      const receipt = await tx.wait();
      expect(receipt?.status).to.equal(1);

      await expect(
        confidentialTokenFactory.connect(signers.deployer).deployConfidentialToken(
          implementation,
          "Deployer Token",
          "DEPLOY",
          18,
          1,
          ethers.ZeroAddress,
          await adminProvider.getAddress(),
          signers.deployer.address, // admin
          signers.deployer.address  // wrapperSetter
        )
      ).to.be.revertedWithCustomError(confidentialTokenFactory, "OwnableUnauthorizedAccount")
        .withArgs(signers.deployer.address);
    });
  });
});
