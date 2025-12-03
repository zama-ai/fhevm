import { MANAGER_ROLE } from '../../tasks/ownership/protocolStakingManagerRole';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { getProtocolStakingContractsFixture } from '../utils';
import { loadFixture } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import hre from 'hardhat';

describe('ProtocolStaking Manager Role Tasks', function () {
  let coproProtocolStaking: any;
  let kmsProtocolStaking: any;
  let daoAddress: string;
  let deployerAddress: string;

  before(async function () {
    daoAddress = getRequiredEnvVar('DAO_ADDRESS');

    // Get the deployer address
    const { deployer } = await hre.getNamedAccounts();
    deployerAddress = deployer;
  });

  // Reset the contracts' state between each test
  beforeEach(async function () {
    const fixture = await loadFixture(getProtocolStakingContractsFixture);
    coproProtocolStaking = fixture.coproProtocolStaking;
    kmsProtocolStaking = fixture.kmsProtocolStaking;
  });

  describe('task:grantProtocolStakingManagerRolesToDAO', function () {
    it('Should grant manager role to the DAO for both coprocessor and KMS protocol staking contracts', async function () {
      // Verify the DAO does not have the manager role before granting
      const coproHasRoleBefore = await coproProtocolStaking.hasRole(MANAGER_ROLE, daoAddress);
      const kmsHasRoleBefore = await kmsProtocolStaking.hasRole(MANAGER_ROLE, daoAddress);
      expect(coproHasRoleBefore).to.be.false;
      expect(kmsHasRoleBefore).to.be.false;

      // Run the task to grant manager roles
      await hre.run('task:grantProtocolStakingManagerRolesToDAO');

      // Verify the DAO now has the manager role
      const coproHasRoleAfter = await coproProtocolStaking.hasRole(MANAGER_ROLE, daoAddress);
      const kmsHasRoleAfter = await kmsProtocolStaking.hasRole(MANAGER_ROLE, daoAddress);
      expect(coproHasRoleAfter).to.be.true;
      expect(kmsHasRoleAfter).to.be.true;
    });
  });

  describe('task:renounceProtocolStakingManagerRolesFromDeployer', function () {
    it('Should renounce manager role from the deployer for both coprocessor and KMS protocol staking contracts', async function () {
      // First, verify the deployer has the manager role
      const coproHasRoleBefore = await coproProtocolStaking.hasRole(MANAGER_ROLE, deployerAddress);
      const kmsHasRoleBefore = await kmsProtocolStaking.hasRole(MANAGER_ROLE, deployerAddress);
      expect(coproHasRoleBefore).to.be.true;
      expect(kmsHasRoleBefore).to.be.true;

      // Run the task to renounce manager roles
      await hre.run('task:renounceProtocolStakingManagerRolesFromDeployer');

      // Verify the deployer no longer has the manager role
      const coproHasRoleAfter = await coproProtocolStaking.hasRole(MANAGER_ROLE, deployerAddress);
      const kmsHasRoleAfter = await kmsProtocolStaking.hasRole(MANAGER_ROLE, deployerAddress);
      expect(coproHasRoleAfter).to.be.false;
      expect(kmsHasRoleAfter).to.be.false;
    });
  });
});
