import { expect } from "chai";
import { Context } from "mocha";

import { Add } from "../../types";
import { Decrypt, createDecrypt, createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";
import { deployAddFixture } from "./Add.fixture";

interface AddContext extends Context {
  contract: Add;
}

describe("Test add", function () {
  let decrypt: Decrypt;
  before(async function (this: AddContext) {
    await initSigners();
    this.signers = await getSigners();
    this.httpz = await createInstance();
    const contract = await deployAddFixture();
    this.contractAddress = await contract.getAddress();
    this.contract = contract;
    decrypt = createDecrypt(this.httpz, this.signers.alice, [this.contractAddress]);
  });

  it("should add 4 bits", async function (this: AddContext) {
    const transaction = await this.contract.add4();
    await transaction.wait();

    const handle = await this.contract.result4();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(3);
  });

  it("should add 4 bits (scalar)", async function (this: AddContext) {
    const transaction = await this.contract.add4Scalar();
    await transaction.wait();

    const handle = await this.contract.result4();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(3);
  });

  it("should add 8 bits", async function (this: AddContext) {
    const transaction = await this.contract.add8();
    await transaction.wait();

    const handle = await this.contract.result8();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(3);
  });

  it("should add 8 bits (scalar)", async function (this: AddContext) {
    const transaction = await this.contract.add8Scalar();
    await transaction.wait();

    const handle = await this.contract.result8();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(3);
  });

  it("should add 16 bits", async function (this: AddContext) {
    const transaction = await this.contract.add16();
    await transaction.wait();

    const handle = await this.contract.result16();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(3);
  });

  it("should add 16 bits (scalar)", async function (this: AddContext) {
    const transaction = await this.contract.add16Scalar();
    await transaction.wait();

    const handle = await this.contract.result16();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(3);
  });

  it("should add 32 bits", async function (this: AddContext) {
    const transaction = await this.contract.add32();
    await transaction.wait();

    const handle = await this.contract.result32();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(3);
  });

  it("should add 32 bits (scalar)", async function (this: AddContext) {
    const transaction = await this.contract.add32Scalar();
    await transaction.wait();

    const handle = await this.contract.result32();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(3);
  });

  it("should add 64 bits", async function (this: AddContext) {
    const transaction = await this.contract.add64();
    await transaction.wait();

    const handle = await this.contract.result64();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(3);
  });

  it("should add 64 bits (scalar)", async function (this: AddContext) {
    const transaction = await this.contract.add64Scalar();
    await transaction.wait();

    const handle = await this.contract.result64();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(3);
  });

  it("should add 128 bits", async function (this: AddContext) {
    const transaction = await this.contract.add128();
    await transaction.wait();

    const handle = await this.contract.result128();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(3);
  });

  it("should add 128 bits (scalar)", async function (this: AddContext) {
    const transaction = await this.contract.add128Scalar();
    await transaction.wait();

    const handle = await this.contract.result128();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(3);
  });

  it("should add 256 bits", async function (this: AddContext) {
    const transaction = await this.contract.add256();
    await transaction.wait();

    const handle = await this.contract.result256();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(3);
  });

  it("should add 256 bits (scalar)", async function (this: AddContext) {
    const transaction = await this.contract.add256Scalar();
    await transaction.wait();

    const handle = await this.contract.result256();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(3);
  });
});
