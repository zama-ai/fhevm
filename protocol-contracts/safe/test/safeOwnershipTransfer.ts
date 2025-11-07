import Safe from "@safe-global/protocol-kit";
import { expect } from "chai";
import { ethers, network } from "hardhat";
import hre from "hardhat";

import { getRequiredEnvVar } from "../tasks/utils/loadVariables";
import { SafeL2 } from "../typechain-types";
import { makeDeployerOnlyOwner } from "./utils/safeOwners";

describe("Ownership transfer", function () {
  // The initial threshold is 1
  const initialThreshold = 1;

  // Get the MultiSend contract address
  const multiSendAddress = getRequiredEnvVar("MULTISEND_ADDRESS");

  // Define variables
  let deployer: string;
  let safeSingleton: any;
  let safeProxy: SafeL2;
  let safeProxyAddress: string;
  let owners: string[];
  let threshold: bigint;
  let initialOwners: string[];
  let newOwners: string[];

  before(async () => {
    // Get the deployer
    const namedAccounts = await hre.getNamedAccounts();
    deployer = namedAccounts.deployer;

    // The initial owners is only the deployer
    initialOwners = [deployer];

    // Define the new owners as all the accounts except the deployer
    newOwners = Object.values(namedAccounts).slice(1);

    // Get the deployed contract:
    // - SafeL2Proxy is the name of the proxy contract
    // - SafeL2 is the name of the implementation contract
    const safeProxyDeployment = await hre.deployments.get("SafeL2Proxy");
    safeProxyAddress = safeProxyDeployment.address;
    safeProxy = await hre.ethers.getContractAt("SafeL2", safeProxyAddress);
    safeSingleton = await hre.ethers.getContractAt("SafeL2", safeProxyAddress);
  });

  it("Transfer ownerships of Safe contract", async function () {
    // Check that the initial owners and threshold are correct
    owners = await safeProxy.getOwners();
    threshold = await safeProxy.getThreshold();
    expect(new Set(owners)).to.deep.equal(new Set(initialOwners));
    expect(threshold).to.equal(initialThreshold);

    // Define the contract networks
    const chain = await ethers.provider.getNetwork();
    const chainIdKey = chain.chainId.toString();

    const contractNetworks = {
      [chainIdKey]: {
        multiSendAddress,
        multiSendCallOnlyAddress: multiSendAddress,
      },
    };

    // Initialize a SafeKit instance with the deployer (currently the only owner of the Safe)
    const safeKitDeployer = await Safe.init({
      provider: network.provider,
      signer: deployer,
      safeAddress: safeProxyAddress,
      contractNetworks,
    });

    // Generate the transactions to add the new owners, without updating the threshold
    const txsData = [];
    for (const newOwnerAddress of newOwners) {
      txsData.push(
        (
          await safeKitDeployer.createAddOwnerTx({
            ownerAddress: newOwnerAddress,
          })
        ).data,
      );
    }

    // Create, sign and execute the transaction batch using the deployer (the Safe's current only owner)
    const batch1 = await safeKitDeployer.createTransaction({
      transactions: txsData,
    });
    await safeKitDeployer.signTransaction(batch1);
    await safeKitDeployer.executeTransaction(batch1);

    // Check that the owners now include the new owners
    owners = await safeProxy.getOwners();
    expect(new Set(owners)).to.deep.equal(new Set([deployer, ...newOwners]));

    // Check that the threshold remains the same
    threshold = await safeProxy.getThreshold();
    expect(threshold).to.equal(initialThreshold);

    // Generate the transaction to remove the deployer from the owners and update the
    // threshold at the same time
    const newThreshold = 2;
    const removeOwnerTx = await safeKitDeployer.createRemoveOwnerTx({
      ownerAddress: deployer,
      threshold: newThreshold,
    });

    // Create, sign and execute the transaction using the deployer (the Safe's current only owner)
    const batch2 = await safeKitDeployer.createTransaction({
      transactions: [removeOwnerTx.data],
    });
    await safeKitDeployer.signTransaction(batch2);
    await safeKitDeployer.executeTransaction(batch2);

    // Check that the owners now exclude the deployer
    owners = await safeProxy.getOwners();
    expect(new Set(owners)).to.deep.equal(new Set(newOwners));

    // Check that the threshold has been updated
    threshold = await safeProxy.getThreshold();
    expect(threshold).to.equal(newThreshold);
  });

  // This task is here to avoid breaking following tests
  it("Should put back the deployer as the only owner and update the threshold to 1", async function () {
    // WARNING: `orderedOwnersToRemove` (here `newOwners`) needs to be in the same order as the
    // owners were added in. See comments in `makeDeployerOnlyOwner` for more details.
    await makeDeployerOnlyOwner(
      deployer,
      newOwners,
      safeProxy,
      multiSendAddress,
    );
  });
});
