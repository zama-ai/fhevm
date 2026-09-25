import assert from "assert";
import { type DeployFunction } from "hardhat-deploy/types";

import { getRequiredEnvVar } from "../tasks/utils/loadVariables";

const contractName = "ProtocolFeesBurner";

const deploy: DeployFunction = async (hre) => {
  const { getNamedAccounts, deployments } = hre;

  const { deploy } = deployments;
  const { deployer } = await getNamedAccounts();

  assert(deployer, "Missing named deployer account");

  console.log(`Network: ${hre.network.name}`);
  console.log(`Deployer: ${deployer}`);

  const tokenAddress = getRequiredEnvVar("ZAMA_ERC20_ADDRESS");

  if (hre.network.name === "ethereum-testnet" || hre.network.name === "ethereum-mainnet") {
    const { address } = await deploy(contractName, {
      from: deployer,
      args: [tokenAddress],
      log: true,
      skipIfAlreadyDeployed: false,
    });
    console.log(`Deployed contract: ${contractName}, network: ${hre.network.name}, address: ${address}`);
  }
};

deploy.tags = [contractName];

export default deploy;
