import { DeployFunction } from 'hardhat-deploy/types';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

const func: DeployFunction = async function (hre: HardhatRuntimeEnvironment) {
  const { deployer } = await hre.getNamedAccounts();
  const { deploy } = hre.deployments;

  const deployed = await deploy('EncryptedERC20', {
    from: deployer,
    args: ['Naraggara', 'NARA'],
    log: true,
  });

  console.log(`EncryptedERC20 contract: `, deployed.address);
};
export default func;
func.id = 'deploy_encryptedERC20'; // id required to prevent reexecution
func.tags = ['EncryptedERC20'];
