import { expect } from "chai";
import hre from "hardhat";

import { getRequiredEnvVar } from "../../tasks/utils/loadVariables";
import { makeDeployerOnlyOwner } from "../../test/utils/safeOwners";
import { createRandomAddresses } from "../../test/utils/utils";
import { SafeL2 } from "../../typechain-types";

describe("Ownership transfers", function () {
  // Get the initial threshold
  const initialThreshold = Number(getRequiredEnvVar("SAFE_THRESHOLD"));

  // Get the MultiSend contract address
  const multiSendAddress = getRequiredEnvVar("MULTISEND_ADDRESS");

  // Define variables
  let deployer: string;
  let alice: string;
  let bob: string;
  let safeProxy: SafeL2;
  let safeAddress: string;
  let owners: string[];
  let threshold: bigint;
  let initialOwners: string[];

  before(async () => {
    // Get the named accounts addresses
    const namedAccounts = await hre.getNamedAccounts();
    deployer = namedAccounts.deployer;
    alice = namedAccounts.alice;
    bob = namedAccounts.bob;

    // The initial owners should only include the deployer
    initialOwners = [deployer];

    // Get the deployed contract:
    // - SafeL2Proxy is the name of the proxy contract
    // - SafeL2 is the name of the implementation contract
    const safeProxyDeployment = await hre.deployments.get("SafeL2Proxy");
    safeAddress = safeProxyDeployment.address;
    safeProxy = await hre.ethers.getContractAt("SafeL2", safeAddress);
  });

  it("Transfer ownership of Safe from deployer to new list in a single transaction", async function () {
    // Check that the initial owners and threshold are correct
    owners = await safeProxy.getOwners();
    threshold = await safeProxy.getThreshold();
    expect(new Set(owners)).to.deep.equal(new Set(initialOwners));
    expect(threshold).to.equal(initialThreshold);

    // Define the new owners (by including Alice and Bob) and the new threshold
    const randomOwners = createRandomAddresses(8);
    const newOwners = [alice, bob, ...randomOwners];
    const newThreshold = 2;

    // Run the task to:
    // - transfer the ownership of the Safe from the deployer to the new list
    // - remove the deployer from the owners
    // - update th threshold
    // All of the above in a single transaction
    const newOwnersString = newOwners.join(",");
    const newThresholdString = newThreshold.toString();
    await hre.run("task:transferSafeOwnershipFromDeployer", {
      newOwners: newOwnersString,
      newThreshold: newThresholdString,
    });

    // Check that the owners are now the new owners only
    owners = await safeProxy.getOwners();
    expect(new Set(owners)).to.deep.equal(new Set(newOwners));

    // Check that the threshold has been updated
    threshold = await safeProxy.getThreshold();
    expect(threshold).to.equal(newThreshold);

    // Put back the deployer as the only owner and update the threshold to 1 (to not break tests)
    // WARNING: `orderedOwnersToRemove` (here `newOwners`) needs to be in the same order as the
    // owners were added in. See comments in `makeDeployerOnlyOwner` for more details.
    await makeDeployerOnlyOwner(
      deployer,
      alice,
      bob,
      safeProxy,
      multiSendAddress,
      newOwners,
    );
  });
});
