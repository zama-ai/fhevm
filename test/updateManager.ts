import { expect } from "chai";
import hre from "hardhat";

describe("UpdateManager", function () {
  async function deployUpdateManagerFixture(fheParamsId: number) {
    const [governance] = await hre.ethers.getSigners();
    const updateManager = await hre.ethers.deployContract("UpdateManager", [governance.address, fheParamsId]);
    return { updateManager };
  }

  it("should be able to update the FheParamsId", async function () {
    const fheParamsId = 1;
    const newFheParamsId = 2;

    // Deploy the UpdateManager contract
    const { updateManager } = await deployUpdateManagerFixture(fheParamsId);

    // Update the FHE params ID
    const result = updateManager.updateFheParams(newFheParamsId);

    // Check that the event is emitted
    await expect(result).to.emit(updateManager, "UpdateFheParams");

    // Check that the FHE params ID is updated
    expect(await updateManager.getFheParamsId()).to.equal(newFheParamsId);
  });

  it("should not be able to update the FheParamsId", async function () {
    const fheParamsId = 1;
    const newFheParamsId = 2;

    // Deploy the UpdateManager contract
    const { updateManager } = await deployUpdateManagerFixture(fheParamsId);

    // Get the user signer (ie, not the governance contract that is allowed to update the FHE params ID)
    const [_, user] = await hre.ethers.getSigners();

    // Check that the user is not authorized to update the FHE params ID
    await expect(updateManager.connect(user).updateFheParams(newFheParamsId)).to.be.revertedWithCustomError(
      updateManager,
      "AccessControlUnauthorizedAccount",
    );
  });
});
