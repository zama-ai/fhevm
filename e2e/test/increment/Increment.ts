import { expect } from "chai";

import { createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";
import { deployIncrementFixture } from "./Increment.fixture";

describe("Increment", function () {
  before(async function () {
    await initSigners();
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployIncrementFixture();
    this.contractAddress = await contract.getAddress();
    this.increment = contract;
    this.fhevm = await createInstance();
  });

  it("should increment", async function () {
    const transaction = await this.increment.increment();
    await transaction.wait();
    const transaction2 = await this.increment.increment();
    await transaction2.wait();

    const counterHandle = await this.increment.counter();
    console.log(counterHandle);
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
    expect(counter).to.equal(2);
  });

  // it("should increment", async function () {
  //   const transaction = await this.increment.increment();
  //   await transaction.wait();
  //   const transaction2 = await this.increment.increment();
  //   await transaction2.wait();

  //   // Reencrypt counter
  //   const counterHandle = await this.increment.counter();
  //   const { publicKey: publicKeyAlice, privateKey: privateKeyAlice } = this.instances.alice.generateKeypair();
  //   const eip712 = this.instances.alice.createEIP712(publicKeyAlice, this.contractAddress);
  //   const signatureAlice = await this.signers.alice.signTypedData(
  //     eip712.domain,
  //     { Reencrypt: eip712.types.Reencrypt },
  //     eip712.message,
  //   );
  //   const counter = await this.instances.alice.reencrypt(
  //     counterHandle,
  //     privateKeyAlice,
  //     publicKeyAlice,
  //     signatureAlice.replace("0x", ""),
  //     this.contractAddress,
  //     this.signers.alice.address,
  //   );
  //   expect(counter).to.equal(2);
  // });
});
