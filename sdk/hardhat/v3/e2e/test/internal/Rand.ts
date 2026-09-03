import { expect } from 'chai';
import { network } from 'hardhat';

import type { Rand } from '../../types/ethers-contracts/index.ts';
import { deployRandFixture } from './Rand.fixture.ts';

// The v2 file minus its `it.skip` snapshot test (skipped upstream too, and bound to mocha's `this`).
const { fhevm } = await network.getOrCreate();

describe('Rand', function () {
  let randContract: Rand;

  beforeEach(async function () {
    const contract: Rand = await deployRandFixture();
    randContract = contract;
  });

  it('ebool generate and decrypt', async function () {
    const values: boolean[] = [];
    for (let i = 0; i < 15; i++) {
      const txn = await randContract.generateBool();
      await txn.wait();
      const valueHandle = (await randContract.valueb()) as `0x${string}`;
      const res = await fhevm.publicDecrypt([valueHandle]);
      const value = res.clearValues[valueHandle];
      expect(typeof value).to.eq('boolean');
      values.push(value as boolean);
    }
    //Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('8 bits generate and decrypt', async function () {
    const values: number[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await randContract.generate8();
      await txn.wait();
      const valueHandle = (await randContract.value8()) as `0x${string}`;
      const res = await fhevm.publicDecrypt([valueHandle]);
      const value = res.clearValues[valueHandle];
      expect(typeof value).to.eq('bigint');
      expect(value).to.be.lessThanOrEqual(0xff);
      values.push(Number(value));
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('8 bits generate with upper bound and decrypt', async function () {
    const values: number[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await randContract.generate8UpperBound(128);
      await txn.wait();
      const valueHandle = (await randContract.value8()) as `0x${string}`;
      const res = await fhevm.publicDecrypt([valueHandle]);
      const value = res.clearValues[valueHandle];
      expect(typeof value).to.eq('bigint');
      expect(value).to.be.lessThanOrEqual(127);
      values.push(Number(value));
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('16 bits generate and decrypt', async function () {
    const values: number[] = [];
    let has16bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await randContract.generate16();
      await txn.wait();
      const valueHandle = (await randContract.value16()) as `0x${string}`;
      const res = await fhevm.publicDecrypt([valueHandle]);
      const value = res.clearValues[valueHandle];
      expect(typeof value).to.eq('bigint');
      const valueNum = Number(value);
      expect(valueNum).to.be.lessThanOrEqual(0xffff);
      if (valueNum > 0xff) {
        has16bit = true;
      }
      values.push(valueNum);
    }
    // Make sure we actually generate 16 bit integers.
    expect(has16bit).to.eq(true);
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('16 bits generate with upper bound and decrypt', async function () {
    const values: number[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await randContract.generate16UpperBound(8192);
      await txn.wait();
      const valueHandle = (await randContract.value16()) as `0x${string}`;
      const res = await fhevm.publicDecrypt([valueHandle]);
      const value = res.clearValues[valueHandle];
      expect(typeof value).to.eq('bigint');
      const valueNum = Number(value);
      expect(valueNum).to.be.lessThanOrEqual(8191);
      values.push(valueNum);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('32 bits generate and decrypt', async function () {
    const values: number[] = [];
    let has32bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await randContract.generate32();
      await txn.wait();
      const valueHandle = (await randContract.value32()) as `0x${string}`;
      const res = await fhevm.publicDecrypt([valueHandle]);
      const value = res.clearValues[valueHandle];
      expect(typeof value).to.eq('bigint');
      const valueNum = Number(value);
      expect(valueNum).to.be.lessThanOrEqual(0xffffffff);
      if (valueNum > 0xffff) {
        has32bit = true;
      }
      values.push(valueNum);
    }
    // Make sure we actually generate 32 bit integers.
    expect(has32bit).to.eq(true);
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('32 bits generate with upper bound and decrypt', async function () {
    const values: number[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await randContract.generate32UpperBound(262144);
      await txn.wait();
      const valueHandle = (await randContract.value32()) as `0x${string}`;
      const res = await fhevm.publicDecrypt([valueHandle]);
      const value = res.clearValues[valueHandle];
      expect(typeof value).to.eq('bigint');
      const valueNum = Number(value);
      expect(valueNum).to.be.lessThanOrEqual(262141);
      values.push(valueNum);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('64 bits generate and decrypt', async function () {
    const values: bigint[] = [];
    let has64bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await randContract.generate64();
      await txn.wait();
      const valueHandle = (await randContract.value64()) as `0x${string}`;
      const res = await fhevm.publicDecrypt([valueHandle]);
      const value = res.clearValues[valueHandle] as bigint;
      expect(value).to.be.lessThanOrEqual(BigInt('0xffffffffffffffff'));
      if (value > BigInt('0xffffffff')) {
        has64bit = true;
      }
      // Make sure we actually generate 64 bit integers.
      expect(has64bit).to.eq(true);
      values.push(value);
    }

    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('64 bits generate with upper bound and decrypt', async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await randContract.generate64UpperBound(262144);
      await txn.wait();
      const valueHandle = (await randContract.value64()) as `0x${string}`;
      const res = await fhevm.publicDecrypt([valueHandle]);
      const value = res.clearValues[valueHandle] as bigint;
      expect(value).to.be.lessThanOrEqual(262141);
      values.push(value);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('128 bits generate and decrypt', async function () {
    const values: bigint[] = [];
    let has128bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await randContract.generate128();
      await txn.wait();
      const valueHandle = (await randContract.value128()) as `0x${string}`;
      const res = await fhevm.publicDecrypt([valueHandle]);
      const value = res.clearValues[valueHandle] as bigint;
      expect(value).to.be.lessThanOrEqual(BigInt('0xffffffffffffffffffffffffffffffff'));
      if (value > BigInt('0xffffffffffffffff')) {
        has128bit = true;
      }
      values.push(value);
      // Make sure we actually generate 128 bit integers.
      expect(has128bit).to.eq(true);
    }
    // Expect at least 4 different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(4);
  });

  it('128 bits generate with upper bound and decrypt', async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await randContract.generate128UpperBound(2n ** 100n);
      await txn.wait();
      const valueHandle = (await randContract.value128()) as `0x${string}`;
      const res = await fhevm.publicDecrypt([valueHandle]);
      const value = res.clearValues[valueHandle] as bigint;
      expect(value).to.be.lessThanOrEqual(2n ** 100n);
      values.push(value);
    }
    // Expect at least 4 different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(4);
  });

  it('256 bits generate and decrypt', async function () {
    const values: bigint[] = [];
    let has256bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await randContract.generate256();
      await txn.wait();
      const valueHandle = (await randContract.value256()) as `0x${string}`;
      const res = await fhevm.publicDecrypt([valueHandle]);
      const value = res.clearValues[valueHandle] as bigint;
      expect(value).to.be.lessThanOrEqual(BigInt('0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff'));
      if (value > BigInt('0xffffffffffffffffffffffffffffffff')) {
        has256bit = true;
      }
      values.push(value);
      // Make sure we actually generate 256 bit integers.
      expect(has256bit).to.eq(true);
    }
    // Expect at least 5 different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(5);
  });

  it('256 bits generate with upper bound and decrypt', async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await randContract.generate256UpperBound(2n ** 200n);
      await txn.wait();
      const valueHandle = (await randContract.value256()) as `0x${string}`;
      const res = await fhevm.publicDecrypt([valueHandle]);
      const value = res.clearValues[valueHandle] as bigint;
      expect(value).to.be.lessThanOrEqual(2n ** 200n);
      values.push(value);
    }
    // Expect at least 5 different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(5);
  });

  it('generating rand in reverting sub-call', async function () {
    const txn = await randContract.generate64Reverting();
    await txn.wait();
    const valueHandle = (await randContract.value64Bounded()) as `0x${string}`;
    const res = await fhevm.publicDecrypt([valueHandle]);
    const value = res.clearValues[valueHandle] as bigint;
    expect(value).to.be.lessThan(1024);
  });
});
