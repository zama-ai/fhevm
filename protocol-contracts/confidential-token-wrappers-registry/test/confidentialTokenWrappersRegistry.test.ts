import { expect } from 'chai';
import { ethers } from 'hardhat';
import { loadFixture } from '@nomicfoundation/hardhat-network-helpers';

import { getRegistryFixture } from './utils/loadFixture';
import { createRandomAddress } from './utils/inputs';

describe('ConfidentialTokenWrappersRegistry', function () {
  before(async function () {
    // Deploy an ERC7984Mock
    async function deployERC7984Mock(label: string) {
      const ERC7984Mock = await ethers.getContractFactory('ERC7984Mock');
      const erc7984Mock = await ERC7984Mock.deploy();
      await erc7984Mock.waitForDeployment();
      console.log(`✅ ${label} deployed to:`, erc7984Mock.target);
      return erc7984Mock;
    }

    // Deploy ERC7984Mock1 and ERC7984Mock2 and assign to context
    const confidentialToken1 = (await deployERC7984Mock('ERC7984Mock1')).target;
    const confidentialToken2 = (await deployERC7984Mock('ERC7984Mock2')).target;
    Object.assign(this, { confidentialToken1, confidentialToken2 });
  });
  beforeEach(async function () {
    const fixture = await loadFixture(getRegistryFixture);

    // Owner is defined as account[1] in .env.example
    const [deployer, owner, anyone] = await ethers.getSigners();

    const token1 = createRandomAddress();
    const token2 = createRandomAddress();

    Object.assign(this, {
      deployer,
      owner,
      anyone,
      token1,
      token2,
      registry: fixture.registry,
    });
  });

  describe('Access Control', function () {
    it('should not upgrade if not authorized', async function () {
      const fakeContractAddress = createRandomAddress();
      await expect(
        this.registry.connect(this.anyone).upgradeToAndCall(fakeContractAddress, '0x'),
      ).to.be.revertedWithCustomError(this.registry, 'OwnableUnauthorizedAccount');
    });
  });

  describe('deployment configuration', function () {
    it('should have the correct initial owner', async function () {
      expect(await this.registry.owner()).to.equal(this.owner.address);
    });
  });

  describe('registerConfidentialToken', function () {
    it('should register a confidential token', async function () {
      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.deep.equal([false, ethers.ZeroAddress]);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.deep.equal([false, ethers.ZeroAddress]);
      expect(await this.registry.getTokenConfidentialTokenPairs()).to.deep.equal([]);

      await expect(this.registry.connect(this.owner).registerConfidentialToken(this.token1, this.confidentialToken1))
        .to.emit(this.registry, 'ConfidentialTokenRegistered')
        .withArgs(this.token1, this.confidentialToken1);

      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.deep.equal([
        false,
        this.confidentialToken1,
      ]);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.deep.equal([false, this.token1]);
      expect(await this.registry.getTokenConfidentialTokenPairs()).to.deep.equal([
        [this.token1, this.confidentialToken1, false],
      ]);
    });

    it('should revert if not authorized', async function () {
      await expect(
        this.registry.connect(this.anyone).registerConfidentialToken(this.token1, this.confidentialToken1),
      ).to.be.revertedWithCustomError(this.registry, 'OwnableUnauthorizedAccount');
    });

    it('should revert if token address is zero', async function () {
      await expect(
        this.registry.connect(this.owner).registerConfidentialToken(ethers.ZeroAddress, this.confidentialToken1),
      ).to.be.revertedWithCustomError(this.registry, 'TokenZeroAddress');
    });

    it('should revert if confidential token address is zero', async function () {
      await expect(
        this.registry.connect(this.owner).registerConfidentialToken(this.token1, ethers.ZeroAddress),
      ).to.be.revertedWithCustomError(this.registry, 'ConfidentialTokenZeroAddress');
    });

    it('should revert if confidential token address does not support ERC165', async function () {
      const fakeConfidentialTokenAddress = createRandomAddress();
      await expect(
        this.registry.connect(this.owner).registerConfidentialToken(this.token1, fakeConfidentialTokenAddress),
      ).to.be.revertedWithCustomError(this.registry, 'ConfidentialTokenDoesNotSupportERC165');
    });

    it('should revert if confidential token address does not support ERC7984 interface', async function () {
      const ERC165Mock = await ethers.getContractFactory('ERC165Mock');
      const erc165Mock = await ERC165Mock.deploy();
      await erc165Mock.waitForDeployment();
      console.log(`✅ ERC165Mock deployed to:`, erc165Mock.target);

      await expect(
        this.registry.connect(this.owner).registerConfidentialToken(this.token1, erc165Mock.target),
      ).to.be.revertedWithCustomError(this.registry, 'NotERC7984');
    });

    it('should revert if token is already associated with a confidential token', async function () {
      await this.registry.connect(this.owner).registerConfidentialToken(this.token1, this.confidentialToken1);

      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.deep.equal([
        false,
        this.confidentialToken1,
      ]);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.deep.equal([false, this.token1]);
      expect(await this.registry.getTokenConfidentialTokenPairs()).to.deep.equal([
        [this.token1, this.confidentialToken1, false],
      ]);

      await expect(this.registry.connect(this.owner).registerConfidentialToken(this.token1, this.confidentialToken2))
        .to.be.revertedWithCustomError(this.registry, 'TokenAlreadyAssociatedWithConfidentialToken')
        .withArgs(this.token1, this.confidentialToken1);
    });

    it('should revert if confidential token is already associated with a token', async function () {
      await this.registry.connect(this.owner).registerConfidentialToken(this.token1, this.confidentialToken1);

      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.deep.equal([
        false,
        this.confidentialToken1,
      ]);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.deep.equal([false, this.token1]);
      expect(await this.registry.getTokenConfidentialTokenPairs()).to.deep.equal([
        [this.token1, this.confidentialToken1, false],
      ]);

      await expect(this.registry.connect(this.owner).registerConfidentialToken(this.token2, this.confidentialToken1))
        .to.be.revertedWithCustomError(this.registry, 'ConfidentialTokenAlreadyAssociatedWithToken')
        .withArgs(this.confidentialToken1, this.token1);
    });
  });

  describe('revokeConfidentialToken', function () {
    beforeEach(async function () {
      await this.registry.connect(this.owner).registerConfidentialToken(this.token1, this.confidentialToken1);
    });

    it('should revoke a confidential token', async function () {
      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.deep.equal([
        false,
        this.confidentialToken1,
      ]);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.deep.equal([false, this.token1]);
      expect(await this.registry.getTokenConfidentialTokenPairs()).to.deep.equal([
        [this.token1, this.confidentialToken1, false],
      ]);

      expect(await this.registry.isConfidentialTokenRevoked(this.confidentialToken1)).to.equal(false);

      await expect(this.registry.connect(this.owner).revokeConfidentialToken(this.confidentialToken1))
        .to.emit(this.registry, 'ConfidentialTokenRevoked')
        .withArgs(this.token1, this.confidentialToken1);

      expect(await this.registry.isConfidentialTokenRevoked(this.confidentialToken1)).to.equal(true);

      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.deep.equal([
        true,
        this.confidentialToken1,
      ]);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.deep.equal([true, this.token1]);
      expect(await this.registry.getTokenConfidentialTokenPairs()).to.deep.equal([
        [this.token1, this.confidentialToken1, true],
      ]);
    });

    it('should revert if not authorized', async function () {
      await expect(
        this.registry.connect(this.anyone).revokeConfidentialToken(this.confidentialToken1),
      ).to.be.revertedWithCustomError(this.registry, 'OwnableUnauthorizedAccount');
    });

    it('should revert if confidential token address is zero', async function () {
      await expect(
        this.registry.connect(this.owner).revokeConfidentialToken(ethers.ZeroAddress),
      ).to.be.revertedWithCustomError(this.registry, 'ConfidentialTokenZeroAddress');
    });

    it('should revert if confidential token is already revoked', async function () {
      await this.registry.connect(this.owner).revokeConfidentialToken(this.confidentialToken1);

      expect(await this.registry.isConfidentialTokenRevoked(this.confidentialToken1)).to.equal(true);

      await expect(this.registry.connect(this.owner).revokeConfidentialToken(this.confidentialToken1))
        .to.be.revertedWithCustomError(this.registry, 'RevokedConfidentialToken')
        .withArgs(this.confidentialToken1);
    });

    it('should revert if no token is associated with the confidential token', async function () {
      expect(await this.registry.getTokenAddress(this.confidentialToken2)).to.deep.equal([false, ethers.ZeroAddress]);

      await expect(this.registry.connect(this.owner).revokeConfidentialToken(this.confidentialToken2))
        .to.be.revertedWithCustomError(this.registry, 'NoTokenAssociatedWithConfidentialToken')
        .withArgs(this.confidentialToken2);
    });
  });
});
