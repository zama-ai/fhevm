import { expect } from "chai";
import { Context } from "mocha";

import { Sub } from "../../types";
import { Decrypt, createDecrypt, createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";
import { deploySubFixture } from "./Sub.fixture";

interface SubContext extends Context {
  contract: Sub;
}

describe("Test sub", function () {
  let decrypt: Decrypt;
  before(async function (this: SubContext) {
    await initSigners();
    this.signers = await getSigners();
    this.httpz = await createInstance();
    const contract = await deploySubFixture();
    this.contractAddress = await contract.getAddress();
    this.contract = contract;
    decrypt = createDecrypt(this.httpz, this.signers.alice, [this.contractAddress]);
  });

  it("should sub 4 bits", async function (this: SubContext) {
    const transaction = await this.contract.sub4();
    await transaction.wait();

    const handle = await this.contract.result4();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });

  it("should sub 4 bits (scalar)", async function (this: SubContext) {
    const transaction = await this.contract.sub4Scalar();
    await transaction.wait();

    const handle = await this.contract.result4();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });

  it("should sub 8 bits", async function (this: SubContext) {
    const transaction = await this.contract.sub8();
    await transaction.wait();

    const handle = await this.contract.result8();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });

  it("should sub 8 bits (scalar)", async function (this: SubContext) {
    const transaction = await this.contract.sub8Scalar();
    await transaction.wait();

    const handle = await this.contract.result8();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });

  it("should sub 16 bits", async function (this: SubContext) {
    const transaction = await this.contract.sub16();
    await transaction.wait();

    const handle = await this.contract.result16();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });

  it("should sub 16 bits (scalar)", async function (this: SubContext) {
    const transaction = await this.contract.sub16Scalar();
    await transaction.wait();

    const handle = await this.contract.result16();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });

  it("should sub 32 bits", async function (this: SubContext) {
    const transaction = await this.contract.sub32();
    await transaction.wait();

    const handle = await this.contract.result32();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });

  it("should sub 32 bits (scalar)", async function (this: SubContext) {
    const transaction = await this.contract.sub32Scalar();
    await transaction.wait();

    const handle = await this.contract.result32();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });

  it("should sub 64 bits", async function (this: SubContext) {
    const transaction = await this.contract.sub64();
    await transaction.wait();

    const handle = await this.contract.result64();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });

  it("should sub 64 bits (scalar)", async function (this: SubContext) {
    const transaction = await this.contract.sub64Scalar();
    await transaction.wait();

    const handle = await this.contract.result64();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });

  it("should sub 128 bits", async function (this: SubContext) {
    const transaction = await this.contract.sub128();
    await transaction.wait();

    const handle = await this.contract.result128();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });

  it("should sub 128 bits (scalar)", async function (this: SubContext) {
    const transaction = await this.contract.sub128Scalar();
    await transaction.wait();

    const handle = await this.contract.result128();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });

  it("should sub 256 bits", async function (this: SubContext) {
    const transaction = await this.contract.sub256();
    await transaction.wait();

    const handle = await this.contract.result256();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });

  it("should sub 256 bits (scalar)", async function (this: SubContext) {
    const transaction = await this.contract.sub256Scalar();
    await transaction.wait();

    const handle = await this.contract.result256();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });
});
