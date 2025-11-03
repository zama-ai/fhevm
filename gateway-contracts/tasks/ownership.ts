import { Wallet } from "ethers";
import { task, types } from "hardhat/config";

import { getRequiredEnvVar } from "./utils/loadVariables";
import { pascalCaseToSnakeCase } from "./utils/stringOps";

task(
  "task:transferGatewayOwnership",
  "Transfers ownership of the gateway contracts to the provided address. This can only be used if the current owner is an EOA.",
)
  .addParam("newOwnerAddress", "Address of the new owner of the gateway contracts.", undefined, types.string)
  .setAction(async function ({ newOwnerAddress }, { ethers }) {
    // Get the deployer wallet.
    const deployer = new Wallet(getRequiredEnvVar("DEPLOYER_PRIVATE_KEY")).connect(ethers.provider);

    // Get the GatewayConfig contract: its owner is the owner of all gateway contracts
    const gatewayConfigSnakeCase = pascalCaseToSnakeCase("GatewayConfig");
    const gatewayConfigAddressEnvVarName = `${gatewayConfigSnakeCase.toUpperCase()}_ADDRESS`;
    const gatewayConfigContractAddress = getRequiredEnvVar(gatewayConfigAddressEnvVarName);
    const gatewayConfigContract = await ethers.getContractAt("GatewayConfig", gatewayConfigContractAddress);

    if ((await gatewayConfigContract.owner()) !== deployer.address) {
      throw new Error(
        `The deployer account ${deployer.address} is not the owner of the GatewayConfig contract ${gatewayConfigContractAddress}`,
      );
    }

    // Transfer ownership of the GatewayConfig contract to the destination address.
    const tx = await gatewayConfigContract.connect(deployer).transferOwnership(newOwnerAddress);

    await tx.wait();

    console.log(
      `Ownership of GatewayConfig contract ${gatewayConfigContractAddress} is now successfully pending for account ${newOwnerAddress}.
       The new owner needs to send an acceptOwnership transaction to validate the transfer`,
    );
  });

task(
  "task:acceptGatewayOwnership",
  "Accepts ownership of the gateway contracts. This can only be used if the account is an EOA.",
).setAction(async function ({}, { ethers }) {
  // Get the new owner wallet.
  const newOwner = new Wallet(getRequiredEnvVar("NEW_OWNER_PRIVATE_KEY")).connect(ethers.provider);

  // Get the GatewayConfig contract: its owner is the owner of all gateway contracts
  const gatewayConfigSnakeCase = pascalCaseToSnakeCase("GatewayConfig");
  const gatewayConfigAddressEnvVarName = `${gatewayConfigSnakeCase.toUpperCase()}_ADDRESS`;
  const gatewayConfigContractAddress = getRequiredEnvVar(gatewayConfigAddressEnvVarName);
  const gatewayConfigContract = await ethers.getContractAt("GatewayConfig", gatewayConfigContractAddress);

  if ((await gatewayConfigContract.pendingOwner()) !== newOwner.address) {
    throw new Error(
      `The new owner account ${newOwner.address} is not the pending owner of the GatewayConfig contract ${gatewayConfigContractAddress}`,
    );
  }

  // Transfer ownership of the GatewayConfig contract to the destination address.
  const tx = await gatewayConfigContract.connect(newOwner).acceptOwnership();

  await tx.wait();

  console.log(
    `Ownership of GatewayConfig contract ${gatewayConfigContractAddress} has been accepted by account ${newOwner.address}`,
  );
});
