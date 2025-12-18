import { loadFixture } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import { ethers } from 'hardhat';
import hre from 'hardhat';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { getRegistryFixture } from '../utils/loadFixture';

describe('register Tasks', function () {
  let registry: any;
  let token1: string;
  let confidentialToken1: string;

  // Reset the contract state between each test
  beforeEach(async function () {
    const fixture = await loadFixture(getRegistryFixture);
    registry = fixture.registry;
  });

  describe('task:registerConfidentialToken', function () {
    it('should register a single token with its confidential token', async function () {
      token1 = getRequiredEnvVar('INITIAL_TOKEN_ADDRESS_0');
      confidentialToken1 = getRequiredEnvVar('INITIAL_CONFIDENTIAL_TOKEN_ADDRESS_0');

      // Verify the token is not registered before
      const confidentialTokenBefore = await registry.getConfidentialTokenAddress(token1);
      expect(confidentialTokenBefore).to.equal(ethers.ZeroAddress);

      // Run the task to register the token
      await hre.run('task:registerConfidentialToken', {
        token: token1,
        confidentialToken: confidentialToken1,
      });

      // Verify the token is now registered
      const confidentialTokenAfter = await registry.getConfidentialTokenAddress(token1);
      expect(confidentialTokenAfter).to.equal(confidentialToken1);

      // Verify the reverse mapping
      const tokenAfter = await registry.getTokenAddress(confidentialToken1);
      expect(tokenAfter).to.equal(token1);
    });
  });

  describe('task:registerAllInitialConfidentialTokens', function () {
    it('should register all initial tokens from environment variables', async function () {
      const numTokens = parseInt(getRequiredEnvVar('INITIAL_NUM_TOKENS'));
      for (let i = 0; i < numTokens; i++) {
        const tokenAddress = getRequiredEnvVar(`INITIAL_TOKEN_ADDRESS_${i}`);
        const confidentialTokenAddress = getRequiredEnvVar(`INITIAL_CONFIDENTIAL_TOKEN_ADDRESS_${i}`);
        expect(await registry.getConfidentialTokenAddress(tokenAddress)).to.equal(ethers.ZeroAddress);
        expect(await registry.getConfidentialTokenAddress(confidentialTokenAddress)).to.equal(ethers.ZeroAddress);
      }

      // Run the task to register all initial tokens
      await hre.run('task:registerAllInitialConfidentialTokens');

      for (let i = 0; i < numTokens; i++) {
        const tokenAddress = getRequiredEnvVar(`INITIAL_TOKEN_ADDRESS_${i}`);
        const confidentialTokenAddress = getRequiredEnvVar(`INITIAL_CONFIDENTIAL_TOKEN_ADDRESS_${i}`);
        expect(await registry.getConfidentialTokenAddress(tokenAddress)).to.equal(confidentialTokenAddress);
        expect(await registry.getTokenAddress(confidentialTokenAddress)).to.equal(tokenAddress);
      }
    });
  });
});
