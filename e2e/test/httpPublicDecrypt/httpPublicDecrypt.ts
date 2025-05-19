import { ethers } from "hardhat";

import { createInstances } from "../instance";
import { getSigners, initSigners } from "../signers";
import { assert } from "chai";

describe("HTTPPublicDecrypt", function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    const contractFactory = await ethers.getContractFactory(
      "HTTPPublicDecrypt"
    );

    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    await this.contract.waitForDeployment();
    this.contractAddress = await this.contract.getAddress();
    this.instances = await createInstances(this.signers);
  });

  it("test HTTPPublicDecrypt ebool", async function () {
    const handleBool = await this.contract.xBool();
    const res = await this.instances.alice.publicDecrypt([handleBool]);
    const expectedRes = {
      [handleBool]:
        true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it("test HTTPPublicDecrypt mixed", async function () {
    const handleBool = await this.contract.xBool();
    const handle32 = await this.contract.xUint32();
    const handleAddress = await this.contract.xAddress();
    const handleBytes128 = await this.contract.xBytes128();
    const res = await this.instances.alice.publicDecrypt([
      handleBool,
      handleBytes128,
      handle32,
      handleAddress,
    ]);
    const expectedRes = {
      [handleBool]:
        true,
      [handleBytes128]:
        "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000d3f1e794f90b63477d50293f0ff0d232ca3f485213a1",
      [handle32]:
        242n,
      [handleAddress]:
        "0xfC4382C084fCA3f4fB07c3BCDA906C01797595a8",
    };
    assert.deepEqual(res, expectedRes);
  });
});
