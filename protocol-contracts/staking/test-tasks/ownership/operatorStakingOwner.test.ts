import { OPERATOR_STAKING_CONTRACT_NAME, OPERATOR_REWARDER_CONTRACT_NAME } from '../../tasks/deployment';
import { getAllOperatorStakingAddresses, getAllOperatorRewarderAddresses } from '../../tasks/utils/getAddresses';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { getOperatorStakingContractsFixture, getOperatorRewarderContractsFixture } from '../utils';
import { loadFixture } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import hre from 'hardhat';

// Simple fixture that just returns empty object - this creates a snapshot point
async function setupFixture() {
  return {};
}

describe('OperatorStaking Ownership Tasks', function () {
  let allOperatorStakingAddresses: string[];
  let allOperatorRewarderAddresses: string[];
  let daoAddress: string;

  before(async function () {
    allOperatorStakingAddresses = await getAllOperatorStakingAddresses(hre);
    allOperatorRewarderAddresses = await getAllOperatorRewarderAddresses(hre);
    daoAddress = getRequiredEnvVar('DAO_ADDRESS');
  });

  // Load the fixture to reset state
  beforeEach(async function () {
    await loadFixture(setupFixture);
  });

  describe('task:transferOperatorStakingOwnershipToDAO', function () {
    it('Should transfer ownership of an operator staking contract to the DAO using the deployer account', async function () {
      // Get the first operator staking contract address
      const operatorStakingAddress = allOperatorStakingAddresses[0];
      const operatorStaking = await hre.ethers.getContractAt(OPERATOR_STAKING_CONTRACT_NAME, operatorStakingAddress);

      // Get the current owner before transfer
      const ownerBefore = await operatorStaking.owner();
      expect(ownerBefore).to.not.equal(daoAddress);

      // Run the task to transfer ownership
      await hre.run('task:transferOperatorStakingOwnershipToDAO', {
        operatorStakingAddress,
      });

      // Verify the ownership has been transferred to the DAO
      const ownerAfter = await operatorStaking.owner();
      expect(ownerAfter).to.equal(daoAddress);
    });
  });

  describe('task:transferOperatorRewarderOwnershipToDAO', function () {
    it('Should transfer ownership of an operator rewarder contract to the DAO using the deployer account', async function () {
      // Get the first operator rewarder contract address
      const operatorRewarderAddress = allOperatorRewarderAddresses[0];
      const operatorRewarder = await hre.ethers.getContractAt(OPERATOR_REWARDER_CONTRACT_NAME, operatorRewarderAddress);

      // Get the current owner before transfer
      const ownerBefore = await operatorRewarder.owner();
      expect(ownerBefore).to.not.equal(daoAddress);

      // Run the task to transfer ownership
      await hre.run('task:transferOperatorRewarderOwnershipToDAO', {
        operatorRewarderAddress,
      });

      // Verify the ownership has been transferred to the DAO
      const ownerAfter = await operatorRewarder.owner();
      expect(ownerAfter).to.equal(daoAddress);
    });
  });

  describe('task:transferAllOperatorStakingRewarderOwnershipsToDAO', function () {
    it('Should transfer ownership of all operator staking and rewarder contracts to the DAO', async function () {
      // Get all operator staking and rewarder contracts
      const { coproOperatorStakings, kmsOperatorStakings } = await loadFixture(getOperatorStakingContractsFixture);
      const { coproOperatorRewarders, kmsOperatorRewarders } = await loadFixture(getOperatorRewarderContractsFixture);
      const operatorStakingContracts = [...coproOperatorStakings, ...kmsOperatorStakings];
      const operatorRewarderContracts = [...coproOperatorRewarders, ...kmsOperatorRewarders];

      // Verify that all contracts are not owned by the DAO before the transfer
      const stakingOwnersBefore = await Promise.all(operatorStakingContracts.map(contract => contract.owner()));
      const rewarderOwnersBefore = await Promise.all(operatorRewarderContracts.map(contract => contract.owner()));
      expect(stakingOwnersBefore.every(owner => owner !== daoAddress)).to.be.true;
      expect(rewarderOwnersBefore.every(owner => owner !== daoAddress)).to.be.true;

      // Run the task to transfer all ownerships
      await hre.run('task:transferAllOperatorStakingRewarderOwnershipsToDAO');

      // Verify that all contracts are now owned by the DAO
      const stakingOwnersAfter = await Promise.all(operatorStakingContracts.map(contract => contract.owner()));
      const rewarderOwnersAfter = await Promise.all(operatorRewarderContracts.map(contract => contract.owner()));

      expect(stakingOwnersAfter.every(owner => owner === daoAddress)).to.be.true;
      expect(rewarderOwnersAfter.every(owner => owner === daoAddress)).to.be.true;
    });
  });
});
