import { expect } from 'chai';
import { ethers, fhevm, network } from 'hardhat';

import type { Rand } from '../../../typechain-types';
import { initSigners } from '../signers';
import { deployRandFixture } from './Rand.fixture';

describe('Rand', function () {
  let randContract: Rand;

  before(async function () {
    await initSigners();
  });

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

  /*
   TODO: try to fix the following test:
   ====================================
   Error: Invalid block filter fromBlock=97 toBlock=92
      at getCoprocessorEvents (/Users/alex/src/me/zama-ai/monorepo/fhevm-mocks/packages/mock-utils/src/fhevm/coprocessor/utils.ts:36:11)
  */
  it.skip('8 and 16 bits generate and decrypt with hardhat snapshots [skip-on-coverage]', async function () {
    if (network.name === 'hardhat') {
      // snapshots are only possible in hardhat node, i.e in mocked mode
      // `send` is typed `Promise<any>`; evm_snapshot answers with the snapshot id as a hex string.
      this.snapshotId = (await ethers.provider.send('evm_snapshot')) as string;
      const values: number[] = [];
      for (let i = 0; i < 5; i++) {
        const txn = await randContract.generate8();
        await txn.wait();
        const valueHandle = (await randContract.value8()) as `0x${string}`;
        const res = await fhevm.publicDecrypt([valueHandle]);
        const value = res.clearValues[valueHandle] as bigint;
        const valueNum = Number(value);
        expect(valueNum).to.be.lessThanOrEqual(0xff);
        values.push(valueNum);
      }
      // Expect at least two different generated values.
      const unique = new Set(values);
      expect(unique.size).to.be.greaterThanOrEqual(2);

      await ethers.provider.send('evm_revert', [this.snapshotId]);
      this.snapshotId = (await ethers.provider.send('evm_snapshot')) as string;

      const values2: number[] = [];
      for (let i = 0; i < 5; i++) {
        const txn = await randContract.generate8();
        await txn.wait();
        const valueHandle = (await randContract.value8()) as `0x${string}`;
        const res = await fhevm.publicDecrypt([valueHandle]);
        const value = res.clearValues[valueHandle] as bigint;
        const valueNum = Number(value);
        expect(valueNum).to.be.lessThanOrEqual(0xff);
        values2.push(valueNum);
      }
      // Expect at least two different generated values.
      const unique2 = new Set(values2);
      expect(unique2.size).to.be.greaterThanOrEqual(2);

      await ethers.provider.send('evm_revert', [this.snapshotId]);
      const values3: number[] = [];
      let has16bit: boolean = false;
      for (let i = 0; i < 5; i++) {
        const txn = await randContract.generate16();
        await txn.wait();
        const valueHandle = (await randContract.value16()) as `0x${string}`;
        const res = await fhevm.publicDecrypt([valueHandle]);
        const value = res.clearValues[valueHandle] as bigint;
        const valueNum = Number(value);
        expect(valueNum).to.be.lessThanOrEqual(0xffff);
        if (valueNum > 0xff) {
          has16bit = true;
        }
        values3.push(valueNum);
      }
      // Make sure we actually generate 16 bit integers.
      expect(has16bit).to.eq(true);
      // Expect at least two different generated values.
      const unique3 = new Set(values3);
      expect(unique3.size).to.be.greaterThanOrEqual(2);
    }
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
