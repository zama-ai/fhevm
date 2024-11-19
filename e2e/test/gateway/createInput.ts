import { expect } from "chai";

import { createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";

describe("Test input creation", function () {
  before(async function () {
    await initSigners();
    this.signers = await getSigners();
    this.fhevm = await createInstance();
  });

  it("should create an input", async function () {
    const input = this.fhevm.createEncryptedInput(
      "0x1337680e44eb49ad81a588fc8a33335d9413818e",
      this.signers.alice.address,
    );
    input.add4(9n);
    input.add128(13n);
    const encryptedAmount = await input.encrypt();
    console.log(encryptedAmount);
    expect(true).to.be.equal(true);
  });
});
