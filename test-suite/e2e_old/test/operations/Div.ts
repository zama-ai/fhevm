import { expect } from "chai";
import { Context } from "mocha";

import { Div } from "../../types";
import { Decrypt, createDecrypt, createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";
import { deployDivFixture } from "./Div.fixture";

interface DivContext extends Context {
  contract: Div;
}

// TODO: add correctness checks
// TODO: remove `.only`
describe("Test div", function () {
  let decrypt: Decrypt;
  before(async function (this: DivContext) {
    await initSigners();
    this.signers = await getSigners();
    this.httpz = await createInstance();
    const contract = await deployDivFixture();
    this.contractAddress = await contract.getAddress();
    this.contract = contract;
    decrypt = createDecrypt(this.httpz, this.signers.alice, [this.contractAddress]);
  });

  it("should div 4 bits (scalar)", async function (this: DivContext) {
    const transaction = await this.contract.div4Scalar();
    await transaction.wait();

    const handle = await this.contract.result4();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(2);
  });

  it("should div 8 bits (scalar)", async function (this: DivContext) {
    const transaction = await this.contract.div8Scalar();
    await transaction.wait();

    const handle = await this.contract.result8();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(12);
  });

  it("should div 16 bits (scalar)", async function (this: DivContext) {
    const transaction = await this.contract.div16Scalar();
    await transaction.wait();

    const handle = await this.contract.result16();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(161);
  });

  it("should div 32 bits (scalar)", async function (this: DivContext) {
    const transaction = await this.contract.div32Scalar();
    await transaction.wait();

    const handle = await this.contract.result32();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(9634407);
  });

  it("should div 64 bits (scalar)", async function (this: DivContext) {
    const transaction = await this.contract.div64Scalar();
    await transaction.wait();

    const handle = await this.contract.result64();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(18991132);
  });

  it("should div 128 bits (scalar)", async function (this: DivContext) {
    const transaction = await this.contract.div128Scalar();
    await transaction.wait();

    const handle = await this.contract.result128();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(18991132);
  });

  it("should div 256 bits (scalar)", async function (this: DivContext) {
    const transaction = await this.contract.div256Scalar();
    await transaction.wait();

    const handle = await this.contract.result256();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(18991132);
  });
});
