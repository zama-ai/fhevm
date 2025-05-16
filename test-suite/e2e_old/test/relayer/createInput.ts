import { expect } from "chai";

import { createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";

describe("Test input creation", function () {
  before(async function () {
    await initSigners();
    this.signers = await getSigners();
    this.httpz = await createInstance();
  });

  it("should create an input and send it to the gateway", async function () {
    const input = this.httpz.createEncryptedInput(
      "0x1337AA343Db8D44238Fe40486aDeECdf354e1f60",
      this.signers.alice.address,
    );
    input.add4(9n);
    input.add128(13n);
    const { handles, inputProof } = await input.encrypt();
    expect(inputProof).to.not.equal(null);
    expect(handles.length).to.be.equal(2);
  });
});
