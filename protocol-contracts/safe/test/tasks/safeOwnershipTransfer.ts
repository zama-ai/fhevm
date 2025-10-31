import { expect } from "chai";
import hre from "hardhat";

import { getRequiredEnvVar } from "../../tasks/utils/loadVariables";
import { makeDeployerOnlyOwner } from "../../test/utils/safeOwners";
import { SafeL2 } from "../../typechain-types";

describe("Ownership transfers", function () {
  // The initial threshold is 1
  const initialThreshold = 1;

  // Get the MultiSend contract address
  const multiSendAddress = getRequiredEnvVar("MULTISEND_ADDRESS");

  // Define variables
  let deployer: string;
  let safeProxy: SafeL2;
  let safeAddress: string;
  let owners: string[];
  let threshold: bigint;
  let initialOwners: string[];
  let newOwners: string[];

  before(async () => {
    // Get the named accounts addresses
    const namedAccounts = await hre.getNamedAccounts();
    deployer = namedAccounts.deployer;

    // The initial owners should only include the deployer
    initialOwners = [deployer];

    // Define the new owners as all the accounts except the deployer
    newOwners = Object.values(namedAccounts).slice(1);

    // Get the deployed contract:
    // - SafeL2Proxy is the name of the proxy contract
    // - SafeL2 is the name of the implementation contract
    const safeProxyDeployment = await hre.deployments.get("SafeL2Proxy");
    safeAddress = safeProxyDeployment.address;
    safeProxy = await hre.ethers.getContractAt("SafeL2", safeAddress);
  });

  it("Should add new owners to the Safe", async function () {
    // Check that the initial owners and threshold are correct
    owners = await safeProxy.getOwners();
    threshold = await safeProxy.getThreshold();
    expect(new Set(owners)).to.deep.equal(new Set(initialOwners));
    expect(threshold).to.equal(initialThreshold);

    // Add the new owners to the Safe:
    // - the deployer is kept as owner
    // - threshold is kept at 1
    const newOwnersString = newOwners.join(",");
    await hre.run("task:addOwnersToSafe", { newOwners: newOwnersString });

    // Check that the owners are now the deployer and the new owners only
    const expectedOwnersAsString = initialOwners.concat(newOwners).join(",");
    await hre.run("task:checkSafeOwners", {
      expectedOwners: expectedOwnersAsString,
    });

    // Check that the threshold is still 1
    threshold = await safeProxy.getThreshold();
    expect(threshold).to.equal(initialThreshold);
  });

  it("Should remove the deployer from the owners and update the threshold", async function () {
    // Remove the deployer from the owners and update the threshold
    await hre.run("task:removeDeployerFromSafeOwnersAndUpdateThreshold");

    // Check that the owners are now the new owners only
    const expectedOwnersAsString = newOwners.join(",");
    await hre.run("task:checkSafeOwners", {
      expectedOwners: expectedOwnersAsString,
    });

    // Check that the threshold is updated to the new threshold (found in environment variables)
    const newThreshold = Number(getRequiredEnvVar("SAFE_NEW_THRESHOLD"));
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
