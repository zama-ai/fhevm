import Safe from "@safe-global/protocol-kit";
import { task } from "hardhat/config";
import { Network, HardhatEthersHelpers } from "hardhat/types";

import { getRequiredEnvVar } from "./utils/loadVariables";

async function getSafeProxyAndAddress(ethers: HardhatEthersHelpers) {
  const safeAddress = getRequiredEnvVar("SAFE_ADDRESS");
  const safeProxy = await ethers.getContractAt("SafeL2", safeAddress);
  return { safeProxy, safeAddress };
}

async function getSafeKitDeployer(
  deployer: string,
  safeAddress: string,
  network: Network,
  ethers: HardhatEthersHelpers,
) {
  // Define the contract networks
  const chain = await ethers.provider.getNetwork();
  const chainIdKey = chain.chainId.toString();

  // Get the MultiSend contract address
  const multiSendAddress = getRequiredEnvVar("MULTISEND_ADDRESS");

  const contractNetworks = {
    [chainIdKey]: {
      multiSendAddress,
      multiSendCallOnlyAddress: multiSendAddress,
    },
  };

  // Initialize a SafeKit instance with the deployer
  const safeKitDeployer = await Safe.init({
    provider: network.provider,
    signer: deployer,
    safeAddress,
    contractNetworks,
  });

  return safeKitDeployer;
}

// Add owners to the Safe
// Also keeps the deployer as owner and threshold at 1
// Example usage:
// npx hardhat task:addOwnersToSafe \
// --newOwners \
// "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC,0x90F79bf6EB2c4f870365E785982E1f101E93b906"
task("task:addOwnersToSafe")
  .addParam(
    "newOwners",
    "Addresses of the new owners of the Safe, comma-separated",
  )
  .setAction(async function (
    { newOwners },
    { getNamedAccounts, ethers, network },
  ) {
    // Get the deployer
    const { deployer } = await getNamedAccounts();

    // Get the Safe proxy and address
    const { safeProxy, safeAddress } = await getSafeProxyAndAddress(ethers);

    // Make sure the deployer is an owner of the SafeL2Proxy contract
    const safeOwners = await safeProxy.getOwners();
    if (!safeOwners.includes(deployer)) {
      throw new Error(
        `Deployer should be an owner of the SafeL2Proxy contract. 
        Current owners: ${safeOwners.join(", ")}, expected: ${deployer}`,
      );
    }

    // Make sure the threshold is 1
    const threshold = await safeProxy.getThreshold();
    if (threshold !== BigInt(1)) {
      throw new Error(`Threshold should be 1. Current threshold: ${threshold}`);
    }

    // Get the SafeKit deployer
    const safeKitDeployer = await getSafeKitDeployer(
      deployer,
      safeAddress,
      network,
      ethers,
    );

    // Parse the newOwners string into an array of strings
    const newOwnersAsArray = newOwners.split(",");

    // Generate the transactions to add the new owners, without updating the threshold
    const addOwnersTxsData = [];
    for (const newOwnerAddress of newOwnersAsArray) {
      addOwnersTxsData.push(
        (
          await safeKitDeployer.createAddOwnerTx({
            ownerAddress: newOwnerAddress,
          })
        ).data,
      );
    }

    // Create, sign and execute the transaction batch to add the new owners in a single transaction,
    // using the deployer (the Safe's current only owner)
    const batch = await safeKitDeployer.createTransaction({
      transactions: addOwnersTxsData,
    });
    await safeKitDeployer.signTransaction(batch);
    await safeKitDeployer.executeTransaction(batch);
  });

// Check that the owners of the Safe are set as expected
// Example usage:
// npx hardhat task:checkSafeOwners \
// --expectedOwners \
// "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC,0x90F79bf6EB2c4f870365E785982E1f101E93b906"
task("task:checkSafeOwners")
  .addParam(
    "expectedOwners",
    "Addresses of the expected owners of the Safe, comma-separated",
  )
  .setAction(async function ({ expectedOwners }, { ethers }) {
    // Get the Safe proxy
    const { safeProxy } = await getSafeProxyAndAddress(ethers);

    // Parse the expectedOwners string into an array of strings
    const expectedOwnersAsArray = expectedOwners.split(",");

    // Check that the owners are correctly set in the Safe
    const owners = await safeProxy.getOwners();

    // Check that the number of owners is correct
    if (owners.length !== expectedOwnersAsArray.length) {
      throw new Error(
        `The number of owners in the Safe is incorrect. Expected: ${expectedOwners} 
      (length ${expectedOwnersAsArray.length}), Got: ${owners.join(", ")} (length ${owners.length})`,
      );
    }

    // Check that all owners are present in the expected owners
    for (const owner of owners) {
      if (!expectedOwnersAsArray.includes(owner)) {
        throw new Error(
          `The owner ${owner} is not in the expected owners. Expected: ${expectedOwners}, Got: ${owners.join(", ")}`,
        );
      }
    }
  });

// Remove deployer from the Safe and update the threshold
// Example usage:
// npx hardhat task:removeDeployerFromSafeOwnersAndUpdateThreshold
task("task:removeDeployerFromSafeOwnersAndUpdateThreshold").setAction(
  async function (_, { getNamedAccounts, ethers, network }) {
    // Get the deployer
    const { deployer } = await getNamedAccounts();

    // Get the Safe proxy and address
    const { safeProxy, safeAddress } = await getSafeProxyAndAddress(ethers);

    // Make sure the deployer is an owner of the SafeL2Proxy contract
    const safeOwners = await safeProxy.getOwners();
    if (!safeOwners.includes(deployer)) {
      throw new Error(
        `Deployer should be an owner of the SafeL2Proxy contract. 
        Current owners: ${safeOwners.join(", ")}, expected: ${deployer}`,
      );
    }

    // Make sure the threshold is 1
    const threshold = await safeProxy.getThreshold();
    if (threshold !== BigInt(1)) {
      throw new Error(`Threshold should be 1. Current threshold: ${threshold}`);
    }

    // Get the SafeKit deployer
    const safeKitDeployer = await getSafeKitDeployer(
      deployer,
      safeAddress,
      network,
      ethers,
    );

    // Get the new threshold
    const newThreshold = Number(getRequiredEnvVar("SAFE_NEW_THRESHOLD"));

    // Generate the transaction to remove the deployer from the owners and update the threshold
    const removeOwnerTx = await safeKitDeployer.createRemoveOwnerTx({
      ownerAddress: deployer,
      threshold: newThreshold,
    });

    // Create, sign and execute the transaction using the deployer (the Safe's current only owner)
    const batch = await safeKitDeployer.createTransaction({
      transactions: [removeOwnerTx.data],
    });
    await safeKitDeployer.signTransaction(batch);
    await safeKitDeployer.executeTransaction(batch);
  },
);
