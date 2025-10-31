import Safe from "@safe-global/protocol-kit";
import { task, types } from "hardhat/config";
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
// npx hardhat task:addOwnersToSafe
task("task:addOwnersToSafe").setAction(async function (
  _,
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

  // Get the number of new owners to add
  const numNewOwners = Number(getRequiredEnvVar("SAFE_NUM_NEW_OWNERS"));

  // Generate the transactions to add the new owners, without updating the threshold
  const addOwnersTxsData = [];
  for (let i = 0; i < numNewOwners; i++) {
    // Get the new owner address
    const newOwnerAddress = getRequiredEnvVar(`SAFE_NEW_OWNER_ADDRESS_${i}`);

    // Generate the transaction to add the new owner
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
// It should be the new added owners, with or without the deployer depending on when this task is called
// Example usage:
// npx hardhat task:checkSafeOwners --includeDeployer true
task("task:checkSafeOwners")
  .addParam(
    "includeDeployer",
    "Whether to include the deployer in the owners",
    false,
    types.boolean,
  )
  .setAction(async function (
    { includeDeployer },
    { getNamedAccounts, ethers },
  ) {
    // Get the Safe proxy
    const { safeProxy } = await getSafeProxyAndAddress(ethers);

    // Get the number of new owners
    const numNewOwners = Number(getRequiredEnvVar("SAFE_NUM_NEW_OWNERS"));

    // Parse the expectedOwners string into an array of strings
    const expectedOwnersAsArray = [];
    for (let i = 0; i < numNewOwners; i++) {
      expectedOwnersAsArray.push(
        getRequiredEnvVar(`SAFE_NEW_OWNER_ADDRESS_${i}`),
      );
    }

    // Add the deployer to the expected owners if needed
    if (includeDeployer) {
      const { deployer } = await getNamedAccounts();
      expectedOwnersAsArray.push(deployer);
    }

    // Check that the owners are correctly set in the Safe
    const owners = await safeProxy.getOwners();

    // Check that the number of owners is correct
    if (owners.length !== expectedOwnersAsArray.length) {
      throw new Error(
        `The number of owners in the Safe is incorrect. Expected: ${expectedOwnersAsArray.join(", ")} 
      (length ${expectedOwnersAsArray.length}), Got: ${owners.join(", ")} (length ${owners.length})`,
      );
    }

    // Check that all owners are present in the expected owners
    for (const owner of owners) {
      if (!expectedOwnersAsArray.includes(owner)) {
        throw new Error(
          `The owner ${owner} is not in the expected owners. Expected: ${expectedOwnersAsArray.join(", ")}, Got: ${owners.join(", ")}`,
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
