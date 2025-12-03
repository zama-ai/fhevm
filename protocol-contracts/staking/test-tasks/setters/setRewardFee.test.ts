import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { getProtocolStakingContractsFixture } from '../utils';
import { loadFixture } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import hre from 'hardhat';

describe('setRewardRate Tasks', function () {
  let coproProtocolStaking: any;
  let kmsProtocolStaking: any;

  // Reset the contracts' state between each test
  beforeEach(async function () {
    const fixture = await loadFixture(getProtocolStakingContractsFixture);
    coproProtocolStaking = fixture.coproProtocolStaking;
    kmsProtocolStaking = fixture.kmsProtocolStaking;
  });

  describe('task:setRewardRate', function () {
    it('Should set the reward rate for the coprocessor protocol staking contract', async function () {
      // Get the reward rate to set (in tokens per second)
      const rewardRate = BigInt(parseInt(getRequiredEnvVar('PROTOCOL_STAKING_COPRO_REWARD_RATE')));

      // Verify the reward rate is 0 before setting
      const rewardRateBefore = await coproProtocolStaking.rewardRate();
      expect(rewardRateBefore).to.equal(0n);

      // Run the task to set the reward rate
      await hre.run('task:setRewardRate', {
        rewardRate,
        protocolStakingProxyAddress: await coproProtocolStaking.getAddress(),
      });

      // Verify the reward rate is now set correctly
      const rewardRateAfter = await coproProtocolStaking.rewardRate();
      expect(rewardRateAfter).to.equal(rewardRate);
    });

    it('Should set the reward rate for the KMS protocol staking contract', async function () {
      // Get the reward rate to set (in tokens per second)
      const rewardRate = BigInt(parseInt(getRequiredEnvVar('PROTOCOL_STAKING_KMS_REWARD_RATE')));

      // Verify the reward rate is 0 before setting
      const rewardRateBefore = await kmsProtocolStaking.rewardRate();
      expect(rewardRateBefore).to.equal(0n);

      // Run the task to set the reward rate
      await hre.run('task:setRewardRate', {
        rewardRate,
        protocolStakingProxyAddress: await kmsProtocolStaking.getAddress(),
      });

      // Verify the reward rate is now set correctly
      const rewardRateAfter = await kmsProtocolStaking.rewardRate();
      expect(rewardRateAfter).to.equal(rewardRate);
    });
  });

  describe('task:setCoprocessorRewardRate', function () {
    it('Should set the reward rate for the coprocessor protocol staking contract from environment', async function () {
      // Get the expected reward rate from environment
      const expectedRewardRate = BigInt(parseInt(getRequiredEnvVar('PROTOCOL_STAKING_COPRO_REWARD_RATE')));

      // Verify the reward rate is 0 before setting
      const rewardRateBefore = await coproProtocolStaking.rewardRate();
      expect(rewardRateBefore).to.equal(0n);

      // Run the task to set the coprocessor reward rate
      await hre.run('task:setCoprocessorRewardRate');

      // Verify the reward rate is now set correctly
      const rewardRateAfter = await coproProtocolStaking.rewardRate();
      expect(rewardRateAfter).to.equal(expectedRewardRate);
    });
  });

  describe('task:setKMSRewardRate', function () {
    it('Should set the reward rate for the KMS protocol staking contract from environment', async function () {
      // Get the expected reward rate from environment
      const expectedRewardRate = BigInt(parseInt(getRequiredEnvVar('PROTOCOL_STAKING_KMS_REWARD_RATE')));

      // Verify the reward rate is 0 before setting
      const rewardRateBefore = await kmsProtocolStaking.rewardRate();
      expect(rewardRateBefore).to.equal(0n);

      // Run the task to set the KMS reward rate
      await hre.run('task:setKMSRewardRate');

      // Verify the reward rate is now set correctly
      const rewardRateAfter = await kmsProtocolStaking.rewardRate();
      expect(rewardRateAfter).to.equal(expectedRewardRate);
    });
  });

  describe('task:setAllRewardRates', function () {
    it('Should set reward rates for all protocol staking contracts (coprocessor and KMS)', async function () {
      // Get the expected reward rates from environment
      const expectedCoproRewardRate = BigInt(parseInt(getRequiredEnvVar('PROTOCOL_STAKING_COPRO_REWARD_RATE')));
      const expectedKmsRewardRate = BigInt(parseInt(getRequiredEnvVar('PROTOCOL_STAKING_KMS_REWARD_RATE')));

      // Verify the reward rates are 0 before setting
      const coproRewardRateBefore = await coproProtocolStaking.rewardRate();
      expect(coproRewardRateBefore).to.equal(0n);
      const kmsRewardRateBefore = await kmsProtocolStaking.rewardRate();
      expect(kmsRewardRateBefore).to.equal(0n);

      // Run the task to set all reward rates
      await hre.run('task:setAllRewardRates');

      // Verify the reward rates are now set correctly
      const coproRewardRateAfter = await coproProtocolStaking.rewardRate();
      expect(coproRewardRateAfter).to.equal(expectedCoproRewardRate);
      const kmsRewardRateAfter = await kmsProtocolStaking.rewardRate();
      expect(kmsRewardRateAfter).to.equal(expectedKmsRewardRate);
    });
  });
});
