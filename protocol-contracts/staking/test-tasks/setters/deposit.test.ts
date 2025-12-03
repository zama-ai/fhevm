import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { getOperatorStakingContractsFixture } from '../utils';
import { loadFixture } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import hre from 'hardhat';

describe('deposit Tasks', function () {
  let coproOperatorStakings: any[];
  let kmsOperatorStakings: any[];

  // Reset the contracts' state between each test
  beforeEach(async function () {
    const fixture = await loadFixture(getOperatorStakingContractsFixture);
    coproOperatorStakings = fixture.coproOperatorStakings;
    kmsOperatorStakings = fixture.kmsOperatorStakings;
  });

  describe('task:depositOperatorStakingFromDeployer', function () {
    it('Should deposit assets into a single operator staking contract', async function () {
      // Get the first coprocessor operator staking contract and its address
      const operatorStaking = await coproOperatorStakings[0];
      const operatorStakingAddress = await operatorStaking.getAddress();

      // Verify the total assets is 0 before depositing
      const totalAssetsBefore = await operatorStaking.totalAssets();
      expect(totalAssetsBefore).to.equal(0n);

      // Get the receiver and assets for the first coprocessor operator staking contract
      const receiver = getRequiredEnvVar(`OPERATOR_STAKING_COPRO_INITIAL_DEPOSIT_RECEIVER_0`);
      const assets = BigInt(parseInt(getRequiredEnvVar('OPERATOR_STAKING_COPRO_INITIAL_DEPOSIT_ASSETS_0')));

      // Run the task to deposit assets
      await hre.run('task:depositOperatorStakingFromDeployer', {
        assets,
        receiver,
        operatorStakingAddress,
      });

      // Verify the total assets is now set correctly
      const totalAssetsAfter = await operatorStaking.totalAssets();
      expect(totalAssetsAfter).to.equal(assets);
    });
  });

  describe('task:depositAllCoproOperatorStakingFromDeployer', function () {
    it('Should deposit assets into all coprocessor operator staking contracts', async function () {
      const numOperatorStakingCopro = coproOperatorStakings.length;

      // Verify all coprocessor total assets are 0 before depositing
      for (let i = 0; i < numOperatorStakingCopro; i++) {
        const totalAssetsBefore = await coproOperatorStakings[i].totalAssets();
        expect(totalAssetsBefore).to.equal(0n);
      }

      // Run the task to deposit all coprocessor operator staking contracts
      await hre.run('task:depositAllCoproOperatorStakingFromDeployer');

      // Collect expected assets and verify all total assets are now set correctly
      for (let i = 0; i < numOperatorStakingCopro; i++) {
        const expectedAsset = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_STAKING_COPRO_INITIAL_DEPOSIT_ASSETS_${i}`)));
        const totalAssetsAfter = await coproOperatorStakings[i].totalAssets();
        expect(totalAssetsAfter).to.equal(expectedAsset);
      }
    });
  });

  describe('task:depositAllKMSOperatorStakingFromDeployer', function () {
    it('Should deposit assets into all KMS operator staking contracts', async function () {
      const numOperatorStakingKms = kmsOperatorStakings.length;

      // Verify all KMS total assets are 0 before depositing
      for (let i = 0; i < numOperatorStakingKms; i++) {
        const totalAssetsBefore = await kmsOperatorStakings[i].totalAssets();
        expect(totalAssetsBefore).to.equal(0n);
      }
      for (let i = 0; i < numOperatorStakingKms; i++) {
        const totalAssetsBefore = await kmsOperatorStakings[i].totalAssets();
        expect(totalAssetsBefore).to.equal(0n);
      }

      // Run the task to deposit all KMS operator staking contracts
      await hre.run('task:depositAllKMSOperatorStakingFromDeployer');

      // Verify all KMS operator staking contracts' expected assets are now set correctly
      for (let i = 0; i < numOperatorStakingKms; i++) {
        const expectedAsset = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_STAKING_KMS_INITIAL_DEPOSIT_ASSETS_${i}`)));
        const totalAssetsAfter = await kmsOperatorStakings[i].totalAssets();
        expect(totalAssetsAfter).to.equal(expectedAsset);
      }
    });
  });

  describe('task:depositAllOperatorStakingFromDeployer', function () {
    it('Should deposit assets into all operator staking contracts (coprocessor and KMS)', async function () {
      const numOperatorStakingCopro = coproOperatorStakings.length;
      const numOperatorStakingKms = kmsOperatorStakings.length;

      // Verify all operator staking contracts' total assets are 0 before depositing
      for (let i = 0; i < numOperatorStakingCopro; i++) {
        const totalAssetsBefore = await coproOperatorStakings[i].totalAssets();
        expect(totalAssetsBefore).to.equal(0n);
      }
      for (let i = 0; i < numOperatorStakingKms; i++) {
        const totalAssetsBefore = await kmsOperatorStakings[i].totalAssets();
        expect(totalAssetsBefore).to.equal(0n);
      }

      // Run the task to deposit all operator staking contracts
      await hre.run('task:depositAllOperatorStakingFromDeployer');

      // Verify all coprocessor total assets are now set correctly
      for (let i = 0; i < numOperatorStakingCopro; i++) {
        const expectedAsset = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_STAKING_COPRO_INITIAL_DEPOSIT_ASSETS_${i}`)));
        const totalAssetsAfter = await coproOperatorStakings[i].totalAssets();
        expect(totalAssetsAfter).to.equal(expectedAsset);
      }

      // Verify all KMS total assets are now set correctly
      for (let i = 0; i < numOperatorStakingKms; i++) {
        const expectedAsset = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_STAKING_KMS_INITIAL_DEPOSIT_ASSETS_${i}`)));
        const totalAssetsAfter = await kmsOperatorStakings[i].totalAssets();
        expect(totalAssetsAfter).to.equal(expectedAsset);
      }
    });
  });
});
