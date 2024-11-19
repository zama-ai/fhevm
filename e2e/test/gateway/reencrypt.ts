import { expect } from "chai";

import { createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";
import { deployIncrementFixture } from "./Increment.fixture";

describe("Test reencrypt", function () {
  before(async function () {
    await initSigners();
    this.signers = await getSigners();
    this.fhevm = await createInstance();
  });

  beforeEach(async function () {
    const contract = await deployIncrementFixture();
    this.contractAddress = await contract.getAddress();
    this.increment = contract;
  });

  it("should reencrypt", async function () {
    const transaction = await this.increment.increment();
    await transaction.wait();

    const counterHandle = await this.increment.counter();
    const { publicKey: publicKeyAlice, privateKey: privateKeyAlice } = this.fhevm.generateKeypair();
    const eip712 = this.fhevm.createEIP712(publicKeyAlice, this.contractAddress);
    const signatureAlice = await this.signers.alice.signTypedData(
      eip712.domain,
      { Reencrypt: eip712.types.Reencrypt },
      eip712.message,
    );
    const counter = await this.fhevm.reencrypt(
      counterHandle,
      privateKeyAlice,
      publicKeyAlice,
      signatureAlice.replace("0x", ""),
      this.contractAddress,
      this.signers.alice.address,
    );
    expect(counter).to.equal(1);
  });
});
