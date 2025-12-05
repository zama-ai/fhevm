import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { getOperatorRewarderContractsFixture } from '../utils';
import { loadFixture } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import hre from 'hardhat';

describe('setFee Tasks', function () {
  let coproOperatorRewarders: any[];
  let kmsOperatorRewarders: any[];

  // Reset the contracts' state between each test
  beforeEach(async function () {
    const fixture = await loadFixture(getOperatorRewarderContractsFixture);
    coproOperatorRewarders = fixture.coproOperatorRewarders;
    kmsOperatorRewarders = fixture.kmsOperatorRewarders;
  });

  describe('task:setFee', function () {
    it('Should set the fee for a single coprocessor operator rewarder contract', async function () {
      // Get the first coprocessor operator rewarder contract
      const operatorRewarder = coproOperatorRewarders[0];
      const operatorRewarderAddress = await operatorRewarder.getAddress();

      // Get the fee to set (in basis points, 1/100th of a percent, so 10000 = 100.00%)
      const fee = BigInt(parseInt(getRequiredEnvVar('OPERATOR_REWARDER_COPRO_FEE_0')));

      // Verify the fee is 0 before setting
      const feeBefore = await operatorRewarder.feeBasisPoints();
      expect(feeBefore).to.equal(0n);

      // Run the task to set the fee
      await hre.run('task:setFee', { fee, operatorRewarderAddress });

      // Verify the fee is now set correctly
      const feeAfter = await operatorRewarder.feeBasisPoints();
      expect(feeAfter).to.equal(fee);
    });

    it('Should set the fee for a single KMS operator rewarder contract', async function () {
      // Get the first KMS operator rewarder contract
      const operatorRewarder = kmsOperatorRewarders[0];
      const operatorRewarderAddress = await operatorRewarder.getAddress();

      // Get the fee to set (in basis points, 1/100th of a percent, so 10000 = 100.00%)
      const fee = BigInt(parseInt(getRequiredEnvVar('OPERATOR_REWARDER_KMS_FEE_0')));

      // Verify the fee is 0 before setting
      const feeBefore = await operatorRewarder.feeBasisPoints();
      expect(feeBefore).to.equal(0n);

      // Run the task to set the fee
      await hre.run('task:setFee', { fee, operatorRewarderAddress });

      // Verify the fee is now set correctly
      const feeAfter = await operatorRewarder.feeBasisPoints();
      expect(feeAfter).to.equal(fee);
    });
  });

  describe('task:setAllCoprocessorFees', function () {
    it('Should set fees for all coprocessor operator rewarder contracts', async function () {
      const numOperatorRewarderCopro = coproOperatorRewarders.length;

      // Collect expected fees and verify initial state
      const expectedFees: bigint[] = [];
      for (let i = 0; i < numOperatorRewarderCopro; i++) {
        const expectedFee = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_COPRO_FEE_${i}`)));
        expectedFees.push(expectedFee);

        // Verify the fee is 0 before setting
        const feeBefore = await coproOperatorRewarders[i].feeBasisPoints();
        expect(feeBefore).to.equal(0n);
      }

      // Run the task to set all coprocessor fees
      await hre.run('task:setAllCoprocessorFees');

      // Verify all fees are now set correctly
      for (let i = 0; i < numOperatorRewarderCopro; i++) {
        const feeAfter = await coproOperatorRewarders[i].feeBasisPoints();
        expect(feeAfter).to.equal(expectedFees[i]);
      }
    });
  });

  describe('task:setAllKMSFees', function () {
    it('Should set fees for all KMS operator rewarder contracts', async function () {
      const numOperatorRewarderKms = kmsOperatorRewarders.length;

      // Collect expected fees and verify initial state
      const expectedFees: bigint[] = [];
      for (let i = 0; i < numOperatorRewarderKms; i++) {
        const expectedFee = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_KMS_FEE_${i}`)));
        expectedFees.push(expectedFee);

        // Verify the fee is 0 before setting
        const feeBefore = await kmsOperatorRewarders[i].feeBasisPoints();
        expect(feeBefore).to.equal(0n);
      }

      // Run the task to set all KMS fees
      await hre.run('task:setAllKMSFees');

      // Verify all fees are now set correctly
      for (let i = 0; i < numOperatorRewarderKms; i++) {
        const feeAfter = await kmsOperatorRewarders[i].feeBasisPoints();
        expect(feeAfter).to.equal(expectedFees[i]);
      }
    });
  });

  describe('task:setAllFees', function () {
    it('Should set fees for all operator rewarder contracts (coprocessor and KMS)', async function () {
      const numOperatorRewarderCopro = coproOperatorRewarders.length;
      const numOperatorRewarderKms = kmsOperatorRewarders.length;

      // Collect expected fees for coprocessor
      const expectedCoproFees: bigint[] = [];
      for (let i = 0; i < numOperatorRewarderCopro; i++) {
        const expectedFee = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_COPRO_FEE_${i}`)));
        expectedCoproFees.push(expectedFee);
      }

      // Collect expected fees for KMS
      const expectedKmsFees: bigint[] = [];
      for (let i = 0; i < numOperatorRewarderKms; i++) {
        const expectedFee = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_KMS_FEE_${i}`)));
        expectedKmsFees.push(expectedFee);
      }

      // Run the task to set all fees
      await hre.run('task:setAllFees');

      // Verify all coprocessor fees are now set correctly
      for (let i = 0; i < numOperatorRewarderCopro; i++) {
        const feeAfter = await coproOperatorRewarders[i].feeBasisPoints();
        expect(feeAfter).to.equal(expectedCoproFees[i]);
      }

      // Verify all KMS fees are now set correctly
      for (let i = 0; i < numOperatorRewarderKms; i++) {
        const feeAfter = await kmsOperatorRewarders[i].feeBasisPoints();
        expect(feeAfter).to.equal(expectedKmsFees[i]);
      }
    });
  });
});
