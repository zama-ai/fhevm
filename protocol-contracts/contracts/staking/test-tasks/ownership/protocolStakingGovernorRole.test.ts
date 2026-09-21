import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { getProtocolStakingContractsFixture } from '../utils';
import { loadFixture } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import hre from 'hardhat';

describe('ProtocolStaking Governor Role Tasks', function () {
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

  describe('task:beginTransferProtocolStakingGovernorRolesToDAO', function () {
    it('Should begin transfer of governor role to the DAO for both coprocessor and KMS protocol staking contracts', async function () {
      // Verify the deployer is the current admin before transfer
      const coproAdminBefore = await coproProtocolStaking.defaultAdmin();
      const kmsAdminBefore = await kmsProtocolStaking.defaultAdmin();
      expect(coproAdminBefore).to.equal(deployerAddress);
      expect(kmsAdminBefore).to.equal(deployerAddress);

      // Verify there is no pending admin before the transfer
      const [coproPendingAdminBefore] = await coproProtocolStaking.pendingDefaultAdmin();
      const [kmsPendingAdminBefore] = await kmsProtocolStaking.pendingDefaultAdmin();
      expect(coproPendingAdminBefore).to.equal(hre.ethers.ZeroAddress);
      expect(kmsPendingAdminBefore).to.equal(hre.ethers.ZeroAddress);

      // Run the task to begin the transfer of governor roles
      await hre.run('task:beginTransferProtocolStakingGovernorRolesToDAO');

      // Verify the DAO is now the pending admin
      const [coproPendingAdminAfter] = await coproProtocolStaking.pendingDefaultAdmin();
      const [kmsPendingAdminAfter] = await kmsProtocolStaking.pendingDefaultAdmin();
      expect(coproPendingAdminAfter).to.equal(daoAddress);
      expect(kmsPendingAdminAfter).to.equal(daoAddress);

      // Verify the deployer is still the current admin (transfer not completed yet)
      const coproAdminAfter = await coproProtocolStaking.defaultAdmin();
      const kmsAdminAfter = await kmsProtocolStaking.defaultAdmin();
      expect(coproAdminAfter).to.equal(deployerAddress);
      expect(kmsAdminAfter).to.equal(deployerAddress);
    });
  });
});
