import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHETestSuite1 } from '../../types/contracts/tests/TFHETestSuite1';
import type { TFHETestSuite2 } from '../../types/contracts/tests/TFHETestSuite2';
import { createInstances } from '../instance';
import type { Signers } from '../types';

async function deployTfheTestFixture1(): Promise<TFHETestSuite1> {
  const signers = await ethers.getSigners();
  const admin = signers[0];

  const contractFactory = await ethers.getContractFactory('TFHETestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture2(): Promise<TFHETestSuite2> {
  const signers = await ethers.getSigners();
  const admin = signers[0];

  const contractFactory = await ethers.getContractFactory('TFHETestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('TFHE operations', function () {
  before(async function () {
    this.signers = {} as Signers;
    const signers = await ethers.getSigners();
    this.signers.alice = signers[0];
    this.signers.bob = signers[1];
    this.signers.carol = signers[2];
    this.signers.dave = signers[3];

    const contract1 = await deployTfheTestFixture1();
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;
    const instances1 = await createInstances(this.contract1Address, ethers, this.signers);
    this.instances1 = instances1;

    const contract2 = await deployTfheTestFixture2();
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;
    const instances2 = await createInstances(this.contract2Address, ethers, this.signers);
    this.instances2 = instances2;
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 1 (3, 4)', async function () {
    const res = await this.contract1.add_euint8_euint8(3, 4);
    expect(res).to.equal(7);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract1.sub_euint8_euint8(4, 3);
    expect(res).to.equal(1);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 1 (3, 4)', async function () {
    const res = await this.contract1.mul_euint8_euint8(3, 4);
    expect(res).to.equal(12);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 1 (255, 15)', async function () {
    const res = await this.contract1.and_euint8_euint8(255, 15);
    expect(res).to.equal(15);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 1 (112, 15)', async function () {
    const res = await this.contract1.or_euint8_euint8(112, 15);
    expect(res).to.equal(127);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 1 (119, 119)', async function () {
    const res = await this.contract1.xor_euint8_euint8(119, 119);
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 2 (12, 34)', async function () {
    const res = await this.contract1.xor_euint8_euint8(12, 34);
    expect(res).to.equal(46);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (2, 1)', async function () {
    const res = await this.contract1.shl_euint8_euint8(2, 1);
    expect(res).to.equal(4);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 2 (2, 4)', async function () {
    const res = await this.contract1.shl_euint8_euint8(2, 4);
    expect(res).to.equal(32);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (2, 1)', async function () {
    const res = await this.contract1.shr_euint8_euint8(2, 1);
    expect(res).to.equal(1);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 2 (32, 4)', async function () {
    const res = await this.contract1.shr_euint8_euint8(32, 4);
    expect(res).to.equal(2);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 1 (12, 49)', async function () {
    const res = await this.contract1.eq_euint8_euint8(12, 49);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 2 (7, 7)', async function () {
    const res = await this.contract1.eq_euint8_euint8(7, 7);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 1 (1, 2)', async function () {
    const res = await this.contract1.ne_euint8_euint8(1, 2);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 2 (2, 2)', async function () {
    const res = await this.contract1.ne_euint8_euint8(2, 2);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 1 (10, 10)', async function () {
    const res = await this.contract1.ge_euint8_euint8(10, 10);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 2 (10, 9)', async function () {
    const res = await this.contract1.ge_euint8_euint8(10, 9);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 3 (10, 11)', async function () {
    const res = await this.contract1.ge_euint8_euint8(10, 11);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 1 (10, 10)', async function () {
    const res = await this.contract1.gt_euint8_euint8(10, 10);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 2 (10, 9)', async function () {
    const res = await this.contract1.gt_euint8_euint8(10, 9);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 3 (10, 11)', async function () {
    const res = await this.contract1.gt_euint8_euint8(10, 11);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 1 (10, 10)', async function () {
    const res = await this.contract1.le_euint8_euint8(10, 10);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 2 (10, 9)', async function () {
    const res = await this.contract1.le_euint8_euint8(10, 9);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 3 (10, 11)', async function () {
    const res = await this.contract1.le_euint8_euint8(10, 11);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 1 (10, 10)', async function () {
    const res = await this.contract1.lt_euint8_euint8(10, 10);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 2 (10, 9)', async function () {
    const res = await this.contract1.lt_euint8_euint8(10, 9);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 3 (10, 11)', async function () {
    const res = await this.contract1.lt_euint8_euint8(10, 11);
    expect(res).to.equal(true);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 1 (10, 10)', async function () {
    const res = await this.contract1.min_euint8_euint8(10, 10);
    expect(res).to.equal(10);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 2 (12, 10)', async function () {
    const res = await this.contract1.min_euint8_euint8(12, 10);
    expect(res).to.equal(10);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 3 (9, 12)', async function () {
    const res = await this.contract1.min_euint8_euint8(9, 12);
    expect(res).to.equal(9);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 1 (10, 10)', async function () {
    const res = await this.contract1.max_euint8_euint8(10, 10);
    expect(res).to.equal(10);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 2 (12, 10)', async function () {
    const res = await this.contract1.max_euint8_euint8(12, 10);
    expect(res).to.equal(12);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 3 (9, 12)', async function () {
    const res = await this.contract1.max_euint8_euint8(9, 12);
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 1 (3, 65280)', async function () {
    const res = await this.contract1.add_euint8_euint16(3, 65280);
    expect(res).to.equal(65283);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 1 (3, 4096)', async function () {
    const res = await this.contract1.sub_euint8_euint16(3, 4096);
    expect(res).to.equal(61443);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 1 (3, 4096)', async function () {
    const res = await this.contract1.mul_euint8_euint16(3, 4096);
    expect(res).to.equal(12288);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 1 (3, 4096)', async function () {
    const res = await this.contract1.and_euint8_euint16(3, 4096);
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 2 (3, 4097)', async function () {
    const res = await this.contract1.and_euint8_euint16(3, 4097);
    expect(res).to.equal(1);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 1 (3, 4096)', async function () {
    const res = await this.contract1.or_euint8_euint16(3, 4096);
    expect(res).to.equal(4099);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 2 (3, 4097)', async function () {
    const res = await this.contract1.or_euint8_euint16(3, 4097);
    expect(res).to.equal(4099);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 1 (255, 65535)', async function () {
    const res = await this.contract1.xor_euint8_euint16(255, 65535);
    expect(res).to.equal(65280);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 2 (255, 65280)', async function () {
    const res = await this.contract1.xor_euint8_euint16(255, 65280);
    expect(res).to.equal(65535);
  });

  it('test operator "shl" overload (euint8, euint16) => euint16 test 1 (255, 256)', async function () {
    const res = await this.contract1.shl_euint8_euint16(255, 256);
    expect(res).to.equal(255);
  });

  it('test operator "shl" overload (euint8, euint16) => euint16 test 2 (2, 1)', async function () {
    const res = await this.contract1.shl_euint8_euint16(2, 1);
    expect(res).to.equal(4);
  });

  it('test operator "shr" overload (euint8, euint16) => euint16 test 1 (255, 256)', async function () {
    const res = await this.contract1.shr_euint8_euint16(255, 256);
    expect(res).to.equal(255);
  });

  it('test operator "shr" overload (euint8, euint16) => euint16 test 2 (255, 1)', async function () {
    const res = await this.contract1.shr_euint8_euint16(255, 1);
    expect(res).to.equal(127);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 1 (255, 255)', async function () {
    const res = await this.contract1.eq_euint8_euint16(255, 255);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 2 (255, 511)', async function () {
    const res = await this.contract1.eq_euint8_euint16(255, 511);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 1 (255, 255)', async function () {
    const res = await this.contract1.ne_euint8_euint16(255, 255);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 2 (255, 511)', async function () {
    const res = await this.contract1.ne_euint8_euint16(255, 511);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 1 (255, 255)', async function () {
    const res = await this.contract1.ge_euint8_euint16(255, 255);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 2 (255, 511)', async function () {
    const res = await this.contract1.ge_euint8_euint16(255, 511);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 3 (255, 127)', async function () {
    const res = await this.contract1.ge_euint8_euint16(255, 127);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 1 (255, 255)', async function () {
    const res = await this.contract1.gt_euint8_euint16(255, 255);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 2 (255, 511)', async function () {
    const res = await this.contract1.gt_euint8_euint16(255, 511);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 3 (255, 127)', async function () {
    const res = await this.contract1.gt_euint8_euint16(255, 127);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 1 (255, 255)', async function () {
    const res = await this.contract1.le_euint8_euint16(255, 255);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 2 (255, 511)', async function () {
    const res = await this.contract1.le_euint8_euint16(255, 511);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 3 (255, 127)', async function () {
    const res = await this.contract1.le_euint8_euint16(255, 127);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 1 (255, 255)', async function () {
    const res = await this.contract1.lt_euint8_euint16(255, 255);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 2 (255, 511)', async function () {
    const res = await this.contract1.lt_euint8_euint16(255, 511);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 3 (255, 127)', async function () {
    const res = await this.contract1.lt_euint8_euint16(255, 127);
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 1 (255, 255)', async function () {
    const res = await this.contract1.min_euint8_euint16(255, 255);
    expect(res).to.equal(255);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 2 (255, 511)', async function () {
    const res = await this.contract1.min_euint8_euint16(255, 511);
    expect(res).to.equal(255);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 3 (255, 127)', async function () {
    const res = await this.contract1.min_euint8_euint16(255, 127);
    expect(res).to.equal(127);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 1 (255, 255)', async function () {
    const res = await this.contract1.max_euint8_euint16(255, 255);
    expect(res).to.equal(255);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 2 (255, 511)', async function () {
    const res = await this.contract1.max_euint8_euint16(255, 511);
    expect(res).to.equal(511);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 3 (255, 127)', async function () {
    const res = await this.contract1.max_euint8_euint16(255, 127);
    expect(res).to.equal(255);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 1 (255, 4294902015)', async function () {
    const res = await this.contract1.add_euint8_euint32(255, 4294902015);
    expect(res).to.equal(4294902270);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 1 (255, 4294902015)', async function () {
    const res = await this.contract1.sub_euint8_euint32(255, 4294902015);
    expect(res).to.equal(65536);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 2 (255, 16)', async function () {
    const res = await this.contract1.sub_euint8_euint32(255, 16);
    expect(res).to.equal(239);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 1 (16, 65536)', async function () {
    const res = await this.contract1.mul_euint8_euint32(16, 65536);
    expect(res).to.equal(1048576);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 1 (16, 65536)', async function () {
    const res = await this.contract1.and_euint8_euint32(16, 65536);
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 2 (17, 65552)', async function () {
    const res = await this.contract1.and_euint8_euint32(17, 65552);
    expect(res).to.equal(16);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 1 (16, 65536)', async function () {
    const res = await this.contract1.or_euint8_euint32(16, 65536);
    expect(res).to.equal(65552);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 2 (17, 65552)', async function () {
    const res = await this.contract1.or_euint8_euint32(17, 65552);
    expect(res).to.equal(65553);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 1 (16, 65536)', async function () {
    const res = await this.contract1.xor_euint8_euint32(16, 65536);
    expect(res).to.equal(65552);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 2 (17, 65552)', async function () {
    const res = await this.contract1.xor_euint8_euint32(17, 65552);
    expect(res).to.equal(65537);
  });

  it('test operator "shl" overload (euint8, euint32) => euint32 test 1 (16, 65536)', async function () {
    const res = await this.contract1.shl_euint8_euint32(16, 65536);
    expect(res).to.equal(16);
  });

  it('test operator "shl" overload (euint8, euint32) => euint32 test 2 (31, 65536)', async function () {
    const res = await this.contract1.shl_euint8_euint32(31, 65536);
    expect(res).to.equal(31);
  });

  it('test operator "shr" overload (euint8, euint32) => euint32 test 1 (16, 65536)', async function () {
    const res = await this.contract1.shr_euint8_euint32(16, 65536);
    expect(res).to.equal(16);
  });

  it('test operator "shr" overload (euint8, euint32) => euint32 test 2 (31, 65536)', async function () {
    const res = await this.contract1.shr_euint8_euint32(31, 65536);
    expect(res).to.equal(31);
  });

  it('test operator "shr" overload (euint8, euint32) => euint32 test 3 (16, 1)', async function () {
    const res = await this.contract1.shr_euint8_euint32(16, 1);
    expect(res).to.equal(8);
  });

  it('test operator "shr" overload (euint8, euint32) => euint32 test 4 (31, 1)', async function () {
    const res = await this.contract1.shr_euint8_euint32(31, 1);
    expect(res).to.equal(15);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.eq_euint8_euint32(1, 1);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 2 (1, 65537)', async function () {
    const res = await this.contract1.eq_euint8_euint32(1, 65537);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.ne_euint8_euint32(1, 1);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 2 (1, 65537)', async function () {
    const res = await this.contract1.ne_euint8_euint32(1, 65537);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.ge_euint8_euint32(1, 1);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 2 (1, 65537)', async function () {
    const res = await this.contract1.ge_euint8_euint32(1, 65537);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 3 (16, 1)', async function () {
    const res = await this.contract1.ge_euint8_euint32(16, 1);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.gt_euint8_euint32(1, 1);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 2 (1, 65537)', async function () {
    const res = await this.contract1.gt_euint8_euint32(1, 65537);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 3 (16, 1)', async function () {
    const res = await this.contract1.gt_euint8_euint32(16, 1);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.le_euint8_euint32(1, 1);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 2 (1, 65537)', async function () {
    const res = await this.contract1.le_euint8_euint32(1, 65537);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 3 (16, 1)', async function () {
    const res = await this.contract1.le_euint8_euint32(16, 1);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.lt_euint8_euint32(1, 1);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 2 (1, 65537)', async function () {
    const res = await this.contract1.lt_euint8_euint32(1, 65537);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 3 (16, 1)', async function () {
    const res = await this.contract1.lt_euint8_euint32(16, 1);
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract1.min_euint8_euint32(1, 1);
    expect(res).to.equal(1);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 2 (1, 65537)', async function () {
    const res = await this.contract1.min_euint8_euint32(1, 65537);
    expect(res).to.equal(1);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 3 (16, 4)', async function () {
    const res = await this.contract1.min_euint8_euint32(16, 4);
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract1.max_euint8_euint32(1, 1);
    expect(res).to.equal(1);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 2 (1, 65537)', async function () {
    const res = await this.contract1.max_euint8_euint32(1, 65537);
    expect(res).to.equal(65537);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 3 (16, 4)', async function () {
    const res = await this.contract1.max_euint8_euint32(16, 4);
    expect(res).to.equal(16);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract1.add_euint8_uint8(4, 3);
    expect(res).to.equal(7);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract1.add_uint8_euint8(4, 3);
    expect(res).to.equal(7);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract1.sub_euint8_uint8(4, 3);
    expect(res).to.equal(1);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 2 (3, 4)', async function () {
    const res = await this.contract1.sub_euint8_uint8(3, 4);
    expect(res).to.equal(255);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract1.sub_uint8_euint8(4, 3);
    expect(res).to.equal(1);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 2 (3, 4)', async function () {
    const res = await this.contract1.sub_uint8_euint8(3, 4);
    expect(res).to.equal(255);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract1.mul_euint8_uint8(4, 3);
    expect(res).to.equal(12);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 2 (3, 4)', async function () {
    const res = await this.contract1.mul_euint8_uint8(3, 4);
    expect(res).to.equal(12);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 3 (8, 2)', async function () {
    const res = await this.contract1.mul_euint8_uint8(8, 2);
    expect(res).to.equal(16);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract1.mul_uint8_euint8(4, 3);
    expect(res).to.equal(12);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 2 (3, 4)', async function () {
    const res = await this.contract1.mul_uint8_euint8(3, 4);
    expect(res).to.equal(12);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 3 (8, 2)', async function () {
    const res = await this.contract1.mul_uint8_euint8(8, 2);
    expect(res).to.equal(16);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 1 (16, 2)', async function () {
    const res = await this.contract1.div_euint8_uint8(16, 2);
    expect(res).to.equal(8);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (16, 1)', async function () {
    const res = await this.contract1.shl_euint8_uint8(16, 1);
    expect(res).to.equal(32);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.shl_euint8_uint8(16, 2);
    expect(res).to.equal(64);
  });

  it('test operator "shl" overload (uint8, euint8) => euint8 test 1 (16, 1)', async function () {
    const res = await this.contract1.shl_uint8_euint8(16, 1);
    expect(res).to.equal(32);
  });

  it('test operator "shl" overload (uint8, euint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.shl_uint8_euint8(16, 2);
    expect(res).to.equal(64);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (16, 1)', async function () {
    const res = await this.contract1.shr_euint8_uint8(16, 1);
    expect(res).to.equal(8);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.shr_euint8_uint8(16, 2);
    expect(res).to.equal(4);
  });

  it('test operator "shr" overload (uint8, euint8) => euint8 test 1 (16, 1)', async function () {
    const res = await this.contract1.shr_uint8_euint8(16, 1);
    expect(res).to.equal(8);
  });

  it('test operator "shr" overload (uint8, euint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.shr_uint8_euint8(16, 2);
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.eq_euint8_uint8(16, 16);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.eq_euint8_uint8(16, 2);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.eq_uint8_euint8(16, 16);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.eq_uint8_euint8(16, 2);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.ne_euint8_uint8(16, 16);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.ne_euint8_uint8(16, 2);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.ne_uint8_euint8(16, 16);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.ne_uint8_euint8(16, 2);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.ge_euint8_uint8(16, 16);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.ge_euint8_uint8(16, 2);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.ge_euint8_uint8(16, 17);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.ge_uint8_euint8(16, 16);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.ge_uint8_euint8(16, 2);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.ge_uint8_euint8(16, 17);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.gt_euint8_uint8(16, 16);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.gt_euint8_uint8(16, 2);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.gt_euint8_uint8(16, 17);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.gt_uint8_euint8(16, 16);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.gt_uint8_euint8(16, 2);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.gt_uint8_euint8(16, 17);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.le_euint8_uint8(16, 16);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.le_euint8_uint8(16, 2);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.le_euint8_uint8(16, 17);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.le_uint8_euint8(16, 16);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.le_uint8_euint8(16, 2);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.le_uint8_euint8(16, 17);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.lt_euint8_uint8(16, 16);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.lt_euint8_uint8(16, 2);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.lt_euint8_uint8(16, 17);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.lt_uint8_euint8(16, 16);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.lt_uint8_euint8(16, 2);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.lt_uint8_euint8(16, 17);
    expect(res).to.equal(true);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (16, 16)', async function () {
    const res = await this.contract1.min_euint8_uint8(16, 16);
    expect(res).to.equal(16);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.min_euint8_uint8(16, 2);
    expect(res).to.equal(2);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (16, 17)', async function () {
    const res = await this.contract1.min_euint8_uint8(16, 17);
    expect(res).to.equal(16);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (16, 16)', async function () {
    const res = await this.contract1.min_uint8_euint8(16, 16);
    expect(res).to.equal(16);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.min_uint8_euint8(16, 2);
    expect(res).to.equal(2);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 3 (16, 17)', async function () {
    const res = await this.contract1.min_uint8_euint8(16, 17);
    expect(res).to.equal(16);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (16, 16)', async function () {
    const res = await this.contract1.max_euint8_uint8(16, 16);
    expect(res).to.equal(16);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.max_euint8_uint8(16, 2);
    expect(res).to.equal(16);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (16, 17)', async function () {
    const res = await this.contract1.max_euint8_uint8(16, 17);
    expect(res).to.equal(17);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 1 (16, 16)', async function () {
    const res = await this.contract1.max_uint8_euint8(16, 16);
    expect(res).to.equal(16);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.max_uint8_euint8(16, 2);
    expect(res).to.equal(16);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 3 (16, 17)', async function () {
    const res = await this.contract1.max_uint8_euint8(16, 17);
    expect(res).to.equal(17);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 1 (4096, 16)', async function () {
    const res = await this.contract1.add_euint16_euint8(4096, 16);
    expect(res).to.equal(4112);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 2 (4112, 16)', async function () {
    const res = await this.contract1.add_euint16_euint8(4112, 16);
    expect(res).to.equal(4128);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (4096, 16)', async function () {
    const res = await this.contract1.sub_euint16_euint8(4096, 16);
    expect(res).to.equal(4080);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (4112, 16)', async function () {
    const res = await this.contract1.sub_euint16_euint8(4112, 16);
    expect(res).to.equal(4096);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (4096, 4)', async function () {
    const res = await this.contract1.mul_euint16_euint8(4096, 4);
    expect(res).to.equal(16384);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 1 (4096, 4)', async function () {
    const res = await this.contract1.and_euint16_euint8(4096, 4);
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 2 (4336, 240)', async function () {
    const res = await this.contract1.and_euint16_euint8(4336, 240);
    expect(res).to.equal(240);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 1 (4096, 4)', async function () {
    const res = await this.contract1.or_euint16_euint8(4096, 4);
    expect(res).to.equal(4100);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 2 (4336, 240)', async function () {
    const res = await this.contract1.or_euint16_euint8(4336, 240);
    expect(res).to.equal(4336);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (4096, 4)', async function () {
    const res = await this.contract1.xor_euint16_euint8(4096, 4);
    expect(res).to.equal(4100);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (4336, 242)', async function () {
    const res = await this.contract1.xor_euint16_euint8(4336, 242);
    expect(res).to.equal(4098);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (4112, 2)', async function () {
    const res = await this.contract1.shl_euint16_euint8(4112, 2);
    expect(res).to.equal(16448);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (4112, 2)', async function () {
    const res = await this.contract1.shr_euint16_euint8(4112, 2);
    expect(res).to.equal(1028);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.eq_euint16_euint8(16, 16);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 2 (272, 16)', async function () {
    const res = await this.contract1.eq_euint16_euint8(272, 16);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.ne_euint16_euint8(16, 16);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (272, 16)', async function () {
    const res = await this.contract1.ne_euint16_euint8(272, 16);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.ge_euint16_euint8(16, 16);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (272, 16)', async function () {
    const res = await this.contract1.ge_euint16_euint8(272, 16);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (15, 16)', async function () {
    const res = await this.contract1.ge_euint16_euint8(15, 16);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.gt_euint16_euint8(16, 16);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 2 (272, 16)', async function () {
    const res = await this.contract1.gt_euint16_euint8(272, 16);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 3 (15, 16)', async function () {
    const res = await this.contract1.gt_euint16_euint8(15, 16);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.le_euint16_euint8(16, 16);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 2 (272, 16)', async function () {
    const res = await this.contract1.le_euint16_euint8(272, 16);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 3 (15, 16)', async function () {
    const res = await this.contract1.le_euint16_euint8(15, 16);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.lt_euint16_euint8(16, 16);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 2 (272, 16)', async function () {
    const res = await this.contract1.lt_euint16_euint8(272, 16);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 3 (15, 16)', async function () {
    const res = await this.contract1.lt_euint16_euint8(15, 16);
    expect(res).to.equal(true);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 1 (16, 16)', async function () {
    const res = await this.contract1.min_euint16_euint8(16, 16);
    expect(res).to.equal(16);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 2 (272, 16)', async function () {
    const res = await this.contract1.min_euint16_euint8(272, 16);
    expect(res).to.equal(16);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 3 (15, 16)', async function () {
    const res = await this.contract1.min_euint16_euint8(15, 16);
    expect(res).to.equal(15);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 1 (16, 16)', async function () {
    const res = await this.contract1.max_euint16_euint8(16, 16);
    expect(res).to.equal(16);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 2 (272, 16)', async function () {
    const res = await this.contract1.max_euint16_euint8(272, 16);
    expect(res).to.equal(272);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 3 (15, 16)', async function () {
    const res = await this.contract1.max_euint16_euint8(15, 16);
    expect(res).to.equal(16);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 1 (258, 513)', async function () {
    const res = await this.contract1.add_euint16_euint16(258, 513);
    expect(res).to.equal(771);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 1 (1027, 258)', async function () {
    const res = await this.contract1.sub_euint16_euint16(1027, 258);
    expect(res).to.equal(769);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract1.mul_euint16_euint16(512, 2);
    expect(res).to.equal(1024);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract1.and_euint16_euint16(512, 2);
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 2 (528, 18)', async function () {
    const res = await this.contract1.and_euint16_euint16(528, 18);
    expect(res).to.equal(16);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract1.or_euint16_euint16(512, 2);
    expect(res).to.equal(514);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 2 (528, 18)', async function () {
    const res = await this.contract1.or_euint16_euint16(528, 18);
    expect(res).to.equal(530);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract1.xor_euint16_euint16(512, 2);
    expect(res).to.equal(514);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (528, 18)', async function () {
    const res = await this.contract1.xor_euint16_euint16(528, 18);
    expect(res).to.equal(514);
  });

  it('test operator "shl" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract1.shl_euint16_euint16(512, 2);
    expect(res).to.equal(2048);
  });

  it('test operator "shr" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract1.shr_euint16_euint16(512, 2);
    expect(res).to.equal(128);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 1 (512, 2)', async function () {
    const res = await this.contract1.eq_euint16_euint16(512, 2);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 2 (512, 512)', async function () {
    const res = await this.contract1.eq_euint16_euint16(512, 512);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 1 (512, 2)', async function () {
    const res = await this.contract1.ne_euint16_euint16(512, 2);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 2 (512, 512)', async function () {
    const res = await this.contract1.ne_euint16_euint16(512, 512);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 1 (512, 2)', async function () {
    const res = await this.contract1.ge_euint16_euint16(512, 2);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 2 (512, 512)', async function () {
    const res = await this.contract1.ge_euint16_euint16(512, 512);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 3 (512, 513)', async function () {
    const res = await this.contract1.ge_euint16_euint16(512, 513);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 1 (512, 2)', async function () {
    const res = await this.contract1.gt_euint16_euint16(512, 2);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 2 (512, 512)', async function () {
    const res = await this.contract1.gt_euint16_euint16(512, 512);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 3 (512, 513)', async function () {
    const res = await this.contract1.gt_euint16_euint16(512, 513);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 1 (512, 2)', async function () {
    const res = await this.contract1.le_euint16_euint16(512, 2);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 2 (512, 512)', async function () {
    const res = await this.contract1.le_euint16_euint16(512, 512);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 3 (512, 513)', async function () {
    const res = await this.contract1.le_euint16_euint16(512, 513);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 1 (512, 2)', async function () {
    const res = await this.contract1.lt_euint16_euint16(512, 2);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 2 (512, 512)', async function () {
    const res = await this.contract1.lt_euint16_euint16(512, 512);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 3 (512, 513)', async function () {
    const res = await this.contract1.lt_euint16_euint16(512, 513);
    expect(res).to.equal(true);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract1.min_euint16_euint16(512, 2);
    expect(res).to.equal(2);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 2 (512, 512)', async function () {
    const res = await this.contract1.min_euint16_euint16(512, 512);
    expect(res).to.equal(512);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 3 (512, 513)', async function () {
    const res = await this.contract1.min_euint16_euint16(512, 513);
    expect(res).to.equal(512);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract1.max_euint16_euint16(512, 2);
    expect(res).to.equal(512);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 2 (512, 512)', async function () {
    const res = await this.contract1.max_euint16_euint16(512, 512);
    expect(res).to.equal(512);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 3 (512, 513)', async function () {
    const res = await this.contract1.max_euint16_euint16(512, 513);
    expect(res).to.equal(513);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 1 (514, 131074)', async function () {
    const res = await this.contract1.add_euint16_euint32(514, 131074);
    expect(res).to.equal(131588);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (514, 2)', async function () {
    const res = await this.contract1.sub_euint16_euint32(514, 2);
    expect(res).to.equal(512);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (514, 65536)', async function () {
    const res = await this.contract1.sub_euint16_euint32(514, 65536);
    expect(res).to.equal(4294902274);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (512, 65536)', async function () {
    const res = await this.contract1.mul_euint16_euint32(512, 65536);
    expect(res).to.equal(33554432);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (514, 65536)', async function () {
    const res = await this.contract1.and_euint16_euint32(514, 65536);
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (514, 65538)', async function () {
    const res = await this.contract1.and_euint16_euint32(514, 65538);
    expect(res).to.equal(2);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (514, 65536)', async function () {
    const res = await this.contract1.or_euint16_euint32(514, 65536);
    expect(res).to.equal(66050);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (514, 65538)', async function () {
    const res = await this.contract1.or_euint16_euint32(514, 65538);
    expect(res).to.equal(66050);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 1 (514, 65536)', async function () {
    const res = await this.contract1.xor_euint16_euint32(514, 65536);
    expect(res).to.equal(66050);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 2 (514, 65538)', async function () {
    const res = await this.contract1.xor_euint16_euint32(514, 65538);
    expect(res).to.equal(66048);
  });

  it('test operator "shl" overload (euint16, euint32) => euint32 test 1 (514, 2)', async function () {
    const res = await this.contract1.shl_euint16_euint32(514, 2);
    expect(res).to.equal(2056);
  });

  it('test operator "shr" overload (euint16, euint32) => euint32 test 1 (514, 2)', async function () {
    const res = await this.contract1.shr_euint16_euint32(514, 2);
    expect(res).to.equal(128);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (514, 66050)', async function () {
    const res = await this.contract1.eq_euint16_euint32(514, 66050);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (514, 514)', async function () {
    const res = await this.contract1.eq_euint16_euint32(514, 514);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 1 (514, 66050)', async function () {
    const res = await this.contract1.ne_euint16_euint32(514, 66050);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 2 (514, 514)', async function () {
    const res = await this.contract1.ne_euint16_euint32(514, 514);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (514, 66050)', async function () {
    const res = await this.contract1.ge_euint16_euint32(514, 66050);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (514, 514)', async function () {
    const res = await this.contract1.ge_euint16_euint32(514, 514);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (514, 513)', async function () {
    const res = await this.contract1.ge_euint16_euint32(514, 513);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 1 (514, 66050)', async function () {
    const res = await this.contract1.gt_euint16_euint32(514, 66050);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 2 (514, 514)', async function () {
    const res = await this.contract1.gt_euint16_euint32(514, 514);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 3 (514, 513)', async function () {
    const res = await this.contract1.gt_euint16_euint32(514, 513);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (514, 66050)', async function () {
    const res = await this.contract1.le_euint16_euint32(514, 66050);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (514, 514)', async function () {
    const res = await this.contract1.le_euint16_euint32(514, 514);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (514, 513)', async function () {
    const res = await this.contract1.le_euint16_euint32(514, 513);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 1 (514, 66050)', async function () {
    const res = await this.contract1.lt_euint16_euint32(514, 66050);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 2 (514, 514)', async function () {
    const res = await this.contract1.lt_euint16_euint32(514, 514);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 3 (514, 513)', async function () {
    const res = await this.contract1.lt_euint16_euint32(514, 513);
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 1 (514, 66050)', async function () {
    const res = await this.contract1.min_euint16_euint32(514, 66050);
    expect(res).to.equal(514);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 2 (514, 514)', async function () {
    const res = await this.contract1.min_euint16_euint32(514, 514);
    expect(res).to.equal(514);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 3 (514, 513)', async function () {
    const res = await this.contract1.min_euint16_euint32(514, 513);
    expect(res).to.equal(513);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (514, 66050)', async function () {
    const res = await this.contract1.max_euint16_euint32(514, 66050);
    expect(res).to.equal(66050);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (514, 514)', async function () {
    const res = await this.contract1.max_euint16_euint32(514, 514);
    expect(res).to.equal(514);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (514, 513)', async function () {
    const res = await this.contract1.max_euint16_euint32(514, 513);
    expect(res).to.equal(514);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (514, 546)', async function () {
    const res = await this.contract1.add_euint16_uint16(514, 546);
    expect(res).to.equal(1060);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (514, 546)', async function () {
    const res = await this.contract1.add_uint16_euint16(514, 546);
    expect(res).to.equal(1060);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (514, 513)', async function () {
    const res = await this.contract1.sub_euint16_uint16(514, 513);
    expect(res).to.equal(1);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (514, 513)', async function () {
    const res = await this.contract1.sub_uint16_euint16(514, 513);
    expect(res).to.equal(1);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (514, 3)', async function () {
    const res = await this.contract1.mul_euint16_uint16(514, 3);
    expect(res).to.equal(1542);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (514, 3)', async function () {
    const res = await this.contract1.mul_uint16_euint16(514, 3);
    expect(res).to.equal(1542);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (1542, 3)', async function () {
    const res = await this.contract1.div_euint16_uint16(1542, 3);
    expect(res).to.equal(514);
  });

  it('test operator "shl" overload (euint16, uint16) => euint16 test 1 (1542, 3)', async function () {
    const res = await this.contract1.shl_euint16_uint16(1542, 3);
    expect(res).to.equal(12336);
  });

  it('test operator "shl" overload (uint16, euint16) => euint16 test 1 (1542, 3)', async function () {
    const res = await this.contract1.shl_uint16_euint16(1542, 3);
    expect(res).to.equal(12336);
  });

  it('test operator "shr" overload (euint16, uint16) => euint16 test 1 (1542, 3)', async function () {
    const res = await this.contract1.shr_euint16_uint16(1542, 3);
    expect(res).to.equal(192);
  });

  it('test operator "shr" overload (uint16, euint16) => euint16 test 1 (1542, 3)', async function () {
    const res = await this.contract1.shr_uint16_euint16(1542, 3);
    expect(res).to.equal(192);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract1.eq_euint16_uint16(1542, 1542);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract1.eq_euint16_uint16(1542, 1541);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract1.eq_uint16_euint16(1542, 1542);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract1.eq_uint16_euint16(1542, 1541);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract1.ne_euint16_uint16(1542, 1542);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract1.ne_euint16_uint16(1542, 1541);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract1.ne_uint16_euint16(1542, 1542);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract1.ne_uint16_euint16(1542, 1541);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract1.ge_euint16_uint16(1542, 1542);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract1.ge_euint16_uint16(1542, 1541);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract1.ge_euint16_uint16(1542, 1543);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract1.ge_uint16_euint16(1542, 1542);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract1.ge_uint16_euint16(1542, 1541);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract1.ge_uint16_euint16(1542, 1543);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract1.gt_euint16_uint16(1542, 1542);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract1.gt_euint16_uint16(1542, 1541);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract1.gt_euint16_uint16(1542, 1543);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract1.gt_uint16_euint16(1542, 1542);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract1.gt_uint16_euint16(1542, 1541);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract1.gt_uint16_euint16(1542, 1543);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract1.le_euint16_uint16(1542, 1542);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract1.le_euint16_uint16(1542, 1541);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract1.le_euint16_uint16(1542, 1543);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract1.le_uint16_euint16(1542, 1542);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract1.le_uint16_euint16(1542, 1541);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract1.le_uint16_euint16(1542, 1543);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract1.lt_euint16_uint16(1542, 1542);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract1.lt_euint16_uint16(1542, 1541);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract1.lt_euint16_uint16(1542, 1543);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract1.lt_uint16_euint16(1542, 1542);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract1.lt_uint16_euint16(1542, 1541);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract1.lt_uint16_euint16(1542, 1543);
    expect(res).to.equal(true);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (1542, 1542)', async function () {
    const res = await this.contract1.min_euint16_uint16(1542, 1542);
    expect(res).to.equal(1542);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (1542, 1541)', async function () {
    const res = await this.contract1.min_euint16_uint16(1542, 1541);
    expect(res).to.equal(1541);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (1542, 1543)', async function () {
    const res = await this.contract1.min_euint16_uint16(1542, 1543);
    expect(res).to.equal(1542);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (1542, 1542)', async function () {
    const res = await this.contract1.min_uint16_euint16(1542, 1542);
    expect(res).to.equal(1542);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (1542, 1541)', async function () {
    const res = await this.contract1.min_uint16_euint16(1542, 1541);
    expect(res).to.equal(1541);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (1542, 1543)', async function () {
    const res = await this.contract1.min_uint16_euint16(1542, 1543);
    expect(res).to.equal(1542);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (1542, 1542)', async function () {
    const res = await this.contract1.max_euint16_uint16(1542, 1542);
    expect(res).to.equal(1542);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (1542, 1541)', async function () {
    const res = await this.contract1.max_euint16_uint16(1542, 1541);
    expect(res).to.equal(1542);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (1542, 1543)', async function () {
    const res = await this.contract1.max_euint16_uint16(1542, 1543);
    expect(res).to.equal(1543);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (1542, 1542)', async function () {
    const res = await this.contract1.max_uint16_euint16(1542, 1542);
    expect(res).to.equal(1542);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (1542, 1541)', async function () {
    const res = await this.contract1.max_uint16_euint16(1542, 1541);
    expect(res).to.equal(1542);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (1542, 1543)', async function () {
    const res = await this.contract1.max_uint16_euint16(1542, 1543);
    expect(res).to.equal(1543);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (50331648, 3)', async function () {
    const res = await this.contract2.add_euint32_euint8(50331648, 3);
    expect(res).to.equal(50331651);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (50331648, 3)', async function () {
    const res = await this.contract2.sub_euint32_euint8(50331648, 3);
    expect(res).to.equal(50331645);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (50331648, 3)', async function () {
    const res = await this.contract2.mul_euint32_euint8(50331648, 3);
    expect(res).to.equal(150994944);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (50397184, 3)', async function () {
    const res = await this.contract2.and_euint32_euint8(50397184, 3);
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (50397187, 3)', async function () {
    const res = await this.contract2.and_euint32_euint8(50397187, 3);
    expect(res).to.equal(3);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (50397184, 3)', async function () {
    const res = await this.contract2.or_euint32_euint8(50397184, 3);
    expect(res).to.equal(50397187);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (50397187, 3)', async function () {
    const res = await this.contract2.or_euint32_euint8(50397187, 3);
    expect(res).to.equal(50397187);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (50397184, 3)', async function () {
    const res = await this.contract2.xor_euint32_euint8(50397184, 3);
    expect(res).to.equal(50397187);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (50397187, 3)', async function () {
    const res = await this.contract2.xor_euint32_euint8(50397187, 3);
    expect(res).to.equal(50397184);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (50397184, 3)', async function () {
    const res = await this.contract2.shl_euint32_euint8(50397184, 3);
    expect(res).to.equal(403177472);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (50397184, 3)', async function () {
    const res = await this.contract2.shr_euint32_euint8(50397184, 3);
    expect(res).to.equal(6299648);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract2.eq_euint32_euint8(3, 3);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (50331651, 3)', async function () {
    const res = await this.contract2.eq_euint32_euint8(50331651, 3);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract2.ne_euint32_euint8(3, 3);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (50331651, 3)', async function () {
    const res = await this.contract2.ne_euint32_euint8(50331651, 3);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract2.ge_euint32_euint8(3, 3);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (50331651, 3)', async function () {
    const res = await this.contract2.ge_euint32_euint8(50331651, 3);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (3, 4)', async function () {
    const res = await this.contract2.ge_euint32_euint8(3, 4);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract2.gt_euint32_euint8(3, 3);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (50331651, 3)', async function () {
    const res = await this.contract2.gt_euint32_euint8(50331651, 3);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (3, 4)', async function () {
    const res = await this.contract2.gt_euint32_euint8(3, 4);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract2.le_euint32_euint8(3, 3);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (50331651, 3)', async function () {
    const res = await this.contract2.le_euint32_euint8(50331651, 3);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (3, 4)', async function () {
    const res = await this.contract2.le_euint32_euint8(3, 4);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract2.lt_euint32_euint8(3, 3);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (50331651, 3)', async function () {
    const res = await this.contract2.lt_euint32_euint8(50331651, 3);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (3, 4)', async function () {
    const res = await this.contract2.lt_euint32_euint8(3, 4);
    expect(res).to.equal(true);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (3, 3)', async function () {
    const res = await this.contract2.min_euint32_euint8(3, 3);
    expect(res).to.equal(3);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (50331651, 3)', async function () {
    const res = await this.contract2.min_euint32_euint8(50331651, 3);
    expect(res).to.equal(3);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (3, 4)', async function () {
    const res = await this.contract2.min_euint32_euint8(3, 4);
    expect(res).to.equal(3);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (3, 3)', async function () {
    const res = await this.contract2.max_euint32_euint8(3, 3);
    expect(res).to.equal(3);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (50331651, 3)', async function () {
    const res = await this.contract2.max_euint32_euint8(50331651, 3);
    expect(res).to.equal(50331651);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (3, 4)', async function () {
    const res = await this.contract2.max_euint32_euint8(3, 4);
    expect(res).to.equal(4);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (50335779, 4099)', async function () {
    const res = await this.contract2.add_euint32_euint16(50335779, 4099);
    expect(res).to.equal(50339878);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (50335779, 4099)', async function () {
    const res = await this.contract2.sub_euint32_euint16(50335779, 4099);
    expect(res).to.equal(50331680);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (50335779, 3)', async function () {
    const res = await this.contract2.mul_euint32_euint16(50335779, 3);
    expect(res).to.equal(151007337);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (50335776, 3)', async function () {
    const res = await this.contract2.and_euint32_euint16(50335776, 3);
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (50335779, 4099)', async function () {
    const res = await this.contract2.and_euint32_euint16(50335779, 4099);
    expect(res).to.equal(4099);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (50331680, 4099)', async function () {
    const res = await this.contract2.or_euint32_euint16(50331680, 4099);
    expect(res).to.equal(50335779);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (50331683, 4099)', async function () {
    const res = await this.contract2.or_euint32_euint16(50331683, 4099);
    expect(res).to.equal(50335779);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (50331683, 4099)', async function () {
    const res = await this.contract2.xor_euint32_euint16(50331683, 4099);
    expect(res).to.equal(50335776);
  });

  it('test operator "shl" overload (euint32, euint16) => euint32 test 1 (50331648, 2)', async function () {
    const res = await this.contract2.shl_euint32_euint16(50331648, 2);
    expect(res).to.equal(201326592);
  });

  it('test operator "shr" overload (euint32, euint16) => euint32 test 1 (50331648, 2)', async function () {
    const res = await this.contract2.shr_euint32_euint16(50331648, 2);
    expect(res).to.equal(12582912);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (4096, 4096)', async function () {
    const res = await this.contract2.eq_euint32_euint16(4096, 4096);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.eq_euint32_euint16(16781312, 4096);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (4096, 4096)', async function () {
    const res = await this.contract2.ne_euint32_euint16(4096, 4096);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.ne_euint32_euint16(16781312, 4096);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (4096, 4096)', async function () {
    const res = await this.contract2.ge_euint32_euint16(4096, 4096);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.ge_euint32_euint16(16781312, 4096);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (4096, 4097)', async function () {
    const res = await this.contract2.ge_euint32_euint16(4096, 4097);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (4096, 4096)', async function () {
    const res = await this.contract2.gt_euint32_euint16(4096, 4096);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.gt_euint32_euint16(16781312, 4096);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (4096, 4097)', async function () {
    const res = await this.contract2.gt_euint32_euint16(4096, 4097);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (4096, 4096)', async function () {
    const res = await this.contract2.le_euint32_euint16(4096, 4096);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.le_euint32_euint16(16781312, 4096);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (4096, 4097)', async function () {
    const res = await this.contract2.le_euint32_euint16(4096, 4097);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (4096, 4096)', async function () {
    const res = await this.contract2.lt_euint32_euint16(4096, 4096);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.lt_euint32_euint16(16781312, 4096);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (4096, 4097)', async function () {
    const res = await this.contract2.lt_euint32_euint16(4096, 4097);
    expect(res).to.equal(true);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (4096, 4096)', async function () {
    const res = await this.contract2.min_euint32_euint16(4096, 4096);
    expect(res).to.equal(4096);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.min_euint32_euint16(16781312, 4096);
    expect(res).to.equal(4096);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (4096, 4097)', async function () {
    const res = await this.contract2.min_euint32_euint16(4096, 4097);
    expect(res).to.equal(4096);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (4096, 4096)', async function () {
    const res = await this.contract2.max_euint32_euint16(4096, 4096);
    expect(res).to.equal(4096);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.max_euint32_euint16(16781312, 4096);
    expect(res).to.equal(16781312);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (4096, 4097)', async function () {
    const res = await this.contract2.max_euint32_euint16(4096, 4097);
    expect(res).to.equal(4097);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (3280896, 1118208)', async function () {
    const res = await this.contract2.add_euint32_euint32(3280896, 1118208);
    expect(res).to.equal(4399104);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (3280896, 1118208)', async function () {
    const res = await this.contract2.sub_euint32_euint32(3280896, 1118208);
    expect(res).to.equal(2162688);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (3280896, 32)', async function () {
    const res = await this.contract2.mul_euint32_euint32(3280896, 32);
    expect(res).to.equal(104988672);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (3280896, 1409286144)', async function () {
    const res = await this.contract2.and_euint32_euint32(3280896, 1409286144);
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (3280896, 1409482752)', async function () {
    const res = await this.contract2.and_euint32_euint32(3280896, 1409482752);
    expect(res).to.equal(131072);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (3280896, 1409286144)', async function () {
    const res = await this.contract2.or_euint32_euint32(3280896, 1409286144);
    expect(res).to.equal(1412567040);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (3280896, 1409482752)', async function () {
    const res = await this.contract2.or_euint32_euint32(3280896, 1409482752);
    expect(res).to.equal(1412632576);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (3280896, 1409286144)', async function () {
    const res = await this.contract2.xor_euint32_euint32(3280896, 1409286144);
    expect(res).to.equal(1412567040);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (3280896, 1409482752)', async function () {
    const res = await this.contract2.xor_euint32_euint32(3280896, 1409482752);
    expect(res).to.equal(1412501504);
  });

  it('test operator "shl" overload (euint32, euint32) => euint32 test 1 (3280896, 2)', async function () {
    const res = await this.contract2.shl_euint32_euint32(3280896, 2);
    expect(res).to.equal(13123584);
  });

  it('test operator "shr" overload (euint32, euint32) => euint32 test 1 (3280896, 2)', async function () {
    const res = await this.contract2.shr_euint32_euint32(3280896, 2);
    expect(res).to.equal(820224);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.eq_euint32_euint32(3280896, 3280896);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.eq_euint32_euint32(3280896, 3280897);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.ne_euint32_euint32(3280896, 3280896);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.ne_euint32_euint32(3280896, 3280897);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.ge_euint32_euint32(3280896, 3280896);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.ge_euint32_euint32(3280896, 3280897);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (3280896, 3280895)', async function () {
    const res = await this.contract2.ge_euint32_euint32(3280896, 3280895);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.gt_euint32_euint32(3280896, 3280896);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.gt_euint32_euint32(3280896, 3280897);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (3280896, 3280895)', async function () {
    const res = await this.contract2.gt_euint32_euint32(3280896, 3280895);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.le_euint32_euint32(3280896, 3280896);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.le_euint32_euint32(3280896, 3280897);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (3280896, 3280895)', async function () {
    const res = await this.contract2.le_euint32_euint32(3280896, 3280895);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.lt_euint32_euint32(3280896, 3280896);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.lt_euint32_euint32(3280896, 3280897);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (3280896, 3280895)', async function () {
    const res = await this.contract2.lt_euint32_euint32(3280896, 3280895);
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.min_euint32_euint32(3280896, 3280896);
    expect(res).to.equal(3280896);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.min_euint32_euint32(3280896, 3280897);
    expect(res).to.equal(3280896);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (3280896, 3280895)', async function () {
    const res = await this.contract2.min_euint32_euint32(3280896, 3280895);
    expect(res).to.equal(3280895);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.max_euint32_euint32(3280896, 3280896);
    expect(res).to.equal(3280896);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.max_euint32_euint32(3280896, 3280897);
    expect(res).to.equal(3280897);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (3280896, 3280895)', async function () {
    const res = await this.contract2.max_euint32_euint32(3280896, 3280895);
    expect(res).to.equal(3280896);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (3416064, 3280896)', async function () {
    const res = await this.contract2.add_euint32_uint32(3416064, 3280896);
    expect(res).to.equal(6696960);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (3416064, 3280896)', async function () {
    const res = await this.contract2.add_uint32_euint32(3416064, 3280896);
    expect(res).to.equal(6696960);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (3416064, 3280896)', async function () {
    const res = await this.contract2.sub_euint32_uint32(3416064, 3280896);
    expect(res).to.equal(135168);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (3416064, 3280896)', async function () {
    const res = await this.contract2.sub_uint32_euint32(3416064, 3280896);
    expect(res).to.equal(135168);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (3416064, 256)', async function () {
    const res = await this.contract2.mul_euint32_uint32(3416064, 256);
    expect(res).to.equal(874512384);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (3416064, 256)', async function () {
    const res = await this.contract2.mul_uint32_euint32(3416064, 256);
    expect(res).to.equal(874512384);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (3416064, 256)', async function () {
    const res = await this.contract2.div_euint32_uint32(3416064, 256);
    expect(res).to.equal(13344);
  });

  it('test operator "shl" overload (euint32, uint32) => euint32 test 1 (3416064, 1)', async function () {
    const res = await this.contract2.shl_euint32_uint32(3416064, 1);
    expect(res).to.equal(6832128);
  });

  it('test operator "shl" overload (uint32, euint32) => euint32 test 1 (3416064, 1)', async function () {
    const res = await this.contract2.shl_uint32_euint32(3416064, 1);
    expect(res).to.equal(6832128);
  });

  it('test operator "shr" overload (euint32, uint32) => euint32 test 1 (3416064, 1)', async function () {
    const res = await this.contract2.shr_euint32_uint32(3416064, 1);
    expect(res).to.equal(1708032);
  });

  it('test operator "shr" overload (uint32, euint32) => euint32 test 1 (3416064, 1)', async function () {
    const res = await this.contract2.shr_uint32_euint32(3416064, 1);
    expect(res).to.equal(1708032);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.eq_euint32_uint32(3416064, 3416064);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.eq_euint32_uint32(3416064, 3416065);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.eq_uint32_euint32(3416064, 3416064);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.eq_uint32_euint32(3416064, 3416065);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.ne_euint32_uint32(3416064, 3416064);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.ne_euint32_uint32(3416064, 3416065);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.ne_uint32_euint32(3416064, 3416064);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.ne_uint32_euint32(3416064, 3416065);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.ge_euint32_uint32(3416064, 3416064);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.ge_euint32_uint32(3416064, 3416065);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract2.ge_euint32_uint32(3416064, 3416063);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.ge_uint32_euint32(3416064, 3416064);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.ge_uint32_euint32(3416064, 3416065);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract2.ge_uint32_euint32(3416064, 3416063);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.gt_euint32_uint32(3416064, 3416064);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.gt_euint32_uint32(3416064, 3416065);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract2.gt_euint32_uint32(3416064, 3416063);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.gt_uint32_euint32(3416064, 3416064);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.gt_uint32_euint32(3416064, 3416065);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract2.gt_uint32_euint32(3416064, 3416063);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.le_euint32_uint32(3416064, 3416064);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.le_euint32_uint32(3416064, 3416065);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract2.le_euint32_uint32(3416064, 3416063);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.le_uint32_euint32(3416064, 3416064);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.le_uint32_euint32(3416064, 3416065);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract2.le_uint32_euint32(3416064, 3416063);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.lt_euint32_uint32(3416064, 3416064);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.lt_euint32_uint32(3416064, 3416065);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract2.lt_euint32_uint32(3416064, 3416063);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.lt_uint32_euint32(3416064, 3416064);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.lt_uint32_euint32(3416064, 3416065);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract2.lt_uint32_euint32(3416064, 3416063);
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.min_euint32_uint32(3416064, 3416064);
    expect(res).to.equal(3416064);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.min_euint32_uint32(3416064, 3416065);
    expect(res).to.equal(3416064);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (3416064, 3416063)', async function () {
    const res = await this.contract2.min_euint32_uint32(3416064, 3416063);
    expect(res).to.equal(3416063);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.min_uint32_euint32(3416064, 3416064);
    expect(res).to.equal(3416064);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.min_uint32_euint32(3416064, 3416065);
    expect(res).to.equal(3416064);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (3416064, 3416063)', async function () {
    const res = await this.contract2.min_uint32_euint32(3416064, 3416063);
    expect(res).to.equal(3416063);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.max_euint32_uint32(3416064, 3416064);
    expect(res).to.equal(3416064);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.max_euint32_uint32(3416064, 3416065);
    expect(res).to.equal(3416065);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (3416064, 3416063)', async function () {
    const res = await this.contract2.max_euint32_uint32(3416064, 3416063);
    expect(res).to.equal(3416064);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (3416064, 3416064)', async function () {
    const res = await this.contract2.max_uint32_euint32(3416064, 3416064);
    expect(res).to.equal(3416064);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (3416064, 3416065)', async function () {
    const res = await this.contract2.max_uint32_euint32(3416064, 3416065);
    expect(res).to.equal(3416065);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (3416064, 3416063)', async function () {
    const res = await this.contract2.max_uint32_euint32(3416064, 3416063);
    expect(res).to.equal(3416064);
  });
});
