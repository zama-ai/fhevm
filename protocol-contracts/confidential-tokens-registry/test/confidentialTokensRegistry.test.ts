import { expect } from 'chai';
import { ethers } from 'hardhat';
import { loadFixture } from '@nomicfoundation/hardhat-network-helpers';

import { getRegistryFixture } from './utils/loadFixture';
import { createRandomAddress } from './utils/inputs';

describe('ConfidentialTokensRegistry', function () {
  beforeEach(async function () {
    const fixture = await loadFixture(getRegistryFixture);

    // Owner is defined as account[1] in .env.example
    const [deployer, owner, anyone] = await ethers.getSigners();

    const token1 = createRandomAddress();
    const token2 = createRandomAddress();
    const confidentialToken1 = createRandomAddress();
    const confidentialToken2 = createRandomAddress();

    Object.assign(this, {
      deployer,
      owner,
      anyone,
      token1,
      token2,
      confidentialToken1,
      confidentialToken2,
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
      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.equal(ethers.ZeroAddress);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.equal(ethers.ZeroAddress);

      await expect(this.registry.connect(this.owner).registerConfidentialToken(this.token1, this.confidentialToken1))
        .to.emit(this.registry, 'ConfidentialTokenRegistered')
        .withArgs(this.token1, this.confidentialToken1);

      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.equal(this.confidentialToken1);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.equal(this.token1);
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

    it('should revert if token is already associated with a confidential token', async function () {
      await this.registry.connect(this.owner).registerConfidentialToken(this.token1, this.confidentialToken1);

      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.equal(this.confidentialToken1);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.equal(this.token1);

      await expect(this.registry.connect(this.owner).registerConfidentialToken(this.token1, this.confidentialToken2))
        .to.be.revertedWithCustomError(this.registry, 'TokenAlreadyAssociatedWithConfidentialToken')
        .withArgs(this.token1, this.confidentialToken1);
    });

    it('should revert if confidential token is already associated with a token', async function () {
      await this.registry.connect(this.owner).registerConfidentialToken(this.token1, this.confidentialToken1);

      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.equal(this.confidentialToken1);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.equal(this.token1);

      await expect(this.registry.connect(this.owner).registerConfidentialToken(this.token2, this.confidentialToken1))
        .to.be.revertedWithCustomError(this.registry, 'ConfidentialTokenAlreadyAssociatedWithToken')
        .withArgs(this.confidentialToken1, this.token1);
    });

    it('should revert if confidential token is revoked', async function () {
      await this.registry.connect(this.owner).registerConfidentialToken(this.token1, this.confidentialToken1);
      await this.registry.connect(this.owner).revokeConfidentialToken(this.confidentialToken1);

      expect(await this.registry.isConfidentialTokenRevoked(this.confidentialToken1)).to.equal(true);

      await expect(this.registry.connect(this.owner).registerConfidentialToken(this.token2, this.confidentialToken1))
        .to.be.revertedWithCustomError(this.registry, 'RevokedConfidentialToken')
        .withArgs(this.confidentialToken1);
    });
  });

  describe('revokeConfidentialToken', function () {
    beforeEach(async function () {
      await this.registry.connect(this.owner).registerConfidentialToken(this.token1, this.confidentialToken1);
    });

    it('should revoke a confidential token', async function () {
      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.equal(this.confidentialToken1);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.equal(this.token1);

      expect(await this.registry.isConfidentialTokenRevoked(this.confidentialToken1)).to.equal(false);

      await expect(this.registry.connect(this.owner).revokeConfidentialToken(this.confidentialToken1))
        .to.emit(this.registry, 'ConfidentialTokenRevoked')
        .withArgs(this.token1, this.confidentialToken1);

      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.equal(ethers.ZeroAddress);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.equal(ethers.ZeroAddress);

      expect(await this.registry.isConfidentialTokenRevoked(this.confidentialToken1)).to.equal(true);
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
      expect(await this.registry.getTokenAddress(this.confidentialToken2)).to.equal(ethers.ZeroAddress);

      await expect(this.registry.connect(this.owner).revokeConfidentialToken(this.confidentialToken2))
        .to.be.revertedWithCustomError(this.registry, 'NoTokenAssociatedWithConfidentialToken')
        .withArgs(this.confidentialToken2);
    });
  });

  describe('reinstateConfidentialToken', function () {
    beforeEach(async function () {
      await this.registry.connect(this.owner).registerConfidentialToken(this.token1, this.confidentialToken1);
      await this.registry.connect(this.owner).revokeConfidentialToken(this.confidentialToken1);
    });

    it('should reinstate a revoked confidential token', async function () {
      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.equal(ethers.ZeroAddress);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.equal(ethers.ZeroAddress);

      expect(await this.registry.isConfidentialTokenRevoked(this.confidentialToken1)).to.equal(true);

      await expect(this.registry.connect(this.owner).reinstateConfidentialToken(this.confidentialToken1))
        .to.emit(this.registry, 'ConfidentialTokenReinstated')
        .withArgs(this.confidentialToken1);

      expect(await this.registry.isConfidentialTokenRevoked(this.confidentialToken1)).to.equal(false);

      // Make sure the association with the token is not restored
      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.equal(ethers.ZeroAddress);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.equal(ethers.ZeroAddress);
    });

    it('should allow re-registration with the same token after reinstatement', async function () {
      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.equal(ethers.ZeroAddress);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.equal(ethers.ZeroAddress);

      await this.registry.connect(this.owner).reinstateConfidentialToken(this.confidentialToken1);

      await expect(this.registry.connect(this.owner).registerConfidentialToken(this.token1, this.confidentialToken1))
        .to.emit(this.registry, 'ConfidentialTokenRegistered')
        .withArgs(this.token1, this.confidentialToken1);

      expect(await this.registry.getConfidentialTokenAddress(this.token1)).to.equal(this.confidentialToken1);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.equal(this.token1);
    });

    it('should allow re-registration with a different token after reinstatement', async function () {
      expect(await this.registry.getConfidentialTokenAddress(this.token2)).to.equal(ethers.ZeroAddress);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.equal(ethers.ZeroAddress);

      await this.registry.connect(this.owner).reinstateConfidentialToken(this.confidentialToken1);

      await expect(this.registry.connect(this.owner).registerConfidentialToken(this.token2, this.confidentialToken1))
        .to.emit(this.registry, 'ConfidentialTokenRegistered')
        .withArgs(this.token2, this.confidentialToken1);

      expect(await this.registry.getConfidentialTokenAddress(this.token2)).to.equal(this.confidentialToken1);
      expect(await this.registry.getTokenAddress(this.confidentialToken1)).to.equal(this.token2);
    });

    it('should revert if not authorized', async function () {
      await expect(
        this.registry.connect(this.anyone).reinstateConfidentialToken(this.confidentialToken1),
      ).to.be.revertedWithCustomError(this.registry, 'OwnableUnauthorizedAccount');
    });

    it('should revert if confidential token address is zero', async function () {
      await expect(
        this.registry.connect(this.owner).reinstateConfidentialToken(ethers.ZeroAddress),
      ).to.be.revertedWithCustomError(this.registry, 'ConfidentialTokenZeroAddress');
    });

    it('should revert if confidential token is not revoked', async function () {
      await this.registry.connect(this.owner).reinstateConfidentialToken(this.confidentialToken1);

      await expect(this.registry.connect(this.owner).reinstateConfidentialToken(this.confidentialToken1))
        .to.be.revertedWithCustomError(this.registry, 'ConfidentialTokenNotRevoked')
        .withArgs(this.confidentialToken1);
    });
  });
});
