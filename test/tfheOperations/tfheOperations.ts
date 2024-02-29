import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHETestSuite1 } from '../../types/contracts/tests/TFHETestSuite1';
import type { TFHETestSuite2 } from '../../types/contracts/tests/TFHETestSuite2';
import type { TFHETestSuite3 } from '../../types/contracts/tests/TFHETestSuite3';
import type { TFHETestSuite4 } from '../../types/contracts/tests/TFHETestSuite4';
import type { TFHETestSuite5 } from '../../types/contracts/tests/TFHETestSuite5';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

async function deployTfheTestFixture1(): Promise<TFHETestSuite1> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture2(): Promise<TFHETestSuite2> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture3(): Promise<TFHETestSuite3> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture4(): Promise<TFHETestSuite4> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture5(): Promise<TFHETestSuite5> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('TFHE operations', function () {
  before(async function () {
    await initSigners(1);
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

    const contract4 = await deployTfheTestFixture4();
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;
    const instances4 = await createInstances(this.contract4Address, ethers, this.signers);
    this.instances4 = instances4;

    const contract5 = await deployTfheTestFixture5();
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;
    const instances5 = await createInstances(this.contract5Address, ethers, this.signers);
    this.instances5 = instances5;
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 1 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(10);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 2 (3, 5)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(10);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 4 (5, 3)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(8);
  });

  it('test operator "sub" overload (euint4, euint4) => euint4 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint4) => euint4 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 1 (2, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(15);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(15);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 1 (13, 13)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt4(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 2 (9, 13)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt4(13),
    );
    expect(res).to.equal(9);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 3 (13, 13)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt4(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 4 (13, 9)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(9);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 1 (13, 13)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt4(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 2 (9, 13)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt4(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 3 (13, 13)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt4(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 4 (13, 9)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(13);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 1 (10, 6)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt4(6),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 1 (10, 1)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 1 (1, 3)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 1 (2, 13)', async function () {
    const res = await this.contract1.ge_euint4_euint4(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt4(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 1 (7, 6)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt4(6),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 1 (11, 2)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 1 (8, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 1 (14, 6)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt4(6),
    );
    expect(res).to.equal(6);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 1 (13, 2)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 1 (1, 8)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(9);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 2 (3, 5)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt8(5),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt8(5),
    );
    expect(res).to.equal(10);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 4 (5, 3)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt8(3),
    );
    expect(res).to.equal(8);
  });

  it('test operator "sub" overload (euint4, euint8) => euint8 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint8) => euint8 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 1 (1, 6)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(6),
    );
    expect(res).to.equal(6);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt8(5),
    );
    expect(res).to.equal(15);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt8(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt8(3),
    );
    expect(res).to.equal(15);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 1 (13, 163)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt8(163),
    );
    expect(res).to.equal(1);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 2 (9, 13)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt8(13),
    );
    expect(res).to.equal(9);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 3 (13, 13)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt8(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 4 (13, 9)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt8(9),
    );
    expect(res).to.equal(9);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 1 (13, 189)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt8(189),
    );
    expect(res).to.equal(189);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 2 (9, 13)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt8(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 3 (13, 13)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt8(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 4 (13, 9)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt8(9),
    );
    expect(res).to.equal(13);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 1 (10, 102)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(102),
    );
    expect(res).to.equal(108);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 2 (6, 10)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 3 (10, 10)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 4 (10, 6)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(6),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 1 (10, 106)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(106),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(6),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 1 (1, 141)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(141),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 1 (2, 171)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt8(171),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 1 (7, 6)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt8(6),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 1 (11, 100)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(100),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt8(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 1 (8, 52)', async function () {
    const res = await this.contract1.lt_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(52),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 1 (14, 175)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(175),
    );
    expect(res).to.equal(14);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 2 (10, 14)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(14),
    );
    expect(res).to.equal(10);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 3 (14, 14)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 4 (14, 10)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(10);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 1 (13, 155)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt8(155),
    );
    expect(res).to.equal(155);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 2 (9, 13)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt8(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 3 (13, 13)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt8(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 4 (13, 9)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt8(9),
    );
    expect(res).to.equal(13);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 1 (1, 10)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(10),
    );
    expect(res).to.equal(11);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 2 (3, 5)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt16(5),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt16(5),
    );
    expect(res).to.equal(10);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 4 (5, 3)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt16(3),
    );
    expect(res).to.equal(8);
  });

  it('test operator "sub" overload (euint4, euint16) => euint16 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint16) => euint16 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 1 (1, 13)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt16(5),
    );
    expect(res).to.equal(15);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt16(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt16(3),
    );
    expect(res).to.equal(15);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 1 (13, 55998)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(55998),
    );
    expect(res).to.equal(12);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 2 (9, 13)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(13),
    );
    expect(res).to.equal(9);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 3 (13, 13)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 4 (13, 9)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(9),
    );
    expect(res).to.equal(9);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 1 (13, 42674)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(42674),
    );
    expect(res).to.equal(42687);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 2 (9, 13)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 3 (13, 13)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 4 (13, 9)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(9),
    );
    expect(res).to.equal(13);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 1 (10, 28735)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(28735),
    );
    expect(res).to.equal(28725);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 2 (6, 10)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt16(10),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 3 (10, 10)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(10),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 4 (10, 6)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(6),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 1 (10, 50800)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(50800),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt16(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(6),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 1 (1, 28941)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(28941),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 1 (2, 31122)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt16(31122),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 1 (7, 37404)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt16(37404),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 1 (11, 61718)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt16(61718),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt16(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt16(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt16(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 1 (8, 46388)', async function () {
    const res = await this.contract1.lt_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(46388),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 1 (14, 38985)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt16(38985),
    );
    expect(res).to.equal(14);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 2 (10, 14)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(14),
    );
    expect(res).to.equal(10);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 3 (14, 14)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt16(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 4 (14, 10)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt16(10),
    );
    expect(res).to.equal(10);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 1 (13, 4074)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(4074),
    );
    expect(res).to.equal(4074);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 2 (9, 13)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 3 (13, 13)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 4 (13, 9)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(9),
    );
    expect(res).to.equal(13);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 1 (1, 14)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(14),
    );
    expect(res).to.equal(15);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 2 (3, 5)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt32(5),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt32(5),
    );
    expect(res).to.equal(10);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 4 (5, 3)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt32(3),
    );
    expect(res).to.equal(8);
  });

  it('test operator "sub" overload (euint4, euint32) => euint32 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint32) => euint32 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 1 (1, 15)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(15),
    );
    expect(res).to.equal(15);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt32(5),
    );
    expect(res).to.equal(15);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt32(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt32(3),
    );
    expect(res).to.equal(15);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 1 (13, 34206859)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(34206859),
    );
    expect(res).to.equal(9);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 2 (9, 13)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(13),
    );
    expect(res).to.equal(9);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 3 (13, 13)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 4 (13, 9)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(9),
    );
    expect(res).to.equal(9);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 1 (13, 65951556)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(65951556),
    );
    expect(res).to.equal(65951565);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 2 (9, 13)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 3 (13, 13)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 4 (13, 9)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(9),
    );
    expect(res).to.equal(13);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 1 (10, 89478471)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(89478471),
    );
    expect(res).to.equal(89478477);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 2 (6, 10)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt32(10),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 3 (10, 10)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(10),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 4 (10, 6)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(6),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 1 (10, 253749712)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(253749712),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt32(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(6),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 1 (1, 54078590)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(54078590),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 1 (2, 94282146)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt32(94282146),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 1 (7, 78307322)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt32(78307322),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 1 (11, 204607742)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt32(204607742),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt32(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt32(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt32(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 1 (8, 223282425)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(223282425),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 1 (14, 38819587)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt32(38819587),
    );
    expect(res).to.equal(14);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 2 (10, 14)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(14),
    );
    expect(res).to.equal(10);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 3 (14, 14)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt32(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 4 (14, 10)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt32(10),
    );
    expect(res).to.equal(10);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 1 (13, 183893221)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(183893221),
    );
    expect(res).to.equal(183893221);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 2 (9, 13)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 3 (13, 13)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 4 (13, 9)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(9),
    );
    expect(res).to.equal(13);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 1 (1, 7)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(7),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 2 (3, 5)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt64(5),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt64(5),
    );
    expect(res).to.equal(10);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 4 (5, 3)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt64(3),
    );
    expect(res).to.equal(8);
  });

  it('test operator "sub" overload (euint4, euint64) => euint64 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint64) => euint64 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 1 (1, 13)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt64(5),
    );
    expect(res).to.equal(15);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt64(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt64(3),
    );
    expect(res).to.equal(15);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 1 (13, 94000820)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt64(94000820),
    );
    expect(res).to.equal(4);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 2 (9, 13)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt64(13),
    );
    expect(res).to.equal(9);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 3 (13, 13)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt64(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 4 (13, 9)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt64(9),
    );
    expect(res).to.equal(9);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 1 (13, 265617259)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt64(265617259),
    );
    expect(res).to.equal(265617263);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 2 (9, 13)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt64(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 3 (13, 13)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt64(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 4 (13, 9)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt64(9),
    );
    expect(res).to.equal(13);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 1 (10, 241197687)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(241197687),
    );
    expect(res).to.equal(241197693);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 2 (6, 10)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 3 (10, 10)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 4 (10, 6)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(6),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 1 (10, 97288685)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(97288685),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(6),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 1 (1, 146059192)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(146059192),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 1 (2, 125028010)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt64(125028010),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 1 (7, 25447544)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt64(25447544),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 1 (11, 54749600)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt64(54749600),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt64(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt64(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt64(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 1 (8, 103658240)', async function () {
    const res = await this.contract1.lt_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(103658240),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 1 (14, 249375209)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt64(249375209),
    );
    expect(res).to.equal(14);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 2 (10, 14)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(14),
    );
    expect(res).to.equal(10);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 3 (14, 14)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt64(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 4 (14, 10)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(10);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 1 (13, 66326451)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt64(66326451),
    );
    expect(res).to.equal(66326451);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 2 (9, 13)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt64(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 3 (13, 13)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt64(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 4 (13, 9)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt64(9),
    );
    expect(res).to.equal(13);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 1 (5, 3)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(5), 3);
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 2 (3, 5)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(3), 5);
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(5), 5);
    expect(res).to.equal(10);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 4 (5, 3)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(5), 3);
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 1 (6, 5)', async function () {
    const res = await this.contract1.add_uint8_euint4(6, this.instances1.alice.encrypt4(5));
    expect(res).to.equal(11);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 2 (3, 5)', async function () {
    const res = await this.contract1.add_uint8_euint4(3, this.instances1.alice.encrypt4(5));
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_uint8_euint4(5, this.instances1.alice.encrypt4(5));
    expect(res).to.equal(10);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 4 (5, 3)', async function () {
    const res = await this.contract1.add_uint8_euint4(5, this.instances1.alice.encrypt4(3));
    expect(res).to.equal(8);
  });

  it('test operator "sub" overload (euint4, uint8) => euint4 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, uint8) => euint4 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(4);
  });

  it('test operator "sub" overload (uint8, euint4) => euint4 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_uint8_euint4(8, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (uint8, euint4) => euint4 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 1 (10, 1)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(10), 1);
    expect(res).to.equal(10);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(3), 5);
    expect(res).to.equal(15);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(2), 2);
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(5), 3);
    expect(res).to.equal(15);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 1 (2, 2)', async function () {
    const res = await this.contract1.mul_uint8_euint4(2, this.instances1.alice.encrypt4(2));
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 2 (2, 4)', async function () {
    const res = await this.contract1.mul_uint8_euint4(2, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(8);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_uint8_euint4(2, this.instances1.alice.encrypt4(2));
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 4 (4, 2)', async function () {
    const res = await this.contract1.mul_uint8_euint4(4, this.instances1.alice.encrypt4(2));
    expect(res).to.equal(8);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 1 (14, 2)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(14), 2);
    expect(res).to.equal(7);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 2 (10, 14)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(10), 14);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 3 (14, 14)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(14), 14);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 4 (14, 10)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(14), 10);
    expect(res).to.equal(1);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 1 (3, 5)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(3), 5);
    expect(res).to.equal(3);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(4);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(0);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 1 (10, 10)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(10), 10);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(6), 10);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(10), 10);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(10), 6);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 1 (8, 1)', async function () {
    const res = await this.contract1.eq_uint8_euint4(8, this.instances1.alice.encrypt4(1));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_uint8_euint4(4, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_uint8_euint4(8, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 1 (1, 10)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(1), 10);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 1 (7, 13)', async function () {
    const res = await this.contract1.ne_uint8_euint4(7, this.instances1.alice.encrypt4(13));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract1.ne_uint8_euint4(9, this.instances1.alice.encrypt4(13));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract1.ne_uint8_euint4(13, this.instances1.alice.encrypt4(13));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract1.ne_uint8_euint4(13, this.instances1.alice.encrypt4(9));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 1 (2, 3)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(2), 3);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 1 (2, 1)', async function () {
    const res = await this.contract1.ge_uint8_euint4(2, this.instances1.alice.encrypt4(1));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_uint8_euint4(4, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_uint8_euint4(8, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 1 (7, 1)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(7), 1);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 1 (3, 13)', async function () {
    const res = await this.contract1.gt_uint8_euint4(3, this.instances1.alice.encrypt4(13));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract1.gt_uint8_euint4(9, this.instances1.alice.encrypt4(13));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract1.gt_uint8_euint4(13, this.instances1.alice.encrypt4(13));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract1.gt_uint8_euint4(13, this.instances1.alice.encrypt4(9));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 1 (11, 11)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(11), 11);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(7), 11);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(11), 11);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(11), 7);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 1 (7, 14)', async function () {
    const res = await this.contract1.le_uint8_euint4(7, this.instances1.alice.encrypt4(14));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.le_uint8_euint4(10, this.instances1.alice.encrypt4(14));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.le_uint8_euint4(14, this.instances1.alice.encrypt4(14));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.le_uint8_euint4(14, this.instances1.alice.encrypt4(10));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 1 (8, 10)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(8), 10);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 1 (5, 1)', async function () {
    const res = await this.contract1.lt_uint8_euint4(5, this.instances1.alice.encrypt4(1));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_uint8_euint4(4, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_uint8_euint4(8, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 1 (14, 9)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(14), 9);
    expect(res).to.equal(9);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 2 (10, 14)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(10), 14);
    expect(res).to.equal(10);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 3 (14, 14)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(14), 14);
    expect(res).to.equal(14);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 4 (14, 10)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(14), 10);
    expect(res).to.equal(10);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 1 (5, 8)', async function () {
    const res = await this.contract1.min_uint8_euint4(5, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(5);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_uint8_euint4(4, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_uint8_euint4(8, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 1 (13, 8)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(13), 8);
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 2 (9, 13)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(9), 13);
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 3 (13, 13)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(13), 13);
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 4 (13, 9)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(13), 9);
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 1 (6, 11)', async function () {
    const res = await this.contract1.max_uint8_euint4(6, this.instances1.alice.encrypt4(11));
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 2 (7, 11)', async function () {
    const res = await this.contract1.max_uint8_euint4(7, this.instances1.alice.encrypt4(11));
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 3 (11, 11)', async function () {
    const res = await this.contract1.max_uint8_euint4(11, this.instances1.alice.encrypt4(11));
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 4 (11, 7)', async function () {
    const res = await this.contract1.max_uint8_euint4(11, this.instances1.alice.encrypt4(7));
    expect(res).to.equal(11);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 1 (11, 1)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(11),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 2 (3, 5)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(5),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(10);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 4 (5, 3)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(5),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(8);
  });

  it('test operator "sub" overload (euint8, euint4) => euint8 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint8, euint4) => euint8 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 1 (5, 1)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(5),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(5);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 2 (2, 4)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(2),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(2),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 4 (4, 2)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(4),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 1 (79, 9)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(79),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(9);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 2 (5, 9)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(5),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(1);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 3 (9, 9)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(9),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(9);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 4 (9, 5)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(9),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(1);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 1 (120, 12)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(120),
      this.instances1.alice.encrypt4(12),
    );
    expect(res).to.equal(124);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 2 (8, 12)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(12),
    );
    expect(res).to.equal(12);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 3 (12, 12)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(12),
      this.instances1.alice.encrypt4(12),
    );
    expect(res).to.equal(12);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 4 (12, 8)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(12),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 1 (44, 8)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(44),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(36);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 1 (129, 1)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(129),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 1 (63, 13)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(63),
      this.instances2.alice.encrypt4(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt4(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(13),
      this.instances2.alice.encrypt4(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(13),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint4) => ebool test 1 (3, 1)', async function () {
    const res = await this.contract2.ge_euint8_euint4(
      this.instances2.alice.encrypt8(3),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ge_euint8_euint4(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ge_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ge_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 1 (211, 13)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(211),
      this.instances2.alice.encrypt4(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt4(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(13),
      this.instances2.alice.encrypt4(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(13),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 1 (157, 14)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(157),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(10),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 1 (209, 1)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(209),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 1 (69, 8)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(69),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 1 (39, 11)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(39),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(39);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 2 (7, 11)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(7),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 3 (11, 11)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 4 (11, 7)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt4(7),
    );
    expect(res).to.equal(11);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 1 (13, 130)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(13),
      this.instances2.alice.encrypt8(130),
    );
    expect(res).to.equal(143);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 2 (9, 13)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt8(13),
    );
    expect(res).to.equal(22);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 3 (13, 13)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(13),
      this.instances2.alice.encrypt8(13),
    );
    expect(res).to.equal(26);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 4 (13, 9)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(13),
      this.instances2.alice.encrypt8(9),
    );
    expect(res).to.equal(22);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 1 (8, 8)', async function () {
    const res = await this.contract2.sub_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 2 (8, 4)', async function () {
    const res = await this.contract2.sub_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 1 (2, 98)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt8(98),
    );
    expect(res).to.equal(196);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(32);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(64);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(32);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 1 (79, 88)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt8(88),
    );
    expect(res).to.equal(72);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 2 (75, 79)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(75),
      this.instances2.alice.encrypt8(79),
    );
    expect(res).to.equal(75);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 3 (79, 79)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt8(79),
    );
    expect(res).to.equal(79);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 4 (79, 75)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt8(75),
    );
    expect(res).to.equal(75);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 1 (120, 101)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(120),
      this.instances2.alice.encrypt8(101),
    );
    expect(res).to.equal(125);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 2 (97, 101)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(97),
      this.instances2.alice.encrypt8(101),
    );
    expect(res).to.equal(101);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 3 (101, 101)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(101),
      this.instances2.alice.encrypt8(101),
    );
    expect(res).to.equal(101);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 4 (101, 97)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(101),
      this.instances2.alice.encrypt8(97),
    );
    expect(res).to.equal(101);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 1 (44, 19)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt8(19),
    );
    expect(res).to.equal(63);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 2 (15, 19)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(15),
      this.instances2.alice.encrypt8(19),
    );
    expect(res).to.equal(28);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 3 (19, 19)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(19),
      this.instances2.alice.encrypt8(19),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 4 (19, 15)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(19),
      this.instances2.alice.encrypt8(15),
    );
    expect(res).to.equal(28);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 1 (8, 206)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(206),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 1 (7, 41)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(7),
      this.instances2.alice.encrypt8(41),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 1 (2, 203)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt8(203),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 1 (3, 44)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(3),
      this.instances2.alice.encrypt8(44),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 1 (7, 208)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(7),
      this.instances2.alice.encrypt8(208),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 1 (5, 181)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt8(181),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 1 (5, 106)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt8(106),
    );
    expect(res).to.equal(5);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 1 (6, 151)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(6),
      this.instances2.alice.encrypt8(151),
    );
    expect(res).to.equal(151);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 1 (1, 237)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(237),
    );
    expect(res).to.equal(238);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 2 (100, 104)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(100),
      this.instances2.alice.encrypt16(104),
    );
    expect(res).to.equal(204);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 3 (104, 104)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(104),
      this.instances2.alice.encrypt16(104),
    );
    expect(res).to.equal(208);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 4 (104, 100)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(104),
      this.instances2.alice.encrypt16(100),
    );
    expect(res).to.equal(204);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 1 (145, 145)', async function () {
    const res = await this.contract2.sub_euint8_euint16(
      this.instances2.alice.encrypt8(145),
      this.instances2.alice.encrypt16(145),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 2 (145, 141)', async function () {
    const res = await this.contract2.sub_euint8_euint16(
      this.instances2.alice.encrypt8(145),
      this.instances2.alice.encrypt16(141),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 1 (1, 161)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(161),
    );
    expect(res).to.equal(161);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 2 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(64);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(64);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 4 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(64);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 1 (79, 63031)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt16(63031),
    );
    expect(res).to.equal(7);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 2 (75, 79)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(75),
      this.instances2.alice.encrypt16(79),
    );
    expect(res).to.equal(75);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 3 (79, 79)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt16(79),
    );
    expect(res).to.equal(79);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 4 (79, 75)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt16(75),
    );
    expect(res).to.equal(75);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 1 (120, 47218)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(120),
      this.instances2.alice.encrypt16(47218),
    );
    expect(res).to.equal(47226);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 2 (116, 120)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(116),
      this.instances2.alice.encrypt16(120),
    );
    expect(res).to.equal(124);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 3 (120, 120)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(120),
      this.instances2.alice.encrypt16(120),
    );
    expect(res).to.equal(120);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 4 (120, 116)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(120),
      this.instances2.alice.encrypt16(116),
    );
    expect(res).to.equal(124);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 1 (44, 50547)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt16(50547),
    );
    expect(res).to.equal(50527);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 2 (40, 44)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(40),
      this.instances2.alice.encrypt16(44),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 3 (44, 44)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt16(44),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 4 (44, 40)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt16(40),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 1 (92, 49874)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt16(49874),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 2 (88, 92)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(88),
      this.instances2.alice.encrypt16(92),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 3 (92, 92)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt16(92),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 4 (92, 88)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt16(88),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 1 (147, 20222)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(147),
      this.instances2.alice.encrypt16(20222),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 2 (143, 147)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(143),
      this.instances2.alice.encrypt16(147),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 3 (147, 147)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(147),
      this.instances2.alice.encrypt16(147),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 4 (147, 143)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(147),
      this.instances2.alice.encrypt16(143),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 1 (254, 25447)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt16(25447),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 2 (250, 254)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt16(254),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 3 (254, 254)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt16(254),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 4 (254, 250)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt16(250),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 1 (240, 31410)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(240),
      this.instances2.alice.encrypt16(31410),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 2 (236, 240)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(236),
      this.instances2.alice.encrypt16(240),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 3 (240, 240)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(240),
      this.instances2.alice.encrypt16(240),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 4 (240, 236)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(240),
      this.instances2.alice.encrypt16(236),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 1 (44, 23355)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt16(23355),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 2 (40, 44)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(40),
      this.instances2.alice.encrypt16(44),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 3 (44, 44)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt16(44),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 4 (44, 40)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt16(40),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 1 (165, 15128)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(165),
      this.instances2.alice.encrypt16(15128),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 2 (161, 165)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt16(165),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 3 (165, 165)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(165),
      this.instances2.alice.encrypt16(165),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 4 (165, 161)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(165),
      this.instances2.alice.encrypt16(161),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 1 (39, 40958)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(39),
      this.instances2.alice.encrypt16(40958),
    );
    expect(res).to.equal(39);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 2 (35, 39)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(35),
      this.instances2.alice.encrypt16(39),
    );
    expect(res).to.equal(35);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 3 (39, 39)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(39),
      this.instances2.alice.encrypt16(39),
    );
    expect(res).to.equal(39);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 4 (39, 35)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(39),
      this.instances2.alice.encrypt16(35),
    );
    expect(res).to.equal(35);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 1 (250, 47824)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt16(47824),
    );
    expect(res).to.equal(47824);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 2 (246, 250)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(246),
      this.instances2.alice.encrypt16(250),
    );
    expect(res).to.equal(250);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 3 (250, 250)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt16(250),
    );
    expect(res).to.equal(250);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 4 (250, 246)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt16(246),
    );
    expect(res).to.equal(250);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 1 (1, 147)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(147),
    );
    expect(res).to.equal(148);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 2 (100, 104)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(100),
      this.instances2.alice.encrypt32(104),
    );
    expect(res).to.equal(204);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 3 (104, 104)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(104),
      this.instances2.alice.encrypt32(104),
    );
    expect(res).to.equal(208);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 4 (104, 100)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(104),
      this.instances2.alice.encrypt32(100),
    );
    expect(res).to.equal(204);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 1 (145, 145)', async function () {
    const res = await this.contract2.sub_euint8_euint32(
      this.instances2.alice.encrypt8(145),
      this.instances2.alice.encrypt32(145),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 2 (145, 141)', async function () {
    const res = await this.contract2.sub_euint8_euint32(
      this.instances2.alice.encrypt8(145),
      this.instances2.alice.encrypt32(141),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 1 (1, 174)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(174),
    );
    expect(res).to.equal(174);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 2 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(64);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(64);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 4 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(64);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 1 (79, 262235113)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt32(262235113),
    );
    expect(res).to.equal(73);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 2 (75, 79)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(75),
      this.instances2.alice.encrypt32(79),
    );
    expect(res).to.equal(75);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 3 (79, 79)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt32(79),
    );
    expect(res).to.equal(79);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 4 (79, 75)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt32(75),
    );
    expect(res).to.equal(75);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 1 (120, 230647135)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(120),
      this.instances2.alice.encrypt32(230647135),
    );
    expect(res).to.equal(230647167);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 2 (116, 120)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(116),
      this.instances2.alice.encrypt32(120),
    );
    expect(res).to.equal(124);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 3 (120, 120)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(120),
      this.instances2.alice.encrypt32(120),
    );
    expect(res).to.equal(120);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 4 (120, 116)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(120),
      this.instances2.alice.encrypt32(116),
    );
    expect(res).to.equal(124);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 1 (44, 19360952)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt32(19360952),
    );
    expect(res).to.equal(19360916);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 2 (40, 44)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(40),
      this.instances2.alice.encrypt32(44),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 3 (44, 44)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt32(44),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 4 (44, 40)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt32(40),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 1 (92, 74149594)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt32(74149594),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 2 (88, 92)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(88),
      this.instances2.alice.encrypt32(92),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 3 (92, 92)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt32(92),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 4 (92, 88)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt32(88),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 1 (147, 134200275)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(147),
      this.instances2.alice.encrypt32(134200275),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 2 (143, 147)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(143),
      this.instances2.alice.encrypt32(147),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 3 (147, 147)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(147),
      this.instances2.alice.encrypt32(147),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 4 (147, 143)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(147),
      this.instances2.alice.encrypt32(143),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 1 (254, 174513350)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt32(174513350),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 2 (250, 254)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt32(254),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 3 (254, 254)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt32(254),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 4 (254, 250)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt32(250),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 1 (240, 230709034)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(240),
      this.instances2.alice.encrypt32(230709034),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 2 (236, 240)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(236),
      this.instances2.alice.encrypt32(240),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 3 (240, 240)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(240),
      this.instances2.alice.encrypt32(240),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 4 (240, 236)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(240),
      this.instances2.alice.encrypt32(236),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 1 (44, 91863139)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt32(91863139),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 2 (40, 44)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(40),
      this.instances2.alice.encrypt32(44),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 3 (44, 44)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt32(44),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 4 (44, 40)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt32(40),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 1 (165, 242355822)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(165),
      this.instances2.alice.encrypt32(242355822),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 2 (161, 165)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt32(165),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 3 (165, 165)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(165),
      this.instances2.alice.encrypt32(165),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 4 (165, 161)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(165),
      this.instances2.alice.encrypt32(161),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 1 (39, 262725927)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(39),
      this.instances2.alice.encrypt32(262725927),
    );
    expect(res).to.equal(39);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 2 (35, 39)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(35),
      this.instances2.alice.encrypt32(39),
    );
    expect(res).to.equal(35);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 3 (39, 39)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(39),
      this.instances2.alice.encrypt32(39),
    );
    expect(res).to.equal(39);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 4 (39, 35)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(39),
      this.instances2.alice.encrypt32(35),
    );
    expect(res).to.equal(35);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 1 (250, 157022809)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt32(157022809),
    );
    expect(res).to.equal(157022809);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 2 (246, 250)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(246),
      this.instances2.alice.encrypt32(250),
    );
    expect(res).to.equal(250);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 3 (250, 250)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt32(250),
    );
    expect(res).to.equal(250);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 4 (250, 246)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt32(246),
    );
    expect(res).to.equal(250);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 1 (1, 159)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(159),
    );
    expect(res).to.equal(160);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 2 (100, 104)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(100),
      this.instances2.alice.encrypt64(104),
    );
    expect(res).to.equal(204);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 3 (104, 104)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(104),
      this.instances2.alice.encrypt64(104),
    );
    expect(res).to.equal(208);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 4 (104, 100)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(104),
      this.instances2.alice.encrypt64(100),
    );
    expect(res).to.equal(204);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 1 (145, 145)', async function () {
    const res = await this.contract2.sub_euint8_euint64(
      this.instances2.alice.encrypt8(145),
      this.instances2.alice.encrypt64(145),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 2 (145, 141)', async function () {
    const res = await this.contract2.sub_euint8_euint64(
      this.instances2.alice.encrypt8(145),
      this.instances2.alice.encrypt64(141),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 1 (1, 154)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(154),
    );
    expect(res).to.equal(154);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 2 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(64);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(64);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 4 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(64);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 1 (79, 112002348)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt64(112002348),
    );
    expect(res).to.equal(12);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 2 (75, 79)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(75),
      this.instances2.alice.encrypt64(79),
    );
    expect(res).to.equal(75);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 3 (79, 79)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt64(79),
    );
    expect(res).to.equal(79);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 4 (79, 75)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt64(75),
    );
    expect(res).to.equal(75);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 1 (120, 225077141)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(120),
      this.instances2.alice.encrypt64(225077141),
    );
    expect(res).to.equal(225077245);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 2 (116, 120)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(116),
      this.instances2.alice.encrypt64(120),
    );
    expect(res).to.equal(124);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 3 (120, 120)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(120),
      this.instances2.alice.encrypt64(120),
    );
    expect(res).to.equal(120);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 4 (120, 116)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(120),
      this.instances2.alice.encrypt64(116),
    );
    expect(res).to.equal(124);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 1 (44, 222033695)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt64(222033695),
    );
    expect(res).to.equal(222033715);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 2 (40, 44)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(40),
      this.instances2.alice.encrypt64(44),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 3 (44, 44)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt64(44),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 4 (44, 40)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt64(40),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 1 (92, 249397548)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt64(249397548),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 2 (88, 92)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(88),
      this.instances2.alice.encrypt64(92),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 3 (92, 92)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt64(92),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 4 (92, 88)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt64(88),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 1 (147, 43277666)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(147),
      this.instances2.alice.encrypt64(43277666),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 2 (143, 147)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(143),
      this.instances2.alice.encrypt64(147),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 3 (147, 147)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(147),
      this.instances2.alice.encrypt64(147),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 4 (147, 143)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(147),
      this.instances2.alice.encrypt64(143),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 1 (254, 249856292)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt64(249856292),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 2 (250, 254)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt64(254),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 3 (254, 254)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt64(254),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 4 (254, 250)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt64(250),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 1 (240, 179585345)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(240),
      this.instances2.alice.encrypt64(179585345),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 2 (236, 240)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(236),
      this.instances2.alice.encrypt64(240),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 3 (240, 240)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(240),
      this.instances2.alice.encrypt64(240),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 4 (240, 236)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(240),
      this.instances2.alice.encrypt64(236),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 1 (44, 264172669)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt64(264172669),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 2 (40, 44)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(40),
      this.instances2.alice.encrypt64(44),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 3 (44, 44)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt64(44),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 4 (44, 40)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(44),
      this.instances2.alice.encrypt64(40),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 1 (165, 87596497)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(165),
      this.instances2.alice.encrypt64(87596497),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 2 (161, 165)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt64(165),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 3 (165, 165)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(165),
      this.instances2.alice.encrypt64(165),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 4 (165, 161)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(165),
      this.instances2.alice.encrypt64(161),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 1 (39, 57110634)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(39),
      this.instances2.alice.encrypt64(57110634),
    );
    expect(res).to.equal(39);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 2 (35, 39)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(35),
      this.instances2.alice.encrypt64(39),
    );
    expect(res).to.equal(35);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 3 (39, 39)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(39),
      this.instances2.alice.encrypt64(39),
    );
    expect(res).to.equal(39);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 4 (39, 35)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(39),
      this.instances2.alice.encrypt64(35),
    );
    expect(res).to.equal(35);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 1 (250, 147804883)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt64(147804883),
    );
    expect(res).to.equal(147804883);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 2 (246, 250)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(246),
      this.instances2.alice.encrypt64(250),
    );
    expect(res).to.equal(250);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 3 (250, 250)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt64(250),
    );
    expect(res).to.equal(250);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 4 (250, 246)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt64(246),
    );
    expect(res).to.equal(250);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 1 (13, 50)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(13), 50);
    expect(res).to.equal(63);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 2 (9, 13)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(9), 13);
    expect(res).to.equal(22);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 3 (13, 13)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(13), 13);
    expect(res).to.equal(26);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 4 (13, 9)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(13), 9);
    expect(res).to.equal(22);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 1 (104, 50)', async function () {
    const res = await this.contract2.add_uint8_euint8(104, this.instances2.alice.encrypt8(50));
    expect(res).to.equal(154);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 2 (9, 13)', async function () {
    const res = await this.contract2.add_uint8_euint8(9, this.instances2.alice.encrypt8(13));
    expect(res).to.equal(22);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 3 (13, 13)', async function () {
    const res = await this.contract2.add_uint8_euint8(13, this.instances2.alice.encrypt8(13));
    expect(res).to.equal(26);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 4 (13, 9)', async function () {
    const res = await this.contract2.add_uint8_euint8(13, this.instances2.alice.encrypt8(9));
    expect(res).to.equal(22);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 1 (8, 8)', async function () {
    const res = await this.contract2.sub_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 2 (8, 4)', async function () {
    const res = await this.contract2.sub_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(4);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 1 (8, 8)', async function () {
    const res = await this.contract2.sub_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 2 (8, 4)', async function () {
    const res = await this.contract2.sub_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 1 (2, 119)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(2), 119);
    expect(res).to.equal(238);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(32);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(64);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(32);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 1 (8, 29)', async function () {
    const res = await this.contract2.mul_uint8_euint8(8, this.instances2.alice.encrypt8(29));
    expect(res).to.equal(232);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.mul_uint8_euint8(4, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(32);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.mul_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(64);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.mul_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(32);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 1 (72, 21)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(72), 21);
    expect(res).to.equal(3);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 2 (68, 72)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(68), 72);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 3 (72, 72)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(72), 72);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 4 (72, 68)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(72), 68);
    expect(res).to.equal(1);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 1 (97, 233)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(97), 233);
    expect(res).to.equal(97);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 2 (93, 97)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(93), 97);
    expect(res).to.equal(93);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 3 (97, 97)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(97), 97);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 4 (97, 93)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(97), 93);
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 1 (8, 233)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(8), 233);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 1 (92, 233)', async function () {
    const res = await this.contract2.eq_uint8_euint8(92, this.instances2.alice.encrypt8(233));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.eq_uint8_euint8(4, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.eq_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.eq_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 1 (7, 100)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(7), 100);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 1 (147, 100)', async function () {
    const res = await this.contract2.ne_uint8_euint8(147, this.instances2.alice.encrypt8(100));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ne_uint8_euint8(4, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ne_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ne_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (2, 65)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(2), 65);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 1 (254, 65)', async function () {
    const res = await this.contract2.ge_uint8_euint8(254, this.instances2.alice.encrypt8(65));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ge_uint8_euint8(4, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ge_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ge_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (3, 110)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(3), 110);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 1 (240, 110)', async function () {
    const res = await this.contract2.gt_uint8_euint8(240, this.instances2.alice.encrypt8(110));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.gt_uint8_euint8(4, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.gt_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.gt_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (7, 168)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(7), 168);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 1 (44, 168)', async function () {
    const res = await this.contract2.le_uint8_euint8(44, this.instances2.alice.encrypt8(168));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.le_uint8_euint8(4, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.le_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.le_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 1 (5, 37)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(5), 37);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (165, 37)', async function () {
    const res = await this.contract2.lt_uint8_euint8(165, this.instances2.alice.encrypt8(37));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.lt_uint8_euint8(4, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.lt_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.lt_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (5, 110)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(5), 110);
    expect(res).to.equal(5);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (39, 110)', async function () {
    const res = await this.contract2.min_uint8_euint8(39, this.instances2.alice.encrypt8(110));
    expect(res).to.equal(39);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.min_uint8_euint8(4, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.min_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.min_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (6, 28)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(6), 28);
    expect(res).to.equal(28);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 1 (250, 28)', async function () {
    const res = await this.contract2.max_uint8_euint8(250, this.instances2.alice.encrypt8(28));
    expect(res).to.equal(250);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.max_uint8_euint8(4, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.max_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.max_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 1 (7, 1)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(7),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 3 (4, 4)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "sub" overload (euint16, euint4) => euint16 test 1 (8, 8)', async function () {
    const res = await this.contract2.sub_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, euint4) => euint16 test 2 (8, 4)', async function () {
    const res = await this.contract2.sub_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 1 (8, 1)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(8);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 2 (2, 3)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(2),
      this.instances2.alice.encrypt4(3),
    );
    expect(res).to.equal(6);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 3 (3, 3)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(3),
      this.instances2.alice.encrypt4(3),
    );
    expect(res).to.equal(9);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 4 (3, 2)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(3),
      this.instances2.alice.encrypt4(2),
    );
    expect(res).to.equal(6);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 1 (26290, 1)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(26290),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 1 (61878, 13)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(61878),
      this.instances2.alice.encrypt4(13),
    );
    expect(res).to.equal(61887);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 2 (9, 13)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(9),
      this.instances2.alice.encrypt4(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 3 (13, 13)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(13),
      this.instances2.alice.encrypt4(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 4 (13, 9)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(13),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(13);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 1 (36773, 10)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(36773),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(36783);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 2 (6, 10)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(6),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 3 (10, 10)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(10),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 4 (10, 6)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(10),
      this.instances2.alice.encrypt4(6),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 1 (44634, 3)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(44634),
      this.instances2.alice.encrypt4(3),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 1 (58512, 7)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(58512),
      this.instances2.alice.encrypt4(7),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 1 (2319, 9)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(2319),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(5),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(9),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(9),
      this.instances2.alice.encrypt4(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 1 (39995, 5)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(39995),
      this.instances2.alice.encrypt4(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 1 (11868, 2)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(11868),
      this.instances2.alice.encrypt4(2),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 1 (13005, 11)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(13005),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(7),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(11),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(11),
      this.instances2.alice.encrypt4(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 1 (59594, 13)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(59594),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 2 (9, 13)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(9),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(9);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 3 (13, 13)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(13),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 4 (13, 9)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(13),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(9);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 1 (45745, 1)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(45745),
      this.instances3.alice.encrypt4(1),
    );
    expect(res).to.equal(45745);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 1 (246, 2)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(246),
      this.instances3.alice.encrypt8(2),
    );
    expect(res).to.equal(248);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 2 (90, 92)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(90),
      this.instances3.alice.encrypt8(92),
    );
    expect(res).to.equal(182);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 3 (92, 92)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(92),
      this.instances3.alice.encrypt8(92),
    );
    expect(res).to.equal(184);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 4 (92, 90)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(92),
      this.instances3.alice.encrypt8(90),
    );
    expect(res).to.equal(182);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (167, 167)', async function () {
    const res = await this.contract3.sub_euint16_euint8(
      this.instances3.alice.encrypt16(167),
      this.instances3.alice.encrypt8(167),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (167, 163)', async function () {
    const res = await this.contract3.sub_euint16_euint8(
      this.instances3.alice.encrypt16(167),
      this.instances3.alice.encrypt8(163),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (136, 1)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(136),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(136);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 2 (13, 15)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(13),
      this.instances3.alice.encrypt8(15),
    );
    expect(res).to.equal(195);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 3 (15, 15)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(15),
      this.instances3.alice.encrypt8(15),
    );
    expect(res).to.equal(225);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 4 (15, 13)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(15),
      this.instances3.alice.encrypt8(13),
    );
    expect(res).to.equal(195);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 1 (26290, 248)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(26290),
      this.instances3.alice.encrypt8(248),
    );
    expect(res).to.equal(176);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 2 (244, 248)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(244),
      this.instances3.alice.encrypt8(248),
    );
    expect(res).to.equal(240);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 3 (248, 248)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(248),
      this.instances3.alice.encrypt8(248),
    );
    expect(res).to.equal(248);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 4 (248, 244)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(248),
      this.instances3.alice.encrypt8(244),
    );
    expect(res).to.equal(240);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 1 (61878, 247)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(61878),
      this.instances3.alice.encrypt8(247),
    );
    expect(res).to.equal(61943);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 2 (243, 247)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(243),
      this.instances3.alice.encrypt8(247),
    );
    expect(res).to.equal(247);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 3 (247, 247)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(247),
      this.instances3.alice.encrypt8(247),
    );
    expect(res).to.equal(247);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 4 (247, 243)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(247),
      this.instances3.alice.encrypt8(243),
    );
    expect(res).to.equal(247);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (36773, 131)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(36773),
      this.instances3.alice.encrypt8(131),
    );
    expect(res).to.equal(36646);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (127, 131)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(127),
      this.instances3.alice.encrypt8(131),
    );
    expect(res).to.equal(252);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 3 (131, 131)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(131),
      this.instances3.alice.encrypt8(131),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 4 (131, 127)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(131),
      this.instances3.alice.encrypt8(127),
    );
    expect(res).to.equal(252);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 1 (44634, 93)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(44634),
      this.instances3.alice.encrypt8(93),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 2 (89, 93)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(89),
      this.instances3.alice.encrypt8(93),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 3 (93, 93)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(93),
      this.instances3.alice.encrypt8(93),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 4 (93, 89)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(93),
      this.instances3.alice.encrypt8(89),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (58512, 116)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(58512),
      this.instances3.alice.encrypt8(116),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (112, 116)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(112),
      this.instances3.alice.encrypt8(116),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 3 (116, 116)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(116),
      this.instances3.alice.encrypt8(116),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 4 (116, 112)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(116),
      this.instances3.alice.encrypt8(112),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (2319, 170)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(2319),
      this.instances3.alice.encrypt8(170),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (166, 170)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(166),
      this.instances3.alice.encrypt8(170),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (170, 170)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(170),
      this.instances3.alice.encrypt8(170),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 4 (170, 166)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(170),
      this.instances3.alice.encrypt8(166),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 1 (39995, 222)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(39995),
      this.instances3.alice.encrypt8(222),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 2 (218, 222)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(218),
      this.instances3.alice.encrypt8(222),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 3 (222, 222)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(222),
      this.instances3.alice.encrypt8(222),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 4 (222, 218)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(222),
      this.instances3.alice.encrypt8(218),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 1 (11868, 206)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(11868),
      this.instances3.alice.encrypt8(206),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 2 (202, 206)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(202),
      this.instances3.alice.encrypt8(206),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 3 (206, 206)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(206),
      this.instances3.alice.encrypt8(206),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 4 (206, 202)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(206),
      this.instances3.alice.encrypt8(202),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 1 (13005, 103)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(13005),
      this.instances3.alice.encrypt8(103),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 2 (99, 103)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(99),
      this.instances3.alice.encrypt8(103),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 3 (103, 103)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(103),
      this.instances3.alice.encrypt8(103),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 4 (103, 99)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(103),
      this.instances3.alice.encrypt8(99),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 1 (59594, 150)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(59594),
      this.instances3.alice.encrypt8(150),
    );
    expect(res).to.equal(150);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 2 (146, 150)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(146),
      this.instances3.alice.encrypt8(150),
    );
    expect(res).to.equal(146);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 3 (150, 150)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(150),
      this.instances3.alice.encrypt8(150),
    );
    expect(res).to.equal(150);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 4 (150, 146)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(150),
      this.instances3.alice.encrypt8(146),
    );
    expect(res).to.equal(146);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 1 (45745, 137)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(45745),
      this.instances3.alice.encrypt8(137),
    );
    expect(res).to.equal(45745);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 2 (133, 137)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(133),
      this.instances3.alice.encrypt8(137),
    );
    expect(res).to.equal(137);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 3 (137, 137)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(137),
      this.instances3.alice.encrypt8(137),
    );
    expect(res).to.equal(137);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 4 (137, 133)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(137),
      this.instances3.alice.encrypt8(133),
    );
    expect(res).to.equal(137);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 1 (15771, 31424)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(15771),
      this.instances3.alice.encrypt16(31424),
    );
    expect(res).to.equal(47195);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 2 (15767, 15771)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(15767),
      this.instances3.alice.encrypt16(15771),
    );
    expect(res).to.equal(31538);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 3 (15771, 15771)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(15771),
      this.instances3.alice.encrypt16(15771),
    );
    expect(res).to.equal(31542);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 4 (15771, 15767)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(15771),
      this.instances3.alice.encrypt16(15767),
    );
    expect(res).to.equal(31538);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 1 (20069, 20069)', async function () {
    const res = await this.contract3.sub_euint16_euint16(
      this.instances3.alice.encrypt16(20069),
      this.instances3.alice.encrypt16(20069),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 2 (20069, 20065)', async function () {
    const res = await this.contract3.sub_euint16_euint16(
      this.instances3.alice.encrypt16(20069),
      this.instances3.alice.encrypt16(20065),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 1 (68, 599)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(68),
      this.instances3.alice.encrypt16(599),
    );
    expect(res).to.equal(40732);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 2 (136, 136)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(136),
      this.instances3.alice.encrypt16(136),
    );
    expect(res).to.equal(18496);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 3 (136, 136)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(136),
      this.instances3.alice.encrypt16(136),
    );
    expect(res).to.equal(18496);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 4 (136, 136)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(136),
      this.instances3.alice.encrypt16(136),
    );
    expect(res).to.equal(18496);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 1 (26290, 23750)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(26290),
      this.instances3.alice.encrypt16(23750),
    );
    expect(res).to.equal(17538);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 2 (23746, 23750)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(23746),
      this.instances3.alice.encrypt16(23750),
    );
    expect(res).to.equal(23746);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 3 (23750, 23750)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(23750),
      this.instances3.alice.encrypt16(23750),
    );
    expect(res).to.equal(23750);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 4 (23750, 23746)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(23750),
      this.instances3.alice.encrypt16(23746),
    );
    expect(res).to.equal(23746);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 1 (61878, 7772)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(61878),
      this.instances3.alice.encrypt16(7772),
    );
    expect(res).to.equal(65534);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 2 (7768, 7772)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(7768),
      this.instances3.alice.encrypt16(7772),
    );
    expect(res).to.equal(7772);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 3 (7772, 7772)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(7772),
      this.instances3.alice.encrypt16(7772),
    );
    expect(res).to.equal(7772);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 4 (7772, 7768)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(7772),
      this.instances3.alice.encrypt16(7768),
    );
    expect(res).to.equal(7772);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (36773, 46302)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(36773),
      this.instances3.alice.encrypt16(46302),
    );
    expect(res).to.equal(15227);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (36769, 36773)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(36769),
      this.instances3.alice.encrypt16(36773),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 3 (36773, 36773)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(36773),
      this.instances3.alice.encrypt16(36773),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 4 (36773, 36769)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(36773),
      this.instances3.alice.encrypt16(36769),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 1 (44634, 41626)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(44634),
      this.instances3.alice.encrypt16(41626),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 2 (41622, 41626)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(41622),
      this.instances3.alice.encrypt16(41626),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 3 (41626, 41626)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(41626),
      this.instances3.alice.encrypt16(41626),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 4 (41626, 41622)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(41626),
      this.instances3.alice.encrypt16(41622),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 1 (58512, 41579)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(58512),
      this.instances3.alice.encrypt16(41579),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 2 (41575, 41579)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(41575),
      this.instances3.alice.encrypt16(41579),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 3 (41579, 41579)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(41579),
      this.instances3.alice.encrypt16(41579),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 4 (41579, 41575)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(41579),
      this.instances3.alice.encrypt16(41575),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 1 (2319, 22034)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(2319),
      this.instances3.alice.encrypt16(22034),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 2 (2315, 2319)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(2315),
      this.instances3.alice.encrypt16(2319),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 3 (2319, 2319)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(2319),
      this.instances3.alice.encrypt16(2319),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 4 (2319, 2315)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(2319),
      this.instances3.alice.encrypt16(2315),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 1 (39995, 43029)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(39995),
      this.instances3.alice.encrypt16(43029),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 2 (39991, 39995)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(39991),
      this.instances3.alice.encrypt16(39995),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 3 (39995, 39995)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(39995),
      this.instances3.alice.encrypt16(39995),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 4 (39995, 39991)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(39995),
      this.instances3.alice.encrypt16(39991),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 1 (11868, 62015)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(11868),
      this.instances3.alice.encrypt16(62015),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 2 (11864, 11868)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(11864),
      this.instances3.alice.encrypt16(11868),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 3 (11868, 11868)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(11868),
      this.instances3.alice.encrypt16(11868),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 4 (11868, 11864)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(11868),
      this.instances3.alice.encrypt16(11864),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 1 (13005, 22857)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(13005),
      this.instances3.alice.encrypt16(22857),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 2 (13001, 13005)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(13001),
      this.instances3.alice.encrypt16(13005),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 3 (13005, 13005)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(13005),
      this.instances3.alice.encrypt16(13005),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 4 (13005, 13001)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(13005),
      this.instances3.alice.encrypt16(13001),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 1 (59594, 35028)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(59594),
      this.instances3.alice.encrypt16(35028),
    );
    expect(res).to.equal(35028);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 2 (35024, 35028)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(35024),
      this.instances3.alice.encrypt16(35028),
    );
    expect(res).to.equal(35024);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 3 (35028, 35028)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(35028),
      this.instances3.alice.encrypt16(35028),
    );
    expect(res).to.equal(35028);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 4 (35028, 35024)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(35028),
      this.instances3.alice.encrypt16(35024),
    );
    expect(res).to.equal(35024);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 1 (45745, 24760)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(45745),
      this.instances3.alice.encrypt16(24760),
    );
    expect(res).to.equal(45745);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 2 (24756, 24760)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(24756),
      this.instances3.alice.encrypt16(24760),
    );
    expect(res).to.equal(24760);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 3 (24760, 24760)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(24760),
      this.instances3.alice.encrypt16(24760),
    );
    expect(res).to.equal(24760);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 4 (24760, 24756)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(24760),
      this.instances3.alice.encrypt16(24756),
    );
    expect(res).to.equal(24760);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 1 (6, 46282)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(6),
      this.instances3.alice.encrypt32(46282),
    );
    expect(res).to.equal(46288);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 2 (27895, 27899)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(27895),
      this.instances3.alice.encrypt32(27899),
    );
    expect(res).to.equal(55794);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 3 (27899, 27899)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(27899),
      this.instances3.alice.encrypt32(27899),
    );
    expect(res).to.equal(55798);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 4 (27899, 27895)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(27899),
      this.instances3.alice.encrypt32(27895),
    );
    expect(res).to.equal(55794);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (63598, 63598)', async function () {
    const res = await this.contract3.sub_euint16_euint32(
      this.instances3.alice.encrypt16(63598),
      this.instances3.alice.encrypt32(63598),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (63598, 63594)', async function () {
    const res = await this.contract3.sub_euint16_euint32(
      this.instances3.alice.encrypt16(63598),
      this.instances3.alice.encrypt32(63594),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (4, 14667)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt32(14667),
    );
    expect(res).to.equal(58668);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 2 (154, 154)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(154),
      this.instances3.alice.encrypt32(154),
    );
    expect(res).to.equal(23716);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 3 (154, 154)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(154),
      this.instances3.alice.encrypt32(154),
    );
    expect(res).to.equal(23716);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 4 (154, 154)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(154),
      this.instances3.alice.encrypt32(154),
    );
    expect(res).to.equal(23716);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (26290, 142339715)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(26290),
      this.instances3.alice.encrypt32(142339715),
    );
    expect(res).to.equal(26242);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (26286, 26290)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(26286),
      this.instances3.alice.encrypt32(26290),
    );
    expect(res).to.equal(26274);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (26290, 26290)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(26290),
      this.instances3.alice.encrypt32(26290),
    );
    expect(res).to.equal(26290);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (26290, 26286)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(26290),
      this.instances3.alice.encrypt32(26286),
    );
    expect(res).to.equal(26274);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (61878, 213802596)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(61878),
      this.instances3.alice.encrypt32(213802596),
    );
    expect(res).to.equal(213843958);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (61874, 61878)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(61874),
      this.instances3.alice.encrypt32(61878),
    );
    expect(res).to.equal(61878);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 3 (61878, 61878)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(61878),
      this.instances3.alice.encrypt32(61878),
    );
    expect(res).to.equal(61878);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 4 (61878, 61874)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(61878),
      this.instances3.alice.encrypt32(61874),
    );
    expect(res).to.equal(61878);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 1 (36773, 153920052)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(36773),
      this.instances3.alice.encrypt32(153920052),
    );
    expect(res).to.equal(153890193);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 2 (36769, 36773)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(36769),
      this.instances3.alice.encrypt32(36773),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 3 (36773, 36773)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(36773),
      this.instances3.alice.encrypt32(36773),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 4 (36773, 36769)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(36773),
      this.instances3.alice.encrypt32(36769),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (58427, 33310393)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(58427),
      this.instances3.alice.encrypt32(33310393),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (58423, 58427)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(58423),
      this.instances3.alice.encrypt32(58427),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 3 (58427, 58427)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(58427),
      this.instances3.alice.encrypt32(58427),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 4 (58427, 58423)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(58427),
      this.instances3.alice.encrypt32(58423),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 1 (22739, 149626103)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(22739),
      this.instances3.alice.encrypt32(149626103),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 2 (22735, 22739)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(22735),
      this.instances3.alice.encrypt32(22739),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 3 (22739, 22739)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(22739),
      this.instances3.alice.encrypt32(22739),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 4 (22739, 22735)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(22739),
      this.instances3.alice.encrypt32(22735),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (15004, 215912519)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(15004),
      this.instances3.alice.encrypt32(215912519),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (15000, 15004)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(15000),
      this.instances3.alice.encrypt32(15004),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (15004, 15004)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(15004),
      this.instances3.alice.encrypt32(15004),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 4 (15004, 15000)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(15004),
      this.instances3.alice.encrypt32(15000),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 1 (31594, 222562117)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(31594),
      this.instances3.alice.encrypt32(222562117),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 2 (31590, 31594)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(31590),
      this.instances3.alice.encrypt32(31594),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 3 (31594, 31594)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(31594),
      this.instances3.alice.encrypt32(31594),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 4 (31594, 31590)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(31594),
      this.instances3.alice.encrypt32(31590),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (2104, 130166929)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(2104),
      this.instances3.alice.encrypt32(130166929),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (2100, 2104)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(2100),
      this.instances3.alice.encrypt32(2104),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (2104, 2104)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(2104),
      this.instances3.alice.encrypt32(2104),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 4 (2104, 2100)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(2104),
      this.instances3.alice.encrypt32(2100),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 1 (6215, 187516000)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(6215),
      this.instances3.alice.encrypt32(187516000),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 2 (6211, 6215)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(6211),
      this.instances3.alice.encrypt32(6215),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 3 (6215, 6215)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(6215),
      this.instances3.alice.encrypt32(6215),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 4 (6215, 6211)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(6215),
      this.instances3.alice.encrypt32(6211),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 1 (24863, 17029307)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(24863),
      this.instances3.alice.encrypt32(17029307),
    );
    expect(res).to.equal(24863);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 2 (24859, 24863)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(24859),
      this.instances3.alice.encrypt32(24863),
    );
    expect(res).to.equal(24859);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 3 (24863, 24863)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(24863),
      this.instances3.alice.encrypt32(24863),
    );
    expect(res).to.equal(24863);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 4 (24863, 24859)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(24863),
      this.instances3.alice.encrypt32(24859),
    );
    expect(res).to.equal(24859);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (47912, 251556706)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(47912),
      this.instances3.alice.encrypt32(251556706),
    );
    expect(res).to.equal(251556706);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (47908, 47912)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(47908),
      this.instances3.alice.encrypt32(47912),
    );
    expect(res).to.equal(47912);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (47912, 47912)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(47912),
      this.instances3.alice.encrypt32(47912),
    );
    expect(res).to.equal(47912);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 4 (47912, 47908)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(47912),
      this.instances3.alice.encrypt32(47908),
    );
    expect(res).to.equal(47912);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 1 (108, 51831)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(108),
      this.instances3.alice.encrypt64(51831),
    );
    expect(res).to.equal(51939);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 2 (27895, 27899)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(27895),
      this.instances3.alice.encrypt64(27899),
    );
    expect(res).to.equal(55794);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 3 (27899, 27899)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(27899),
      this.instances3.alice.encrypt64(27899),
    );
    expect(res).to.equal(55798);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 4 (27899, 27895)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(27899),
      this.instances3.alice.encrypt64(27895),
    );
    expect(res).to.equal(55794);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 1 (63598, 63598)', async function () {
    const res = await this.contract3.sub_euint16_euint64(
      this.instances3.alice.encrypt16(63598),
      this.instances3.alice.encrypt64(63598),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 2 (63598, 63594)', async function () {
    const res = await this.contract3.sub_euint16_euint64(
      this.instances3.alice.encrypt16(63598),
      this.instances3.alice.encrypt64(63594),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 1 (2, 16788)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt64(16788),
    );
    expect(res).to.equal(33576);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 2 (154, 154)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(154),
      this.instances3.alice.encrypt64(154),
    );
    expect(res).to.equal(23716);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 3 (154, 154)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(154),
      this.instances3.alice.encrypt64(154),
    );
    expect(res).to.equal(23716);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 4 (154, 154)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(154),
      this.instances3.alice.encrypt64(154),
    );
    expect(res).to.equal(23716);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 1 (26290, 78773969)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(26290),
      this.instances3.alice.encrypt64(78773969),
    );
    expect(res).to.equal(26256);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 2 (26286, 26290)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(26286),
      this.instances3.alice.encrypt64(26290),
    );
    expect(res).to.equal(26274);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 3 (26290, 26290)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(26290),
      this.instances3.alice.encrypt64(26290),
    );
    expect(res).to.equal(26290);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 4 (26290, 26286)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(26290),
      this.instances3.alice.encrypt64(26286),
    );
    expect(res).to.equal(26274);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 1 (61878, 192916939)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(61878),
      this.instances3.alice.encrypt64(192916939),
    );
    expect(res).to.equal(192937471);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 2 (61874, 61878)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(61874),
      this.instances3.alice.encrypt64(61878),
    );
    expect(res).to.equal(61878);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 3 (61878, 61878)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(61878),
      this.instances3.alice.encrypt64(61878),
    );
    expect(res).to.equal(61878);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 4 (61878, 61874)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(61878),
      this.instances3.alice.encrypt64(61874),
    );
    expect(res).to.equal(61878);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 1 (36773, 191475857)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(36773),
      this.instances3.alice.encrypt64(191475857),
    );
    expect(res).to.equal(191446836);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 2 (36769, 36773)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(36769),
      this.instances3.alice.encrypt64(36773),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 3 (36773, 36773)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(36773),
      this.instances3.alice.encrypt64(36773),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 4 (36773, 36769)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(36773),
      this.instances3.alice.encrypt64(36769),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 1 (58427, 265593759)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(58427),
      this.instances3.alice.encrypt64(265593759),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 2 (58423, 58427)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(58423),
      this.instances3.alice.encrypt64(58427),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 3 (58427, 58427)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(58427),
      this.instances3.alice.encrypt64(58427),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 4 (58427, 58423)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(58427),
      this.instances3.alice.encrypt64(58423),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 1 (22739, 233045312)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(22739),
      this.instances3.alice.encrypt64(233045312),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 2 (22735, 22739)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(22735),
      this.instances3.alice.encrypt64(22739),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 3 (22739, 22739)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(22739),
      this.instances3.alice.encrypt64(22739),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 4 (22739, 22735)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(22739),
      this.instances3.alice.encrypt64(22735),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 1 (15004, 5255422)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(15004),
      this.instances3.alice.encrypt64(5255422),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 2 (15000, 15004)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(15000),
      this.instances3.alice.encrypt64(15004),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 3 (15004, 15004)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(15004),
      this.instances3.alice.encrypt64(15004),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 4 (15004, 15000)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(15004),
      this.instances3.alice.encrypt64(15000),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 1 (31594, 160571558)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(31594),
      this.instances3.alice.encrypt64(160571558),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 2 (31590, 31594)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(31590),
      this.instances3.alice.encrypt64(31594),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 3 (31594, 31594)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(31594),
      this.instances3.alice.encrypt64(31594),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 4 (31594, 31590)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(31594),
      this.instances3.alice.encrypt64(31590),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 1 (2104, 188633784)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(2104),
      this.instances3.alice.encrypt64(188633784),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 2 (2100, 2104)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(2100),
      this.instances3.alice.encrypt64(2104),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 3 (2104, 2104)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(2104),
      this.instances3.alice.encrypt64(2104),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 4 (2104, 2100)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(2104),
      this.instances3.alice.encrypt64(2100),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 1 (6215, 123133979)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(6215),
      this.instances3.alice.encrypt64(123133979),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 2 (6211, 6215)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(6211),
      this.instances3.alice.encrypt64(6215),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 3 (6215, 6215)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(6215),
      this.instances3.alice.encrypt64(6215),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 4 (6215, 6211)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(6215),
      this.instances3.alice.encrypt64(6211),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 1 (24863, 217108470)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(24863),
      this.instances3.alice.encrypt64(217108470),
    );
    expect(res).to.equal(24863);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 2 (24859, 24863)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(24859),
      this.instances3.alice.encrypt64(24863),
    );
    expect(res).to.equal(24859);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 3 (24863, 24863)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(24863),
      this.instances3.alice.encrypt64(24863),
    );
    expect(res).to.equal(24863);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 4 (24863, 24859)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(24863),
      this.instances3.alice.encrypt64(24859),
    );
    expect(res).to.equal(24859);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 1 (47912, 227727600)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(47912),
      this.instances3.alice.encrypt64(227727600),
    );
    expect(res).to.equal(227727600);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 2 (47908, 47912)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(47908),
      this.instances3.alice.encrypt64(47912),
    );
    expect(res).to.equal(47912);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 3 (47912, 47912)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(47912),
      this.instances3.alice.encrypt64(47912),
    );
    expect(res).to.equal(47912);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 4 (47912, 47908)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(47912),
      this.instances3.alice.encrypt64(47908),
    );
    expect(res).to.equal(47912);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (7885, 25213)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(7885), 25213);
    expect(res).to.equal(33098);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (15767, 15771)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(15767), 15771);
    expect(res).to.equal(31538);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (15771, 15771)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(15771), 15771);
    expect(res).to.equal(31542);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (15771, 15767)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(15771), 15767);
    expect(res).to.equal(31538);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (13949, 25213)', async function () {
    const res = await this.contract3.add_uint16_euint16(13949, this.instances3.alice.encrypt16(25213));
    expect(res).to.equal(39162);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 2 (15767, 15771)', async function () {
    const res = await this.contract3.add_uint16_euint16(15767, this.instances3.alice.encrypt16(15771));
    expect(res).to.equal(31538);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 3 (15771, 15771)', async function () {
    const res = await this.contract3.add_uint16_euint16(15771, this.instances3.alice.encrypt16(15771));
    expect(res).to.equal(31542);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 4 (15771, 15767)', async function () {
    const res = await this.contract3.add_uint16_euint16(15771, this.instances3.alice.encrypt16(15767));
    expect(res).to.equal(31538);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (20069, 20069)', async function () {
    const res = await this.contract3.sub_euint16_uint16(this.instances3.alice.encrypt16(20069), 20069);
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 2 (20069, 20065)', async function () {
    const res = await this.contract3.sub_euint16_uint16(this.instances3.alice.encrypt16(20069), 20065);
    expect(res).to.equal(4);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (20069, 20069)', async function () {
    const res = await this.contract3.sub_uint16_euint16(20069, this.instances3.alice.encrypt16(20069));
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (20069, 20065)', async function () {
    const res = await this.contract3.sub_uint16_euint16(20069, this.instances3.alice.encrypt16(20065));
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (136, 406)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(136), 406);
    expect(res).to.equal(55216);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (136, 136)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(136), 136);
    expect(res).to.equal(18496);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (136, 136)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(136), 136);
    expect(res).to.equal(18496);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (136, 136)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(136), 136);
    expect(res).to.equal(18496);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (308, 203)', async function () {
    const res = await this.contract3.mul_uint16_euint16(308, this.instances3.alice.encrypt16(203));
    expect(res).to.equal(62524);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 2 (136, 136)', async function () {
    const res = await this.contract3.mul_uint16_euint16(136, this.instances3.alice.encrypt16(136));
    expect(res).to.equal(18496);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 3 (136, 136)', async function () {
    const res = await this.contract3.mul_uint16_euint16(136, this.instances3.alice.encrypt16(136));
    expect(res).to.equal(18496);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 4 (136, 136)', async function () {
    const res = await this.contract3.mul_uint16_euint16(136, this.instances3.alice.encrypt16(136));
    expect(res).to.equal(18496);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (34461, 47014)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(34461), 47014);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (34457, 34461)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(34457), 34461);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (34461, 34461)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(34461), 34461);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (34461, 34457)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(34461), 34457);
    expect(res).to.equal(1);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (302, 42308)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(302), 42308);
    expect(res).to.equal(302);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 2 (298, 302)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(298), 302);
    expect(res).to.equal(298);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 3 (302, 302)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(302), 302);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 4 (302, 298)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(302), 298);
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (44634, 11079)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(44634), 11079);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (41622, 41626)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(41622), 41626);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (41626, 41626)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(41626), 41626);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (41626, 41622)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(41626), 41622);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (58427, 11079)', async function () {
    const res = await this.contract3.eq_uint16_euint16(58427, this.instances3.alice.encrypt16(11079));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (41622, 41626)', async function () {
    const res = await this.contract3.eq_uint16_euint16(41622, this.instances3.alice.encrypt16(41626));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 3 (41626, 41626)', async function () {
    const res = await this.contract3.eq_uint16_euint16(41626, this.instances3.alice.encrypt16(41626));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 4 (41626, 41622)', async function () {
    const res = await this.contract3.eq_uint16_euint16(41626, this.instances3.alice.encrypt16(41622));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (58512, 5930)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(58512), 5930);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (41575, 41579)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(41575), 41579);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 3 (41579, 41579)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(41579), 41579);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 4 (41579, 41575)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(41579), 41575);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (22739, 5930)', async function () {
    const res = await this.contract3.ne_uint16_euint16(22739, this.instances3.alice.encrypt16(5930));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (41575, 41579)', async function () {
    const res = await this.contract3.ne_uint16_euint16(41575, this.instances3.alice.encrypt16(41579));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 3 (41579, 41579)', async function () {
    const res = await this.contract3.ne_uint16_euint16(41579, this.instances3.alice.encrypt16(41579));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 4 (41579, 41575)', async function () {
    const res = await this.contract3.ne_uint16_euint16(41579, this.instances3.alice.encrypt16(41575));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (2319, 46898)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(2319), 46898);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (2315, 2319)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(2315), 2319);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (2319, 2319)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(2319), 2319);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 4 (2319, 2315)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(2319), 2315);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (15004, 46898)', async function () {
    const res = await this.contract3.ge_uint16_euint16(15004, this.instances3.alice.encrypt16(46898));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (2315, 2319)', async function () {
    const res = await this.contract3.ge_uint16_euint16(2315, this.instances3.alice.encrypt16(2319));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (2319, 2319)', async function () {
    const res = await this.contract3.ge_uint16_euint16(2319, this.instances3.alice.encrypt16(2319));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 4 (2319, 2315)', async function () {
    const res = await this.contract3.ge_uint16_euint16(2319, this.instances3.alice.encrypt16(2315));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (39995, 40403)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(39995), 40403);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (39991, 39995)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(39991), 39995);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (39995, 39995)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(39995), 39995);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 4 (39995, 39991)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(39995), 39991);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (31594, 40403)', async function () {
    const res = await this.contract3.gt_uint16_euint16(31594, this.instances3.alice.encrypt16(40403));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (39991, 39995)', async function () {
    const res = await this.contract3.gt_uint16_euint16(39991, this.instances3.alice.encrypt16(39995));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (39995, 39995)', async function () {
    const res = await this.contract3.gt_uint16_euint16(39995, this.instances3.alice.encrypt16(39995));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 4 (39995, 39991)', async function () {
    const res = await this.contract3.gt_uint16_euint16(39995, this.instances3.alice.encrypt16(39991));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (11868, 63783)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(11868), 63783);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (11864, 11868)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(11864), 11868);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (11868, 11868)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(11868), 11868);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 4 (11868, 11864)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(11868), 11864);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (2104, 63783)', async function () {
    const res = await this.contract3.le_uint16_euint16(2104, this.instances3.alice.encrypt16(63783));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (11864, 11868)', async function () {
    const res = await this.contract3.le_uint16_euint16(11864, this.instances3.alice.encrypt16(11868));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (11868, 11868)', async function () {
    const res = await this.contract3.le_uint16_euint16(11868, this.instances3.alice.encrypt16(11868));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (11868, 11864)', async function () {
    const res = await this.contract3.le_uint16_euint16(11868, this.instances3.alice.encrypt16(11864));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (13005, 41485)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(13005), 41485);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (13001, 13005)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(13001), 13005);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (13005, 13005)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(13005), 13005);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 4 (13005, 13001)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(13005), 13001);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (6215, 41485)', async function () {
    const res = await this.contract3.lt_uint16_euint16(6215, this.instances3.alice.encrypt16(41485));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (13001, 13005)', async function () {
    const res = await this.contract3.lt_uint16_euint16(13001, this.instances3.alice.encrypt16(13005));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (13005, 13005)', async function () {
    const res = await this.contract3.lt_uint16_euint16(13005, this.instances3.alice.encrypt16(13005));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 4 (13005, 13001)', async function () {
    const res = await this.contract3.lt_uint16_euint16(13005, this.instances3.alice.encrypt16(13001));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (59594, 37209)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(59594), 37209);
    expect(res).to.equal(37209);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (35024, 35028)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(35024), 35028);
    expect(res).to.equal(35024);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (35028, 35028)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(35028), 35028);
    expect(res).to.equal(35028);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 4 (35028, 35024)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(35028), 35024);
    expect(res).to.equal(35024);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (24863, 37209)', async function () {
    const res = await this.contract3.min_uint16_euint16(24863, this.instances3.alice.encrypt16(37209));
    expect(res).to.equal(24863);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (35024, 35028)', async function () {
    const res = await this.contract3.min_uint16_euint16(35024, this.instances3.alice.encrypt16(35028));
    expect(res).to.equal(35024);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (35028, 35028)', async function () {
    const res = await this.contract3.min_uint16_euint16(35028, this.instances3.alice.encrypt16(35028));
    expect(res).to.equal(35028);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (35028, 35024)', async function () {
    const res = await this.contract3.min_uint16_euint16(35028, this.instances3.alice.encrypt16(35024));
    expect(res).to.equal(35024);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (45745, 18939)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(45745), 18939);
    expect(res).to.equal(45745);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (24756, 24760)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(24756), 24760);
    expect(res).to.equal(24760);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (24760, 24760)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(24760), 24760);
    expect(res).to.equal(24760);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (24760, 24756)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(24760), 24756);
    expect(res).to.equal(24760);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (47912, 18939)', async function () {
    const res = await this.contract3.max_uint16_euint16(47912, this.instances3.alice.encrypt16(18939));
    expect(res).to.equal(47912);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (24756, 24760)', async function () {
    const res = await this.contract3.max_uint16_euint16(24756, this.instances3.alice.encrypt16(24760));
    expect(res).to.equal(24760);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (24760, 24760)', async function () {
    const res = await this.contract3.max_uint16_euint16(24760, this.instances3.alice.encrypt16(24760));
    expect(res).to.equal(24760);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (24760, 24756)', async function () {
    const res = await this.contract3.max_uint16_euint16(24760, this.instances3.alice.encrypt16(24756));
    expect(res).to.equal(24760);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 1 (10, 1)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(10),
      this.instances3.alice.encrypt4(1),
    );
    expect(res).to.equal(11);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 3 (4, 4)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "sub" overload (euint32, euint4) => euint32 test 1 (10, 10)', async function () {
    const res = await this.contract3.sub_euint32_euint4(
      this.instances3.alice.encrypt32(10),
      this.instances3.alice.encrypt4(10),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, euint4) => euint32 test 2 (10, 6)', async function () {
    const res = await this.contract3.sub_euint32_euint4(
      this.instances3.alice.encrypt32(10),
      this.instances3.alice.encrypt4(6),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 1 (9, 1)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(1),
    );
    expect(res).to.equal(9);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 2 (3, 5)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(3),
      this.instances3.alice.encrypt4(5),
    );
    expect(res).to.equal(15);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 3 (2, 2)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(2),
      this.instances3.alice.encrypt4(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 4 (5, 3)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(5),
      this.instances3.alice.encrypt4(3),
    );
    expect(res).to.equal(15);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 1 (241094180, 10)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(241094180),
      this.instances3.alice.encrypt4(10),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 2 (6, 10)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(6),
      this.instances3.alice.encrypt4(10),
    );
    expect(res).to.equal(2);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 3 (10, 10)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(10),
      this.instances3.alice.encrypt4(10),
    );
    expect(res).to.equal(10);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 4 (10, 6)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(10),
      this.instances3.alice.encrypt4(6),
    );
    expect(res).to.equal(2);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 1 (51366153, 1)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(51366153),
      this.instances3.alice.encrypt4(1),
    );
    expect(res).to.equal(51366153);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 1 (30847431, 8)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(30847431),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(30847439);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 1 (82686069, 12)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(82686069),
      this.instances3.alice.encrypt4(12),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(12),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(12),
      this.instances3.alice.encrypt4(12),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(12),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 1 (3660714, 9)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(3660714),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(5),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 1 (182035180, 9)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(182035180),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(5),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 1 (228583889, 14)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(228583889),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(10),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(14),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(14),
      this.instances3.alice.encrypt4(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 1 (232635457, 13)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(232635457),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(13),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(13),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 1 (89546642, 8)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(89546642),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 1 (58829340, 8)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(58829340),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 1 (8800615, 7)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(8800615),
      this.instances3.alice.encrypt4(7),
    );
    expect(res).to.equal(8800615);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (165, 1)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(165),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(166);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (13, 17)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(13),
      this.instances3.alice.encrypt8(17),
    );
    expect(res).to.equal(30);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (17, 17)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(17),
      this.instances3.alice.encrypt8(17),
    );
    expect(res).to.equal(34);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (17, 13)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(17),
      this.instances3.alice.encrypt8(13),
    );
    expect(res).to.equal(30);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (123, 123)', async function () {
    const res = await this.contract3.sub_euint32_euint8(
      this.instances3.alice.encrypt32(123),
      this.instances3.alice.encrypt8(123),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (123, 119)', async function () {
    const res = await this.contract3.sub_euint32_euint8(
      this.instances3.alice.encrypt32(123),
      this.instances3.alice.encrypt8(119),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (146, 1)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(146),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(146);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 2 (9, 10)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt8(10),
    );
    expect(res).to.equal(90);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 3 (10, 10)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(10),
      this.instances3.alice.encrypt8(10),
    );
    expect(res).to.equal(100);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 4 (10, 9)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(10),
      this.instances3.alice.encrypt8(9),
    );
    expect(res).to.equal(90);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (241094180, 159)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(241094180),
      this.instances3.alice.encrypt8(159),
    );
    expect(res).to.equal(4);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (155, 159)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(155),
      this.instances3.alice.encrypt8(159),
    );
    expect(res).to.equal(155);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (159, 159)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(159),
      this.instances3.alice.encrypt8(159),
    );
    expect(res).to.equal(159);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (159, 155)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(159),
      this.instances3.alice.encrypt8(155),
    );
    expect(res).to.equal(155);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (51366153, 56)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(51366153),
      this.instances4.alice.encrypt8(56),
    );
    expect(res).to.equal(51366201);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (52, 56)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(52),
      this.instances4.alice.encrypt8(56),
    );
    expect(res).to.equal(60);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (56, 56)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(56),
      this.instances4.alice.encrypt8(56),
    );
    expect(res).to.equal(56);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (56, 52)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(56),
      this.instances4.alice.encrypt8(52),
    );
    expect(res).to.equal(60);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (30847431, 23)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(30847431),
      this.instances4.alice.encrypt8(23),
    );
    expect(res).to.equal(30847440);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (19, 23)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(19),
      this.instances4.alice.encrypt8(23),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (23, 23)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(23),
      this.instances4.alice.encrypt8(23),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (23, 19)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(23),
      this.instances4.alice.encrypt8(19),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (82686069, 42)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(82686069),
      this.instances4.alice.encrypt8(42),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (38, 42)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(38),
      this.instances4.alice.encrypt8(42),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 3 (42, 42)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(42),
      this.instances4.alice.encrypt8(42),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 4 (42, 38)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(42),
      this.instances4.alice.encrypt8(38),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (3660714, 206)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(3660714),
      this.instances4.alice.encrypt8(206),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (202, 206)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(202),
      this.instances4.alice.encrypt8(206),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 3 (206, 206)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(206),
      this.instances4.alice.encrypt8(206),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 4 (206, 202)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(206),
      this.instances4.alice.encrypt8(202),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (182035180, 64)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(182035180),
      this.instances4.alice.encrypt8(64),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (60, 64)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(60),
      this.instances4.alice.encrypt8(64),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (64, 64)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(64),
      this.instances4.alice.encrypt8(64),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 4 (64, 60)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(64),
      this.instances4.alice.encrypt8(60),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (228583889, 150)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(228583889),
      this.instances4.alice.encrypt8(150),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (146, 150)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(146),
      this.instances4.alice.encrypt8(150),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (150, 150)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(150),
      this.instances4.alice.encrypt8(150),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 4 (150, 146)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(150),
      this.instances4.alice.encrypt8(146),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (232635457, 170)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(232635457),
      this.instances4.alice.encrypt8(170),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (166, 170)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(166),
      this.instances4.alice.encrypt8(170),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (170, 170)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(170),
      this.instances4.alice.encrypt8(170),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (170, 166)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(170),
      this.instances4.alice.encrypt8(166),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (89546642, 186)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(89546642),
      this.instances4.alice.encrypt8(186),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (182, 186)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(182),
      this.instances4.alice.encrypt8(186),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (186, 186)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(186),
      this.instances4.alice.encrypt8(186),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 4 (186, 182)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(186),
      this.instances4.alice.encrypt8(182),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (58829340, 181)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(58829340),
      this.instances4.alice.encrypt8(181),
    );
    expect(res).to.equal(181);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (177, 181)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(177),
      this.instances4.alice.encrypt8(181),
    );
    expect(res).to.equal(177);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (181, 181)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(181),
      this.instances4.alice.encrypt8(181),
    );
    expect(res).to.equal(181);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (181, 177)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(181),
      this.instances4.alice.encrypt8(177),
    );
    expect(res).to.equal(177);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (8800615, 210)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(8800615),
      this.instances4.alice.encrypt8(210),
    );
    expect(res).to.equal(8800615);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (206, 210)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(206),
      this.instances4.alice.encrypt8(210),
    );
    expect(res).to.equal(210);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (210, 210)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(210),
      this.instances4.alice.encrypt8(210),
    );
    expect(res).to.equal(210);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 4 (210, 206)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(210),
      this.instances4.alice.encrypt8(206),
    );
    expect(res).to.equal(210);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (42385, 126)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(42385),
      this.instances4.alice.encrypt16(126),
    );
    expect(res).to.equal(42511);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (32398, 32400)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(32398),
      this.instances4.alice.encrypt16(32400),
    );
    expect(res).to.equal(64798);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (32400, 32400)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(32400),
      this.instances4.alice.encrypt16(32400),
    );
    expect(res).to.equal(64800);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (32400, 32398)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(32400),
      this.instances4.alice.encrypt16(32398),
    );
    expect(res).to.equal(64798);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (43081, 43081)', async function () {
    const res = await this.contract4.sub_euint32_euint16(
      this.instances4.alice.encrypt32(43081),
      this.instances4.alice.encrypt16(43081),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (43081, 43077)', async function () {
    const res = await this.contract4.sub_euint32_euint16(
      this.instances4.alice.encrypt32(43081),
      this.instances4.alice.encrypt16(43077),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (9346, 3)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(9346),
      this.instances4.alice.encrypt16(3),
    );
    expect(res).to.equal(28038);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (206, 206)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(206),
      this.instances4.alice.encrypt16(206),
    );
    expect(res).to.equal(42436);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (206, 206)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(206),
      this.instances4.alice.encrypt16(206),
    );
    expect(res).to.equal(42436);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (206, 206)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(206),
      this.instances4.alice.encrypt16(206),
    );
    expect(res).to.equal(42436);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (241094180, 53800)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(241094180),
      this.instances4.alice.encrypt16(53800),
    );
    expect(res).to.equal(49696);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (53796, 53800)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(53796),
      this.instances4.alice.encrypt16(53800),
    );
    expect(res).to.equal(53792);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (53800, 53800)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(53800),
      this.instances4.alice.encrypt16(53800),
    );
    expect(res).to.equal(53800);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (53800, 53796)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(53800),
      this.instances4.alice.encrypt16(53796),
    );
    expect(res).to.equal(53792);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (51366153, 45753)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(51366153),
      this.instances4.alice.encrypt16(45753),
    );
    expect(res).to.equal(51379129);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (45749, 45753)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(45749),
      this.instances4.alice.encrypt16(45753),
    );
    expect(res).to.equal(45757);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 3 (45753, 45753)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(45753),
      this.instances4.alice.encrypt16(45753),
    );
    expect(res).to.equal(45753);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 4 (45753, 45749)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(45753),
      this.instances4.alice.encrypt16(45749),
    );
    expect(res).to.equal(45757);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (30847431, 51762)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(30847431),
      this.instances4.alice.encrypt16(51762),
    );
    expect(res).to.equal(30833653);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (51758, 51762)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(51758),
      this.instances4.alice.encrypt16(51762),
    );
    expect(res).to.equal(28);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (51762, 51762)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(51762),
      this.instances4.alice.encrypt16(51762),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (51762, 51758)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(51762),
      this.instances4.alice.encrypt16(51758),
    );
    expect(res).to.equal(28);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (82686069, 20674)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(82686069),
      this.instances4.alice.encrypt16(20674),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (20670, 20674)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(20670),
      this.instances4.alice.encrypt16(20674),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (20674, 20674)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(20674),
      this.instances4.alice.encrypt16(20674),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (20674, 20670)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(20674),
      this.instances4.alice.encrypt16(20670),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (3660714, 6484)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(3660714),
      this.instances4.alice.encrypt16(6484),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (6480, 6484)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(6480),
      this.instances4.alice.encrypt16(6484),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 3 (6484, 6484)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(6484),
      this.instances4.alice.encrypt16(6484),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 4 (6484, 6480)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(6484),
      this.instances4.alice.encrypt16(6480),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (182035180, 57858)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(182035180),
      this.instances4.alice.encrypt16(57858),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (57854, 57858)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(57854),
      this.instances4.alice.encrypt16(57858),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (57858, 57858)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(57858),
      this.instances4.alice.encrypt16(57858),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 4 (57858, 57854)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(57858),
      this.instances4.alice.encrypt16(57854),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (228583889, 65231)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(228583889),
      this.instances4.alice.encrypt16(65231),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (65227, 65231)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(65227),
      this.instances4.alice.encrypt16(65231),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (65231, 65231)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(65231),
      this.instances4.alice.encrypt16(65231),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 4 (65231, 65227)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(65231),
      this.instances4.alice.encrypt16(65227),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (232635457, 42879)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(232635457),
      this.instances4.alice.encrypt16(42879),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (42875, 42879)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(42875),
      this.instances4.alice.encrypt16(42879),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (42879, 42879)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(42879),
      this.instances4.alice.encrypt16(42879),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (42879, 42875)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(42879),
      this.instances4.alice.encrypt16(42875),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (89546642, 11355)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(89546642),
      this.instances4.alice.encrypt16(11355),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (11351, 11355)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(11351),
      this.instances4.alice.encrypt16(11355),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (11355, 11355)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(11355),
      this.instances4.alice.encrypt16(11355),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (11355, 11351)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(11355),
      this.instances4.alice.encrypt16(11351),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (58829340, 33486)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(58829340),
      this.instances4.alice.encrypt16(33486),
    );
    expect(res).to.equal(33486);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (33482, 33486)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(33482),
      this.instances4.alice.encrypt16(33486),
    );
    expect(res).to.equal(33482);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (33486, 33486)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(33486),
      this.instances4.alice.encrypt16(33486),
    );
    expect(res).to.equal(33486);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (33486, 33482)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(33486),
      this.instances4.alice.encrypt16(33482),
    );
    expect(res).to.equal(33482);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (8800615, 31093)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(8800615),
      this.instances4.alice.encrypt16(31093),
    );
    expect(res).to.equal(8800615);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (31089, 31093)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(31089),
      this.instances4.alice.encrypt16(31093),
    );
    expect(res).to.equal(31093);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (31093, 31093)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(31093),
      this.instances4.alice.encrypt16(31093),
    );
    expect(res).to.equal(31093);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (31093, 31089)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(31093),
      this.instances4.alice.encrypt16(31089),
    );
    expect(res).to.equal(31093);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (21701237, 233681477)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(21701237),
      this.instances4.alice.encrypt32(233681477),
    );
    expect(res).to.equal(255382714);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (21701233, 21701237)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(21701233),
      this.instances4.alice.encrypt32(21701237),
    );
    expect(res).to.equal(43402470);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (21701237, 21701237)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(21701237),
      this.instances4.alice.encrypt32(21701237),
    );
    expect(res).to.equal(43402474);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (21701237, 21701233)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(21701237),
      this.instances4.alice.encrypt32(21701233),
    );
    expect(res).to.equal(43402470);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (142194314, 142194314)', async function () {
    const res = await this.contract4.sub_euint32_euint32(
      this.instances4.alice.encrypt32(142194314),
      this.instances4.alice.encrypt32(142194314),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (142194314, 142194310)', async function () {
    const res = await this.contract4.sub_euint32_euint32(
      this.instances4.alice.encrypt32(142194314),
      this.instances4.alice.encrypt32(142194310),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (74769, 34417)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(74769),
      this.instances4.alice.encrypt32(34417),
    );
    expect(res).to.equal(2573324673);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 2 (34417, 34417)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(34417),
      this.instances4.alice.encrypt32(34417),
    );
    expect(res).to.equal(1184529889);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 3 (34417, 34417)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(34417),
      this.instances4.alice.encrypt32(34417),
    );
    expect(res).to.equal(1184529889);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 4 (34417, 34417)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(34417),
      this.instances4.alice.encrypt32(34417),
    );
    expect(res).to.equal(1184529889);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (241094180, 79267520)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(241094180),
      this.instances4.alice.encrypt32(79267520),
    );
    expect(res).to.equal(68716032);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (79267516, 79267520)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(79267516),
      this.instances4.alice.encrypt32(79267520),
    );
    expect(res).to.equal(79267456);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 3 (79267520, 79267520)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(79267520),
      this.instances4.alice.encrypt32(79267520),
    );
    expect(res).to.equal(79267520);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 4 (79267520, 79267516)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(79267520),
      this.instances4.alice.encrypt32(79267516),
    );
    expect(res).to.equal(79267456);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (51366153, 128317949)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(51366153),
      this.instances4.alice.encrypt32(128317949),
    );
    expect(res).to.equal(128973309);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (51366149, 51366153)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(51366149),
      this.instances4.alice.encrypt32(51366153),
    );
    expect(res).to.equal(51366157);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 3 (51366153, 51366153)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(51366153),
      this.instances4.alice.encrypt32(51366153),
    );
    expect(res).to.equal(51366153);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 4 (51366153, 51366149)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(51366153),
      this.instances4.alice.encrypt32(51366149),
    );
    expect(res).to.equal(51366157);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (30847431, 53515901)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(30847431),
      this.instances4.alice.encrypt32(53515901),
    );
    expect(res).to.equal(48637882);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (30847427, 30847431)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(30847427),
      this.instances4.alice.encrypt32(30847431),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (30847431, 30847431)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(30847431),
      this.instances4.alice.encrypt32(30847431),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (30847431, 30847427)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(30847431),
      this.instances4.alice.encrypt32(30847427),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (82686069, 111825281)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(82686069),
      this.instances4.alice.encrypt32(111825281),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (82686065, 82686069)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(82686065),
      this.instances4.alice.encrypt32(82686069),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (82686069, 82686069)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(82686069),
      this.instances4.alice.encrypt32(82686069),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (82686069, 82686065)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(82686069),
      this.instances4.alice.encrypt32(82686065),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (3660714, 184322766)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(3660714),
      this.instances4.alice.encrypt32(184322766),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (3660710, 3660714)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(3660710),
      this.instances4.alice.encrypt32(3660714),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 3 (3660714, 3660714)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(3660714),
      this.instances4.alice.encrypt32(3660714),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 4 (3660714, 3660710)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(3660714),
      this.instances4.alice.encrypt32(3660710),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (182035180, 102858487)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(182035180),
      this.instances4.alice.encrypt32(102858487),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (102858483, 102858487)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(102858483),
      this.instances4.alice.encrypt32(102858487),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (102858487, 102858487)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(102858487),
      this.instances4.alice.encrypt32(102858487),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (102858487, 102858483)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(102858487),
      this.instances4.alice.encrypt32(102858483),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (228583889, 44696680)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(228583889),
      this.instances4.alice.encrypt32(44696680),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (44696676, 44696680)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(44696676),
      this.instances4.alice.encrypt32(44696680),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (44696680, 44696680)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(44696680),
      this.instances4.alice.encrypt32(44696680),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 4 (44696680, 44696676)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(44696680),
      this.instances4.alice.encrypt32(44696676),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (232635457, 156819794)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(232635457),
      this.instances4.alice.encrypt32(156819794),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (156819790, 156819794)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(156819790),
      this.instances4.alice.encrypt32(156819794),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (156819794, 156819794)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(156819794),
      this.instances4.alice.encrypt32(156819794),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 4 (156819794, 156819790)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(156819794),
      this.instances4.alice.encrypt32(156819790),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (89546642, 135434402)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(89546642),
      this.instances4.alice.encrypt32(135434402),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (89546638, 89546642)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(89546638),
      this.instances4.alice.encrypt32(89546642),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (89546642, 89546642)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(89546642),
      this.instances4.alice.encrypt32(89546642),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (89546642, 89546638)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(89546642),
      this.instances4.alice.encrypt32(89546638),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (58829340, 209742110)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(58829340),
      this.instances4.alice.encrypt32(209742110),
    );
    expect(res).to.equal(58829340);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (58829336, 58829340)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(58829336),
      this.instances4.alice.encrypt32(58829340),
    );
    expect(res).to.equal(58829336);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (58829340, 58829340)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(58829340),
      this.instances4.alice.encrypt32(58829340),
    );
    expect(res).to.equal(58829340);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (58829340, 58829336)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(58829340),
      this.instances4.alice.encrypt32(58829336),
    );
    expect(res).to.equal(58829336);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (8800615, 151759654)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(8800615),
      this.instances4.alice.encrypt32(151759654),
    );
    expect(res).to.equal(151759654);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (8800611, 8800615)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(8800611),
      this.instances4.alice.encrypt32(8800615),
    );
    expect(res).to.equal(8800615);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (8800615, 8800615)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(8800615),
      this.instances4.alice.encrypt32(8800615),
    );
    expect(res).to.equal(8800615);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (8800615, 8800611)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(8800615),
      this.instances4.alice.encrypt32(8800611),
    );
    expect(res).to.equal(8800615);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 1 (203659740, 114096043)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(203659740),
      this.instances4.alice.encrypt64(114096043),
    );
    expect(res).to.equal(317755783);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 2 (114096039, 114096043)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(114096039),
      this.instances4.alice.encrypt64(114096043),
    );
    expect(res).to.equal(228192082);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 3 (114096043, 114096043)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(114096043),
      this.instances4.alice.encrypt64(114096043),
    );
    expect(res).to.equal(228192086);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 4 (114096043, 114096039)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(114096043),
      this.instances4.alice.encrypt64(114096039),
    );
    expect(res).to.equal(228192082);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (116748811, 116748811)', async function () {
    const res = await this.contract4.sub_euint32_euint64(
      this.instances4.alice.encrypt32(116748811),
      this.instances4.alice.encrypt64(116748811),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (116748811, 116748807)', async function () {
    const res = await this.contract4.sub_euint32_euint64(
      this.instances4.alice.encrypt32(116748811),
      this.instances4.alice.encrypt64(116748807),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (10792, 161277)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(10792),
      this.instances4.alice.encrypt64(161277),
    );
    expect(res).to.equal(1740501384);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (43171, 43171)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(43171),
      this.instances4.alice.encrypt64(43171),
    );
    expect(res).to.equal(1863735241);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (43171, 43171)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(43171),
      this.instances4.alice.encrypt64(43171),
    );
    expect(res).to.equal(1863735241);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (43171, 43171)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(43171),
      this.instances4.alice.encrypt64(43171),
    );
    expect(res).to.equal(1863735241);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 1 (241094180, 246901357)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(241094180),
      this.instances4.alice.encrypt64(246901357),
    );
    expect(res).to.equal(236341796);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 2 (241094176, 241094180)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(241094176),
      this.instances4.alice.encrypt64(241094180),
    );
    expect(res).to.equal(241094176);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 3 (241094180, 241094180)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(241094180),
      this.instances4.alice.encrypt64(241094180),
    );
    expect(res).to.equal(241094180);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 4 (241094180, 241094176)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(241094180),
      this.instances4.alice.encrypt64(241094176),
    );
    expect(res).to.equal(241094176);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 1 (51366153, 141446953)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(51366153),
      this.instances4.alice.encrypt64(141446953),
    );
    expect(res).to.equal(191876905);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 2 (51366149, 51366153)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(51366149),
      this.instances4.alice.encrypt64(51366153),
    );
    expect(res).to.equal(51366157);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 3 (51366153, 51366153)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(51366153),
      this.instances4.alice.encrypt64(51366153),
    );
    expect(res).to.equal(51366153);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 4 (51366153, 51366149)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(51366153),
      this.instances4.alice.encrypt64(51366149),
    );
    expect(res).to.equal(51366157);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (30847431, 224936709)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(30847431),
      this.instances4.alice.encrypt64(224936709),
    );
    expect(res).to.equal(213840578);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (30847427, 30847431)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(30847427),
      this.instances4.alice.encrypt64(30847431),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (30847431, 30847431)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(30847431),
      this.instances4.alice.encrypt64(30847431),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (30847431, 30847427)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(30847431),
      this.instances4.alice.encrypt64(30847427),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 1 (6049542, 40221551)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(6049542),
      this.instances4.alice.encrypt64(40221551),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 2 (6049538, 6049542)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(6049538),
      this.instances4.alice.encrypt64(6049542),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 3 (6049542, 6049542)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(6049542),
      this.instances4.alice.encrypt64(6049542),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 4 (6049542, 6049538)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(6049542),
      this.instances4.alice.encrypt64(6049538),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (223424023, 59824452)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(223424023),
      this.instances4.alice.encrypt64(59824452),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (59824448, 59824452)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(59824448),
      this.instances4.alice.encrypt64(59824452),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (59824452, 59824452)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(59824452),
      this.instances4.alice.encrypt64(59824452),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (59824452, 59824448)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(59824452),
      this.instances4.alice.encrypt64(59824448),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 1 (173909597, 200114020)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(173909597),
      this.instances4.alice.encrypt64(200114020),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 2 (173909593, 173909597)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(173909593),
      this.instances4.alice.encrypt64(173909597),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 3 (173909597, 173909597)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(173909597),
      this.instances4.alice.encrypt64(173909597),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 4 (173909597, 173909593)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(173909597),
      this.instances4.alice.encrypt64(173909593),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 1 (51988955, 19324508)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(51988955),
      this.instances4.alice.encrypt64(19324508),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 2 (19324504, 19324508)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(19324504),
      this.instances4.alice.encrypt64(19324508),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 3 (19324508, 19324508)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(19324508),
      this.instances4.alice.encrypt64(19324508),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 4 (19324508, 19324504)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(19324508),
      this.instances4.alice.encrypt64(19324504),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 1 (239805571, 15147063)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(239805571),
      this.instances4.alice.encrypt64(15147063),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 2 (15147059, 15147063)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(15147059),
      this.instances4.alice.encrypt64(15147063),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 3 (15147063, 15147063)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(15147063),
      this.instances4.alice.encrypt64(15147063),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 4 (15147063, 15147059)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(15147063),
      this.instances4.alice.encrypt64(15147059),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 1 (87828731, 91983750)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(87828731),
      this.instances4.alice.encrypt64(91983750),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 2 (87828727, 87828731)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(87828727),
      this.instances4.alice.encrypt64(87828731),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 3 (87828731, 87828731)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(87828731),
      this.instances4.alice.encrypt64(87828731),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 4 (87828731, 87828727)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(87828731),
      this.instances4.alice.encrypt64(87828727),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 1 (20594417, 150398136)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(20594417),
      this.instances4.alice.encrypt64(150398136),
    );
    expect(res).to.equal(20594417);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 2 (20594413, 20594417)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(20594413),
      this.instances4.alice.encrypt64(20594417),
    );
    expect(res).to.equal(20594413);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 3 (20594417, 20594417)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(20594417),
      this.instances4.alice.encrypt64(20594417),
    );
    expect(res).to.equal(20594417);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 4 (20594417, 20594413)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(20594417),
      this.instances4.alice.encrypt64(20594413),
    );
    expect(res).to.equal(20594413);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (220335343, 138563730)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(220335343),
      this.instances4.alice.encrypt64(138563730),
    );
    expect(res).to.equal(220335343);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (138563726, 138563730)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(138563726),
      this.instances4.alice.encrypt64(138563730),
    );
    expect(res).to.equal(138563730);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (138563730, 138563730)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(138563730),
      this.instances4.alice.encrypt64(138563730),
    );
    expect(res).to.equal(138563730);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (138563730, 138563726)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(138563730),
      this.instances4.alice.encrypt64(138563726),
    );
    expect(res).to.equal(138563730);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (21701237, 252093913)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(21701237), 252093913);
    expect(res).to.equal(273795150);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (21701233, 21701237)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(21701233), 21701237);
    expect(res).to.equal(43402470);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (21701237, 21701237)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(21701237), 21701237);
    expect(res).to.equal(43402474);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (21701237, 21701233)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(21701237), 21701233);
    expect(res).to.equal(43402470);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (203659740, 252093913)', async function () {
    const res = await this.contract4.add_uint32_euint32(203659740, this.instances4.alice.encrypt32(252093913));
    expect(res).to.equal(455753653);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 2 (21701233, 21701237)', async function () {
    const res = await this.contract4.add_uint32_euint32(21701233, this.instances4.alice.encrypt32(21701237));
    expect(res).to.equal(43402470);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 3 (21701237, 21701237)', async function () {
    const res = await this.contract4.add_uint32_euint32(21701237, this.instances4.alice.encrypt32(21701237));
    expect(res).to.equal(43402474);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 4 (21701237, 21701233)', async function () {
    const res = await this.contract4.add_uint32_euint32(21701237, this.instances4.alice.encrypt32(21701233));
    expect(res).to.equal(43402470);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (142194314, 142194314)', async function () {
    const res = await this.contract4.sub_euint32_uint32(this.instances4.alice.encrypt32(142194314), 142194314);
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 2 (142194314, 142194310)', async function () {
    const res = await this.contract4.sub_euint32_uint32(this.instances4.alice.encrypt32(142194314), 142194310);
    expect(res).to.equal(4);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (142194314, 142194314)', async function () {
    const res = await this.contract4.sub_uint32_euint32(142194314, this.instances4.alice.encrypt32(142194314));
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 2 (142194314, 142194310)', async function () {
    const res = await this.contract4.sub_uint32_euint32(142194314, this.instances4.alice.encrypt32(142194310));
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (37384, 56630)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(37384), 56630);
    expect(res).to.equal(2117055920);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 2 (34417, 34417)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(34417), 34417);
    expect(res).to.equal(1184529889);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 3 (34417, 34417)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(34417), 34417);
    expect(res).to.equal(1184529889);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 4 (34417, 34417)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(34417), 34417);
    expect(res).to.equal(1184529889);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (5396, 226523)', async function () {
    const res = await this.contract4.mul_uint32_euint32(5396, this.instances4.alice.encrypt32(226523));
    expect(res).to.equal(1222318108);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (34417, 34417)', async function () {
    const res = await this.contract4.mul_uint32_euint32(34417, this.instances4.alice.encrypt32(34417));
    expect(res).to.equal(1184529889);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (34417, 34417)', async function () {
    const res = await this.contract4.mul_uint32_euint32(34417, this.instances4.alice.encrypt32(34417));
    expect(res).to.equal(1184529889);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (34417, 34417)', async function () {
    const res = await this.contract4.mul_uint32_euint32(34417, this.instances4.alice.encrypt32(34417));
    expect(res).to.equal(1184529889);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (20956235, 20556991)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(20956235), 20556991);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (20956231, 20956235)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(20956231), 20956235);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (20956235, 20956235)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(20956235), 20956235);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (20956235, 20956231)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(20956235), 20956231);
    expect(res).to.equal(1);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (227728407, 99085597)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(227728407), 99085597);
    expect(res).to.equal(29557213);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 2 (187916193, 187916197)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(187916193), 187916197);
    expect(res).to.equal(187916193);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 3 (187916197, 187916197)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(187916197), 187916197);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 4 (187916197, 187916193)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(187916197), 187916193);
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (82686069, 99342572)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(82686069), 99342572);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (82686065, 82686069)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(82686065), 82686069);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 3 (82686069, 82686069)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(82686069), 82686069);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 4 (82686069, 82686065)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(82686069), 82686065);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (6049542, 99342572)', async function () {
    const res = await this.contract4.eq_uint32_euint32(6049542, this.instances4.alice.encrypt32(99342572));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (82686065, 82686069)', async function () {
    const res = await this.contract4.eq_uint32_euint32(82686065, this.instances4.alice.encrypt32(82686069));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 3 (82686069, 82686069)', async function () {
    const res = await this.contract4.eq_uint32_euint32(82686069, this.instances4.alice.encrypt32(82686069));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 4 (82686069, 82686065)', async function () {
    const res = await this.contract4.eq_uint32_euint32(82686069, this.instances4.alice.encrypt32(82686065));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (3660714, 206743981)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(3660714), 206743981);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (3660710, 3660714)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(3660710), 3660714);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 3 (3660714, 3660714)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(3660714), 3660714);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 4 (3660714, 3660710)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(3660714), 3660710);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (223424023, 206743981)', async function () {
    const res = await this.contract4.ne_uint32_euint32(223424023, this.instances4.alice.encrypt32(206743981));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (3660710, 3660714)', async function () {
    const res = await this.contract4.ne_uint32_euint32(3660710, this.instances4.alice.encrypt32(3660714));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 3 (3660714, 3660714)', async function () {
    const res = await this.contract4.ne_uint32_euint32(3660714, this.instances4.alice.encrypt32(3660714));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 4 (3660714, 3660710)', async function () {
    const res = await this.contract4.ne_uint32_euint32(3660714, this.instances4.alice.encrypt32(3660710));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (182035180, 57241334)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(182035180), 57241334);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (102858483, 102858487)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(102858483), 102858487);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (102858487, 102858487)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(102858487), 102858487);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 4 (102858487, 102858483)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(102858487), 102858483);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (173909597, 57241334)', async function () {
    const res = await this.contract4.ge_uint32_euint32(173909597, this.instances4.alice.encrypt32(57241334));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (102858483, 102858487)', async function () {
    const res = await this.contract4.ge_uint32_euint32(102858483, this.instances4.alice.encrypt32(102858487));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (102858487, 102858487)', async function () {
    const res = await this.contract4.ge_uint32_euint32(102858487, this.instances4.alice.encrypt32(102858487));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (102858487, 102858483)', async function () {
    const res = await this.contract4.ge_uint32_euint32(102858487, this.instances4.alice.encrypt32(102858483));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (228583889, 207183969)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(228583889), 207183969);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (44696676, 44696680)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(44696676), 44696680);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (44696680, 44696680)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(44696680), 44696680);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 4 (44696680, 44696676)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(44696680), 44696676);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (51988955, 207183969)', async function () {
    const res = await this.contract4.gt_uint32_euint32(51988955, this.instances4.alice.encrypt32(207183969));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (44696676, 44696680)', async function () {
    const res = await this.contract4.gt_uint32_euint32(44696676, this.instances4.alice.encrypt32(44696680));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (44696680, 44696680)', async function () {
    const res = await this.contract4.gt_uint32_euint32(44696680, this.instances4.alice.encrypt32(44696680));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 4 (44696680, 44696676)', async function () {
    const res = await this.contract4.gt_uint32_euint32(44696680, this.instances4.alice.encrypt32(44696676));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (232635457, 92190924)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(232635457), 92190924);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (156819790, 156819794)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(156819790), 156819794);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (156819794, 156819794)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(156819794), 156819794);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 4 (156819794, 156819790)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(156819794), 156819790);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (239805571, 92190924)', async function () {
    const res = await this.contract4.le_uint32_euint32(239805571, this.instances4.alice.encrypt32(92190924));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (156819790, 156819794)', async function () {
    const res = await this.contract4.le_uint32_euint32(156819790, this.instances4.alice.encrypt32(156819794));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (156819794, 156819794)', async function () {
    const res = await this.contract4.le_uint32_euint32(156819794, this.instances4.alice.encrypt32(156819794));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 4 (156819794, 156819790)', async function () {
    const res = await this.contract4.le_uint32_euint32(156819794, this.instances4.alice.encrypt32(156819790));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (89546642, 53290899)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(89546642), 53290899);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (89546638, 89546642)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(89546638), 89546642);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (89546642, 89546642)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(89546642), 89546642);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 4 (89546642, 89546638)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(89546642), 89546638);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (87828731, 53290899)', async function () {
    const res = await this.contract4.lt_uint32_euint32(87828731, this.instances4.alice.encrypt32(53290899));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (89546638, 89546642)', async function () {
    const res = await this.contract4.lt_uint32_euint32(89546638, this.instances4.alice.encrypt32(89546642));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (89546642, 89546642)', async function () {
    const res = await this.contract4.lt_uint32_euint32(89546642, this.instances4.alice.encrypt32(89546642));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 4 (89546642, 89546638)', async function () {
    const res = await this.contract4.lt_uint32_euint32(89546642, this.instances4.alice.encrypt32(89546638));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (58829340, 90321027)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(58829340), 90321027);
    expect(res).to.equal(58829340);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (58829336, 58829340)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(58829336), 58829340);
    expect(res).to.equal(58829336);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (58829340, 58829340)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(58829340), 58829340);
    expect(res).to.equal(58829340);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 4 (58829340, 58829336)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(58829340), 58829336);
    expect(res).to.equal(58829336);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (20594417, 90321027)', async function () {
    const res = await this.contract4.min_uint32_euint32(20594417, this.instances4.alice.encrypt32(90321027));
    expect(res).to.equal(20594417);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (58829336, 58829340)', async function () {
    const res = await this.contract4.min_uint32_euint32(58829336, this.instances4.alice.encrypt32(58829340));
    expect(res).to.equal(58829336);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (58829340, 58829340)', async function () {
    const res = await this.contract4.min_uint32_euint32(58829340, this.instances4.alice.encrypt32(58829340));
    expect(res).to.equal(58829340);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 4 (58829340, 58829336)', async function () {
    const res = await this.contract4.min_uint32_euint32(58829340, this.instances4.alice.encrypt32(58829336));
    expect(res).to.equal(58829336);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (8800615, 201659044)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(8800615), 201659044);
    expect(res).to.equal(201659044);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (8800611, 8800615)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(8800611), 8800615);
    expect(res).to.equal(8800615);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (8800615, 8800615)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(8800615), 8800615);
    expect(res).to.equal(8800615);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 4 (8800615, 8800611)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(8800615), 8800611);
    expect(res).to.equal(8800615);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (220335343, 201659044)', async function () {
    const res = await this.contract4.max_uint32_euint32(220335343, this.instances4.alice.encrypt32(201659044));
    expect(res).to.equal(220335343);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (8800611, 8800615)', async function () {
    const res = await this.contract4.max_uint32_euint32(8800611, this.instances4.alice.encrypt32(8800615));
    expect(res).to.equal(8800615);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (8800615, 8800615)', async function () {
    const res = await this.contract4.max_uint32_euint32(8800615, this.instances4.alice.encrypt32(8800615));
    expect(res).to.equal(8800615);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 4 (8800615, 8800611)', async function () {
    const res = await this.contract4.max_uint32_euint32(8800615, this.instances4.alice.encrypt32(8800611));
    expect(res).to.equal(8800615);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 1 (13, 1)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(13),
      this.instances4.alice.encrypt4(1),
    );
    expect(res).to.equal(14);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 2 (3, 5)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(3),
      this.instances4.alice.encrypt4(5),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 3 (5, 5)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(5),
      this.instances4.alice.encrypt4(5),
    );
    expect(res).to.equal(10);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 4 (5, 3)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(5),
      this.instances4.alice.encrypt4(3),
    );
    expect(res).to.equal(8);
  });

  it('test operator "sub" overload (euint64, euint4) => euint64 test 1 (13, 13)', async function () {
    const res = await this.contract4.sub_euint64_euint4(
      this.instances4.alice.encrypt64(13),
      this.instances4.alice.encrypt4(13),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, euint4) => euint64 test 2 (13, 9)', async function () {
    const res = await this.contract4.sub_euint64_euint4(
      this.instances4.alice.encrypt64(13),
      this.instances4.alice.encrypt4(9),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 1 (11, 1)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(11),
      this.instances4.alice.encrypt4(1),
    );
    expect(res).to.equal(11);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 2 (3, 5)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(3),
      this.instances4.alice.encrypt4(5),
    );
    expect(res).to.equal(15);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 3 (2, 2)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(2),
      this.instances4.alice.encrypt4(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 4 (5, 3)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(5),
      this.instances4.alice.encrypt4(3),
    );
    expect(res).to.equal(15);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 1 (190497921, 5)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(190497921),
      this.instances4.alice.encrypt4(5),
    );
    expect(res).to.equal(1);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 1 (144995790, 3)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(144995790),
      this.instances4.alice.encrypt4(3),
    );
    expect(res).to.equal(144995791);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 1 (236438189, 1)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(236438189),
      this.instances4.alice.encrypt4(1),
    );
    expect(res).to.equal(236438188);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 1 (173975280, 4)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(173975280),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 1 (86198263, 13)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(86198263),
      this.instances4.alice.encrypt4(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(9),
      this.instances4.alice.encrypt4(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(13),
      this.instances4.alice.encrypt4(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(13),
      this.instances4.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 1 (105082351, 8)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(105082351),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 1 (264058266, 10)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(264058266),
      this.instances4.alice.encrypt4(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(6),
      this.instances4.alice.encrypt4(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt4(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt4(6),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 1 (4853853, 7)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(4853853),
      this.instances4.alice.encrypt4(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 1 (109092109, 11)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(109092109),
      this.instances4.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(7),
      this.instances4.alice.encrypt4(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(11),
      this.instances4.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(11),
      this.instances4.alice.encrypt4(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 1 (244314733, 7)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(244314733),
      this.instances4.alice.encrypt4(7),
    );
    expect(res).to.equal(7);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 1 (18255176, 3)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(18255176),
      this.instances4.alice.encrypt4(3),
    );
    expect(res).to.equal(18255176);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 1 (222, 1)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(222),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(223);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 2 (78, 82)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(78),
      this.instances4.alice.encrypt8(82),
    );
    expect(res).to.equal(160);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 3 (82, 82)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(82),
      this.instances4.alice.encrypt8(82),
    );
    expect(res).to.equal(164);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 4 (82, 78)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(82),
      this.instances4.alice.encrypt8(78),
    );
    expect(res).to.equal(160);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 1 (236, 236)', async function () {
    const res = await this.contract4.sub_euint64_euint8(
      this.instances4.alice.encrypt64(236),
      this.instances4.alice.encrypt8(236),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 2 (236, 232)', async function () {
    const res = await this.contract4.sub_euint64_euint8(
      this.instances4.alice.encrypt64(236),
      this.instances4.alice.encrypt8(232),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 1 (177, 1)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(177),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(177);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 2 (10, 10)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt8(10),
    );
    expect(res).to.equal(100);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 3 (10, 10)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt8(10),
    );
    expect(res).to.equal(100);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 4 (10, 10)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt8(10),
    );
    expect(res).to.equal(100);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 1 (190497921, 253)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(190497921),
      this.instances4.alice.encrypt8(253),
    );
    expect(res).to.equal(129);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 2 (249, 253)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(249),
      this.instances4.alice.encrypt8(253),
    );
    expect(res).to.equal(249);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 3 (253, 253)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(253),
      this.instances4.alice.encrypt8(253),
    );
    expect(res).to.equal(253);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 4 (253, 249)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(253),
      this.instances4.alice.encrypt8(249),
    );
    expect(res).to.equal(249);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 1 (144995790, 114)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(144995790),
      this.instances4.alice.encrypt8(114),
    );
    expect(res).to.equal(144995838);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 2 (110, 114)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(110),
      this.instances4.alice.encrypt8(114),
    );
    expect(res).to.equal(126);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 3 (114, 114)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(114),
      this.instances4.alice.encrypt8(114),
    );
    expect(res).to.equal(114);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 4 (114, 110)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(114),
      this.instances4.alice.encrypt8(110),
    );
    expect(res).to.equal(126);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 1 (236438189, 12)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(236438189),
      this.instances4.alice.encrypt8(12),
    );
    expect(res).to.equal(236438177);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 2 (8, 12)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(12),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 3 (12, 12)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(12),
      this.instances4.alice.encrypt8(12),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 4 (12, 8)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(12),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 1 (173975280, 29)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(173975280),
      this.instances4.alice.encrypt8(29),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 2 (25, 29)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(25),
      this.instances4.alice.encrypt8(29),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 3 (29, 29)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(29),
      this.instances4.alice.encrypt8(29),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 4 (29, 25)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(29),
      this.instances4.alice.encrypt8(25),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 1 (86198263, 223)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(86198263),
      this.instances4.alice.encrypt8(223),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 2 (219, 223)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(219),
      this.instances4.alice.encrypt8(223),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 3 (223, 223)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(223),
      this.instances4.alice.encrypt8(223),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 4 (223, 219)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(223),
      this.instances4.alice.encrypt8(219),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 1 (105082351, 165)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(105082351),
      this.instances4.alice.encrypt8(165),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 2 (161, 165)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(161),
      this.instances4.alice.encrypt8(165),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 3 (165, 165)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(165),
      this.instances4.alice.encrypt8(165),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 4 (165, 161)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(165),
      this.instances4.alice.encrypt8(161),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 1 (264058266, 147)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(264058266),
      this.instances4.alice.encrypt8(147),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 2 (143, 147)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(143),
      this.instances4.alice.encrypt8(147),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 3 (147, 147)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(147),
      this.instances4.alice.encrypt8(147),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 4 (147, 143)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(147),
      this.instances4.alice.encrypt8(143),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 1 (4853853, 164)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(4853853),
      this.instances5.alice.encrypt8(164),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 2 (160, 164)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(160),
      this.instances5.alice.encrypt8(164),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 3 (164, 164)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(164),
      this.instances5.alice.encrypt8(164),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 4 (164, 160)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(164),
      this.instances5.alice.encrypt8(160),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 1 (109092109, 64)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(109092109),
      this.instances5.alice.encrypt8(64),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 2 (60, 64)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(60),
      this.instances5.alice.encrypt8(64),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 3 (64, 64)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(64),
      this.instances5.alice.encrypt8(64),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 4 (64, 60)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(64),
      this.instances5.alice.encrypt8(60),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 1 (244314733, 93)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(244314733),
      this.instances5.alice.encrypt8(93),
    );
    expect(res).to.equal(93);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 2 (89, 93)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(89),
      this.instances5.alice.encrypt8(93),
    );
    expect(res).to.equal(89);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 3 (93, 93)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(93),
      this.instances5.alice.encrypt8(93),
    );
    expect(res).to.equal(93);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 4 (93, 89)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(93),
      this.instances5.alice.encrypt8(89),
    );
    expect(res).to.equal(89);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 1 (18255176, 147)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(18255176),
      this.instances5.alice.encrypt8(147),
    );
    expect(res).to.equal(18255176);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 2 (143, 147)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(143),
      this.instances5.alice.encrypt8(147),
    );
    expect(res).to.equal(147);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 3 (147, 147)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(147),
      this.instances5.alice.encrypt8(147),
    );
    expect(res).to.equal(147);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 4 (147, 143)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(147),
      this.instances5.alice.encrypt8(143),
    );
    expect(res).to.equal(147);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 1 (57062, 3)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(57062),
      this.instances5.alice.encrypt16(3),
    );
    expect(res).to.equal(57065);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 2 (16133, 16137)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(16133),
      this.instances5.alice.encrypt16(16137),
    );
    expect(res).to.equal(32270);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 3 (16137, 16137)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(16137),
      this.instances5.alice.encrypt16(16137),
    );
    expect(res).to.equal(32274);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 4 (16137, 16133)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(16137),
      this.instances5.alice.encrypt16(16133),
    );
    expect(res).to.equal(32270);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 1 (23608, 23608)', async function () {
    const res = await this.contract5.sub_euint64_euint16(
      this.instances5.alice.encrypt64(23608),
      this.instances5.alice.encrypt16(23608),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 2 (23608, 23604)', async function () {
    const res = await this.contract5.sub_euint64_euint16(
      this.instances5.alice.encrypt64(23608),
      this.instances5.alice.encrypt16(23604),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 1 (11381, 2)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(11381),
      this.instances5.alice.encrypt16(2),
    );
    expect(res).to.equal(22762);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 2 (150, 150)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(150),
      this.instances5.alice.encrypt16(150),
    );
    expect(res).to.equal(22500);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 3 (150, 150)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(150),
      this.instances5.alice.encrypt16(150),
    );
    expect(res).to.equal(22500);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 4 (150, 150)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(150),
      this.instances5.alice.encrypt16(150),
    );
    expect(res).to.equal(22500);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 1 (190497921, 60802)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(190497921),
      this.instances5.alice.encrypt16(60802),
    );
    expect(res).to.equal(50304);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 2 (60798, 60802)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(60798),
      this.instances5.alice.encrypt16(60802),
    );
    expect(res).to.equal(60674);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 3 (60802, 60802)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(60802),
      this.instances5.alice.encrypt16(60802),
    );
    expect(res).to.equal(60802);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 4 (60802, 60798)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(60802),
      this.instances5.alice.encrypt16(60798),
    );
    expect(res).to.equal(60674);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 1 (144995790, 62529)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(144995790),
      this.instances5.alice.encrypt16(62529),
    );
    expect(res).to.equal(145028559);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 2 (62525, 62529)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(62525),
      this.instances5.alice.encrypt16(62529),
    );
    expect(res).to.equal(62589);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 3 (62529, 62529)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(62529),
      this.instances5.alice.encrypt16(62529),
    );
    expect(res).to.equal(62529);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 4 (62529, 62525)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(62529),
      this.instances5.alice.encrypt16(62525),
    );
    expect(res).to.equal(62589);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 1 (236438189, 51285)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(236438189),
      this.instances5.alice.encrypt16(51285),
    );
    expect(res).to.equal(236391160);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 2 (51281, 51285)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(51281),
      this.instances5.alice.encrypt16(51285),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 3 (51285, 51285)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(51285),
      this.instances5.alice.encrypt16(51285),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 4 (51285, 51281)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(51285),
      this.instances5.alice.encrypt16(51281),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 1 (173975280, 43434)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(173975280),
      this.instances5.alice.encrypt16(43434),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 2 (43430, 43434)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(43430),
      this.instances5.alice.encrypt16(43434),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 3 (43434, 43434)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(43434),
      this.instances5.alice.encrypt16(43434),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 4 (43434, 43430)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(43434),
      this.instances5.alice.encrypt16(43430),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 1 (86198263, 29976)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(86198263),
      this.instances5.alice.encrypt16(29976),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 2 (29972, 29976)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(29972),
      this.instances5.alice.encrypt16(29976),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 3 (29976, 29976)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(29976),
      this.instances5.alice.encrypt16(29976),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 4 (29976, 29972)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(29976),
      this.instances5.alice.encrypt16(29972),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 1 (105082351, 16536)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(105082351),
      this.instances5.alice.encrypt16(16536),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 2 (16532, 16536)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(16532),
      this.instances5.alice.encrypt16(16536),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 3 (16536, 16536)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(16536),
      this.instances5.alice.encrypt16(16536),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 4 (16536, 16532)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(16536),
      this.instances5.alice.encrypt16(16532),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 1 (264058266, 4272)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(264058266),
      this.instances5.alice.encrypt16(4272),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 2 (4268, 4272)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(4268),
      this.instances5.alice.encrypt16(4272),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 3 (4272, 4272)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(4272),
      this.instances5.alice.encrypt16(4272),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 4 (4272, 4268)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(4272),
      this.instances5.alice.encrypt16(4268),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 1 (4853853, 22531)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(4853853),
      this.instances5.alice.encrypt16(22531),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 2 (22527, 22531)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(22527),
      this.instances5.alice.encrypt16(22531),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 3 (22531, 22531)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(22531),
      this.instances5.alice.encrypt16(22531),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 4 (22531, 22527)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(22531),
      this.instances5.alice.encrypt16(22527),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 1 (109092109, 63916)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(109092109),
      this.instances5.alice.encrypt16(63916),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 2 (63912, 63916)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(63912),
      this.instances5.alice.encrypt16(63916),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 3 (63916, 63916)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(63916),
      this.instances5.alice.encrypt16(63916),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 4 (63916, 63912)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(63916),
      this.instances5.alice.encrypt16(63912),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 1 (244314733, 6873)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(244314733),
      this.instances5.alice.encrypt16(6873),
    );
    expect(res).to.equal(6873);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 2 (6869, 6873)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(6869),
      this.instances5.alice.encrypt16(6873),
    );
    expect(res).to.equal(6869);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 3 (6873, 6873)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(6873),
      this.instances5.alice.encrypt16(6873),
    );
    expect(res).to.equal(6873);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 4 (6873, 6869)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(6873),
      this.instances5.alice.encrypt16(6869),
    );
    expect(res).to.equal(6869);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (18255176, 7223)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(18255176),
      this.instances5.alice.encrypt16(7223),
    );
    expect(res).to.equal(18255176);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (7219, 7223)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(7219),
      this.instances5.alice.encrypt16(7223),
    );
    expect(res).to.equal(7223);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (7223, 7223)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(7223),
      this.instances5.alice.encrypt16(7223),
    );
    expect(res).to.equal(7223);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (7223, 7219)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(7223),
      this.instances5.alice.encrypt16(7219),
    );
    expect(res).to.equal(7223);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 1 (233729261, 108243872)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(233729261),
      this.instances5.alice.encrypt32(108243872),
    );
    expect(res).to.equal(341973133);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 2 (108243868, 108243872)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(108243868),
      this.instances5.alice.encrypt32(108243872),
    );
    expect(res).to.equal(216487740);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 3 (108243872, 108243872)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(108243872),
      this.instances5.alice.encrypt32(108243872),
    );
    expect(res).to.equal(216487744);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 4 (108243872, 108243868)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(108243872),
      this.instances5.alice.encrypt32(108243868),
    );
    expect(res).to.equal(216487740);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 1 (279607, 279607)', async function () {
    const res = await this.contract5.sub_euint64_euint32(
      this.instances5.alice.encrypt64(279607),
      this.instances5.alice.encrypt32(279607),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 2 (279607, 279603)', async function () {
    const res = await this.contract5.sub_euint64_euint32(
      this.instances5.alice.encrypt64(279607),
      this.instances5.alice.encrypt32(279603),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 1 (91048, 15465)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(91048),
      this.instances5.alice.encrypt32(15465),
    );
    expect(res).to.equal(1408057320);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 2 (61860, 61860)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(61860),
      this.instances5.alice.encrypt32(61860),
    );
    expect(res).to.equal(3826659600);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 3 (61860, 61860)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(61860),
      this.instances5.alice.encrypt32(61860),
    );
    expect(res).to.equal(3826659600);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 4 (61860, 61860)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(61860),
      this.instances5.alice.encrypt32(61860),
    );
    expect(res).to.equal(3826659600);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 1 (190497921, 164161585)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(190497921),
      this.instances5.alice.encrypt32(164161585),
    );
    expect(res).to.equal(155762689);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 2 (164161581, 164161585)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(164161581),
      this.instances5.alice.encrypt32(164161585),
    );
    expect(res).to.equal(164161569);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 3 (164161585, 164161585)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(164161585),
      this.instances5.alice.encrypt32(164161585),
    );
    expect(res).to.equal(164161585);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 4 (164161585, 164161581)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(164161585),
      this.instances5.alice.encrypt32(164161581),
    );
    expect(res).to.equal(164161569);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 1 (144995790, 109170461)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(144995790),
      this.instances5.alice.encrypt32(109170461),
    );
    expect(res).to.equal(245759967);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 2 (109170457, 109170461)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(109170457),
      this.instances5.alice.encrypt32(109170461),
    );
    expect(res).to.equal(109170461);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 3 (109170461, 109170461)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(109170461),
      this.instances5.alice.encrypt32(109170461),
    );
    expect(res).to.equal(109170461);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 4 (109170461, 109170457)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(109170461),
      this.instances5.alice.encrypt32(109170457),
    );
    expect(res).to.equal(109170461);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 1 (236438189, 33113563)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(236438189),
      this.instances5.alice.encrypt32(33113563),
    );
    expect(res).to.equal(267290486);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 2 (33113559, 33113563)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(33113559),
      this.instances5.alice.encrypt32(33113563),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 3 (33113563, 33113563)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(33113563),
      this.instances5.alice.encrypt32(33113563),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 4 (33113563, 33113559)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(33113563),
      this.instances5.alice.encrypt32(33113559),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 1 (173975280, 95821853)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(173975280),
      this.instances5.alice.encrypt32(95821853),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 2 (95821849, 95821853)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(95821849),
      this.instances5.alice.encrypt32(95821853),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 3 (95821853, 95821853)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(95821853),
      this.instances5.alice.encrypt32(95821853),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 4 (95821853, 95821849)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(95821853),
      this.instances5.alice.encrypt32(95821849),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 1 (86198263, 155458175)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(86198263),
      this.instances5.alice.encrypt32(155458175),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 2 (86198259, 86198263)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(86198259),
      this.instances5.alice.encrypt32(86198263),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 3 (86198263, 86198263)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(86198263),
      this.instances5.alice.encrypt32(86198263),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 4 (86198263, 86198259)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(86198263),
      this.instances5.alice.encrypt32(86198259),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 1 (105082351, 181571496)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(105082351),
      this.instances5.alice.encrypt32(181571496),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 2 (105082347, 105082351)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(105082347),
      this.instances5.alice.encrypt32(105082351),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 3 (105082351, 105082351)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(105082351),
      this.instances5.alice.encrypt32(105082351),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 4 (105082351, 105082347)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(105082351),
      this.instances5.alice.encrypt32(105082347),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 1 (264058266, 243897804)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(264058266),
      this.instances5.alice.encrypt32(243897804),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 2 (243897800, 243897804)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(243897800),
      this.instances5.alice.encrypt32(243897804),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 3 (243897804, 243897804)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(243897804),
      this.instances5.alice.encrypt32(243897804),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 4 (243897804, 243897800)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(243897804),
      this.instances5.alice.encrypt32(243897800),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (4853853, 119615921)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(4853853),
      this.instances5.alice.encrypt32(119615921),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (4853849, 4853853)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(4853849),
      this.instances5.alice.encrypt32(4853853),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (4853853, 4853853)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(4853853),
      this.instances5.alice.encrypt32(4853853),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (4853853, 4853849)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(4853853),
      this.instances5.alice.encrypt32(4853849),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (109092109, 83205790)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(109092109),
      this.instances5.alice.encrypt32(83205790),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (83205786, 83205790)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(83205786),
      this.instances5.alice.encrypt32(83205790),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (83205790, 83205790)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(83205790),
      this.instances5.alice.encrypt32(83205790),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (83205790, 83205786)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(83205790),
      this.instances5.alice.encrypt32(83205786),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (244314733, 157421113)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(244314733),
      this.instances5.alice.encrypt32(157421113),
    );
    expect(res).to.equal(157421113);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (157421109, 157421113)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(157421109),
      this.instances5.alice.encrypt32(157421113),
    );
    expect(res).to.equal(157421109);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (157421113, 157421113)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(157421113),
      this.instances5.alice.encrypt32(157421113),
    );
    expect(res).to.equal(157421113);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (157421113, 157421109)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(157421113),
      this.instances5.alice.encrypt32(157421109),
    );
    expect(res).to.equal(157421109);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (18255176, 254973729)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(18255176),
      this.instances5.alice.encrypt32(254973729),
    );
    expect(res).to.equal(254973729);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (18255172, 18255176)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(18255172),
      this.instances5.alice.encrypt32(18255176),
    );
    expect(res).to.equal(18255176);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (18255176, 18255176)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(18255176),
      this.instances5.alice.encrypt32(18255176),
    );
    expect(res).to.equal(18255176);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (18255176, 18255172)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(18255176),
      this.instances5.alice.encrypt32(18255172),
    );
    expect(res).to.equal(18255176);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (233729261, 36139823)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(233729261),
      this.instances5.alice.encrypt64(36139823),
    );
    expect(res).to.equal(269869084);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (36139819, 36139823)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(36139819),
      this.instances5.alice.encrypt64(36139823),
    );
    expect(res).to.equal(72279642);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (36139823, 36139823)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(36139823),
      this.instances5.alice.encrypt64(36139823),
    );
    expect(res).to.equal(72279646);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (36139823, 36139819)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(36139823),
      this.instances5.alice.encrypt64(36139819),
    );
    expect(res).to.equal(72279642);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (279607, 279607)', async function () {
    const res = await this.contract5.sub_euint64_euint64(
      this.instances5.alice.encrypt64(279607),
      this.instances5.alice.encrypt64(279607),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (279607, 279603)', async function () {
    const res = await this.contract5.sub_euint64_euint64(
      this.instances5.alice.encrypt64(279607),
      this.instances5.alice.encrypt64(279603),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (186466494, 16277264)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(186466494),
      this.instances5.alice.encrypt64(16277264),
    );
    expect(res).to.equal(3035164349992416);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (16277260, 16277264)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(16277260),
      this.instances5.alice.encrypt64(16277264),
    );
    expect(res).to.equal(264949258216640);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (16277264, 16277264)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(16277264),
      this.instances5.alice.encrypt64(16277264),
    );
    expect(res).to.equal(264949323325696);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (16277264, 16277260)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(16277264),
      this.instances5.alice.encrypt64(16277260),
    );
    expect(res).to.equal(264949258216640);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (190497921, 34355274)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(190497921),
      this.instances5.alice.encrypt64(34355274),
    );
    expect(res).to.equal(34078720);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (34355270, 34355274)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(34355270),
      this.instances5.alice.encrypt64(34355274),
    );
    expect(res).to.equal(34355266);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (34355274, 34355274)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(34355274),
      this.instances5.alice.encrypt64(34355274),
    );
    expect(res).to.equal(34355274);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (34355274, 34355270)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(34355274),
      this.instances5.alice.encrypt64(34355270),
    );
    expect(res).to.equal(34355266);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 1 (144995790, 193337091)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(144995790),
      this.instances5.alice.encrypt64(193337091),
    );
    expect(res).to.equal(195459023);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 2 (144995786, 144995790)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(144995786),
      this.instances5.alice.encrypt64(144995790),
    );
    expect(res).to.equal(144995790);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 3 (144995790, 144995790)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(144995790),
      this.instances5.alice.encrypt64(144995790),
    );
    expect(res).to.equal(144995790);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 4 (144995790, 144995786)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(144995790),
      this.instances5.alice.encrypt64(144995786),
    );
    expect(res).to.equal(144995790);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 1 (236438189, 150524138)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(236438189),
      this.instances5.alice.encrypt64(150524138),
    );
    expect(res).to.equal(116331079);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 2 (150524134, 150524138)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(150524134),
      this.instances5.alice.encrypt64(150524138),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 3 (150524138, 150524138)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(150524138),
      this.instances5.alice.encrypt64(150524138),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 4 (150524138, 150524134)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(150524138),
      this.instances5.alice.encrypt64(150524134),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 1 (173975280, 53304343)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(173975280),
      this.instances5.alice.encrypt64(53304343),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 2 (53304339, 53304343)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(53304339),
      this.instances5.alice.encrypt64(53304343),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 3 (53304343, 53304343)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(53304343),
      this.instances5.alice.encrypt64(53304343),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 4 (53304343, 53304339)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(53304343),
      this.instances5.alice.encrypt64(53304339),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (86198263, 178805096)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(86198263),
      this.instances5.alice.encrypt64(178805096),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (86198259, 86198263)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(86198259),
      this.instances5.alice.encrypt64(86198263),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (86198263, 86198263)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(86198263),
      this.instances5.alice.encrypt64(86198263),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (86198263, 86198259)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(86198263),
      this.instances5.alice.encrypt64(86198259),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 1 (105082351, 33126302)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(105082351),
      this.instances5.alice.encrypt64(33126302),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 2 (33126298, 33126302)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(33126298),
      this.instances5.alice.encrypt64(33126302),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 3 (33126302, 33126302)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(33126302),
      this.instances5.alice.encrypt64(33126302),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 4 (33126302, 33126298)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(33126302),
      this.instances5.alice.encrypt64(33126298),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (264058266, 143096333)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(264058266),
      this.instances5.alice.encrypt64(143096333),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (143096329, 143096333)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(143096329),
      this.instances5.alice.encrypt64(143096333),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (143096333, 143096333)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(143096333),
      this.instances5.alice.encrypt64(143096333),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (143096333, 143096329)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(143096333),
      this.instances5.alice.encrypt64(143096329),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (4853853, 166520373)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(4853853),
      this.instances5.alice.encrypt64(166520373),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (4853849, 4853853)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(4853849),
      this.instances5.alice.encrypt64(4853853),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (4853853, 4853853)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(4853853),
      this.instances5.alice.encrypt64(4853853),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (4853853, 4853849)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(4853853),
      this.instances5.alice.encrypt64(4853849),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 1 (109092109, 141689285)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(109092109),
      this.instances5.alice.encrypt64(141689285),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 2 (109092105, 109092109)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(109092105),
      this.instances5.alice.encrypt64(109092109),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 3 (109092109, 109092109)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(109092109),
      this.instances5.alice.encrypt64(109092109),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 4 (109092109, 109092105)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(109092109),
      this.instances5.alice.encrypt64(109092105),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 1 (244314733, 181820547)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(244314733),
      this.instances5.alice.encrypt64(181820547),
    );
    expect(res).to.equal(181820547);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 2 (181820543, 181820547)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(181820543),
      this.instances5.alice.encrypt64(181820547),
    );
    expect(res).to.equal(181820543);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 3 (181820547, 181820547)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(181820547),
      this.instances5.alice.encrypt64(181820547),
    );
    expect(res).to.equal(181820547);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 4 (181820547, 181820543)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(181820547),
      this.instances5.alice.encrypt64(181820543),
    );
    expect(res).to.equal(181820543);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (18255176, 49337889)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18255176),
      this.instances5.alice.encrypt64(49337889),
    );
    expect(res).to.equal(49337889);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (18255172, 18255176)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18255172),
      this.instances5.alice.encrypt64(18255176),
    );
    expect(res).to.equal(18255176);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (18255176, 18255176)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18255176),
      this.instances5.alice.encrypt64(18255176),
    );
    expect(res).to.equal(18255176);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (18255176, 18255172)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18255176),
      this.instances5.alice.encrypt64(18255172),
    );
    expect(res).to.equal(18255176);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (233729261, 163957114)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(233729261), 163957114);
    expect(res).to.equal(397686375);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (36139819, 36139823)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(36139819), 36139823);
    expect(res).to.equal(72279642);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (36139823, 36139823)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(36139823), 36139823);
    expect(res).to.equal(72279646);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (36139823, 36139819)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(36139823), 36139819);
    expect(res).to.equal(72279642);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 1 (35664883, 163957114)', async function () {
    const res = await this.contract5.add_uint64_euint64(35664883, this.instances5.alice.encrypt64(163957114));
    expect(res).to.equal(199621997);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 2 (36139819, 36139823)', async function () {
    const res = await this.contract5.add_uint64_euint64(36139819, this.instances5.alice.encrypt64(36139823));
    expect(res).to.equal(72279642);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 3 (36139823, 36139823)', async function () {
    const res = await this.contract5.add_uint64_euint64(36139823, this.instances5.alice.encrypt64(36139823));
    expect(res).to.equal(72279646);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 4 (36139823, 36139819)', async function () {
    const res = await this.contract5.add_uint64_euint64(36139823, this.instances5.alice.encrypt64(36139819));
    expect(res).to.equal(72279642);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (279607, 279607)', async function () {
    const res = await this.contract5.sub_euint64_uint64(this.instances5.alice.encrypt64(279607), 279607);
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (279607, 279603)', async function () {
    const res = await this.contract5.sub_euint64_uint64(this.instances5.alice.encrypt64(279607), 279603);
    expect(res).to.equal(4);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (279607, 279607)', async function () {
    const res = await this.contract5.sub_uint64_euint64(279607, this.instances5.alice.encrypt64(279607));
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (279607, 279603)', async function () {
    const res = await this.contract5.sub_uint64_euint64(279607, this.instances5.alice.encrypt64(279603));
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (186466494, 244564286)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(186466494), 244564286);
    expect(res).to.equal(45603044968033280);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (16277260, 16277264)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(16277260), 16277264);
    expect(res).to.equal(264949258216640);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (16277264, 16277264)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(16277264), 16277264);
    expect(res).to.equal(264949323325696);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (16277264, 16277260)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(16277264), 16277260);
    expect(res).to.equal(264949258216640);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 1 (8323036, 244564286)', async function () {
    const res = await this.contract5.mul_uint64_euint64(8323036, this.instances5.alice.encrypt64(244564286));
    expect(res).to.equal(2035517356692296);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 2 (16277260, 16277264)', async function () {
    const res = await this.contract5.mul_uint64_euint64(16277260, this.instances5.alice.encrypt64(16277264));
    expect(res).to.equal(264949258216640);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 3 (16277264, 16277264)', async function () {
    const res = await this.contract5.mul_uint64_euint64(16277264, this.instances5.alice.encrypt64(16277264));
    expect(res).to.equal(264949323325696);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 4 (16277264, 16277260)', async function () {
    const res = await this.contract5.mul_uint64_euint64(16277264, this.instances5.alice.encrypt64(16277260));
    expect(res).to.equal(264949258216640);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (253358888, 98592501)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(253358888), 98592501);
    expect(res).to.equal(2);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (216552409, 216552413)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(216552409), 216552413);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (216552413, 216552413)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(216552413), 216552413);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (216552413, 216552409)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(216552413), 216552409);
    expect(res).to.equal(1);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 1 (188635834, 135019919)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(188635834), 135019919);
    expect(res).to.equal(53615915);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 2 (188635830, 188635834)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(188635830), 188635834);
    expect(res).to.equal(188635830);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 3 (188635834, 188635834)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(188635834), 188635834);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 4 (188635834, 188635830)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(188635834), 188635830);
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (173975280, 166663710)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(173975280), 166663710);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (53304339, 53304343)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(53304339), 53304343);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (53304343, 53304343)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(53304343), 53304343);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (53304343, 53304339)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(53304343), 53304339);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (122047770, 166663710)', async function () {
    const res = await this.contract5.eq_uint64_euint64(122047770, this.instances5.alice.encrypt64(166663710));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (53304339, 53304343)', async function () {
    const res = await this.contract5.eq_uint64_euint64(53304339, this.instances5.alice.encrypt64(53304343));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (53304343, 53304343)', async function () {
    const res = await this.contract5.eq_uint64_euint64(53304343, this.instances5.alice.encrypt64(53304343));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (53304343, 53304339)', async function () {
    const res = await this.contract5.eq_uint64_euint64(53304343, this.instances5.alice.encrypt64(53304339));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (86198263, 40246882)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(86198263), 40246882);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (86198259, 86198263)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(86198259), 86198263);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (86198263, 86198263)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(86198263), 86198263);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (86198263, 86198259)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(86198263), 86198259);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (167642465, 40246882)', async function () {
    const res = await this.contract5.ne_uint64_euint64(167642465, this.instances5.alice.encrypt64(40246882));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (86198259, 86198263)', async function () {
    const res = await this.contract5.ne_uint64_euint64(86198259, this.instances5.alice.encrypt64(86198263));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (86198263, 86198263)', async function () {
    const res = await this.contract5.ne_uint64_euint64(86198263, this.instances5.alice.encrypt64(86198263));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (86198263, 86198259)', async function () {
    const res = await this.contract5.ne_uint64_euint64(86198263, this.instances5.alice.encrypt64(86198259));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (105082351, 48493907)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(105082351), 48493907);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (33126298, 33126302)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(33126298), 33126302);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (33126302, 33126302)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(33126302), 33126302);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (33126302, 33126298)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(33126302), 33126298);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (322150, 48493907)', async function () {
    const res = await this.contract5.ge_uint64_euint64(322150, this.instances5.alice.encrypt64(48493907));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (33126298, 33126302)', async function () {
    const res = await this.contract5.ge_uint64_euint64(33126298, this.instances5.alice.encrypt64(33126302));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (33126302, 33126302)', async function () {
    const res = await this.contract5.ge_uint64_euint64(33126302, this.instances5.alice.encrypt64(33126302));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (33126302, 33126298)', async function () {
    const res = await this.contract5.ge_uint64_euint64(33126302, this.instances5.alice.encrypt64(33126298));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (264058266, 101035647)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(264058266), 101035647);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (143096329, 143096333)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(143096329), 143096333);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (143096333, 143096333)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(143096333), 143096333);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (143096333, 143096329)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(143096333), 143096329);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 1 (236395744, 101035647)', async function () {
    const res = await this.contract5.gt_uint64_euint64(236395744, this.instances5.alice.encrypt64(101035647));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 2 (143096329, 143096333)', async function () {
    const res = await this.contract5.gt_uint64_euint64(143096329, this.instances5.alice.encrypt64(143096333));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 3 (143096333, 143096333)', async function () {
    const res = await this.contract5.gt_uint64_euint64(143096333, this.instances5.alice.encrypt64(143096333));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 4 (143096333, 143096329)', async function () {
    const res = await this.contract5.gt_uint64_euint64(143096333, this.instances5.alice.encrypt64(143096329));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (4853853, 71100277)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(4853853), 71100277);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (4853849, 4853853)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(4853849), 4853853);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (4853853, 4853853)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(4853853), 4853853);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (4853853, 4853849)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(4853853), 4853849);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (111458198, 71100277)', async function () {
    const res = await this.contract5.le_uint64_euint64(111458198, this.instances5.alice.encrypt64(71100277));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (4853849, 4853853)', async function () {
    const res = await this.contract5.le_uint64_euint64(4853849, this.instances5.alice.encrypt64(4853853));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (4853853, 4853853)', async function () {
    const res = await this.contract5.le_uint64_euint64(4853853, this.instances5.alice.encrypt64(4853853));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (4853853, 4853849)', async function () {
    const res = await this.contract5.le_uint64_euint64(4853853, this.instances5.alice.encrypt64(4853849));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (109092109, 187709038)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(109092109), 187709038);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (109092105, 109092109)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(109092105), 109092109);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (109092109, 109092109)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(109092109), 109092109);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (109092109, 109092105)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(109092109), 109092105);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (191874287, 187709038)', async function () {
    const res = await this.contract5.lt_uint64_euint64(191874287, this.instances5.alice.encrypt64(187709038));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (109092105, 109092109)', async function () {
    const res = await this.contract5.lt_uint64_euint64(109092105, this.instances5.alice.encrypt64(109092109));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (109092109, 109092109)', async function () {
    const res = await this.contract5.lt_uint64_euint64(109092109, this.instances5.alice.encrypt64(109092109));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (109092109, 109092105)', async function () {
    const res = await this.contract5.lt_uint64_euint64(109092109, this.instances5.alice.encrypt64(109092105));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 1 (244314733, 105400651)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(244314733), 105400651);
    expect(res).to.equal(105400651);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 2 (181820543, 181820547)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(181820543), 181820547);
    expect(res).to.equal(181820543);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 3 (181820547, 181820547)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(181820547), 181820547);
    expect(res).to.equal(181820547);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 4 (181820547, 181820543)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(181820547), 181820543);
    expect(res).to.equal(181820543);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (92885905, 105400651)', async function () {
    const res = await this.contract5.min_uint64_euint64(92885905, this.instances5.alice.encrypt64(105400651));
    expect(res).to.equal(92885905);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (181820543, 181820547)', async function () {
    const res = await this.contract5.min_uint64_euint64(181820543, this.instances5.alice.encrypt64(181820547));
    expect(res).to.equal(181820543);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (181820547, 181820547)', async function () {
    const res = await this.contract5.min_uint64_euint64(181820547, this.instances5.alice.encrypt64(181820547));
    expect(res).to.equal(181820547);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (181820547, 181820543)', async function () {
    const res = await this.contract5.min_uint64_euint64(181820547, this.instances5.alice.encrypt64(181820543));
    expect(res).to.equal(181820543);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (18255176, 62769148)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(18255176), 62769148);
    expect(res).to.equal(62769148);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (18255172, 18255176)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(18255172), 18255176);
    expect(res).to.equal(18255176);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (18255176, 18255176)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(18255176), 18255176);
    expect(res).to.equal(18255176);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (18255176, 18255172)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(18255176), 18255172);
    expect(res).to.equal(18255176);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (31807553, 62769148)', async function () {
    const res = await this.contract5.max_uint64_euint64(31807553, this.instances5.alice.encrypt64(62769148));
    expect(res).to.equal(62769148);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18255172, 18255176)', async function () {
    const res = await this.contract5.max_uint64_euint64(18255172, this.instances5.alice.encrypt64(18255176));
    expect(res).to.equal(18255176);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18255176, 18255176)', async function () {
    const res = await this.contract5.max_uint64_euint64(18255176, this.instances5.alice.encrypt64(18255176));
    expect(res).to.equal(18255176);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18255176, 18255172)', async function () {
    const res = await this.contract5.max_uint64_euint64(18255176, this.instances5.alice.encrypt64(18255172));
    expect(res).to.equal(18255176);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 1 (1, 7)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(1), 7);
    expect(res).to.equal(8);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(4), 8);
    expect(res).to.equal(4);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(8), 8);
    expect(res).to.equal(8);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(8), 4);
    expect(res).to.equal(8);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 1 (6, 5)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(6), 5);
    expect(res).to.equal(3);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(4), 8);
    expect(res).to.equal(4);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(8), 8);
    expect(res).to.equal(8);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(8), 4);
    expect(res).to.equal(8);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (89, 1)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(89),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(178);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(128);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (89, 1)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(89), 1);
    expect(res).to.equal(178);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(4), 8);
    expect(res).to.equal(4);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(8), 8);
    expect(res).to.equal(8);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(8), 4);
    expect(res).to.equal(128);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (83, 5)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(83),
      this.instances5.alice.encrypt8(5),
    );
    expect(res).to.equal(2);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (83, 5)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(83), 5);
    expect(res).to.equal(2);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(4), 8);
    expect(res).to.equal(4);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(8), 8);
    expect(res).to.equal(8);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(8), 4);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (52308, 7)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(52308),
      this.instances5.alice.encrypt8(7),
    );
    expect(res).to.equal(10752);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(1024);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(2048);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(128);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 1 (52308, 7)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(52308), 7);
    expect(res).to.equal(10752);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(4), 8);
    expect(res).to.equal(1024);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(8), 8);
    expect(res).to.equal(2048);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(8), 4);
    expect(res).to.equal(128);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (60963, 1)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(60963),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(30481);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (60963, 1)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(60963), 1);
    expect(res).to.equal(30481);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(4), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(8), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(8), 4);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (136621573, 1)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(136621573),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(273243146);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(1024);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(2048);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(128);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 1 (136621573, 1)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(136621573), 1);
    expect(res).to.equal(273243146);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(4), 8);
    expect(res).to.equal(1024);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(8), 8);
    expect(res).to.equal(2048);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(8), 4);
    expect(res).to.equal(128);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (157098209, 6)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(157098209),
      this.instances5.alice.encrypt8(6),
    );
    expect(res).to.equal(2454659);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 1 (157098209, 6)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(157098209), 6);
    expect(res).to.equal(2454659);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(4), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(8), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(8), 4);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 1 (163054538, 3)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(163054538),
      this.instances5.alice.encrypt8(3),
    );
    expect(res).to.equal(1304436304);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(1024);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(2048);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(128);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (163054538, 3)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(163054538), 3);
    expect(res).to.equal(1304436304);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(4), 8);
    expect(res).to.equal(1024);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(8), 8);
    expect(res).to.equal(2048);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(8), 4);
    expect(res).to.equal(128);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (151463759, 7)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(151463759),
      this.instances5.alice.encrypt8(7),
    );
    expect(res).to.equal(1183310);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (151463759, 7)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(151463759), 7);
    expect(res).to.equal(1183310);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(4), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(8), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(8), 4);
    expect(res).to.equal(0);
  });

  it('test operator "neg" overload (euint4) => euint4 test 1 (11)', async function () {
    const res = await this.contract5.neg_euint4(this.instances5.alice.encrypt4(11));
    expect(res).to.equal(5n);
  });

  it('test operator "not" overload (euint4) => euint4 test 1 (9)', async function () {
    const res = await this.contract5.not_euint4(this.instances5.alice.encrypt4(9));
    expect(res).to.equal(6n);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (177)', async function () {
    const res = await this.contract5.neg_euint8(this.instances5.alice.encrypt8(177));
    expect(res).to.equal(79n);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (30)', async function () {
    const res = await this.contract5.not_euint8(this.instances5.alice.encrypt8(30));
    expect(res).to.equal(225n);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (26575)', async function () {
    const res = await this.contract5.neg_euint16(this.instances5.alice.encrypt16(26575));
    expect(res).to.equal(38961n);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (15626)', async function () {
    const res = await this.contract5.not_euint16(this.instances5.alice.encrypt16(15626));
    expect(res).to.equal(49909n);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (204914882)', async function () {
    const res = await this.contract5.neg_euint32(this.instances5.alice.encrypt32(204914882));
    expect(res).to.equal(4090052414n);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (22116422)', async function () {
    const res = await this.contract5.not_euint32(this.instances5.alice.encrypt32(22116422));
    expect(res).to.equal(4272850873n);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (39253840)', async function () {
    const res = await this.contract5.neg_euint64(this.instances5.alice.encrypt64(39253840));
    expect(res).to.equal(18446744073670297776n);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (172305424)', async function () {
    const res = await this.contract5.not_euint64(this.instances5.alice.encrypt64(172305424));
    expect(res).to.equal(18446744073537246191n);
  });
});
