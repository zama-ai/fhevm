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
      "0x826feb7d3125d43475f3710144322d5eeabe22df63ff00000000000030390000":
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
      "0x826feb7d3125d43475f3710144322d5eeabe22df63ff00000000000030390000":
        true,
      "0x48c1b3df0b6e96d395c1d4f0dc88f7688261c26eb0ff00000000000030390a00":
        "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000d3f1e794f90b63477d50293f0ff0d232ca3f485213a1",
      "0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400":
        242n,
      "0x207db7f48ef83342828ff2084e891be48f9db07691ff00000000000030390700":
        "0xfC4382C084fCA3f4fB07c3BCDA906C01797595a8",
    };
    assert.deepEqual(res, expectedRes);
  });
});
