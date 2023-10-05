import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHETestSuite1 } from '../../types/contracts/tests/TFHETestSuite1';
import type { TFHETestSuite2 } from '../../types/contracts/tests/TFHETestSuite2';
import type { TFHETestSuite3 } from '../../types/contracts/tests/TFHETestSuite3';
import { createInstances } from '../instance';
import { getSigners } from '../signers';

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

async function deployTfheTestFixture3(): Promise<TFHETestSuite3> {
  const signers = await ethers.getSigners();
  const admin = signers[0];

  const contractFactory = await ethers.getContractFactory('TFHETestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('TFHE operations', function () {
  before(async function () {
    this.signers = await getSigners();

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

    const contract3 = await deployTfheTestFixture3();
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;
    const instances3 = await createInstances(this.contract3Address, ethers, this.signers);
    this.instances3 = instances3;
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 1 (3, 4)', async function () {
    const res = await this.contract1.add_euint8_euint8(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(7);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract1.sub_euint8_euint8(
      this.instances1.alice.encrypt8(4),
      this.instances1.alice.encrypt8(3),
    );
    expect(res).to.equal(1);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 1 (3, 4)', async function () {
    const res = await this.contract1.mul_euint8_euint8(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 1 (255, 15)', async function () {
    const res = await this.contract1.and_euint8_euint8(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt8(15),
    );
    expect(res).to.equal(15);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 1 (112, 15)', async function () {
    const res = await this.contract1.or_euint8_euint8(
      this.instances1.alice.encrypt8(112),
      this.instances1.alice.encrypt8(15),
    );
    expect(res).to.equal(127);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 1 (119, 119)', async function () {
    const res = await this.contract1.xor_euint8_euint8(
      this.instances1.alice.encrypt8(119),
      this.instances1.alice.encrypt8(119),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 2 (12, 34)', async function () {
    const res = await this.contract1.xor_euint8_euint8(
      this.instances1.alice.encrypt8(12),
      this.instances1.alice.encrypt8(34),
    );
    expect(res).to.equal(46);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (2, 1)', async function () {
    const res = await this.contract1.shl_euint8_euint8(
      this.instances1.alice.encrypt8(2),
      this.instances1.alice.encrypt8(1),
    );
    expect(res).to.equal(4);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 2 (2, 4)', async function () {
    const res = await this.contract1.shl_euint8_euint8(
      this.instances1.alice.encrypt8(2),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(32);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (2, 1)', async function () {
    const res = await this.contract1.shr_euint8_euint8(
      this.instances1.alice.encrypt8(2),
      this.instances1.alice.encrypt8(1),
    );
    expect(res).to.equal(1);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 2 (32, 4)', async function () {
    const res = await this.contract1.shr_euint8_euint8(
      this.instances1.alice.encrypt8(32),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(2);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 1 (12, 49)', async function () {
    const res = await this.contract1.eq_euint8_euint8(
      this.instances1.alice.encrypt8(12),
      this.instances1.alice.encrypt8(49),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 2 (7, 7)', async function () {
    const res = await this.contract1.eq_euint8_euint8(
      this.instances1.alice.encrypt8(7),
      this.instances1.alice.encrypt8(7),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 1 (1, 2)', async function () {
    const res = await this.contract1.ne_euint8_euint8(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt8(2),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 2 (2, 2)', async function () {
    const res = await this.contract1.ne_euint8_euint8(
      this.instances1.alice.encrypt8(2),
      this.instances1.alice.encrypt8(2),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 1 (10, 10)', async function () {
    const res = await this.contract1.ge_euint8_euint8(
      this.instances1.alice.encrypt8(10),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 2 (10, 9)', async function () {
    const res = await this.contract1.ge_euint8_euint8(
      this.instances1.alice.encrypt8(10),
      this.instances1.alice.encrypt8(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 3 (10, 11)', async function () {
    const res = await this.contract1.ge_euint8_euint8(
      this.instances1.alice.encrypt8(10),
      this.instances1.alice.encrypt8(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 1 (10, 10)', async function () {
    const res = await this.contract1.gt_euint8_euint8(
      this.instances1.alice.encrypt8(10),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 2 (10, 9)', async function () {
    const res = await this.contract1.gt_euint8_euint8(
      this.instances1.alice.encrypt8(10),
      this.instances1.alice.encrypt8(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 3 (10, 11)', async function () {
    const res = await this.contract1.gt_euint8_euint8(
      this.instances1.alice.encrypt8(10),
      this.instances1.alice.encrypt8(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 1 (10, 10)', async function () {
    const res = await this.contract1.le_euint8_euint8(
      this.instances1.alice.encrypt8(10),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 2 (10, 9)', async function () {
    const res = await this.contract1.le_euint8_euint8(
      this.instances1.alice.encrypt8(10),
      this.instances1.alice.encrypt8(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 3 (10, 11)', async function () {
    const res = await this.contract1.le_euint8_euint8(
      this.instances1.alice.encrypt8(10),
      this.instances1.alice.encrypt8(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 1 (10, 10)', async function () {
    const res = await this.contract1.lt_euint8_euint8(
      this.instances1.alice.encrypt8(10),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 2 (10, 9)', async function () {
    const res = await this.contract1.lt_euint8_euint8(
      this.instances1.alice.encrypt8(10),
      this.instances1.alice.encrypt8(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 3 (10, 11)', async function () {
    const res = await this.contract1.lt_euint8_euint8(
      this.instances1.alice.encrypt8(10),
      this.instances1.alice.encrypt8(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 1 (10, 10)', async function () {
    const res = await this.contract1.min_euint8_euint8(
      this.instances1.alice.encrypt8(10),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(10);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 2 (12, 10)', async function () {
    const res = await this.contract1.min_euint8_euint8(
      this.instances1.alice.encrypt8(12),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(10);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 3 (9, 12)', async function () {
    const res = await this.contract1.min_euint8_euint8(
      this.instances1.alice.encrypt8(9),
      this.instances1.alice.encrypt8(12),
    );
    expect(res).to.equal(9);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 1 (10, 10)', async function () {
    const res = await this.contract1.max_euint8_euint8(
      this.instances1.alice.encrypt8(10),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(10);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 2 (12, 10)', async function () {
    const res = await this.contract1.max_euint8_euint8(
      this.instances1.alice.encrypt8(12),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(12);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 3 (9, 12)', async function () {
    const res = await this.contract1.max_euint8_euint8(
      this.instances1.alice.encrypt8(9),
      this.instances1.alice.encrypt8(12),
    );
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 1 (3, 65280)', async function () {
    const res = await this.contract1.add_euint8_euint16(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt16(65280),
    );
    expect(res).to.equal(65283);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 1 (3, 4096)', async function () {
    const res = await this.contract1.sub_euint8_euint16(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt16(4096),
    );
    expect(res).to.equal(61443);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 1 (3, 4096)', async function () {
    const res = await this.contract1.mul_euint8_euint16(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt16(4096),
    );
    expect(res).to.equal(12288);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 1 (3, 4096)', async function () {
    const res = await this.contract1.and_euint8_euint16(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt16(4096),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 2 (3, 4097)', async function () {
    const res = await this.contract1.and_euint8_euint16(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt16(4097),
    );
    expect(res).to.equal(1);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 1 (3, 4096)', async function () {
    const res = await this.contract1.or_euint8_euint16(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt16(4096),
    );
    expect(res).to.equal(4099);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 2 (3, 4097)', async function () {
    const res = await this.contract1.or_euint8_euint16(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt16(4097),
    );
    expect(res).to.equal(4099);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 1 (255, 65535)', async function () {
    const res = await this.contract1.xor_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(65535),
    );
    expect(res).to.equal(65280);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 2 (255, 65280)', async function () {
    const res = await this.contract1.xor_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(65280),
    );
    expect(res).to.equal(65535);
  });

  it('test operator "shl" overload (euint8, euint16) => euint16 test 1 (255, 256)', async function () {
    const res = await this.contract1.shl_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(256),
    );
    expect(res).to.equal(255);
  });

  it('test operator "shl" overload (euint8, euint16) => euint16 test 2 (2, 1)', async function () {
    const res = await this.contract1.shl_euint8_euint16(
      this.instances1.alice.encrypt8(2),
      this.instances1.alice.encrypt16(1),
    );
    expect(res).to.equal(4);
  });

  it('test operator "shr" overload (euint8, euint16) => euint16 test 1 (255, 256)', async function () {
    const res = await this.contract1.shr_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(256),
    );
    expect(res).to.equal(255);
  });

  it('test operator "shr" overload (euint8, euint16) => euint16 test 2 (255, 1)', async function () {
    const res = await this.contract1.shr_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(1),
    );
    expect(res).to.equal(127);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 1 (255, 255)', async function () {
    const res = await this.contract1.eq_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(255),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 2 (255, 511)', async function () {
    const res = await this.contract1.eq_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(511),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 1 (255, 255)', async function () {
    const res = await this.contract1.ne_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(255),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 2 (255, 511)', async function () {
    const res = await this.contract1.ne_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(511),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 1 (255, 255)', async function () {
    const res = await this.contract1.ge_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(255),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 2 (255, 511)', async function () {
    const res = await this.contract1.ge_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(511),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 3 (255, 127)', async function () {
    const res = await this.contract1.ge_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(127),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 1 (255, 255)', async function () {
    const res = await this.contract1.gt_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(255),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 2 (255, 511)', async function () {
    const res = await this.contract1.gt_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(511),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 3 (255, 127)', async function () {
    const res = await this.contract1.gt_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(127),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 1 (255, 255)', async function () {
    const res = await this.contract1.le_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(255),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 2 (255, 511)', async function () {
    const res = await this.contract1.le_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(511),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 3 (255, 127)', async function () {
    const res = await this.contract1.le_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(127),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 1 (255, 255)', async function () {
    const res = await this.contract1.lt_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(255),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 2 (255, 511)', async function () {
    const res = await this.contract1.lt_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(511),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 3 (255, 127)', async function () {
    const res = await this.contract1.lt_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(127),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 1 (255, 255)', async function () {
    const res = await this.contract1.min_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(255),
    );
    expect(res).to.equal(255);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 2 (255, 511)', async function () {
    const res = await this.contract1.min_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(511),
    );
    expect(res).to.equal(255);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 3 (255, 127)', async function () {
    const res = await this.contract1.min_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(127),
    );
    expect(res).to.equal(127);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 1 (255, 255)', async function () {
    const res = await this.contract1.max_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(255),
    );
    expect(res).to.equal(255);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 2 (255, 511)', async function () {
    const res = await this.contract1.max_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(511),
    );
    expect(res).to.equal(511);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 3 (255, 127)', async function () {
    const res = await this.contract1.max_euint8_euint16(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt16(127),
    );
    expect(res).to.equal(255);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 1 (255, 4294902015)', async function () {
    const res = await this.contract1.add_euint8_euint32(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt32(4294902015),
    );
    expect(res).to.equal(4294902270);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 1 (255, 4294902015)', async function () {
    const res = await this.contract1.sub_euint8_euint32(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt32(4294902015),
    );
    expect(res).to.equal(65536);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 2 (255, 16)', async function () {
    const res = await this.contract1.sub_euint8_euint32(
      this.instances1.alice.encrypt8(255),
      this.instances1.alice.encrypt32(16),
    );
    expect(res).to.equal(239);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 1 (16, 65536)', async function () {
    const res = await this.contract1.mul_euint8_euint32(
      this.instances1.alice.encrypt8(16),
      this.instances1.alice.encrypt32(65536),
    );
    expect(res).to.equal(1048576);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 1 (16, 65536)', async function () {
    const res = await this.contract1.and_euint8_euint32(
      this.instances1.alice.encrypt8(16),
      this.instances1.alice.encrypt32(65536),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 2 (17, 65552)', async function () {
    const res = await this.contract1.and_euint8_euint32(
      this.instances1.alice.encrypt8(17),
      this.instances1.alice.encrypt32(65552),
    );
    expect(res).to.equal(16);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 1 (16, 65536)', async function () {
    const res = await this.contract1.or_euint8_euint32(
      this.instances1.alice.encrypt8(16),
      this.instances1.alice.encrypt32(65536),
    );
    expect(res).to.equal(65552);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 2 (17, 65552)', async function () {
    const res = await this.contract1.or_euint8_euint32(
      this.instances1.alice.encrypt8(17),
      this.instances1.alice.encrypt32(65552),
    );
    expect(res).to.equal(65553);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 1 (16, 65536)', async function () {
    const res = await this.contract1.xor_euint8_euint32(
      this.instances1.alice.encrypt8(16),
      this.instances1.alice.encrypt32(65536),
    );
    expect(res).to.equal(65552);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 2 (17, 65552)', async function () {
    const res = await this.contract1.xor_euint8_euint32(
      this.instances1.alice.encrypt8(17),
      this.instances1.alice.encrypt32(65552),
    );
    expect(res).to.equal(65537);
  });

  it('test operator "shl" overload (euint8, euint32) => euint32 test 1 (16, 65536)', async function () {
    const res = await this.contract1.shl_euint8_euint32(
      this.instances1.alice.encrypt8(16),
      this.instances1.alice.encrypt32(65536),
    );
    expect(res).to.equal(16);
  });

  it('test operator "shl" overload (euint8, euint32) => euint32 test 2 (31, 65536)', async function () {
    const res = await this.contract1.shl_euint8_euint32(
      this.instances1.alice.encrypt8(31),
      this.instances1.alice.encrypt32(65536),
    );
    expect(res).to.equal(31);
  });

  it('test operator "shr" overload (euint8, euint32) => euint32 test 1 (16, 65536)', async function () {
    const res = await this.contract1.shr_euint8_euint32(
      this.instances1.alice.encrypt8(16),
      this.instances1.alice.encrypt32(65536),
    );
    expect(res).to.equal(16);
  });

  it('test operator "shr" overload (euint8, euint32) => euint32 test 2 (31, 65536)', async function () {
    const res = await this.contract1.shr_euint8_euint32(
      this.instances1.alice.encrypt8(31),
      this.instances1.alice.encrypt32(65536),
    );
    expect(res).to.equal(31);
  });

  it('test operator "shr" overload (euint8, euint32) => euint32 test 3 (16, 1)', async function () {
    const res = await this.contract1.shr_euint8_euint32(
      this.instances1.alice.encrypt8(16),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(8);
  });

  it('test operator "shr" overload (euint8, euint32) => euint32 test 4 (31, 1)', async function () {
    const res = await this.contract1.shr_euint8_euint32(
      this.instances1.alice.encrypt8(31),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(15);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.eq_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 2 (1, 65537)', async function () {
    const res = await this.contract1.eq_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(65537),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.ne_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 2 (1, 65537)', async function () {
    const res = await this.contract1.ne_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(65537),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.ge_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 2 (1, 65537)', async function () {
    const res = await this.contract1.ge_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(65537),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 3 (16, 1)', async function () {
    const res = await this.contract1.ge_euint8_euint32(
      this.instances1.alice.encrypt8(16),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.gt_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 2 (1, 65537)', async function () {
    const res = await this.contract1.gt_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(65537),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 3 (16, 1)', async function () {
    const res = await this.contract1.gt_euint8_euint32(
      this.instances1.alice.encrypt8(16),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.le_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 2 (1, 65537)', async function () {
    const res = await this.contract1.le_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(65537),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 3 (16, 1)', async function () {
    const res = await this.contract1.le_euint8_euint32(
      this.instances1.alice.encrypt8(16),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.lt_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 2 (1, 65537)', async function () {
    const res = await this.contract1.lt_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(65537),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 3 (16, 1)', async function () {
    const res = await this.contract1.lt_euint8_euint32(
      this.instances1.alice.encrypt8(16),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract1.min_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(1);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 2 (1, 65537)', async function () {
    const res = await this.contract1.min_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(65537),
    );
    expect(res).to.equal(1);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 3 (16, 4)', async function () {
    const res = await this.contract1.min_euint8_euint32(
      this.instances1.alice.encrypt8(16),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract1.max_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(1);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 2 (1, 65537)', async function () {
    const res = await this.contract1.max_euint8_euint32(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt32(65537),
    );
    expect(res).to.equal(65537);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 3 (16, 4)', async function () {
    const res = await this.contract1.max_euint8_euint32(
      this.instances1.alice.encrypt8(16),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(16);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract1.add_euint8_uint8(this.instances1.alice.encrypt8(4), 3);
    expect(res).to.equal(7);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract1.add_uint8_euint8(4, this.instances1.alice.encrypt8(3));
    expect(res).to.equal(7);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract1.sub_euint8_uint8(this.instances1.alice.encrypt8(4), 3);
    expect(res).to.equal(1);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 2 (3, 4)', async function () {
    const res = await this.contract1.sub_euint8_uint8(this.instances1.alice.encrypt8(3), 4);
    expect(res).to.equal(255);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract1.sub_uint8_euint8(4, this.instances1.alice.encrypt8(3));
    expect(res).to.equal(1);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 2 (3, 4)', async function () {
    const res = await this.contract1.sub_uint8_euint8(3, this.instances1.alice.encrypt8(4));
    expect(res).to.equal(255);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract1.mul_euint8_uint8(this.instances1.alice.encrypt8(4), 3);
    expect(res).to.equal(12);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 2 (3, 4)', async function () {
    const res = await this.contract1.mul_euint8_uint8(this.instances1.alice.encrypt8(3), 4);
    expect(res).to.equal(12);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 3 (8, 2)', async function () {
    const res = await this.contract1.mul_euint8_uint8(this.instances1.alice.encrypt8(8), 2);
    expect(res).to.equal(16);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract1.mul_uint8_euint8(4, this.instances1.alice.encrypt8(3));
    expect(res).to.equal(12);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 2 (3, 4)', async function () {
    const res = await this.contract1.mul_uint8_euint8(3, this.instances1.alice.encrypt8(4));
    expect(res).to.equal(12);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 3 (8, 2)', async function () {
    const res = await this.contract1.mul_uint8_euint8(8, this.instances1.alice.encrypt8(2));
    expect(res).to.equal(16);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 1 (16, 2)', async function () {
    const res = await this.contract1.div_euint8_uint8(this.instances1.alice.encrypt8(16), 2);
    expect(res).to.equal(8);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 1 (8, 3)', async function () {
    const res = await this.contract1.rem_euint8_uint8(this.instances1.alice.encrypt8(8), 3);
    expect(res).to.equal(2);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (16, 1)', async function () {
    const res = await this.contract1.shl_euint8_uint8(this.instances1.alice.encrypt8(16), 1);
    expect(res).to.equal(32);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.shl_euint8_uint8(this.instances1.alice.encrypt8(16), 2);
    expect(res).to.equal(64);
  });

  it('test operator "shl" overload (uint8, euint8) => euint8 test 1 (16, 1)', async function () {
    const res = await this.contract1.shl_uint8_euint8(16, this.instances1.alice.encrypt8(1));
    expect(res).to.equal(32);
  });

  it('test operator "shl" overload (uint8, euint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.shl_uint8_euint8(16, this.instances1.alice.encrypt8(2));
    expect(res).to.equal(64);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (16, 1)', async function () {
    const res = await this.contract1.shr_euint8_uint8(this.instances1.alice.encrypt8(16), 1);
    expect(res).to.equal(8);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.shr_euint8_uint8(this.instances1.alice.encrypt8(16), 2);
    expect(res).to.equal(4);
  });

  it('test operator "shr" overload (uint8, euint8) => euint8 test 1 (16, 1)', async function () {
    const res = await this.contract1.shr_uint8_euint8(16, this.instances1.alice.encrypt8(1));
    expect(res).to.equal(8);
  });

  it('test operator "shr" overload (uint8, euint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.shr_uint8_euint8(16, this.instances1.alice.encrypt8(2));
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.eq_euint8_uint8(this.instances1.alice.encrypt8(16), 16);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.eq_euint8_uint8(this.instances1.alice.encrypt8(16), 2);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.eq_uint8_euint8(16, this.instances1.alice.encrypt8(16));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.eq_uint8_euint8(16, this.instances1.alice.encrypt8(2));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.ne_euint8_uint8(this.instances1.alice.encrypt8(16), 16);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.ne_euint8_uint8(this.instances1.alice.encrypt8(16), 2);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.ne_uint8_euint8(16, this.instances1.alice.encrypt8(16));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.ne_uint8_euint8(16, this.instances1.alice.encrypt8(2));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.ge_euint8_uint8(this.instances1.alice.encrypt8(16), 16);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.ge_euint8_uint8(this.instances1.alice.encrypt8(16), 2);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.ge_euint8_uint8(this.instances1.alice.encrypt8(16), 17);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.ge_uint8_euint8(16, this.instances1.alice.encrypt8(16));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.ge_uint8_euint8(16, this.instances1.alice.encrypt8(2));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.ge_uint8_euint8(16, this.instances1.alice.encrypt8(17));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.gt_euint8_uint8(this.instances1.alice.encrypt8(16), 16);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.gt_euint8_uint8(this.instances1.alice.encrypt8(16), 2);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.gt_euint8_uint8(this.instances1.alice.encrypt8(16), 17);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.gt_uint8_euint8(16, this.instances1.alice.encrypt8(16));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.gt_uint8_euint8(16, this.instances1.alice.encrypt8(2));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.gt_uint8_euint8(16, this.instances1.alice.encrypt8(17));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.le_euint8_uint8(this.instances1.alice.encrypt8(16), 16);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.le_euint8_uint8(this.instances1.alice.encrypt8(16), 2);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.le_euint8_uint8(this.instances1.alice.encrypt8(16), 17);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.le_uint8_euint8(16, this.instances1.alice.encrypt8(16));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.le_uint8_euint8(16, this.instances1.alice.encrypt8(2));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.le_uint8_euint8(16, this.instances1.alice.encrypt8(17));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.lt_euint8_uint8(this.instances1.alice.encrypt8(16), 16);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.lt_euint8_uint8(this.instances1.alice.encrypt8(16), 2);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.lt_euint8_uint8(this.instances1.alice.encrypt8(16), 17);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.lt_uint8_euint8(16, this.instances1.alice.encrypt8(16));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 2 (16, 2)', async function () {
    const res = await this.contract1.lt_uint8_euint8(16, this.instances1.alice.encrypt8(2));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 3 (16, 17)', async function () {
    const res = await this.contract1.lt_uint8_euint8(16, this.instances1.alice.encrypt8(17));
    expect(res).to.equal(true);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (16, 16)', async function () {
    const res = await this.contract1.min_euint8_uint8(this.instances1.alice.encrypt8(16), 16);
    expect(res).to.equal(16);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.min_euint8_uint8(this.instances1.alice.encrypt8(16), 2);
    expect(res).to.equal(2);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (16, 17)', async function () {
    const res = await this.contract1.min_euint8_uint8(this.instances1.alice.encrypt8(16), 17);
    expect(res).to.equal(16);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (16, 16)', async function () {
    const res = await this.contract1.min_uint8_euint8(16, this.instances1.alice.encrypt8(16));
    expect(res).to.equal(16);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.min_uint8_euint8(16, this.instances1.alice.encrypt8(2));
    expect(res).to.equal(2);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 3 (16, 17)', async function () {
    const res = await this.contract1.min_uint8_euint8(16, this.instances1.alice.encrypt8(17));
    expect(res).to.equal(16);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (16, 16)', async function () {
    const res = await this.contract1.max_euint8_uint8(this.instances1.alice.encrypt8(16), 16);
    expect(res).to.equal(16);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.max_euint8_uint8(this.instances1.alice.encrypt8(16), 2);
    expect(res).to.equal(16);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (16, 17)', async function () {
    const res = await this.contract1.max_euint8_uint8(this.instances1.alice.encrypt8(16), 17);
    expect(res).to.equal(17);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 1 (16, 16)', async function () {
    const res = await this.contract1.max_uint8_euint8(16, this.instances1.alice.encrypt8(16));
    expect(res).to.equal(16);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 2 (16, 2)', async function () {
    const res = await this.contract1.max_uint8_euint8(16, this.instances1.alice.encrypt8(2));
    expect(res).to.equal(16);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 3 (16, 17)', async function () {
    const res = await this.contract1.max_uint8_euint8(16, this.instances1.alice.encrypt8(17));
    expect(res).to.equal(17);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 1 (4096, 16)', async function () {
    const res = await this.contract1.add_euint16_euint8(
      this.instances1.alice.encrypt16(4096),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(4112);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 2 (4112, 16)', async function () {
    const res = await this.contract1.add_euint16_euint8(
      this.instances1.alice.encrypt16(4112),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(4128);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (4096, 16)', async function () {
    const res = await this.contract1.sub_euint16_euint8(
      this.instances1.alice.encrypt16(4096),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(4080);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (4112, 16)', async function () {
    const res = await this.contract1.sub_euint16_euint8(
      this.instances1.alice.encrypt16(4112),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(4096);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (4096, 4)', async function () {
    const res = await this.contract1.mul_euint16_euint8(
      this.instances1.alice.encrypt16(4096),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(16384);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 1 (4096, 4)', async function () {
    const res = await this.contract1.and_euint16_euint8(
      this.instances1.alice.encrypt16(4096),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 2 (4336, 240)', async function () {
    const res = await this.contract1.and_euint16_euint8(
      this.instances1.alice.encrypt16(4336),
      this.instances1.alice.encrypt8(240),
    );
    expect(res).to.equal(240);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 1 (4096, 4)', async function () {
    const res = await this.contract1.or_euint16_euint8(
      this.instances1.alice.encrypt16(4096),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(4100);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 2 (4336, 240)', async function () {
    const res = await this.contract1.or_euint16_euint8(
      this.instances1.alice.encrypt16(4336),
      this.instances1.alice.encrypt8(240),
    );
    expect(res).to.equal(4336);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (4096, 4)', async function () {
    const res = await this.contract1.xor_euint16_euint8(
      this.instances1.alice.encrypt16(4096),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(4100);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (4336, 242)', async function () {
    const res = await this.contract1.xor_euint16_euint8(
      this.instances1.alice.encrypt16(4336),
      this.instances1.alice.encrypt8(242),
    );
    expect(res).to.equal(4098);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (4112, 2)', async function () {
    const res = await this.contract1.shl_euint16_euint8(
      this.instances1.alice.encrypt16(4112),
      this.instances1.alice.encrypt8(2),
    );
    expect(res).to.equal(16448);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (4112, 2)', async function () {
    const res = await this.contract1.shr_euint16_euint8(
      this.instances1.alice.encrypt16(4112),
      this.instances1.alice.encrypt8(2),
    );
    expect(res).to.equal(1028);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.eq_euint16_euint8(
      this.instances1.alice.encrypt16(16),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 2 (272, 16)', async function () {
    const res = await this.contract1.eq_euint16_euint8(
      this.instances1.alice.encrypt16(272),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.ne_euint16_euint8(
      this.instances1.alice.encrypt16(16),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (272, 16)', async function () {
    const res = await this.contract1.ne_euint16_euint8(
      this.instances1.alice.encrypt16(272),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.ge_euint16_euint8(
      this.instances1.alice.encrypt16(16),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (272, 16)', async function () {
    const res = await this.contract1.ge_euint16_euint8(
      this.instances1.alice.encrypt16(272),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (15, 16)', async function () {
    const res = await this.contract1.ge_euint16_euint8(
      this.instances1.alice.encrypt16(15),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.gt_euint16_euint8(
      this.instances1.alice.encrypt16(16),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 2 (272, 16)', async function () {
    const res = await this.contract1.gt_euint16_euint8(
      this.instances1.alice.encrypt16(272),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 3 (15, 16)', async function () {
    const res = await this.contract1.gt_euint16_euint8(
      this.instances1.alice.encrypt16(15),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.le_euint16_euint8(
      this.instances1.alice.encrypt16(16),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 2 (272, 16)', async function () {
    const res = await this.contract1.le_euint16_euint8(
      this.instances1.alice.encrypt16(272),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 3 (15, 16)', async function () {
    const res = await this.contract1.le_euint16_euint8(
      this.instances1.alice.encrypt16(15),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 1 (16, 16)', async function () {
    const res = await this.contract1.lt_euint16_euint8(
      this.instances1.alice.encrypt16(16),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 2 (272, 16)', async function () {
    const res = await this.contract1.lt_euint16_euint8(
      this.instances1.alice.encrypt16(272),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 3 (15, 16)', async function () {
    const res = await this.contract1.lt_euint16_euint8(
      this.instances1.alice.encrypt16(15),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(true);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 1 (16, 16)', async function () {
    const res = await this.contract1.min_euint16_euint8(
      this.instances1.alice.encrypt16(16),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(16);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 2 (272, 16)', async function () {
    const res = await this.contract1.min_euint16_euint8(
      this.instances1.alice.encrypt16(272),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(16);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 3 (15, 16)', async function () {
    const res = await this.contract1.min_euint16_euint8(
      this.instances1.alice.encrypt16(15),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(15);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 1 (16, 16)', async function () {
    const res = await this.contract1.max_euint16_euint8(
      this.instances1.alice.encrypt16(16),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(16);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 2 (272, 16)', async function () {
    const res = await this.contract1.max_euint16_euint8(
      this.instances1.alice.encrypt16(272),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(272);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 3 (15, 16)', async function () {
    const res = await this.contract1.max_euint16_euint8(
      this.instances1.alice.encrypt16(15),
      this.instances1.alice.encrypt8(16),
    );
    expect(res).to.equal(16);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 1 (258, 513)', async function () {
    const res = await this.contract1.add_euint16_euint16(
      this.instances1.alice.encrypt16(258),
      this.instances1.alice.encrypt16(513),
    );
    expect(res).to.equal(771);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 1 (1027, 258)', async function () {
    const res = await this.contract1.sub_euint16_euint16(
      this.instances1.alice.encrypt16(1027),
      this.instances1.alice.encrypt16(258),
    );
    expect(res).to.equal(769);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract1.mul_euint16_euint16(
      this.instances1.alice.encrypt16(512),
      this.instances1.alice.encrypt16(2),
    );
    expect(res).to.equal(1024);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract1.and_euint16_euint16(
      this.instances1.alice.encrypt16(512),
      this.instances1.alice.encrypt16(2),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 2 (528, 18)', async function () {
    const res = await this.contract1.and_euint16_euint16(
      this.instances1.alice.encrypt16(528),
      this.instances1.alice.encrypt16(18),
    );
    expect(res).to.equal(16);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract1.or_euint16_euint16(
      this.instances1.alice.encrypt16(512),
      this.instances1.alice.encrypt16(2),
    );
    expect(res).to.equal(514);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 2 (528, 18)', async function () {
    const res = await this.contract1.or_euint16_euint16(
      this.instances1.alice.encrypt16(528),
      this.instances1.alice.encrypt16(18),
    );
    expect(res).to.equal(530);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract1.xor_euint16_euint16(
      this.instances1.alice.encrypt16(512),
      this.instances1.alice.encrypt16(2),
    );
    expect(res).to.equal(514);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (528, 18)', async function () {
    const res = await this.contract1.xor_euint16_euint16(
      this.instances1.alice.encrypt16(528),
      this.instances1.alice.encrypt16(18),
    );
    expect(res).to.equal(514);
  });

  it('test operator "shl" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract1.shl_euint16_euint16(
      this.instances1.alice.encrypt16(512),
      this.instances1.alice.encrypt16(2),
    );
    expect(res).to.equal(2048);
  });

  it('test operator "shr" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract1.shr_euint16_euint16(
      this.instances1.alice.encrypt16(512),
      this.instances1.alice.encrypt16(2),
    );
    expect(res).to.equal(128);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 1 (512, 2)', async function () {
    const res = await this.contract2.eq_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(2),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 2 (512, 512)', async function () {
    const res = await this.contract2.eq_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(512),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 1 (512, 2)', async function () {
    const res = await this.contract2.ne_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(2),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 2 (512, 512)', async function () {
    const res = await this.contract2.ne_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(512),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 1 (512, 2)', async function () {
    const res = await this.contract2.ge_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(2),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 2 (512, 512)', async function () {
    const res = await this.contract2.ge_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(512),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 3 (512, 513)', async function () {
    const res = await this.contract2.ge_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(513),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 1 (512, 2)', async function () {
    const res = await this.contract2.gt_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(2),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 2 (512, 512)', async function () {
    const res = await this.contract2.gt_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(512),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 3 (512, 513)', async function () {
    const res = await this.contract2.gt_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(513),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 1 (512, 2)', async function () {
    const res = await this.contract2.le_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(2),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 2 (512, 512)', async function () {
    const res = await this.contract2.le_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(512),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 3 (512, 513)', async function () {
    const res = await this.contract2.le_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(513),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 1 (512, 2)', async function () {
    const res = await this.contract2.lt_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(2),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 2 (512, 512)', async function () {
    const res = await this.contract2.lt_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(512),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 3 (512, 513)', async function () {
    const res = await this.contract2.lt_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(513),
    );
    expect(res).to.equal(true);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract2.min_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(2),
    );
    expect(res).to.equal(2);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 2 (512, 512)', async function () {
    const res = await this.contract2.min_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(512),
    );
    expect(res).to.equal(512);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 3 (512, 513)', async function () {
    const res = await this.contract2.min_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(513),
    );
    expect(res).to.equal(512);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 1 (512, 2)', async function () {
    const res = await this.contract2.max_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(2),
    );
    expect(res).to.equal(512);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 2 (512, 512)', async function () {
    const res = await this.contract2.max_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(512),
    );
    expect(res).to.equal(512);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 3 (512, 513)', async function () {
    const res = await this.contract2.max_euint16_euint16(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt16(513),
    );
    expect(res).to.equal(513);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 1 (514, 131074)', async function () {
    const res = await this.contract2.add_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(131074),
    );
    expect(res).to.equal(131588);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (514, 2)', async function () {
    const res = await this.contract2.sub_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(2),
    );
    expect(res).to.equal(512);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (514, 65536)', async function () {
    const res = await this.contract2.sub_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(65536),
    );
    expect(res).to.equal(4294902274);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (512, 65536)', async function () {
    const res = await this.contract2.mul_euint16_euint32(
      this.instances2.alice.encrypt16(512),
      this.instances2.alice.encrypt32(65536),
    );
    expect(res).to.equal(33554432);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (514, 65536)', async function () {
    const res = await this.contract2.and_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(65536),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (514, 65538)', async function () {
    const res = await this.contract2.and_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(65538),
    );
    expect(res).to.equal(2);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (514, 65536)', async function () {
    const res = await this.contract2.or_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(65536),
    );
    expect(res).to.equal(66050);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (514, 65538)', async function () {
    const res = await this.contract2.or_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(65538),
    );
    expect(res).to.equal(66050);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 1 (514, 65536)', async function () {
    const res = await this.contract2.xor_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(65536),
    );
    expect(res).to.equal(66050);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 2 (514, 65538)', async function () {
    const res = await this.contract2.xor_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(65538),
    );
    expect(res).to.equal(66048);
  });

  it('test operator "shl" overload (euint16, euint32) => euint32 test 1 (514, 2)', async function () {
    const res = await this.contract2.shl_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(2),
    );
    expect(res).to.equal(2056);
  });

  it('test operator "shr" overload (euint16, euint32) => euint32 test 1 (514, 2)', async function () {
    const res = await this.contract2.shr_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(2),
    );
    expect(res).to.equal(128);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (514, 66050)', async function () {
    const res = await this.contract2.eq_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(66050),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (514, 514)', async function () {
    const res = await this.contract2.eq_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(514),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 1 (514, 66050)', async function () {
    const res = await this.contract2.ne_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(66050),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 2 (514, 514)', async function () {
    const res = await this.contract2.ne_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(514),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (514, 66050)', async function () {
    const res = await this.contract2.ge_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(66050),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (514, 514)', async function () {
    const res = await this.contract2.ge_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(514),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (514, 513)', async function () {
    const res = await this.contract2.ge_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(513),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 1 (514, 66050)', async function () {
    const res = await this.contract2.gt_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(66050),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 2 (514, 514)', async function () {
    const res = await this.contract2.gt_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(514),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 3 (514, 513)', async function () {
    const res = await this.contract2.gt_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(513),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (514, 66050)', async function () {
    const res = await this.contract2.le_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(66050),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (514, 514)', async function () {
    const res = await this.contract2.le_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(514),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (514, 513)', async function () {
    const res = await this.contract2.le_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(513),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 1 (514, 66050)', async function () {
    const res = await this.contract2.lt_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(66050),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 2 (514, 514)', async function () {
    const res = await this.contract2.lt_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(514),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 3 (514, 513)', async function () {
    const res = await this.contract2.lt_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(513),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 1 (514, 66050)', async function () {
    const res = await this.contract2.min_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(66050),
    );
    expect(res).to.equal(514);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 2 (514, 514)', async function () {
    const res = await this.contract2.min_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(514),
    );
    expect(res).to.equal(514);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 3 (514, 513)', async function () {
    const res = await this.contract2.min_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(513),
    );
    expect(res).to.equal(513);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (514, 66050)', async function () {
    const res = await this.contract2.max_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(66050),
    );
    expect(res).to.equal(66050);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (514, 514)', async function () {
    const res = await this.contract2.max_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(514),
    );
    expect(res).to.equal(514);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (514, 513)', async function () {
    const res = await this.contract2.max_euint16_euint32(
      this.instances2.alice.encrypt16(514),
      this.instances2.alice.encrypt32(513),
    );
    expect(res).to.equal(514);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (514, 546)', async function () {
    const res = await this.contract2.add_euint16_uint16(this.instances2.alice.encrypt16(514), 546);
    expect(res).to.equal(1060);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (514, 546)', async function () {
    const res = await this.contract2.add_uint16_euint16(514, this.instances2.alice.encrypt16(546));
    expect(res).to.equal(1060);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (514, 513)', async function () {
    const res = await this.contract2.sub_euint16_uint16(this.instances2.alice.encrypt16(514), 513);
    expect(res).to.equal(1);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (514, 513)', async function () {
    const res = await this.contract2.sub_uint16_euint16(514, this.instances2.alice.encrypt16(513));
    expect(res).to.equal(1);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (514, 3)', async function () {
    const res = await this.contract2.mul_euint16_uint16(this.instances2.alice.encrypt16(514), 3);
    expect(res).to.equal(1542);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (514, 3)', async function () {
    const res = await this.contract2.mul_uint16_euint16(514, this.instances2.alice.encrypt16(3));
    expect(res).to.equal(1542);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (1542, 3)', async function () {
    const res = await this.contract2.div_euint16_uint16(this.instances2.alice.encrypt16(1542), 3);
    expect(res).to.equal(514);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (1544, 3)', async function () {
    const res = await this.contract2.rem_euint16_uint16(this.instances2.alice.encrypt16(1544), 3);
    expect(res).to.equal(2);
  });

  it('test operator "shl" overload (euint16, uint16) => euint16 test 1 (1542, 3)', async function () {
    const res = await this.contract2.shl_euint16_uint16(this.instances2.alice.encrypt16(1542), 3);
    expect(res).to.equal(12336);
  });

  it('test operator "shl" overload (uint16, euint16) => euint16 test 1 (1542, 3)', async function () {
    const res = await this.contract2.shl_uint16_euint16(1542, this.instances2.alice.encrypt16(3));
    expect(res).to.equal(12336);
  });

  it('test operator "shr" overload (euint16, uint16) => euint16 test 1 (1542, 3)', async function () {
    const res = await this.contract2.shr_euint16_uint16(this.instances2.alice.encrypt16(1542), 3);
    expect(res).to.equal(192);
  });

  it('test operator "shr" overload (uint16, euint16) => euint16 test 1 (1542, 3)', async function () {
    const res = await this.contract2.shr_uint16_euint16(1542, this.instances2.alice.encrypt16(3));
    expect(res).to.equal(192);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract2.eq_euint16_uint16(this.instances2.alice.encrypt16(1542), 1542);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract2.eq_euint16_uint16(this.instances2.alice.encrypt16(1542), 1541);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract2.eq_uint16_euint16(1542, this.instances2.alice.encrypt16(1542));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract2.eq_uint16_euint16(1542, this.instances2.alice.encrypt16(1541));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract2.ne_euint16_uint16(this.instances2.alice.encrypt16(1542), 1542);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract2.ne_euint16_uint16(this.instances2.alice.encrypt16(1542), 1541);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract2.ne_uint16_euint16(1542, this.instances2.alice.encrypt16(1542));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract2.ne_uint16_euint16(1542, this.instances2.alice.encrypt16(1541));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract2.ge_euint16_uint16(this.instances2.alice.encrypt16(1542), 1542);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract2.ge_euint16_uint16(this.instances2.alice.encrypt16(1542), 1541);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract2.ge_euint16_uint16(this.instances2.alice.encrypt16(1542), 1543);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract2.ge_uint16_euint16(1542, this.instances2.alice.encrypt16(1542));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract2.ge_uint16_euint16(1542, this.instances2.alice.encrypt16(1541));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract2.ge_uint16_euint16(1542, this.instances2.alice.encrypt16(1543));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract2.gt_euint16_uint16(this.instances2.alice.encrypt16(1542), 1542);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract2.gt_euint16_uint16(this.instances2.alice.encrypt16(1542), 1541);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract2.gt_euint16_uint16(this.instances2.alice.encrypt16(1542), 1543);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract2.gt_uint16_euint16(1542, this.instances2.alice.encrypt16(1542));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract2.gt_uint16_euint16(1542, this.instances2.alice.encrypt16(1541));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract2.gt_uint16_euint16(1542, this.instances2.alice.encrypt16(1543));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract2.le_euint16_uint16(this.instances2.alice.encrypt16(1542), 1542);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract2.le_euint16_uint16(this.instances2.alice.encrypt16(1542), 1541);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract2.le_euint16_uint16(this.instances2.alice.encrypt16(1542), 1543);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract2.le_uint16_euint16(1542, this.instances2.alice.encrypt16(1542));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract2.le_uint16_euint16(1542, this.instances2.alice.encrypt16(1541));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract2.le_uint16_euint16(1542, this.instances2.alice.encrypt16(1543));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract2.lt_euint16_uint16(this.instances2.alice.encrypt16(1542), 1542);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract2.lt_euint16_uint16(this.instances2.alice.encrypt16(1542), 1541);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract2.lt_euint16_uint16(this.instances2.alice.encrypt16(1542), 1543);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (1542, 1542)', async function () {
    const res = await this.contract2.lt_uint16_euint16(1542, this.instances2.alice.encrypt16(1542));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (1542, 1541)', async function () {
    const res = await this.contract2.lt_uint16_euint16(1542, this.instances2.alice.encrypt16(1541));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (1542, 1543)', async function () {
    const res = await this.contract2.lt_uint16_euint16(1542, this.instances2.alice.encrypt16(1543));
    expect(res).to.equal(true);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (1542, 1542)', async function () {
    const res = await this.contract2.min_euint16_uint16(this.instances2.alice.encrypt16(1542), 1542);
    expect(res).to.equal(1542);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (1542, 1541)', async function () {
    const res = await this.contract2.min_euint16_uint16(this.instances2.alice.encrypt16(1542), 1541);
    expect(res).to.equal(1541);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (1542, 1543)', async function () {
    const res = await this.contract2.min_euint16_uint16(this.instances2.alice.encrypt16(1542), 1543);
    expect(res).to.equal(1542);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (1542, 1542)', async function () {
    const res = await this.contract2.min_uint16_euint16(1542, this.instances2.alice.encrypt16(1542));
    expect(res).to.equal(1542);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (1542, 1541)', async function () {
    const res = await this.contract2.min_uint16_euint16(1542, this.instances2.alice.encrypt16(1541));
    expect(res).to.equal(1541);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (1542, 1543)', async function () {
    const res = await this.contract2.min_uint16_euint16(1542, this.instances2.alice.encrypt16(1543));
    expect(res).to.equal(1542);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (1542, 1542)', async function () {
    const res = await this.contract2.max_euint16_uint16(this.instances2.alice.encrypt16(1542), 1542);
    expect(res).to.equal(1542);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (1542, 1541)', async function () {
    const res = await this.contract2.max_euint16_uint16(this.instances2.alice.encrypt16(1542), 1541);
    expect(res).to.equal(1542);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (1542, 1543)', async function () {
    const res = await this.contract2.max_euint16_uint16(this.instances2.alice.encrypt16(1542), 1543);
    expect(res).to.equal(1543);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (1542, 1542)', async function () {
    const res = await this.contract2.max_uint16_euint16(1542, this.instances2.alice.encrypt16(1542));
    expect(res).to.equal(1542);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (1542, 1541)', async function () {
    const res = await this.contract2.max_uint16_euint16(1542, this.instances2.alice.encrypt16(1541));
    expect(res).to.equal(1542);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (1542, 1543)', async function () {
    const res = await this.contract2.max_uint16_euint16(1542, this.instances2.alice.encrypt16(1543));
    expect(res).to.equal(1543);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (50331648, 3)', async function () {
    const res = await this.contract2.add_euint32_euint8(
      this.instances2.alice.encrypt32(50331648),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(50331651);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (50331648, 3)', async function () {
    const res = await this.contract2.sub_euint32_euint8(
      this.instances2.alice.encrypt32(50331648),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(50331645);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (50331648, 3)', async function () {
    const res = await this.contract2.mul_euint32_euint8(
      this.instances2.alice.encrypt32(50331648),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(150994944);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (50397184, 3)', async function () {
    const res = await this.contract2.and_euint32_euint8(
      this.instances2.alice.encrypt32(50397184),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (50397187, 3)', async function () {
    const res = await this.contract2.and_euint32_euint8(
      this.instances2.alice.encrypt32(50397187),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(3);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (50397184, 3)', async function () {
    const res = await this.contract2.or_euint32_euint8(
      this.instances2.alice.encrypt32(50397184),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(50397187);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (50397187, 3)', async function () {
    const res = await this.contract2.or_euint32_euint8(
      this.instances2.alice.encrypt32(50397187),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(50397187);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (50397184, 3)', async function () {
    const res = await this.contract2.xor_euint32_euint8(
      this.instances2.alice.encrypt32(50397184),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(50397187);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (50397187, 3)', async function () {
    const res = await this.contract2.xor_euint32_euint8(
      this.instances2.alice.encrypt32(50397187),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(50397184);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (50397184, 3)', async function () {
    const res = await this.contract2.shl_euint32_euint8(
      this.instances2.alice.encrypt32(50397184),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(403177472);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (50397184, 3)', async function () {
    const res = await this.contract2.shr_euint32_euint8(
      this.instances2.alice.encrypt32(50397184),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(6299648);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract2.eq_euint32_euint8(
      this.instances2.alice.encrypt32(3),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (50331651, 3)', async function () {
    const res = await this.contract2.eq_euint32_euint8(
      this.instances2.alice.encrypt32(50331651),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract2.ne_euint32_euint8(
      this.instances2.alice.encrypt32(3),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (50331651, 3)', async function () {
    const res = await this.contract2.ne_euint32_euint8(
      this.instances2.alice.encrypt32(50331651),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract2.ge_euint32_euint8(
      this.instances2.alice.encrypt32(3),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (50331651, 3)', async function () {
    const res = await this.contract2.ge_euint32_euint8(
      this.instances2.alice.encrypt32(50331651),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (3, 4)', async function () {
    const res = await this.contract2.ge_euint32_euint8(
      this.instances2.alice.encrypt32(3),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract2.gt_euint32_euint8(
      this.instances2.alice.encrypt32(3),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (50331651, 3)', async function () {
    const res = await this.contract2.gt_euint32_euint8(
      this.instances2.alice.encrypt32(50331651),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (3, 4)', async function () {
    const res = await this.contract2.gt_euint32_euint8(
      this.instances2.alice.encrypt32(3),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract2.le_euint32_euint8(
      this.instances2.alice.encrypt32(3),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (50331651, 3)', async function () {
    const res = await this.contract2.le_euint32_euint8(
      this.instances2.alice.encrypt32(50331651),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (3, 4)', async function () {
    const res = await this.contract2.le_euint32_euint8(
      this.instances2.alice.encrypt32(3),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract2.lt_euint32_euint8(
      this.instances2.alice.encrypt32(3),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (50331651, 3)', async function () {
    const res = await this.contract2.lt_euint32_euint8(
      this.instances2.alice.encrypt32(50331651),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (3, 4)', async function () {
    const res = await this.contract2.lt_euint32_euint8(
      this.instances2.alice.encrypt32(3),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (3, 3)', async function () {
    const res = await this.contract2.min_euint32_euint8(
      this.instances2.alice.encrypt32(3),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(3);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (50331651, 3)', async function () {
    const res = await this.contract2.min_euint32_euint8(
      this.instances2.alice.encrypt32(50331651),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(3);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (3, 4)', async function () {
    const res = await this.contract2.min_euint32_euint8(
      this.instances2.alice.encrypt32(3),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(3);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (3, 3)', async function () {
    const res = await this.contract2.max_euint32_euint8(
      this.instances2.alice.encrypt32(3),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(3);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (50331651, 3)', async function () {
    const res = await this.contract2.max_euint32_euint8(
      this.instances2.alice.encrypt32(50331651),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(50331651);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (3, 4)', async function () {
    const res = await this.contract2.max_euint32_euint8(
      this.instances2.alice.encrypt32(3),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (50335779, 4099)', async function () {
    const res = await this.contract2.add_euint32_euint16(
      this.instances2.alice.encrypt32(50335779),
      this.instances2.alice.encrypt16(4099),
    );
    expect(res).to.equal(50339878);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (50335779, 4099)', async function () {
    const res = await this.contract2.sub_euint32_euint16(
      this.instances2.alice.encrypt32(50335779),
      this.instances2.alice.encrypt16(4099),
    );
    expect(res).to.equal(50331680);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (50335779, 3)', async function () {
    const res = await this.contract2.mul_euint32_euint16(
      this.instances2.alice.encrypt32(50335779),
      this.instances2.alice.encrypt16(3),
    );
    expect(res).to.equal(151007337);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (50335776, 3)', async function () {
    const res = await this.contract2.and_euint32_euint16(
      this.instances2.alice.encrypt32(50335776),
      this.instances2.alice.encrypt16(3),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (50335779, 4099)', async function () {
    const res = await this.contract2.and_euint32_euint16(
      this.instances2.alice.encrypt32(50335779),
      this.instances2.alice.encrypt16(4099),
    );
    expect(res).to.equal(4099);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (50331680, 4099)', async function () {
    const res = await this.contract2.or_euint32_euint16(
      this.instances2.alice.encrypt32(50331680),
      this.instances2.alice.encrypt16(4099),
    );
    expect(res).to.equal(50335779);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (50331683, 4099)', async function () {
    const res = await this.contract2.or_euint32_euint16(
      this.instances2.alice.encrypt32(50331683),
      this.instances2.alice.encrypt16(4099),
    );
    expect(res).to.equal(50335779);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (50331683, 4099)', async function () {
    const res = await this.contract2.xor_euint32_euint16(
      this.instances2.alice.encrypt32(50331683),
      this.instances2.alice.encrypt16(4099),
    );
    expect(res).to.equal(50335776);
  });

  it('test operator "shl" overload (euint32, euint16) => euint32 test 1 (50331648, 2)', async function () {
    const res = await this.contract2.shl_euint32_euint16(
      this.instances2.alice.encrypt32(50331648),
      this.instances2.alice.encrypt16(2),
    );
    expect(res).to.equal(201326592);
  });

  it('test operator "shr" overload (euint32, euint16) => euint32 test 1 (50331648, 2)', async function () {
    const res = await this.contract2.shr_euint32_euint16(
      this.instances2.alice.encrypt32(50331648),
      this.instances2.alice.encrypt16(2),
    );
    expect(res).to.equal(12582912);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (4096, 4096)', async function () {
    const res = await this.contract2.eq_euint32_euint16(
      this.instances2.alice.encrypt32(4096),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.eq_euint32_euint16(
      this.instances2.alice.encrypt32(16781312),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (4096, 4096)', async function () {
    const res = await this.contract2.ne_euint32_euint16(
      this.instances2.alice.encrypt32(4096),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.ne_euint32_euint16(
      this.instances2.alice.encrypt32(16781312),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (4096, 4096)', async function () {
    const res = await this.contract2.ge_euint32_euint16(
      this.instances2.alice.encrypt32(4096),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.ge_euint32_euint16(
      this.instances2.alice.encrypt32(16781312),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (4096, 4097)', async function () {
    const res = await this.contract2.ge_euint32_euint16(
      this.instances2.alice.encrypt32(4096),
      this.instances2.alice.encrypt16(4097),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (4096, 4096)', async function () {
    const res = await this.contract2.gt_euint32_euint16(
      this.instances2.alice.encrypt32(4096),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.gt_euint32_euint16(
      this.instances2.alice.encrypt32(16781312),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (4096, 4097)', async function () {
    const res = await this.contract2.gt_euint32_euint16(
      this.instances2.alice.encrypt32(4096),
      this.instances2.alice.encrypt16(4097),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (4096, 4096)', async function () {
    const res = await this.contract2.le_euint32_euint16(
      this.instances2.alice.encrypt32(4096),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.le_euint32_euint16(
      this.instances2.alice.encrypt32(16781312),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (4096, 4097)', async function () {
    const res = await this.contract2.le_euint32_euint16(
      this.instances2.alice.encrypt32(4096),
      this.instances2.alice.encrypt16(4097),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (4096, 4096)', async function () {
    const res = await this.contract2.lt_euint32_euint16(
      this.instances2.alice.encrypt32(4096),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.lt_euint32_euint16(
      this.instances2.alice.encrypt32(16781312),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (4096, 4097)', async function () {
    const res = await this.contract2.lt_euint32_euint16(
      this.instances2.alice.encrypt32(4096),
      this.instances2.alice.encrypt16(4097),
    );
    expect(res).to.equal(true);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (4096, 4096)', async function () {
    const res = await this.contract2.min_euint32_euint16(
      this.instances2.alice.encrypt32(4096),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(4096);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.min_euint32_euint16(
      this.instances2.alice.encrypt32(16781312),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(4096);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (4096, 4097)', async function () {
    const res = await this.contract2.min_euint32_euint16(
      this.instances2.alice.encrypt32(4096),
      this.instances2.alice.encrypt16(4097),
    );
    expect(res).to.equal(4096);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (4096, 4096)', async function () {
    const res = await this.contract2.max_euint32_euint16(
      this.instances2.alice.encrypt32(4096),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(4096);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (16781312, 4096)', async function () {
    const res = await this.contract2.max_euint32_euint16(
      this.instances2.alice.encrypt32(16781312),
      this.instances2.alice.encrypt16(4096),
    );
    expect(res).to.equal(16781312);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (4096, 4097)', async function () {
    const res = await this.contract2.max_euint32_euint16(
      this.instances2.alice.encrypt32(4096),
      this.instances2.alice.encrypt16(4097),
    );
    expect(res).to.equal(4097);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (3280896, 1118208)', async function () {
    const res = await this.contract2.add_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(1118208),
    );
    expect(res).to.equal(4399104);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (3280896, 1118208)', async function () {
    const res = await this.contract2.sub_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(1118208),
    );
    expect(res).to.equal(2162688);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (3280896, 32)', async function () {
    const res = await this.contract2.mul_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(32),
    );
    expect(res).to.equal(104988672);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (3280896, 1409286144)', async function () {
    const res = await this.contract2.and_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(1409286144),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (3280896, 1409482752)', async function () {
    const res = await this.contract2.and_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(1409482752),
    );
    expect(res).to.equal(131072);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (3280896, 1409286144)', async function () {
    const res = await this.contract2.or_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(1409286144),
    );
    expect(res).to.equal(1412567040);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (3280896, 1409482752)', async function () {
    const res = await this.contract2.or_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(1409482752),
    );
    expect(res).to.equal(1412632576);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (3280896, 1409286144)', async function () {
    const res = await this.contract2.xor_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(1409286144),
    );
    expect(res).to.equal(1412567040);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (3280896, 1409482752)', async function () {
    const res = await this.contract2.xor_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(1409482752),
    );
    expect(res).to.equal(1412501504);
  });

  it('test operator "shl" overload (euint32, euint32) => euint32 test 1 (3280896, 2)', async function () {
    const res = await this.contract2.shl_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(2),
    );
    expect(res).to.equal(13123584);
  });

  it('test operator "shr" overload (euint32, euint32) => euint32 test 1 (3280896, 2)', async function () {
    const res = await this.contract2.shr_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(2),
    );
    expect(res).to.equal(820224);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.eq_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280896),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.eq_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280897),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.ne_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280896),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.ne_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280897),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.ge_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280896),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.ge_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280897),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (3280896, 3280895)', async function () {
    const res = await this.contract2.ge_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280895),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.gt_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280896),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.gt_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280897),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (3280896, 3280895)', async function () {
    const res = await this.contract2.gt_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280895),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.le_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280896),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.le_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280897),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (3280896, 3280895)', async function () {
    const res = await this.contract2.le_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280895),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.lt_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280896),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.lt_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280897),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (3280896, 3280895)', async function () {
    const res = await this.contract2.lt_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280895),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.min_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280896),
    );
    expect(res).to.equal(3280896);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.min_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280897),
    );
    expect(res).to.equal(3280896);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (3280896, 3280895)', async function () {
    const res = await this.contract2.min_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280895),
    );
    expect(res).to.equal(3280895);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (3280896, 3280896)', async function () {
    const res = await this.contract2.max_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280896),
    );
    expect(res).to.equal(3280896);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (3280896, 3280897)', async function () {
    const res = await this.contract2.max_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280897),
    );
    expect(res).to.equal(3280897);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (3280896, 3280895)', async function () {
    const res = await this.contract2.max_euint32_euint32(
      this.instances2.alice.encrypt32(3280896),
      this.instances2.alice.encrypt32(3280895),
    );
    expect(res).to.equal(3280896);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (3416064, 3280896)', async function () {
    const res = await this.contract3.add_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3280896);
    expect(res).to.equal(6696960);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (3416064, 3280896)', async function () {
    const res = await this.contract3.add_uint32_euint32(3416064, this.instances3.alice.encrypt32(3280896));
    expect(res).to.equal(6696960);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (3416064, 3280896)', async function () {
    const res = await this.contract3.sub_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3280896);
    expect(res).to.equal(135168);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (3416064, 3280896)', async function () {
    const res = await this.contract3.sub_uint32_euint32(3416064, this.instances3.alice.encrypt32(3280896));
    expect(res).to.equal(135168);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (3416064, 256)', async function () {
    const res = await this.contract3.mul_euint32_uint32(this.instances3.alice.encrypt32(3416064), 256);
    expect(res).to.equal(874512384);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (3416064, 256)', async function () {
    const res = await this.contract3.mul_uint32_euint32(3416064, this.instances3.alice.encrypt32(256));
    expect(res).to.equal(874512384);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (3416064, 256)', async function () {
    const res = await this.contract3.div_euint32_uint32(this.instances3.alice.encrypt32(3416064), 256);
    expect(res).to.equal(13344);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (3416121, 256)', async function () {
    const res = await this.contract3.rem_euint32_uint32(this.instances3.alice.encrypt32(3416121), 256);
    expect(res).to.equal(57);
  });

  it('test operator "shl" overload (euint32, uint32) => euint32 test 1 (3416064, 1)', async function () {
    const res = await this.contract3.shl_euint32_uint32(this.instances3.alice.encrypt32(3416064), 1);
    expect(res).to.equal(6832128);
  });

  it('test operator "shl" overload (uint32, euint32) => euint32 test 1 (3416064, 1)', async function () {
    const res = await this.contract3.shl_uint32_euint32(3416064, this.instances3.alice.encrypt32(1));
    expect(res).to.equal(6832128);
  });

  it('test operator "shr" overload (euint32, uint32) => euint32 test 1 (3416064, 1)', async function () {
    const res = await this.contract3.shr_euint32_uint32(this.instances3.alice.encrypt32(3416064), 1);
    expect(res).to.equal(1708032);
  });

  it('test operator "shr" overload (uint32, euint32) => euint32 test 1 (3416064, 1)', async function () {
    const res = await this.contract3.shr_uint32_euint32(3416064, this.instances3.alice.encrypt32(1));
    expect(res).to.equal(1708032);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.eq_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416064);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.eq_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416065);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.eq_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416064));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.eq_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416065));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.ne_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416064);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.ne_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416065);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.ne_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416064));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.ne_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416065));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.ge_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416064);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.ge_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416065);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract3.ge_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416063);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.ge_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416064));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.ge_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416065));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract3.ge_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416063));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.gt_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416064);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.gt_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416065);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract3.gt_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416063);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.gt_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416064));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.gt_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416065));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract3.gt_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416063));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.le_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416064);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.le_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416065);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract3.le_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416063);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.le_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416064));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.le_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416065));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract3.le_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416063));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.lt_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416064);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.lt_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416065);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract3.lt_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416063);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.lt_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416064));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.lt_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416065));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (3416064, 3416063)', async function () {
    const res = await this.contract3.lt_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416063));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.min_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416064);
    expect(res).to.equal(3416064);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.min_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416065);
    expect(res).to.equal(3416064);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (3416064, 3416063)', async function () {
    const res = await this.contract3.min_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416063);
    expect(res).to.equal(3416063);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.min_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416064));
    expect(res).to.equal(3416064);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.min_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416065));
    expect(res).to.equal(3416064);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (3416064, 3416063)', async function () {
    const res = await this.contract3.min_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416063));
    expect(res).to.equal(3416063);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.max_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416064);
    expect(res).to.equal(3416064);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.max_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416065);
    expect(res).to.equal(3416065);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (3416064, 3416063)', async function () {
    const res = await this.contract3.max_euint32_uint32(this.instances3.alice.encrypt32(3416064), 3416063);
    expect(res).to.equal(3416064);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (3416064, 3416064)', async function () {
    const res = await this.contract3.max_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416064));
    expect(res).to.equal(3416064);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (3416064, 3416065)', async function () {
    const res = await this.contract3.max_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416065));
    expect(res).to.equal(3416065);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (3416064, 3416063)', async function () {
    const res = await this.contract3.max_uint32_euint32(3416064, this.instances3.alice.encrypt32(3416063));
    expect(res).to.equal(3416064);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (1)', async function () {
    const res = await this.contract3.neg_euint8(this.instances3.alice.encrypt8(1));
    expect(res).to.equal(255);
  });

  it('test operator "neg" overload (euint8) => euint8 test 2 (2)', async function () {
    const res = await this.contract3.neg_euint8(this.instances3.alice.encrypt8(2));
    expect(res).to.equal(254);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (3)', async function () {
    const res = await this.contract3.not_euint8(this.instances3.alice.encrypt8(3));
    expect(res).to.equal(252);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (1)', async function () {
    const res = await this.contract3.neg_euint16(this.instances3.alice.encrypt16(1));
    expect(res).to.equal(65535);
  });

  it('test operator "neg" overload (euint16) => euint16 test 2 (2)', async function () {
    const res = await this.contract3.neg_euint16(this.instances3.alice.encrypt16(2));
    expect(res).to.equal(65534);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (241)', async function () {
    const res = await this.contract3.not_euint16(this.instances3.alice.encrypt16(241));
    expect(res).to.equal(65294);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (1)', async function () {
    const res = await this.contract3.neg_euint32(this.instances3.alice.encrypt32(1));
    expect(res).to.equal(4294967295);
  });

  it('test operator "neg" overload (euint32) => euint32 test 2 (2)', async function () {
    const res = await this.contract3.neg_euint32(this.instances3.alice.encrypt32(2));
    expect(res).to.equal(4294967294);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (65534)', async function () {
    const res = await this.contract3.not_euint32(this.instances3.alice.encrypt32(65534));
    expect(res).to.equal(4294901761);
  });

  it('test operator "bin_op_add" overload (euint8, euint8) => euint8 test 1 (3, 4)', async function () {
    const res = await this.contract3.bin_op_add_euint8_euint8(
      this.instances3.alice.encrypt8(3),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(7);
  });

  it('test operator "bin_op_sub" overload (euint8, euint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract3.bin_op_sub_euint8_euint8(
      this.instances3.alice.encrypt8(4),
      this.instances3.alice.encrypt8(3),
    );
    expect(res).to.equal(1);
  });

  it('test operator "bin_op_mul" overload (euint8, euint8) => euint8 test 1 (4, 3)', async function () {
    const res = await this.contract3.bin_op_mul_euint8_euint8(
      this.instances3.alice.encrypt8(4),
      this.instances3.alice.encrypt8(3),
    );
    expect(res).to.equal(12);
  });

  it('test operator "bin_op_and" overload (euint8, euint8) => euint8 test 1 (239, 240)', async function () {
    const res = await this.contract3.bin_op_and_euint8_euint8(
      this.instances3.alice.encrypt8(239),
      this.instances3.alice.encrypt8(240),
    );
    expect(res).to.equal(224);
  });

  it('test operator "bin_op_or" overload (euint8, euint8) => euint8 test 1 (239, 240)', async function () {
    const res = await this.contract3.bin_op_or_euint8_euint8(
      this.instances3.alice.encrypt8(239),
      this.instances3.alice.encrypt8(240),
    );
    expect(res).to.equal(255);
  });

  it('test operator "bin_op_xor" overload (euint8, euint8) => euint8 test 1 (239, 240)', async function () {
    const res = await this.contract3.bin_op_xor_euint8_euint8(
      this.instances3.alice.encrypt8(239),
      this.instances3.alice.encrypt8(240),
    );
    expect(res).to.equal(31);
  });

  it('test operator "unary_op_neg" overload (euint8) => euint8 test 1 (2)', async function () {
    const res = await this.contract3.unary_op_neg_euint8(this.instances3.alice.encrypt8(2));
    expect(res).to.equal(254);
  });

  it('test operator "unary_op_not" overload (euint8) => euint8 test 1 (15)', async function () {
    const res = await this.contract3.unary_op_not_euint8(this.instances3.alice.encrypt8(15));
    expect(res).to.equal(240);
  });

  it('test operator "bin_op_add" overload (euint16, euint16) => euint16 test 1 (259, 516)', async function () {
    const res = await this.contract3.bin_op_add_euint16_euint16(
      this.instances3.alice.encrypt16(259),
      this.instances3.alice.encrypt16(516),
    );
    expect(res).to.equal(775);
  });

  it('test operator "bin_op_sub" overload (euint16, euint16) => euint16 test 1 (516, 259)', async function () {
    const res = await this.contract3.bin_op_sub_euint16_euint16(
      this.instances3.alice.encrypt16(516),
      this.instances3.alice.encrypt16(259),
    );
    expect(res).to.equal(257);
  });

  it('test operator "bin_op_mul" overload (euint16, euint16) => euint16 test 1 (260, 3)', async function () {
    const res = await this.contract3.bin_op_mul_euint16_euint16(
      this.instances3.alice.encrypt16(260),
      this.instances3.alice.encrypt16(3),
    );
    expect(res).to.equal(780);
  });

  it('test operator "bin_op_and" overload (euint16, euint16) => euint16 test 1 (61423, 61680)', async function () {
    const res = await this.contract3.bin_op_and_euint16_euint16(
      this.instances3.alice.encrypt16(61423),
      this.instances3.alice.encrypt16(61680),
    );
    expect(res).to.equal(57568);
  });

  it('test operator "bin_op_or" overload (euint16, euint16) => euint16 test 1 (61423, 496)', async function () {
    const res = await this.contract3.bin_op_or_euint16_euint16(
      this.instances3.alice.encrypt16(61423),
      this.instances3.alice.encrypt16(496),
    );
    expect(res).to.equal(61439);
  });

  it('test operator "bin_op_xor" overload (euint16, euint16) => euint16 test 1 (61423, 61680)', async function () {
    const res = await this.contract3.bin_op_xor_euint16_euint16(
      this.instances3.alice.encrypt16(61423),
      this.instances3.alice.encrypt16(61680),
    );
    expect(res).to.equal(7967);
  });

  it('test operator "unary_op_neg" overload (euint16) => euint16 test 1 (3)', async function () {
    const res = await this.contract3.unary_op_neg_euint16(this.instances3.alice.encrypt16(3));
    expect(res).to.equal(65533);
  });

  it('test operator "unary_op_not" overload (euint16) => euint16 test 1 (3855)', async function () {
    const res = await this.contract3.unary_op_not_euint16(this.instances3.alice.encrypt16(3855));
    expect(res).to.equal(61680);
  });

  it('test operator "bin_op_add" overload (euint32, euint32) => euint32 test 1 (1048835, 4194820)', async function () {
    const res = await this.contract3.bin_op_add_euint32_euint32(
      this.instances3.alice.encrypt32(1048835),
      this.instances3.alice.encrypt32(4194820),
    );
    expect(res).to.equal(5243655);
  });

  it('test operator "bin_op_sub" overload (euint32, euint32) => euint32 test 1 (2415919620, 1342177539)', async function () {
    const res = await this.contract3.bin_op_sub_euint32_euint32(
      this.instances3.alice.encrypt32(2415919620),
      this.instances3.alice.encrypt32(1342177539),
    );
    expect(res).to.equal(1073742081);
  });

  it('test operator "bin_op_mul" overload (euint32, euint32) => euint32 test 1 (33554692, 3)', async function () {
    const res = await this.contract3.bin_op_mul_euint32_euint32(
      this.instances3.alice.encrypt32(33554692),
      this.instances3.alice.encrypt32(3),
    );
    expect(res).to.equal(100664076);
  });

  it('test operator "bin_op_and" overload (euint32, euint32) => euint32 test 1 (4025479151, 4042322160)', async function () {
    const res = await this.contract3.bin_op_and_euint32_euint32(
      this.instances3.alice.encrypt32(4025479151),
      this.instances3.alice.encrypt32(4042322160),
    );
    expect(res).to.equal(3772834016);
  });

  it('test operator "bin_op_or" overload (euint32, euint32) => euint32 test 1 (4025479151, 32506352)', async function () {
    const res = await this.contract3.bin_op_or_euint32_euint32(
      this.instances3.alice.encrypt32(4025479151),
      this.instances3.alice.encrypt32(32506352),
    );
    expect(res).to.equal(4026527743);
  });

  it('test operator "bin_op_xor" overload (euint32, euint32) => euint32 test 1 (4025479151, 4042322160)', async function () {
    const res = await this.contract3.bin_op_xor_euint32_euint32(
      this.instances3.alice.encrypt32(4025479151),
      this.instances3.alice.encrypt32(4042322160),
    );
    expect(res).to.equal(522133279);
  });

  it('test operator "unary_op_neg" overload (euint32) => euint32 test 1 (4)', async function () {
    const res = await this.contract3.unary_op_neg_euint32(this.instances3.alice.encrypt32(4));
    expect(res).to.equal(4294967292);
  });

  it('test operator "unary_op_not" overload (euint32) => euint32 test 1 (252645135)', async function () {
    const res = await this.contract3.unary_op_not_euint32(this.instances3.alice.encrypt32(252645135));
    expect(res).to.equal(4042322160);
  });
});
