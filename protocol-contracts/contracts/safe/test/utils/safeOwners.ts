import Safe from "@safe-global/protocol-kit";
import { expect } from "chai";
import { ethers, network } from "hardhat";

import { SafeL2 } from "../../typechain-types";

// This function puts back the deployer as the only owner and updates the threshold to 1:
// - the deployer is added back as the only owner
// - the threshold is updated to 1
// - the other owners are removed
// Important: `orderedOwnersToRemove` needs to:
// - be in the same order as the owners were added in
// - not contain the deployer
export async function makeDeployerOnlyOwner(
  deployer: string,
  orderedOwnersToRemove: string[],
  safeProxy: SafeL2,
  multiSendAddress: string,
) {
  if (orderedOwnersToRemove.length === 0) {
    throw new Error(`"orderedOwnersToRemove" should not be empty.`);
  }

  for (const owner of orderedOwnersToRemove) {
    if (!(await safeProxy.isOwner(owner))) {
      throw new Error(
        `${owner} in "orderedOwnersToRemove" should be an owner.`,
      );
    }
    if (owner === deployer) {
      throw new Error(`Deployer should not be in "orderedOwnersToRemove".`);
    }
  }

  if (await safeProxy.isOwner(deployer)) {
    throw new Error(`Deployer should not be an owner.`);
  }

  // Define the contract networks
  const chain = await ethers.provider.getNetwork();
  const chainIdKey = chain.chainId.toString();

  const contractNetworks = {
    [chainIdKey]: {
      multiSendAddress,
      multiSendCallOnlyAddress: multiSendAddress,
    },
  };

  // Get the Safe contract address
  const safeAddress = await safeProxy.getAddress();

  // Define a SafeKit instances for  alice and bob
  const safeKitOwners = [];
  for (const owner of orderedOwnersToRemove) {
    safeKitOwners.push(
      await Safe.init({
        provider: network.provider,
        signer: owner,
        safeAddress,
        contractNetworks,
      }),
    );
  }

  // Get the first SafeKit instance to use for creating and executing the transactions
  const firstSafeKitOwner = safeKitOwners[0];

  // This function puts back the deployer as the only owner and updates the threshold to 1
  const newThreshold = 1;
  const newOwners = [deployer];

  // Add the deployer back
  const addDeployerTx = await firstSafeKitOwner.createAddOwnerTx({
    ownerAddress: newOwners[0],
    threshold: newThreshold,
  });

  // Get the tx and sign it with alice and bob (2 of the current owners)
  // It is not important who creates and executes the transaction, but since the threshold is 2, at least 2
  // signatures from owners are need to be present in the transaction.
  // It is possible to do so by making them signing the resulting transaction successively as below.
  let tx = await firstSafeKitOwner.createTransaction({
    transactions: [addDeployerTx.data],
  });

  // Sign the transaction with all the owners to make sure the threshold is met
  for (const safeKitOwner of safeKitOwners) {
    tx = await safeKitOwner.signTransaction(tx);
  }

  // Execute the transaction with the signatures
  await firstSafeKitOwner.executeTransaction(tx);

  // Initialize a SafeKit instance with the deployer: he is able to update the set of owners alone
  // since he is now an owner and the threshold is 1
  const safeKitDeployer = await Safe.init({
    provider: network.provider,
    signer: deployer,
    safeAddress,
    contractNetworks,
  });

  // Remove all the other owners
  // WARNING: the order of the owners is important here: each owner needs to be removed in the
  // same order they were added in. If not, the batched transaction will be reverted. This is
  // because the Safe contract uses linked list internally to manage the owners.
  const resetOwnersTxsData = [];
  for (const owner of orderedOwnersToRemove) {
    const resetOwnerTx = await safeKitDeployer.createRemoveOwnerTx({
      ownerAddress: owner,
      threshold: newThreshold,
    });
    resetOwnersTxsData.push(resetOwnerTx.data);
  }

  // Get the tx, sign and execute it with the deployer (the Safe's current only owner)
  const batch = await safeKitDeployer.createTransaction({
    transactions: resetOwnersTxsData,
  });
  await safeKitDeployer.signTransaction(batch);
  await safeKitDeployer.executeTransaction(batch);

  // Check that the threshold and owners are correct
  const threshold = await safeProxy.getThreshold();
  expect(threshold).to.equal(newThreshold);
  const owners = await safeProxy.getOwners();
  expect(new Set(owners)).to.deep.equal(new Set(newOwners));
}
