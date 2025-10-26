import assert from "assert";
import { type DeployFunction } from "hardhat-deploy/types";

import { getRequiredEnvVar } from "./utils/loadVariables";

const contractName = "FeesSenderToBurner";

const deploy: DeployFunction = async (hre) => {
  const { getNamedAccounts, deployments } = hre;

  const { deploy } = deployments;
  const { deployer } = await getNamedAccounts();

  assert(deployer, "Missing named deployer account");

  console.log(`Network: ${hre.network.name}`);
  console.log(`Deployer: ${deployer}`);

  const oftAddress = getRequiredEnvVar("ZAMA_OFT_ADDRESS");
  const protocolFeesBurner = getRequiredEnvVar("PROTOCOL_FEES_BURNER_ADDRESS");

  const { address } = await deploy(contractName, {
    from: deployer,
    args: [oftAddress, protocolFeesBurner],
    log: true,
    skipIfAlreadyDeployed: false,
  });
  console.log(`Deployed contract: ${contractName}, network: ${hre.network.name}, address: ${address}`);
};

deploy.tags = [contractName];

export default deploy;
