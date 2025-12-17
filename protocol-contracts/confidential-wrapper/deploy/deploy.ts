import { DeployFunction } from "hardhat-deploy/types";
import { HardhatRuntimeEnvironment } from "hardhat/types";

const func: DeployFunction = async function (hre: HardhatRuntimeEnvironment) {
  const { deployer } = await hre.getNamedAccounts();
  const { deploy } = hre.deployments;

  // Deploy SanctionsList first
  const deployedSanctionsList = await deploy("SanctionsList", {
    from: deployer,
    log: true,
  });

  // Deploy FeeManager
  const deployedFeeManager = await deploy("FeeManager", {
    from: deployer,
    log: true,
  });

  // Deploy AdminProvider with SanctionsList and FeeManager
  const deployedAdminProvider = await deploy("AdminProvider", {
    from: deployer,
    args: [deployedSanctionsList.address, deployedFeeManager.address],
    log: true,
  });

  // Deploy WrapperFactory with AdminProvider
  const deployedWrapperFactory = await deploy("WrapperFactory", {
    from: deployer,
    args: [deployedAdminProvider.address],
    log: true,
  });

  // Set up initial fee configuration
  const feeManagerContract = await hre.ethers.getContractAt("FeeManager", deployedFeeManager.address);
  const deployFee = "1000000000000000000"; // 1 ETH in wei
  const feeRecipient = deployer; // Use deployer as fee recipient for now
  
  await feeManagerContract.setDeployFee(deployFee);
  await feeManagerContract.setFeeRecipient(feeRecipient);

  console.log(`SanctionsList contract: `, deployedSanctionsList.address);
  console.log(`FeeManager contract: `, deployedFeeManager.address);
  console.log(`AdminProvider contract: `, deployedAdminProvider.address);
  console.log(`WrapperFactory contract: `, deployedWrapperFactory.address);
  console.log(`Deploy fee set to: `, deployFee, " wei");
  console.log(`Fee recipient set to: `, feeRecipient);
};
export default func;
func.id = "deploy_wrapper_factory"; // id required to prevent reexecution
func.tags = ["WrapperFactory", "AdminProvider", "FeeManager", "SanctionsList"];
