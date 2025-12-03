import { getOperatorRewarderName, getOperatorStakingName, getProtocolStakingProxyName } from '../../tasks/deployment';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { expectContractDeployed } from '../utils';
import { expect } from 'chai';
import hre from 'hardhat';

describe('OperatorStaking Deployment', function () {
  describe('Coprocessor OperatorStaking', function () {
    it('Should deploy all coprocessor OperatorStaking contracts with correct configuration', async function () {
      const numOperatorStakingCopro = parseInt(getRequiredEnvVar('NUM_OPERATOR_STAKING_COPRO'));
      const coproProtocolStakingTokenName = getRequiredEnvVar('PROTOCOL_STAKING_COPRO_TOKEN_NAME');
      const coproProtocolStakingProxyDeployment = await hre.deployments.get(
        getProtocolStakingProxyName(coproProtocolStakingTokenName),
      );

      // Test each coprocessor operator staking contract
      for (let i = 0; i < numOperatorStakingCopro; i++) {
        const coproTokenName = getRequiredEnvVar(`OPERATOR_STAKING_COPRO_TOKEN_NAME_${i}`);
        const coproTokenSymbol = getRequiredEnvVar(`OPERATOR_STAKING_COPRO_TOKEN_SYMBOL_${i}`);
        const coproOwnerAddress = getRequiredEnvVar(`OPERATOR_STAKING_COPRO_OWNER_ADDRESS_${i}`);

        // Get the deployed operator staking contract
        const operatorStakingDeployment = await hre.deployments.get(getOperatorStakingName(coproTokenName));
        const operatorStaking = await hre.ethers.getContractAt('OperatorStaking', operatorStakingDeployment.address);

        // Verify the contract was deployed
        await expectContractDeployed(operatorStakingDeployment.address);

        // Verify the contract configuration
        expect(await operatorStaking.name()).to.equal(coproTokenName);
        expect(await operatorStaking.symbol()).to.equal(coproTokenSymbol);
        expect(await operatorStaking.protocolStaking()).to.equal(coproProtocolStakingProxyDeployment.address);
        expect(await operatorStaking.owner()).to.equal(coproOwnerAddress);

        // Verify the rewarder was deployed and has bytecode
        const rewarderAddress = await operatorStaking.rewarder();
        await expectContractDeployed(rewarderAddress);
      }
    });
  });

  describe('KMS OperatorStaking', function () {
    it('Should deploy all KMS OperatorStaking contracts with correct configuration', async function () {
      const numOperatorStakingKms = parseInt(getRequiredEnvVar('NUM_OPERATOR_STAKING_KMS'));
      const kmsProtocolStakingTokenName = getRequiredEnvVar('PROTOCOL_STAKING_KMS_TOKEN_NAME');
      const kmsProtocolStakingProxyDeployment = await hre.deployments.get(
        getProtocolStakingProxyName(kmsProtocolStakingTokenName),
      );

      // Test each KMS operator staking contract
      for (let i = 0; i < numOperatorStakingKms; i++) {
        const kmsTokenName = getRequiredEnvVar(`OPERATOR_STAKING_KMS_TOKEN_NAME_${i}`);
        const kmsTokenSymbol = getRequiredEnvVar(`OPERATOR_STAKING_KMS_TOKEN_SYMBOL_${i}`);
        const kmsOwnerAddress = getRequiredEnvVar(`OPERATOR_STAKING_KMS_OWNER_ADDRESS_${i}`);

        // Get the deployed operator staking contract
        const operatorStakingDeployment = await hre.deployments.get(getOperatorStakingName(kmsTokenName));
        const operatorStaking = await hre.ethers.getContractAt('OperatorStaking', operatorStakingDeployment.address);

        // Verify the contract was deployed
        await expectContractDeployed(operatorStakingDeployment.address);

        // Verify the contract configuration
        expect(await operatorStaking.name()).to.equal(kmsTokenName);
        expect(await operatorStaking.symbol()).to.equal(kmsTokenSymbol);
        expect(await operatorStaking.protocolStaking()).to.equal(kmsProtocolStakingProxyDeployment.address);
        expect(await operatorStaking.owner()).to.equal(kmsOwnerAddress);

        // Verify the rewarder was deployed and has bytecode
        const rewarderAddress = await operatorStaking.rewarder();
        await expectContractDeployed(rewarderAddress);
      }
    });
  });

  describe('Helper Functions', function () {
    it('Should generate correct OperatorStaking names', function () {
      expect(getOperatorStakingName('TestToken')).to.equal('TestToken_Staking');
      expect(getOperatorStakingName('COPRO')).to.equal('COPRO_Staking');
      expect(getOperatorStakingName('KMS')).to.equal('KMS_Staking');
    });

    it('Should generate correct OperatorRewarder names', function () {
      expect(getOperatorRewarderName('TestToken')).to.equal('TestToken_Rewarder');
      expect(getOperatorRewarderName('COPRO')).to.equal('COPRO_Rewarder');
      expect(getOperatorRewarderName('KMS')).to.equal('KMS_Rewarder');
    });
  });
});
