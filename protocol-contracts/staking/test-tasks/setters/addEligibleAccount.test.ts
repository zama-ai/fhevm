import {
  getAllOperatorStakingCoproAddresses,
  getAllOperatorStakingKMSAddresses,
  getProtocolStakingCoproProxyAddress,
  getProtocolStakingKMSProxyAddress,
} from '../../tasks/utils/getAddresses';
import { getProtocolStakingContractsFixture } from '../utils';
import { loadFixture } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import { ethers } from 'hardhat';
import hre from 'hardhat';

const ELIGIBLE_ACCOUNT_ROLE = ethers.id('ELIGIBLE_ACCOUNT_ROLE');

describe('addEligibleAccount Tasks', function () {
  let coproProtocolStaking: any;
  let kmsProtocolStaking: any;
  let coproOperatorStakingAddresses: string[];
  let kmsOperatorStakingAddresses: string[];
  let kmsProtocolStakingAddress: string;
  let coproProtocolStakingAddress: string;

  // Get contracts addresses once
  before(async function () {
    coproOperatorStakingAddresses = await getAllOperatorStakingCoproAddresses(hre);
    kmsOperatorStakingAddresses = await getAllOperatorStakingKMSAddresses(hre);
    coproProtocolStakingAddress = await getProtocolStakingCoproProxyAddress(hre);
    kmsProtocolStakingAddress = await getProtocolStakingKMSProxyAddress(hre);
  });

  // Reset the contracts' state between each test
  beforeEach(async function () {
    const fixture = await loadFixture(getProtocolStakingContractsFixture);
    coproProtocolStaking = fixture.coproProtocolStaking;
    kmsProtocolStaking = fixture.kmsProtocolStaking;
  });

  describe('task:addOperatorAsEligibleInProtocolStaking', function () {
    it('Should add a single coprocessor operator as eligible in the protocol staking contract', async function () {
      const operatorStakingAddress = coproOperatorStakingAddresses[0];

      // Verify the operator is not eligible before adding
      const isEligibleBefore = await coproProtocolStaking.hasRole(ELIGIBLE_ACCOUNT_ROLE, operatorStakingAddress);
      expect(isEligibleBefore).to.be.false;

      // Run the task to add the operator as eligible
      await hre.run('task:addOperatorAsEligibleInProtocolStaking', {
        operatorStakingAddress,
        protocolStakingProxyAddress: coproProtocolStakingAddress,
      });

      // Verify the operator is now eligible
      const isEligibleAfter = await coproProtocolStaking.hasRole(ELIGIBLE_ACCOUNT_ROLE, operatorStakingAddress);
      expect(isEligibleAfter).to.be.true;
    });

    it('Should add a single KMS operator as eligible in the protocol staking contract', async function () {
      const operatorStakingAddress = kmsOperatorStakingAddresses[0];

      // Verify the operator is not eligible before adding
      const isEligibleBefore = await kmsProtocolStaking.hasRole(ELIGIBLE_ACCOUNT_ROLE, operatorStakingAddress);
      expect(isEligibleBefore).to.be.false;

      // Run the task to add the operator as eligible
      await hre.run('task:addOperatorAsEligibleInProtocolStaking', {
        operatorStakingAddress,
        protocolStakingProxyAddress: kmsProtocolStakingAddress,
      });

      // Verify the operator is now eligible
      const isEligibleAfter = await kmsProtocolStaking.hasRole(ELIGIBLE_ACCOUNT_ROLE, operatorStakingAddress);
      expect(isEligibleAfter).to.be.true;
    });
  });

  describe('task:addAllCoproOperatorsAsEligible', function () {
    it('Should add all coprocessor operators as eligible in the coprocessor protocol staking contract', async function () {
      // Verify all operators are not eligible before
      for (const operatorStakingAddress of coproOperatorStakingAddresses) {
        const isEligibleBefore = await coproProtocolStaking.hasRole(ELIGIBLE_ACCOUNT_ROLE, operatorStakingAddress);
        expect(isEligibleBefore).to.be.false;
      }

      // Run the task to add all coprocessor operators as eligible
      await hre.run('task:addAllCoproOperatorsAsEligible');

      // Verify all operators are now eligible
      for (const operatorStakingAddress of coproOperatorStakingAddresses) {
        const isEligibleAfter = await coproProtocolStaking.hasRole(ELIGIBLE_ACCOUNT_ROLE, operatorStakingAddress);
        expect(isEligibleAfter).to.be.true;
      }
    });
  });

  describe('task:addAllKMSOperatorsAsEligible', function () {
    it('Should add all KMS operators as eligible in the KMS protocol staking contract', async function () {
      // Verify all operators are not eligible before
      for (const operatorStakingAddress of kmsOperatorStakingAddresses) {
        const isEligibleBefore = await kmsProtocolStaking.hasRole(ELIGIBLE_ACCOUNT_ROLE, operatorStakingAddress);
        expect(isEligibleBefore).to.be.false;
      }

      // Run the task to add all KMS operators as eligible
      await hre.run('task:addAllKMSOperatorsAsEligible');

      // Verify all operators are now eligible
      for (const operatorStakingAddress of kmsOperatorStakingAddresses) {
        const isEligibleAfter = await kmsProtocolStaking.hasRole(ELIGIBLE_ACCOUNT_ROLE, operatorStakingAddress);
        expect(isEligibleAfter).to.be.true;
      }
    });
  });

  describe('task:addAllOperatorsAsEligible', function () {
    it('Should add all operators (coprocessor and KMS) as eligible in their respective protocol staking contracts', async function () {
      // Verify all operators are not eligible before
      for (const operatorStakingAddress of coproOperatorStakingAddresses) {
        const isEligibleBefore = await coproProtocolStaking.hasRole(ELIGIBLE_ACCOUNT_ROLE, operatorStakingAddress);
        expect(isEligibleBefore).to.be.false;
      }

      for (const operatorStakingAddress of kmsOperatorStakingAddresses) {
        const isEligibleBefore = await kmsProtocolStaking.hasRole(ELIGIBLE_ACCOUNT_ROLE, operatorStakingAddress);
        expect(isEligibleBefore).to.be.false;
      }

      // Run the task to add all operators as eligible
      await hre.run('task:addAllOperatorsAsEligible');

      // Verify all coprocessor operators are now eligible
      for (const operatorStakingAddress of coproOperatorStakingAddresses) {
        const isEligibleAfter = await coproProtocolStaking.hasRole(ELIGIBLE_ACCOUNT_ROLE, operatorStakingAddress);
        expect(isEligibleAfter).to.be.true;
      }

      // Verify all KMS operators are now eligible
      for (const operatorStakingAddress of kmsOperatorStakingAddresses) {
        const isEligibleAfter = await kmsProtocolStaking.hasRole(ELIGIBLE_ACCOUNT_ROLE, operatorStakingAddress);
        expect(isEligibleAfter).to.be.true;
      }
    });
  });
});
