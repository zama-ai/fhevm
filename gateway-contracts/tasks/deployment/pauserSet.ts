import { Wallet } from "ethers";
import { task } from "hardhat/config";

import { getRequiredEnvVar } from "../utils/loadVariables";

// Deploy the PauserSet contract
task("task:deployPauserSet").setAction(async function (_, hre) {
  // Get a deployer wallet
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  console.log("Deploying PauserSet...");
  const pauserSetFactory = await hre.ethers.getContractFactory("PauserSet", deployer);
  const pauserSet = await pauserSetFactory.deploy();
  const pauserSetAddress = await pauserSet.getAddress();

  await hre.run("task:setContractAddress", {
    name: "PauserSet",
    address: pauserSetAddress,
  });
});
