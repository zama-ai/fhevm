import { Wallet } from "ethers";
import { task, types } from "hardhat/config";

import { getRequiredEnvVar } from "./utils/loadVariables";
import { pascalCaseToSnakeCase } from "./utils/stringOps";

task(
  "task:transferGatewayOwnership",
  "Transfers ownership of the gateway contracts to the provided address. This can only be used if the current owner is an EOA.",
)
  .addParam(
    "currentOwnerPrivateKey",
    "Private key of the current owner of the gateway contracts.",
    undefined,
    types.string,
  )
  .addParam("newOwnerAddress", "Address of the new owner of the gateway contracts.", undefined, types.string)
  .setAction(async function ({ currentOwnerPrivateKey, newOwnerAddress }, { ethers }) {
    // Get the current owner wallet.
    const currentOwner = new Wallet(currentOwnerPrivateKey).connect(ethers.provider);

    // Get the GatewayConfig contract: its owner is the owner of all gateway contracts
    const gatewayConfigSnakeCase = pascalCaseToSnakeCase("GatewayConfig");
    const gatewayConfigAddressEnvVarName = `${gatewayConfigSnakeCase.toUpperCase()}_ADDRESS`;
    const gatewayConfigContractAddress = getRequiredEnvVar(gatewayConfigAddressEnvVarName);
    const gatewayConfigContract = await ethers.getContractAt("GatewayConfig", gatewayConfigContractAddress);

    // Transfer ownership of the GatewayConfig contract to the destination address.
    const tx = await gatewayConfigContract.connect(currentOwner).transferOwnership(newOwnerAddress);

    await tx.wait();

    console.log(
      `Ownership of GatewayConfig contract ${gatewayConfigContractAddress} successfully transferred to EOA ${newOwnerAddress}`,
    );
  });

task(
  "task:acceptGatewayOwnership",
  `Accepts ownership of the gateway contracts. This can only be used if the new owner is an EOA.`,
)
  .addParam(
    "newOwnerPrivateKey",
    "Private key of the new owner that will accept the ownership of the gateway contracts.",
    undefined,
    types.string,
  )
  .setAction(async function ({ newOwnerPrivateKey }, { ethers }) {
    // Get the new owner wallet.
    const newOwner = new Wallet(newOwnerPrivateKey).connect(ethers.provider);

    // Get the GatewayConfig contract: its owner is the owner of all gateway contracts
    const gatewayConfigSnakeCase = pascalCaseToSnakeCase("GatewayConfig");
    const gatewayConfigAddressEnvVarName = `${gatewayConfigSnakeCase.toUpperCase()}_ADDRESS`;
    const gatewayConfigContractAddress = getRequiredEnvVar(gatewayConfigAddressEnvVarName);
    const gatewayConfigContract = await ethers.getContractAt("GatewayConfig", gatewayConfigContractAddress);

    // Accept the ownership of the GatewayConfig contract.
    const tx = await gatewayConfigContract.connect(newOwner).acceptOwnership();

    await tx.wait();

    console.log(
      `Ownership of GatewayConfig contract ${gatewayConfigContractAddress} successfully accepted by EOA ${newOwner.address}`,
    );
  });
