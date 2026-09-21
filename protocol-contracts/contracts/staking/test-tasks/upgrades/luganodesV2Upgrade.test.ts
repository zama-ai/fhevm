import { deployOperatorStaking, getOperatorStakingName, OPERATOR_STAKING_CONTRACT_NAME } from '../../tasks/deployment';
import { LUGANODES_OPERATOR_STAKING_V2_CONTRACT } from '../../tasks/deployment/upgrades/luganodesV2';
import { getProtocolStakingCoproProxyAddress } from '../../tasks/utils/getAddresses';
import { expect } from 'chai';
import hre from 'hardhat';

describe('LuganodesOperatorStakingV2 Upgrade', function () {
  // Expected values after upgrade (these are the correct values)
  const EXPECTED_NAME = 'Mock Luganodes Staked ZAMA (Coprocessor)';
  const EXPECTED_SYMBOL = 'stZAMA-Mock-Luganodes-Coprocessor';

  describe('Upgrade Flow', function () {
    it('Should fix swapped name and symbol after upgrade', async function () {
      const { deployer } = await hre.getNamedAccounts();
      const deployerSigner = await hre.ethers.getSigner(deployer);

      // Get the protocol staking address
      const protocolStakingAddress = await getProtocolStakingCoproProxyAddress(hre);

      // Deploy a new operator staking contract with SWAPPED name and symbol (simulating the bug)
      // The bug was that name and symbol were swapped during initialization
      await deployOperatorStaking(
        EXPECTED_SYMBOL, // Swapped: symbol passed as name
        EXPECTED_NAME, // Swapped: name passed as symbol
        protocolStakingAddress,
        deployer,
        1000,
        100,
        hre,
      );

      // Get the deployed operator staking contract using the token name (which is the swapped symbol)
      const operatorStakingDeployment = await hre.deployments.get(getOperatorStakingName(EXPECTED_SYMBOL));
      const operatorStakingProxyAddress = operatorStakingDeployment.address;
      const operatorStaking = await hre.ethers.getContractAt(
        OPERATOR_STAKING_CONTRACT_NAME,
        operatorStakingProxyAddress,
      );

      // Verify initial state has swapped name and symbol (simulating the bug)
      // Name contains symbol value and symbol contains name value
      expect(await operatorStaking.name()).to.equal(EXPECTED_SYMBOL);
      expect(await operatorStaking.symbol()).to.equal(EXPECTED_NAME);

      // Deposit and get total assets for verifying state consistency
      await hre.run('task:depositOperatorStakingFromDeployer', {
        assets: BigInt(1000),
        receiver: deployer,
        operatorStakingAddress: operatorStakingProxyAddress,
      });
      const totalAssets = await operatorStaking.totalAssets();

      console.log('totalAssets', totalAssets);

      // Run the task to deploy the new implementation contract with new Luganodes names and symbol
      await hre.run('task:deployLuganodesOperatorStakingV2Impl', {
        operatorStakingProxyAddress,
      });

      // Retrieve the deployment artifact for the new implementation contract to get its address
      const implementationDeployment = await hre.deployments.get(LUGANODES_OPERATOR_STAKING_V2_CONTRACT + '_Impl');
      const implementationAddress = implementationDeployment.address;

      // Upgrade the proxy to the new implementation
      const calldata = // returned via: `cast calldata "reinitializeV2(string,string)" "Mock Luganodes Staked ZAMA (Coprocessor)" "stZAMA-Mock-Luganodes-Coprocessor"`
        '0x91da124c000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000284d6f636b204c7567616e6f646573205374616b6564205a414d412028436f70726f636573736f7229000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002173745a414d412d4d6f636b2d4c7567616e6f6465732d436f70726f636573736f7200000000000000000000000000000000000000000000000000000000000000';
      await operatorStaking.connect(deployerSigner).upgradeToAndCall(implementationAddress, calldata);

      // Get the upgraded operator staking V2 contract
      const operatorStakingV2 = await hre.ethers.getContractAt(
        LUGANODES_OPERATOR_STAKING_V2_CONTRACT,
        operatorStakingProxyAddress,
      );

      // Verify name and symbol are now correct in V2
      expect(await operatorStakingV2.name()).to.equal(EXPECTED_NAME);
      expect(await operatorStakingV2.symbol()).to.equal(EXPECTED_SYMBOL);

      // Verify total assets are the same after upgrade
      expect(await operatorStakingV2.totalAssets()).to.equal(totalAssets);
    });
  });
});
