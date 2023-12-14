import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { deployRandFixture } from './Rand.fixture';

describe('Rand', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployRandFixture();
    this.contractAddress = await contract.getAddress();
    this.rand = contract;
    this.instances = await createInstances(this.contractAddress, ethers, this.signers);
  });

  it('8 bits generate and decrypt', async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate8();
      await txn.wait();
      const value = await this.rand.decrypt8();
      expect(value).to.be.lessThanOrEqual(0xff);
      values.push(value);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('8 bits generate with upper bound and decrypt', async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate8UpperBound(128);
      await txn.wait();
      const value = await this.rand.decrypt8();
      expect(value).to.be.lessThanOrEqual(127);
      values.push(value);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('16 bits generate and decrypt', async function () {
    const values: bigint[] = [];
    let has16bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate16();
      await txn.wait();
      const value = await this.rand.decrypt16();
      expect(value).to.be.lessThanOrEqual(0xffff);
      if (value > 0xff) {
        has16bit = true;
      }
      values.push(value);
    }
    // Make sure we actually generate 16 bit integers.
    expect(has16bit).to.be.true;
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('16 bits generate with upper bound and decrypt', async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate16UpperBound(8192);
      await txn.wait();
      const value = await this.rand.decrypt16();
      expect(value).to.be.lessThanOrEqual(8191);
      values.push(value);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('32 bits generate and decrypt', async function () {
    const values: bigint[] = [];
    let has32bit: boolean = false;
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate32();
      await txn.wait();
      const value = await this.rand.decrypt32();
      expect(value).to.be.lessThanOrEqual(0xffffffff);
      if (value > 0xffff) {
        has32bit = true;
      }
      values.push(value);
    }
    // Make sure we actually generate 32 bit integers.
    expect(has32bit).to.be.true;
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('32 bits generate with upper bound and decrypt', async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate32UpperBound(262144);
      await txn.wait();
      const value = await this.rand.decrypt32();
      expect(value).to.be.lessThanOrEqual(262141);
      values.push(value);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('8 bits generate, decrypt and store', async function () {
    const txnGen = await this.rand.generate8();
    await txnGen.wait();
    const txnDecAndStore = await this.rand.decryptAndStore8();
    await expect(txnDecAndStore.wait()).to.not.be.rejected;
  });

  it('8 bits generate with upper bound, decrypt and store', async function () {
    const txnGen = await this.rand.generate8UpperBound(64);
    await txnGen.wait();
    const txnDecAndStore = await this.rand.decryptAndStore8();
    await expect(txnDecAndStore.wait()).to.not.be.rejected;
  });

  it('16 bits generate, decrypt and store', async function () {
    const txnGen = await this.rand.generate16();
    await txnGen.wait();
    const txnDecAndStore = await this.rand.decryptAndStore16();
    await expect(txnDecAndStore.wait()).to.not.be.rejected;
  });

  it('16 bits generate with upper bound, decrypt and store', async function () {
    const txnGen = await this.rand.generate16UpperBound(4096);
    await txnGen.wait();
    const txnDecAndStore = await this.rand.decryptAndStore16();
    await expect(txnDecAndStore.wait()).to.not.be.rejected;
  });

  it('32 bits generate, decrypt and store', async function () {
    const txnGen = await this.rand.generate32();
    await txnGen.wait();
    const txnDecAndStore = await this.rand.decryptAndStore32();
    await expect(txnDecAndStore.wait()).to.not.be.rejected;
  });

  it('32 bits generate with upper bound, decrypt and store', async function () {
    const txnGen = await this.rand.generate32UpperBound(32768);
    await txnGen.wait();
    const txnDecAndStore = await this.rand.decryptAndStore32();
    await expect(txnDecAndStore.wait()).to.not.be.rejected;
  });

  it('8 bits in view', async function () {
    if (process.env.HARDHAT_NETWORK !== 'hardhat') {
      await expect(this.rand.generate8InView()).to.be.rejected;
    }
  });

  it('8 bits with upper bound in view', async function () {
    if (process.env.HARDHAT_NETWORK !== 'hardhat') {
      await expect(this.rand.generate8UpperBoundInView(32)).to.be.rejected;
    }
  });

  it('16 bits in view', async function () {
    if (process.env.HARDHAT_NETWORK !== 'hardhat') {
      await expect(this.rand.generate16InView()).to.be.rejected;
    }
  });

  it('16 bits with upper bound in view', async function () {
    if (process.env.HARDHAT_NETWORK !== 'hardhat') {
      await expect(this.rand.generate16UpperBoundInView(128)).to.be.rejected;
    }
  });

  it('32 bits in view', async function () {
    if (process.env.HARDHAT_NETWORK !== 'hardhat') {
      await expect(this.rand.generate32InView()).to.be.rejected;
    }
  });

  it('32 bits with upper bound in view', async function () {
    if (process.env.HARDHAT_NETWORK !== 'hardhat') {
      await expect(this.rand.generate32UpperBoundInView(512)).to.be.rejected;
    }
  });
});
