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

  it('test operator "add" overload (euint4, euint4) => euint4 test 1 (5, 3)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 3 (4, 4)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "sub" overload (euint4, euint4) => euint4 test 1 (9, 9)', async function () {
    const res = await this.contract1.sub_euint4_euint4(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint4) => euint4 test 2 (9, 5)', async function () {
    const res = await this.contract1.sub_euint4_euint4(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 1 (1, 4)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 2 (2, 4)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 4 (4, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 1 (6, 2)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(2);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 1 (5, 9)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 1 (10, 9)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(3);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 2 (5, 9)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 3 (9, 9)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 4 (9, 5)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 1 (11, 12)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt4(12),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt4(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt4(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 1 (2, 13)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt4(13),
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

  it('test operator "ge" overload (euint4, euint4) => ebool test 1 (7, 14)', async function () {
    const res = await this.contract1.ge_euint4_euint4(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt4(14),
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

  it('test operator "gt" overload (euint4, euint4) => ebool test 1 (6, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
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

  it('test operator "le" overload (euint4, euint4) => ebool test 1 (6, 9)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
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

  it('test operator "lt" overload (euint4, euint4) => ebool test 1 (4, 6)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(6),
    );
    expect(res).to.equal(true);
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

  it('test operator "min" overload (euint4, euint4) => euint4 test 1 (6, 11)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt4(11),
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

  it('test operator "max" overload (euint4, euint4) => euint4 test 1 (1, 12)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt4(12),
    );
    expect(res).to.equal(12);
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

  it('test operator "add" overload (euint4, euint8) => euint8 test 1 (1, 7)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(7),
    );
    expect(res).to.equal(8);
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

  it('test operator "sub" overload (euint4, euint8) => euint8 test 1 (12, 12)', async function () {
    const res = await this.contract1.sub_euint4_euint8(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt8(12),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint8) => euint8 test 2 (12, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint8(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 1 (1, 8)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 2 (2, 4)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt8(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 4 (4, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(2),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 1 (6, 41)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt8(41),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 1 (1, 13)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 1 (5, 12)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt8(12),
    );
    expect(res).to.equal(9);
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

  it('test operator "eq" overload (euint4, euint8) => ebool test 1 (11, 134)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(134),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt8(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 1 (2, 86)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt8(86),
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

  it('test operator "ge" overload (euint4, euint8) => ebool test 1 (7, 148)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt8(148),
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

  it('test operator "gt" overload (euint4, euint8) => ebool test 1 (6, 250)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt8(250),
    );
    expect(res).to.equal(false);
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

  it('test operator "le" overload (euint4, euint8) => ebool test 1 (6, 104)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt8(104),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 1 (4, 236)', async function () {
    const res = await this.contract1.lt_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(236),
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

  it('test operator "min" overload (euint4, euint8) => euint8 test 1 (6, 1)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt8(1),
    );
    expect(res).to.equal(1);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 1 (1, 8)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 1 (1, 14)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(14),
    );
    expect(res).to.equal(15);
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

  it('test operator "sub" overload (euint4, euint16) => euint16 test 1 (12, 12)', async function () {
    const res = await this.contract1.sub_euint4_euint16(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt16(12),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint16) => euint16 test 2 (12, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint16(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 1 (1, 8)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 2 (2, 4)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt16(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 4 (4, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(2),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 1 (6, 28819)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt16(28819),
    );
    expect(res).to.equal(2);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 1 (1, 12)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(12),
    );
    expect(res).to.equal(13);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 1 (1, 15)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(15),
    );
    expect(res).to.equal(14);
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

  it('test operator "eq" overload (euint4, euint16) => ebool test 1 (11, 56998)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt16(56998),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt16(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt16(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt16(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 1 (2, 15611)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt16(15611),
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

  it('test operator "ge" overload (euint4, euint16) => ebool test 1 (7, 22312)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt16(22312),
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

  it('test operator "gt" overload (euint4, euint16) => ebool test 1 (6, 26898)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt16(26898),
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

  it('test operator "le" overload (euint4, euint16) => ebool test 1 (6, 39475)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt16(39475),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 1 (4, 31822)', async function () {
    const res = await this.contract1.lt_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(31822),
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

  it('test operator "min" overload (euint4, euint16) => euint16 test 1 (6, 45682)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt16(45682),
    );
    expect(res).to.equal(6);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 1 (1, 14)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 1 (1, 10)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(10),
    );
    expect(res).to.equal(11);
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

  it('test operator "sub" overload (euint4, euint32) => euint32 test 1 (12, 12)', async function () {
    const res = await this.contract1.sub_euint4_euint32(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt32(12),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint32) => euint32 test 2 (12, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint32(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt32(8),
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

  it('test operator "mul" overload (euint4, euint32) => euint32 test 2 (2, 4)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt32(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 4 (4, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(2),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 1 (6, 159427402)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt32(159427402),
    );
    expect(res).to.equal(2);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 1 (1, 11)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 1 (1, 15)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(15),
    );
    expect(res).to.equal(14);
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

  it('test operator "eq" overload (euint4, euint32) => ebool test 1 (11, 991467756)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt32(991467756),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt32(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt32(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt32(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 1 (2, 2115089145)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt32(2115089145),
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

  it('test operator "ge" overload (euint4, euint32) => ebool test 1 (7, 2084141274)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt32(2084141274),
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

  it('test operator "gt" overload (euint4, euint32) => ebool test 1 (6, 1743193522)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt32(1743193522),
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

  it('test operator "le" overload (euint4, euint32) => ebool test 1 (6, 204501643)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt32(204501643),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 1 (4, 329229639)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(329229639),
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

  it('test operator "min" overload (euint4, euint32) => euint32 test 1 (6, 629182656)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt32(629182656),
    );
    expect(res).to.equal(6);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 1 (1, 11)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(8);
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

  it('test operator "sub" overload (euint4, euint64) => euint64 test 1 (12, 12)', async function () {
    const res = await this.contract1.sub_euint4_euint64(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt64(12),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint64) => euint64 test 2 (12, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint64(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 1 (1, 10)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(10);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 2 (2, 4)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt64(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 4 (4, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(2),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 1 (6, 698915120)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt64(698915120),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 1 (1, 10)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(11);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 1 (1, 11)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(11),
    );
    expect(res).to.equal(10);
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

  it('test operator "eq" overload (euint4, euint64) => ebool test 1 (11, 1242271547)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt64(1242271547),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt64(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt64(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt64(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 1 (2, 537819520)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt64(537819520),
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

  it('test operator "ge" overload (euint4, euint64) => ebool test 1 (7, 1017706709)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt64(1017706709),
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

  it('test operator "gt" overload (euint4, euint64) => ebool test 1 (6, 8303014)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt64(8303014),
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

  it('test operator "le" overload (euint4, euint64) => ebool test 1 (6, 102260366)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt64(102260366),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 1 (4, 1152793041)', async function () {
    const res = await this.contract1.lt_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(1152793041),
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

  it('test operator "min" overload (euint4, euint64) => euint64 test 1 (6, 1380027485)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt64(1380027485),
    );
    expect(res).to.equal(6);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 1 (1, 12)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(12),
    );
    expect(res).to.equal(12);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 1 (11, 1)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(11), 1);
    expect(res).to.equal(12);
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

  it('test operator "add" overload (uint8, euint4) => euint4 test 1 (8, 4)', async function () {
    const res = await this.contract1.add_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_uint8_euint4(4, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 3 (4, 4)', async function () {
    const res = await this.contract1.add_uint8_euint4(4, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(12);
  });

  it('test operator "sub" overload (euint4, uint8) => euint4 test 1 (12, 12)', async function () {
    const res = await this.contract1.sub_euint4_uint8(this.instances1.alice.encrypt4(12), 12);
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, uint8) => euint4 test 2 (12, 8)', async function () {
    const res = await this.contract1.sub_euint4_uint8(this.instances1.alice.encrypt4(12), 8);
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

  it('test operator "mul" overload (euint4, uint8) => euint4 test 1 (1, 7)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(1), 7);
    expect(res).to.equal(7);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 2 (2, 4)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(2), 4);
    expect(res).to.equal(8);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(2), 2);
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 4 (4, 2)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(4), 2);
    expect(res).to.equal(8);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 1 (6, 2)', async function () {
    const res = await this.contract1.mul_uint8_euint4(6, this.instances1.alice.encrypt4(2));
    expect(res).to.equal(12);
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

  it('test operator "div" overload (euint4, uint8) => euint4 test 1 (2, 8)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(2), 8);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(2);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 1 (12, 1)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(12), 1);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 2 (8, 12)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(8), 12);
    expect(res).to.equal(8);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 3 (12, 12)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(12), 12);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 4 (12, 8)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(12), 8);
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 1 (11, 3)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(11), 3);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(7), 11);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(11), 11);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(11), 7);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 1 (6, 2)', async function () {
    const res = await this.contract1.eq_uint8_euint4(6, this.instances1.alice.encrypt4(2));
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

  it('test operator "ne" overload (euint4, uint8) => ebool test 1 (2, 3)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(2), 3);
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

  it('test operator "ne" overload (uint8, euint4) => ebool test 1 (11, 13)', async function () {
    const res = await this.contract1.ne_uint8_euint4(11, this.instances1.alice.encrypt4(13));
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

  it('test operator "ge" overload (euint4, uint8) => ebool test 1 (7, 6)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(7), 6);
    expect(res).to.equal(true);
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

  it('test operator "ge" overload (uint8, euint4) => ebool test 1 (8, 3)', async function () {
    const res = await this.contract1.ge_uint8_euint4(8, this.instances1.alice.encrypt4(3));
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

  it('test operator "gt" overload (euint4, uint8) => ebool test 1 (6, 1)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(6), 1);
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

  it('test operator "gt" overload (uint8, euint4) => ebool test 1 (1, 9)', async function () {
    const res = await this.contract1.gt_uint8_euint4(1, this.instances1.alice.encrypt4(9));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.gt_uint8_euint4(5, this.instances1.alice.encrypt4(9));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.gt_uint8_euint4(9, this.instances1.alice.encrypt4(9));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.gt_uint8_euint4(9, this.instances1.alice.encrypt4(5));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 1 (6, 1)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(6), 1);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 1 (9, 9)', async function () {
    const res = await this.contract1.le_uint8_euint4(9, this.instances1.alice.encrypt4(9));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.le_uint8_euint4(5, this.instances1.alice.encrypt4(9));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.le_uint8_euint4(9, this.instances1.alice.encrypt4(9));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.le_uint8_euint4(9, this.instances1.alice.encrypt4(5));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 1 (4, 1)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(4), 1);
    expect(res).to.equal(false);
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

  it('test operator "lt" overload (uint8, euint4) => ebool test 1 (3, 8)', async function () {
    const res = await this.contract1.lt_uint8_euint4(3, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(true);
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

  it('test operator "min" overload (euint4, uint8) => euint4 test 1 (6, 3)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(6), 3);
    expect(res).to.equal(3);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 1 (11, 12)', async function () {
    const res = await this.contract1.min_uint8_euint4(11, this.instances1.alice.encrypt4(12));
    expect(res).to.equal(11);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 2 (8, 12)', async function () {
    const res = await this.contract1.min_uint8_euint4(8, this.instances1.alice.encrypt4(12));
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 3 (12, 12)', async function () {
    const res = await this.contract1.min_uint8_euint4(12, this.instances1.alice.encrypt4(12));
    expect(res).to.equal(12);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 4 (12, 8)', async function () {
    const res = await this.contract1.min_uint8_euint4(12, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 1 (1, 4)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(1), 4);
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 1 (2, 8)', async function () {
    const res = await this.contract1.max_uint8_euint4(2, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_uint8_euint4(4, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_uint8_euint4(8, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 1 (7, 1)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(7),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 3 (4, 4)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(4),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
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

  it('test operator "mul" overload (euint8, euint4) => euint8 test 1 (13, 1)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(13),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(13);
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

  it('test operator "and" overload (euint8, euint4) => euint8 test 1 (209, 7)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(209),
      this.instances1.alice.encrypt4(7),
    );
    expect(res).to.equal(1);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 1 (11, 1)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(11),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(11);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 1 (1, 11)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt4(11),
    );
    expect(res).to.equal(10);
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

  it('test operator "eq" overload (euint8, euint4) => ebool test 1 (59, 2)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(59),
      this.instances2.alice.encrypt4(2),
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

  it('test operator "ne" overload (euint8, euint4) => ebool test 1 (18, 13)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(18),
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

  it('test operator "ge" overload (euint8, euint4) => ebool test 1 (182, 3)', async function () {
    const res = await this.contract2.ge_euint8_euint4(
      this.instances2.alice.encrypt8(182),
      this.instances2.alice.encrypt4(3),
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

  it('test operator "gt" overload (euint8, euint4) => ebool test 1 (37, 9)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(37),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt4(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 1 (85, 9)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(85),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt4(5),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 1 (50, 8)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(50),
      this.instances2.alice.encrypt4(8),
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

  it('test operator "min" overload (euint8, euint4) => euint8 test 1 (79, 12)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt4(12),
    );
    expect(res).to.equal(12);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 2 (8, 12)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(12),
    );
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 3 (12, 12)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(12),
      this.instances2.alice.encrypt4(12),
    );
    expect(res).to.equal(12);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 4 (12, 8)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(12),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 1 (11, 8)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 1 (8, 73)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(73),
    );
    expect(res).to.equal(81);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(16);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 1 (14, 14)', async function () {
    const res = await this.contract2.sub_euint8_euint8(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt8(14),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 2 (14, 10)', async function () {
    const res = await this.contract2.sub_euint8_euint8(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt8(10),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 1 (6, 19)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(6),
      this.instances2.alice.encrypt8(19),
    );
    expect(res).to.equal(114);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 2 (8, 12)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(12),
    );
    expect(res).to.equal(96);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 3 (12, 12)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(12),
      this.instances2.alice.encrypt8(12),
    );
    expect(res).to.equal(144);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 4 (12, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(12),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(96);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 1 (209, 109)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(209),
      this.instances2.alice.encrypt8(109),
    );
    expect(res).to.equal(65);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 2 (105, 109)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(105),
      this.instances2.alice.encrypt8(109),
    );
    expect(res).to.equal(105);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 3 (109, 109)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(109),
      this.instances2.alice.encrypt8(109),
    );
    expect(res).to.equal(109);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 4 (109, 105)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(109),
      this.instances2.alice.encrypt8(105),
    );
    expect(res).to.equal(105);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 1 (93, 243)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt8(243),
    );
    expect(res).to.equal(255);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 2 (89, 93)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(89),
      this.instances2.alice.encrypt8(93),
    );
    expect(res).to.equal(93);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 3 (93, 93)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt8(93),
    );
    expect(res).to.equal(93);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 4 (93, 89)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt8(89),
    );
    expect(res).to.equal(93);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 1 (1, 166)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt8(166),
    );
    expect(res).to.equal(167);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 1 (6, 193)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(6),
      this.instances2.alice.encrypt8(193),
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

  it('test operator "ne" overload (euint8, euint8) => ebool test 1 (11, 49)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt8(49),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(7),
      this.instances2.alice.encrypt8(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt8(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt8(7),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 1 (8, 253)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(253),
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

  it('test operator "gt" overload (euint8, euint8) => ebool test 1 (1, 112)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt8(112),
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

  it('test operator "le" overload (euint8, euint8) => ebool test 1 (9, 136)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt8(136),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt8(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt8(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt8(5),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 1 (3, 82)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(3),
      this.instances2.alice.encrypt8(82),
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

  it('test operator "min" overload (euint8, euint8) => euint8 test 1 (11, 214)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt8(214),
    );
    expect(res).to.equal(11);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 2 (7, 11)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(7),
      this.instances2.alice.encrypt8(11),
    );
    expect(res).to.equal(7);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 3 (11, 11)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt8(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 4 (11, 7)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt8(7),
    );
    expect(res).to.equal(7);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 1 (2, 44)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt8(44),
    );
    expect(res).to.equal(44);
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

  it('test operator "add" overload (euint8, euint16) => euint16 test 1 (1, 251)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(251),
    );
    expect(res).to.equal(252);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(16);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 1 (62, 62)', async function () {
    const res = await this.contract2.sub_euint8_euint16(
      this.instances2.alice.encrypt8(62),
      this.instances2.alice.encrypt16(62),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 2 (62, 58)', async function () {
    const res = await this.contract2.sub_euint8_euint16(
      this.instances2.alice.encrypt8(62),
      this.instances2.alice.encrypt16(58),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 1 (2, 33)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt16(33),
    );
    expect(res).to.equal(66);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 2 (11, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt16(11),
    );
    expect(res).to.equal(121);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 3 (11, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt16(11),
    );
    expect(res).to.equal(121);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 4 (11, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt16(11),
    );
    expect(res).to.equal(121);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 1 (209, 33383)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(209),
      this.instances2.alice.encrypt16(33383),
    );
    expect(res).to.equal(65);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 2 (205, 209)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(205),
      this.instances2.alice.encrypt16(209),
    );
    expect(res).to.equal(193);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 3 (209, 209)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(209),
      this.instances2.alice.encrypt16(209),
    );
    expect(res).to.equal(209);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 4 (209, 205)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(209),
      this.instances2.alice.encrypt16(205),
    );
    expect(res).to.equal(193);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 1 (1, 196)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(196),
    );
    expect(res).to.equal(197);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 2 (89, 93)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(89),
      this.instances2.alice.encrypt16(93),
    );
    expect(res).to.equal(93);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 3 (93, 93)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt16(93),
    );
    expect(res).to.equal(93);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 4 (93, 89)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt16(89),
    );
    expect(res).to.equal(93);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 1 (1, 217)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(217),
    );
    expect(res).to.equal(216);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 1 (224, 28749)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(224),
      this.instances2.alice.encrypt16(28749),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 2 (220, 224)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(220),
      this.instances2.alice.encrypt16(224),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 3 (224, 224)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(224),
      this.instances2.alice.encrypt16(224),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 4 (224, 220)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(224),
      this.instances2.alice.encrypt16(220),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 1 (223, 38601)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(223),
      this.instances2.alice.encrypt16(38601),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 2 (219, 223)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(219),
      this.instances2.alice.encrypt16(223),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 3 (223, 223)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(223),
      this.instances2.alice.encrypt16(223),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 4 (223, 219)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(223),
      this.instances2.alice.encrypt16(219),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 1 (97, 54950)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(97),
      this.instances2.alice.encrypt16(54950),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 2 (93, 97)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt16(97),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 3 (97, 97)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(97),
      this.instances2.alice.encrypt16(97),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 4 (97, 93)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(97),
      this.instances2.alice.encrypt16(93),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 1 (79, 20669)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt16(20669),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 2 (75, 79)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(75),
      this.instances2.alice.encrypt16(79),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 3 (79, 79)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt16(79),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 4 (79, 75)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt16(75),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 1 (52, 17587)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(52),
      this.instances2.alice.encrypt16(17587),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 2 (48, 52)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(48),
      this.instances2.alice.encrypt16(52),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 3 (52, 52)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(52),
      this.instances2.alice.encrypt16(52),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 4 (52, 48)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(52),
      this.instances2.alice.encrypt16(48),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 1 (148, 35784)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(148),
      this.instances2.alice.encrypt16(35784),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 2 (144, 148)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(144),
      this.instances2.alice.encrypt16(148),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 3 (148, 148)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(148),
      this.instances2.alice.encrypt16(148),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 4 (148, 144)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(148),
      this.instances2.alice.encrypt16(144),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 1 (45, 12728)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(45),
      this.instances2.alice.encrypt16(12728),
    );
    expect(res).to.equal(45);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 2 (41, 45)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(41),
      this.instances2.alice.encrypt16(45),
    );
    expect(res).to.equal(41);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 3 (45, 45)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(45),
      this.instances2.alice.encrypt16(45),
    );
    expect(res).to.equal(45);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 4 (45, 41)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(45),
      this.instances2.alice.encrypt16(41),
    );
    expect(res).to.equal(41);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 1 (1, 173)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(173),
    );
    expect(res).to.equal(173);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 2 (174, 178)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(174),
      this.instances2.alice.encrypt16(178),
    );
    expect(res).to.equal(178);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 3 (178, 178)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(178),
      this.instances2.alice.encrypt16(178),
    );
    expect(res).to.equal(178);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 4 (178, 174)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(178),
      this.instances2.alice.encrypt16(174),
    );
    expect(res).to.equal(178);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 1 (1, 207)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(207),
    );
    expect(res).to.equal(208);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(16);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 1 (62, 62)', async function () {
    const res = await this.contract2.sub_euint8_euint32(
      this.instances2.alice.encrypt8(62),
      this.instances2.alice.encrypt32(62),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 2 (62, 58)', async function () {
    const res = await this.contract2.sub_euint8_euint32(
      this.instances2.alice.encrypt8(62),
      this.instances2.alice.encrypt32(58),
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

  it('test operator "mul" overload (euint8, euint32) => euint32 test 2 (11, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt32(11),
    );
    expect(res).to.equal(121);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 3 (11, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt32(11),
    );
    expect(res).to.equal(121);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 4 (11, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt32(11),
    );
    expect(res).to.equal(121);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 1 (209, 527165394)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(209),
      this.instances2.alice.encrypt32(527165394),
    );
    expect(res).to.equal(208);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 2 (205, 209)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(205),
      this.instances2.alice.encrypt32(209),
    );
    expect(res).to.equal(193);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 3 (209, 209)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(209),
      this.instances2.alice.encrypt32(209),
    );
    expect(res).to.equal(209);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 4 (209, 205)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(209),
      this.instances2.alice.encrypt32(205),
    );
    expect(res).to.equal(193);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 1 (1, 143)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(143),
    );
    expect(res).to.equal(143);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 2 (89, 93)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(89),
      this.instances2.alice.encrypt32(93),
    );
    expect(res).to.equal(93);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 3 (93, 93)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt32(93),
    );
    expect(res).to.equal(93);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 4 (93, 89)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt32(89),
    );
    expect(res).to.equal(93);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 1 (1, 186)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(186),
    );
    expect(res).to.equal(187);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 1 (224, 1978797433)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(224),
      this.instances2.alice.encrypt32(1978797433),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 2 (220, 224)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(220),
      this.instances2.alice.encrypt32(224),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 3 (224, 224)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(224),
      this.instances2.alice.encrypt32(224),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 4 (224, 220)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(224),
      this.instances2.alice.encrypt32(220),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 1 (223, 19817588)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(223),
      this.instances2.alice.encrypt32(19817588),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 2 (219, 223)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(219),
      this.instances2.alice.encrypt32(223),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 3 (223, 223)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(223),
      this.instances2.alice.encrypt32(223),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 4 (223, 219)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(223),
      this.instances2.alice.encrypt32(219),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 1 (97, 1858963305)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(97),
      this.instances2.alice.encrypt32(1858963305),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 2 (93, 97)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt32(97),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 3 (97, 97)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(97),
      this.instances2.alice.encrypt32(97),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 4 (97, 93)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(97),
      this.instances2.alice.encrypt32(93),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 1 (79, 2069042933)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt32(2069042933),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 2 (75, 79)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(75),
      this.instances2.alice.encrypt32(79),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 3 (79, 79)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt32(79),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 4 (79, 75)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt32(75),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 1 (52, 1808707750)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(52),
      this.instances2.alice.encrypt32(1808707750),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 2 (48, 52)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(48),
      this.instances2.alice.encrypt32(52),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 3 (52, 52)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(52),
      this.instances2.alice.encrypt32(52),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 4 (52, 48)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(52),
      this.instances2.alice.encrypt32(48),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 1 (148, 1735514833)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(148),
      this.instances2.alice.encrypt32(1735514833),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 2 (144, 148)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(144),
      this.instances2.alice.encrypt32(148),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 3 (148, 148)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(148),
      this.instances2.alice.encrypt32(148),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 4 (148, 144)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(148),
      this.instances2.alice.encrypt32(144),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 1 (45, 1572730534)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(45),
      this.instances2.alice.encrypt32(1572730534),
    );
    expect(res).to.equal(45);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 2 (41, 45)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(41),
      this.instances2.alice.encrypt32(45),
    );
    expect(res).to.equal(41);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 3 (45, 45)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(45),
      this.instances2.alice.encrypt32(45),
    );
    expect(res).to.equal(45);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 4 (45, 41)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(45),
      this.instances2.alice.encrypt32(41),
    );
    expect(res).to.equal(41);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 1 (1, 176)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(176),
    );
    expect(res).to.equal(176);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 2 (174, 178)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(174),
      this.instances2.alice.encrypt32(178),
    );
    expect(res).to.equal(178);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 3 (178, 178)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(178),
      this.instances2.alice.encrypt32(178),
    );
    expect(res).to.equal(178);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 4 (178, 174)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(178),
      this.instances2.alice.encrypt32(174),
    );
    expect(res).to.equal(178);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 1 (1, 212)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(212),
    );
    expect(res).to.equal(213);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(16);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 1 (62, 62)', async function () {
    const res = await this.contract2.sub_euint8_euint64(
      this.instances2.alice.encrypt8(62),
      this.instances2.alice.encrypt64(62),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 2 (62, 58)', async function () {
    const res = await this.contract2.sub_euint8_euint64(
      this.instances2.alice.encrypt8(62),
      this.instances2.alice.encrypt64(58),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 1 (1, 145)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(145),
    );
    expect(res).to.equal(145);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 2 (11, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt64(11),
    );
    expect(res).to.equal(121);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 3 (11, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt64(11),
    );
    expect(res).to.equal(121);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 4 (11, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt64(11),
    );
    expect(res).to.equal(121);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 1 (209, 1641438334)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(209),
      this.instances2.alice.encrypt64(1641438334),
    );
    expect(res).to.equal(80);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 2 (205, 209)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(205),
      this.instances2.alice.encrypt64(209),
    );
    expect(res).to.equal(193);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 3 (209, 209)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(209),
      this.instances2.alice.encrypt64(209),
    );
    expect(res).to.equal(209);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 4 (209, 205)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(209),
      this.instances2.alice.encrypt64(205),
    );
    expect(res).to.equal(193);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 1 (1, 139)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(139),
    );
    expect(res).to.equal(139);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 2 (89, 93)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(89),
      this.instances2.alice.encrypt64(93),
    );
    expect(res).to.equal(93);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 3 (93, 93)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt64(93),
    );
    expect(res).to.equal(93);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 4 (93, 89)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt64(89),
    );
    expect(res).to.equal(93);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 1 (1, 136)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(136),
    );
    expect(res).to.equal(137);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 1 (224, 1900324216)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(224),
      this.instances2.alice.encrypt64(1900324216),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 2 (220, 224)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(220),
      this.instances2.alice.encrypt64(224),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 3 (224, 224)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(224),
      this.instances2.alice.encrypt64(224),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 4 (224, 220)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(224),
      this.instances2.alice.encrypt64(220),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 1 (223, 1747152964)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(223),
      this.instances2.alice.encrypt64(1747152964),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 2 (219, 223)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(219),
      this.instances2.alice.encrypt64(223),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 3 (223, 223)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(223),
      this.instances2.alice.encrypt64(223),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 4 (223, 219)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(223),
      this.instances2.alice.encrypt64(219),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 1 (97, 24079369)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(97),
      this.instances2.alice.encrypt64(24079369),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 2 (93, 97)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt64(97),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 3 (97, 97)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(97),
      this.instances2.alice.encrypt64(97),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 4 (97, 93)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(97),
      this.instances2.alice.encrypt64(93),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 1 (79, 1487732246)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt64(1487732246),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 2 (75, 79)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(75),
      this.instances2.alice.encrypt64(79),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 3 (79, 79)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt64(79),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 4 (79, 75)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(79),
      this.instances2.alice.encrypt64(75),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 1 (52, 1397250357)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(52),
      this.instances2.alice.encrypt64(1397250357),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 2 (48, 52)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(48),
      this.instances2.alice.encrypt64(52),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 3 (52, 52)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(52),
      this.instances2.alice.encrypt64(52),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 4 (52, 48)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(52),
      this.instances2.alice.encrypt64(48),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 1 (148, 1575025918)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(148),
      this.instances2.alice.encrypt64(1575025918),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 2 (144, 148)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(144),
      this.instances2.alice.encrypt64(148),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 3 (148, 148)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(148),
      this.instances2.alice.encrypt64(148),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 4 (148, 144)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(148),
      this.instances2.alice.encrypt64(144),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 1 (45, 1376771886)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(45),
      this.instances2.alice.encrypt64(1376771886),
    );
    expect(res).to.equal(45);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 2 (41, 45)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(41),
      this.instances2.alice.encrypt64(45),
    );
    expect(res).to.equal(41);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 3 (45, 45)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(45),
      this.instances2.alice.encrypt64(45),
    );
    expect(res).to.equal(45);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 4 (45, 41)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(45),
      this.instances2.alice.encrypt64(41),
    );
    expect(res).to.equal(41);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 1 (1, 143)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(143),
    );
    expect(res).to.equal(143);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 2 (174, 178)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(174),
      this.instances2.alice.encrypt64(178),
    );
    expect(res).to.equal(178);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 3 (178, 178)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(178),
      this.instances2.alice.encrypt64(178),
    );
    expect(res).to.equal(178);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 4 (178, 174)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(178),
      this.instances2.alice.encrypt64(174),
    );
    expect(res).to.equal(178);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 1 (8, 136)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(8), 136);
    expect(res).to.equal(144);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(16);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 1 (1, 136)', async function () {
    const res = await this.contract2.add_uint8_euint8(1, this.instances2.alice.encrypt8(136));
    expect(res).to.equal(137);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.add_uint8_euint8(4, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.add_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(16);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.add_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(12);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 1 (14, 14)', async function () {
    const res = await this.contract2.sub_euint8_uint8(this.instances2.alice.encrypt8(14), 14);
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 2 (14, 10)', async function () {
    const res = await this.contract2.sub_euint8_uint8(this.instances2.alice.encrypt8(14), 10);
    expect(res).to.equal(4);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 1 (14, 14)', async function () {
    const res = await this.contract2.sub_uint8_euint8(14, this.instances2.alice.encrypt8(14));
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 2 (14, 10)', async function () {
    const res = await this.contract2.sub_uint8_euint8(14, this.instances2.alice.encrypt8(10));
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 1 (3, 48)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(3), 48);
    expect(res).to.equal(144);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 2 (8, 12)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(8), 12);
    expect(res).to.equal(96);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 3 (12, 12)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(12), 12);
    expect(res).to.equal(144);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 4 (12, 8)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(12), 8);
    expect(res).to.equal(96);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 1 (11, 12)', async function () {
    const res = await this.contract2.mul_uint8_euint8(11, this.instances2.alice.encrypt8(12));
    expect(res).to.equal(132);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 2 (8, 12)', async function () {
    const res = await this.contract2.mul_uint8_euint8(8, this.instances2.alice.encrypt8(12));
    expect(res).to.equal(96);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 3 (12, 12)', async function () {
    const res = await this.contract2.mul_uint8_euint8(12, this.instances2.alice.encrypt8(12));
    expect(res).to.equal(144);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 4 (12, 8)', async function () {
    const res = await this.contract2.mul_uint8_euint8(12, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(96);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 1 (209, 108)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(209), 108);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 2 (112, 116)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(112), 116);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 3 (116, 116)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(116), 116);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 4 (116, 112)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(116), 112);
    expect(res).to.equal(1);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 1 (242, 190)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(242), 190);
    expect(res).to.equal(52);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 2 (177, 181)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(177), 181);
    expect(res).to.equal(177);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 3 (181, 181)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(181), 181);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 4 (181, 177)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(181), 177);
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 1 (6, 35)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(6), 35);
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

  it('test operator "eq" overload (uint8, euint8) => ebool test 1 (224, 35)', async function () {
    const res = await this.contract2.eq_uint8_euint8(224, this.instances2.alice.encrypt8(35));
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

  it('test operator "ne" overload (euint8, uint8) => ebool test 1 (11, 2)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(11), 2);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(7), 11);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(11), 11);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(11), 7);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 1 (223, 2)', async function () {
    const res = await this.contract2.ne_uint8_euint8(223, this.instances2.alice.encrypt8(2));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract2.ne_uint8_euint8(7, this.instances2.alice.encrypt8(11));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract2.ne_uint8_euint8(11, this.instances2.alice.encrypt8(11));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract2.ne_uint8_euint8(11, this.instances2.alice.encrypt8(7));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (8, 73)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(8), 73);
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

  it('test operator "ge" overload (uint8, euint8) => ebool test 1 (97, 73)', async function () {
    const res = await this.contract2.ge_uint8_euint8(97, this.instances2.alice.encrypt8(73));
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

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (1, 127)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(1), 127);
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

  it('test operator "gt" overload (uint8, euint8) => ebool test 1 (79, 127)', async function () {
    const res = await this.contract2.gt_uint8_euint8(79, this.instances2.alice.encrypt8(127));
    expect(res).to.equal(false);
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

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (9, 180)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(9), 180);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(5), 9);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(9), 9);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(9), 5);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 1 (52, 180)', async function () {
    const res = await this.contract2.le_uint8_euint8(52, this.instances2.alice.encrypt8(180));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract2.le_uint8_euint8(5, this.instances2.alice.encrypt8(9));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract2.le_uint8_euint8(9, this.instances2.alice.encrypt8(9));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract2.le_uint8_euint8(9, this.instances2.alice.encrypt8(5));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 1 (3, 223)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(3), 223);
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

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (148, 223)', async function () {
    const res = await this.contract2.lt_uint8_euint8(148, this.instances2.alice.encrypt8(223));
    expect(res).to.equal(true);
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

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (11, 200)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(11), 200);
    expect(res).to.equal(11);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (7, 11)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(7), 11);
    expect(res).to.equal(7);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (11, 11)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(11), 11);
    expect(res).to.equal(11);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 4 (11, 7)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(11), 7);
    expect(res).to.equal(7);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (45, 200)', async function () {
    const res = await this.contract2.min_uint8_euint8(45, this.instances2.alice.encrypt8(200));
    expect(res).to.equal(45);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 2 (7, 11)', async function () {
    const res = await this.contract2.min_uint8_euint8(7, this.instances2.alice.encrypt8(11));
    expect(res).to.equal(7);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 3 (11, 11)', async function () {
    const res = await this.contract2.min_uint8_euint8(11, this.instances2.alice.encrypt8(11));
    expect(res).to.equal(11);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 4 (11, 7)', async function () {
    const res = await this.contract2.min_uint8_euint8(11, this.instances2.alice.encrypt8(7));
    expect(res).to.equal(7);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (2, 123)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(2), 123);
    expect(res).to.equal(123);
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

  it('test operator "max" overload (uint8, euint8) => euint8 test 1 (178, 123)', async function () {
    const res = await this.contract2.max_uint8_euint8(178, this.instances2.alice.encrypt8(123));
    expect(res).to.equal(178);
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

  it('test operator "add" overload (euint16, euint4) => euint16 test 1 (14, 1)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(14),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(15);
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

  it('test operator "mul" overload (euint16, euint4) => euint16 test 1 (13, 1)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(13),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(13);
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

  it('test operator "and" overload (euint16, euint4) => euint16 test 1 (14563, 10)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(14563),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(2);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 2 (6, 10)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(6),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(2);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 3 (10, 10)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(10),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(10);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 4 (10, 6)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(10),
      this.instances2.alice.encrypt4(6),
    );
    expect(res).to.equal(2);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 1 (15, 1)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(15),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(15);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 1 (11, 1)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(11),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(10);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 1 (5627, 12)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(5627),
      this.instances2.alice.encrypt4(12),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(12),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(12),
      this.instances2.alice.encrypt4(12),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(12),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 1 (20601, 5)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(20601),
      this.instances2.alice.encrypt4(5),
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

  it('test operator "ge" overload (euint16, euint4) => ebool test 1 (44342, 8)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(44342),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 1 (42324, 1)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(42324),
      this.instances2.alice.encrypt4(1),
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

  it('test operator "le" overload (euint16, euint4) => ebool test 1 (5574, 6)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(5574),
      this.instances2.alice.encrypt4(6),
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

  it('test operator "lt" overload (euint16, euint4) => ebool test 1 (64394, 14)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(64394),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(10),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(14),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(14),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 1 (3854, 8)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(3854),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 1 (10, 1)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(10),
      this.instances3.alice.encrypt4(1),
    );
    expect(res).to.equal(10);
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

  it('test operator "add" overload (euint16, euint8) => euint16 test 1 (236, 1)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(236),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(237);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 2 (99, 101)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(99),
      this.instances3.alice.encrypt8(101),
    );
    expect(res).to.equal(200);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 3 (101, 101)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(101),
      this.instances3.alice.encrypt8(101),
    );
    expect(res).to.equal(202);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 4 (101, 99)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(101),
      this.instances3.alice.encrypt8(99),
    );
    expect(res).to.equal(200);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (100, 100)', async function () {
    const res = await this.contract3.sub_euint16_euint8(
      this.instances3.alice.encrypt16(100),
      this.instances3.alice.encrypt8(100),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (100, 96)', async function () {
    const res = await this.contract3.sub_euint16_euint8(
      this.instances3.alice.encrypt16(100),
      this.instances3.alice.encrypt8(96),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (221, 1)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(221),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(221);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 2 (12, 13)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(12),
      this.instances3.alice.encrypt8(13),
    );
    expect(res).to.equal(156);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 3 (13, 13)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(13),
      this.instances3.alice.encrypt8(13),
    );
    expect(res).to.equal(169);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 4 (13, 12)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(13),
      this.instances3.alice.encrypt8(12),
    );
    expect(res).to.equal(156);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 1 (14563, 200)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(14563),
      this.instances3.alice.encrypt8(200),
    );
    expect(res).to.equal(192);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 2 (196, 200)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(196),
      this.instances3.alice.encrypt8(200),
    );
    expect(res).to.equal(192);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 3 (200, 200)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(200),
      this.instances3.alice.encrypt8(200),
    );
    expect(res).to.equal(200);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 4 (200, 196)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(200),
      this.instances3.alice.encrypt8(196),
    );
    expect(res).to.equal(192);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 1 (241, 1)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(241),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(241);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 2 (120, 124)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(120),
      this.instances3.alice.encrypt8(124),
    );
    expect(res).to.equal(124);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 3 (124, 124)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(124),
      this.instances3.alice.encrypt8(124),
    );
    expect(res).to.equal(124);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 4 (124, 120)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(124),
      this.instances3.alice.encrypt8(120),
    );
    expect(res).to.equal(124);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (187, 1)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(187),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(186);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (25, 29)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(25),
      this.instances3.alice.encrypt8(29),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 3 (29, 29)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(29),
      this.instances3.alice.encrypt8(29),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 4 (29, 25)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(29),
      this.instances3.alice.encrypt8(25),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 1 (5627, 83)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(5627),
      this.instances3.alice.encrypt8(83),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 2 (79, 83)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(79),
      this.instances3.alice.encrypt8(83),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 3 (83, 83)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(83),
      this.instances3.alice.encrypt8(83),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 4 (83, 79)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(83),
      this.instances3.alice.encrypt8(79),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (20601, 251)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(20601),
      this.instances3.alice.encrypt8(251),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (247, 251)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(247),
      this.instances3.alice.encrypt8(251),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 3 (251, 251)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(251),
      this.instances3.alice.encrypt8(251),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 4 (251, 247)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(251),
      this.instances3.alice.encrypt8(247),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (44342, 190)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(44342),
      this.instances3.alice.encrypt8(190),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (186, 190)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(186),
      this.instances3.alice.encrypt8(190),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (190, 190)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(190),
      this.instances3.alice.encrypt8(190),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 4 (190, 186)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(190),
      this.instances3.alice.encrypt8(186),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 1 (42324, 76)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(42324),
      this.instances3.alice.encrypt8(76),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 2 (72, 76)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(72),
      this.instances3.alice.encrypt8(76),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 3 (76, 76)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(76),
      this.instances3.alice.encrypt8(76),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 4 (76, 72)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(76),
      this.instances3.alice.encrypt8(72),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 1 (5574, 242)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(5574),
      this.instances3.alice.encrypt8(242),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 2 (238, 242)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(238),
      this.instances3.alice.encrypt8(242),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 3 (242, 242)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(242),
      this.instances3.alice.encrypt8(242),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 4 (242, 238)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(242),
      this.instances3.alice.encrypt8(238),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 1 (64394, 92)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(64394),
      this.instances3.alice.encrypt8(92),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 2 (88, 92)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(88),
      this.instances3.alice.encrypt8(92),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 3 (92, 92)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(92),
      this.instances3.alice.encrypt8(92),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 4 (92, 88)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(92),
      this.instances3.alice.encrypt8(88),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 1 (3854, 166)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(3854),
      this.instances3.alice.encrypt8(166),
    );
    expect(res).to.equal(166);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 2 (162, 166)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(162),
      this.instances3.alice.encrypt8(166),
    );
    expect(res).to.equal(162);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 3 (166, 166)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(166),
      this.instances3.alice.encrypt8(166),
    );
    expect(res).to.equal(166);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 4 (166, 162)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(166),
      this.instances3.alice.encrypt8(162),
    );
    expect(res).to.equal(162);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 1 (168, 1)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(168),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(168);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 2 (62, 66)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(62),
      this.instances3.alice.encrypt8(66),
    );
    expect(res).to.equal(66);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 3 (66, 66)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(66),
      this.instances3.alice.encrypt8(66),
    );
    expect(res).to.equal(66);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 4 (66, 62)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(66),
      this.instances3.alice.encrypt8(62),
    );
    expect(res).to.equal(66);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 1 (30233, 31275)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(30233),
      this.instances3.alice.encrypt16(31275),
    );
    expect(res).to.equal(61508);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 2 (30229, 30233)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(30229),
      this.instances3.alice.encrypt16(30233),
    );
    expect(res).to.equal(60462);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 3 (30233, 30233)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(30233),
      this.instances3.alice.encrypt16(30233),
    );
    expect(res).to.equal(60466);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 4 (30233, 30229)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(30233),
      this.instances3.alice.encrypt16(30229),
    );
    expect(res).to.equal(60462);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 1 (7674, 7674)', async function () {
    const res = await this.contract3.sub_euint16_euint16(
      this.instances3.alice.encrypt16(7674),
      this.instances3.alice.encrypt16(7674),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 2 (7674, 7670)', async function () {
    const res = await this.contract3.sub_euint16_euint16(
      this.instances3.alice.encrypt16(7674),
      this.instances3.alice.encrypt16(7670),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 1 (221, 180)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(221),
      this.instances3.alice.encrypt16(180),
    );
    expect(res).to.equal(39780);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 2 (180, 180)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(180),
      this.instances3.alice.encrypt16(180),
    );
    expect(res).to.equal(32400);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 3 (180, 180)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(180),
      this.instances3.alice.encrypt16(180),
    );
    expect(res).to.equal(32400);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 4 (180, 180)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(180),
      this.instances3.alice.encrypt16(180),
    );
    expect(res).to.equal(32400);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 1 (14563, 4918)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(14563),
      this.instances3.alice.encrypt16(4918),
    );
    expect(res).to.equal(4130);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 2 (4914, 4918)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(4914),
      this.instances3.alice.encrypt16(4918),
    );
    expect(res).to.equal(4914);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 3 (4918, 4918)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(4918),
      this.instances3.alice.encrypt16(4918),
    );
    expect(res).to.equal(4918);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 4 (4918, 4914)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(4918),
      this.instances3.alice.encrypt16(4914),
    );
    expect(res).to.equal(4914);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 1 (15468, 53935)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(15468),
      this.instances3.alice.encrypt16(53935),
    );
    expect(res).to.equal(65263);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 2 (15464, 15468)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(15464),
      this.instances3.alice.encrypt16(15468),
    );
    expect(res).to.equal(15468);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 3 (15468, 15468)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(15468),
      this.instances3.alice.encrypt16(15468),
    );
    expect(res).to.equal(15468);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 4 (15468, 15464)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(15468),
      this.instances3.alice.encrypt16(15464),
    );
    expect(res).to.equal(15468);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (48081, 52727)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(48081),
      this.instances3.alice.encrypt16(52727),
    );
    expect(res).to.equal(30246);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (48077, 48081)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(48077),
      this.instances3.alice.encrypt16(48081),
    );
    expect(res).to.equal(28);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 3 (48081, 48081)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(48081),
      this.instances3.alice.encrypt16(48081),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 4 (48081, 48077)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(48081),
      this.instances3.alice.encrypt16(48077),
    );
    expect(res).to.equal(28);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 1 (5627, 4536)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(5627),
      this.instances3.alice.encrypt16(4536),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 2 (4532, 4536)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(4532),
      this.instances3.alice.encrypt16(4536),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 3 (4536, 4536)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(4536),
      this.instances3.alice.encrypt16(4536),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 4 (4536, 4532)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(4536),
      this.instances3.alice.encrypt16(4532),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 1 (20601, 31416)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(20601),
      this.instances3.alice.encrypt16(31416),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 2 (20597, 20601)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(20597),
      this.instances3.alice.encrypt16(20601),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 3 (20601, 20601)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(20601),
      this.instances3.alice.encrypt16(20601),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 4 (20601, 20597)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(20601),
      this.instances3.alice.encrypt16(20597),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 1 (44342, 57666)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(44342),
      this.instances3.alice.encrypt16(57666),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 2 (44338, 44342)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(44338),
      this.instances3.alice.encrypt16(44342),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 3 (44342, 44342)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(44342),
      this.instances3.alice.encrypt16(44342),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 4 (44342, 44338)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(44342),
      this.instances3.alice.encrypt16(44338),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 1 (42324, 60441)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(42324),
      this.instances3.alice.encrypt16(60441),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 2 (42320, 42324)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(42320),
      this.instances3.alice.encrypt16(42324),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 3 (42324, 42324)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(42324),
      this.instances3.alice.encrypt16(42324),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 4 (42324, 42320)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(42324),
      this.instances3.alice.encrypt16(42320),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 1 (5574, 28657)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(5574),
      this.instances3.alice.encrypt16(28657),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 2 (5570, 5574)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(5570),
      this.instances3.alice.encrypt16(5574),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 3 (5574, 5574)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(5574),
      this.instances3.alice.encrypt16(5574),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 4 (5574, 5570)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(5574),
      this.instances3.alice.encrypt16(5570),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 1 (64394, 46201)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(64394),
      this.instances3.alice.encrypt16(46201),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 2 (46197, 46201)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(46197),
      this.instances3.alice.encrypt16(46201),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 3 (46201, 46201)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(46201),
      this.instances3.alice.encrypt16(46201),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 4 (46201, 46197)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(46201),
      this.instances3.alice.encrypt16(46197),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 1 (3854, 29273)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(3854),
      this.instances3.alice.encrypt16(29273),
    );
    expect(res).to.equal(3854);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 2 (3850, 3854)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(3850),
      this.instances3.alice.encrypt16(3854),
    );
    expect(res).to.equal(3850);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 3 (3854, 3854)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(3854),
      this.instances3.alice.encrypt16(3854),
    );
    expect(res).to.equal(3854);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 4 (3854, 3850)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(3854),
      this.instances3.alice.encrypt16(3850),
    );
    expect(res).to.equal(3850);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 1 (43071, 37043)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(43071),
      this.instances3.alice.encrypt16(37043),
    );
    expect(res).to.equal(43071);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 2 (37039, 37043)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(37039),
      this.instances3.alice.encrypt16(37043),
    );
    expect(res).to.equal(37043);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 3 (37043, 37043)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(37043),
      this.instances3.alice.encrypt16(37043),
    );
    expect(res).to.equal(37043);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 4 (37043, 37039)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(37043),
      this.instances3.alice.encrypt16(37039),
    );
    expect(res).to.equal(37043);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 1 (1, 40500)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt32(40500),
    );
    expect(res).to.equal(40501);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 2 (27826, 27830)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(27826),
      this.instances3.alice.encrypt32(27830),
    );
    expect(res).to.equal(55656);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 3 (27830, 27830)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(27830),
      this.instances3.alice.encrypt32(27830),
    );
    expect(res).to.equal(55660);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 4 (27830, 27826)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(27830),
      this.instances3.alice.encrypt32(27826),
    );
    expect(res).to.equal(55656);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (34964, 34964)', async function () {
    const res = await this.contract3.sub_euint16_euint32(
      this.instances3.alice.encrypt16(34964),
      this.instances3.alice.encrypt32(34964),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (34964, 34960)', async function () {
    const res = await this.contract3.sub_euint16_euint32(
      this.instances3.alice.encrypt16(34964),
      this.instances3.alice.encrypt32(34960),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (1, 39192)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt32(39192),
    );
    expect(res).to.equal(39192);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 2 (144, 144)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(144),
      this.instances3.alice.encrypt32(144),
    );
    expect(res).to.equal(20736);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 3 (144, 144)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(144),
      this.instances3.alice.encrypt32(144),
    );
    expect(res).to.equal(20736);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 4 (144, 144)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(144),
      this.instances3.alice.encrypt32(144),
    );
    expect(res).to.equal(20736);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (14563, 408583146)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(14563),
      this.instances3.alice.encrypt32(408583146),
    );
    expect(res).to.equal(14562);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (14559, 14563)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(14559),
      this.instances3.alice.encrypt32(14563),
    );
    expect(res).to.equal(14531);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (14563, 14563)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(14563),
      this.instances3.alice.encrypt32(14563),
    );
    expect(res).to.equal(14563);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (14563, 14559)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(14563),
      this.instances3.alice.encrypt32(14559),
    );
    expect(res).to.equal(14531);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (1, 37057)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt32(37057),
    );
    expect(res).to.equal(37057);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (15464, 15468)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(15464),
      this.instances3.alice.encrypt32(15468),
    );
    expect(res).to.equal(15468);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 3 (15468, 15468)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(15468),
      this.instances3.alice.encrypt32(15468),
    );
    expect(res).to.equal(15468);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 4 (15468, 15464)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(15468),
      this.instances3.alice.encrypt32(15464),
    );
    expect(res).to.equal(15468);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 1 (1, 33460)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt32(33460),
    );
    expect(res).to.equal(33461);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 2 (48077, 48081)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(48077),
      this.instances3.alice.encrypt32(48081),
    );
    expect(res).to.equal(28);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 3 (48081, 48081)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(48081),
      this.instances3.alice.encrypt32(48081),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 4 (48081, 48077)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(48081),
      this.instances3.alice.encrypt32(48077),
    );
    expect(res).to.equal(28);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (65041, 696957537)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(65041),
      this.instances3.alice.encrypt32(696957537),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (65037, 65041)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(65037),
      this.instances3.alice.encrypt32(65041),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 3 (65041, 65041)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(65041),
      this.instances3.alice.encrypt32(65041),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 4 (65041, 65037)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(65041),
      this.instances3.alice.encrypt32(65037),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 1 (16027, 1143925973)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(16027),
      this.instances3.alice.encrypt32(1143925973),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 2 (16023, 16027)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(16023),
      this.instances3.alice.encrypt32(16027),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 3 (16027, 16027)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(16027),
      this.instances3.alice.encrypt32(16027),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 4 (16027, 16023)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(16027),
      this.instances3.alice.encrypt32(16023),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (64532, 892657805)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(64532),
      this.instances3.alice.encrypt32(892657805),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (64528, 64532)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(64528),
      this.instances3.alice.encrypt32(64532),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (64532, 64532)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(64532),
      this.instances3.alice.encrypt32(64532),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 4 (64532, 64528)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(64532),
      this.instances3.alice.encrypt32(64528),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 1 (47205, 1988625224)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(47205),
      this.instances3.alice.encrypt32(1988625224),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 2 (47201, 47205)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(47201),
      this.instances3.alice.encrypt32(47205),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 3 (47205, 47205)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(47205),
      this.instances3.alice.encrypt32(47205),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 4 (47205, 47201)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(47205),
      this.instances3.alice.encrypt32(47201),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (63255, 340383439)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(63255),
      this.instances3.alice.encrypt32(340383439),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (63251, 63255)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(63251),
      this.instances3.alice.encrypt32(63255),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (63255, 63255)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(63255),
      this.instances3.alice.encrypt32(63255),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 4 (63255, 63251)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(63255),
      this.instances3.alice.encrypt32(63251),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 1 (50578, 1677650454)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(50578),
      this.instances3.alice.encrypt32(1677650454),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 2 (50574, 50578)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(50574),
      this.instances3.alice.encrypt32(50578),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 3 (50578, 50578)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(50578),
      this.instances3.alice.encrypt32(50578),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 4 (50578, 50574)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(50578),
      this.instances3.alice.encrypt32(50574),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 1 (27643, 2066729287)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(27643),
      this.instances3.alice.encrypt32(2066729287),
    );
    expect(res).to.equal(27643);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 2 (27639, 27643)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(27639),
      this.instances3.alice.encrypt32(27643),
    );
    expect(res).to.equal(27639);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 3 (27643, 27643)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(27643),
      this.instances3.alice.encrypt32(27643),
    );
    expect(res).to.equal(27643);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 4 (27643, 27639)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(27643),
      this.instances3.alice.encrypt32(27639),
    );
    expect(res).to.equal(27639);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (1, 46246)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt32(46246),
    );
    expect(res).to.equal(46246);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (31967, 31971)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(31967),
      this.instances3.alice.encrypt32(31971),
    );
    expect(res).to.equal(31971);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (31971, 31971)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(31971),
      this.instances3.alice.encrypt32(31971),
    );
    expect(res).to.equal(31971);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 4 (31971, 31967)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(31971),
      this.instances3.alice.encrypt32(31967),
    );
    expect(res).to.equal(31971);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 1 (1, 46642)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt64(46642),
    );
    expect(res).to.equal(46643);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 2 (27826, 27830)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(27826),
      this.instances3.alice.encrypt64(27830),
    );
    expect(res).to.equal(55656);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 3 (27830, 27830)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(27830),
      this.instances3.alice.encrypt64(27830),
    );
    expect(res).to.equal(55660);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 4 (27830, 27826)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(27830),
      this.instances3.alice.encrypt64(27826),
    );
    expect(res).to.equal(55656);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 1 (34964, 34964)', async function () {
    const res = await this.contract3.sub_euint16_euint64(
      this.instances3.alice.encrypt16(34964),
      this.instances3.alice.encrypt64(34964),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 2 (34964, 34960)', async function () {
    const res = await this.contract3.sub_euint16_euint64(
      this.instances3.alice.encrypt16(34964),
      this.instances3.alice.encrypt64(34960),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 1 (1, 22059)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt64(22059),
    );
    expect(res).to.equal(22059);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 2 (144, 144)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(144),
      this.instances3.alice.encrypt64(144),
    );
    expect(res).to.equal(20736);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 3 (144, 144)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(144),
      this.instances3.alice.encrypt64(144),
    );
    expect(res).to.equal(20736);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 4 (144, 144)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(144),
      this.instances3.alice.encrypt64(144),
    );
    expect(res).to.equal(20736);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 1 (14563, 1642657503)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(14563),
      this.instances3.alice.encrypt64(1642657503),
    );
    expect(res).to.equal(12483);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 2 (14559, 14563)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(14559),
      this.instances3.alice.encrypt64(14563),
    );
    expect(res).to.equal(14531);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 3 (14563, 14563)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(14563),
      this.instances3.alice.encrypt64(14563),
    );
    expect(res).to.equal(14563);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 4 (14563, 14559)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(14563),
      this.instances3.alice.encrypt64(14559),
    );
    expect(res).to.equal(14531);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 1 (1, 52075)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt64(52075),
    );
    expect(res).to.equal(52075);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 2 (15464, 15468)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(15464),
      this.instances3.alice.encrypt64(15468),
    );
    expect(res).to.equal(15468);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 3 (15468, 15468)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(15468),
      this.instances3.alice.encrypt64(15468),
    );
    expect(res).to.equal(15468);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 4 (15468, 15464)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(15468),
      this.instances3.alice.encrypt64(15464),
    );
    expect(res).to.equal(15468);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 1 (5, 40064)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(5),
      this.instances3.alice.encrypt64(40064),
    );
    expect(res).to.equal(40069);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 2 (48077, 48081)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(48077),
      this.instances3.alice.encrypt64(48081),
    );
    expect(res).to.equal(28);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 3 (48081, 48081)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(48081),
      this.instances3.alice.encrypt64(48081),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 4 (48081, 48077)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(48081),
      this.instances3.alice.encrypt64(48077),
    );
    expect(res).to.equal(28);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 1 (65041, 1498344478)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(65041),
      this.instances3.alice.encrypt64(1498344478),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 2 (65037, 65041)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(65037),
      this.instances3.alice.encrypt64(65041),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 3 (65041, 65041)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(65041),
      this.instances3.alice.encrypt64(65041),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 4 (65041, 65037)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(65041),
      this.instances3.alice.encrypt64(65037),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 1 (16027, 1755085159)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(16027),
      this.instances3.alice.encrypt64(1755085159),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 2 (16023, 16027)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(16023),
      this.instances3.alice.encrypt64(16027),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 3 (16027, 16027)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(16027),
      this.instances3.alice.encrypt64(16027),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 4 (16027, 16023)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(16027),
      this.instances3.alice.encrypt64(16023),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 1 (64532, 1819047757)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(64532),
      this.instances3.alice.encrypt64(1819047757),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 2 (64528, 64532)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(64528),
      this.instances3.alice.encrypt64(64532),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 3 (64532, 64532)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(64532),
      this.instances3.alice.encrypt64(64532),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 4 (64532, 64528)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(64532),
      this.instances3.alice.encrypt64(64528),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 1 (47205, 1792915583)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(47205),
      this.instances3.alice.encrypt64(1792915583),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 2 (47201, 47205)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(47201),
      this.instances3.alice.encrypt64(47205),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 3 (47205, 47205)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(47205),
      this.instances3.alice.encrypt64(47205),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 4 (47205, 47201)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(47205),
      this.instances3.alice.encrypt64(47201),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 1 (63255, 1184396370)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(63255),
      this.instances3.alice.encrypt64(1184396370),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 2 (63251, 63255)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(63251),
      this.instances3.alice.encrypt64(63255),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 3 (63255, 63255)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(63255),
      this.instances3.alice.encrypt64(63255),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 4 (63255, 63251)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(63255),
      this.instances3.alice.encrypt64(63251),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 1 (50578, 1995209966)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(50578),
      this.instances3.alice.encrypt64(1995209966),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 2 (50574, 50578)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(50574),
      this.instances3.alice.encrypt64(50578),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 3 (50578, 50578)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(50578),
      this.instances3.alice.encrypt64(50578),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 4 (50578, 50574)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(50578),
      this.instances3.alice.encrypt64(50574),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 1 (27643, 1529181739)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(27643),
      this.instances3.alice.encrypt64(1529181739),
    );
    expect(res).to.equal(27643);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 2 (27639, 27643)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(27639),
      this.instances3.alice.encrypt64(27643),
    );
    expect(res).to.equal(27639);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 3 (27643, 27643)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(27643),
      this.instances3.alice.encrypt64(27643),
    );
    expect(res).to.equal(27643);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 4 (27643, 27639)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(27643),
      this.instances3.alice.encrypt64(27639),
    );
    expect(res).to.equal(27639);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 1 (3, 35170)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(3),
      this.instances3.alice.encrypt64(35170),
    );
    expect(res).to.equal(35170);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 2 (31967, 31971)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(31967),
      this.instances3.alice.encrypt64(31971),
    );
    expect(res).to.equal(31971);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 3 (31971, 31971)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(31971),
      this.instances3.alice.encrypt64(31971),
    );
    expect(res).to.equal(31971);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 4 (31971, 31967)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(31971),
      this.instances3.alice.encrypt64(31967),
    );
    expect(res).to.equal(31971);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (30233, 10584)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(30233), 10584);
    expect(res).to.equal(40817);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (30229, 30233)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(30229), 30233);
    expect(res).to.equal(60462);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (30233, 30233)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(30233), 30233);
    expect(res).to.equal(60466);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (30233, 30229)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(30233), 30229);
    expect(res).to.equal(60462);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (27830, 10584)', async function () {
    const res = await this.contract3.add_uint16_euint16(27830, this.instances3.alice.encrypt16(10584));
    expect(res).to.equal(38414);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 2 (30229, 30233)', async function () {
    const res = await this.contract3.add_uint16_euint16(30229, this.instances3.alice.encrypt16(30233));
    expect(res).to.equal(60462);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 3 (30233, 30233)', async function () {
    const res = await this.contract3.add_uint16_euint16(30233, this.instances3.alice.encrypt16(30233));
    expect(res).to.equal(60466);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 4 (30233, 30229)', async function () {
    const res = await this.contract3.add_uint16_euint16(30233, this.instances3.alice.encrypt16(30229));
    expect(res).to.equal(60462);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (7674, 7674)', async function () {
    const res = await this.contract3.sub_euint16_uint16(this.instances3.alice.encrypt16(7674), 7674);
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 2 (7674, 7670)', async function () {
    const res = await this.contract3.sub_euint16_uint16(this.instances3.alice.encrypt16(7674), 7670);
    expect(res).to.equal(4);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (7674, 7674)', async function () {
    const res = await this.contract3.sub_uint16_euint16(7674, this.instances3.alice.encrypt16(7674));
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (7674, 7670)', async function () {
    const res = await this.contract3.sub_uint16_euint16(7674, this.instances3.alice.encrypt16(7670));
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (442, 133)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(442), 133);
    expect(res).to.equal(58786);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (180, 180)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(180), 180);
    expect(res).to.equal(32400);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (180, 180)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(180), 180);
    expect(res).to.equal(32400);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (180, 180)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(180), 180);
    expect(res).to.equal(32400);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (288, 133)', async function () {
    const res = await this.contract3.mul_uint16_euint16(288, this.instances3.alice.encrypt16(133));
    expect(res).to.equal(38304);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 2 (180, 180)', async function () {
    const res = await this.contract3.mul_uint16_euint16(180, this.instances3.alice.encrypt16(180));
    expect(res).to.equal(32400);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 3 (180, 180)', async function () {
    const res = await this.contract3.mul_uint16_euint16(180, this.instances3.alice.encrypt16(180));
    expect(res).to.equal(32400);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 4 (180, 180)', async function () {
    const res = await this.contract3.mul_uint16_euint16(180, this.instances3.alice.encrypt16(180));
    expect(res).to.equal(32400);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (30116, 13141)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(30116), 13141);
    expect(res).to.equal(2);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (8385, 8389)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(8385), 8389);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (8389, 8389)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(8389), 8389);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (8389, 8385)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(8389), 8385);
    expect(res).to.equal(1);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (50030, 38531)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(50030), 38531);
    expect(res).to.equal(11499);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 2 (13739, 13743)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(13739), 13743);
    expect(res).to.equal(13739);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 3 (13743, 13743)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(13743), 13743);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 4 (13743, 13739)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(13743), 13739);
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (5627, 53030)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(5627), 53030);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (4532, 4536)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(4532), 4536);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (4536, 4536)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(4536), 4536);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (4536, 4532)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(4536), 4532);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (65041, 53030)', async function () {
    const res = await this.contract3.eq_uint16_euint16(65041, this.instances3.alice.encrypt16(53030));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (4532, 4536)', async function () {
    const res = await this.contract3.eq_uint16_euint16(4532, this.instances3.alice.encrypt16(4536));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 3 (4536, 4536)', async function () {
    const res = await this.contract3.eq_uint16_euint16(4536, this.instances3.alice.encrypt16(4536));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 4 (4536, 4532)', async function () {
    const res = await this.contract3.eq_uint16_euint16(4536, this.instances3.alice.encrypt16(4532));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (20601, 24726)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(20601), 24726);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (20597, 20601)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(20597), 20601);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 3 (20601, 20601)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(20601), 20601);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 4 (20601, 20597)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(20601), 20597);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (16027, 24726)', async function () {
    const res = await this.contract3.ne_uint16_euint16(16027, this.instances3.alice.encrypt16(24726));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (20597, 20601)', async function () {
    const res = await this.contract3.ne_uint16_euint16(20597, this.instances3.alice.encrypt16(20601));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 3 (20601, 20601)', async function () {
    const res = await this.contract3.ne_uint16_euint16(20601, this.instances3.alice.encrypt16(20601));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 4 (20601, 20597)', async function () {
    const res = await this.contract3.ne_uint16_euint16(20601, this.instances3.alice.encrypt16(20597));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (44342, 6260)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(44342), 6260);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (44338, 44342)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(44338), 44342);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (44342, 44342)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(44342), 44342);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 4 (44342, 44338)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(44342), 44338);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (64532, 6260)', async function () {
    const res = await this.contract3.ge_uint16_euint16(64532, this.instances3.alice.encrypt16(6260));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (44338, 44342)', async function () {
    const res = await this.contract3.ge_uint16_euint16(44338, this.instances3.alice.encrypt16(44342));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (44342, 44342)', async function () {
    const res = await this.contract3.ge_uint16_euint16(44342, this.instances3.alice.encrypt16(44342));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 4 (44342, 44338)', async function () {
    const res = await this.contract3.ge_uint16_euint16(44342, this.instances3.alice.encrypt16(44338));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (42324, 7407)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(42324), 7407);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (42320, 42324)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(42320), 42324);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (42324, 42324)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(42324), 42324);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 4 (42324, 42320)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(42324), 42320);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (47205, 7407)', async function () {
    const res = await this.contract3.gt_uint16_euint16(47205, this.instances3.alice.encrypt16(7407));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (42320, 42324)', async function () {
    const res = await this.contract3.gt_uint16_euint16(42320, this.instances3.alice.encrypt16(42324));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (42324, 42324)', async function () {
    const res = await this.contract3.gt_uint16_euint16(42324, this.instances3.alice.encrypt16(42324));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 4 (42324, 42320)', async function () {
    const res = await this.contract3.gt_uint16_euint16(42324, this.instances3.alice.encrypt16(42320));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (5574, 3979)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(5574), 3979);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (5570, 5574)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(5570), 5574);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (5574, 5574)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(5574), 5574);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 4 (5574, 5570)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(5574), 5570);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (63255, 3979)', async function () {
    const res = await this.contract3.le_uint16_euint16(63255, this.instances3.alice.encrypt16(3979));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (5570, 5574)', async function () {
    const res = await this.contract3.le_uint16_euint16(5570, this.instances3.alice.encrypt16(5574));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (5574, 5574)', async function () {
    const res = await this.contract3.le_uint16_euint16(5574, this.instances3.alice.encrypt16(5574));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (5574, 5570)', async function () {
    const res = await this.contract3.le_uint16_euint16(5574, this.instances3.alice.encrypt16(5570));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (64394, 5464)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(64394), 5464);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (46197, 46201)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(46197), 46201);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (46201, 46201)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(46201), 46201);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 4 (46201, 46197)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(46201), 46197);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (50578, 5464)', async function () {
    const res = await this.contract3.lt_uint16_euint16(50578, this.instances3.alice.encrypt16(5464));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (46197, 46201)', async function () {
    const res = await this.contract3.lt_uint16_euint16(46197, this.instances3.alice.encrypt16(46201));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (46201, 46201)', async function () {
    const res = await this.contract3.lt_uint16_euint16(46201, this.instances3.alice.encrypt16(46201));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 4 (46201, 46197)', async function () {
    const res = await this.contract3.lt_uint16_euint16(46201, this.instances3.alice.encrypt16(46197));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (3854, 54603)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(3854), 54603);
    expect(res).to.equal(3854);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (3850, 3854)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(3850), 3854);
    expect(res).to.equal(3850);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (3854, 3854)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(3854), 3854);
    expect(res).to.equal(3854);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 4 (3854, 3850)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(3854), 3850);
    expect(res).to.equal(3850);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (27643, 54603)', async function () {
    const res = await this.contract3.min_uint16_euint16(27643, this.instances3.alice.encrypt16(54603));
    expect(res).to.equal(27643);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (3850, 3854)', async function () {
    const res = await this.contract3.min_uint16_euint16(3850, this.instances3.alice.encrypt16(3854));
    expect(res).to.equal(3850);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (3854, 3854)', async function () {
    const res = await this.contract3.min_uint16_euint16(3854, this.instances3.alice.encrypt16(3854));
    expect(res).to.equal(3854);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (3854, 3850)', async function () {
    const res = await this.contract3.min_uint16_euint16(3854, this.instances3.alice.encrypt16(3850));
    expect(res).to.equal(3850);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (43071, 57395)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(43071), 57395);
    expect(res).to.equal(57395);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (37039, 37043)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(37039), 37043);
    expect(res).to.equal(37043);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (37043, 37043)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(37043), 37043);
    expect(res).to.equal(37043);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (37043, 37039)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(37043), 37039);
    expect(res).to.equal(37043);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (31971, 57395)', async function () {
    const res = await this.contract3.max_uint16_euint16(31971, this.instances3.alice.encrypt16(57395));
    expect(res).to.equal(57395);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (37039, 37043)', async function () {
    const res = await this.contract3.max_uint16_euint16(37039, this.instances3.alice.encrypt16(37043));
    expect(res).to.equal(37043);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (37043, 37043)', async function () {
    const res = await this.contract3.max_uint16_euint16(37043, this.instances3.alice.encrypt16(37043));
    expect(res).to.equal(37043);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (37043, 37039)', async function () {
    const res = await this.contract3.max_uint16_euint16(37043, this.instances3.alice.encrypt16(37039));
    expect(res).to.equal(37043);
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

  it('test operator "sub" overload (euint32, euint4) => euint32 test 1 (8, 8)', async function () {
    const res = await this.contract3.sub_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, euint4) => euint32 test 2 (8, 4)', async function () {
    const res = await this.contract3.sub_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 1 (13, 1)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(13),
      this.instances3.alice.encrypt4(1),
    );
    expect(res).to.equal(13);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 2 (2, 3)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(2),
      this.instances3.alice.encrypt4(3),
    );
    expect(res).to.equal(6);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 3 (3, 3)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(3),
      this.instances3.alice.encrypt4(3),
    );
    expect(res).to.equal(9);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 4 (3, 2)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(3),
      this.instances3.alice.encrypt4(2),
    );
    expect(res).to.equal(6);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 1 (1757898274, 2)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(1757898274),
      this.instances3.alice.encrypt4(2),
    );
    expect(res).to.equal(2);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 1 (9, 1)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(1),
    );
    expect(res).to.equal(9);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 2 (10, 14)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(10),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 3 (14, 14)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(14),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 4 (14, 10)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(14),
      this.instances3.alice.encrypt4(10),
    );
    expect(res).to.equal(14);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 1 (13, 1)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(13),
      this.instances3.alice.encrypt4(1),
    );
    expect(res).to.equal(12);
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

  it('test operator "eq" overload (euint32, euint4) => ebool test 1 (1270694518, 8)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(1270694518),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 1 (1211702728, 8)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(1211702728),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 1 (627970552, 13)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(627970552),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(13),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(13),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 1 (1801714413, 14)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(1801714413),
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

  it('test operator "le" overload (euint32, euint4) => ebool test 1 (523722139, 11)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(523722139),
      this.instances3.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(7),
      this.instances3.alice.encrypt4(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(11),
      this.instances3.alice.encrypt4(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(11),
      this.instances3.alice.encrypt4(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 1 (1044281941, 14)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(1044281941),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(10),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(14),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(14),
      this.instances3.alice.encrypt4(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 1 (390943086, 2)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(390943086),
      this.instances3.alice.encrypt4(2),
    );
    expect(res).to.equal(2);
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

  it('test operator "max" overload (euint32, euint4) => euint32 test 1 (8, 1)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(1),
    );
    expect(res).to.equal(8);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 2 (7, 11)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(7),
      this.instances3.alice.encrypt4(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 3 (11, 11)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(11),
      this.instances3.alice.encrypt4(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 4 (11, 7)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(11),
      this.instances3.alice.encrypt4(7),
    );
    expect(res).to.equal(11);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (162, 1)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(162),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(163);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (115, 117)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(115),
      this.instances3.alice.encrypt8(117),
    );
    expect(res).to.equal(232);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (117, 117)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(117),
      this.instances3.alice.encrypt8(117),
    );
    expect(res).to.equal(234);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (117, 115)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(117),
      this.instances3.alice.encrypt8(115),
    );
    expect(res).to.equal(232);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (239, 239)', async function () {
    const res = await this.contract3.sub_euint32_euint8(
      this.instances3.alice.encrypt32(239),
      this.instances3.alice.encrypt8(239),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (239, 235)', async function () {
    const res = await this.contract3.sub_euint32_euint8(
      this.instances3.alice.encrypt32(239),
      this.instances3.alice.encrypt8(235),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (222, 1)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(222),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(222);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 2 (10, 14)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(10),
      this.instances3.alice.encrypt8(14),
    );
    expect(res).to.equal(140);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 3 (14, 14)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(14),
      this.instances3.alice.encrypt8(14),
    );
    expect(res).to.equal(196);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 4 (14, 10)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(14),
      this.instances3.alice.encrypt8(10),
    );
    expect(res).to.equal(140);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (1757898274, 2)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(1757898274),
      this.instances3.alice.encrypt8(2),
    );
    expect(res).to.equal(2);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (156, 1)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(156),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(157);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (25, 29)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(25),
      this.instances4.alice.encrypt8(29),
    );
    expect(res).to.equal(29);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (29, 29)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(29),
      this.instances4.alice.encrypt8(29),
    );
    expect(res).to.equal(29);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (29, 25)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(29),
      this.instances4.alice.encrypt8(25),
    );
    expect(res).to.equal(29);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (210, 1)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(210),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(211);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (54, 58)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(54),
      this.instances4.alice.encrypt8(58),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (58, 58)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(58),
      this.instances4.alice.encrypt8(58),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (58, 54)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(58),
      this.instances4.alice.encrypt8(54),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (1270694518, 11)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(1270694518),
      this.instances4.alice.encrypt8(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(7),
      this.instances4.alice.encrypt8(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(11),
      this.instances4.alice.encrypt8(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(11),
      this.instances4.alice.encrypt8(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (1211702728, 132)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(1211702728),
      this.instances4.alice.encrypt8(132),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (128, 132)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(128),
      this.instances4.alice.encrypt8(132),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 3 (132, 132)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(132),
      this.instances4.alice.encrypt8(132),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 4 (132, 128)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(132),
      this.instances4.alice.encrypt8(128),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (627970552, 5)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(627970552),
      this.instances4.alice.encrypt8(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (1801714413, 95)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(1801714413),
      this.instances4.alice.encrypt8(95),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (91, 95)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(91),
      this.instances4.alice.encrypt8(95),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (95, 95)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(95),
      this.instances4.alice.encrypt8(95),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 4 (95, 91)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(95),
      this.instances4.alice.encrypt8(91),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (523722139, 41)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(523722139),
      this.instances4.alice.encrypt8(41),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (37, 41)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(37),
      this.instances4.alice.encrypt8(41),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (41, 41)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(41),
      this.instances4.alice.encrypt8(41),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (41, 37)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(41),
      this.instances4.alice.encrypt8(37),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (1044281941, 77)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(1044281941),
      this.instances4.alice.encrypt8(77),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (73, 77)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(73),
      this.instances4.alice.encrypt8(77),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (77, 77)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(77),
      this.instances4.alice.encrypt8(77),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 4 (77, 73)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(77),
      this.instances4.alice.encrypt8(73),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (390943086, 113)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(390943086),
      this.instances4.alice.encrypt8(113),
    );
    expect(res).to.equal(113);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (109, 113)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(109),
      this.instances4.alice.encrypt8(113),
    );
    expect(res).to.equal(109);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (113, 113)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(113),
      this.instances4.alice.encrypt8(113),
    );
    expect(res).to.equal(113);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (113, 109)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(113),
      this.instances4.alice.encrypt8(109),
    );
    expect(res).to.equal(109);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (135, 1)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(135),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(135);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (81, 85)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(81),
      this.instances4.alice.encrypt8(85),
    );
    expect(res).to.equal(85);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (85, 85)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(85),
      this.instances4.alice.encrypt8(85),
    );
    expect(res).to.equal(85);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 4 (85, 81)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(85),
      this.instances4.alice.encrypt8(81),
    );
    expect(res).to.equal(85);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (41642, 13)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(41642),
      this.instances4.alice.encrypt16(13),
    );
    expect(res).to.equal(41655);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (28344, 28348)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(28344),
      this.instances4.alice.encrypt16(28348),
    );
    expect(res).to.equal(56692);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (28348, 28348)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(28348),
      this.instances4.alice.encrypt16(28348),
    );
    expect(res).to.equal(56696);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (28348, 28344)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(28348),
      this.instances4.alice.encrypt16(28344),
    );
    expect(res).to.equal(56692);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (4191, 4191)', async function () {
    const res = await this.contract4.sub_euint32_euint16(
      this.instances4.alice.encrypt32(4191),
      this.instances4.alice.encrypt16(4191),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (4191, 4187)', async function () {
    const res = await this.contract4.sub_euint32_euint16(
      this.instances4.alice.encrypt32(4191),
      this.instances4.alice.encrypt16(4187),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (56978, 1)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(56978),
      this.instances4.alice.encrypt16(1),
    );
    expect(res).to.equal(56978);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (159, 159)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(159),
      this.instances4.alice.encrypt16(159),
    );
    expect(res).to.equal(25281);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (159, 159)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(159),
      this.instances4.alice.encrypt16(159),
    );
    expect(res).to.equal(25281);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (159, 159)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(159),
      this.instances4.alice.encrypt16(159),
    );
    expect(res).to.equal(25281);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (1757898274, 20019)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(1757898274),
      this.instances4.alice.encrypt16(20019),
    );
    expect(res).to.equal(17954);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (20015, 20019)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(20015),
      this.instances4.alice.encrypt16(20019),
    );
    expect(res).to.equal(20003);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (20019, 20019)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(20019),
      this.instances4.alice.encrypt16(20019),
    );
    expect(res).to.equal(20019);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (20019, 20015)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(20019),
      this.instances4.alice.encrypt16(20015),
    );
    expect(res).to.equal(20003);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (40006, 1)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(40006),
      this.instances4.alice.encrypt16(1),
    );
    expect(res).to.equal(40007);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (46882, 46886)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(46882),
      this.instances4.alice.encrypt16(46886),
    );
    expect(res).to.equal(46886);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 3 (46886, 46886)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(46886),
      this.instances4.alice.encrypt16(46886),
    );
    expect(res).to.equal(46886);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 4 (46886, 46882)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(46886),
      this.instances4.alice.encrypt16(46882),
    );
    expect(res).to.equal(46886);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (53760, 15)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(53760),
      this.instances4.alice.encrypt16(15),
    );
    expect(res).to.equal(53775);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (64891, 64895)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(64891),
      this.instances4.alice.encrypt16(64895),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (64895, 64895)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(64895),
      this.instances4.alice.encrypt16(64895),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (64895, 64891)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(64895),
      this.instances4.alice.encrypt16(64891),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (1270694518, 22132)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(1270694518),
      this.instances4.alice.encrypt16(22132),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (22128, 22132)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(22128),
      this.instances4.alice.encrypt16(22132),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (22132, 22132)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(22132),
      this.instances4.alice.encrypt16(22132),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (22132, 22128)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(22132),
      this.instances4.alice.encrypt16(22128),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (1211702728, 13344)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(1211702728),
      this.instances4.alice.encrypt16(13344),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (13340, 13344)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(13340),
      this.instances4.alice.encrypt16(13344),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 3 (13344, 13344)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(13344),
      this.instances4.alice.encrypt16(13344),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 4 (13344, 13340)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(13344),
      this.instances4.alice.encrypt16(13340),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (627970552, 58999)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(627970552),
      this.instances4.alice.encrypt16(58999),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (58995, 58999)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(58995),
      this.instances4.alice.encrypt16(58999),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (58999, 58999)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(58999),
      this.instances4.alice.encrypt16(58999),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 4 (58999, 58995)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(58999),
      this.instances4.alice.encrypt16(58995),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (1801714413, 56461)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(1801714413),
      this.instances4.alice.encrypt16(56461),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (56457, 56461)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(56457),
      this.instances4.alice.encrypt16(56461),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (56461, 56461)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(56461),
      this.instances4.alice.encrypt16(56461),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 4 (56461, 56457)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(56461),
      this.instances4.alice.encrypt16(56457),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (523722139, 51250)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(523722139),
      this.instances4.alice.encrypt16(51250),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (51246, 51250)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(51246),
      this.instances4.alice.encrypt16(51250),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (51250, 51250)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(51250),
      this.instances4.alice.encrypt16(51250),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (51250, 51246)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(51250),
      this.instances4.alice.encrypt16(51246),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (1044281941, 44435)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(1044281941),
      this.instances4.alice.encrypt16(44435),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (44431, 44435)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(44431),
      this.instances4.alice.encrypt16(44435),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (44435, 44435)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(44435),
      this.instances4.alice.encrypt16(44435),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (44435, 44431)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(44435),
      this.instances4.alice.encrypt16(44431),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (390943086, 41085)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(390943086),
      this.instances4.alice.encrypt16(41085),
    );
    expect(res).to.equal(41085);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (41081, 41085)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(41081),
      this.instances4.alice.encrypt16(41085),
    );
    expect(res).to.equal(41081);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (41085, 41085)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(41085),
      this.instances4.alice.encrypt16(41085),
    );
    expect(res).to.equal(41085);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (41085, 41081)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(41085),
      this.instances4.alice.encrypt16(41081),
    );
    expect(res).to.equal(41081);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (34584, 1)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(34584),
      this.instances4.alice.encrypt16(1),
    );
    expect(res).to.equal(34584);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (2745, 2749)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(2745),
      this.instances4.alice.encrypt16(2749),
    );
    expect(res).to.equal(2749);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (2749, 2749)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(2749),
      this.instances4.alice.encrypt16(2749),
    );
    expect(res).to.equal(2749);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (2749, 2745)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(2749),
      this.instances4.alice.encrypt16(2745),
    );
    expect(res).to.equal(2749);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (85283614, 1389046047)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(85283614),
      this.instances4.alice.encrypt32(1389046047),
    );
    expect(res).to.equal(1474329661);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (85283610, 85283614)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(85283610),
      this.instances4.alice.encrypt32(85283614),
    );
    expect(res).to.equal(170567224);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (85283614, 85283614)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(85283614),
      this.instances4.alice.encrypt32(85283614),
    );
    expect(res).to.equal(170567228);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (85283614, 85283610)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(85283614),
      this.instances4.alice.encrypt32(85283610),
    );
    expect(res).to.equal(170567224);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (949346704, 949346704)', async function () {
    const res = await this.contract4.sub_euint32_euint32(
      this.instances4.alice.encrypt32(949346704),
      this.instances4.alice.encrypt32(949346704),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (949346704, 949346700)', async function () {
    const res = await this.contract4.sub_euint32_euint32(
      this.instances4.alice.encrypt32(949346704),
      this.instances4.alice.encrypt32(949346700),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (28489, 38748)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(28489),
      this.instances4.alice.encrypt32(38748),
    );
    expect(res).to.equal(1103891772);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 2 (56978, 56978)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(56978),
      this.instances4.alice.encrypt32(56978),
    );
    expect(res).to.equal(3246492484);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 3 (56978, 56978)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(56978),
      this.instances4.alice.encrypt32(56978),
    );
    expect(res).to.equal(3246492484);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 4 (56978, 56978)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(56978),
      this.instances4.alice.encrypt32(56978),
    );
    expect(res).to.equal(3246492484);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (1757898274, 1599687665)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(1757898274),
      this.instances4.alice.encrypt32(1599687665),
    );
    expect(res).to.equal(1212236320);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (1599687661, 1599687665)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(1599687661),
      this.instances4.alice.encrypt32(1599687665),
    );
    expect(res).to.equal(1599687649);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 3 (1599687665, 1599687665)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(1599687665),
      this.instances4.alice.encrypt32(1599687665),
    );
    expect(res).to.equal(1599687665);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 4 (1599687665, 1599687661)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(1599687665),
      this.instances4.alice.encrypt32(1599687661),
    );
    expect(res).to.equal(1599687649);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (1310933534, 968535512)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(1310933534),
      this.instances4.alice.encrypt32(968535512),
    );
    expect(res).to.equal(2143023070);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (968535508, 968535512)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(968535508),
      this.instances4.alice.encrypt32(968535512),
    );
    expect(res).to.equal(968535516);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 3 (968535512, 968535512)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(968535512),
      this.instances4.alice.encrypt32(968535512),
    );
    expect(res).to.equal(968535512);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 4 (968535512, 968535508)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(968535512),
      this.instances4.alice.encrypt32(968535508),
    );
    expect(res).to.equal(968535516);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (220202687, 1588836729)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(220202687),
      this.instances4.alice.encrypt32(1588836729),
    );
    expect(res).to.equal(1402191814);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (220202683, 220202687)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(220202683),
      this.instances4.alice.encrypt32(220202687),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (220202687, 220202687)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(220202687),
      this.instances4.alice.encrypt32(220202687),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (220202687, 220202683)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(220202687),
      this.instances4.alice.encrypt32(220202683),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (1270694518, 247535168)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(1270694518),
      this.instances4.alice.encrypt32(247535168),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (247535164, 247535168)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(247535164),
      this.instances4.alice.encrypt32(247535168),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (247535168, 247535168)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(247535168),
      this.instances4.alice.encrypt32(247535168),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (247535168, 247535164)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(247535168),
      this.instances4.alice.encrypt32(247535164),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (1211702728, 563095027)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(1211702728),
      this.instances4.alice.encrypt32(563095027),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (563095023, 563095027)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(563095023),
      this.instances4.alice.encrypt32(563095027),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 3 (563095027, 563095027)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(563095027),
      this.instances4.alice.encrypt32(563095027),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 4 (563095027, 563095023)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(563095027),
      this.instances4.alice.encrypt32(563095023),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (627970552, 2136326684)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(627970552),
      this.instances4.alice.encrypt32(2136326684),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (627970548, 627970552)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(627970548),
      this.instances4.alice.encrypt32(627970552),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (627970552, 627970552)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(627970552),
      this.instances4.alice.encrypt32(627970552),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (627970552, 627970548)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(627970552),
      this.instances4.alice.encrypt32(627970548),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (1801714413, 831227104)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(1801714413),
      this.instances4.alice.encrypt32(831227104),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (831227100, 831227104)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(831227100),
      this.instances4.alice.encrypt32(831227104),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (831227104, 831227104)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(831227104),
      this.instances4.alice.encrypt32(831227104),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 4 (831227104, 831227100)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(831227104),
      this.instances4.alice.encrypt32(831227100),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (523722139, 878801545)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(523722139),
      this.instances4.alice.encrypt32(878801545),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (523722135, 523722139)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(523722135),
      this.instances4.alice.encrypt32(523722139),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (523722139, 523722139)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(523722139),
      this.instances4.alice.encrypt32(523722139),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 4 (523722139, 523722135)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(523722139),
      this.instances4.alice.encrypt32(523722135),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (1044281941, 27475171)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(1044281941),
      this.instances4.alice.encrypt32(27475171),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (27475167, 27475171)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(27475167),
      this.instances4.alice.encrypt32(27475171),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (27475171, 27475171)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(27475171),
      this.instances4.alice.encrypt32(27475171),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (27475171, 27475167)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(27475171),
      this.instances4.alice.encrypt32(27475167),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (390943086, 2102211879)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(390943086),
      this.instances4.alice.encrypt32(2102211879),
    );
    expect(res).to.equal(390943086);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (390943082, 390943086)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(390943082),
      this.instances4.alice.encrypt32(390943086),
    );
    expect(res).to.equal(390943082);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (390943086, 390943086)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(390943086),
      this.instances4.alice.encrypt32(390943086),
    );
    expect(res).to.equal(390943086);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (390943086, 390943082)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(390943086),
      this.instances4.alice.encrypt32(390943082),
    );
    expect(res).to.equal(390943082);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (283314521, 1686574997)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(283314521),
      this.instances4.alice.encrypt32(1686574997),
    );
    expect(res).to.equal(1686574997);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (283314517, 283314521)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(283314517),
      this.instances4.alice.encrypt32(283314521),
    );
    expect(res).to.equal(283314521);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (283314521, 283314521)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(283314521),
      this.instances4.alice.encrypt32(283314521),
    );
    expect(res).to.equal(283314521);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (283314521, 283314517)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(283314521),
      this.instances4.alice.encrypt32(283314517),
    );
    expect(res).to.equal(283314521);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 1 (343329587, 532996009)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(343329587),
      this.instances4.alice.encrypt64(532996009),
    );
    expect(res).to.equal(876325596);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 2 (343329583, 343329587)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(343329583),
      this.instances4.alice.encrypt64(343329587),
    );
    expect(res).to.equal(686659170);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 3 (343329587, 343329587)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(343329587),
      this.instances4.alice.encrypt64(343329587),
    );
    expect(res).to.equal(686659174);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 4 (343329587, 343329583)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(343329587),
      this.instances4.alice.encrypt64(343329583),
    );
    expect(res).to.equal(686659170);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (108582607, 108582607)', async function () {
    const res = await this.contract4.sub_euint32_euint64(
      this.instances4.alice.encrypt32(108582607),
      this.instances4.alice.encrypt64(108582607),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (108582607, 108582603)', async function () {
    const res = await this.contract4.sub_euint32_euint64(
      this.instances4.alice.encrypt32(108582607),
      this.instances4.alice.encrypt64(108582603),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (145351, 9637)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(145351),
      this.instances4.alice.encrypt64(9637),
    );
    expect(res).to.equal(1400747587);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (38548, 38548)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(38548),
      this.instances4.alice.encrypt64(38548),
    );
    expect(res).to.equal(1485948304);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (38548, 38548)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(38548),
      this.instances4.alice.encrypt64(38548),
    );
    expect(res).to.equal(1485948304);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (38548, 38548)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(38548),
      this.instances4.alice.encrypt64(38548),
    );
    expect(res).to.equal(1485948304);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 1 (1757898274, 276645072)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(1757898274),
      this.instances4.alice.encrypt64(276645072),
    );
    expect(res).to.equal(4539392);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 2 (276645068, 276645072)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(276645068),
      this.instances4.alice.encrypt64(276645072),
    );
    expect(res).to.equal(276645056);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 3 (276645072, 276645072)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(276645072),
      this.instances4.alice.encrypt64(276645072),
    );
    expect(res).to.equal(276645072);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 4 (276645072, 276645068)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(276645072),
      this.instances4.alice.encrypt64(276645068),
    );
    expect(res).to.equal(276645056);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 1 (1310933534, 287712997)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(1310933534),
      this.instances4.alice.encrypt64(287712997),
    );
    expect(res).to.equal(1596417791);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 2 (287712993, 287712997)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(287712993),
      this.instances4.alice.encrypt64(287712997),
    );
    expect(res).to.equal(287712997);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 3 (287712997, 287712997)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(287712997),
      this.instances4.alice.encrypt64(287712997),
    );
    expect(res).to.equal(287712997);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 4 (287712997, 287712993)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(287712997),
      this.instances4.alice.encrypt64(287712993),
    );
    expect(res).to.equal(287712997);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (220202687, 58005830)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(220202687),
      this.instances4.alice.encrypt64(58005830),
    );
    expect(res).to.equal(240459769);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (58005826, 58005830)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(58005826),
      this.instances4.alice.encrypt64(58005830),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (58005830, 58005830)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(58005830),
      this.instances4.alice.encrypt64(58005830),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (58005830, 58005826)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(58005830),
      this.instances4.alice.encrypt64(58005826),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 1 (1877579186, 1428129540)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(1877579186),
      this.instances4.alice.encrypt64(1428129540),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 2 (1428129536, 1428129540)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(1428129536),
      this.instances4.alice.encrypt64(1428129540),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 3 (1428129540, 1428129540)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(1428129540),
      this.instances4.alice.encrypt64(1428129540),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 4 (1428129540, 1428129536)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(1428129540),
      this.instances4.alice.encrypt64(1428129536),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (1464361694, 2125136992)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(1464361694),
      this.instances4.alice.encrypt64(2125136992),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (1464361690, 1464361694)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(1464361690),
      this.instances4.alice.encrypt64(1464361694),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (1464361694, 1464361694)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(1464361694),
      this.instances4.alice.encrypt64(1464361694),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (1464361694, 1464361690)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(1464361694),
      this.instances4.alice.encrypt64(1464361690),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 1 (1972025848, 1698971084)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(1972025848),
      this.instances4.alice.encrypt64(1698971084),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 2 (1698971080, 1698971084)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(1698971080),
      this.instances4.alice.encrypt64(1698971084),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 3 (1698971084, 1698971084)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(1698971084),
      this.instances4.alice.encrypt64(1698971084),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 4 (1698971084, 1698971080)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(1698971084),
      this.instances4.alice.encrypt64(1698971080),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 1 (258393672, 1790809342)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(258393672),
      this.instances4.alice.encrypt64(1790809342),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 2 (258393668, 258393672)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(258393668),
      this.instances4.alice.encrypt64(258393672),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 3 (258393672, 258393672)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(258393672),
      this.instances4.alice.encrypt64(258393672),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 4 (258393672, 258393668)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(258393672),
      this.instances4.alice.encrypt64(258393668),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 1 (1167528973, 1661227936)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(1167528973),
      this.instances4.alice.encrypt64(1661227936),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 2 (1167528969, 1167528973)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(1167528969),
      this.instances4.alice.encrypt64(1167528973),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 3 (1167528973, 1167528973)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(1167528973),
      this.instances4.alice.encrypt64(1167528973),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 4 (1167528973, 1167528969)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(1167528973),
      this.instances4.alice.encrypt64(1167528969),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 1 (1829077110, 871332370)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(1829077110),
      this.instances4.alice.encrypt64(871332370),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 2 (871332366, 871332370)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(871332366),
      this.instances4.alice.encrypt64(871332370),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 3 (871332370, 871332370)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(871332370),
      this.instances4.alice.encrypt64(871332370),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 4 (871332370, 871332366)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(871332370),
      this.instances4.alice.encrypt64(871332366),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 1 (1819271941, 1588645185)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(1819271941),
      this.instances4.alice.encrypt64(1588645185),
    );
    expect(res).to.equal(1588645185);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 2 (1588645181, 1588645185)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(1588645181),
      this.instances4.alice.encrypt64(1588645185),
    );
    expect(res).to.equal(1588645181);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 3 (1588645185, 1588645185)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(1588645185),
      this.instances4.alice.encrypt64(1588645185),
    );
    expect(res).to.equal(1588645185);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 4 (1588645185, 1588645181)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(1588645185),
      this.instances4.alice.encrypt64(1588645181),
    );
    expect(res).to.equal(1588645181);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (1446821827, 98162433)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(1446821827),
      this.instances4.alice.encrypt64(98162433),
    );
    expect(res).to.equal(1446821827);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (98162429, 98162433)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(98162429),
      this.instances4.alice.encrypt64(98162433),
    );
    expect(res).to.equal(98162433);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (98162433, 98162433)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(98162433),
      this.instances4.alice.encrypt64(98162433),
    );
    expect(res).to.equal(98162433);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (98162433, 98162429)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(98162433),
      this.instances4.alice.encrypt64(98162429),
    );
    expect(res).to.equal(98162433);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (85283614, 1598412404)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(85283614), 1598412404);
    expect(res).to.equal(1683696018);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (85283610, 85283614)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(85283610), 85283614);
    expect(res).to.equal(170567224);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (85283614, 85283614)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(85283614), 85283614);
    expect(res).to.equal(170567228);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (85283614, 85283610)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(85283614), 85283610);
    expect(res).to.equal(170567224);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (343329587, 1598412404)', async function () {
    const res = await this.contract4.add_uint32_euint32(343329587, this.instances4.alice.encrypt32(1598412404));
    expect(res).to.equal(1941741991);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 2 (85283610, 85283614)', async function () {
    const res = await this.contract4.add_uint32_euint32(85283610, this.instances4.alice.encrypt32(85283614));
    expect(res).to.equal(170567224);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 3 (85283614, 85283614)', async function () {
    const res = await this.contract4.add_uint32_euint32(85283614, this.instances4.alice.encrypt32(85283614));
    expect(res).to.equal(170567228);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 4 (85283614, 85283610)', async function () {
    const res = await this.contract4.add_uint32_euint32(85283614, this.instances4.alice.encrypt32(85283610));
    expect(res).to.equal(170567224);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (949346704, 949346704)', async function () {
    const res = await this.contract4.sub_euint32_uint32(this.instances4.alice.encrypt32(949346704), 949346704);
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 2 (949346704, 949346700)', async function () {
    const res = await this.contract4.sub_euint32_uint32(this.instances4.alice.encrypt32(949346704), 949346700);
    expect(res).to.equal(4);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (949346704, 949346704)', async function () {
    const res = await this.contract4.sub_uint32_euint32(949346704, this.instances4.alice.encrypt32(949346704));
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 2 (949346704, 949346700)', async function () {
    const res = await this.contract4.sub_uint32_euint32(949346704, this.instances4.alice.encrypt32(949346700));
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (28489, 51753)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(28489), 51753);
    expect(res).to.equal(1474391217);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 2 (56978, 56978)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(56978), 56978);
    expect(res).to.equal(3246492484);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 3 (56978, 56978)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(56978), 56978);
    expect(res).to.equal(3246492484);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 4 (56978, 56978)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(56978), 56978);
    expect(res).to.equal(3246492484);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (72675, 51753)', async function () {
    const res = await this.contract4.mul_uint32_euint32(72675, this.instances4.alice.encrypt32(51753));
    expect(res).to.equal(3761149275);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (56978, 56978)', async function () {
    const res = await this.contract4.mul_uint32_euint32(56978, this.instances4.alice.encrypt32(56978));
    expect(res).to.equal(3246492484);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (56978, 56978)', async function () {
    const res = await this.contract4.mul_uint32_euint32(56978, this.instances4.alice.encrypt32(56978));
    expect(res).to.equal(3246492484);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (56978, 56978)', async function () {
    const res = await this.contract4.mul_uint32_euint32(56978, this.instances4.alice.encrypt32(56978));
    expect(res).to.equal(3246492484);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (836400951, 1557627728)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(836400951), 1557627728);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (392005478, 392005482)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(392005478), 392005482);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (392005482, 392005482)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(392005482), 392005482);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (392005482, 392005478)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(392005482), 392005478);
    expect(res).to.equal(1);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (1083212025, 1390178884)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(1083212025), 1390178884);
    expect(res).to.equal(1083212025);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 2 (1083212021, 1083212025)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(1083212021), 1083212025);
    expect(res).to.equal(1083212021);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 3 (1083212025, 1083212025)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(1083212025), 1083212025);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 4 (1083212025, 1083212021)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(1083212025), 1083212021);
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (1270694518, 945618490)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(1270694518), 945618490);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (247535164, 247535168)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(247535164), 247535168);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 3 (247535168, 247535168)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(247535168), 247535168);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 4 (247535168, 247535164)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(247535168), 247535164);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (1877579186, 945618490)', async function () {
    const res = await this.contract4.eq_uint32_euint32(1877579186, this.instances4.alice.encrypt32(945618490));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (247535164, 247535168)', async function () {
    const res = await this.contract4.eq_uint32_euint32(247535164, this.instances4.alice.encrypt32(247535168));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 3 (247535168, 247535168)', async function () {
    const res = await this.contract4.eq_uint32_euint32(247535168, this.instances4.alice.encrypt32(247535168));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 4 (247535168, 247535164)', async function () {
    const res = await this.contract4.eq_uint32_euint32(247535168, this.instances4.alice.encrypt32(247535164));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (1211702728, 426850373)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(1211702728), 426850373);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (563095023, 563095027)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(563095023), 563095027);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 3 (563095027, 563095027)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(563095027), 563095027);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 4 (563095027, 563095023)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(563095027), 563095023);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (1464361694, 426850373)', async function () {
    const res = await this.contract4.ne_uint32_euint32(1464361694, this.instances4.alice.encrypt32(426850373));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (563095023, 563095027)', async function () {
    const res = await this.contract4.ne_uint32_euint32(563095023, this.instances4.alice.encrypt32(563095027));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 3 (563095027, 563095027)', async function () {
    const res = await this.contract4.ne_uint32_euint32(563095027, this.instances4.alice.encrypt32(563095027));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 4 (563095027, 563095023)', async function () {
    const res = await this.contract4.ne_uint32_euint32(563095027, this.instances4.alice.encrypt32(563095023));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (627970552, 1986041075)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(627970552), 1986041075);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (627970548, 627970552)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(627970548), 627970552);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (627970552, 627970552)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(627970552), 627970552);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 4 (627970552, 627970548)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(627970552), 627970548);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (1972025848, 1986041075)', async function () {
    const res = await this.contract4.ge_uint32_euint32(1972025848, this.instances4.alice.encrypt32(1986041075));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (627970548, 627970552)', async function () {
    const res = await this.contract4.ge_uint32_euint32(627970548, this.instances4.alice.encrypt32(627970552));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (627970552, 627970552)', async function () {
    const res = await this.contract4.ge_uint32_euint32(627970552, this.instances4.alice.encrypt32(627970552));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (627970552, 627970548)', async function () {
    const res = await this.contract4.ge_uint32_euint32(627970552, this.instances4.alice.encrypt32(627970548));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (1801714413, 448229258)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(1801714413), 448229258);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (831227100, 831227104)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(831227100), 831227104);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (831227104, 831227104)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(831227104), 831227104);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 4 (831227104, 831227100)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(831227104), 831227100);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (258393672, 448229258)', async function () {
    const res = await this.contract4.gt_uint32_euint32(258393672, this.instances4.alice.encrypt32(448229258));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (831227100, 831227104)', async function () {
    const res = await this.contract4.gt_uint32_euint32(831227100, this.instances4.alice.encrypt32(831227104));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (831227104, 831227104)', async function () {
    const res = await this.contract4.gt_uint32_euint32(831227104, this.instances4.alice.encrypt32(831227104));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 4 (831227104, 831227100)', async function () {
    const res = await this.contract4.gt_uint32_euint32(831227104, this.instances4.alice.encrypt32(831227100));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (523722139, 1994886988)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(523722139), 1994886988);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (523722135, 523722139)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(523722135), 523722139);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (523722139, 523722139)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(523722139), 523722139);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 4 (523722139, 523722135)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(523722139), 523722135);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (1167528973, 1994886988)', async function () {
    const res = await this.contract4.le_uint32_euint32(1167528973, this.instances4.alice.encrypt32(1994886988));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (523722135, 523722139)', async function () {
    const res = await this.contract4.le_uint32_euint32(523722135, this.instances4.alice.encrypt32(523722139));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (523722139, 523722139)', async function () {
    const res = await this.contract4.le_uint32_euint32(523722139, this.instances4.alice.encrypt32(523722139));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 4 (523722139, 523722135)', async function () {
    const res = await this.contract4.le_uint32_euint32(523722139, this.instances4.alice.encrypt32(523722135));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (1044281941, 1474237696)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(1044281941), 1474237696);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (27475167, 27475171)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(27475167), 27475171);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (27475171, 27475171)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(27475171), 27475171);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 4 (27475171, 27475167)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(27475171), 27475167);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (1829077110, 1474237696)', async function () {
    const res = await this.contract4.lt_uint32_euint32(1829077110, this.instances4.alice.encrypt32(1474237696));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (27475167, 27475171)', async function () {
    const res = await this.contract4.lt_uint32_euint32(27475167, this.instances4.alice.encrypt32(27475171));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (27475171, 27475171)', async function () {
    const res = await this.contract4.lt_uint32_euint32(27475171, this.instances4.alice.encrypt32(27475171));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 4 (27475171, 27475167)', async function () {
    const res = await this.contract4.lt_uint32_euint32(27475171, this.instances4.alice.encrypt32(27475167));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (390943086, 304405118)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(390943086), 304405118);
    expect(res).to.equal(304405118);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (390943082, 390943086)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(390943082), 390943086);
    expect(res).to.equal(390943082);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (390943086, 390943086)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(390943086), 390943086);
    expect(res).to.equal(390943086);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 4 (390943086, 390943082)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(390943086), 390943082);
    expect(res).to.equal(390943082);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (1819271941, 304405118)', async function () {
    const res = await this.contract4.min_uint32_euint32(1819271941, this.instances4.alice.encrypt32(304405118));
    expect(res).to.equal(304405118);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (390943082, 390943086)', async function () {
    const res = await this.contract4.min_uint32_euint32(390943082, this.instances4.alice.encrypt32(390943086));
    expect(res).to.equal(390943082);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (390943086, 390943086)', async function () {
    const res = await this.contract4.min_uint32_euint32(390943086, this.instances4.alice.encrypt32(390943086));
    expect(res).to.equal(390943086);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 4 (390943086, 390943082)', async function () {
    const res = await this.contract4.min_uint32_euint32(390943086, this.instances4.alice.encrypt32(390943082));
    expect(res).to.equal(390943082);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (283314521, 1397934692)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(283314521), 1397934692);
    expect(res).to.equal(1397934692);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (283314517, 283314521)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(283314517), 283314521);
    expect(res).to.equal(283314521);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (283314521, 283314521)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(283314521), 283314521);
    expect(res).to.equal(283314521);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 4 (283314521, 283314517)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(283314521), 283314517);
    expect(res).to.equal(283314521);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (1446821827, 1397934692)', async function () {
    const res = await this.contract4.max_uint32_euint32(1446821827, this.instances4.alice.encrypt32(1397934692));
    expect(res).to.equal(1446821827);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (283314517, 283314521)', async function () {
    const res = await this.contract4.max_uint32_euint32(283314517, this.instances4.alice.encrypt32(283314521));
    expect(res).to.equal(283314521);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (283314521, 283314521)', async function () {
    const res = await this.contract4.max_uint32_euint32(283314521, this.instances4.alice.encrypt32(283314521));
    expect(res).to.equal(283314521);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 4 (283314521, 283314517)', async function () {
    const res = await this.contract4.max_uint32_euint32(283314521, this.instances4.alice.encrypt32(283314517));
    expect(res).to.equal(283314521);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 1 (11, 1)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(11),
      this.instances4.alice.encrypt4(1),
    );
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 3 (4, 4)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(12);
  });

  it('test operator "sub" overload (euint64, euint4) => euint64 test 1 (8, 8)', async function () {
    const res = await this.contract4.sub_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, euint4) => euint64 test 2 (8, 4)', async function () {
    const res = await this.contract4.sub_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 1 (8, 1)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(1),
    );
    expect(res).to.equal(8);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 2 (2, 4)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(2),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(8);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 3 (2, 2)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(2),
      this.instances4.alice.encrypt4(2),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 4 (4, 2)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(2),
    );
    expect(res).to.equal(8);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 1 (1294685955, 14)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(1294685955),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(2);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 2 (10, 14)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(10);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 3 (14, 14)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(14),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 4 (14, 10)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(14),
      this.instances4.alice.encrypt4(10),
    );
    expect(res).to.equal(10);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 1 (10, 1)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt4(1),
    );
    expect(res).to.equal(11);
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

  it('test operator "xor" overload (euint64, euint4) => euint64 test 1 (15, 1)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(15),
      this.instances4.alice.encrypt4(1),
    );
    expect(res).to.equal(14);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 2 (8, 12)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(12),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 3 (12, 12)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(12),
      this.instances4.alice.encrypt4(12),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 4 (12, 8)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(12),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 1 (2078992513, 6)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(2078992513),
      this.instances4.alice.encrypt4(6),
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

  it('test operator "ne" overload (euint64, euint4) => ebool test 1 (1037057063, 9)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(1037057063),
      this.instances4.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(5),
      this.instances4.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(9),
      this.instances4.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(9),
      this.instances4.alice.encrypt4(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 1 (1885602752, 6)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(1885602752),
      this.instances4.alice.encrypt4(6),
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

  it('test operator "gt" overload (euint64, euint4) => ebool test 1 (307981806, 12)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(307981806),
      this.instances4.alice.encrypt4(12),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(12),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(12),
      this.instances4.alice.encrypt4(12),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(12),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 1 (2018662588, 9)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(2018662588),
      this.instances4.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(5),
      this.instances4.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(9),
      this.instances4.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(9),
      this.instances4.alice.encrypt4(5),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 1 (102600113, 13)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(102600113),
      this.instances4.alice.encrypt4(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(9),
      this.instances4.alice.encrypt4(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(13),
      this.instances4.alice.encrypt4(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(13),
      this.instances4.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 1 (1933734274, 2)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(1933734274),
      this.instances4.alice.encrypt4(2),
    );
    expect(res).to.equal(2);
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

  it('test operator "max" overload (euint64, euint4) => euint64 test 1 (14, 1)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(14),
      this.instances4.alice.encrypt4(1),
    );
    expect(res).to.equal(14);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 2 (9, 13)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(9),
      this.instances4.alice.encrypt4(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 3 (13, 13)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(13),
      this.instances4.alice.encrypt4(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 4 (13, 9)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(13),
      this.instances4.alice.encrypt4(9),
    );
    expect(res).to.equal(13);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 1 (181, 1)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(181),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(182);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 2 (66, 68)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(66),
      this.instances4.alice.encrypt8(68),
    );
    expect(res).to.equal(134);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 3 (68, 68)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(68),
      this.instances4.alice.encrypt8(68),
    );
    expect(res).to.equal(136);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 4 (68, 66)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(68),
      this.instances4.alice.encrypt8(66),
    );
    expect(res).to.equal(134);
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

  it('test operator "mul" overload (euint64, euint8) => euint64 test 1 (132, 1)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(132),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(132);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 2 (14, 15)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(14),
      this.instances4.alice.encrypt8(15),
    );
    expect(res).to.equal(210);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 3 (15, 15)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(15),
      this.instances4.alice.encrypt8(15),
    );
    expect(res).to.equal(225);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 4 (15, 14)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(15),
      this.instances4.alice.encrypt8(14),
    );
    expect(res).to.equal(210);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 1 (1294685955, 133)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(1294685955),
      this.instances4.alice.encrypt8(133),
    );
    expect(res).to.equal(1);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 2 (129, 133)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(129),
      this.instances4.alice.encrypt8(133),
    );
    expect(res).to.equal(129);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 3 (133, 133)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(133),
      this.instances4.alice.encrypt8(133),
    );
    expect(res).to.equal(133);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 4 (133, 129)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(133),
      this.instances4.alice.encrypt8(129),
    );
    expect(res).to.equal(129);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 1 (167, 1)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(167),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(167);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 2 (97, 101)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(97),
      this.instances4.alice.encrypt8(101),
    );
    expect(res).to.equal(101);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 3 (101, 101)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(101),
      this.instances4.alice.encrypt8(101),
    );
    expect(res).to.equal(101);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 4 (101, 97)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(101),
      this.instances4.alice.encrypt8(97),
    );
    expect(res).to.equal(101);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 1 (246, 1)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(246),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(247);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 2 (22, 26)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(22),
      this.instances4.alice.encrypt8(26),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 3 (26, 26)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(26),
      this.instances4.alice.encrypt8(26),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 4 (26, 22)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(26),
      this.instances4.alice.encrypt8(22),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 1 (2078992513, 51)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(2078992513),
      this.instances4.alice.encrypt8(51),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 2 (47, 51)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(47),
      this.instances4.alice.encrypt8(51),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 3 (51, 51)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(51),
      this.instances4.alice.encrypt8(51),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 4 (51, 47)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(51),
      this.instances4.alice.encrypt8(47),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 1 (1037057063, 209)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(1037057063),
      this.instances4.alice.encrypt8(209),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 2 (205, 209)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(205),
      this.instances4.alice.encrypt8(209),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 3 (209, 209)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(209),
      this.instances4.alice.encrypt8(209),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 4 (209, 205)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(209),
      this.instances4.alice.encrypt8(205),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 1 (1885602752, 64)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(1885602752),
      this.instances4.alice.encrypt8(64),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 2 (60, 64)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(60),
      this.instances4.alice.encrypt8(64),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 3 (64, 64)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(64),
      this.instances4.alice.encrypt8(64),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 4 (64, 60)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(64),
      this.instances4.alice.encrypt8(60),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 1 (307981806, 246)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(307981806),
      this.instances4.alice.encrypt8(246),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 2 (242, 246)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(242),
      this.instances4.alice.encrypt8(246),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 3 (246, 246)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(246),
      this.instances4.alice.encrypt8(246),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 4 (246, 242)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(246),
      this.instances4.alice.encrypt8(242),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 1 (2018662588, 245)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(2018662588),
      this.instances5.alice.encrypt8(245),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 2 (241, 245)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(241),
      this.instances5.alice.encrypt8(245),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 3 (245, 245)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(245),
      this.instances5.alice.encrypt8(245),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 4 (245, 241)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(245),
      this.instances5.alice.encrypt8(241),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 1 (102600113, 117)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(102600113),
      this.instances5.alice.encrypt8(117),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 2 (113, 117)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(113),
      this.instances5.alice.encrypt8(117),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 3 (117, 117)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(117),
      this.instances5.alice.encrypt8(117),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 4 (117, 113)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(117),
      this.instances5.alice.encrypt8(113),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 1 (1933734274, 92)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(1933734274),
      this.instances5.alice.encrypt8(92),
    );
    expect(res).to.equal(92);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 2 (88, 92)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(88),
      this.instances5.alice.encrypt8(92),
    );
    expect(res).to.equal(88);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 3 (92, 92)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(92),
      this.instances5.alice.encrypt8(92),
    );
    expect(res).to.equal(92);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 4 (92, 88)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(92),
      this.instances5.alice.encrypt8(88),
    );
    expect(res).to.equal(88);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 1 (231, 1)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(231),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(231);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 2 (210, 214)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(210),
      this.instances5.alice.encrypt8(214),
    );
    expect(res).to.equal(214);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 3 (214, 214)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(214),
      this.instances5.alice.encrypt8(214),
    );
    expect(res).to.equal(214);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 4 (214, 210)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(214),
      this.instances5.alice.encrypt8(210),
    );
    expect(res).to.equal(214);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 1 (46497, 1)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(46497),
      this.instances5.alice.encrypt16(1),
    );
    expect(res).to.equal(46498);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 2 (1781, 1785)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(1781),
      this.instances5.alice.encrypt16(1785),
    );
    expect(res).to.equal(3566);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 3 (1785, 1785)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(1785),
      this.instances5.alice.encrypt16(1785),
    );
    expect(res).to.equal(3570);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 4 (1785, 1781)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(1785),
      this.instances5.alice.encrypt16(1781),
    );
    expect(res).to.equal(3566);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 1 (42609, 42609)', async function () {
    const res = await this.contract5.sub_euint64_euint16(
      this.instances5.alice.encrypt64(42609),
      this.instances5.alice.encrypt16(42609),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 2 (42609, 42605)', async function () {
    const res = await this.contract5.sub_euint64_euint16(
      this.instances5.alice.encrypt64(42609),
      this.instances5.alice.encrypt16(42605),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 1 (16971, 3)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(16971),
      this.instances5.alice.encrypt16(3),
    );
    expect(res).to.equal(50913);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 2 (227, 227)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(227),
      this.instances5.alice.encrypt16(227),
    );
    expect(res).to.equal(51529);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 3 (227, 227)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(227),
      this.instances5.alice.encrypt16(227),
    );
    expect(res).to.equal(51529);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 4 (227, 227)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(227),
      this.instances5.alice.encrypt16(227),
    );
    expect(res).to.equal(51529);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 1 (1294685955, 63959)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(1294685955),
      this.instances5.alice.encrypt16(63959),
    );
    expect(res).to.equal(20739);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 2 (63955, 63959)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(63955),
      this.instances5.alice.encrypt16(63959),
    );
    expect(res).to.equal(63955);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 3 (63959, 63959)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(63959),
      this.instances5.alice.encrypt16(63959),
    );
    expect(res).to.equal(63959);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 4 (63959, 63955)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(63959),
      this.instances5.alice.encrypt16(63955),
    );
    expect(res).to.equal(63955);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 1 (42889, 1)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(42889),
      this.instances5.alice.encrypt16(1),
    );
    expect(res).to.equal(42889);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 2 (21447, 21451)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(21447),
      this.instances5.alice.encrypt16(21451),
    );
    expect(res).to.equal(21455);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 3 (21451, 21451)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(21451),
      this.instances5.alice.encrypt16(21451),
    );
    expect(res).to.equal(21451);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 4 (21451, 21447)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(21451),
      this.instances5.alice.encrypt16(21447),
    );
    expect(res).to.equal(21455);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 1 (63024, 2)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(63024),
      this.instances5.alice.encrypt16(2),
    );
    expect(res).to.equal(63026);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 2 (33630, 33634)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(33630),
      this.instances5.alice.encrypt16(33634),
    );
    expect(res).to.equal(60);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 3 (33634, 33634)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(33634),
      this.instances5.alice.encrypt16(33634),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 4 (33634, 33630)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(33634),
      this.instances5.alice.encrypt16(33630),
    );
    expect(res).to.equal(60);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 1 (2078992513, 32629)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(2078992513),
      this.instances5.alice.encrypt16(32629),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 2 (32625, 32629)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(32625),
      this.instances5.alice.encrypt16(32629),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 3 (32629, 32629)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(32629),
      this.instances5.alice.encrypt16(32629),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 4 (32629, 32625)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(32629),
      this.instances5.alice.encrypt16(32625),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 1 (1037057063, 50271)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(1037057063),
      this.instances5.alice.encrypt16(50271),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 2 (50267, 50271)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(50267),
      this.instances5.alice.encrypt16(50271),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 3 (50271, 50271)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(50271),
      this.instances5.alice.encrypt16(50271),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 4 (50271, 50267)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(50271),
      this.instances5.alice.encrypt16(50267),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 1 (1885602752, 42598)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(1885602752),
      this.instances5.alice.encrypt16(42598),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 2 (42594, 42598)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(42594),
      this.instances5.alice.encrypt16(42598),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 3 (42598, 42598)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(42598),
      this.instances5.alice.encrypt16(42598),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 4 (42598, 42594)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(42598),
      this.instances5.alice.encrypt16(42594),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 1 (307981806, 10827)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(307981806),
      this.instances5.alice.encrypt16(10827),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 2 (10823, 10827)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(10823),
      this.instances5.alice.encrypt16(10827),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 3 (10827, 10827)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(10827),
      this.instances5.alice.encrypt16(10827),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 4 (10827, 10823)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(10827),
      this.instances5.alice.encrypt16(10823),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 1 (2018662588, 22989)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(2018662588),
      this.instances5.alice.encrypt16(22989),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 2 (22985, 22989)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(22985),
      this.instances5.alice.encrypt16(22989),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 3 (22989, 22989)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(22989),
      this.instances5.alice.encrypt16(22989),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 4 (22989, 22985)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(22989),
      this.instances5.alice.encrypt16(22985),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 1 (102600113, 19620)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(102600113),
      this.instances5.alice.encrypt16(19620),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 2 (19616, 19620)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(19616),
      this.instances5.alice.encrypt16(19620),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 3 (19620, 19620)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(19620),
      this.instances5.alice.encrypt16(19620),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 4 (19620, 19616)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(19620),
      this.instances5.alice.encrypt16(19616),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 1 (1933734274, 63835)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(1933734274),
      this.instances5.alice.encrypt16(63835),
    );
    expect(res).to.equal(63835);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 2 (63831, 63835)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(63831),
      this.instances5.alice.encrypt16(63835),
    );
    expect(res).to.equal(63831);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 3 (63835, 63835)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(63835),
      this.instances5.alice.encrypt16(63835),
    );
    expect(res).to.equal(63835);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 4 (63835, 63831)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(63835),
      this.instances5.alice.encrypt16(63831),
    );
    expect(res).to.equal(63831);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (59321, 1)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(59321),
      this.instances5.alice.encrypt16(1),
    );
    expect(res).to.equal(59321);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (49394, 49398)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(49394),
      this.instances5.alice.encrypt16(49398),
    );
    expect(res).to.equal(49398);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (49398, 49398)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(49398),
      this.instances5.alice.encrypt16(49398),
    );
    expect(res).to.equal(49398);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (49398, 49394)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(49398),
      this.instances5.alice.encrypt16(49394),
    );
    expect(res).to.equal(49398);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 1 (761810509, 2065807481)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(761810509),
      this.instances5.alice.encrypt32(2065807481),
    );
    expect(res).to.equal(2827617990);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 2 (761810505, 761810509)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(761810505),
      this.instances5.alice.encrypt32(761810509),
    );
    expect(res).to.equal(1523621014);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 3 (761810509, 761810509)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(761810509),
      this.instances5.alice.encrypt32(761810509),
    );
    expect(res).to.equal(1523621018);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 4 (761810509, 761810505)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(761810509),
      this.instances5.alice.encrypt32(761810505),
    );
    expect(res).to.equal(1523621014);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 1 (1679860005, 1679860005)', async function () {
    const res = await this.contract5.sub_euint64_euint32(
      this.instances5.alice.encrypt64(1679860005),
      this.instances5.alice.encrypt32(1679860005),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 2 (1679860005, 1679860001)', async function () {
    const res = await this.contract5.sub_euint64_euint32(
      this.instances5.alice.encrypt64(1679860005),
      this.instances5.alice.encrypt32(1679860001),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 1 (33943, 59095)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(33943),
      this.instances5.alice.encrypt32(59095),
    );
    expect(res).to.equal(2005861585);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 2 (33943, 33943)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(33943),
      this.instances5.alice.encrypt32(33943),
    );
    expect(res).to.equal(1152127249);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 3 (33943, 33943)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(33943),
      this.instances5.alice.encrypt32(33943),
    );
    expect(res).to.equal(1152127249);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 4 (33943, 33943)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(33943),
      this.instances5.alice.encrypt32(33943),
    );
    expect(res).to.equal(1152127249);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 1 (1294685955, 1745450667)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(1294685955),
      this.instances5.alice.encrypt32(1745450667),
    );
    expect(res).to.equal(1208571395);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 2 (1294685951, 1294685955)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(1294685951),
      this.instances5.alice.encrypt32(1294685955),
    );
    expect(res).to.equal(1294685699);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 3 (1294685955, 1294685955)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(1294685955),
      this.instances5.alice.encrypt32(1294685955),
    );
    expect(res).to.equal(1294685955);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 4 (1294685955, 1294685951)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(1294685955),
      this.instances5.alice.encrypt32(1294685951),
    );
    expect(res).to.equal(1294685699);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 1 (1405387766, 963544644)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(1405387766),
      this.instances5.alice.encrypt32(963544644),
    );
    expect(res).to.equal(2079229942);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 2 (963544640, 963544644)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(963544640),
      this.instances5.alice.encrypt32(963544644),
    );
    expect(res).to.equal(963544644);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 3 (963544644, 963544644)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(963544644),
      this.instances5.alice.encrypt32(963544644),
    );
    expect(res).to.equal(963544644);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 4 (963544644, 963544640)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(963544644),
      this.instances5.alice.encrypt32(963544640),
    );
    expect(res).to.equal(963544644);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 1 (1032596672, 1451108916)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(1032596672),
      this.instances5.alice.encrypt32(1451108916),
    );
    expect(res).to.equal(1811023604);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 2 (1032596668, 1032596672)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(1032596668),
      this.instances5.alice.encrypt32(1032596672),
    );
    expect(res).to.equal(124);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 3 (1032596672, 1032596672)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(1032596672),
      this.instances5.alice.encrypt32(1032596672),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 4 (1032596672, 1032596668)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(1032596672),
      this.instances5.alice.encrypt32(1032596668),
    );
    expect(res).to.equal(124);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 1 (2078992513, 810545451)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(2078992513),
      this.instances5.alice.encrypt32(810545451),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 2 (810545447, 810545451)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(810545447),
      this.instances5.alice.encrypt32(810545451),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 3 (810545451, 810545451)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(810545451),
      this.instances5.alice.encrypt32(810545451),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 4 (810545451, 810545447)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(810545451),
      this.instances5.alice.encrypt32(810545447),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 1 (1037057063, 358604604)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(1037057063),
      this.instances5.alice.encrypt32(358604604),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 2 (358604600, 358604604)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(358604600),
      this.instances5.alice.encrypt32(358604604),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 3 (358604604, 358604604)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(358604604),
      this.instances5.alice.encrypt32(358604604),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 4 (358604604, 358604600)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(358604604),
      this.instances5.alice.encrypt32(358604600),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 1 (1885602752, 2120758041)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(1885602752),
      this.instances5.alice.encrypt32(2120758041),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 2 (1885602748, 1885602752)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(1885602748),
      this.instances5.alice.encrypt32(1885602752),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 3 (1885602752, 1885602752)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(1885602752),
      this.instances5.alice.encrypt32(1885602752),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 4 (1885602752, 1885602748)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(1885602752),
      this.instances5.alice.encrypt32(1885602748),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 1 (307981806, 973826729)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(307981806),
      this.instances5.alice.encrypt32(973826729),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 2 (307981802, 307981806)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(307981802),
      this.instances5.alice.encrypt32(307981806),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 3 (307981806, 307981806)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(307981806),
      this.instances5.alice.encrypt32(307981806),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 4 (307981806, 307981802)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(307981806),
      this.instances5.alice.encrypt32(307981802),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (2018662588, 812190927)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(2018662588),
      this.instances5.alice.encrypt32(812190927),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (812190923, 812190927)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(812190923),
      this.instances5.alice.encrypt32(812190927),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (812190927, 812190927)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(812190927),
      this.instances5.alice.encrypt32(812190927),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (812190927, 812190923)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(812190927),
      this.instances5.alice.encrypt32(812190923),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (102600113, 1018674751)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(102600113),
      this.instances5.alice.encrypt32(1018674751),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (102600109, 102600113)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(102600109),
      this.instances5.alice.encrypt32(102600113),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (102600113, 102600113)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(102600113),
      this.instances5.alice.encrypt32(102600113),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (102600113, 102600109)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(102600113),
      this.instances5.alice.encrypt32(102600109),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (1933734274, 1600509978)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(1933734274),
      this.instances5.alice.encrypt32(1600509978),
    );
    expect(res).to.equal(1600509978);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (1600509974, 1600509978)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(1600509974),
      this.instances5.alice.encrypt32(1600509978),
    );
    expect(res).to.equal(1600509974);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (1600509978, 1600509978)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(1600509978),
      this.instances5.alice.encrypt32(1600509978),
    );
    expect(res).to.equal(1600509978);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (1600509978, 1600509974)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(1600509978),
      this.instances5.alice.encrypt32(1600509974),
    );
    expect(res).to.equal(1600509974);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (1943842367, 2024517605)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(1943842367),
      this.instances5.alice.encrypt32(2024517605),
    );
    expect(res).to.equal(2024517605);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (1943842363, 1943842367)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(1943842363),
      this.instances5.alice.encrypt32(1943842367),
    );
    expect(res).to.equal(1943842367);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (1943842367, 1943842367)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(1943842367),
      this.instances5.alice.encrypt32(1943842367),
    );
    expect(res).to.equal(1943842367);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (1943842367, 1943842363)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(1943842367),
      this.instances5.alice.encrypt32(1943842363),
    );
    expect(res).to.equal(1943842367);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (761810509, 74167544)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(761810509),
      this.instances5.alice.encrypt64(74167544),
    );
    expect(res).to.equal(835978053);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (74167540, 74167544)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(74167540),
      this.instances5.alice.encrypt64(74167544),
    );
    expect(res).to.equal(148335084);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (74167544, 74167544)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(74167544),
      this.instances5.alice.encrypt64(74167544),
    );
    expect(res).to.equal(148335088);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (74167544, 74167540)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(74167544),
      this.instances5.alice.encrypt64(74167540),
    );
    expect(res).to.equal(148335084);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (1450674385, 1450674385)', async function () {
    const res = await this.contract5.sub_euint64_euint64(
      this.instances5.alice.encrypt64(1450674385),
      this.instances5.alice.encrypt64(1450674385),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (1450674385, 1450674381)', async function () {
    const res = await this.contract5.sub_euint64_euint64(
      this.instances5.alice.encrypt64(1450674385),
      this.instances5.alice.encrypt64(1450674381),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (278067877, 1869129475)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(278067877),
      this.instances5.alice.encrypt64(1869129475),
    );
    expect(res).to.equal(519744864951374600);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (278067873, 278067877)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(278067873),
      this.instances5.alice.encrypt64(278067877),
    );
    expect(res).to.equal(77321743107015620);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (278067877, 278067877)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(278067877),
      this.instances5.alice.encrypt64(278067877),
    );
    expect(res).to.equal(77321744219287140);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (278067877, 278067873)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(278067877),
      this.instances5.alice.encrypt64(278067873),
    );
    expect(res).to.equal(77321743107015620);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (1294685955, 55045679)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(1294685955),
      this.instances5.alice.encrypt64(55045679),
    );
    expect(res).to.equal(16991747);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (55045675, 55045679)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(55045675),
      this.instances5.alice.encrypt64(55045679),
    );
    expect(res).to.equal(55045675);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (55045679, 55045679)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(55045679),
      this.instances5.alice.encrypt64(55045679),
    );
    expect(res).to.equal(55045679);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (55045679, 55045675)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(55045679),
      this.instances5.alice.encrypt64(55045675),
    );
    expect(res).to.equal(55045675);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 1 (1405387766, 1463413902)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(1405387766),
      this.instances5.alice.encrypt64(1463413902),
    );
    expect(res).to.equal(1476259838);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 2 (1405387762, 1405387766)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(1405387762),
      this.instances5.alice.encrypt64(1405387766),
    );
    expect(res).to.equal(1405387766);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 3 (1405387766, 1405387766)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(1405387766),
      this.instances5.alice.encrypt64(1405387766),
    );
    expect(res).to.equal(1405387766);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 4 (1405387766, 1405387762)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(1405387766),
      this.instances5.alice.encrypt64(1405387762),
    );
    expect(res).to.equal(1405387766);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 1 (1032596672, 34433579)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(1032596672),
      this.instances5.alice.encrypt64(34433579),
    );
    expect(res).to.equal(1065436907);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 2 (34433575, 34433579)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(34433575),
      this.instances5.alice.encrypt64(34433579),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 3 (34433579, 34433579)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(34433579),
      this.instances5.alice.encrypt64(34433579),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 4 (34433579, 34433575)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(34433579),
      this.instances5.alice.encrypt64(34433575),
    );
    expect(res).to.equal(12);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 1 (2078992513, 1060802657)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(2078992513),
      this.instances5.alice.encrypt64(1060802657),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 2 (1060802653, 1060802657)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(1060802653),
      this.instances5.alice.encrypt64(1060802657),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 3 (1060802657, 1060802657)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(1060802657),
      this.instances5.alice.encrypt64(1060802657),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 4 (1060802657, 1060802653)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(1060802657),
      this.instances5.alice.encrypt64(1060802653),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (1037057063, 451065562)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(1037057063),
      this.instances5.alice.encrypt64(451065562),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (451065558, 451065562)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(451065558),
      this.instances5.alice.encrypt64(451065562),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (451065562, 451065562)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(451065562),
      this.instances5.alice.encrypt64(451065562),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (451065562, 451065558)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(451065562),
      this.instances5.alice.encrypt64(451065558),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 1 (1885602752, 1033076257)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(1885602752),
      this.instances5.alice.encrypt64(1033076257),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 2 (1033076253, 1033076257)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(1033076253),
      this.instances5.alice.encrypt64(1033076257),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 3 (1033076257, 1033076257)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(1033076257),
      this.instances5.alice.encrypt64(1033076257),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 4 (1033076257, 1033076253)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(1033076257),
      this.instances5.alice.encrypt64(1033076253),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (307981806, 618210327)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(307981806),
      this.instances5.alice.encrypt64(618210327),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (307981802, 307981806)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(307981802),
      this.instances5.alice.encrypt64(307981806),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (307981806, 307981806)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(307981806),
      this.instances5.alice.encrypt64(307981806),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (307981806, 307981802)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(307981806),
      this.instances5.alice.encrypt64(307981802),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (2018662588, 1848670692)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(2018662588),
      this.instances5.alice.encrypt64(1848670692),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (1848670688, 1848670692)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(1848670688),
      this.instances5.alice.encrypt64(1848670692),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (1848670692, 1848670692)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(1848670692),
      this.instances5.alice.encrypt64(1848670692),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (1848670692, 1848670688)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(1848670692),
      this.instances5.alice.encrypt64(1848670688),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 1 (102600113, 1772405733)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(102600113),
      this.instances5.alice.encrypt64(1772405733),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 2 (102600109, 102600113)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(102600109),
      this.instances5.alice.encrypt64(102600113),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 3 (102600113, 102600113)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(102600113),
      this.instances5.alice.encrypt64(102600113),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 4 (102600113, 102600109)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(102600113),
      this.instances5.alice.encrypt64(102600109),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 1 (1933734274, 484998270)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(1933734274),
      this.instances5.alice.encrypt64(484998270),
    );
    expect(res).to.equal(484998270);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 2 (484998266, 484998270)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(484998266),
      this.instances5.alice.encrypt64(484998270),
    );
    expect(res).to.equal(484998266);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 3 (484998270, 484998270)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(484998270),
      this.instances5.alice.encrypt64(484998270),
    );
    expect(res).to.equal(484998270);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 4 (484998270, 484998266)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(484998270),
      this.instances5.alice.encrypt64(484998266),
    );
    expect(res).to.equal(484998266);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (1943842367, 1448051868)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(1943842367),
      this.instances5.alice.encrypt64(1448051868),
    );
    expect(res).to.equal(1943842367);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (1448051864, 1448051868)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(1448051864),
      this.instances5.alice.encrypt64(1448051868),
    );
    expect(res).to.equal(1448051868);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (1448051868, 1448051868)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(1448051868),
      this.instances5.alice.encrypt64(1448051868),
    );
    expect(res).to.equal(1448051868);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (1448051868, 1448051864)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(1448051868),
      this.instances5.alice.encrypt64(1448051864),
    );
    expect(res).to.equal(1448051868);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (761810509, 1968908307)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(761810509), 1968908307);
    expect(res).to.equal(2730718816);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (74167540, 74167544)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(74167540), 74167544);
    expect(res).to.equal(148335084);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (74167544, 74167544)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(74167544), 74167544);
    expect(res).to.equal(148335088);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (74167544, 74167540)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(74167544), 74167540);
    expect(res).to.equal(148335084);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 1 (1954867210, 1968908307)', async function () {
    const res = await this.contract5.add_uint64_euint64(1954867210, this.instances5.alice.encrypt64(1968908307));
    expect(res).to.equal(3923775517);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 2 (74167540, 74167544)', async function () {
    const res = await this.contract5.add_uint64_euint64(74167540, this.instances5.alice.encrypt64(74167544));
    expect(res).to.equal(148335084);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 3 (74167544, 74167544)', async function () {
    const res = await this.contract5.add_uint64_euint64(74167544, this.instances5.alice.encrypt64(74167544));
    expect(res).to.equal(148335088);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 4 (74167544, 74167540)', async function () {
    const res = await this.contract5.add_uint64_euint64(74167544, this.instances5.alice.encrypt64(74167540));
    expect(res).to.equal(148335084);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (1450674385, 1450674385)', async function () {
    const res = await this.contract5.sub_euint64_uint64(this.instances5.alice.encrypt64(1450674385), 1450674385);
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (1450674385, 1450674381)', async function () {
    const res = await this.contract5.sub_euint64_uint64(this.instances5.alice.encrypt64(1450674385), 1450674381);
    expect(res).to.equal(4);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (1450674385, 1450674385)', async function () {
    const res = await this.contract5.sub_uint64_euint64(1450674385, this.instances5.alice.encrypt64(1450674385));
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (1450674385, 1450674381)', async function () {
    const res = await this.contract5.sub_uint64_euint64(1450674385, this.instances5.alice.encrypt64(1450674381));
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (278067877, 45641442)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(278067877), 45641442);
    expect(res).to.equal(12691418880158634);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (278067873, 278067877)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(278067873), 278067877);
    expect(res).to.equal(77321743107015620);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (278067877, 278067877)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(278067877), 278067877);
    expect(res).to.equal(77321744219287140);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (278067877, 278067873)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(278067877), 278067873);
    expect(res).to.equal(77321743107015620);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 1 (1066035620, 45641442)', async function () {
    const res = await this.contract5.mul_uint64_euint64(1066035620, this.instances5.alice.encrypt64(45641442));
    expect(res).to.equal(48655402920164040);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 2 (278067873, 278067877)', async function () {
    const res = await this.contract5.mul_uint64_euint64(278067873, this.instances5.alice.encrypt64(278067877));
    expect(res).to.equal(77321743107015620);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 3 (278067877, 278067877)', async function () {
    const res = await this.contract5.mul_uint64_euint64(278067877, this.instances5.alice.encrypt64(278067877));
    expect(res).to.equal(77321744219287140);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 4 (278067877, 278067873)', async function () {
    const res = await this.contract5.mul_uint64_euint64(278067877, this.instances5.alice.encrypt64(278067873));
    expect(res).to.equal(77321743107015620);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (1965907341, 1364383301)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(1965907341), 1364383301);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (16210309, 16210313)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(16210309), 16210313);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (16210313, 16210313)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(16210313), 16210313);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (16210313, 16210309)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(16210313), 16210309);
    expect(res).to.equal(1);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 1 (1066501377, 1975528768)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(1066501377), 1975528768);
    expect(res).to.equal(1066501377);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 2 (1066501373, 1066501377)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(1066501373), 1066501377);
    expect(res).to.equal(1066501373);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 3 (1066501377, 1066501377)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(1066501377), 1066501377);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 4 (1066501377, 1066501373)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(1066501377), 1066501373);
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (2078992513, 415471663)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(2078992513), 415471663);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (1060802653, 1060802657)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(1060802653), 1060802657);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (1060802657, 1060802657)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(1060802657), 1060802657);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (1060802657, 1060802653)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(1060802657), 1060802653);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (810072111, 415471663)', async function () {
    const res = await this.contract5.eq_uint64_euint64(810072111, this.instances5.alice.encrypt64(415471663));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (1060802653, 1060802657)', async function () {
    const res = await this.contract5.eq_uint64_euint64(1060802653, this.instances5.alice.encrypt64(1060802657));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (1060802657, 1060802657)', async function () {
    const res = await this.contract5.eq_uint64_euint64(1060802657, this.instances5.alice.encrypt64(1060802657));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (1060802657, 1060802653)', async function () {
    const res = await this.contract5.eq_uint64_euint64(1060802657, this.instances5.alice.encrypt64(1060802653));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (1037057063, 1794841801)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(1037057063), 1794841801);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (451065558, 451065562)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(451065558), 451065562);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (451065562, 451065562)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(451065562), 451065562);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (451065562, 451065558)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(451065562), 451065558);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (1447708235, 1794841801)', async function () {
    const res = await this.contract5.ne_uint64_euint64(1447708235, this.instances5.alice.encrypt64(1794841801));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (451065558, 451065562)', async function () {
    const res = await this.contract5.ne_uint64_euint64(451065558, this.instances5.alice.encrypt64(451065562));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (451065562, 451065562)', async function () {
    const res = await this.contract5.ne_uint64_euint64(451065562, this.instances5.alice.encrypt64(451065562));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (451065562, 451065558)', async function () {
    const res = await this.contract5.ne_uint64_euint64(451065562, this.instances5.alice.encrypt64(451065558));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (1885602752, 494320060)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(1885602752), 494320060);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (1033076253, 1033076257)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(1033076253), 1033076257);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (1033076257, 1033076257)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(1033076257), 1033076257);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (1033076257, 1033076253)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(1033076257), 1033076253);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (1544604409, 494320060)', async function () {
    const res = await this.contract5.ge_uint64_euint64(1544604409, this.instances5.alice.encrypt64(494320060));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (1033076253, 1033076257)', async function () {
    const res = await this.contract5.ge_uint64_euint64(1033076253, this.instances5.alice.encrypt64(1033076257));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (1033076257, 1033076257)', async function () {
    const res = await this.contract5.ge_uint64_euint64(1033076257, this.instances5.alice.encrypt64(1033076257));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (1033076257, 1033076253)', async function () {
    const res = await this.contract5.ge_uint64_euint64(1033076257, this.instances5.alice.encrypt64(1033076253));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (307981806, 218365735)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(307981806), 218365735);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (307981802, 307981806)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(307981802), 307981806);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (307981806, 307981806)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(307981806), 307981806);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (307981806, 307981802)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(307981806), 307981802);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 1 (342213755, 218365735)', async function () {
    const res = await this.contract5.gt_uint64_euint64(342213755, this.instances5.alice.encrypt64(218365735));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 2 (307981802, 307981806)', async function () {
    const res = await this.contract5.gt_uint64_euint64(307981802, this.instances5.alice.encrypt64(307981806));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 3 (307981806, 307981806)', async function () {
    const res = await this.contract5.gt_uint64_euint64(307981806, this.instances5.alice.encrypt64(307981806));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 4 (307981806, 307981802)', async function () {
    const res = await this.contract5.gt_uint64_euint64(307981806, this.instances5.alice.encrypt64(307981802));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (2018662588, 32215999)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(2018662588), 32215999);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (1848670688, 1848670692)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(1848670688), 1848670692);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (1848670692, 1848670692)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(1848670692), 1848670692);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (1848670692, 1848670688)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(1848670692), 1848670688);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (119652561, 32215999)', async function () {
    const res = await this.contract5.le_uint64_euint64(119652561, this.instances5.alice.encrypt64(32215999));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (1848670688, 1848670692)', async function () {
    const res = await this.contract5.le_uint64_euint64(1848670688, this.instances5.alice.encrypt64(1848670692));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (1848670692, 1848670692)', async function () {
    const res = await this.contract5.le_uint64_euint64(1848670692, this.instances5.alice.encrypt64(1848670692));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (1848670692, 1848670688)', async function () {
    const res = await this.contract5.le_uint64_euint64(1848670692, this.instances5.alice.encrypt64(1848670688));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (102600113, 1945807436)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(102600113), 1945807436);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (102600109, 102600113)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(102600109), 102600113);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (102600113, 102600113)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(102600113), 102600113);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (102600113, 102600109)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(102600113), 102600109);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (1582166407, 1945807436)', async function () {
    const res = await this.contract5.lt_uint64_euint64(1582166407, this.instances5.alice.encrypt64(1945807436));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (102600109, 102600113)', async function () {
    const res = await this.contract5.lt_uint64_euint64(102600109, this.instances5.alice.encrypt64(102600113));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (102600113, 102600113)', async function () {
    const res = await this.contract5.lt_uint64_euint64(102600113, this.instances5.alice.encrypt64(102600113));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (102600113, 102600109)', async function () {
    const res = await this.contract5.lt_uint64_euint64(102600113, this.instances5.alice.encrypt64(102600109));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 1 (1933734274, 2130607481)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(1933734274), 2130607481);
    expect(res).to.equal(1933734274);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 2 (484998266, 484998270)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(484998266), 484998270);
    expect(res).to.equal(484998266);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 3 (484998270, 484998270)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(484998270), 484998270);
    expect(res).to.equal(484998270);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 4 (484998270, 484998266)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(484998270), 484998266);
    expect(res).to.equal(484998266);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (1509813623, 2130607481)', async function () {
    const res = await this.contract5.min_uint64_euint64(1509813623, this.instances5.alice.encrypt64(2130607481));
    expect(res).to.equal(1509813623);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (484998266, 484998270)', async function () {
    const res = await this.contract5.min_uint64_euint64(484998266, this.instances5.alice.encrypt64(484998270));
    expect(res).to.equal(484998266);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (484998270, 484998270)', async function () {
    const res = await this.contract5.min_uint64_euint64(484998270, this.instances5.alice.encrypt64(484998270));
    expect(res).to.equal(484998270);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (484998270, 484998266)', async function () {
    const res = await this.contract5.min_uint64_euint64(484998270, this.instances5.alice.encrypt64(484998266));
    expect(res).to.equal(484998266);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (1943842367, 447177064)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(1943842367), 447177064);
    expect(res).to.equal(1943842367);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (1448051864, 1448051868)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(1448051864), 1448051868);
    expect(res).to.equal(1448051868);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (1448051868, 1448051868)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(1448051868), 1448051868);
    expect(res).to.equal(1448051868);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (1448051868, 1448051864)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(1448051868), 1448051864);
    expect(res).to.equal(1448051868);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (776673220, 447177064)', async function () {
    const res = await this.contract5.max_uint64_euint64(776673220, this.instances5.alice.encrypt64(447177064));
    expect(res).to.equal(776673220);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (1448051864, 1448051868)', async function () {
    const res = await this.contract5.max_uint64_euint64(1448051864, this.instances5.alice.encrypt64(1448051868));
    expect(res).to.equal(1448051868);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (1448051868, 1448051868)', async function () {
    const res = await this.contract5.max_uint64_euint64(1448051868, this.instances5.alice.encrypt64(1448051868));
    expect(res).to.equal(1448051868);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (1448051868, 1448051864)', async function () {
    const res = await this.contract5.max_uint64_euint64(1448051868, this.instances5.alice.encrypt64(1448051864));
    expect(res).to.equal(1448051868);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 1 (1, 9)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(1), 9);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(4), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(8), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(8), 4);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 1 (5, 7)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(5), 7);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(4), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(8), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(8), 4);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (15, 7)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(15),
      this.instances5.alice.encrypt8(7),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (15, 173)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(15), 173);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(4), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(8), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(8), 4);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (113, 7)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(113),
      this.instances5.alice.encrypt8(7),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (113, 128)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(113), 128);
    expect(res).to.equal(113);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(4), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(8), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(8), 4);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (470, 1)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(470),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(235);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 1 (60273, 120)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(60273), 120);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(4), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(8), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(8), 4);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (386, 1)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(386),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(193);
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

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (49490, 216)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(49490), 216);
    expect(res).to.equal(0);
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

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (465, 1)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(465),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(232);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 1 (465, 1)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(465), 1);
    expect(res).to.equal(232);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(4), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(8), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(8), 4);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (315, 1)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(315),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(157);
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

  it('test operator "shr" overload (euint32, uint8) => euint32 test 1 (315, 1)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(315), 1);
    expect(res).to.equal(157);
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

  it('test operator "shl" overload (euint64, euint8) => euint64 test 1 (449, 1)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(449),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(224);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (1886374881, 60)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(1886374881), 60);
    expect(res).to.equal(7);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(4), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(8), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(8), 4);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (443, 1)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(443),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(221);
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

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (29073964, 153)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(29073964), 153);
    expect(res).to.equal(0);
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

  it('test operator "neg" overload (euint4) => euint4 test 1 (10)', async function () {
    const res = await this.contract5.neg_euint4(this.instances5.alice.encrypt4(10));
    expect(res).to.equal(5);
  });

  it('test operator "not" overload (euint4) => euint4 test 1 (2)', async function () {
    const res = await this.contract5.not_euint4(this.instances5.alice.encrypt4(2));
    expect(res).to.equal(13);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (29)', async function () {
    const res = await this.contract5.neg_euint8(this.instances5.alice.encrypt8(29));
    expect(res).to.equal(226);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (151)', async function () {
    const res = await this.contract5.not_euint8(this.instances5.alice.encrypt8(151));
    expect(res).to.equal(104);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (55816)', async function () {
    const res = await this.contract5.neg_euint16(this.instances5.alice.encrypt16(55816));
    expect(res).to.equal(9719);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (58147)', async function () {
    const res = await this.contract5.not_euint16(this.instances5.alice.encrypt16(58147));
    expect(res).to.equal(7388);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (384529386)', async function () {
    const res = await this.contract5.neg_euint32(this.instances5.alice.encrypt32(384529386));
    expect(res).to.equal(1762954261);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (1014323468)', async function () {
    const res = await this.contract5.not_euint32(this.instances5.alice.encrypt32(1014323468));
    expect(res).to.equal(1133160179);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (2136222472)', async function () {
    const res = await this.contract5.neg_euint64(this.instances5.alice.encrypt64(2136222472));
    expect(res).to.equal(11261175);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (566602200)', async function () {
    const res = await this.contract5.not_euint64(this.instances5.alice.encrypt64(566602200));
    expect(res).to.equal(1580881447);
  });
});
