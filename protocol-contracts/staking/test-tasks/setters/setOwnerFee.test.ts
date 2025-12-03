import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { getOperatorRewarderContractsFixture } from '../utils';
import { loadFixture } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import hre from 'hardhat';

describe('setOwnerFee Tasks', function () {
  let coproOperatorRewarders: any[];
  let kmsOperatorRewarders: any[];

  // Reset the contracts' state between each test
  beforeEach(async function () {
    const fixture = await loadFixture(getOperatorRewarderContractsFixture);
    coproOperatorRewarders = fixture.coproOperatorRewarders;
    kmsOperatorRewarders = fixture.kmsOperatorRewarders;
  });

  describe('task:setOwnerFee', function () {
    it('Should set the owner fee for a single coprocessor operator rewarder contract', async function () {
      // Get the first coprocessor operator rewarder contract
      const operatorRewarder = coproOperatorRewarders[0];
      const operatorRewarderAddress = await operatorRewarder.getAddress();

      // Get the owner fee to set (in basis points, 1/100th of a percent, so 10000 = 100.00%)
      const ownerFee = BigInt(parseInt(getRequiredEnvVar('OPERATOR_REWARDER_COPRO_OWNER_FEE_0')));

      // Verify the owner fee is 0 before setting
      const ownerFeeBefore = await operatorRewarder.ownerFeeBasisPoints();
      expect(ownerFeeBefore).to.equal(0n);

      // Run the task to set the owner fee
      await hre.run('task:setOwnerFee', { ownerFee, operatorRewarderAddress });

      // Verify the owner fee is now set correctly
      const ownerFeeAfter = await operatorRewarder.ownerFeeBasisPoints();
      expect(ownerFeeAfter).to.equal(ownerFee);
    });

    it('Should set the owner fee for a single KMS operator rewarder contract', async function () {
      // Get the first KMS operator rewarder contract
      const operatorRewarder = kmsOperatorRewarders[0];
      const operatorRewarderAddress = await operatorRewarder.getAddress();

      // Get the owner fee to set (in basis points, 1/100th of a percent, so 10000 = 100.00%)
      const ownerFee = BigInt(parseInt(getRequiredEnvVar('OPERATOR_REWARDER_KMS_OWNER_FEE_0')));

      // Verify the owner fee is 0 before setting
      const ownerFeeBefore = await operatorRewarder.ownerFeeBasisPoints();
      expect(ownerFeeBefore).to.equal(0n);

      // Run the task to set the owner fee
      await hre.run('task:setOwnerFee', { ownerFee, operatorRewarderAddress });

      // Verify the owner fee is now set correctly
      const ownerFeeAfter = await operatorRewarder.ownerFeeBasisPoints();
      expect(ownerFeeAfter).to.equal(ownerFee);
    });
  });

  describe('task:setAllCoprocessorOwnerFees', function () {
    it('Should set owner fees for all coprocessor operator rewarder contracts', async function () {
      const numOperatorRewarderCopro = coproOperatorRewarders.length;

      // Collect expected fees and verify initial state
      const expectedFees: bigint[] = [];
      for (let i = 0; i < numOperatorRewarderCopro; i++) {
        const expectedFee = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_COPRO_OWNER_FEE_${i}`)));
        expectedFees.push(expectedFee);

        // Verify the owner fee is 0 before setting
        const ownerFeeBefore = await coproOperatorRewarders[i].ownerFeeBasisPoints();
        expect(ownerFeeBefore).to.equal(0n);
      }

      // Run the task to set all coprocessor owner fees
      await hre.run('task:setAllCoprocessorOwnerFees');

      // Verify all owner fees are now set correctly
      for (let i = 0; i < numOperatorRewarderCopro; i++) {
        const ownerFeeAfter = await coproOperatorRewarders[i].ownerFeeBasisPoints();
        expect(ownerFeeAfter).to.equal(expectedFees[i]);
      }
    });
  });

  describe('task:setAllKMSOwnerFees', function () {
    it('Should set owner fees for all KMS operator rewarder contracts', async function () {
      const numOperatorRewarderKms = kmsOperatorRewarders.length;

      // Collect expected fees and verify initial state
      const expectedFees: bigint[] = [];
      for (let i = 0; i < numOperatorRewarderKms; i++) {
        const expectedFee = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_KMS_OWNER_FEE_${i}`)));
        expectedFees.push(expectedFee);

        // Verify the owner fee is 0 before setting
        const ownerFeeBefore = await kmsOperatorRewarders[i].ownerFeeBasisPoints();
        expect(ownerFeeBefore).to.equal(0n);
      }

      // Run the task to set all KMS owner fees
      await hre.run('task:setAllKMSOwnerFees');

      // Verify all owner fees are now set correctly
      for (let i = 0; i < numOperatorRewarderKms; i++) {
        const ownerFeeAfter = await kmsOperatorRewarders[i].ownerFeeBasisPoints();
        expect(ownerFeeAfter).to.equal(expectedFees[i]);
      }
    });
  });

  describe('task:setAllOwnerFees', function () {
    it('Should set owner fees for all operator rewarder contracts (coprocessor and KMS)', async function () {
      const numOperatorRewarderCopro = coproOperatorRewarders.length;
      const numOperatorRewarderKms = kmsOperatorRewarders.length;

      // Collect expected fees for coprocessor
      const expectedCoproFees: bigint[] = [];
      for (let i = 0; i < numOperatorRewarderCopro; i++) {
        const expectedFee = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_COPRO_OWNER_FEE_${i}`)));
        expectedCoproFees.push(expectedFee);
      }

      // Collect expected fees for KMS
      const expectedKmsFees: bigint[] = [];
      for (let i = 0; i < numOperatorRewarderKms; i++) {
        const expectedFee = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_KMS_OWNER_FEE_${i}`)));
        expectedKmsFees.push(expectedFee);
      }

      // Run the task to set all owner fees
      await hre.run('task:setAllOwnerFees');

      // Verify all coprocessor owner fees are now set correctly
      for (let i = 0; i < numOperatorRewarderCopro; i++) {
        const ownerFeeAfter = await coproOperatorRewarders[i].ownerFeeBasisPoints();
        expect(ownerFeeAfter).to.equal(expectedCoproFees[i]);
      }

      // Verify all KMS owner fees are now set correctly
      for (let i = 0; i < numOperatorRewarderKms; i++) {
        const ownerFeeAfter = await kmsOperatorRewarders[i].ownerFeeBasisPoints();
        expect(ownerFeeAfter).to.equal(expectedKmsFees[i]);
      }
    });
  });
});
