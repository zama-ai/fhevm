import { expect } from 'chai';
import { ethers } from 'hardhat';
import { loadFixture } from '@nomicfoundation/hardhat-network-helpers';
import { getRegistryFixture } from './utils/loadFixture';

describe('ConfidentialTokensRegistry', function () {
  beforeEach(async function () {
    const fixture = await loadFixture(getRegistryFixture);

    const [
      owner,
      newOwner,
      anyone,
      token1,
      token2,
      token3,
      confidentialToken1,
      confidentialToken2,
      confidentialToken3,
    ] = await ethers.getSigners();

    Object.assign(this, {
      owner,
      newOwner,
      anyone,
      token1,
      token2,
      token3,
      confidentialToken1,
      confidentialToken2,
      confidentialToken3,
      registry: fixture.registry,
    });
  });

  describe('Access Control', function () {
    it('should not upgrade if not authorized', async function () {
      await expect(
        this.registry.connect(this.anyone).upgradeToAndCall(this.registry.target, '0x'),
      ).to.be.revertedWithCustomError(this.registry, 'OwnableUnauthorizedAccount');
    });
  });

  describe('registerConfidentialToken', function () {
    it('should register a confidential token', async function () {
      expect(await this.registry.getConfidentialTokenAddress(this.token1.address)).to.equal(ethers.ZeroAddress);
      expect(await this.registry.getTokenAddress(this.confidentialToken1.address)).to.equal(ethers.ZeroAddress);

      await expect(
        this.registry
          .connect(this.owner)
          .registerConfidentialToken(this.token1.address, this.confidentialToken1.address),
      )
        .to.emit(this.registry, 'ConfidentialTokenRegistered')
        .withArgs(this.token1.address, this.confidentialToken1.address);

      expect(await this.registry.getConfidentialTokenAddress(this.token1.address)).to.equal(
        this.confidentialToken1.address,
      );
      expect(await this.registry.getTokenAddress(this.confidentialToken1.address)).to.equal(this.token1.address);
    });

    it('should revert if not authorized', async function () {
      await expect(
        this.registry
          .connect(this.anyone)
          .registerConfidentialToken(this.token1.address, this.confidentialToken1.address),
      ).to.be.revertedWithCustomError(this.registry, 'OwnableUnauthorizedAccount');
    });

    it('should revert if token address is zero', async function () {
      await expect(
        this.registry
          .connect(this.owner)
          .registerConfidentialToken(ethers.ZeroAddress, this.confidentialToken1.address),
      ).to.be.revertedWithCustomError(this.registry, 'TokenZeroAddress');
    });

    it('should revert if confidential token address is zero', async function () {
      await expect(
        this.registry.connect(this.owner).registerConfidentialToken(this.token1.address, ethers.ZeroAddress),
      ).to.be.revertedWithCustomError(this.registry, 'ConfidentialTokenZeroAddress');
    });

    it('should revert if token is already associated with a confidential token', async function () {
      await this.registry
        .connect(this.owner)
        .registerConfidentialToken(this.token1.address, this.confidentialToken1.address);

      expect(await this.registry.getConfidentialTokenAddress(this.token1.address)).to.equal(
        this.confidentialToken1.address,
      );
      expect(await this.registry.getTokenAddress(this.confidentialToken1.address)).to.equal(this.token1.address);

      await expect(
        this.registry
          .connect(this.owner)
          .registerConfidentialToken(this.token1.address, this.confidentialToken2.address),
      )
        .to.be.revertedWithCustomError(this.registry, 'TokenAlreadyAssociatedWithConfidentialToken')
        .withArgs(this.token1.address, this.confidentialToken1.address);
    });

    it('should revert if confidential token is already associated with a token', async function () {
      await this.registry
        .connect(this.owner)
        .registerConfidentialToken(this.token1.address, this.confidentialToken1.address);

      expect(await this.registry.getConfidentialTokenAddress(this.token1.address)).to.equal(
        this.confidentialToken1.address,
      );
      expect(await this.registry.getTokenAddress(this.confidentialToken1.address)).to.equal(this.token1.address);

      await expect(
        this.registry
          .connect(this.owner)
          .registerConfidentialToken(this.token2.address, this.confidentialToken1.address),
      )
        .to.be.revertedWithCustomError(this.registry, 'ConfidentialTokenAlreadyAssociatedWithToken')
        .withArgs(this.confidentialToken1.address, this.token1.address);
    });

    it('should revert if confidential token is revoked', async function () {
      await this.registry
        .connect(this.owner)
        .registerConfidentialToken(this.token1.address, this.confidentialToken1.address);
      await this.registry.connect(this.owner).revokeConfidentialToken(this.confidentialToken1.address);

      expect(await this.registry.isConfidentialTokenRevoked(this.confidentialToken1.address)).to.equal(true);

      await expect(
        this.registry
          .connect(this.owner)
          .registerConfidentialToken(this.token2.address, this.confidentialToken1.address),
      )
        .to.be.revertedWithCustomError(this.registry, 'RevokedConfidentialToken')
        .withArgs(this.confidentialToken1.address);
    });
  });

  describe('revokeConfidentialToken', function () {
    beforeEach(async function () {
      await this.registry
        .connect(this.owner)
        .registerConfidentialToken(this.token1.address, this.confidentialToken1.address);
    });

    it('should revoke a confidential token', async function () {
      expect(await this.registry.getConfidentialTokenAddress(this.token1.address)).to.equal(
        this.confidentialToken1.address,
      );
      expect(await this.registry.getTokenAddress(this.confidentialToken1.address)).to.equal(this.token1.address);

      expect(await this.registry.isConfidentialTokenRevoked(this.confidentialToken1.address)).to.equal(false);

      await expect(this.registry.connect(this.owner).revokeConfidentialToken(this.confidentialToken1.address))
        .to.emit(this.registry, 'ConfidentialTokenRevoked')
        .withArgs(this.token1.address, this.confidentialToken1.address);

      expect(await this.registry.getConfidentialTokenAddress(this.token1.address)).to.equal(ethers.ZeroAddress);
      expect(await this.registry.getTokenAddress(this.confidentialToken1.address)).to.equal(ethers.ZeroAddress);

      expect(await this.registry.isConfidentialTokenRevoked(this.confidentialToken1.address)).to.equal(true);
    });

    it('should revert if not authorized', async function () {
      await expect(
        this.registry.connect(this.anyone).revokeConfidentialToken(this.confidentialToken1.address),
      ).to.be.revertedWithCustomError(this.registry, 'OwnableUnauthorizedAccount');
    });

    it('should revert if confidential token address is zero', async function () {
      await expect(
        this.registry.connect(this.owner).revokeConfidentialToken(ethers.ZeroAddress),
      ).to.be.revertedWithCustomError(this.registry, 'ConfidentialTokenZeroAddress');
    });

    it('should revert if confidential token is already revoked', async function () {
      await this.registry.connect(this.owner).revokeConfidentialToken(this.confidentialToken1.address);

      expect(await this.registry.isConfidentialTokenRevoked(this.confidentialToken1.address)).to.equal(true);

      await expect(this.registry.connect(this.owner).revokeConfidentialToken(this.confidentialToken1.address))
        .to.be.revertedWithCustomError(this.registry, 'RevokedConfidentialToken')
        .withArgs(this.confidentialToken1.address);
    });

    it('should revert if no token is associated with the confidential token', async function () {
      expect(await this.registry.getTokenAddress(this.confidentialToken2.address)).to.equal(ethers.ZeroAddress);

      await expect(this.registry.connect(this.owner).revokeConfidentialToken(this.confidentialToken2.address))
        .to.be.revertedWithCustomError(this.registry, 'NoTokenAssociatedWithConfidentialToken')
        .withArgs(this.confidentialToken2.address);
    });
  });

  describe('reinstateConfidentialToken', function () {
    beforeEach(async function () {
      await this.registry
        .connect(this.owner)
        .registerConfidentialToken(this.token1.address, this.confidentialToken1.address);
      await this.registry.connect(this.owner).revokeConfidentialToken(this.confidentialToken1.address);
    });

    it('should reinstate a revoked confidential token', async function () {
      expect(await this.registry.getConfidentialTokenAddress(this.token1.address)).to.equal(ethers.ZeroAddress);
      expect(await this.registry.getTokenAddress(this.confidentialToken1.address)).to.equal(ethers.ZeroAddress);

      expect(await this.registry.isConfidentialTokenRevoked(this.confidentialToken1.address)).to.equal(true);

      await expect(this.registry.connect(this.owner).reinstateConfidentialToken(this.confidentialToken1.address))
        .to.emit(this.registry, 'ConfidentialTokenReinstated')
        .withArgs(this.confidentialToken1.address);

      expect(await this.registry.isConfidentialTokenRevoked(this.confidentialToken1.address)).to.equal(false);

      // Make sure the association with the token is not restored
      expect(await this.registry.getConfidentialTokenAddress(this.token1.address)).to.equal(ethers.ZeroAddress);
      expect(await this.registry.getTokenAddress(this.confidentialToken1.address)).to.equal(ethers.ZeroAddress);
    });

    it('should allow re-registration with the same token after reinstatement', async function () {
      expect(await this.registry.getConfidentialTokenAddress(this.token1.address)).to.equal(ethers.ZeroAddress);
      expect(await this.registry.getTokenAddress(this.confidentialToken1.address)).to.equal(ethers.ZeroAddress);

      await this.registry.connect(this.owner).reinstateConfidentialToken(this.confidentialToken1.address);

      await expect(
        this.registry
          .connect(this.owner)
          .registerConfidentialToken(this.token1.address, this.confidentialToken1.address),
      )
        .to.emit(this.registry, 'ConfidentialTokenRegistered')
        .withArgs(this.token1.address, this.confidentialToken1.address);

      expect(await this.registry.getConfidentialTokenAddress(this.token1.address)).to.equal(
        this.confidentialToken1.address,
      );
      expect(await this.registry.getTokenAddress(this.confidentialToken1.address)).to.equal(this.token1.address);
    });

    it('should allow re-registration with a different token after reinstatement', async function () {
      expect(await this.registry.getConfidentialTokenAddress(this.token2.address)).to.equal(ethers.ZeroAddress);
      expect(await this.registry.getTokenAddress(this.confidentialToken1.address)).to.equal(ethers.ZeroAddress);

      await this.registry.connect(this.owner).reinstateConfidentialToken(this.confidentialToken1.address);

      await expect(
        this.registry
          .connect(this.owner)
          .registerConfidentialToken(this.token2.address, this.confidentialToken1.address),
      )
        .to.emit(this.registry, 'ConfidentialTokenRegistered')
        .withArgs(this.token2.address, this.confidentialToken1.address);

      expect(await this.registry.getConfidentialTokenAddress(this.token2.address)).to.equal(
        this.confidentialToken1.address,
      );
      expect(await this.registry.getTokenAddress(this.confidentialToken1.address)).to.equal(this.token2.address);
    });

    it('should revert if not authorized', async function () {
      await expect(
        this.registry.connect(this.anyone).reinstateConfidentialToken(this.confidentialToken1.address),
      ).to.be.revertedWithCustomError(this.registry, 'OwnableUnauthorizedAccount');
    });

    it('should revert if confidential token address is zero', async function () {
      await expect(
        this.registry.connect(this.owner).reinstateConfidentialToken(ethers.ZeroAddress),
      ).to.be.revertedWithCustomError(this.registry, 'ConfidentialTokenZeroAddress');
    });

    it('should revert if confidential token is not revoked', async function () {
      await this.registry.connect(this.owner).reinstateConfidentialToken(this.confidentialToken1.address);

      await expect(this.registry.connect(this.owner).reinstateConfidentialToken(this.confidentialToken1.address))
        .to.be.revertedWithCustomError(this.registry, 'ConfidentialTokenNotRevoked')
        .withArgs(this.confidentialToken1.address);
    });
  });
});
