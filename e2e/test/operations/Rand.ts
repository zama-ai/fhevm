import { expect } from "chai";
import { ethers, network } from "hardhat";

import { Decrypt, createDecrypt, createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";
import { deployRandFixture } from "./Rand.fixture";

describe("Rand", function () {
  let decrypt: Decrypt;
  before(async function () {
    await initSigners();
    this.signers = await getSigners();
    const contract = await deployRandFixture();
    this.contractAddress = await contract.getAddress();
    this.rand = contract;
    this.httpz = await createInstance();
    decrypt = createDecrypt(this.httpz, this.signers.alice, [this.contractAddress]);
  });

  it("ebool generate and decrypt", async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generateBool();
      await txn.wait();
      const valueHandle = await this.rand.valueb();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      values.push(value[0]);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it("4 bits generate and decrypt", async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate4();
      await txn.wait();
      const valueHandle = await this.rand.value4();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThanOrEqual(0xf);
      values.push(value[0]);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it("4 bits generate with upper bound and decrypt", async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 7; i++) {
      const txn = await this.rand.generate4UpperBound(8);
      await txn.wait();
      const valueHandle = await this.rand.value4();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThanOrEqual(7);
      values.push(value[0]);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it("8 bits generate and decrypt", async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate8();
      await txn.wait();
      const valueHandle = await this.rand.value8();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThanOrEqual(0xff);
      values.push(value[0]);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it("8 bits generate with upper bound and decrypt", async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate8UpperBound(128);
      await txn.wait();
      const valueHandle = await this.rand.value8();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThanOrEqual(127);
      values.push(value[0]);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it("16 bits generate and decrypt", async function () {
    const values: bigint[] = [];
    let has16bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate16();
      await txn.wait();
      const valueHandle = await this.rand.value16();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThanOrEqual(0xffff);
      if (value[0] > 0xff) {
        has16bit = true;
      }
      values.push(value[0]);
    }
    // Make sure we actually generate 16 bit integers.
    expect(has16bit).to.be.equal(true);
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it("16 bits generate with upper bound and decrypt", async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate16UpperBound(8192);
      await txn.wait();
      const valueHandle = await this.rand.value16();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThanOrEqual(8191);
      values.push(value[0]);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it("32 bits generate and decrypt", async function () {
    const values: bigint[] = [];
    let has32bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate32();
      await txn.wait();
      const valueHandle = await this.rand.value32();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThanOrEqual(0xffffffff);
      if (value[0] > 0xffff) {
        has32bit = true;
      }
      values.push(value[0]);
    }
    // Make sure we actually generate 32 bit integers.
    expect(has32bit).to.be.equal(true);
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it("32 bits generate with upper bound and decrypt", async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate32UpperBound(262144);
      await txn.wait();
      const valueHandle = await this.rand.value32();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThanOrEqual(262141);
      values.push(value[0]);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it("64 bits generate and decrypt", async function () {
    const values: bigint[] = [];
    let has64bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate64();
      await txn.wait();
      const valueHandle = await this.rand.value64();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThanOrEqual(BigInt("0xffffffffffffffff"));
      if (value[0] > BigInt("0xffffffff")) {
        has64bit = true;
      }
      // Make sure we actually generate 64 bit integers.
      expect(has64bit).to.be.equal(true);
      values.push(value[0]);
    }

    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it("64 bits generate with upper bound and decrypt", async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate64UpperBound(262144);
      await txn.wait();
      const valueHandle = await this.rand.value64();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThanOrEqual(262141);
      values.push(value[0]);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it("128 bits generate and decrypt", async function () {
    const values: bigint[] = [];
    let has128bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate128();
      await txn.wait();
      const valueHandle = await this.rand.value128();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThanOrEqual(BigInt("0xffffffffffffffffffffffffffffffff"));
      if (value[0] > BigInt("0xffffffffffffffff")) {
        has128bit = true;
      }
      values.push(value[0]);
      // Make sure we actually generate 128 bit integers.
      expect(has128bit).to.be.equal(true);
    }
    // Expect at least 4 different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(4);
  });

  it("128 bits generate with upper bound and decrypt", async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate128UpperBound(2n ** 100n);
      await txn.wait();
      const valueHandle = await this.rand.value128();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThanOrEqual(2n ** 100n);
      values.push(value[0]);
    }
    // Expect at least 4 different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(4);
  });

  it("256 bits generate and decrypt", async function () {
    const values: bigint[] = [];
    let has256bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate256();
      await txn.wait();
      const valueHandle = await this.rand.value256();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThanOrEqual(
        BigInt("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"),
      );
      if (value[0] > BigInt("0xffffffffffffffffffffffffffffffff")) {
        has256bit = true;
      }
      values.push(value[0]);
      // Make sure we actually generate 256 bit integers.
      expect(has256bit).to.be.equal(true);
    }
    // Expect at least 5 different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(5);
  });

  it("256 bits generate with upper bound and decrypt", async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate256UpperBound(2n ** 200n);
      await txn.wait();
      const valueHandle = await this.rand.value256();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThanOrEqual(2n ** 200n);
      values.push(value[0]);
    }
    // Expect at least 5 different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(5);
  });

  it("512 bits generate and decrypt", async function () {
    const values: bigint[] = [];
    let has512bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate512();
      await txn.wait();
      const valueHandle = await this.rand.value512();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThan(2n ** 512n);
      if (value[0] > 2n ** 256n) {
        has512bit = true;
      }
      values.push(value[0]);
      expect(has512bit).to.be.equal(true);
    }
    // Expect at least 5 different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(5);
  });

  it("1024 bits generate and decrypt", async function () {
    const values: bigint[] = [];
    let has1024bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate1024();
      await txn.wait();
      const valueHandle = await this.rand.value1024();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThan(2n ** 1024n);
      if (value[0] > 2n ** 512n) {
        has1024bit = true;
      }
      values.push(value[0]);
      expect(has1024bit).to.be.equal(true);
    }
    // Expect at least 5 different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(5);
  });

  it("2048 bits generate and decrypt", async function () {
    const values: bigint[] = [];
    let has2048bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate2048();
      await txn.wait();
      const valueHandle = await this.rand.value2048();
      const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
      expect(value[0]).to.be.lessThan(2n ** 2048n);
      if (value[0] > 2n ** 1024n) {
        has2048bit = true;
      }
      values.push(value[0]);
      expect(has2048bit).to.be.equal(true);
    }
    // Expect at least 5 different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(5);
  });

  it("8 and 16 bits generate and decrypt with hardhat snapshots [skip-on-coverage]", async function () {
    if (network.name === "hardhat") {
      // snapshots are only possible in hardhat node, i.e in mocked mode
      this.snapshotId = await ethers.provider.send("evm_snapshot");
      const values: bigint[] = [];
      for (let i = 0; i < 5; i++) {
        const txn = await this.rand.generate8();
        await txn.wait();
        const valueHandle = await this.rand.value8();
        const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
        expect(value[0]).to.be.lessThanOrEqual(0xff);
        values.push(value[0]);
      }
      // Expect at least two different generated values.
      const unique = new Set(values);
      expect(unique.size).to.be.greaterThanOrEqual(2);

      await ethers.provider.send("evm_revert", [this.snapshotId]);
      this.snapshotId = await ethers.provider.send("evm_snapshot");

      const values2: bigint[] = [];
      for (let i = 0; i < 5; i++) {
        const txn = await this.rand.generate8();
        await txn.wait();
        const valueHandle = await this.rand.value8();
        const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
        expect(value[0]).to.be.lessThanOrEqual(0xff);
        values2.push(value[0]);
      }
      // Expect at least two different generated values.
      const unique2 = new Set(values2);
      expect(unique2.size).to.be.greaterThanOrEqual(2);

      await ethers.provider.send("evm_revert", [this.snapshotId]);
      const values3: bigint[] = [];
      let has16bit: boolean = false;
      for (let i = 0; i < 5; i++) {
        const txn = await this.rand.generate16();
        await txn.wait();
        const valueHandle = await this.rand.value16();
        const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
        expect(value[0]).to.be.lessThanOrEqual(0xffff);
        if (value[0] > 0xff) {
          has16bit = true;
        }
        values3.push(value[0]);
      }
      // Make sure we actually generate 16 bit integers.
      expect(has16bit).to.be.equal(true);
      // Expect at least two different generated values.
      const unique3 = new Set(values3);
      expect(unique3.size).to.be.greaterThanOrEqual(2);
    }
  });

  it("generating rand in reverting sub-call", async function () {
    const txn = await this.rand.generate64Reverting();
    await txn.wait();
    const valueHandle = await this.rand.value64Bounded();
    const value = await decrypt([{ ctHandle: valueHandle, contractAddress: this.contractAddress }]);
    expect(value[0]).to.be.lessThan(1024);
  });
});
