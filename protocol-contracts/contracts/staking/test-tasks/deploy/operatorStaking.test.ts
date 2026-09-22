import {
  getOperatorRewarderName,
  getOperatorStakingName,
  getOperatorStakingImplName,
  getProtocolStakingProxyName,
} from '../../tasks/deployment';
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
        // Get the env vars for the coprocessor operator staking contract
        const coproTokenName = getRequiredEnvVar(`OPERATOR_STAKING_COPRO_TOKEN_NAME_${i}`);
        const coproTokenSymbol = getRequiredEnvVar(`OPERATOR_STAKING_COPRO_TOKEN_SYMBOL_${i}`);

        // Get the deployed operator staking contract
        const operatorStakingDeployment = await hre.deployments.get(getOperatorStakingName(coproTokenName));
        const operatorStaking = await hre.ethers.getContractAt('OperatorStaking', operatorStakingDeployment.address);

        // Verify the contract was deployed
        await expectContractDeployed(operatorStakingDeployment.address);

        // Verify the contract configuration
        expect(await operatorStaking.name()).to.equal(coproTokenName);
        expect(await operatorStaking.symbol()).to.equal(coproTokenSymbol);
        expect(await operatorStaking.protocolStaking()).to.equal(coproProtocolStakingProxyDeployment.address);

        // Verify the rewarder was deployed and has bytecode
        const rewarderAddress = await operatorStaking.rewarder();
        await expectContractDeployed(rewarderAddress);

        // Get the env vars for the coprocessor operator rewarder contract
        const coproBeneficiaryAddress = getRequiredEnvVar(`OPERATOR_REWARDER_COPRO_BENEFICIARY_${i}`);
        const coproInitialFee = parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_COPRO_FEE_${i}`));
        const coproInitialMaxFee = parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_COPRO_MAX_FEE_${i}`));

        // Verify the rewarder contract configuration
        const rewarder = await hre.ethers.getContractAt('OperatorRewarder', rewarderAddress);
        expect(await rewarder.beneficiary()).to.equal(coproBeneficiaryAddress);
        expect(await rewarder.feeBasisPoints()).to.equal(coproInitialFee);
        expect(await rewarder.maxFeeBasisPoints()).to.equal(coproInitialMaxFee);
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
        // Get the env vars for the KMS operator staking contract
        const kmsTokenName = getRequiredEnvVar(`OPERATOR_STAKING_KMS_TOKEN_NAME_${i}`);
        const kmsTokenSymbol = getRequiredEnvVar(`OPERATOR_STAKING_KMS_TOKEN_SYMBOL_${i}`);

        // Get the deployed operator staking contract
        const operatorStakingDeployment = await hre.deployments.get(getOperatorStakingName(kmsTokenName));
        const operatorStaking = await hre.ethers.getContractAt('OperatorStaking', operatorStakingDeployment.address);

        // Verify the contract was deployed
        await expectContractDeployed(operatorStakingDeployment.address);

        // Verify the contract configuration
        expect(await operatorStaking.name()).to.equal(kmsTokenName);
        expect(await operatorStaking.symbol()).to.equal(kmsTokenSymbol);
        expect(await operatorStaking.protocolStaking()).to.equal(kmsProtocolStakingProxyDeployment.address);

        // Verify the rewarder was deployed and has bytecode
        const rewarderAddress = await operatorStaking.rewarder();
        await expectContractDeployed(rewarderAddress);

        // Get the env vars for the KMS operator rewarder contract
        const kmsBeneficiaryAddress = getRequiredEnvVar(`OPERATOR_REWARDER_KMS_BENEFICIARY_${i}`);
        const kmsInitialFee = parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_KMS_FEE_${i}`));
        const kmsInitialMaxFee = parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_KMS_MAX_FEE_${i}`));

        // Verify the rewarder contract configuration
        const rewarder = await hre.ethers.getContractAt('OperatorRewarder', rewarderAddress);
        expect(await rewarder.beneficiary()).to.equal(kmsBeneficiaryAddress);
        expect(await rewarder.feeBasisPoints()).to.equal(kmsInitialFee);
        expect(await rewarder.maxFeeBasisPoints()).to.equal(kmsInitialMaxFee);
      }
    });
  });

  describe('Helper Functions', function () {
    it('Should generate correct OperatorStaking proxy name', function () {
      expect(getOperatorStakingName('MyContract')).to.equal('MyContract_Staking_Proxy');
    });

    it('Should generate correct OperatorStaking implementation name', function () {
      expect(getOperatorStakingImplName('MyContract')).to.equal('MyContract_Staking_Impl');
    });

    it('Should generate correct OperatorRewarder name', function () {
      expect(getOperatorRewarderName('MyContract')).to.equal('MyContract_Rewarder');
    });
  });
});
