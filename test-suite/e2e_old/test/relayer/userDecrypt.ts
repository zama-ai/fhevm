import { expect } from "chai";
import { Context } from "mocha";

import { Reencrypt } from "../../types";
import { Decrypt, createDecrypt, createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";
import { deployUserDecryptFixture } from "./UserDecrypt.fixture";

interface ReencryptContext extends Context {
  contract: Reencrypt;
}

describe("Test reencrypt", function () {
  let decrypt: Decrypt;
  before(async function (this: ReencryptContext) {
    await initSigners();
    this.signers = await getSigners();
    this.httpz = await createInstance();
    const contract = await deployUserDecryptFixture();
    this.contractAddress = await contract.getAddress();
    this.contract = contract;
    decrypt = createDecrypt(this.httpz, this.signers.alice, [this.contractAddress]);
  });

  it("should reencrypt a bool value", async function (this: ReencryptContext) {
    const handle = await this.contract.resultBool();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1);
  });

  it("should reencrypt a 4bits value", async function (this: ReencryptContext) {
    const handle = await this.contract.result4();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(2);
  });

  it("should reencrypt a 8bits value", async function (this: ReencryptContext) {
    const handle = await this.contract.result8();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(4);
  });

  it("should reencrypt a 16bits value", async function (this: ReencryptContext) {
    const handle = await this.contract.result16();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(8);
  });

  it("should reencrypt a 32bits value", async function (this: ReencryptContext) {
    const handle = await this.contract.result32();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(16);
  });

  it("should reencrypt a 64bits value", async function (this: ReencryptContext) {
    const handle = await this.contract.result64();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(32);
  });

  it("should reencrypt a 128bits value", async function (this: ReencryptContext) {
    const handle = await this.contract.result128();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(64);
  });

  it("should reencrypt a 256bits value", async function (this: ReencryptContext) {
    const handle = await this.contract.result256();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(128);
  });

  it("should reencrypt a bytes64 value", async function (this: ReencryptContext) {
    const handle = await this.contract.resultEbytes64();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(256);
  });

  it("should reencrypt a bytes128 value", async function (this: ReencryptContext) {
    const handle = await this.contract.resultEbytes128();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(512);
  });

  it("should reencrypt a bytes256 value", async function (this: ReencryptContext) {
    const handle = await this.contract.resultEbytes256();

    const result = await decrypt([{ ctHandle: handle, contractAddress: this.contractAddress }]);
    expect(result).to.equal(1024);
  });
});
