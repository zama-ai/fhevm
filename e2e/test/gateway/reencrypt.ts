import { expect } from "chai";
import { Context } from "mocha";

import { Add } from "../../types";
import { Decrypt, createDecrypt, createInstance } from "../instance";
import { deployAddFixture } from "../operations/Add.fixture";
import { getSigners, initSigners } from "../signers";

interface AddContext extends Context {
  contract: Add;
}

describe("Test reencrypt", function () {
  let decrypt: Decrypt;
  before(async function (this: AddContext) {
    await initSigners();
    this.signers = await getSigners();
    this.fhevm = await createInstance();
    const contract = await deployAddFixture();
    this.contractAddress = await contract.getAddress();
    this.contract = contract;
    decrypt = createDecrypt(this.fhevm, this.signers.alice, this.contractAddress);
  });

  it("should reencrypt a 8bits value", async function (this: AddContext) {
    const transaction = await this.contract.add8();
    await transaction.wait();

    const handle = await this.contract.result8();

    const result = await decrypt(handle);
    expect(result).to.equal(3);
  });
});
