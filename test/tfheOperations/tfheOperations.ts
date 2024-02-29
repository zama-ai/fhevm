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

  it('test operator "add" overload (euint4, euint4) => euint4 test 1 (4, 7)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(7),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 2 (5, 9)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 3 (4, 4)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 4 (9, 5)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "sub" overload (euint4, euint4) => euint4 test 1 (10, 10)', async function () {
    const res = await this.contract1.sub_euint4_euint4(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt4(10),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint4) => euint4 test 2 (10, 6)', async function () {
    const res = await this.contract1.sub_euint4_euint4(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt4(6),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 1 (7, 1)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(7n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 2 (2, 4)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 3 (2, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 4 (4, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 1 (1, 12)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt4(12),
    );
    expect(res).to.equal(0);
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

  it('test operator "or" overload (euint4, euint4) => euint4 test 1 (14, 12)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt4(12),
    );
    expect(res).to.equal(14);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 2 (8, 12)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(12),
    );
    expect(res).to.equal(12);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 3 (12, 12)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt4(12),
    );
    expect(res).to.equal(12);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 4 (12, 8)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 1 (13, 2)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(15);
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

  it('test operator "eq" overload (euint4, euint4) => ebool test 1 (5, 2)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt4(2),
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

  it('test operator "ne" overload (euint4, euint4) => ebool test 1 (7, 1)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt4(1),
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

  it('test operator "ge" overload (euint4, euint4) => ebool test 1 (9, 4)', async function () {
    const res = await this.contract1.ge_euint4_euint4(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
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

  it('test operator "gt" overload (euint4, euint4) => ebool test 1 (9, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 1 (10, 10)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt4(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt4(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt4(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt4(6),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 1 (4, 1)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(1),
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

  it('test operator "min" overload (euint4, euint4) => euint4 test 1 (1, 10)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt4(10),
    );
    expect(res).to.equal(1);
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

  it('test operator "max" overload (euint4, euint4) => euint4 test 1 (11, 7)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt4(7),
    );
    expect(res).to.equal(11);
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

  it('test operator "add" overload (euint4, euint8) => euint8 test 1 (1, 12)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(12),
    );
    expect(res).to.equal(13n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 2 (5, 9)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt8(9),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 3 (4, 4)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 4 (9, 5)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt8(5),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "sub" overload (euint4, euint8) => euint8 test 1 (10, 10)', async function () {
    const res = await this.contract1.sub_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint8) => euint8 test 2 (10, 6)', async function () {
    const res = await this.contract1.sub_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(6),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 1 (1, 9)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(9),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 2 (2, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt8(3),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt8(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 4 (3, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt8(2),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 1 (1, 208)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(208),
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

  it('test operator "or" overload (euint4, euint8) => euint8 test 1 (14, 81)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(81),
    );
    expect(res).to.equal(95);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 2 (10, 14)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 3 (14, 14)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 4 (14, 10)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(14);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 1 (13, 95)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt8(95),
    );
    expect(res).to.equal(82);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 2 (9, 13)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt8(13),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 3 (13, 13)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt8(13),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 4 (13, 9)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt8(9),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 1 (5, 81)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt8(81),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 1 (7, 29)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt8(29),
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

  it('test operator "ge" overload (euint4, euint8) => ebool test 1 (9, 39)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt8(39),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt8(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt8(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt8(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 1 (9, 247)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt8(247),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt8(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt8(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt8(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 1 (10, 82)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(82),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(6),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 1 (4, 146)', async function () {
    const res = await this.contract1.lt_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(146),
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

  it('test operator "min" overload (euint4, euint8) => euint8 test 1 (1, 21)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(21),
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

  it('test operator "max" overload (euint4, euint8) => euint8 test 1 (11, 223)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(223),
    );
    expect(res).to.equal(223);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 2 (7, 11)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt8(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 3 (11, 11)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 4 (11, 7)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(7),
    );
    expect(res).to.equal(11);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 1 (1, 9)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(9),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 2 (5, 9)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt16(9),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 3 (4, 4)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 4 (9, 5)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(5),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "sub" overload (euint4, euint16) => euint16 test 1 (10, 10)', async function () {
    const res = await this.contract1.sub_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(10),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint16) => euint16 test 2 (10, 6)', async function () {
    const res = await this.contract1.sub_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(6),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 1 (1, 13)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(13),
    );
    expect(res).to.equal(13n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 2 (2, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt16(3),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt16(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 4 (3, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt16(2),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 1 (1, 44006)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(44006),
    );
    expect(res).to.equal(0);
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

  it('test operator "or" overload (euint4, euint16) => euint16 test 1 (14, 32563)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt16(32563),
    );
    expect(res).to.equal(32575);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 2 (10, 14)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 3 (14, 14)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt16(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 4 (14, 10)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt16(10),
    );
    expect(res).to.equal(14);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 1 (13, 19075)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(19075),
    );
    expect(res).to.equal(19086);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 2 (9, 13)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(13),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 3 (13, 13)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(13),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 4 (13, 9)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(9),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 1 (5, 1690)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt16(1690),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 1 (7, 4713)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt16(4713),
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

  it('test operator "ge" overload (euint4, euint16) => ebool test 1 (9, 12663)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(12663),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt16(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 1 (9, 35570)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(35570),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt16(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 1 (10, 44221)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(44221),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt16(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(6),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 1 (4, 61168)', async function () {
    const res = await this.contract1.lt_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(61168),
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

  it('test operator "min" overload (euint4, euint16) => euint16 test 1 (1, 60443)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(60443),
    );
    expect(res).to.equal(1);
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

  it('test operator "max" overload (euint4, euint16) => euint16 test 1 (11, 33061)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt16(33061),
    );
    expect(res).to.equal(33061);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 2 (7, 11)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt16(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 3 (11, 11)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt16(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 4 (11, 7)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt16(7),
    );
    expect(res).to.equal(11);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 1 (1, 7)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(7),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 2 (5, 9)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt32(9),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 3 (4, 4)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 4 (9, 5)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(5),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "sub" overload (euint4, euint32) => euint32 test 1 (10, 10)', async function () {
    const res = await this.contract1.sub_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(10),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint32) => euint32 test 2 (10, 6)', async function () {
    const res = await this.contract1.sub_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(6),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 1 (1, 15)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(15),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 2 (2, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt32(3),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt32(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 4 (3, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt32(2),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 1 (1, 245829777)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(245829777),
    );
    expect(res).to.equal(1);
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

  it('test operator "or" overload (euint4, euint32) => euint32 test 1 (14, 14664100)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt32(14664100),
    );
    expect(res).to.equal(14664110);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 2 (10, 14)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 3 (14, 14)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt32(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 4 (14, 10)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt32(10),
    );
    expect(res).to.equal(14);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 1 (13, 56718435)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(56718435),
    );
    expect(res).to.equal(56718446);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 2 (9, 13)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(13),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 3 (13, 13)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(13),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 4 (13, 9)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(9),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 1 (5, 67800295)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt32(67800295),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 1 (7, 24833418)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt32(24833418),
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

  it('test operator "ge" overload (euint4, euint32) => ebool test 1 (9, 36890948)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(36890948),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt32(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 1 (9, 89558247)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(89558247),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt32(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 1 (10, 3145467)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(3145467),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt32(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(6),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 1 (4, 187774996)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(187774996),
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

  it('test operator "min" overload (euint4, euint32) => euint32 test 1 (1, 66587261)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(66587261),
    );
    expect(res).to.equal(1);
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

  it('test operator "max" overload (euint4, euint32) => euint32 test 1 (11, 9483424)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt32(9483424),
    );
    expect(res).to.equal(9483424);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 2 (7, 11)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt32(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 3 (11, 11)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt32(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 4 (11, 7)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt32(7),
    );
    expect(res).to.equal(11);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 1 (1, 9)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(9),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 2 (5, 9)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt64(9),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 3 (4, 4)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 4 (9, 5)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt64(5),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "sub" overload (euint4, euint64) => euint64 test 1 (10, 10)', async function () {
    const res = await this.contract1.sub_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, euint64) => euint64 test 2 (10, 6)', async function () {
    const res = await this.contract1.sub_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(6),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 1 (1, 15)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(15),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 2 (2, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt64(3),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt64(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 4 (3, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt64(2),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 1 (1, 113854917)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(113854917),
    );
    expect(res).to.equal(1);
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

  it('test operator "or" overload (euint4, euint64) => euint64 test 1 (14, 170899961)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt64(170899961),
    );
    expect(res).to.equal(170899967);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 2 (10, 14)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 3 (14, 14)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt64(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 4 (14, 10)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(14);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 1 (13, 34180683)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt64(34180683),
    );
    expect(res).to.equal(34180678);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 2 (9, 13)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt64(13),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 3 (13, 13)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt64(13),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 4 (13, 9)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt64(9),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 1 (5, 143961113)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt64(143961113),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 1 (7, 198024790)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt64(198024790),
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

  it('test operator "ge" overload (euint4, euint64) => ebool test 1 (9, 16491235)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt64(16491235),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt64(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt64(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt64(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 1 (9, 53485181)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt64(53485181),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt64(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt64(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt64(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 1 (10, 167407557)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(167407557),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(6),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 1 (4, 169419678)', async function () {
    const res = await this.contract1.lt_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(169419678),
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

  it('test operator "min" overload (euint4, euint64) => euint64 test 1 (1, 243162021)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(243162021),
    );
    expect(res).to.equal(1);
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

  it('test operator "max" overload (euint4, euint64) => euint64 test 1 (11, 268368365)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt64(268368365),
    );
    expect(res).to.equal(268368365);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 2 (7, 11)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt64(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 3 (11, 11)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt64(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 4 (11, 7)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt64(7),
    );
    expect(res).to.equal(11);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 1 (4, 4)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(4), 4);
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 2 (5, 9)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(5), 9);
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 3 (4, 4)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(4), 4);
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 4 (9, 5)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(9), 5);
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 1 (5, 7)', async function () {
    const res = await this.contract1.add_uint8_euint4(5, this.instances1.alice.encrypt4(7));
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_uint8_euint4(4, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 3 (4, 4)', async function () {
    const res = await this.contract1.add_uint8_euint4(4, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, uint8) => euint4 test 1 (10, 10)', async function () {
    const res = await this.contract1.sub_euint4_uint8(this.instances1.alice.encrypt4(10), 10);
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint4, uint8) => euint4 test 2 (10, 6)', async function () {
    const res = await this.contract1.sub_euint4_uint8(this.instances1.alice.encrypt4(10), 6);
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

  it('test operator "mul" overload (euint4, uint8) => euint4 test 1 (7, 2)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(7), 2);
    expect(res).to.equal(14n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 2 (2, 3)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(2), 3);
    expect(res).to.equal(6n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(3), 3);
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 4 (3, 2)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(3), 2);
    expect(res).to.equal(6n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 1 (1, 3)', async function () {
    const res = await this.contract1.mul_uint8_euint4(1, this.instances1.alice.encrypt4(3));
    expect(res).to.equal(3n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 2 (2, 3)', async function () {
    const res = await this.contract1.mul_uint8_euint4(2, this.instances1.alice.encrypt4(3));
    expect(res).to.equal(6n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_uint8_euint4(3, this.instances1.alice.encrypt4(3));
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 4 (3, 2)', async function () {
    const res = await this.contract1.mul_uint8_euint4(3, this.instances1.alice.encrypt4(2));
    expect(res).to.equal(6n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 1 (11, 8)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(11), 8);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 2 (7, 11)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(7), 11);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 3 (11, 11)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(11), 11);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 4 (11, 7)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(11), 7);
    expect(res).to.equal(1);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 1 (1, 12)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(1), 12);
    expect(res).to.equal(1);
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

  it('test operator "eq" overload (euint4, uint8) => ebool test 1 (5, 7)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(5), 7);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 1 (6, 11)', async function () {
    const res = await this.contract1.eq_uint8_euint4(6, this.instances1.alice.encrypt4(11));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.eq_uint8_euint4(7, this.instances1.alice.encrypt4(11));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.eq_uint8_euint4(11, this.instances1.alice.encrypt4(11));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.eq_uint8_euint4(11, this.instances1.alice.encrypt4(7));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 1 (7, 5)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(7), 5);
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

  it('test operator "ne" overload (uint8, euint4) => ebool test 1 (4, 13)', async function () {
    const res = await this.contract1.ne_uint8_euint4(4, this.instances1.alice.encrypt4(13));
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

  it('test operator "ge" overload (euint4, uint8) => ebool test 1 (9, 2)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(9), 2);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(5), 9);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(9), 9);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(9), 5);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 1 (8, 1)', async function () {
    const res = await this.contract1.ge_uint8_euint4(8, this.instances1.alice.encrypt4(1));
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

  it('test operator "gt" overload (euint4, uint8) => ebool test 1 (9, 9)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(9), 9);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(5), 9);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(9), 9);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(9), 5);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.gt_uint8_euint4(1, this.instances1.alice.encrypt4(1));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.gt_uint8_euint4(4, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.gt_uint8_euint4(8, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.gt_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 1 (10, 7)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(10), 7);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(6), 10);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(10), 10);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(10), 6);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 1 (10, 11)', async function () {
    const res = await this.contract1.le_uint8_euint4(10, this.instances1.alice.encrypt4(11));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.le_uint8_euint4(7, this.instances1.alice.encrypt4(11));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.le_uint8_euint4(11, this.instances1.alice.encrypt4(11));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.le_uint8_euint4(11, this.instances1.alice.encrypt4(7));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 1 (4, 14)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(4), 14);
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

  it('test operator "lt" overload (uint8, euint4) => ebool test 1 (6, 14)', async function () {
    const res = await this.contract1.lt_uint8_euint4(6, this.instances1.alice.encrypt4(14));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.lt_uint8_euint4(10, this.instances1.alice.encrypt4(14));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.lt_uint8_euint4(14, this.instances1.alice.encrypt4(14));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.lt_uint8_euint4(14, this.instances1.alice.encrypt4(10));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 1 (1, 3)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(1), 3);
    expect(res).to.equal(1);
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

  it('test operator "min" overload (uint8, euint4) => euint4 test 1 (4, 9)', async function () {
    const res = await this.contract1.min_uint8_euint4(4, this.instances1.alice.encrypt4(9));
    expect(res).to.equal(4);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 2 (5, 9)', async function () {
    const res = await this.contract1.min_uint8_euint4(5, this.instances1.alice.encrypt4(9));
    expect(res).to.equal(5);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 3 (9, 9)', async function () {
    const res = await this.contract1.min_uint8_euint4(9, this.instances1.alice.encrypt4(9));
    expect(res).to.equal(9);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 4 (9, 5)', async function () {
    const res = await this.contract1.min_uint8_euint4(9, this.instances1.alice.encrypt4(5));
    expect(res).to.equal(5);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 1 (11, 8)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(11), 8);
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 2 (7, 11)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(7), 11);
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 3 (11, 11)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(11), 11);
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 4 (11, 7)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(11), 7);
    expect(res).to.equal(11);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 1 (9, 5)', async function () {
    const res = await this.contract1.max_uint8_euint4(9, this.instances1.alice.encrypt4(5));
    expect(res).to.equal(9);
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

  it('test operator "add" overload (euint8, euint4) => euint8 test 1 (9, 1)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(9),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 3 (4, 4)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(4),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
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

  it('test operator "mul" overload (euint8, euint4) => euint8 test 1 (8, 1)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 2 (2, 3)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(2),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 4 (3, 2)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 1 (245, 4)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(245),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(4);
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

  it('test operator "or" overload (euint8, euint4) => euint8 test 1 (96, 6)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(96),
      this.instances1.alice.encrypt4(6),
    );
    expect(res).to.equal(102);
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

  it('test operator "xor" overload (euint8, euint4) => euint8 test 1 (5, 3)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(5),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(6);
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

  it('test operator "eq" overload (euint8, euint4) => ebool test 1 (152, 11)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(152),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(7),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt4(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 1 (106, 13)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(106),
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

  it('test operator "ge" overload (euint8, euint4) => ebool test 1 (130, 1)', async function () {
    const res = await this.contract2.ge_euint8_euint4(
      this.instances2.alice.encrypt8(130),
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

  it('test operator "gt" overload (euint8, euint4) => ebool test 1 (130, 1)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(130),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 1 (207, 11)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(207),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(7),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt4(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 1 (94, 14)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(94),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(10),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 1 (110, 9)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(110),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(9);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 2 (5, 9)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(5);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 3 (9, 9)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(9);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 4 (9, 5)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt4(5),
    );
    expect(res).to.equal(5);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 1 (77, 5)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(77),
      this.instances2.alice.encrypt4(5),
    );
    expect(res).to.equal(77);
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

  it('test operator "add" overload (euint8, euint8) => euint8 test 1 (5, 172)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt8(172),
    );
    expect(res).to.equal(177n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(12n);
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

  it('test operator "mul" overload (euint8, euint8) => euint8 test 1 (3, 69)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(3),
      this.instances2.alice.encrypt8(69),
    );
    expect(res).to.equal(207n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 1 (245, 164)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(245),
      this.instances2.alice.encrypt8(164),
    );
    expect(res).to.equal(164);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 2 (160, 164)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(160),
      this.instances2.alice.encrypt8(164),
    );
    expect(res).to.equal(160);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 3 (164, 164)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(164),
      this.instances2.alice.encrypt8(164),
    );
    expect(res).to.equal(164);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 4 (164, 160)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(164),
      this.instances2.alice.encrypt8(160),
    );
    expect(res).to.equal(160);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 1 (96, 249)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt8(249),
    );
    expect(res).to.equal(249);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 2 (92, 96)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt8(96),
    );
    expect(res).to.equal(124);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 3 (96, 96)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt8(96),
    );
    expect(res).to.equal(96);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 4 (96, 92)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt8(92),
    );
    expect(res).to.equal(124);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 1 (5, 61)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt8(61),
    );
    expect(res).to.equal(56);
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

  it('test operator "eq" overload (euint8, euint8) => ebool test 1 (6, 177)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(6),
      this.instances2.alice.encrypt8(177),
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

  it('test operator "ne" overload (euint8, euint8) => ebool test 1 (4, 157)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(157),
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

  it('test operator "ge" overload (euint8, euint8) => ebool test 1 (8, 191)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(191),
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

  it('test operator "gt" overload (euint8, euint8) => ebool test 1 (1, 180)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt8(180),
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

  it('test operator "le" overload (euint8, euint8) => ebool test 1 (10, 238)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(10),
      this.instances2.alice.encrypt8(238),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(6),
      this.instances2.alice.encrypt8(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(10),
      this.instances2.alice.encrypt8(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(10),
      this.instances2.alice.encrypt8(6),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 1 (6, 240)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(6),
      this.instances2.alice.encrypt8(240),
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

  it('test operator "min" overload (euint8, euint8) => euint8 test 1 (4, 142)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(142),
    );
    expect(res).to.equal(4);
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

  it('test operator "max" overload (euint8, euint8) => euint8 test 1 (9, 9)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt8(9),
    );
    expect(res).to.equal(9);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 2 (5, 9)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt8(9),
    );
    expect(res).to.equal(9);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 3 (9, 9)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt8(9),
    );
    expect(res).to.equal(9);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 4 (9, 5)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt8(5),
    );
    expect(res).to.equal(9);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 1 (1, 227)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(227),
    );
    expect(res).to.equal(228n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 2 (22, 26)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(22),
      this.instances2.alice.encrypt16(26),
    );
    expect(res).to.equal(48n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 3 (26, 26)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(26),
      this.instances2.alice.encrypt16(26),
    );
    expect(res).to.equal(52n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 4 (26, 22)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(26),
      this.instances2.alice.encrypt16(22),
    );
    expect(res).to.equal(48n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 1 (202, 202)', async function () {
    const res = await this.contract2.sub_euint8_euint16(
      this.instances2.alice.encrypt8(202),
      this.instances2.alice.encrypt16(202),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 2 (202, 198)', async function () {
    const res = await this.contract2.sub_euint8_euint16(
      this.instances2.alice.encrypt8(202),
      this.instances2.alice.encrypt16(198),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 1 (1, 235)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(235),
    );
    expect(res).to.equal(235n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 2 (14, 14)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt16(14),
    );
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 3 (14, 14)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt16(14),
    );
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 4 (14, 14)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt16(14),
    );
    expect(res).to.equal(196n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 1 (245, 17782)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(245),
      this.instances2.alice.encrypt16(17782),
    );
    expect(res).to.equal(116);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 2 (241, 245)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(241),
      this.instances2.alice.encrypt16(245),
    );
    expect(res).to.equal(241);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 3 (245, 245)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(245),
      this.instances2.alice.encrypt16(245),
    );
    expect(res).to.equal(245);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 4 (245, 241)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(245),
      this.instances2.alice.encrypt16(241),
    );
    expect(res).to.equal(241);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 1 (96, 1790)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt16(1790),
    );
    expect(res).to.equal(1790);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 2 (92, 96)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt16(96),
    );
    expect(res).to.equal(124);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 3 (96, 96)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt16(96),
    );
    expect(res).to.equal(96);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 4 (96, 92)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt16(92),
    );
    expect(res).to.equal(124);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 1 (5, 1712)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt16(1712),
    );
    expect(res).to.equal(1717);
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

  it('test operator "eq" overload (euint8, euint16) => ebool test 1 (18, 27181)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(18),
      this.instances2.alice.encrypt16(27181),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 2 (14, 18)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt16(18),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 3 (18, 18)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(18),
      this.instances2.alice.encrypt16(18),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 4 (18, 14)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(18),
      this.instances2.alice.encrypt16(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 1 (78, 21962)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(78),
      this.instances2.alice.encrypt16(21962),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 2 (74, 78)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(74),
      this.instances2.alice.encrypt16(78),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 3 (78, 78)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(78),
      this.instances2.alice.encrypt16(78),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 4 (78, 74)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(78),
      this.instances2.alice.encrypt16(74),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 1 (1, 54905)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(54905),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 1 (102, 32183)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(102),
      this.instances2.alice.encrypt16(32183),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 2 (98, 102)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(98),
      this.instances2.alice.encrypt16(102),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 3 (102, 102)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(102),
      this.instances2.alice.encrypt16(102),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 4 (102, 98)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(102),
      this.instances2.alice.encrypt16(98),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 1 (96, 42413)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt16(42413),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 2 (92, 96)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt16(96),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 3 (96, 96)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt16(96),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 4 (96, 92)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt16(92),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 1 (198, 55658)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(198),
      this.instances2.alice.encrypt16(55658),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 2 (194, 198)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(194),
      this.instances2.alice.encrypt16(198),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 3 (198, 198)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(198),
      this.instances2.alice.encrypt16(198),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 4 (198, 194)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(198),
      this.instances2.alice.encrypt16(194),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 1 (200, 2568)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(200),
      this.instances2.alice.encrypt16(2568),
    );
    expect(res).to.equal(200);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 2 (196, 200)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(196),
      this.instances2.alice.encrypt16(200),
    );
    expect(res).to.equal(196);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 3 (200, 200)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(200),
      this.instances2.alice.encrypt16(200),
    );
    expect(res).to.equal(200);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 4 (200, 196)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(200),
      this.instances2.alice.encrypt16(196),
    );
    expect(res).to.equal(196);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 1 (161, 11628)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt16(11628),
    );
    expect(res).to.equal(11628);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 2 (157, 161)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(157),
      this.instances2.alice.encrypt16(161),
    );
    expect(res).to.equal(161);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 3 (161, 161)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt16(161),
    );
    expect(res).to.equal(161);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 4 (161, 157)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt16(157),
    );
    expect(res).to.equal(161);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 1 (1, 228)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(228),
    );
    expect(res).to.equal(229n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 2 (22, 26)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(22),
      this.instances2.alice.encrypt32(26),
    );
    expect(res).to.equal(48n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 3 (26, 26)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(26),
      this.instances2.alice.encrypt32(26),
    );
    expect(res).to.equal(52n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 4 (26, 22)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(26),
      this.instances2.alice.encrypt32(22),
    );
    expect(res).to.equal(48n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 1 (202, 202)', async function () {
    const res = await this.contract2.sub_euint8_euint32(
      this.instances2.alice.encrypt8(202),
      this.instances2.alice.encrypt32(202),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 2 (202, 198)', async function () {
    const res = await this.contract2.sub_euint8_euint32(
      this.instances2.alice.encrypt8(202),
      this.instances2.alice.encrypt32(198),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 1 (1, 215)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(215),
    );
    expect(res).to.equal(215n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 2 (14, 14)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt32(14),
    );
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 3 (14, 14)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt32(14),
    );
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 4 (14, 14)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt32(14),
    );
    expect(res).to.equal(196n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 1 (245, 107977808)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(245),
      this.instances2.alice.encrypt32(107977808),
    );
    expect(res).to.equal(80);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 2 (241, 245)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(241),
      this.instances2.alice.encrypt32(245),
    );
    expect(res).to.equal(241);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 3 (245, 245)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(245),
      this.instances2.alice.encrypt32(245),
    );
    expect(res).to.equal(245);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 4 (245, 241)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(245),
      this.instances2.alice.encrypt32(241),
    );
    expect(res).to.equal(241);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 1 (96, 81378325)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt32(81378325),
    );
    expect(res).to.equal(81378421);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 2 (92, 96)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt32(96),
    );
    expect(res).to.equal(124);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 3 (96, 96)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt32(96),
    );
    expect(res).to.equal(96);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 4 (96, 92)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt32(92),
    );
    expect(res).to.equal(124);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 1 (5, 115830538)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt32(115830538),
    );
    expect(res).to.equal(115830543);
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

  it('test operator "eq" overload (euint8, euint32) => ebool test 1 (18, 186668088)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(18),
      this.instances2.alice.encrypt32(186668088),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 2 (14, 18)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt32(18),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 3 (18, 18)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(18),
      this.instances2.alice.encrypt32(18),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 4 (18, 14)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(18),
      this.instances2.alice.encrypt32(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 1 (78, 92852552)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(78),
      this.instances2.alice.encrypt32(92852552),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 2 (74, 78)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(74),
      this.instances2.alice.encrypt32(78),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 3 (78, 78)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(78),
      this.instances2.alice.encrypt32(78),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 4 (78, 74)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(78),
      this.instances2.alice.encrypt32(74),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 1 (1, 66874588)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(66874588),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 1 (102, 173287703)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(102),
      this.instances2.alice.encrypt32(173287703),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 2 (98, 102)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(98),
      this.instances2.alice.encrypt32(102),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 3 (102, 102)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(102),
      this.instances2.alice.encrypt32(102),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 4 (102, 98)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(102),
      this.instances2.alice.encrypt32(98),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 1 (96, 74740688)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt32(74740688),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 2 (92, 96)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt32(96),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 3 (96, 96)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt32(96),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 4 (96, 92)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt32(92),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 1 (198, 20210081)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(198),
      this.instances2.alice.encrypt32(20210081),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 2 (194, 198)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(194),
      this.instances2.alice.encrypt32(198),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 3 (198, 198)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(198),
      this.instances2.alice.encrypt32(198),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 4 (198, 194)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(198),
      this.instances2.alice.encrypt32(194),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 1 (200, 114061461)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(200),
      this.instances2.alice.encrypt32(114061461),
    );
    expect(res).to.equal(200);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 2 (196, 200)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(196),
      this.instances2.alice.encrypt32(200),
    );
    expect(res).to.equal(196);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 3 (200, 200)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(200),
      this.instances2.alice.encrypt32(200),
    );
    expect(res).to.equal(200);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 4 (200, 196)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(200),
      this.instances2.alice.encrypt32(196),
    );
    expect(res).to.equal(196);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 1 (161, 55227414)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt32(55227414),
    );
    expect(res).to.equal(55227414);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 2 (157, 161)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(157),
      this.instances2.alice.encrypt32(161),
    );
    expect(res).to.equal(161);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 3 (161, 161)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt32(161),
    );
    expect(res).to.equal(161);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 4 (161, 157)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt32(157),
    );
    expect(res).to.equal(161);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 1 (1, 248)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(248),
    );
    expect(res).to.equal(249n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 2 (22, 26)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(22),
      this.instances2.alice.encrypt64(26),
    );
    expect(res).to.equal(48n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 3 (26, 26)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(26),
      this.instances2.alice.encrypt64(26),
    );
    expect(res).to.equal(52n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 4 (26, 22)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(26),
      this.instances2.alice.encrypt64(22),
    );
    expect(res).to.equal(48n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 1 (202, 202)', async function () {
    const res = await this.contract2.sub_euint8_euint64(
      this.instances2.alice.encrypt8(202),
      this.instances2.alice.encrypt64(202),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 2 (202, 198)', async function () {
    const res = await this.contract2.sub_euint8_euint64(
      this.instances2.alice.encrypt8(202),
      this.instances2.alice.encrypt64(198),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 1 (1, 223)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(223),
    );
    expect(res).to.equal(223n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 2 (14, 14)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt64(14),
    );
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 3 (14, 14)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt64(14),
    );
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 4 (14, 14)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt64(14),
    );
    expect(res).to.equal(196n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 1 (245, 233482558)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(245),
      this.instances2.alice.encrypt64(233482558),
    );
    expect(res).to.equal(52);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 2 (241, 245)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(241),
      this.instances2.alice.encrypt64(245),
    );
    expect(res).to.equal(241);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 3 (245, 245)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(245),
      this.instances2.alice.encrypt64(245),
    );
    expect(res).to.equal(245);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 4 (245, 241)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(245),
      this.instances2.alice.encrypt64(241),
    );
    expect(res).to.equal(241);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 1 (96, 83329888)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt64(83329888),
    );
    expect(res).to.equal(83329888);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 2 (92, 96)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt64(96),
    );
    expect(res).to.equal(124);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 3 (96, 96)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt64(96),
    );
    expect(res).to.equal(96);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 4 (96, 92)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt64(92),
    );
    expect(res).to.equal(124);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 1 (5, 103206592)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt64(103206592),
    );
    expect(res).to.equal(103206597);
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

  it('test operator "eq" overload (euint8, euint64) => ebool test 1 (18, 130872852)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(18),
      this.instances2.alice.encrypt64(130872852),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 2 (14, 18)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt64(18),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 3 (18, 18)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(18),
      this.instances2.alice.encrypt64(18),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 4 (18, 14)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(18),
      this.instances2.alice.encrypt64(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 1 (78, 43366180)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(78),
      this.instances2.alice.encrypt64(43366180),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 2 (74, 78)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(74),
      this.instances2.alice.encrypt64(78),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 3 (78, 78)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(78),
      this.instances2.alice.encrypt64(78),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 4 (78, 74)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(78),
      this.instances2.alice.encrypt64(74),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 1 (1, 155404527)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(155404527),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 1 (102, 257990052)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(102),
      this.instances2.alice.encrypt64(257990052),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 2 (98, 102)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(98),
      this.instances2.alice.encrypt64(102),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 3 (102, 102)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(102),
      this.instances2.alice.encrypt64(102),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 4 (102, 98)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(102),
      this.instances2.alice.encrypt64(98),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 1 (96, 34093048)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt64(34093048),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 2 (92, 96)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt64(96),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 3 (96, 96)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt64(96),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 4 (96, 92)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt64(92),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 1 (198, 171810732)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(198),
      this.instances2.alice.encrypt64(171810732),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 2 (194, 198)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(194),
      this.instances2.alice.encrypt64(198),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 3 (198, 198)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(198),
      this.instances2.alice.encrypt64(198),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 4 (198, 194)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(198),
      this.instances2.alice.encrypt64(194),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 1 (200, 124180225)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(200),
      this.instances2.alice.encrypt64(124180225),
    );
    expect(res).to.equal(200);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 2 (196, 200)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(196),
      this.instances2.alice.encrypt64(200),
    );
    expect(res).to.equal(196);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 3 (200, 200)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(200),
      this.instances2.alice.encrypt64(200),
    );
    expect(res).to.equal(200);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 4 (200, 196)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(200),
      this.instances2.alice.encrypt64(196),
    );
    expect(res).to.equal(196);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 1 (161, 242190943)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt64(242190943),
    );
    expect(res).to.equal(242190943);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 2 (157, 161)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(157),
      this.instances2.alice.encrypt64(161),
    );
    expect(res).to.equal(161);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 3 (161, 161)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt64(161),
    );
    expect(res).to.equal(161);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 4 (161, 157)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt64(157),
    );
    expect(res).to.equal(161);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 1 (5, 209)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(5), 209);
    expect(res).to.equal(214n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 1 (26, 209)', async function () {
    const res = await this.contract2.add_uint8_euint8(26, this.instances2.alice.encrypt8(209));
    expect(res).to.equal(235n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.add_uint8_euint8(4, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.add_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.add_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(12n);
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

  it('test operator "mul" overload (euint8, uint8) => euint8 test 1 (3, 50)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(3), 50);
    expect(res).to.equal(150n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 1 (14, 6)', async function () {
    const res = await this.contract2.mul_uint8_euint8(14, this.instances2.alice.encrypt8(6));
    expect(res).to.equal(84n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.mul_uint8_euint8(4, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.mul_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.mul_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(32n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 1 (192, 181)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(192), 181);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 2 (39, 43)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(39), 43);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 3 (43, 43)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(43), 43);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 4 (43, 39)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(43), 39);
    expect(res).to.equal(1);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 1 (221, 143)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(221), 143);
    expect(res).to.equal(78);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(4);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(0);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 1 (6, 242)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(6), 242);
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

  it('test operator "eq" overload (uint8, euint8) => ebool test 1 (18, 242)', async function () {
    const res = await this.contract2.eq_uint8_euint8(18, this.instances2.alice.encrypt8(242));
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

  it('test operator "ne" overload (euint8, uint8) => ebool test 1 (4, 177)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(4), 177);
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

  it('test operator "ne" overload (uint8, euint8) => ebool test 1 (78, 177)', async function () {
    const res = await this.contract2.ne_uint8_euint8(78, this.instances2.alice.encrypt8(177));
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

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (8, 42)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(8), 42);
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

  it('test operator "ge" overload (uint8, euint8) => ebool test 1 (1, 42)', async function () {
    const res = await this.contract2.ge_uint8_euint8(1, this.instances2.alice.encrypt8(42));
    expect(res).to.equal(false);
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

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (1, 53)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(1), 53);
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

  it('test operator "gt" overload (uint8, euint8) => ebool test 1 (102, 53)', async function () {
    const res = await this.contract2.gt_uint8_euint8(102, this.instances2.alice.encrypt8(53));
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

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (10, 232)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(10), 232);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(6), 10);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(10), 10);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(10), 6);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 1 (96, 232)', async function () {
    const res = await this.contract2.le_uint8_euint8(96, this.instances2.alice.encrypt8(232));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract2.le_uint8_euint8(6, this.instances2.alice.encrypt8(10));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract2.le_uint8_euint8(10, this.instances2.alice.encrypt8(10));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract2.le_uint8_euint8(10, this.instances2.alice.encrypt8(6));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 1 (6, 39)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(6), 39);
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

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (198, 39)', async function () {
    const res = await this.contract2.lt_uint8_euint8(198, this.instances2.alice.encrypt8(39));
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

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (4, 122)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(4), 122);
    expect(res).to.equal(4);
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

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (200, 122)', async function () {
    const res = await this.contract2.min_uint8_euint8(200, this.instances2.alice.encrypt8(122));
    expect(res).to.equal(122);
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

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (9, 61)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(9), 61);
    expect(res).to.equal(61);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (5, 9)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(5), 9);
    expect(res).to.equal(9);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (9, 9)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(9), 9);
    expect(res).to.equal(9);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 4 (9, 5)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(9), 5);
    expect(res).to.equal(9);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 1 (161, 61)', async function () {
    const res = await this.contract2.max_uint8_euint8(161, this.instances2.alice.encrypt8(61));
    expect(res).to.equal(161);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 2 (5, 9)', async function () {
    const res = await this.contract2.max_uint8_euint8(5, this.instances2.alice.encrypt8(9));
    expect(res).to.equal(9);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 3 (9, 9)', async function () {
    const res = await this.contract2.max_uint8_euint8(9, this.instances2.alice.encrypt8(9));
    expect(res).to.equal(9);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 4 (9, 5)', async function () {
    const res = await this.contract2.max_uint8_euint8(9, this.instances2.alice.encrypt8(5));
    expect(res).to.equal(9);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 1 (14, 1)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(14),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 2 (4, 6)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(6),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 3 (6, 6)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(6),
      this.instances2.alice.encrypt4(6),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 4 (6, 4)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(6),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(10n);
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

  it('test operator "mul" overload (euint16, euint4) => euint16 test 1 (12, 1)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(12),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 2 (2, 4)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(2),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 3 (2, 2)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(2),
      this.instances2.alice.encrypt4(2),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 4 (4, 2)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(2),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 1 (55927, 3)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(55927),
      this.instances2.alice.encrypt4(3),
    );
    expect(res).to.equal(3);
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

  it('test operator "or" overload (euint16, euint4) => euint16 test 1 (25484, 8)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(25484),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(25484);
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

  it('test operator "xor" overload (euint16, euint4) => euint16 test 1 (20626, 7)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(20626),
      this.instances2.alice.encrypt4(7),
    );
    expect(res).to.equal(20629);
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

  it('test operator "eq" overload (euint16, euint4) => ebool test 1 (5463, 2)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(5463),
      this.instances2.alice.encrypt4(2),
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

  it('test operator "ne" overload (euint16, euint4) => ebool test 1 (24558, 12)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(24558),
      this.instances2.alice.encrypt4(12),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(12),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(12),
      this.instances2.alice.encrypt4(12),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(12),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 1 (51421, 10)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(51421),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(6),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(10),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(10),
      this.instances2.alice.encrypt4(6),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 1 (44465, 6)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(44465),
      this.instances2.alice.encrypt4(6),
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

  it('test operator "le" overload (euint16, euint4) => ebool test 1 (8195, 5)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(8195),
      this.instances2.alice.encrypt4(5),
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

  it('test operator "lt" overload (euint16, euint4) => ebool test 1 (59556, 1)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(59556),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 1 (40365, 13)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(40365),
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

  it('test operator "max" overload (euint16, euint4) => euint16 test 1 (45208, 13)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(45208),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(45208);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 2 (9, 13)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(9),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 3 (13, 13)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(13),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(13);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 4 (13, 9)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(13),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(13);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 1 (116, 14)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(116),
      this.instances3.alice.encrypt8(14),
    );
    expect(res).to.equal(130n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 2 (115, 117)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(115),
      this.instances3.alice.encrypt8(117),
    );
    expect(res).to.equal(232n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 3 (117, 117)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(117),
      this.instances3.alice.encrypt8(117),
    );
    expect(res).to.equal(234n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 4 (117, 115)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(117),
      this.instances3.alice.encrypt8(115),
    );
    expect(res).to.equal(232n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (125, 125)', async function () {
    const res = await this.contract3.sub_euint16_euint8(
      this.instances3.alice.encrypt16(125),
      this.instances3.alice.encrypt8(125),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (125, 121)', async function () {
    const res = await this.contract3.sub_euint16_euint8(
      this.instances3.alice.encrypt16(125),
      this.instances3.alice.encrypt8(121),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (207, 1)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(207),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(207n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 2 (12, 14)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(12),
      this.instances3.alice.encrypt8(14),
    );
    expect(res).to.equal(168n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 3 (14, 14)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(14),
      this.instances3.alice.encrypt8(14),
    );
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 4 (14, 12)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(14),
      this.instances3.alice.encrypt8(12),
    );
    expect(res).to.equal(168n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 1 (55927, 14)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(55927),
      this.instances3.alice.encrypt8(14),
    );
    expect(res).to.equal(6);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 2 (10, 14)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(10),
      this.instances3.alice.encrypt8(14),
    );
    expect(res).to.equal(10);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 3 (14, 14)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(14),
      this.instances3.alice.encrypt8(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 4 (14, 10)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(14),
      this.instances3.alice.encrypt8(10),
    );
    expect(res).to.equal(10);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 1 (25484, 207)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(25484),
      this.instances3.alice.encrypt8(207),
    );
    expect(res).to.equal(25551);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 2 (203, 207)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(203),
      this.instances3.alice.encrypt8(207),
    );
    expect(res).to.equal(207);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 3 (207, 207)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(207),
      this.instances3.alice.encrypt8(207),
    );
    expect(res).to.equal(207);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 4 (207, 203)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(207),
      this.instances3.alice.encrypt8(203),
    );
    expect(res).to.equal(207);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (20626, 38)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(20626),
      this.instances3.alice.encrypt8(38),
    );
    expect(res).to.equal(20660);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (34, 38)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(34),
      this.instances3.alice.encrypt8(38),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 3 (38, 38)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(38),
      this.instances3.alice.encrypt8(38),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 4 (38, 34)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(38),
      this.instances3.alice.encrypt8(34),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 1 (5463, 69)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(5463),
      this.instances3.alice.encrypt8(69),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 2 (65, 69)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(65),
      this.instances3.alice.encrypt8(69),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 3 (69, 69)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(69),
      this.instances3.alice.encrypt8(69),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 4 (69, 65)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(69),
      this.instances3.alice.encrypt8(65),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (24558, 239)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(24558),
      this.instances3.alice.encrypt8(239),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (235, 239)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(235),
      this.instances3.alice.encrypt8(239),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 3 (239, 239)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(239),
      this.instances3.alice.encrypt8(239),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 4 (239, 235)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(239),
      this.instances3.alice.encrypt8(235),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (51421, 242)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(51421),
      this.instances3.alice.encrypt8(242),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (238, 242)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(238),
      this.instances3.alice.encrypt8(242),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (242, 242)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(242),
      this.instances3.alice.encrypt8(242),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 4 (242, 238)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(242),
      this.instances3.alice.encrypt8(238),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 1 (44465, 202)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(44465),
      this.instances3.alice.encrypt8(202),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 2 (198, 202)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(198),
      this.instances3.alice.encrypt8(202),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 3 (202, 202)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(202),
      this.instances3.alice.encrypt8(202),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 4 (202, 198)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(202),
      this.instances3.alice.encrypt8(198),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 1 (8195, 93)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(8195),
      this.instances3.alice.encrypt8(93),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 2 (89, 93)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(89),
      this.instances3.alice.encrypt8(93),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 3 (93, 93)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(93),
      this.instances3.alice.encrypt8(93),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 4 (93, 89)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(93),
      this.instances3.alice.encrypt8(89),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 1 (59556, 229)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(59556),
      this.instances3.alice.encrypt8(229),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 2 (225, 229)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(225),
      this.instances3.alice.encrypt8(229),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 3 (229, 229)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(229),
      this.instances3.alice.encrypt8(229),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 4 (229, 225)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(229),
      this.instances3.alice.encrypt8(225),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 1 (40365, 88)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(40365),
      this.instances3.alice.encrypt8(88),
    );
    expect(res).to.equal(88);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 2 (84, 88)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(84),
      this.instances3.alice.encrypt8(88),
    );
    expect(res).to.equal(84);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 3 (88, 88)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(88),
      this.instances3.alice.encrypt8(88),
    );
    expect(res).to.equal(88);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 4 (88, 84)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(88),
      this.instances3.alice.encrypt8(84),
    );
    expect(res).to.equal(84);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 1 (45208, 175)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(45208),
      this.instances3.alice.encrypt8(175),
    );
    expect(res).to.equal(45208);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 2 (171, 175)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(171),
      this.instances3.alice.encrypt8(175),
    );
    expect(res).to.equal(175);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 3 (175, 175)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(175),
      this.instances3.alice.encrypt8(175),
    );
    expect(res).to.equal(175);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 4 (175, 171)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(175),
      this.instances3.alice.encrypt8(171),
    );
    expect(res).to.equal(175);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 1 (1870, 63235)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(1870),
      this.instances3.alice.encrypt16(63235),
    );
    expect(res).to.equal(65105n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 2 (1866, 1870)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(1866),
      this.instances3.alice.encrypt16(1870),
    );
    expect(res).to.equal(3736n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 3 (1870, 1870)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(1870),
      this.instances3.alice.encrypt16(1870),
    );
    expect(res).to.equal(3740n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 4 (1870, 1866)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(1870),
      this.instances3.alice.encrypt16(1866),
    );
    expect(res).to.equal(3736n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 1 (4130, 4130)', async function () {
    const res = await this.contract3.sub_euint16_euint16(
      this.instances3.alice.encrypt16(4130),
      this.instances3.alice.encrypt16(4130),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 2 (4130, 4126)', async function () {
    const res = await this.contract3.sub_euint16_euint16(
      this.instances3.alice.encrypt16(4130),
      this.instances3.alice.encrypt16(4126),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 1 (103, 420)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(103),
      this.instances3.alice.encrypt16(420),
    );
    expect(res).to.equal(43260n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 2 (206, 207)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(206),
      this.instances3.alice.encrypt16(207),
    );
    expect(res).to.equal(42642n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 3 (207, 207)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(207),
      this.instances3.alice.encrypt16(207),
    );
    expect(res).to.equal(42849n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 4 (207, 206)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(207),
      this.instances3.alice.encrypt16(206),
    );
    expect(res).to.equal(42642n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 1 (55927, 33200)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(55927),
      this.instances3.alice.encrypt16(33200),
    );
    expect(res).to.equal(32816);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 2 (33196, 33200)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(33196),
      this.instances3.alice.encrypt16(33200),
    );
    expect(res).to.equal(33184);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 3 (33200, 33200)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(33200),
      this.instances3.alice.encrypt16(33200),
    );
    expect(res).to.equal(33200);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 4 (33200, 33196)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(33200),
      this.instances3.alice.encrypt16(33196),
    );
    expect(res).to.equal(33184);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 1 (25484, 6586)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(25484),
      this.instances3.alice.encrypt16(6586),
    );
    expect(res).to.equal(31678);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 2 (6582, 6586)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(6582),
      this.instances3.alice.encrypt16(6586),
    );
    expect(res).to.equal(6590);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 3 (6586, 6586)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(6586),
      this.instances3.alice.encrypt16(6586),
    );
    expect(res).to.equal(6586);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 4 (6586, 6582)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(6586),
      this.instances3.alice.encrypt16(6582),
    );
    expect(res).to.equal(6590);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (20626, 44825)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(20626),
      this.instances3.alice.encrypt16(44825),
    );
    expect(res).to.equal(65419);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (20622, 20626)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(20622),
      this.instances3.alice.encrypt16(20626),
    );
    expect(res).to.equal(28);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 3 (20626, 20626)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(20626),
      this.instances3.alice.encrypt16(20626),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 4 (20626, 20622)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(20626),
      this.instances3.alice.encrypt16(20622),
    );
    expect(res).to.equal(28);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 1 (5463, 64736)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(5463),
      this.instances3.alice.encrypt16(64736),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 2 (5459, 5463)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(5459),
      this.instances3.alice.encrypt16(5463),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 3 (5463, 5463)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(5463),
      this.instances3.alice.encrypt16(5463),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 4 (5463, 5459)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(5463),
      this.instances3.alice.encrypt16(5459),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 1 (24558, 63661)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(24558),
      this.instances3.alice.encrypt16(63661),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 2 (24554, 24558)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(24554),
      this.instances3.alice.encrypt16(24558),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 3 (24558, 24558)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(24558),
      this.instances3.alice.encrypt16(24558),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 4 (24558, 24554)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(24558),
      this.instances3.alice.encrypt16(24554),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 1 (51421, 31489)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(51421),
      this.instances3.alice.encrypt16(31489),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 2 (31485, 31489)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(31485),
      this.instances3.alice.encrypt16(31489),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 3 (31489, 31489)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(31489),
      this.instances3.alice.encrypt16(31489),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 4 (31489, 31485)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(31489),
      this.instances3.alice.encrypt16(31485),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 1 (44465, 46930)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(44465),
      this.instances3.alice.encrypt16(46930),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 2 (44461, 44465)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(44461),
      this.instances3.alice.encrypt16(44465),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 3 (44465, 44465)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(44465),
      this.instances3.alice.encrypt16(44465),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 4 (44465, 44461)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(44465),
      this.instances3.alice.encrypt16(44461),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 1 (8195, 18657)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(8195),
      this.instances3.alice.encrypt16(18657),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 2 (8191, 8195)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(8191),
      this.instances3.alice.encrypt16(8195),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 3 (8195, 8195)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(8195),
      this.instances3.alice.encrypt16(8195),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 4 (8195, 8191)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(8195),
      this.instances3.alice.encrypt16(8191),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 1 (59556, 39223)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(59556),
      this.instances3.alice.encrypt16(39223),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 2 (39219, 39223)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(39219),
      this.instances3.alice.encrypt16(39223),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 3 (39223, 39223)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(39223),
      this.instances3.alice.encrypt16(39223),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 4 (39223, 39219)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(39223),
      this.instances3.alice.encrypt16(39219),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 1 (40365, 36544)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(40365),
      this.instances3.alice.encrypt16(36544),
    );
    expect(res).to.equal(36544);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 2 (36540, 36544)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(36540),
      this.instances3.alice.encrypt16(36544),
    );
    expect(res).to.equal(36540);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 3 (36544, 36544)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(36544),
      this.instances3.alice.encrypt16(36544),
    );
    expect(res).to.equal(36544);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 4 (36544, 36540)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(36544),
      this.instances3.alice.encrypt16(36540),
    );
    expect(res).to.equal(36540);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 1 (45208, 37832)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(45208),
      this.instances3.alice.encrypt16(37832),
    );
    expect(res).to.equal(45208);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 2 (37828, 37832)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(37828),
      this.instances3.alice.encrypt16(37832),
    );
    expect(res).to.equal(37832);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 3 (37832, 37832)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(37832),
      this.instances3.alice.encrypt16(37832),
    );
    expect(res).to.equal(37832);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 4 (37832, 37828)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(37832),
      this.instances3.alice.encrypt16(37828),
    );
    expect(res).to.equal(37832);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 1 (12, 38505)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(12),
      this.instances3.alice.encrypt32(38505),
    );
    expect(res).to.equal(38517n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 2 (24647, 24651)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(24647),
      this.instances3.alice.encrypt32(24651),
    );
    expect(res).to.equal(49298n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 3 (24651, 24651)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(24651),
      this.instances3.alice.encrypt32(24651),
    );
    expect(res).to.equal(49302n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 4 (24651, 24647)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(24651),
      this.instances3.alice.encrypt32(24647),
    );
    expect(res).to.equal(49298n);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (7479, 7479)', async function () {
    const res = await this.contract3.sub_euint16_euint32(
      this.instances3.alice.encrypt16(7479),
      this.instances3.alice.encrypt32(7479),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (7479, 7475)', async function () {
    const res = await this.contract3.sub_euint16_euint32(
      this.instances3.alice.encrypt16(7479),
      this.instances3.alice.encrypt32(7475),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (3, 8498)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(3),
      this.instances3.alice.encrypt32(8498),
    );
    expect(res).to.equal(25494n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 2 (209, 209)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(209),
      this.instances3.alice.encrypt32(209),
    );
    expect(res).to.equal(43681n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 3 (209, 209)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(209),
      this.instances3.alice.encrypt32(209),
    );
    expect(res).to.equal(43681n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 4 (209, 209)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(209),
      this.instances3.alice.encrypt32(209),
    );
    expect(res).to.equal(43681n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (55927, 26519890)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(55927),
      this.instances3.alice.encrypt32(26519890),
    );
    expect(res).to.equal(34898);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (55923, 55927)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(55923),
      this.instances3.alice.encrypt32(55927),
    );
    expect(res).to.equal(55923);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (55927, 55927)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(55927),
      this.instances3.alice.encrypt32(55927),
    );
    expect(res).to.equal(55927);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (55927, 55923)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(55927),
      this.instances3.alice.encrypt32(55923),
    );
    expect(res).to.equal(55923);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (25484, 156227915)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(25484),
      this.instances3.alice.encrypt32(156227915),
    );
    expect(res).to.equal(156236751);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (25480, 25484)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(25480),
      this.instances3.alice.encrypt32(25484),
    );
    expect(res).to.equal(25484);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 3 (25484, 25484)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(25484),
      this.instances3.alice.encrypt32(25484),
    );
    expect(res).to.equal(25484);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 4 (25484, 25480)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(25484),
      this.instances3.alice.encrypt32(25480),
    );
    expect(res).to.equal(25484);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 1 (20626, 50344759)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(20626),
      this.instances3.alice.encrypt32(50344759),
    );
    expect(res).to.equal(50357157);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 2 (20622, 20626)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(20622),
      this.instances3.alice.encrypt32(20626),
    );
    expect(res).to.equal(28);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 3 (20626, 20626)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(20626),
      this.instances3.alice.encrypt32(20626),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 4 (20626, 20622)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(20626),
      this.instances3.alice.encrypt32(20622),
    );
    expect(res).to.equal(28);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (47820, 152613323)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(47820),
      this.instances3.alice.encrypt32(152613323),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (47816, 47820)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(47816),
      this.instances3.alice.encrypt32(47820),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 3 (47820, 47820)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(47820),
      this.instances3.alice.encrypt32(47820),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 4 (47820, 47816)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(47820),
      this.instances3.alice.encrypt32(47816),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 1 (40202, 108078460)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(40202),
      this.instances3.alice.encrypt32(108078460),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 2 (40198, 40202)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(40198),
      this.instances3.alice.encrypt32(40202),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 3 (40202, 40202)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(40202),
      this.instances3.alice.encrypt32(40202),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 4 (40202, 40198)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(40202),
      this.instances3.alice.encrypt32(40198),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (44552, 89317183)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(44552),
      this.instances3.alice.encrypt32(89317183),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (44548, 44552)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(44548),
      this.instances3.alice.encrypt32(44552),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (44552, 44552)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(44552),
      this.instances3.alice.encrypt32(44552),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 4 (44552, 44548)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(44552),
      this.instances3.alice.encrypt32(44548),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 1 (40972, 46817193)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(40972),
      this.instances3.alice.encrypt32(46817193),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 2 (40968, 40972)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(40968),
      this.instances3.alice.encrypt32(40972),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 3 (40972, 40972)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(40972),
      this.instances3.alice.encrypt32(40972),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 4 (40972, 40968)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(40972),
      this.instances3.alice.encrypt32(40968),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (61583, 12489816)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(61583),
      this.instances3.alice.encrypt32(12489816),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (61579, 61583)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(61579),
      this.instances3.alice.encrypt32(61583),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (61583, 61583)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(61583),
      this.instances3.alice.encrypt32(61583),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 4 (61583, 61579)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(61583),
      this.instances3.alice.encrypt32(61579),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 1 (30406, 222597839)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(30406),
      this.instances3.alice.encrypt32(222597839),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 2 (30402, 30406)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(30402),
      this.instances3.alice.encrypt32(30406),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 3 (30406, 30406)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(30406),
      this.instances3.alice.encrypt32(30406),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 4 (30406, 30402)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(30406),
      this.instances3.alice.encrypt32(30402),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 1 (24104, 263622414)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(24104),
      this.instances3.alice.encrypt32(263622414),
    );
    expect(res).to.equal(24104);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 2 (24100, 24104)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(24100),
      this.instances3.alice.encrypt32(24104),
    );
    expect(res).to.equal(24100);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 3 (24104, 24104)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(24104),
      this.instances3.alice.encrypt32(24104),
    );
    expect(res).to.equal(24104);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 4 (24104, 24100)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(24104),
      this.instances3.alice.encrypt32(24100),
    );
    expect(res).to.equal(24100);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (62549, 239578021)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(62549),
      this.instances3.alice.encrypt32(239578021),
    );
    expect(res).to.equal(239578021);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (62545, 62549)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(62545),
      this.instances3.alice.encrypt32(62549),
    );
    expect(res).to.equal(62549);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (62549, 62549)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(62549),
      this.instances3.alice.encrypt32(62549),
    );
    expect(res).to.equal(62549);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 4 (62549, 62545)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(62549),
      this.instances3.alice.encrypt32(62545),
    );
    expect(res).to.equal(62549);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 1 (24, 33045)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(24),
      this.instances3.alice.encrypt64(33045),
    );
    expect(res).to.equal(33069n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 2 (24647, 24651)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(24647),
      this.instances3.alice.encrypt64(24651),
    );
    expect(res).to.equal(49298n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 3 (24651, 24651)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(24651),
      this.instances3.alice.encrypt64(24651),
    );
    expect(res).to.equal(49302n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 4 (24651, 24647)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(24651),
      this.instances3.alice.encrypt64(24647),
    );
    expect(res).to.equal(49298n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 1 (7479, 7479)', async function () {
    const res = await this.contract3.sub_euint16_euint64(
      this.instances3.alice.encrypt16(7479),
      this.instances3.alice.encrypt64(7479),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 2 (7479, 7475)', async function () {
    const res = await this.contract3.sub_euint16_euint64(
      this.instances3.alice.encrypt16(7479),
      this.instances3.alice.encrypt64(7475),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 1 (6, 6087)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(6),
      this.instances3.alice.encrypt64(6087),
    );
    expect(res).to.equal(36522n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 2 (209, 209)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(209),
      this.instances3.alice.encrypt64(209),
    );
    expect(res).to.equal(43681n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 3 (209, 209)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(209),
      this.instances3.alice.encrypt64(209),
    );
    expect(res).to.equal(43681n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 4 (209, 209)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(209),
      this.instances3.alice.encrypt64(209),
    );
    expect(res).to.equal(43681n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 1 (55927, 66357780)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(55927),
      this.instances3.alice.encrypt64(66357780),
    );
    expect(res).to.equal(35348);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 2 (55923, 55927)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(55923),
      this.instances3.alice.encrypt64(55927),
    );
    expect(res).to.equal(55923);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 3 (55927, 55927)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(55927),
      this.instances3.alice.encrypt64(55927),
    );
    expect(res).to.equal(55927);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 4 (55927, 55923)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(55927),
      this.instances3.alice.encrypt64(55923),
    );
    expect(res).to.equal(55923);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 1 (25484, 265773773)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(25484),
      this.instances3.alice.encrypt64(265773773),
    );
    expect(res).to.equal(265774029);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 2 (25480, 25484)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(25480),
      this.instances3.alice.encrypt64(25484),
    );
    expect(res).to.equal(25484);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 3 (25484, 25484)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(25484),
      this.instances3.alice.encrypt64(25484),
    );
    expect(res).to.equal(25484);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 4 (25484, 25480)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(25484),
      this.instances3.alice.encrypt64(25480),
    );
    expect(res).to.equal(25484);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 1 (20626, 20674993)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(20626),
      this.instances3.alice.encrypt64(20674993),
    );
    expect(res).to.equal(20654371);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 2 (20622, 20626)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(20622),
      this.instances3.alice.encrypt64(20626),
    );
    expect(res).to.equal(28);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 3 (20626, 20626)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(20626),
      this.instances3.alice.encrypt64(20626),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 4 (20626, 20622)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(20626),
      this.instances3.alice.encrypt64(20622),
    );
    expect(res).to.equal(28);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 1 (47820, 145445907)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(47820),
      this.instances3.alice.encrypt64(145445907),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 2 (47816, 47820)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(47816),
      this.instances3.alice.encrypt64(47820),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 3 (47820, 47820)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(47820),
      this.instances3.alice.encrypt64(47820),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 4 (47820, 47816)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(47820),
      this.instances3.alice.encrypt64(47816),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 1 (40202, 163102475)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(40202),
      this.instances3.alice.encrypt64(163102475),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 2 (40198, 40202)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(40198),
      this.instances3.alice.encrypt64(40202),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 3 (40202, 40202)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(40202),
      this.instances3.alice.encrypt64(40202),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 4 (40202, 40198)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(40202),
      this.instances3.alice.encrypt64(40198),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 1 (44552, 221643140)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(44552),
      this.instances3.alice.encrypt64(221643140),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 2 (44548, 44552)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(44548),
      this.instances3.alice.encrypt64(44552),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 3 (44552, 44552)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(44552),
      this.instances3.alice.encrypt64(44552),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 4 (44552, 44548)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(44552),
      this.instances3.alice.encrypt64(44548),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 1 (40972, 29158802)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(40972),
      this.instances3.alice.encrypt64(29158802),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 2 (40968, 40972)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(40968),
      this.instances3.alice.encrypt64(40972),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 3 (40972, 40972)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(40972),
      this.instances3.alice.encrypt64(40972),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 4 (40972, 40968)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(40972),
      this.instances3.alice.encrypt64(40968),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 1 (61583, 200076583)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(61583),
      this.instances3.alice.encrypt64(200076583),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 2 (61579, 61583)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(61579),
      this.instances3.alice.encrypt64(61583),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 3 (61583, 61583)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(61583),
      this.instances3.alice.encrypt64(61583),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 4 (61583, 61579)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(61583),
      this.instances3.alice.encrypt64(61579),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 1 (30406, 141165411)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(30406),
      this.instances3.alice.encrypt64(141165411),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 2 (30402, 30406)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(30402),
      this.instances3.alice.encrypt64(30406),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 3 (30406, 30406)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(30406),
      this.instances3.alice.encrypt64(30406),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 4 (30406, 30402)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(30406),
      this.instances3.alice.encrypt64(30402),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 1 (24104, 100961112)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(24104),
      this.instances3.alice.encrypt64(100961112),
    );
    expect(res).to.equal(24104);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 2 (24100, 24104)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(24100),
      this.instances3.alice.encrypt64(24104),
    );
    expect(res).to.equal(24100);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 3 (24104, 24104)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(24104),
      this.instances3.alice.encrypt64(24104),
    );
    expect(res).to.equal(24104);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 4 (24104, 24100)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(24104),
      this.instances3.alice.encrypt64(24100),
    );
    expect(res).to.equal(24100);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 1 (62549, 228731288)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(62549),
      this.instances3.alice.encrypt64(228731288),
    );
    expect(res).to.equal(228731288);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 2 (62545, 62549)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(62545),
      this.instances3.alice.encrypt64(62549),
    );
    expect(res).to.equal(62549);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 3 (62549, 62549)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(62549),
      this.instances3.alice.encrypt64(62549),
    );
    expect(res).to.equal(62549);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 4 (62549, 62545)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(62549),
      this.instances3.alice.encrypt64(62545),
    );
    expect(res).to.equal(62549);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (1870, 31236)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(1870), 31236);
    expect(res).to.equal(33106n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (1866, 1870)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(1866), 1870);
    expect(res).to.equal(3736n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (1870, 1870)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(1870), 1870);
    expect(res).to.equal(3740n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (1870, 1866)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(1870), 1866);
    expect(res).to.equal(3736n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (24651, 31236)', async function () {
    const res = await this.contract3.add_uint16_euint16(24651, this.instances3.alice.encrypt16(31236));
    expect(res).to.equal(55887n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 2 (1866, 1870)', async function () {
    const res = await this.contract3.add_uint16_euint16(1866, this.instances3.alice.encrypt16(1870));
    expect(res).to.equal(3736n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 3 (1870, 1870)', async function () {
    const res = await this.contract3.add_uint16_euint16(1870, this.instances3.alice.encrypt16(1870));
    expect(res).to.equal(3740n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 4 (1870, 1866)', async function () {
    const res = await this.contract3.add_uint16_euint16(1870, this.instances3.alice.encrypt16(1866));
    expect(res).to.equal(3736n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (4130, 4130)', async function () {
    const res = await this.contract3.sub_euint16_uint16(this.instances3.alice.encrypt16(4130), 4130);
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 2 (4130, 4126)', async function () {
    const res = await this.contract3.sub_euint16_uint16(this.instances3.alice.encrypt16(4130), 4126);
    expect(res).to.equal(4);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (4130, 4130)', async function () {
    const res = await this.contract3.sub_uint16_euint16(4130, this.instances3.alice.encrypt16(4130));
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (4130, 4126)', async function () {
    const res = await this.contract3.sub_uint16_euint16(4130, this.instances3.alice.encrypt16(4126));
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (207, 286)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(207), 286);
    expect(res).to.equal(59202n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (206, 207)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(206), 207);
    expect(res).to.equal(42642n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (207, 207)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(207), 207);
    expect(res).to.equal(42849n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (207, 206)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(207), 206);
    expect(res).to.equal(42642n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (419, 71)', async function () {
    const res = await this.contract3.mul_uint16_euint16(419, this.instances3.alice.encrypt16(71));
    expect(res).to.equal(29749n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 2 (206, 207)', async function () {
    const res = await this.contract3.mul_uint16_euint16(206, this.instances3.alice.encrypt16(207));
    expect(res).to.equal(42642n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 3 (207, 207)', async function () {
    const res = await this.contract3.mul_uint16_euint16(207, this.instances3.alice.encrypt16(207));
    expect(res).to.equal(42849n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 4 (207, 206)', async function () {
    const res = await this.contract3.mul_uint16_euint16(207, this.instances3.alice.encrypt16(206));
    expect(res).to.equal(42642n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (21163, 49228)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(21163), 49228);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (21159, 21163)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(21159), 21163);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (21163, 21163)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(21163), 21163);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (21163, 21159)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(21163), 21159);
    expect(res).to.equal(1);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (44670, 10464)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(44670), 10464);
    expect(res).to.equal(2814);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 2 (44666, 44670)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(44666), 44670);
    expect(res).to.equal(44666);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 3 (44670, 44670)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(44670), 44670);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 4 (44670, 44666)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(44670), 44666);
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (5463, 19063)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(5463), 19063);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (5459, 5463)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(5459), 5463);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (5463, 5463)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(5463), 5463);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (5463, 5459)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(5463), 5459);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (47820, 19063)', async function () {
    const res = await this.contract3.eq_uint16_euint16(47820, this.instances3.alice.encrypt16(19063));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (5459, 5463)', async function () {
    const res = await this.contract3.eq_uint16_euint16(5459, this.instances3.alice.encrypt16(5463));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 3 (5463, 5463)', async function () {
    const res = await this.contract3.eq_uint16_euint16(5463, this.instances3.alice.encrypt16(5463));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 4 (5463, 5459)', async function () {
    const res = await this.contract3.eq_uint16_euint16(5463, this.instances3.alice.encrypt16(5459));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (24558, 4639)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(24558), 4639);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (24554, 24558)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(24554), 24558);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 3 (24558, 24558)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(24558), 24558);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 4 (24558, 24554)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(24558), 24554);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (40202, 4639)', async function () {
    const res = await this.contract3.ne_uint16_euint16(40202, this.instances3.alice.encrypt16(4639));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (24554, 24558)', async function () {
    const res = await this.contract3.ne_uint16_euint16(24554, this.instances3.alice.encrypt16(24558));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 3 (24558, 24558)', async function () {
    const res = await this.contract3.ne_uint16_euint16(24558, this.instances3.alice.encrypt16(24558));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 4 (24558, 24554)', async function () {
    const res = await this.contract3.ne_uint16_euint16(24558, this.instances3.alice.encrypt16(24554));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (51421, 1063)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(51421), 1063);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (31485, 31489)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(31485), 31489);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (31489, 31489)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(31489), 31489);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 4 (31489, 31485)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(31489), 31485);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (44552, 1063)', async function () {
    const res = await this.contract3.ge_uint16_euint16(44552, this.instances3.alice.encrypt16(1063));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (31485, 31489)', async function () {
    const res = await this.contract3.ge_uint16_euint16(31485, this.instances3.alice.encrypt16(31489));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (31489, 31489)', async function () {
    const res = await this.contract3.ge_uint16_euint16(31489, this.instances3.alice.encrypt16(31489));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 4 (31489, 31485)', async function () {
    const res = await this.contract3.ge_uint16_euint16(31489, this.instances3.alice.encrypt16(31485));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (44465, 56813)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(44465), 56813);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (44461, 44465)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(44461), 44465);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (44465, 44465)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(44465), 44465);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 4 (44465, 44461)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(44465), 44461);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (40972, 56813)', async function () {
    const res = await this.contract3.gt_uint16_euint16(40972, this.instances3.alice.encrypt16(56813));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (44461, 44465)', async function () {
    const res = await this.contract3.gt_uint16_euint16(44461, this.instances3.alice.encrypt16(44465));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (44465, 44465)', async function () {
    const res = await this.contract3.gt_uint16_euint16(44465, this.instances3.alice.encrypt16(44465));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 4 (44465, 44461)', async function () {
    const res = await this.contract3.gt_uint16_euint16(44465, this.instances3.alice.encrypt16(44461));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (8195, 15612)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(8195), 15612);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (8191, 8195)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(8191), 8195);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (8195, 8195)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(8195), 8195);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 4 (8195, 8191)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(8195), 8191);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (61583, 15612)', async function () {
    const res = await this.contract3.le_uint16_euint16(61583, this.instances3.alice.encrypt16(15612));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (8191, 8195)', async function () {
    const res = await this.contract3.le_uint16_euint16(8191, this.instances3.alice.encrypt16(8195));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (8195, 8195)', async function () {
    const res = await this.contract3.le_uint16_euint16(8195, this.instances3.alice.encrypt16(8195));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (8195, 8191)', async function () {
    const res = await this.contract3.le_uint16_euint16(8195, this.instances3.alice.encrypt16(8191));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (59556, 9412)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(59556), 9412);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (39219, 39223)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(39219), 39223);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (39223, 39223)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(39223), 39223);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 4 (39223, 39219)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(39223), 39219);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (30406, 9412)', async function () {
    const res = await this.contract3.lt_uint16_euint16(30406, this.instances3.alice.encrypt16(9412));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (39219, 39223)', async function () {
    const res = await this.contract3.lt_uint16_euint16(39219, this.instances3.alice.encrypt16(39223));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (39223, 39223)', async function () {
    const res = await this.contract3.lt_uint16_euint16(39223, this.instances3.alice.encrypt16(39223));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 4 (39223, 39219)', async function () {
    const res = await this.contract3.lt_uint16_euint16(39223, this.instances3.alice.encrypt16(39219));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (40365, 56848)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(40365), 56848);
    expect(res).to.equal(40365);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (36540, 36544)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(36540), 36544);
    expect(res).to.equal(36540);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (36544, 36544)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(36544), 36544);
    expect(res).to.equal(36544);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 4 (36544, 36540)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(36544), 36540);
    expect(res).to.equal(36540);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (24104, 56848)', async function () {
    const res = await this.contract3.min_uint16_euint16(24104, this.instances3.alice.encrypt16(56848));
    expect(res).to.equal(24104);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (36540, 36544)', async function () {
    const res = await this.contract3.min_uint16_euint16(36540, this.instances3.alice.encrypt16(36544));
    expect(res).to.equal(36540);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (36544, 36544)', async function () {
    const res = await this.contract3.min_uint16_euint16(36544, this.instances3.alice.encrypt16(36544));
    expect(res).to.equal(36544);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (36544, 36540)', async function () {
    const res = await this.contract3.min_uint16_euint16(36544, this.instances3.alice.encrypt16(36540));
    expect(res).to.equal(36540);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (45208, 8803)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(45208), 8803);
    expect(res).to.equal(45208);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (37828, 37832)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(37828), 37832);
    expect(res).to.equal(37832);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (37832, 37832)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(37832), 37832);
    expect(res).to.equal(37832);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (37832, 37828)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(37832), 37828);
    expect(res).to.equal(37832);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (62549, 8803)', async function () {
    const res = await this.contract3.max_uint16_euint16(62549, this.instances3.alice.encrypt16(8803));
    expect(res).to.equal(62549);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (37828, 37832)', async function () {
    const res = await this.contract3.max_uint16_euint16(37828, this.instances3.alice.encrypt16(37832));
    expect(res).to.equal(37832);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (37832, 37832)', async function () {
    const res = await this.contract3.max_uint16_euint16(37832, this.instances3.alice.encrypt16(37832));
    expect(res).to.equal(37832);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (37832, 37828)', async function () {
    const res = await this.contract3.max_uint16_euint16(37832, this.instances3.alice.encrypt16(37828));
    expect(res).to.equal(37832);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 1 (8, 1)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(1),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 2 (3, 5)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(3),
      this.instances3.alice.encrypt4(5),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 3 (5, 5)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(5),
      this.instances3.alice.encrypt4(5),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 4 (5, 3)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(5),
      this.instances3.alice.encrypt4(3),
    );
    expect(res).to.equal(8n);
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

  it('test operator "mul" overload (euint32, euint4) => euint32 test 1 (9, 1)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(1),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 2 (2, 4)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(2),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 3 (2, 2)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(2),
      this.instances3.alice.encrypt4(2),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 4 (4, 2)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(2),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 1 (145081557, 6)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(145081557),
      this.instances3.alice.encrypt4(6),
    );
    expect(res).to.equal(4);
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

  it('test operator "or" overload (euint32, euint4) => euint32 test 1 (138135196, 6)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(138135196),
      this.instances3.alice.encrypt4(6),
    );
    expect(res).to.equal(138135198);
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

  it('test operator "xor" overload (euint32, euint4) => euint32 test 1 (27190055, 14)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(27190055),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(27190057);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 2 (10, 14)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(10),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 3 (14, 14)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(14),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 4 (14, 10)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(14),
      this.instances3.alice.encrypt4(10),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 1 (60042820, 13)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(60042820),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(13),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(13),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 1 (83574454, 8)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(83574454),
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

  it('test operator "ge" overload (euint32, euint4) => ebool test 1 (55218710, 2)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(55218710),
      this.instances3.alice.encrypt4(2),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 1 (44051971, 14)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(44051971),
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

  it('test operator "le" overload (euint32, euint4) => ebool test 1 (253532839, 12)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(253532839),
      this.instances3.alice.encrypt4(12),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(12),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(12),
      this.instances3.alice.encrypt4(12),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(12),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 1 (77180163, 11)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(77180163),
      this.instances3.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(7),
      this.instances3.alice.encrypt4(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(11),
      this.instances3.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(11),
      this.instances3.alice.encrypt4(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 1 (178824069, 14)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(178824069),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 2 (10, 14)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(10),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(10);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 3 (14, 14)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(14),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(14);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 4 (14, 10)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(14),
      this.instances3.alice.encrypt4(10),
    );
    expect(res).to.equal(10);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 1 (234155387, 12)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(234155387),
      this.instances3.alice.encrypt4(12),
    );
    expect(res).to.equal(234155387);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 2 (8, 12)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(12),
    );
    expect(res).to.equal(12);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 3 (12, 12)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(12),
      this.instances3.alice.encrypt4(12),
    );
    expect(res).to.equal(12);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 4 (12, 8)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(12),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(12);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (134, 1)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(134),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(135n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (76, 80)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(76),
      this.instances3.alice.encrypt8(80),
    );
    expect(res).to.equal(156n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (80, 80)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(80),
      this.instances3.alice.encrypt8(80),
    );
    expect(res).to.equal(160n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (80, 76)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(80),
      this.instances3.alice.encrypt8(76),
    );
    expect(res).to.equal(156n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (186, 186)', async function () {
    const res = await this.contract3.sub_euint32_euint8(
      this.instances3.alice.encrypt32(186),
      this.instances3.alice.encrypt8(186),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (186, 182)', async function () {
    const res = await this.contract3.sub_euint32_euint8(
      this.instances3.alice.encrypt32(186),
      this.instances3.alice.encrypt8(182),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (146, 1)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(146),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(146n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 2 (12, 12)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(12),
      this.instances3.alice.encrypt8(12),
    );
    expect(res).to.equal(144n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 3 (12, 12)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(12),
      this.instances3.alice.encrypt8(12),
    );
    expect(res).to.equal(144n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 4 (12, 12)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(12),
      this.instances3.alice.encrypt8(12),
    );
    expect(res).to.equal(144n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (145081557, 138)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(145081557),
      this.instances3.alice.encrypt8(138),
    );
    expect(res).to.equal(128);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (134, 138)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(134),
      this.instances3.alice.encrypt8(138),
    );
    expect(res).to.equal(130);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (138, 138)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(138),
      this.instances3.alice.encrypt8(138),
    );
    expect(res).to.equal(138);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (138, 134)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(138),
      this.instances3.alice.encrypt8(134),
    );
    expect(res).to.equal(130);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (138135196, 250)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(138135196),
      this.instances4.alice.encrypt8(250),
    );
    expect(res).to.equal(138135294);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (246, 250)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(246),
      this.instances4.alice.encrypt8(250),
    );
    expect(res).to.equal(254);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (250, 250)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(250),
      this.instances4.alice.encrypt8(250),
    );
    expect(res).to.equal(250);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (250, 246)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(250),
      this.instances4.alice.encrypt8(246),
    );
    expect(res).to.equal(254);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (27190055, 85)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(27190055),
      this.instances4.alice.encrypt8(85),
    );
    expect(res).to.equal(27190130);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (81, 85)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(81),
      this.instances4.alice.encrypt8(85),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (85, 85)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(85),
      this.instances4.alice.encrypt8(85),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (85, 81)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(85),
      this.instances4.alice.encrypt8(81),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (60042820, 117)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(60042820),
      this.instances4.alice.encrypt8(117),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (113, 117)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(113),
      this.instances4.alice.encrypt8(117),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 3 (117, 117)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(117),
      this.instances4.alice.encrypt8(117),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 4 (117, 113)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(117),
      this.instances4.alice.encrypt8(113),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (83574454, 68)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(83574454),
      this.instances4.alice.encrypt8(68),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (64, 68)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(64),
      this.instances4.alice.encrypt8(68),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 3 (68, 68)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(68),
      this.instances4.alice.encrypt8(68),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 4 (68, 64)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(68),
      this.instances4.alice.encrypt8(64),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (55218710, 19)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(55218710),
      this.instances4.alice.encrypt8(19),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (15, 19)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(15),
      this.instances4.alice.encrypt8(19),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (19, 19)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(19),
      this.instances4.alice.encrypt8(19),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 4 (19, 15)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(19),
      this.instances4.alice.encrypt8(15),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (44051971, 62)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(44051971),
      this.instances4.alice.encrypt8(62),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (58, 62)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(58),
      this.instances4.alice.encrypt8(62),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (62, 62)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(62),
      this.instances4.alice.encrypt8(62),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 4 (62, 58)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(62),
      this.instances4.alice.encrypt8(58),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (253532839, 184)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(253532839),
      this.instances4.alice.encrypt8(184),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (180, 184)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(180),
      this.instances4.alice.encrypt8(184),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (184, 184)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(184),
      this.instances4.alice.encrypt8(184),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (184, 180)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(184),
      this.instances4.alice.encrypt8(180),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (77180163, 96)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(77180163),
      this.instances4.alice.encrypt8(96),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (92, 96)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(92),
      this.instances4.alice.encrypt8(96),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (96, 96)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(96),
      this.instances4.alice.encrypt8(96),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 4 (96, 92)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(96),
      this.instances4.alice.encrypt8(92),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (178824069, 96)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(178824069),
      this.instances4.alice.encrypt8(96),
    );
    expect(res).to.equal(96);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (92, 96)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(92),
      this.instances4.alice.encrypt8(96),
    );
    expect(res).to.equal(92);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (96, 96)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(96),
      this.instances4.alice.encrypt8(96),
    );
    expect(res).to.equal(96);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (96, 92)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(96),
      this.instances4.alice.encrypt8(92),
    );
    expect(res).to.equal(92);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (234155387, 198)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(234155387),
      this.instances4.alice.encrypt8(198),
    );
    expect(res).to.equal(234155387);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (194, 198)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(194),
      this.instances4.alice.encrypt8(198),
    );
    expect(res).to.equal(198);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (198, 198)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(198),
      this.instances4.alice.encrypt8(198),
    );
    expect(res).to.equal(198);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 4 (198, 194)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(198),
      this.instances4.alice.encrypt8(194),
    );
    expect(res).to.equal(198);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (34382, 13)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(34382),
      this.instances4.alice.encrypt16(13),
    );
    expect(res).to.equal(34395n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (27184, 27186)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(27184),
      this.instances4.alice.encrypt16(27186),
    );
    expect(res).to.equal(54370n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (27186, 27186)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(27186),
      this.instances4.alice.encrypt16(27186),
    );
    expect(res).to.equal(54372n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (27186, 27184)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(27186),
      this.instances4.alice.encrypt16(27184),
    );
    expect(res).to.equal(54370n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (30277, 30277)', async function () {
    const res = await this.contract4.sub_euint32_euint16(
      this.instances4.alice.encrypt32(30277),
      this.instances4.alice.encrypt16(30277),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (30277, 30273)', async function () {
    const res = await this.contract4.sub_euint32_euint16(
      this.instances4.alice.encrypt32(30277),
      this.instances4.alice.encrypt16(30273),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (9347, 3)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(9347),
      this.instances4.alice.encrypt16(3),
    );
    expect(res).to.equal(28041n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (234, 234)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(234),
      this.instances4.alice.encrypt16(234),
    );
    expect(res).to.equal(54756n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (234, 234)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(234),
      this.instances4.alice.encrypt16(234),
    );
    expect(res).to.equal(54756n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (234, 234)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(234),
      this.instances4.alice.encrypt16(234),
    );
    expect(res).to.equal(54756n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (145081557, 34219)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(145081557),
      this.instances4.alice.encrypt16(34219),
    );
    expect(res).to.equal(33921);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (34215, 34219)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(34215),
      this.instances4.alice.encrypt16(34219),
    );
    expect(res).to.equal(34211);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (34219, 34219)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(34219),
      this.instances4.alice.encrypt16(34219),
    );
    expect(res).to.equal(34219);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (34219, 34215)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(34219),
      this.instances4.alice.encrypt16(34215),
    );
    expect(res).to.equal(34211);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (138135196, 24679)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(138135196),
      this.instances4.alice.encrypt16(24679),
    );
    expect(res).to.equal(138143487);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (24675, 24679)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(24675),
      this.instances4.alice.encrypt16(24679),
    );
    expect(res).to.equal(24679);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 3 (24679, 24679)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(24679),
      this.instances4.alice.encrypt16(24679),
    );
    expect(res).to.equal(24679);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 4 (24679, 24675)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(24679),
      this.instances4.alice.encrypt16(24675),
    );
    expect(res).to.equal(24679);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (27190055, 32965)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(27190055),
      this.instances4.alice.encrypt16(32965),
    );
    expect(res).to.equal(27157474);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (32961, 32965)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(32961),
      this.instances4.alice.encrypt16(32965),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (32965, 32965)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(32965),
      this.instances4.alice.encrypt16(32965),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (32965, 32961)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(32965),
      this.instances4.alice.encrypt16(32961),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (60042820, 3366)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(60042820),
      this.instances4.alice.encrypt16(3366),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (3362, 3366)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(3362),
      this.instances4.alice.encrypt16(3366),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (3366, 3366)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(3366),
      this.instances4.alice.encrypt16(3366),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (3366, 3362)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(3366),
      this.instances4.alice.encrypt16(3362),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (83574454, 51513)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(83574454),
      this.instances4.alice.encrypt16(51513),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (51509, 51513)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(51509),
      this.instances4.alice.encrypt16(51513),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 3 (51513, 51513)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(51513),
      this.instances4.alice.encrypt16(51513),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 4 (51513, 51509)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(51513),
      this.instances4.alice.encrypt16(51509),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (55218710, 21355)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(55218710),
      this.instances4.alice.encrypt16(21355),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (21351, 21355)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(21351),
      this.instances4.alice.encrypt16(21355),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (21355, 21355)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(21355),
      this.instances4.alice.encrypt16(21355),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 4 (21355, 21351)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(21355),
      this.instances4.alice.encrypt16(21351),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (44051971, 62126)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(44051971),
      this.instances4.alice.encrypt16(62126),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (62122, 62126)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(62122),
      this.instances4.alice.encrypt16(62126),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (62126, 62126)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(62126),
      this.instances4.alice.encrypt16(62126),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 4 (62126, 62122)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(62126),
      this.instances4.alice.encrypt16(62122),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (253532839, 30629)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(253532839),
      this.instances4.alice.encrypt16(30629),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (30625, 30629)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(30625),
      this.instances4.alice.encrypt16(30629),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (30629, 30629)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(30629),
      this.instances4.alice.encrypt16(30629),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (30629, 30625)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(30629),
      this.instances4.alice.encrypt16(30625),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (77180163, 31631)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(77180163),
      this.instances4.alice.encrypt16(31631),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (31627, 31631)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(31627),
      this.instances4.alice.encrypt16(31631),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (31631, 31631)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(31631),
      this.instances4.alice.encrypt16(31631),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (31631, 31627)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(31631),
      this.instances4.alice.encrypt16(31627),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (178824069, 29702)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(178824069),
      this.instances4.alice.encrypt16(29702),
    );
    expect(res).to.equal(29702);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (29698, 29702)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(29698),
      this.instances4.alice.encrypt16(29702),
    );
    expect(res).to.equal(29698);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (29702, 29702)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(29702),
      this.instances4.alice.encrypt16(29702),
    );
    expect(res).to.equal(29702);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (29702, 29698)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(29702),
      this.instances4.alice.encrypt16(29698),
    );
    expect(res).to.equal(29698);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (234155387, 33887)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(234155387),
      this.instances4.alice.encrypt16(33887),
    );
    expect(res).to.equal(234155387);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (33883, 33887)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(33883),
      this.instances4.alice.encrypt16(33887),
    );
    expect(res).to.equal(33887);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (33887, 33887)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(33887),
      this.instances4.alice.encrypt16(33887),
    );
    expect(res).to.equal(33887);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (33887, 33883)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(33887),
      this.instances4.alice.encrypt16(33883),
    );
    expect(res).to.equal(33887);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (140830493, 233846954)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(140830493),
      this.instances4.alice.encrypt32(233846954),
    );
    expect(res).to.equal(374677447n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (140830489, 140830493)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(140830489),
      this.instances4.alice.encrypt32(140830493),
    );
    expect(res).to.equal(281660982n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (140830493, 140830493)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(140830493),
      this.instances4.alice.encrypt32(140830493),
    );
    expect(res).to.equal(281660986n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (140830493, 140830489)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(140830493),
      this.instances4.alice.encrypt32(140830489),
    );
    expect(res).to.equal(281660982n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (72155792, 72155792)', async function () {
    const res = await this.contract4.sub_euint32_euint32(
      this.instances4.alice.encrypt32(72155792),
      this.instances4.alice.encrypt32(72155792),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (72155792, 72155788)', async function () {
    const res = await this.contract4.sub_euint32_euint32(
      this.instances4.alice.encrypt32(72155792),
      this.instances4.alice.encrypt32(72155788),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (149553, 18251)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(149553),
      this.instances4.alice.encrypt32(18251),
    );
    expect(res).to.equal(2729491803n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 2 (36502, 36502)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(36502),
      this.instances4.alice.encrypt32(36502),
    );
    expect(res).to.equal(1332396004n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 3 (36502, 36502)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(36502),
      this.instances4.alice.encrypt32(36502),
    );
    expect(res).to.equal(1332396004n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 4 (36502, 36502)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(36502),
      this.instances4.alice.encrypt32(36502),
    );
    expect(res).to.equal(1332396004n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (145081557, 40551396)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(145081557),
      this.instances4.alice.encrypt32(40551396),
    );
    expect(res).to.equal(2146500);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (40551392, 40551396)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(40551392),
      this.instances4.alice.encrypt32(40551396),
    );
    expect(res).to.equal(40551392);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 3 (40551396, 40551396)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(40551396),
      this.instances4.alice.encrypt32(40551396),
    );
    expect(res).to.equal(40551396);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 4 (40551396, 40551392)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(40551396),
      this.instances4.alice.encrypt32(40551392),
    );
    expect(res).to.equal(40551392);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (138135196, 147717442)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(138135196),
      this.instances4.alice.encrypt32(147717442),
    );
    expect(res).to.equal(150994910);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (138135192, 138135196)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(138135192),
      this.instances4.alice.encrypt32(138135196),
    );
    expect(res).to.equal(138135196);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 3 (138135196, 138135196)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(138135196),
      this.instances4.alice.encrypt32(138135196),
    );
    expect(res).to.equal(138135196);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 4 (138135196, 138135192)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(138135196),
      this.instances4.alice.encrypt32(138135192),
    );
    expect(res).to.equal(138135196);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (27190055, 97550225)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(27190055),
      this.instances4.alice.encrypt32(97550225),
    );
    expect(res).to.equal(72260790);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (27190051, 27190055)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(27190051),
      this.instances4.alice.encrypt32(27190055),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (27190055, 27190055)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(27190055),
      this.instances4.alice.encrypt32(27190055),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (27190055, 27190051)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(27190055),
      this.instances4.alice.encrypt32(27190051),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (60042820, 132291744)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(60042820),
      this.instances4.alice.encrypt32(132291744),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (60042816, 60042820)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(60042816),
      this.instances4.alice.encrypt32(60042820),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (60042820, 60042820)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(60042820),
      this.instances4.alice.encrypt32(60042820),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (60042820, 60042816)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(60042820),
      this.instances4.alice.encrypt32(60042816),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (83574454, 51569943)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(83574454),
      this.instances4.alice.encrypt32(51569943),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (51569939, 51569943)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(51569939),
      this.instances4.alice.encrypt32(51569943),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 3 (51569943, 51569943)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(51569943),
      this.instances4.alice.encrypt32(51569943),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 4 (51569943, 51569939)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(51569943),
      this.instances4.alice.encrypt32(51569939),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (55218710, 165530810)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(55218710),
      this.instances4.alice.encrypt32(165530810),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (55218706, 55218710)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(55218706),
      this.instances4.alice.encrypt32(55218710),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (55218710, 55218710)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(55218710),
      this.instances4.alice.encrypt32(55218710),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (55218710, 55218706)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(55218710),
      this.instances4.alice.encrypt32(55218706),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (44051971, 234370209)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(44051971),
      this.instances4.alice.encrypt32(234370209),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (44051967, 44051971)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(44051967),
      this.instances4.alice.encrypt32(44051971),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (44051971, 44051971)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(44051971),
      this.instances4.alice.encrypt32(44051971),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 4 (44051971, 44051967)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(44051971),
      this.instances4.alice.encrypt32(44051967),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (253532839, 259475014)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(253532839),
      this.instances4.alice.encrypt32(259475014),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (253532835, 253532839)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(253532835),
      this.instances4.alice.encrypt32(253532839),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (253532839, 253532839)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(253532839),
      this.instances4.alice.encrypt32(253532839),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 4 (253532839, 253532835)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(253532839),
      this.instances4.alice.encrypt32(253532835),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (77180163, 25179168)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(77180163),
      this.instances4.alice.encrypt32(25179168),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (25179164, 25179168)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(25179164),
      this.instances4.alice.encrypt32(25179168),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (25179168, 25179168)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(25179168),
      this.instances4.alice.encrypt32(25179168),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (25179168, 25179164)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(25179168),
      this.instances4.alice.encrypt32(25179164),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (178824069, 78448235)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(178824069),
      this.instances4.alice.encrypt32(78448235),
    );
    expect(res).to.equal(78448235);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (78448231, 78448235)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(78448231),
      this.instances4.alice.encrypt32(78448235),
    );
    expect(res).to.equal(78448231);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (78448235, 78448235)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(78448235),
      this.instances4.alice.encrypt32(78448235),
    );
    expect(res).to.equal(78448235);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (78448235, 78448231)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(78448235),
      this.instances4.alice.encrypt32(78448231),
    );
    expect(res).to.equal(78448231);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (234155387, 122578470)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(234155387),
      this.instances4.alice.encrypt32(122578470),
    );
    expect(res).to.equal(234155387);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (122578466, 122578470)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(122578466),
      this.instances4.alice.encrypt32(122578470),
    );
    expect(res).to.equal(122578470);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (122578470, 122578470)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(122578470),
      this.instances4.alice.encrypt32(122578470),
    );
    expect(res).to.equal(122578470);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (122578470, 122578466)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(122578470),
      this.instances4.alice.encrypt32(122578466),
    );
    expect(res).to.equal(122578470);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 1 (88545588, 105438338)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(88545588),
      this.instances4.alice.encrypt64(105438338),
    );
    expect(res).to.equal(193983926n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 2 (88545584, 88545588)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(88545584),
      this.instances4.alice.encrypt64(88545588),
    );
    expect(res).to.equal(177091172n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 3 (88545588, 88545588)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(88545588),
      this.instances4.alice.encrypt64(88545588),
    );
    expect(res).to.equal(177091176n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 4 (88545588, 88545584)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(88545588),
      this.instances4.alice.encrypt64(88545584),
    );
    expect(res).to.equal(177091172n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (119509877, 119509877)', async function () {
    const res = await this.contract4.sub_euint32_euint64(
      this.instances4.alice.encrypt32(119509877),
      this.instances4.alice.encrypt64(119509877),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (119509877, 119509873)', async function () {
    const res = await this.contract4.sub_euint32_euint64(
      this.instances4.alice.encrypt32(119509877),
      this.instances4.alice.encrypt64(119509873),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (64389, 42894)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(64389),
      this.instances4.alice.encrypt64(42894),
    );
    expect(res).to.equal(2761901766n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (42894, 42894)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(42894),
      this.instances4.alice.encrypt64(42894),
    );
    expect(res).to.equal(1839895236n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (42894, 42894)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(42894),
      this.instances4.alice.encrypt64(42894),
    );
    expect(res).to.equal(1839895236n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (42894, 42894)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(42894),
      this.instances4.alice.encrypt64(42894),
    );
    expect(res).to.equal(1839895236n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 1 (145081557, 212896076)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(145081557),
      this.instances4.alice.encrypt64(212896076),
    );
    expect(res).to.equal(144736324);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 2 (145081553, 145081557)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(145081553),
      this.instances4.alice.encrypt64(145081557),
    );
    expect(res).to.equal(145081553);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 3 (145081557, 145081557)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(145081557),
      this.instances4.alice.encrypt64(145081557),
    );
    expect(res).to.equal(145081557);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 4 (145081557, 145081553)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(145081557),
      this.instances4.alice.encrypt64(145081553),
    );
    expect(res).to.equal(145081553);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 1 (138135196, 193074409)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(138135196),
      this.instances4.alice.encrypt64(193074409),
    );
    expect(res).to.equal(196859645);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 2 (138135192, 138135196)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(138135192),
      this.instances4.alice.encrypt64(138135196),
    );
    expect(res).to.equal(138135196);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 3 (138135196, 138135196)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(138135196),
      this.instances4.alice.encrypt64(138135196),
    );
    expect(res).to.equal(138135196);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 4 (138135196, 138135192)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(138135196),
      this.instances4.alice.encrypt64(138135192),
    );
    expect(res).to.equal(138135196);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (27190055, 77242514)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(27190055),
      this.instances4.alice.encrypt64(77242514),
    );
    expect(res).to.equal(84165557);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (27190051, 27190055)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(27190051),
      this.instances4.alice.encrypt64(27190055),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (27190055, 27190055)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(27190055),
      this.instances4.alice.encrypt64(27190055),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (27190055, 27190051)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(27190055),
      this.instances4.alice.encrypt64(27190051),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 1 (112966432, 124746241)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(112966432),
      this.instances4.alice.encrypt64(124746241),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 2 (112966428, 112966432)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(112966428),
      this.instances4.alice.encrypt64(112966432),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 3 (112966432, 112966432)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(112966432),
      this.instances4.alice.encrypt64(112966432),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 4 (112966432, 112966428)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(112966432),
      this.instances4.alice.encrypt64(112966428),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (216563924, 186359172)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(216563924),
      this.instances4.alice.encrypt64(186359172),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (186359168, 186359172)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(186359168),
      this.instances4.alice.encrypt64(186359172),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (186359172, 186359172)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(186359172),
      this.instances4.alice.encrypt64(186359172),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (186359172, 186359168)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(186359172),
      this.instances4.alice.encrypt64(186359168),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 1 (252617126, 78317021)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(252617126),
      this.instances4.alice.encrypt64(78317021),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 2 (78317017, 78317021)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(78317017),
      this.instances4.alice.encrypt64(78317021),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 3 (78317021, 78317021)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(78317021),
      this.instances4.alice.encrypt64(78317021),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 4 (78317021, 78317017)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(78317021),
      this.instances4.alice.encrypt64(78317017),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 1 (66236212, 254779952)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(66236212),
      this.instances4.alice.encrypt64(254779952),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 2 (66236208, 66236212)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(66236208),
      this.instances4.alice.encrypt64(66236212),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 3 (66236212, 66236212)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(66236212),
      this.instances4.alice.encrypt64(66236212),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 4 (66236212, 66236208)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(66236212),
      this.instances4.alice.encrypt64(66236208),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 1 (106864717, 136047136)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(106864717),
      this.instances4.alice.encrypt64(136047136),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 2 (106864713, 106864717)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(106864713),
      this.instances4.alice.encrypt64(106864717),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 3 (106864717, 106864717)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(106864717),
      this.instances4.alice.encrypt64(106864717),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 4 (106864717, 106864713)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(106864717),
      this.instances4.alice.encrypt64(106864713),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 1 (68403682, 75018920)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(68403682),
      this.instances4.alice.encrypt64(75018920),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 2 (68403678, 68403682)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(68403678),
      this.instances4.alice.encrypt64(68403682),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 3 (68403682, 68403682)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(68403682),
      this.instances4.alice.encrypt64(68403682),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 4 (68403682, 68403678)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(68403682),
      this.instances4.alice.encrypt64(68403678),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 1 (189968743, 122440911)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(189968743),
      this.instances4.alice.encrypt64(122440911),
    );
    expect(res).to.equal(122440911);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 2 (122440907, 122440911)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(122440907),
      this.instances4.alice.encrypt64(122440911),
    );
    expect(res).to.equal(122440907);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 3 (122440911, 122440911)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(122440911),
      this.instances4.alice.encrypt64(122440911),
    );
    expect(res).to.equal(122440911);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 4 (122440911, 122440907)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(122440911),
      this.instances4.alice.encrypt64(122440907),
    );
    expect(res).to.equal(122440907);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (88769233, 236236514)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(88769233),
      this.instances4.alice.encrypt64(236236514),
    );
    expect(res).to.equal(236236514);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (88769229, 88769233)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(88769229),
      this.instances4.alice.encrypt64(88769233),
    );
    expect(res).to.equal(88769233);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (88769233, 88769233)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(88769233),
      this.instances4.alice.encrypt64(88769233),
    );
    expect(res).to.equal(88769233);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (88769233, 88769229)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(88769233),
      this.instances4.alice.encrypt64(88769229),
    );
    expect(res).to.equal(88769233);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (140830493, 37413668)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(140830493), 37413668);
    expect(res).to.equal(178244161n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (140830489, 140830493)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(140830489), 140830493);
    expect(res).to.equal(281660982n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (140830493, 140830493)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(140830493), 140830493);
    expect(res).to.equal(281660986n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (140830493, 140830489)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(140830493), 140830489);
    expect(res).to.equal(281660982n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (88545588, 37413668)', async function () {
    const res = await this.contract4.add_uint32_euint32(88545588, this.instances4.alice.encrypt32(37413668));
    expect(res).to.equal(125959256n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 2 (140830489, 140830493)', async function () {
    const res = await this.contract4.add_uint32_euint32(140830489, this.instances4.alice.encrypt32(140830493));
    expect(res).to.equal(281660982n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 3 (140830493, 140830493)', async function () {
    const res = await this.contract4.add_uint32_euint32(140830493, this.instances4.alice.encrypt32(140830493));
    expect(res).to.equal(281660986n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 4 (140830493, 140830489)', async function () {
    const res = await this.contract4.add_uint32_euint32(140830493, this.instances4.alice.encrypt32(140830489));
    expect(res).to.equal(281660982n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (72155792, 72155792)', async function () {
    const res = await this.contract4.sub_euint32_uint32(this.instances4.alice.encrypt32(72155792), 72155792);
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 2 (72155792, 72155788)', async function () {
    const res = await this.contract4.sub_euint32_uint32(this.instances4.alice.encrypt32(72155792), 72155788);
    expect(res).to.equal(4);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (72155792, 72155792)', async function () {
    const res = await this.contract4.sub_uint32_euint32(72155792, this.instances4.alice.encrypt32(72155792));
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 2 (72155792, 72155788)', async function () {
    const res = await this.contract4.sub_uint32_euint32(72155792, this.instances4.alice.encrypt32(72155788));
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (37388, 46853)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(37388), 46853);
    expect(res).to.equal(1751739964n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 2 (36502, 36502)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(36502), 36502);
    expect(res).to.equal(1332396004n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 3 (36502, 36502)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(36502), 36502);
    expect(res).to.equal(1332396004n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 4 (36502, 36502)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(36502), 36502);
    expect(res).to.equal(1332396004n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (64389, 46853)', async function () {
    const res = await this.contract4.mul_uint32_euint32(64389, this.instances4.alice.encrypt32(46853));
    expect(res).to.equal(3016817817n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (36502, 36502)', async function () {
    const res = await this.contract4.mul_uint32_euint32(36502, this.instances4.alice.encrypt32(36502));
    expect(res).to.equal(1332396004n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (36502, 36502)', async function () {
    const res = await this.contract4.mul_uint32_euint32(36502, this.instances4.alice.encrypt32(36502));
    expect(res).to.equal(1332396004n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (36502, 36502)', async function () {
    const res = await this.contract4.mul_uint32_euint32(36502, this.instances4.alice.encrypt32(36502));
    expect(res).to.equal(1332396004n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (4617026, 124132502)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(4617026), 124132502);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (4617022, 4617026)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(4617022), 4617026);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (4617026, 4617026)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(4617026), 4617026);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (4617026, 4617022)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(4617026), 4617022);
    expect(res).to.equal(1);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (48793720, 87597111)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(48793720), 87597111);
    expect(res).to.equal(48793720);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 2 (48793716, 48793720)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(48793716), 48793720);
    expect(res).to.equal(48793716);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 3 (48793720, 48793720)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(48793720), 48793720);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 4 (48793720, 48793716)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(48793720), 48793716);
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (60042820, 16792460)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(60042820), 16792460);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (60042816, 60042820)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(60042816), 60042820);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 3 (60042820, 60042820)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(60042820), 60042820);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 4 (60042820, 60042816)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(60042820), 60042816);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (112966432, 16792460)', async function () {
    const res = await this.contract4.eq_uint32_euint32(112966432, this.instances4.alice.encrypt32(16792460));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (60042816, 60042820)', async function () {
    const res = await this.contract4.eq_uint32_euint32(60042816, this.instances4.alice.encrypt32(60042820));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 3 (60042820, 60042820)', async function () {
    const res = await this.contract4.eq_uint32_euint32(60042820, this.instances4.alice.encrypt32(60042820));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 4 (60042820, 60042816)', async function () {
    const res = await this.contract4.eq_uint32_euint32(60042820, this.instances4.alice.encrypt32(60042816));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (83574454, 88187509)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(83574454), 88187509);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (51569939, 51569943)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(51569939), 51569943);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 3 (51569943, 51569943)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(51569943), 51569943);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 4 (51569943, 51569939)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(51569943), 51569939);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (216563924, 88187509)', async function () {
    const res = await this.contract4.ne_uint32_euint32(216563924, this.instances4.alice.encrypt32(88187509));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (51569939, 51569943)', async function () {
    const res = await this.contract4.ne_uint32_euint32(51569939, this.instances4.alice.encrypt32(51569943));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 3 (51569943, 51569943)', async function () {
    const res = await this.contract4.ne_uint32_euint32(51569943, this.instances4.alice.encrypt32(51569943));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 4 (51569943, 51569939)', async function () {
    const res = await this.contract4.ne_uint32_euint32(51569943, this.instances4.alice.encrypt32(51569939));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (55218710, 91594514)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(55218710), 91594514);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (55218706, 55218710)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(55218706), 55218710);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (55218710, 55218710)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(55218710), 55218710);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 4 (55218710, 55218706)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(55218710), 55218706);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (252617126, 91594514)', async function () {
    const res = await this.contract4.ge_uint32_euint32(252617126, this.instances4.alice.encrypt32(91594514));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (55218706, 55218710)', async function () {
    const res = await this.contract4.ge_uint32_euint32(55218706, this.instances4.alice.encrypt32(55218710));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (55218710, 55218710)', async function () {
    const res = await this.contract4.ge_uint32_euint32(55218710, this.instances4.alice.encrypt32(55218710));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (55218710, 55218706)', async function () {
    const res = await this.contract4.ge_uint32_euint32(55218710, this.instances4.alice.encrypt32(55218706));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (44051971, 72656620)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(44051971), 72656620);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (44051967, 44051971)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(44051967), 44051971);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (44051971, 44051971)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(44051971), 44051971);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 4 (44051971, 44051967)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(44051971), 44051967);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (66236212, 72656620)', async function () {
    const res = await this.contract4.gt_uint32_euint32(66236212, this.instances4.alice.encrypt32(72656620));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (44051967, 44051971)', async function () {
    const res = await this.contract4.gt_uint32_euint32(44051967, this.instances4.alice.encrypt32(44051971));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (44051971, 44051971)', async function () {
    const res = await this.contract4.gt_uint32_euint32(44051971, this.instances4.alice.encrypt32(44051971));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 4 (44051971, 44051967)', async function () {
    const res = await this.contract4.gt_uint32_euint32(44051971, this.instances4.alice.encrypt32(44051967));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (253532839, 235451435)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(253532839), 235451435);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (253532835, 253532839)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(253532835), 253532839);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (253532839, 253532839)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(253532839), 253532839);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 4 (253532839, 253532835)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(253532839), 253532835);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (106864717, 235451435)', async function () {
    const res = await this.contract4.le_uint32_euint32(106864717, this.instances4.alice.encrypt32(235451435));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (253532835, 253532839)', async function () {
    const res = await this.contract4.le_uint32_euint32(253532835, this.instances4.alice.encrypt32(253532839));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (253532839, 253532839)', async function () {
    const res = await this.contract4.le_uint32_euint32(253532839, this.instances4.alice.encrypt32(253532839));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 4 (253532839, 253532835)', async function () {
    const res = await this.contract4.le_uint32_euint32(253532839, this.instances4.alice.encrypt32(253532835));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (77180163, 83634693)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(77180163), 83634693);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (25179164, 25179168)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(25179164), 25179168);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (25179168, 25179168)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(25179168), 25179168);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 4 (25179168, 25179164)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(25179168), 25179164);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (68403682, 83634693)', async function () {
    const res = await this.contract4.lt_uint32_euint32(68403682, this.instances4.alice.encrypt32(83634693));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (25179164, 25179168)', async function () {
    const res = await this.contract4.lt_uint32_euint32(25179164, this.instances4.alice.encrypt32(25179168));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (25179168, 25179168)', async function () {
    const res = await this.contract4.lt_uint32_euint32(25179168, this.instances4.alice.encrypt32(25179168));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 4 (25179168, 25179164)', async function () {
    const res = await this.contract4.lt_uint32_euint32(25179168, this.instances4.alice.encrypt32(25179164));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (178824069, 197749167)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(178824069), 197749167);
    expect(res).to.equal(178824069);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (78448231, 78448235)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(78448231), 78448235);
    expect(res).to.equal(78448231);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (78448235, 78448235)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(78448235), 78448235);
    expect(res).to.equal(78448235);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 4 (78448235, 78448231)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(78448235), 78448231);
    expect(res).to.equal(78448231);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (189968743, 197749167)', async function () {
    const res = await this.contract4.min_uint32_euint32(189968743, this.instances4.alice.encrypt32(197749167));
    expect(res).to.equal(189968743);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (78448231, 78448235)', async function () {
    const res = await this.contract4.min_uint32_euint32(78448231, this.instances4.alice.encrypt32(78448235));
    expect(res).to.equal(78448231);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (78448235, 78448235)', async function () {
    const res = await this.contract4.min_uint32_euint32(78448235, this.instances4.alice.encrypt32(78448235));
    expect(res).to.equal(78448235);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 4 (78448235, 78448231)', async function () {
    const res = await this.contract4.min_uint32_euint32(78448235, this.instances4.alice.encrypt32(78448231));
    expect(res).to.equal(78448231);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (234155387, 181088842)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(234155387), 181088842);
    expect(res).to.equal(234155387);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (122578466, 122578470)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(122578466), 122578470);
    expect(res).to.equal(122578470);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (122578470, 122578470)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(122578470), 122578470);
    expect(res).to.equal(122578470);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 4 (122578470, 122578466)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(122578470), 122578466);
    expect(res).to.equal(122578470);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (88769233, 181088842)', async function () {
    const res = await this.contract4.max_uint32_euint32(88769233, this.instances4.alice.encrypt32(181088842));
    expect(res).to.equal(181088842);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (122578466, 122578470)', async function () {
    const res = await this.contract4.max_uint32_euint32(122578466, this.instances4.alice.encrypt32(122578470));
    expect(res).to.equal(122578470);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (122578470, 122578470)', async function () {
    const res = await this.contract4.max_uint32_euint32(122578470, this.instances4.alice.encrypt32(122578470));
    expect(res).to.equal(122578470);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 4 (122578470, 122578466)', async function () {
    const res = await this.contract4.max_uint32_euint32(122578470, this.instances4.alice.encrypt32(122578466));
    expect(res).to.equal(122578470);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 1 (10, 1)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt4(1),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 3 (4, 4)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint64, euint4) => euint64 test 1 (10, 10)', async function () {
    const res = await this.contract4.sub_euint64_euint4(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt4(10),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, euint4) => euint64 test 2 (10, 6)', async function () {
    const res = await this.contract4.sub_euint64_euint4(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt4(6),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 1 (10, 1)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt4(1),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 2 (2, 4)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(2),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 3 (2, 2)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(2),
      this.instances4.alice.encrypt4(2),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 4 (4, 2)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(2),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 1 (238356224, 9)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(238356224),
      this.instances4.alice.encrypt4(9),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 2 (5, 9)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(5),
      this.instances4.alice.encrypt4(9),
    );
    expect(res).to.equal(1);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 3 (9, 9)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(9),
      this.instances4.alice.encrypt4(9),
    );
    expect(res).to.equal(9);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 4 (9, 5)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(9),
      this.instances4.alice.encrypt4(5),
    );
    expect(res).to.equal(1);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 1 (50536480, 11)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(50536480),
      this.instances4.alice.encrypt4(11),
    );
    expect(res).to.equal(50536491);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 2 (7, 11)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(7),
      this.instances4.alice.encrypt4(11),
    );
    expect(res).to.equal(15);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 3 (11, 11)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(11),
      this.instances4.alice.encrypt4(11),
    );
    expect(res).to.equal(11);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 4 (11, 7)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(11),
      this.instances4.alice.encrypt4(7),
    );
    expect(res).to.equal(15);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 1 (147215301, 1)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(147215301),
      this.instances4.alice.encrypt4(1),
    );
    expect(res).to.equal(147215300);
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

  it('test operator "eq" overload (euint64, euint4) => ebool test 1 (113183278, 7)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(113183278),
      this.instances4.alice.encrypt4(7),
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

  it('test operator "ne" overload (euint64, euint4) => ebool test 1 (129256608, 13)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(129256608),
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

  it('test operator "ge" overload (euint64, euint4) => ebool test 1 (116051290, 10)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(116051290),
      this.instances4.alice.encrypt4(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(6),
      this.instances4.alice.encrypt4(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt4(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt4(6),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 1 (208318592, 6)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(208318592),
      this.instances4.alice.encrypt4(6),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 1 (217701220, 1)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(217701220),
      this.instances4.alice.encrypt4(1),
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

  it('test operator "lt" overload (euint64, euint4) => ebool test 1 (264374974, 2)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(264374974),
      this.instances4.alice.encrypt4(2),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 1 (67905839, 8)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(67905839),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(8);
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

  it('test operator "max" overload (euint64, euint4) => euint64 test 1 (215341582, 6)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(215341582),
      this.instances4.alice.encrypt4(6),
    );
    expect(res).to.equal(215341582);
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

  it('test operator "add" overload (euint64, euint8) => euint64 test 1 (162, 1)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(162),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(163n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 2 (109, 111)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(109),
      this.instances4.alice.encrypt8(111),
    );
    expect(res).to.equal(220n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 3 (111, 111)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(111),
      this.instances4.alice.encrypt8(111),
    );
    expect(res).to.equal(222n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 4 (111, 109)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(111),
      this.instances4.alice.encrypt8(109),
    );
    expect(res).to.equal(220n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 1 (240, 240)', async function () {
    const res = await this.contract4.sub_euint64_euint8(
      this.instances4.alice.encrypt64(240),
      this.instances4.alice.encrypt8(240),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 2 (240, 236)', async function () {
    const res = await this.contract4.sub_euint64_euint8(
      this.instances4.alice.encrypt64(240),
      this.instances4.alice.encrypt8(236),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 1 (175, 1)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(175),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(175n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 2 (10, 10)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt8(10),
    );
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 3 (10, 10)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt8(10),
    );
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 4 (10, 10)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt8(10),
    );
    expect(res).to.equal(100n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 1 (238356224, 211)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(238356224),
      this.instances4.alice.encrypt8(211),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 2 (207, 211)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(207),
      this.instances4.alice.encrypt8(211),
    );
    expect(res).to.equal(195);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 3 (211, 211)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(211),
      this.instances4.alice.encrypt8(211),
    );
    expect(res).to.equal(211);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 4 (211, 207)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(211),
      this.instances4.alice.encrypt8(207),
    );
    expect(res).to.equal(195);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 1 (50536480, 64)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(50536480),
      this.instances4.alice.encrypt8(64),
    );
    expect(res).to.equal(50536544);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 2 (60, 64)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(60),
      this.instances4.alice.encrypt8(64),
    );
    expect(res).to.equal(124);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 3 (64, 64)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(64),
      this.instances4.alice.encrypt8(64),
    );
    expect(res).to.equal(64);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 4 (64, 60)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(64),
      this.instances4.alice.encrypt8(60),
    );
    expect(res).to.equal(124);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 1 (147215301, 174)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(147215301),
      this.instances4.alice.encrypt8(174),
    );
    expect(res).to.equal(147215211);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 2 (170, 174)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(170),
      this.instances4.alice.encrypt8(174),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 3 (174, 174)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(174),
      this.instances4.alice.encrypt8(174),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 4 (174, 170)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(174),
      this.instances4.alice.encrypt8(170),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 1 (113183278, 90)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(113183278),
      this.instances4.alice.encrypt8(90),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 2 (86, 90)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(86),
      this.instances4.alice.encrypt8(90),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 3 (90, 90)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(90),
      this.instances4.alice.encrypt8(90),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 4 (90, 86)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(90),
      this.instances4.alice.encrypt8(86),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 1 (129256608, 230)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(129256608),
      this.instances4.alice.encrypt8(230),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 2 (226, 230)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(226),
      this.instances4.alice.encrypt8(230),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 3 (230, 230)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(230),
      this.instances4.alice.encrypt8(230),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 4 (230, 226)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(230),
      this.instances4.alice.encrypt8(226),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 1 (116051290, 252)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(116051290),
      this.instances4.alice.encrypt8(252),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 2 (248, 252)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(248),
      this.instances4.alice.encrypt8(252),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 3 (252, 252)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(252),
      this.instances4.alice.encrypt8(252),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 4 (252, 248)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(252),
      this.instances4.alice.encrypt8(248),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 1 (208318592, 189)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(208318592),
      this.instances4.alice.encrypt8(189),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 2 (185, 189)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(185),
      this.instances4.alice.encrypt8(189),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 3 (189, 189)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(189),
      this.instances4.alice.encrypt8(189),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 4 (189, 185)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(189),
      this.instances4.alice.encrypt8(185),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 1 (217701220, 215)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(217701220),
      this.instances5.alice.encrypt8(215),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 2 (211, 215)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(211),
      this.instances5.alice.encrypt8(215),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 3 (215, 215)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(215),
      this.instances5.alice.encrypt8(215),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 4 (215, 211)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(215),
      this.instances5.alice.encrypt8(211),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 1 (264374974, 197)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(264374974),
      this.instances5.alice.encrypt8(197),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 2 (193, 197)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(193),
      this.instances5.alice.encrypt8(197),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 3 (197, 197)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(197),
      this.instances5.alice.encrypt8(197),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 4 (197, 193)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(197),
      this.instances5.alice.encrypt8(193),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 1 (67905839, 86)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(67905839),
      this.instances5.alice.encrypt8(86),
    );
    expect(res).to.equal(86);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 2 (82, 86)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(82),
      this.instances5.alice.encrypt8(86),
    );
    expect(res).to.equal(82);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 3 (86, 86)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(86),
      this.instances5.alice.encrypt8(86),
    );
    expect(res).to.equal(86);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 4 (86, 82)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(86),
      this.instances5.alice.encrypt8(82),
    );
    expect(res).to.equal(82);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 1 (215341582, 146)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(215341582),
      this.instances5.alice.encrypt8(146),
    );
    expect(res).to.equal(215341582);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 2 (142, 146)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(142),
      this.instances5.alice.encrypt8(146),
    );
    expect(res).to.equal(146);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 3 (146, 146)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(146),
      this.instances5.alice.encrypt8(146),
    );
    expect(res).to.equal(146);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 4 (146, 142)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(146),
      this.instances5.alice.encrypt8(142),
    );
    expect(res).to.equal(146);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 1 (41532, 11)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(41532),
      this.instances5.alice.encrypt16(11),
    );
    expect(res).to.equal(41543n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 2 (23875, 23877)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(23875),
      this.instances5.alice.encrypt16(23877),
    );
    expect(res).to.equal(47752n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 3 (23877, 23877)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(23877),
      this.instances5.alice.encrypt16(23877),
    );
    expect(res).to.equal(47754n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 4 (23877, 23875)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(23877),
      this.instances5.alice.encrypt16(23875),
    );
    expect(res).to.equal(47752n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 1 (19514, 19514)', async function () {
    const res = await this.contract5.sub_euint64_euint16(
      this.instances5.alice.encrypt64(19514),
      this.instances5.alice.encrypt16(19514),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 2 (19514, 19510)', async function () {
    const res = await this.contract5.sub_euint64_euint16(
      this.instances5.alice.encrypt64(19514),
      this.instances5.alice.encrypt16(19510),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 1 (11257, 5)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(11257),
      this.instances5.alice.encrypt16(5),
    );
    expect(res).to.equal(56285n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 2 (165, 165)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(165),
      this.instances5.alice.encrypt16(165),
    );
    expect(res).to.equal(27225n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 3 (165, 165)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(165),
      this.instances5.alice.encrypt16(165),
    );
    expect(res).to.equal(27225n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 4 (165, 165)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(165),
      this.instances5.alice.encrypt16(165),
    );
    expect(res).to.equal(27225n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 1 (238356224, 18546)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(238356224),
      this.instances5.alice.encrypt16(18546),
    );
    expect(res).to.equal(0);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 2 (18542, 18546)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(18542),
      this.instances5.alice.encrypt16(18546),
    );
    expect(res).to.equal(18530);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 3 (18546, 18546)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(18546),
      this.instances5.alice.encrypt16(18546),
    );
    expect(res).to.equal(18546);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 4 (18546, 18542)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(18546),
      this.instances5.alice.encrypt16(18542),
    );
    expect(res).to.equal(18530);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 1 (50536480, 3556)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(50536480),
      this.instances5.alice.encrypt16(3556),
    );
    expect(res).to.equal(50540004);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 2 (3552, 3556)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(3552),
      this.instances5.alice.encrypt16(3556),
    );
    expect(res).to.equal(3556);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 3 (3556, 3556)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(3556),
      this.instances5.alice.encrypt16(3556),
    );
    expect(res).to.equal(3556);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 4 (3556, 3552)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(3556),
      this.instances5.alice.encrypt16(3552),
    );
    expect(res).to.equal(3556);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 1 (147215301, 35671)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(147215301),
      this.instances5.alice.encrypt16(35671),
    );
    expect(res).to.equal(147249298);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 2 (35667, 35671)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(35667),
      this.instances5.alice.encrypt16(35671),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 3 (35671, 35671)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(35671),
      this.instances5.alice.encrypt16(35671),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 4 (35671, 35667)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(35671),
      this.instances5.alice.encrypt16(35667),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 1 (113183278, 17213)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(113183278),
      this.instances5.alice.encrypt16(17213),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 2 (17209, 17213)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(17209),
      this.instances5.alice.encrypt16(17213),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 3 (17213, 17213)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(17213),
      this.instances5.alice.encrypt16(17213),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 4 (17213, 17209)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(17213),
      this.instances5.alice.encrypt16(17209),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 1 (129256608, 25758)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(129256608),
      this.instances5.alice.encrypt16(25758),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 2 (25754, 25758)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(25754),
      this.instances5.alice.encrypt16(25758),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 3 (25758, 25758)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(25758),
      this.instances5.alice.encrypt16(25758),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 4 (25758, 25754)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(25758),
      this.instances5.alice.encrypt16(25754),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 1 (116051290, 2338)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(116051290),
      this.instances5.alice.encrypt16(2338),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 2 (2334, 2338)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(2334),
      this.instances5.alice.encrypt16(2338),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 3 (2338, 2338)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(2338),
      this.instances5.alice.encrypt16(2338),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 4 (2338, 2334)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(2338),
      this.instances5.alice.encrypt16(2334),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 1 (208318592, 48949)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(208318592),
      this.instances5.alice.encrypt16(48949),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 2 (48945, 48949)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(48945),
      this.instances5.alice.encrypt16(48949),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 3 (48949, 48949)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(48949),
      this.instances5.alice.encrypt16(48949),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 4 (48949, 48945)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(48949),
      this.instances5.alice.encrypt16(48945),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 1 (217701220, 49721)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(217701220),
      this.instances5.alice.encrypt16(49721),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 2 (49717, 49721)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(49717),
      this.instances5.alice.encrypt16(49721),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 3 (49721, 49721)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(49721),
      this.instances5.alice.encrypt16(49721),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 4 (49721, 49717)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(49721),
      this.instances5.alice.encrypt16(49717),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 1 (264374974, 23947)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(264374974),
      this.instances5.alice.encrypt16(23947),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 2 (23943, 23947)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(23943),
      this.instances5.alice.encrypt16(23947),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 3 (23947, 23947)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(23947),
      this.instances5.alice.encrypt16(23947),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 4 (23947, 23943)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(23947),
      this.instances5.alice.encrypt16(23943),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 1 (67905839, 62663)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(67905839),
      this.instances5.alice.encrypt16(62663),
    );
    expect(res).to.equal(62663);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 2 (62659, 62663)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(62659),
      this.instances5.alice.encrypt16(62663),
    );
    expect(res).to.equal(62659);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 3 (62663, 62663)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(62663),
      this.instances5.alice.encrypt16(62663),
    );
    expect(res).to.equal(62663);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 4 (62663, 62659)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(62663),
      this.instances5.alice.encrypt16(62659),
    );
    expect(res).to.equal(62659);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (215341582, 59093)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(215341582),
      this.instances5.alice.encrypt16(59093),
    );
    expect(res).to.equal(215341582);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (59089, 59093)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(59089),
      this.instances5.alice.encrypt16(59093),
    );
    expect(res).to.equal(59093);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (59093, 59093)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(59093),
      this.instances5.alice.encrypt16(59093),
    );
    expect(res).to.equal(59093);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (59093, 59089)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(59093),
      this.instances5.alice.encrypt16(59089),
    );
    expect(res).to.equal(59093);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 1 (170118033, 266084851)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(170118033),
      this.instances5.alice.encrypt32(266084851),
    );
    expect(res).to.equal(436202884n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 2 (170118029, 170118033)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(170118029),
      this.instances5.alice.encrypt32(170118033),
    );
    expect(res).to.equal(340236062n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 3 (170118033, 170118033)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(170118033),
      this.instances5.alice.encrypt32(170118033),
    );
    expect(res).to.equal(340236066n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 4 (170118033, 170118029)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(170118033),
      this.instances5.alice.encrypt32(170118029),
    );
    expect(res).to.equal(340236062n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 1 (69685503, 69685503)', async function () {
    const res = await this.contract5.sub_euint64_euint32(
      this.instances5.alice.encrypt64(69685503),
      this.instances5.alice.encrypt32(69685503),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 2 (69685503, 69685499)', async function () {
    const res = await this.contract5.sub_euint64_euint32(
      this.instances5.alice.encrypt64(69685503),
      this.instances5.alice.encrypt32(69685499),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 1 (45030, 88250)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(45030),
      this.instances5.alice.encrypt32(88250),
    );
    expect(res).to.equal(3973897500n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 2 (45030, 45030)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(45030),
      this.instances5.alice.encrypt32(45030),
    );
    expect(res).to.equal(2027700900n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 3 (45030, 45030)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(45030),
      this.instances5.alice.encrypt32(45030),
    );
    expect(res).to.equal(2027700900n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 4 (45030, 45030)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(45030),
      this.instances5.alice.encrypt32(45030),
    );
    expect(res).to.equal(2027700900n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 1 (238356224, 212953386)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(238356224),
      this.instances5.alice.encrypt32(212953386),
    );
    expect(res).to.equal(204538112);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 2 (212953382, 212953386)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(212953382),
      this.instances5.alice.encrypt32(212953386),
    );
    expect(res).to.equal(212953378);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 3 (212953386, 212953386)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(212953386),
      this.instances5.alice.encrypt32(212953386),
    );
    expect(res).to.equal(212953386);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 4 (212953386, 212953382)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(212953386),
      this.instances5.alice.encrypt32(212953382),
    );
    expect(res).to.equal(212953378);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 1 (50536480, 16913643)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(50536480),
      this.instances5.alice.encrypt32(16913643),
    );
    expect(res).to.equal(50541803);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 2 (16913639, 16913643)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(16913639),
      this.instances5.alice.encrypt32(16913643),
    );
    expect(res).to.equal(16913647);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 3 (16913643, 16913643)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(16913643),
      this.instances5.alice.encrypt32(16913643),
    );
    expect(res).to.equal(16913643);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 4 (16913643, 16913639)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(16913643),
      this.instances5.alice.encrypt32(16913639),
    );
    expect(res).to.equal(16913647);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 1 (147215301, 173065039)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(147215301),
      this.instances5.alice.encrypt32(173065039),
    );
    expect(res).to.equal(43421834);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 2 (147215297, 147215301)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(147215297),
      this.instances5.alice.encrypt32(147215301),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 3 (147215301, 147215301)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(147215301),
      this.instances5.alice.encrypt32(147215301),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 4 (147215301, 147215297)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(147215301),
      this.instances5.alice.encrypt32(147215297),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 1 (113183278, 255380188)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(113183278),
      this.instances5.alice.encrypt32(255380188),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 2 (113183274, 113183278)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(113183274),
      this.instances5.alice.encrypt32(113183278),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 3 (113183278, 113183278)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(113183278),
      this.instances5.alice.encrypt32(113183278),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 4 (113183278, 113183274)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(113183278),
      this.instances5.alice.encrypt32(113183274),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 1 (129256608, 191248966)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(129256608),
      this.instances5.alice.encrypt32(191248966),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 2 (129256604, 129256608)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(129256604),
      this.instances5.alice.encrypt32(129256608),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 3 (129256608, 129256608)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(129256608),
      this.instances5.alice.encrypt32(129256608),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 4 (129256608, 129256604)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(129256608),
      this.instances5.alice.encrypt32(129256604),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 1 (116051290, 95719061)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(116051290),
      this.instances5.alice.encrypt32(95719061),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 2 (95719057, 95719061)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(95719057),
      this.instances5.alice.encrypt32(95719061),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 3 (95719061, 95719061)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(95719061),
      this.instances5.alice.encrypt32(95719061),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 4 (95719061, 95719057)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(95719061),
      this.instances5.alice.encrypt32(95719057),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 1 (208318592, 245955324)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(208318592),
      this.instances5.alice.encrypt32(245955324),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 2 (208318588, 208318592)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(208318588),
      this.instances5.alice.encrypt32(208318592),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 3 (208318592, 208318592)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(208318592),
      this.instances5.alice.encrypt32(208318592),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 4 (208318592, 208318588)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(208318592),
      this.instances5.alice.encrypt32(208318588),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (217701220, 133414572)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(217701220),
      this.instances5.alice.encrypt32(133414572),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (133414568, 133414572)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(133414568),
      this.instances5.alice.encrypt32(133414572),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (133414572, 133414572)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(133414572),
      this.instances5.alice.encrypt32(133414572),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (133414572, 133414568)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(133414572),
      this.instances5.alice.encrypt32(133414568),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (264374974, 253580652)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(264374974),
      this.instances5.alice.encrypt32(253580652),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (253580648, 253580652)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(253580648),
      this.instances5.alice.encrypt32(253580652),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (253580652, 253580652)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(253580652),
      this.instances5.alice.encrypt32(253580652),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (253580652, 253580648)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(253580652),
      this.instances5.alice.encrypt32(253580648),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (67905839, 157717094)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(67905839),
      this.instances5.alice.encrypt32(157717094),
    );
    expect(res).to.equal(67905839);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (67905835, 67905839)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(67905835),
      this.instances5.alice.encrypt32(67905839),
    );
    expect(res).to.equal(67905835);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (67905839, 67905839)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(67905839),
      this.instances5.alice.encrypt32(67905839),
    );
    expect(res).to.equal(67905839);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (67905839, 67905835)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(67905839),
      this.instances5.alice.encrypt32(67905835),
    );
    expect(res).to.equal(67905835);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (215341582, 62329989)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(215341582),
      this.instances5.alice.encrypt32(62329989),
    );
    expect(res).to.equal(215341582);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (62329985, 62329989)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(62329985),
      this.instances5.alice.encrypt32(62329989),
    );
    expect(res).to.equal(62329989);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (62329989, 62329989)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(62329989),
      this.instances5.alice.encrypt32(62329989),
    );
    expect(res).to.equal(62329989);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (62329989, 62329985)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(62329989),
      this.instances5.alice.encrypt32(62329985),
    );
    expect(res).to.equal(62329989);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (170118033, 75374623)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(170118033),
      this.instances5.alice.encrypt64(75374623),
    );
    expect(res).to.equal(245492656n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (75374619, 75374623)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(75374619),
      this.instances5.alice.encrypt64(75374623),
    );
    expect(res).to.equal(150749242n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (75374623, 75374623)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(75374623),
      this.instances5.alice.encrypt64(75374623),
    );
    expect(res).to.equal(150749246n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (75374623, 75374619)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(75374623),
      this.instances5.alice.encrypt64(75374619),
    );
    expect(res).to.equal(150749242n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (65505221, 65505221)', async function () {
    const res = await this.contract5.sub_euint64_euint64(
      this.instances5.alice.encrypt64(65505221),
      this.instances5.alice.encrypt64(65505221),
    );
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (65505221, 65505217)', async function () {
    const res = await this.contract5.sub_euint64_euint64(
      this.instances5.alice.encrypt64(65505221),
      this.instances5.alice.encrypt64(65505217),
    );
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (92221761, 267518717)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(92221761),
      this.instances5.alice.encrypt64(267518717),
    );
    expect(res).to.equal(24671047182200637n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (92221757, 92221761)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(92221757),
      this.instances5.alice.encrypt64(92221761),
    );
    expect(res).to.equal(8504852833054077n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (92221761, 92221761)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(92221761),
      this.instances5.alice.encrypt64(92221761),
    );
    expect(res).to.equal(8504853201941121n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (92221761, 92221757)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(92221761),
      this.instances5.alice.encrypt64(92221757),
    );
    expect(res).to.equal(8504852833054077n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (238356224, 13141105)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(238356224),
      this.instances5.alice.encrypt64(13141105),
    );
    expect(res).to.equal(1024);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (13141101, 13141105)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(13141101),
      this.instances5.alice.encrypt64(13141105),
    );
    expect(res).to.equal(13141089);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (13141105, 13141105)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(13141105),
      this.instances5.alice.encrypt64(13141105),
    );
    expect(res).to.equal(13141105);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (13141105, 13141101)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(13141105),
      this.instances5.alice.encrypt64(13141101),
    );
    expect(res).to.equal(13141089);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 1 (50536480, 244448701)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(50536480),
      this.instances5.alice.encrypt64(244448701),
    );
    expect(res).to.equal(261356989);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 2 (50536476, 50536480)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(50536476),
      this.instances5.alice.encrypt64(50536480),
    );
    expect(res).to.equal(50536508);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 3 (50536480, 50536480)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(50536480),
      this.instances5.alice.encrypt64(50536480),
    );
    expect(res).to.equal(50536480);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 4 (50536480, 50536476)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(50536480),
      this.instances5.alice.encrypt64(50536476),
    );
    expect(res).to.equal(50536508);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 1 (147215301, 32869222)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(147215301),
      this.instances5.alice.encrypt64(32869222),
    );
    expect(res).to.equal(154392739);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 2 (32869218, 32869222)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(32869218),
      this.instances5.alice.encrypt64(32869222),
    );
    expect(res).to.equal(4);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 3 (32869222, 32869222)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(32869222),
      this.instances5.alice.encrypt64(32869222),
    );
    expect(res).to.equal(0);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 4 (32869222, 32869218)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(32869222),
      this.instances5.alice.encrypt64(32869218),
    );
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 1 (113183278, 1631862)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(113183278),
      this.instances5.alice.encrypt64(1631862),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 2 (1631858, 1631862)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(1631858),
      this.instances5.alice.encrypt64(1631862),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 3 (1631862, 1631862)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(1631862),
      this.instances5.alice.encrypt64(1631862),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 4 (1631862, 1631858)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(1631862),
      this.instances5.alice.encrypt64(1631858),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (129256608, 198214797)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(129256608),
      this.instances5.alice.encrypt64(198214797),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (129256604, 129256608)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(129256604),
      this.instances5.alice.encrypt64(129256608),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (129256608, 129256608)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(129256608),
      this.instances5.alice.encrypt64(129256608),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (129256608, 129256604)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(129256608),
      this.instances5.alice.encrypt64(129256604),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 1 (116051290, 122147524)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(116051290),
      this.instances5.alice.encrypt64(122147524),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 2 (116051286, 116051290)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(116051286),
      this.instances5.alice.encrypt64(116051290),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 3 (116051290, 116051290)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(116051290),
      this.instances5.alice.encrypt64(116051290),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 4 (116051290, 116051286)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(116051290),
      this.instances5.alice.encrypt64(116051286),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (208318592, 117484228)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(208318592),
      this.instances5.alice.encrypt64(117484228),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (117484224, 117484228)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(117484224),
      this.instances5.alice.encrypt64(117484228),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (117484228, 117484228)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(117484228),
      this.instances5.alice.encrypt64(117484228),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (117484228, 117484224)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(117484228),
      this.instances5.alice.encrypt64(117484224),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (217701220, 184213537)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(217701220),
      this.instances5.alice.encrypt64(184213537),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (184213533, 184213537)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(184213533),
      this.instances5.alice.encrypt64(184213537),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (184213537, 184213537)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(184213537),
      this.instances5.alice.encrypt64(184213537),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (184213537, 184213533)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(184213537),
      this.instances5.alice.encrypt64(184213533),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 1 (264374974, 138721892)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(264374974),
      this.instances5.alice.encrypt64(138721892),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 2 (138721888, 138721892)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(138721888),
      this.instances5.alice.encrypt64(138721892),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 3 (138721892, 138721892)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(138721892),
      this.instances5.alice.encrypt64(138721892),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 4 (138721892, 138721888)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(138721892),
      this.instances5.alice.encrypt64(138721888),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 1 (67905839, 52961865)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(67905839),
      this.instances5.alice.encrypt64(52961865),
    );
    expect(res).to.equal(52961865);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 2 (52961861, 52961865)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(52961861),
      this.instances5.alice.encrypt64(52961865),
    );
    expect(res).to.equal(52961861);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 3 (52961865, 52961865)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(52961865),
      this.instances5.alice.encrypt64(52961865),
    );
    expect(res).to.equal(52961865);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 4 (52961865, 52961861)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(52961865),
      this.instances5.alice.encrypt64(52961861),
    );
    expect(res).to.equal(52961861);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (215341582, 39049834)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(215341582),
      this.instances5.alice.encrypt64(39049834),
    );
    expect(res).to.equal(215341582);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (39049830, 39049834)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(39049830),
      this.instances5.alice.encrypt64(39049834),
    );
    expect(res).to.equal(39049834);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (39049834, 39049834)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(39049834),
      this.instances5.alice.encrypt64(39049834),
    );
    expect(res).to.equal(39049834);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (39049834, 39049830)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(39049834),
      this.instances5.alice.encrypt64(39049830),
    );
    expect(res).to.equal(39049834);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (170118033, 60441680)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(170118033), 60441680);
    expect(res).to.equal(230559713n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (75374619, 75374623)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(75374619), 75374623);
    expect(res).to.equal(150749242n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (75374623, 75374623)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(75374623), 75374623);
    expect(res).to.equal(150749246n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (75374623, 75374619)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(75374623), 75374619);
    expect(res).to.equal(150749242n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 1 (187809144, 60441680)', async function () {
    const res = await this.contract5.add_uint64_euint64(187809144, this.instances5.alice.encrypt64(60441680));
    expect(res).to.equal(248250824n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 2 (75374619, 75374623)', async function () {
    const res = await this.contract5.add_uint64_euint64(75374619, this.instances5.alice.encrypt64(75374623));
    expect(res).to.equal(150749242n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 3 (75374623, 75374623)', async function () {
    const res = await this.contract5.add_uint64_euint64(75374623, this.instances5.alice.encrypt64(75374623));
    expect(res).to.equal(150749246n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 4 (75374623, 75374619)', async function () {
    const res = await this.contract5.add_uint64_euint64(75374623, this.instances5.alice.encrypt64(75374619));
    expect(res).to.equal(150749242n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (65505221, 65505221)', async function () {
    const res = await this.contract5.sub_euint64_uint64(this.instances5.alice.encrypt64(65505221), 65505221);
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (65505221, 65505217)', async function () {
    const res = await this.contract5.sub_euint64_uint64(this.instances5.alice.encrypt64(65505221), 65505217);
    expect(res).to.equal(4);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (65505221, 65505221)', async function () {
    const res = await this.contract5.sub_uint64_euint64(65505221, this.instances5.alice.encrypt64(65505221));
    expect(res).to.equal(0);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (65505221, 65505217)', async function () {
    const res = await this.contract5.sub_uint64_euint64(65505221, this.instances5.alice.encrypt64(65505217));
    expect(res).to.equal(4);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (92221761, 195753020)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(92221761), 195753020);
    expect(res).to.equal(18052688225468220n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (92221757, 92221761)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(92221757), 92221761);
    expect(res).to.equal(8504852833054077n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (92221761, 92221761)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(92221761), 92221761);
    expect(res).to.equal(8504853201941121n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (92221761, 92221757)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(92221761), 92221757);
    expect(res).to.equal(8504852833054077n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 1 (213768336, 195753020)', async function () {
    const res = await this.contract5.mul_uint64_euint64(213768336, this.instances5.alice.encrypt64(195753020));
    expect(res).to.equal(41845797352374720n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 2 (92221757, 92221761)', async function () {
    const res = await this.contract5.mul_uint64_euint64(92221757, this.instances5.alice.encrypt64(92221761));
    expect(res).to.equal(8504852833054077n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 3 (92221761, 92221761)', async function () {
    const res = await this.contract5.mul_uint64_euint64(92221761, this.instances5.alice.encrypt64(92221761));
    expect(res).to.equal(8504853201941121n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 4 (92221761, 92221757)', async function () {
    const res = await this.contract5.mul_uint64_euint64(92221761, this.instances5.alice.encrypt64(92221757));
    expect(res).to.equal(8504852833054077n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (163939989, 238706096)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(163939989), 238706096);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (98898938, 98898942)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(98898938), 98898942);
    expect(res).to.equal(0);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (98898942, 98898942)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(98898942), 98898942);
    expect(res).to.equal(1);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (98898942, 98898938)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(98898942), 98898938);
    expect(res).to.equal(1);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 1 (81522034, 93915045)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(81522034), 93915045);
    expect(res).to.equal(81522034);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 2 (81522030, 81522034)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(81522030), 81522034);
    expect(res).to.equal(81522030);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 3 (81522034, 81522034)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(81522034), 81522034);
    expect(res).to.equal(0);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 4 (81522034, 81522030)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(81522034), 81522030);
    expect(res).to.equal(4);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (113183278, 36833882)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(113183278), 36833882);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (1631858, 1631862)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(1631858), 1631862);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (1631862, 1631862)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(1631862), 1631862);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (1631862, 1631858)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(1631862), 1631858);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (95843694, 36833882)', async function () {
    const res = await this.contract5.eq_uint64_euint64(95843694, this.instances5.alice.encrypt64(36833882));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (1631858, 1631862)', async function () {
    const res = await this.contract5.eq_uint64_euint64(1631858, this.instances5.alice.encrypt64(1631862));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (1631862, 1631862)', async function () {
    const res = await this.contract5.eq_uint64_euint64(1631862, this.instances5.alice.encrypt64(1631862));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (1631862, 1631858)', async function () {
    const res = await this.contract5.eq_uint64_euint64(1631862, this.instances5.alice.encrypt64(1631858));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (129256608, 155795571)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(129256608), 155795571);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (129256604, 129256608)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(129256604), 129256608);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (129256608, 129256608)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(129256608), 129256608);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (129256608, 129256604)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(129256608), 129256604);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (38125121, 155795571)', async function () {
    const res = await this.contract5.ne_uint64_euint64(38125121, this.instances5.alice.encrypt64(155795571));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (129256604, 129256608)', async function () {
    const res = await this.contract5.ne_uint64_euint64(129256604, this.instances5.alice.encrypt64(129256608));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (129256608, 129256608)', async function () {
    const res = await this.contract5.ne_uint64_euint64(129256608, this.instances5.alice.encrypt64(129256608));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (129256608, 129256604)', async function () {
    const res = await this.contract5.ne_uint64_euint64(129256608, this.instances5.alice.encrypt64(129256604));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (116051290, 135280853)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(116051290), 135280853);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (116051286, 116051290)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(116051286), 116051290);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (116051290, 116051290)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(116051290), 116051290);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (116051290, 116051286)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(116051290), 116051286);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (182502154, 135280853)', async function () {
    const res = await this.contract5.ge_uint64_euint64(182502154, this.instances5.alice.encrypt64(135280853));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (116051286, 116051290)', async function () {
    const res = await this.contract5.ge_uint64_euint64(116051286, this.instances5.alice.encrypt64(116051290));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (116051290, 116051290)', async function () {
    const res = await this.contract5.ge_uint64_euint64(116051290, this.instances5.alice.encrypt64(116051290));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (116051290, 116051286)', async function () {
    const res = await this.contract5.ge_uint64_euint64(116051290, this.instances5.alice.encrypt64(116051286));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (208318592, 117293406)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(208318592), 117293406);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (117484224, 117484228)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(117484224), 117484228);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (117484228, 117484228)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(117484228), 117484228);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (117484228, 117484224)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(117484228), 117484224);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 1 (29790592, 117293406)', async function () {
    const res = await this.contract5.gt_uint64_euint64(29790592, this.instances5.alice.encrypt64(117293406));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 2 (117484224, 117484228)', async function () {
    const res = await this.contract5.gt_uint64_euint64(117484224, this.instances5.alice.encrypt64(117484228));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 3 (117484228, 117484228)', async function () {
    const res = await this.contract5.gt_uint64_euint64(117484228, this.instances5.alice.encrypt64(117484228));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 4 (117484228, 117484224)', async function () {
    const res = await this.contract5.gt_uint64_euint64(117484228, this.instances5.alice.encrypt64(117484224));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (217701220, 169995885)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(217701220), 169995885);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (184213533, 184213537)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(184213533), 184213537);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (184213537, 184213537)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(184213537), 184213537);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (184213537, 184213533)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(184213537), 184213533);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (133029453, 169995885)', async function () {
    const res = await this.contract5.le_uint64_euint64(133029453, this.instances5.alice.encrypt64(169995885));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (184213533, 184213537)', async function () {
    const res = await this.contract5.le_uint64_euint64(184213533, this.instances5.alice.encrypt64(184213537));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (184213537, 184213537)', async function () {
    const res = await this.contract5.le_uint64_euint64(184213537, this.instances5.alice.encrypt64(184213537));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (184213537, 184213533)', async function () {
    const res = await this.contract5.le_uint64_euint64(184213537, this.instances5.alice.encrypt64(184213533));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (264374974, 177602426)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(264374974), 177602426);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (138721888, 138721892)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(138721888), 138721892);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (138721892, 138721892)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(138721892), 138721892);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (138721892, 138721888)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(138721892), 138721888);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (220331487, 177602426)', async function () {
    const res = await this.contract5.lt_uint64_euint64(220331487, this.instances5.alice.encrypt64(177602426));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (138721888, 138721892)', async function () {
    const res = await this.contract5.lt_uint64_euint64(138721888, this.instances5.alice.encrypt64(138721892));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (138721892, 138721892)', async function () {
    const res = await this.contract5.lt_uint64_euint64(138721892, this.instances5.alice.encrypt64(138721892));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (138721892, 138721888)', async function () {
    const res = await this.contract5.lt_uint64_euint64(138721892, this.instances5.alice.encrypt64(138721888));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 1 (67905839, 116349816)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(67905839), 116349816);
    expect(res).to.equal(67905839);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 2 (52961861, 52961865)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(52961861), 52961865);
    expect(res).to.equal(52961861);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 3 (52961865, 52961865)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(52961865), 52961865);
    expect(res).to.equal(52961865);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 4 (52961865, 52961861)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(52961865), 52961861);
    expect(res).to.equal(52961861);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (78138466, 116349816)', async function () {
    const res = await this.contract5.min_uint64_euint64(78138466, this.instances5.alice.encrypt64(116349816));
    expect(res).to.equal(78138466);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (52961861, 52961865)', async function () {
    const res = await this.contract5.min_uint64_euint64(52961861, this.instances5.alice.encrypt64(52961865));
    expect(res).to.equal(52961861);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (52961865, 52961865)', async function () {
    const res = await this.contract5.min_uint64_euint64(52961865, this.instances5.alice.encrypt64(52961865));
    expect(res).to.equal(52961865);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (52961865, 52961861)', async function () {
    const res = await this.contract5.min_uint64_euint64(52961865, this.instances5.alice.encrypt64(52961861));
    expect(res).to.equal(52961861);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (215341582, 64923876)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(215341582), 64923876);
    expect(res).to.equal(215341582);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (39049830, 39049834)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(39049830), 39049834);
    expect(res).to.equal(39049834);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (39049834, 39049834)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(39049834), 39049834);
    expect(res).to.equal(39049834);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (39049834, 39049830)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(39049834), 39049830);
    expect(res).to.equal(39049834);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (174351991, 64923876)', async function () {
    const res = await this.contract5.max_uint64_euint64(174351991, this.instances5.alice.encrypt64(64923876));
    expect(res).to.equal(174351991);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (39049830, 39049834)', async function () {
    const res = await this.contract5.max_uint64_euint64(39049830, this.instances5.alice.encrypt64(39049834));
    expect(res).to.equal(39049834);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (39049834, 39049834)', async function () {
    const res = await this.contract5.max_uint64_euint64(39049834, this.instances5.alice.encrypt64(39049834));
    expect(res).to.equal(39049834);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (39049834, 39049830)', async function () {
    const res = await this.contract5.max_uint64_euint64(39049834, this.instances5.alice.encrypt64(39049830));
    expect(res).to.equal(39049834);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 1 (7, 4)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(7), 4);
    expect(res).to.equal(7);
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

  it('test operator "shr" overload (euint4, uint8) => euint4 test 1 (8, 2)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(8), 2);
    expect(res).to.equal(2);
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

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (107, 3)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(107),
      this.instances5.alice.encrypt8(3),
    );
    expect(res).to.equal(88);
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

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (107, 3)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(107), 3);
    expect(res).to.equal(88);
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

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (226, 1)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(226),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(113);
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

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (226, 1)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(226), 1);
    expect(res).to.equal(113);
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

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (31359, 1)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(31359),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(62718);
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

  it('test operator "shl" overload (euint16, uint8) => euint16 test 1 (31359, 1)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(31359), 1);
    expect(res).to.equal(62718);
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

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (47516, 3)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(47516),
      this.instances5.alice.encrypt8(3),
    );
    expect(res).to.equal(5939);
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

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (47516, 3)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(47516), 3);
    expect(res).to.equal(5939);
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

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (226777595, 3)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(226777595),
      this.instances5.alice.encrypt8(3),
    );
    expect(res).to.equal(1814220760);
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

  it('test operator "shl" overload (euint32, uint8) => euint32 test 1 (226777595, 3)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(226777595), 3);
    expect(res).to.equal(1814220760);
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

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (145753602, 4)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(145753602),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(9109600);
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

  it('test operator "shr" overload (euint32, uint8) => euint32 test 1 (145753602, 4)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(145753602), 4);
    expect(res).to.equal(9109600);
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

  it('test operator "shl" overload (euint64, euint8) => euint64 test 1 (39313001, 2)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(39313001),
      this.instances5.alice.encrypt8(2),
    );
    expect(res).to.equal(157252004);
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

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (39313001, 2)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(39313001), 2);
    expect(res).to.equal(157252004);
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

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (231610183, 4)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(231610183),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(14475636);
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

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (231610183, 4)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(231610183), 4);
    expect(res).to.equal(14475636);
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

  it('test operator "neg" overload (euint4) => euint4 test 1 (3)', async function () {
    const res = await this.contract5.neg_euint4(this.instances5.alice.encrypt4(3));
    expect(res).to.equal(13n);
  });

  it('test operator "not" overload (euint4) => euint4 test 1 (12)', async function () {
    const res = await this.contract5.not_euint4(this.instances5.alice.encrypt4(12));
    expect(res).to.equal(3n);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (165)', async function () {
    const res = await this.contract5.neg_euint8(this.instances5.alice.encrypt8(165));
    expect(res).to.equal(91n);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (74)', async function () {
    const res = await this.contract5.not_euint8(this.instances5.alice.encrypt8(74));
    expect(res).to.equal(181n);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (17405)', async function () {
    const res = await this.contract5.neg_euint16(this.instances5.alice.encrypt16(17405));
    expect(res).to.equal(48131n);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (51762)', async function () {
    const res = await this.contract5.not_euint16(this.instances5.alice.encrypt16(51762));
    expect(res).to.equal(13773n);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (202665091)', async function () {
    const res = await this.contract5.neg_euint32(this.instances5.alice.encrypt32(202665091));
    expect(res).to.equal(4092302205n);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (34192436)', async function () {
    const res = await this.contract5.not_euint32(this.instances5.alice.encrypt32(34192436));
    expect(res).to.equal(4260774859n);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (6247856)', async function () {
    const res = await this.contract5.neg_euint64(this.instances5.alice.encrypt64(6247856));
    expect(res).to.equal(18446744073703303760n);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (131894418)', async function () {
    const res = await this.contract5.not_euint64(this.instances5.alice.encrypt64(131894418));
    expect(res).to.equal(18446744073577657197n);
  });
});
