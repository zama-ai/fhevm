import { getProtocolStakingProxyName, getProtocolStakingImplName } from '../../tasks/deployment';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { expectContractDeployed } from '../utils';
import { expect } from 'chai';
import { ethers } from 'hardhat';
import hre from 'hardhat';

const MANAGER_ROLE = ethers.id('MANAGER_ROLE');

describe('ProtocolStaking Deployment', function () {
  describe('Coprocessor ProtocolStaking', function () {
    it('Should deploy the coprocessor ProtocolStaking proxy contract with correct configuration', async function () {
      // Get the expected token name from environment
      const coproTokenName = getRequiredEnvVar('PROTOCOL_STAKING_COPRO_TOKEN_NAME');
      const coproTokenSymbol = getRequiredEnvVar('PROTOCOL_STAKING_COPRO_TOKEN_SYMBOL');
      const coproCooldown = parseInt(getRequiredEnvVar('PROTOCOL_STAKING_COPRO_COOLDOWN_PERIOD'));

      // Get the deployed proxy contract
      const proxyDeployment = await hre.deployments.get(getProtocolStakingProxyName(coproTokenName));
      const protocolStaking = await hre.ethers.getContractAt('ProtocolStaking', proxyDeployment.address);

      // Verify the contract was deployed
      await expectContractDeployed(proxyDeployment.address);

      // Verify the contract configuration
      expect(await protocolStaking.name()).to.equal(coproTokenName);
      expect(await protocolStaking.symbol()).to.equal(coproTokenSymbol);
      expect(await protocolStaking.unstakeCooldownPeriod()).to.equal(coproCooldown);

      // Verify the staking token address
      const zamaTokenAddress = getRequiredEnvVar('ZAMA_TOKEN_ADDRESS');
      expect(await protocolStaking.stakingToken()).to.equal(zamaTokenAddress);

      const { deployer } = await hre.getNamedAccounts();

      // Verify the deployer has the default admin role (governor) initially
      const DEFAULT_ADMIN_ROLE = ethers.ZeroHash;
      expect(await protocolStaking.hasRole(DEFAULT_ADMIN_ROLE, deployer)).to.be.true;

      // Verify the deployer has the manager role initially
      expect(await protocolStaking.hasRole(MANAGER_ROLE, deployer)).to.be.true;
    });

    it('Should deploy the coprocessor ProtocolStaking implementation contract', async function () {
      const coproTokenName = getRequiredEnvVar('PROTOCOL_STAKING_COPRO_TOKEN_NAME');
      const implName = coproTokenName + '_Impl';

      // Get the deployed implementation contract
      const implDeployment = await hre.deployments.get(implName);

      // Verify the implementation was deployed
      await expectContractDeployed(implDeployment.address);
    });
  });

  describe('KMS ProtocolStaking', function () {
    it('Should deploy the KMS ProtocolStaking proxy contract with correct configuration', async function () {
      // Get the expected token name from environment
      const kmsTokenName = getRequiredEnvVar('PROTOCOL_STAKING_KMS_TOKEN_NAME');
      const kmsTokenSymbol = getRequiredEnvVar('PROTOCOL_STAKING_KMS_TOKEN_SYMBOL');
      const kmsCooldown = parseInt(getRequiredEnvVar('PROTOCOL_STAKING_KMS_COOLDOWN_PERIOD'));

      // Get the deployed proxy contract
      const proxyDeployment = await hre.deployments.get(getProtocolStakingProxyName(kmsTokenName));
      const protocolStaking = await hre.ethers.getContractAt('ProtocolStaking', proxyDeployment.address);

      // Verify the contract was deployed
      await expectContractDeployed(proxyDeployment.address);

      // Verify the contract configuration
      expect(await protocolStaking.name()).to.equal(kmsTokenName);
      expect(await protocolStaking.symbol()).to.equal(kmsTokenSymbol);
      expect(await protocolStaking.unstakeCooldownPeriod()).to.equal(kmsCooldown);

      // Verify the staking token address
      const zamaTokenAddress = getRequiredEnvVar('ZAMA_TOKEN_ADDRESS');
      expect(await protocolStaking.stakingToken()).to.equal(zamaTokenAddress);

      const { deployer } = await hre.getNamedAccounts();

      // Verify the deployer has the default admin role (governor) initially
      const DEFAULT_ADMIN_ROLE = ethers.ZeroHash;
      expect(await protocolStaking.hasRole(DEFAULT_ADMIN_ROLE, deployer)).to.be.true;

      // Verify the deployer has the manager role initially
      expect(await protocolStaking.hasRole(MANAGER_ROLE, deployer)).to.be.true;
    });

    it('Should deploy the KMS ProtocolStaking implementation contract', async function () {
      const kmsTokenName = getRequiredEnvVar('PROTOCOL_STAKING_KMS_TOKEN_NAME');
      const implName = kmsTokenName + '_Impl';

      // Get the deployed implementation contract
      const implDeployment = await hre.deployments.get(implName);

      // Verify the implementation was deployed
      await expectContractDeployed(implDeployment.address);
    });
  });

  describe('Helper Functions', function () {
    it('Should generate correct ProtocolStaking proxy name', function () {
      expect(getProtocolStakingProxyName('MyContract')).to.equal('MyContract_Proxy');
    });

    it('Should generate correct ProtocolStaking implementation name', function () {
      expect(getProtocolStakingImplName('MyContract')).to.equal('MyContract_Impl');
    });
  });
});
