import assert from "assert";
import * as fs from "fs";
import { type DeployFunction, Deployment } from "hardhat-deploy/types";

import { getRequiredEnvVar } from "../tasks/utils/loadVariables";

const contractName = "FeesSenderToBurner";

const deploy: DeployFunction = async (hre) => {
  const { getNamedAccounts, deployments } = hre;

  const { deploy } = deployments;
  const { deployer } = await getNamedAccounts();

  assert(deployer, "Missing named deployer account");

  console.log(`Network: ${hre.network.name}`);
  console.log(`Deployer: ${deployer}`);

  const oftAddress = getRequiredEnvVar("ZAMA_OFT_ADDRESS");
  // Use the PROTOCOL_FEES_BURNER_ADDRESS if provided.
  let protocolFeesBurner = process.env.PROTOCOL_FEES_BURNER_ADDRESS;
  // Otherwise (when deploying both contracts in the same script), look up for the ProtocolFeesBurner address from the deployments.
  if (!protocolFeesBurner) {
    // Match a deployment on gateway-mainnet to ethereum-mainnet, otherwise with ethereum-testnet.
    const protocolFeesBurnerNetworkname =
      hre.network.name === "gateway-mainnet" ? "ethereum-mainnet" : "ethereum-testnet";

    const protocolFeesBurnerDeployment: Deployment = JSON.parse(
      fs.readFileSync(`deployments/${protocolFeesBurnerNetworkname}/ProtocolFeesBurner.json`, "utf8"),
    );
    protocolFeesBurner = protocolFeesBurnerDeployment.address;
  }

  if (!protocolFeesBurner) {
    throw new Error(`The ProtocolFeesBurner address cannot be empty: ${protocolFeesBurner}`);
  }

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
