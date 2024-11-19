import { expect } from "chai";

import { createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";

describe("Test input creation", function() {
  before(async function() {
    await initSigners();
    this.signers = await getSigners();
    this.fhevm = await createInstance();
  });

  it("should create an input", async function() {
    const input = this.fhevm.createEncryptedInput(
      "0x1337AA343Db8D44238Fe40486aDeECdf354e1f60",
      this.signers.alice.address,
    );
    input.add4(9n);
    input.add128(13n);
    const encryptedAmount = await input.encrypt();
    console.log(encryptedAmount);
    expect(true).to.be.equal(true);
  });
});
