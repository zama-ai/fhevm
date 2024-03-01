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

  it('test operator "add" overload (euint4, euint4) => euint4 test 1 (11, 1)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, euint4) => euint4 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint4) => euint4 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 1 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 2 (3, 4)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 4 (4, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 1 (5, 2)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 1 (6, 5)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(7n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 1 (2, 10)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt4(10),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 1 (8, 2)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(8),
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

  it('test operator "ne" overload (euint4, euint4) => ebool test 1 (11, 12)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt4(12),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt4(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt4(7),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 1 (3, 4)', async function () {
    const res = await this.contract1.ge_euint4_euint4(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt4(4),
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

  it('test operator "gt" overload (euint4, euint4) => ebool test 1 (9, 12)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt4(12),
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

  it('test operator "le" overload (euint4, euint4) => ebool test 1 (8, 6)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(6),
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

  it('test operator "lt" overload (euint4, euint4) => ebool test 1 (10, 9)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 1 (6, 5)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(5n);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 1 (8, 3)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 1 (2, 12)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt8(12),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt8(5),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, euint8) => euint8 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint8) => euint8 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 1 (1, 14)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(14),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt8(5),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt8(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt8(3),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 1 (12, 110)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt8(110),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 2 (8, 12)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(12),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 3 (12, 12)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt8(12),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 4 (12, 8)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 1 (14, 110)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(110),
    );
    expect(res).to.equal(110n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 2 (10, 14)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(14),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 3 (14, 14)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(14),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 4 (14, 10)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 1 (12, 244)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt8(244),
    );
    expect(res).to.equal(248n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 2 (8, 12)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(12),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 3 (12, 12)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt8(12),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 4 (12, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 1 (14, 180)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(180),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 1 (7, 94)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt8(94),
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

  it('test operator "ge" overload (euint4, euint8) => ebool test 1 (12, 141)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt8(141),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(12),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt8(12),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 1 (7, 49)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt8(49),
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

  it('test operator "le" overload (euint4, euint8) => ebool test 1 (14, 151)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(151),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt8(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt8(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 1 (7, 92)', async function () {
    const res = await this.contract1.lt_euint4_euint8(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt8(92),
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

  it('test operator "min" overload (euint4, euint8) => euint8 test 1 (11, 237)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(237),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 2 (7, 11)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt8(11),
    );
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 3 (11, 11)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(11),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 4 (11, 7)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(7),
    );
    expect(res).to.equal(7n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 1 (11, 181)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(181),
    );
    expect(res).to.equal(181n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 2 (7, 11)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt8(11),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 3 (11, 11)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(11),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 4 (11, 7)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt8(7),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 1 (2, 9)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt16(9),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 2 (6, 8)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt16(5),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 4 (8, 6)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(6),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "sub" overload (euint4, euint16) => euint16 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint16) => euint16 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 1 (2, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt16(5),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt16(5),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt16(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt16(3),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 1 (10, 16702)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(16702),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 2 (6, 10)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt16(10),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 3 (10, 10)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(10),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 4 (10, 6)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(6),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 1 (3, 60460)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt16(60460),
    );
    expect(res).to.equal(60463n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 1 (6, 58722)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt16(58722),
    );
    expect(res).to.equal(58724n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 1 (9, 7831)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(7831),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt16(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(5),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 1 (2, 63787)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt16(63787),
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

  it('test operator "ge" overload (euint4, euint16) => ebool test 1 (9, 63554)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(63554),
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

  it('test operator "gt" overload (euint4, euint16) => ebool test 1 (13, 28270)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(28270),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt16(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt16(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 1 (1, 51341)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(51341),
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

  it('test operator "lt" overload (euint4, euint16) => ebool test 1 (6, 15496)', async function () {
    const res = await this.contract1.lt_euint4_euint16(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt16(15496),
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

  it('test operator "min" overload (euint4, euint16) => euint16 test 1 (10, 26263)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(26263),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 2 (6, 10)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt16(10),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 3 (10, 10)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(10),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 4 (10, 6)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt16(6),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 1 (2, 61770)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt16(61770),
    );
    expect(res).to.equal(61770n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 1 (2, 11)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt32(11),
    );
    expect(res).to.equal(13n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt32(5),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, euint32) => euint32 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint32) => euint32 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 1 (2, 7)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt32(7),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 2 (3, 4)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt32(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 4 (4, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(3),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 1 (6, 3534750602)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt32(3534750602),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 1 (3, 2840015564)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt32(2840015564),
    );
    expect(res).to.equal(2840015567n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 1 (6, 2183122437)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt32(2183122437),
    );
    expect(res).to.equal(2183122435n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 1 (6, 2902824357)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt32(2902824357),
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

  it('test operator "ne" overload (euint4, euint32) => ebool test 1 (13, 3287915476)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(3287915476),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(13),
      this.instances1.alice.encrypt32(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 1 (9, 3023026099)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(3023026099),
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

  it('test operator "gt" overload (euint4, euint32) => ebool test 1 (10, 2302702329)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(2302702329),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt32(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt32(6),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 1 (9, 467708048)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(467708048),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt32(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(9),
      this.instances1.alice.encrypt32(5),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 1 (11, 1378961804)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt32(1378961804),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt32(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt32(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt32(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 1 (5, 2670542696)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt32(2670542696),
    );
    expect(res).to.equal(5n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 1 (2, 1031795645)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt32(1031795645),
    );
    expect(res).to.equal(1031795645n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 1 (2, 9)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt64(9),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 2 (4, 6)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(6),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 3 (6, 6)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt64(6),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 4 (6, 4)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "sub" overload (euint4, euint64) => euint64 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint64) => euint64 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 1 (2, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt64(5),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt64(5),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt64(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt64(3),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 1 (7, 18444452142912700231)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt64(18444452142912700231),
    );
    expect(res).to.equal(7n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 1 (10, 18439589984533574615)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(18439589984533574615),
    );
    expect(res).to.equal(18439589984533574623n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 2 (6, 10)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 3 (10, 10)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 4 (10, 6)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(6),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 1 (7, 18438536745518625067)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt64(18438536745518625067),
    );
    expect(res).to.equal(18438536745518625068n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 1 (4, 18441783865246079825)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(18441783865246079825),
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

  it('test operator "ne" overload (euint4, euint64) => ebool test 1 (11, 18442031616327904827)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt64(18442031616327904827),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(7),
      this.instances1.alice.encrypt64(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt64(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt64(7),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 1 (14, 18441505983320830973)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt64(18441505983320830973),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(10),
      this.instances1.alice.encrypt64(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt64(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(14),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 1 (8, 18443962302044821049)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(18443962302044821049),
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

  it('test operator "le" overload (euint4, euint64) => ebool test 1 (12, 18438657387678135029)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt64(18438657387678135029),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(12),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt64(12),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(12),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 1 (8, 18442139041620940861)', async function () {
    const res = await this.contract1.lt_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(18442139041620940861),
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

  it('test operator "min" overload (euint4, euint64) => euint64 test 1 (6, 18437937380503493287)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt64(18437937380503493287),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 1 (3, 18443395042624107977)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt64(18443395042624107977),
    );
    expect(res).to.equal(18443395042624107977n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 1 (7, 7)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(7), 7);
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(5), 5);
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 1 (12, 3)', async function () {
    const res = await this.contract1.add_uint8_euint4(12, this.instances1.alice.encrypt4(3));
    expect(res).to.equal(15n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_uint8_euint4(4, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_uint8_euint4(5, this.instances1.alice.encrypt4(5));
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, uint8) => euint4 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, uint8) => euint4 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint8, euint4) => euint4 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_uint8_euint4(8, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint8, euint4) => euint4 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 1 (1, 10)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(1), 10);
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(3), 5);
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(3), 3);
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(5), 3);
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 1 (1, 10)', async function () {
    const res = await this.contract1.mul_uint8_euint4(1, this.instances1.alice.encrypt4(10));
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 2 (3, 4)', async function () {
    const res = await this.contract1.mul_uint8_euint4(3, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_uint8_euint4(3, this.instances1.alice.encrypt4(3));
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 4 (4, 3)', async function () {
    const res = await this.contract1.mul_uint8_euint4(4, this.instances1.alice.encrypt4(3));
    expect(res).to.equal(12n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 1 (3, 6)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(3), 6);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(2n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 1 (4, 10)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(4), 10);
    expect(res).to.equal(4n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(4n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(0n);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 1 (14, 4)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(14), 4);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(10), 14);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(14), 14);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(14), 10);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 1 (11, 1)', async function () {
    const res = await this.contract1.eq_uint8_euint4(11, this.instances1.alice.encrypt4(1));
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

  it('test operator "ne" overload (euint4, uint8) => ebool test 1 (7, 12)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(7), 12);
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

  it('test operator "ne" overload (uint8, euint4) => ebool test 1 (3, 1)', async function () {
    const res = await this.contract1.ne_uint8_euint4(3, this.instances1.alice.encrypt4(1));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ne_uint8_euint4(4, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ne_uint8_euint4(8, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ne_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 1 (12, 11)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(12), 11);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(8), 12);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(12), 12);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(12), 8);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 1 (2, 4)', async function () {
    const res = await this.contract1.ge_uint8_euint4(2, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(false);
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

  it('test operator "gt" overload (euint4, uint8) => ebool test 1 (7, 10)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(7), 10);
    expect(res).to.equal(false);
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

  it('test operator "gt" overload (uint8, euint4) => ebool test 1 (4, 14)', async function () {
    const res = await this.contract1.gt_uint8_euint4(4, this.instances1.alice.encrypt4(14));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.gt_uint8_euint4(10, this.instances1.alice.encrypt4(14));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.gt_uint8_euint4(14, this.instances1.alice.encrypt4(14));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.gt_uint8_euint4(14, this.instances1.alice.encrypt4(10));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 1 (14, 8)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(14), 8);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(10), 14);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(14), 14);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(14), 10);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 1 (11, 8)', async function () {
    const res = await this.contract1.le_uint8_euint4(11, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.le_uint8_euint4(4, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.le_uint8_euint4(8, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.le_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 1 (7, 14)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(7), 14);
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

  it('test operator "lt" overload (uint8, euint4) => ebool test 1 (12, 5)', async function () {
    const res = await this.contract1.lt_uint8_euint4(12, this.instances1.alice.encrypt4(5));
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

  it('test operator "min" overload (euint4, uint8) => euint4 test 1 (11, 10)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(11), 10);
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 2 (7, 11)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(7), 11);
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 3 (11, 11)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(11), 11);
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 4 (11, 7)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(11), 7);
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 1 (8, 11)', async function () {
    const res = await this.contract1.min_uint8_euint4(8, this.instances1.alice.encrypt4(11));
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 2 (7, 11)', async function () {
    const res = await this.contract1.min_uint8_euint4(7, this.instances1.alice.encrypt4(11));
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 3 (11, 11)', async function () {
    const res = await this.contract1.min_uint8_euint4(11, this.instances1.alice.encrypt4(11));
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 4 (11, 7)', async function () {
    const res = await this.contract1.min_uint8_euint4(11, this.instances1.alice.encrypt4(7));
    expect(res).to.equal(7n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 1 (11, 7)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(11), 7);
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 2 (7, 11)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(7), 11);
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 3 (11, 11)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(11), 11);
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 4 (11, 7)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(11), 7);
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 1 (7, 14)', async function () {
    const res = await this.contract1.max_uint8_euint4(7, this.instances1.alice.encrypt4(14));
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 2 (10, 14)', async function () {
    const res = await this.contract1.max_uint8_euint4(10, this.instances1.alice.encrypt4(14));
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 3 (14, 14)', async function () {
    const res = await this.contract1.max_uint8_euint4(14, this.instances1.alice.encrypt4(14));
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 4 (14, 10)', async function () {
    const res = await this.contract1.max_uint8_euint4(14, this.instances1.alice.encrypt4(10));
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 1 (9, 2)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(9),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(5),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(10n);
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
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint4) => euint8 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 1 (5, 2)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(5),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 2 (3, 4)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 4 (4, 3)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(4),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 1 (143, 6)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(143),
      this.instances1.alice.encrypt4(6),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 1 (89, 1)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(89),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(89n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 1 (100, 12)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(100),
      this.instances1.alice.encrypt4(12),
    );
    expect(res).to.equal(104n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 2 (8, 12)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(12),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 3 (12, 12)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(12),
      this.instances1.alice.encrypt4(12),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 4 (12, 8)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(12),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 1 (168, 1)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(168),
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

  it('test operator "ne" overload (euint8, euint4) => ebool test 1 (237, 1)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(237),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint4) => ebool test 1 (228, 4)', async function () {
    const res = await this.contract2.ge_euint8_euint4(
      this.instances2.alice.encrypt8(228),
      this.instances2.alice.encrypt4(4),
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

  it('test operator "gt" overload (euint8, euint4) => ebool test 1 (53, 14)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(53),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(10),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 1 (194, 8)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(194),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 1 (130, 5)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(130),
      this.instances2.alice.encrypt4(5),
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

  it('test operator "min" overload (euint8, euint4) => euint8 test 1 (58, 11)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(58),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 2 (7, 11)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(7),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 3 (11, 11)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt4(11),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 4 (11, 7)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt4(7),
    );
    expect(res).to.equal(7n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 1 (211, 14)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(211),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(211n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 2 (10, 14)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(10),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 3 (14, 14)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 4 (14, 10)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 1 (93, 114)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt8(114),
    );
    expect(res).to.equal(207n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 2 (91, 93)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(91),
      this.instances2.alice.encrypt8(93),
    );
    expect(res).to.equal(184n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 3 (93, 93)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt8(93),
    );
    expect(res).to.equal(186n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 4 (93, 91)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(93),
      this.instances2.alice.encrypt8(91),
    );
    expect(res).to.equal(184n);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 1 (52, 52)', async function () {
    const res = await this.contract2.sub_euint8_euint8(
      this.instances2.alice.encrypt8(52),
      this.instances2.alice.encrypt8(52),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 2 (52, 48)', async function () {
    const res = await this.contract2.sub_euint8_euint8(
      this.instances2.alice.encrypt8(52),
      this.instances2.alice.encrypt8(48),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 1 (9, 17)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt8(17),
    );
    expect(res).to.equal(153n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 2 (15, 16)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(15),
      this.instances2.alice.encrypt8(16),
    );
    expect(res).to.equal(240n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 3 (9, 9)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt8(9),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 4 (16, 15)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(16),
      this.instances2.alice.encrypt8(15),
    );
    expect(res).to.equal(240n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 1 (254, 204)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt8(204),
    );
    expect(res).to.equal(204n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 2 (200, 204)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(200),
      this.instances2.alice.encrypt8(204),
    );
    expect(res).to.equal(200n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 3 (204, 204)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(204),
      this.instances2.alice.encrypt8(204),
    );
    expect(res).to.equal(204n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 4 (204, 200)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(204),
      this.instances2.alice.encrypt8(200),
    );
    expect(res).to.equal(200n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 1 (174, 137)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(174),
      this.instances2.alice.encrypt8(137),
    );
    expect(res).to.equal(175n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 2 (133, 137)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(133),
      this.instances2.alice.encrypt8(137),
    );
    expect(res).to.equal(141n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 3 (137, 137)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(137),
      this.instances2.alice.encrypt8(137),
    );
    expect(res).to.equal(137n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 4 (137, 133)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(137),
      this.instances2.alice.encrypt8(133),
    );
    expect(res).to.equal(141n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 1 (183, 10)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(183),
      this.instances2.alice.encrypt8(10),
    );
    expect(res).to.equal(189n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 2 (6, 10)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(6),
      this.instances2.alice.encrypt8(10),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 3 (10, 10)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(10),
      this.instances2.alice.encrypt8(10),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 4 (10, 6)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(10),
      this.instances2.alice.encrypt8(6),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 1 (101, 188)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(101),
      this.instances2.alice.encrypt8(188),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 2 (97, 101)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(97),
      this.instances2.alice.encrypt8(101),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 3 (101, 101)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(101),
      this.instances2.alice.encrypt8(101),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 4 (101, 97)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(101),
      this.instances2.alice.encrypt8(97),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 1 (250, 143)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt8(143),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 2 (139, 143)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(139),
      this.instances2.alice.encrypt8(143),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 3 (143, 143)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(143),
      this.instances2.alice.encrypt8(143),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 4 (143, 139)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(143),
      this.instances2.alice.encrypt8(139),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 1 (41, 173)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(41),
      this.instances2.alice.encrypt8(173),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 2 (37, 41)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(37),
      this.instances2.alice.encrypt8(41),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 3 (41, 41)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(41),
      this.instances2.alice.encrypt8(41),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 4 (41, 37)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(41),
      this.instances2.alice.encrypt8(37),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 1 (225, 96)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(225),
      this.instances2.alice.encrypt8(96),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 2 (92, 96)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(92),
      this.instances2.alice.encrypt8(96),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 3 (96, 96)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt8(96),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 4 (96, 92)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt8(92),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 1 (102, 4)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(102),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
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

  it('test operator "lt" overload (euint8, euint8) => ebool test 1 (253, 98)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(253),
      this.instances2.alice.encrypt8(98),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 2 (94, 98)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(94),
      this.instances2.alice.encrypt8(98),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 3 (98, 98)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(98),
      this.instances2.alice.encrypt8(98),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 4 (98, 94)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(98),
      this.instances2.alice.encrypt8(94),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 1 (53, 97)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(53),
      this.instances2.alice.encrypt8(97),
    );
    expect(res).to.equal(53n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 2 (49, 53)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(49),
      this.instances2.alice.encrypt8(53),
    );
    expect(res).to.equal(49n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 3 (53, 53)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(53),
      this.instances2.alice.encrypt8(53),
    );
    expect(res).to.equal(53n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 4 (53, 49)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(53),
      this.instances2.alice.encrypt8(49),
    );
    expect(res).to.equal(49n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 1 (198, 211)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(198),
      this.instances2.alice.encrypt8(211),
    );
    expect(res).to.equal(211n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 2 (194, 198)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(194),
      this.instances2.alice.encrypt8(198),
    );
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 3 (198, 198)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(198),
      this.instances2.alice.encrypt8(198),
    );
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 4 (198, 194)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(198),
      this.instances2.alice.encrypt8(194),
    );
    expect(res).to.equal(198n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 1 (2, 208)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt16(208),
    );
    expect(res).to.equal(210n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 2 (83, 85)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(83),
      this.instances2.alice.encrypt16(85),
    );
    expect(res).to.equal(168n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 3 (85, 85)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(85),
      this.instances2.alice.encrypt16(85),
    );
    expect(res).to.equal(170n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 4 (85, 83)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(85),
      this.instances2.alice.encrypt16(83),
    );
    expect(res).to.equal(168n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 1 (236, 236)', async function () {
    const res = await this.contract2.sub_euint8_euint16(
      this.instances2.alice.encrypt8(236),
      this.instances2.alice.encrypt16(236),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 2 (236, 232)', async function () {
    const res = await this.contract2.sub_euint8_euint16(
      this.instances2.alice.encrypt8(236),
      this.instances2.alice.encrypt16(232),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 1 (3, 60)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(3),
      this.instances2.alice.encrypt16(60),
    );
    expect(res).to.equal(180n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 2 (9, 9)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt16(9),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 3 (9, 9)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt16(9),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 4 (9, 9)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(9),
      this.instances2.alice.encrypt16(9),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 1 (112, 22367)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(112),
      this.instances2.alice.encrypt16(22367),
    );
    expect(res).to.equal(80n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 2 (108, 112)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(108),
      this.instances2.alice.encrypt16(112),
    );
    expect(res).to.equal(96n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 3 (112, 112)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(112),
      this.instances2.alice.encrypt16(112),
    );
    expect(res).to.equal(112n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 4 (112, 108)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(112),
      this.instances2.alice.encrypt16(108),
    );
    expect(res).to.equal(96n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 1 (99, 46031)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(99),
      this.instances2.alice.encrypt16(46031),
    );
    expect(res).to.equal(46063n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 2 (95, 99)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(95),
      this.instances2.alice.encrypt16(99),
    );
    expect(res).to.equal(127n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 3 (99, 99)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(99),
      this.instances2.alice.encrypt16(99),
    );
    expect(res).to.equal(99n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 4 (99, 95)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(99),
      this.instances2.alice.encrypt16(95),
    );
    expect(res).to.equal(127n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 1 (129, 2322)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(129),
      this.instances2.alice.encrypt16(2322),
    );
    expect(res).to.equal(2451n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 2 (125, 129)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(125),
      this.instances2.alice.encrypt16(129),
    );
    expect(res).to.equal(252n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 3 (129, 129)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(129),
      this.instances2.alice.encrypt16(129),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 4 (129, 125)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(129),
      this.instances2.alice.encrypt16(125),
    );
    expect(res).to.equal(252n);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 1 (111, 58517)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(111),
      this.instances2.alice.encrypt16(58517),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 2 (107, 111)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(107),
      this.instances2.alice.encrypt16(111),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 3 (111, 111)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(111),
      this.instances2.alice.encrypt16(111),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 4 (111, 107)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(111),
      this.instances2.alice.encrypt16(107),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 1 (86, 21243)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(86),
      this.instances2.alice.encrypt16(21243),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 2 (82, 86)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(82),
      this.instances2.alice.encrypt16(86),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 3 (86, 86)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(86),
      this.instances2.alice.encrypt16(86),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 4 (86, 82)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(86),
      this.instances2.alice.encrypt16(82),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 1 (204, 4953)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(204),
      this.instances2.alice.encrypt16(4953),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 2 (200, 204)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(200),
      this.instances2.alice.encrypt16(204),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 3 (204, 204)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(204),
      this.instances2.alice.encrypt16(204),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 4 (204, 200)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(204),
      this.instances2.alice.encrypt16(200),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 1 (254, 24172)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt16(24172),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 2 (250, 254)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt16(254),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 3 (254, 254)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt16(254),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 4 (254, 250)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt16(250),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 1 (31, 28651)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(31),
      this.instances2.alice.encrypt16(28651),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 2 (27, 31)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(27),
      this.instances2.alice.encrypt16(31),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 3 (31, 31)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(31),
      this.instances2.alice.encrypt16(31),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 4 (31, 27)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(31),
      this.instances2.alice.encrypt16(27),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 1 (113, 2877)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(113),
      this.instances2.alice.encrypt16(2877),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 2 (109, 113)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(109),
      this.instances2.alice.encrypt16(113),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 3 (113, 113)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(113),
      this.instances2.alice.encrypt16(113),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 4 (113, 109)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(113),
      this.instances2.alice.encrypt16(109),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 1 (190, 58049)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(190),
      this.instances2.alice.encrypt16(58049),
    );
    expect(res).to.equal(190n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 2 (186, 190)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(186),
      this.instances2.alice.encrypt16(190),
    );
    expect(res).to.equal(186n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 3 (190, 190)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(190),
      this.instances2.alice.encrypt16(190),
    );
    expect(res).to.equal(190n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 4 (190, 186)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(190),
      this.instances2.alice.encrypt16(186),
    );
    expect(res).to.equal(186n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 1 (8, 11440)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(11440),
    );
    expect(res).to.equal(11440n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 1 (2, 161)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt32(161),
    );
    expect(res).to.equal(163n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 2 (96, 100)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(96),
      this.instances2.alice.encrypt32(100),
    );
    expect(res).to.equal(196n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 3 (100, 100)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(100),
      this.instances2.alice.encrypt32(100),
    );
    expect(res).to.equal(200n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 4 (100, 96)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(100),
      this.instances2.alice.encrypt32(96),
    );
    expect(res).to.equal(196n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 1 (83, 83)', async function () {
    const res = await this.contract2.sub_euint8_euint32(
      this.instances2.alice.encrypt8(83),
      this.instances2.alice.encrypt32(83),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 2 (83, 79)', async function () {
    const res = await this.contract2.sub_euint8_euint32(
      this.instances2.alice.encrypt8(83),
      this.instances2.alice.encrypt32(79),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 1 (2, 96)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt32(96),
    );
    expect(res).to.equal(192n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 2 (10, 12)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(10),
      this.instances2.alice.encrypt32(12),
    );
    expect(res).to.equal(120n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 3 (12, 12)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(12),
      this.instances2.alice.encrypt32(12),
    );
    expect(res).to.equal(144n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 4 (12, 10)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(12),
      this.instances2.alice.encrypt32(10),
    );
    expect(res).to.equal(120n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 1 (36, 664013992)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(36),
      this.instances2.alice.encrypt32(664013992),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 2 (32, 36)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(32),
      this.instances2.alice.encrypt32(36),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 3 (36, 36)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(36),
      this.instances2.alice.encrypt32(36),
    );
    expect(res).to.equal(36n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 4 (36, 32)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(36),
      this.instances2.alice.encrypt32(32),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 1 (50, 1388677537)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(50),
      this.instances2.alice.encrypt32(1388677537),
    );
    expect(res).to.equal(1388677555n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 2 (46, 50)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(46),
      this.instances2.alice.encrypt32(50),
    );
    expect(res).to.equal(62n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 3 (50, 50)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(50),
      this.instances2.alice.encrypt32(50),
    );
    expect(res).to.equal(50n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 4 (50, 46)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(50),
      this.instances2.alice.encrypt32(46),
    );
    expect(res).to.equal(62n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 1 (67, 2386754441)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(67),
      this.instances2.alice.encrypt32(2386754441),
    );
    expect(res).to.equal(2386754506n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 2 (63, 67)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(63),
      this.instances2.alice.encrypt32(67),
    );
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 3 (67, 67)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(67),
      this.instances2.alice.encrypt32(67),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 4 (67, 63)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(67),
      this.instances2.alice.encrypt32(63),
    );
    expect(res).to.equal(124n);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 1 (161, 1325601812)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt32(1325601812),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 2 (157, 161)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(157),
      this.instances2.alice.encrypt32(161),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 3 (161, 161)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt32(161),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 4 (161, 157)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(161),
      this.instances2.alice.encrypt32(157),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 1 (185, 1521229668)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(185),
      this.instances2.alice.encrypt32(1521229668),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 2 (181, 185)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(181),
      this.instances2.alice.encrypt32(185),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 3 (185, 185)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(185),
      this.instances2.alice.encrypt32(185),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 4 (185, 181)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(185),
      this.instances2.alice.encrypt32(181),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 1 (214, 2636986545)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(214),
      this.instances2.alice.encrypt32(2636986545),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 2 (210, 214)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(210),
      this.instances2.alice.encrypt32(214),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 3 (214, 214)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(214),
      this.instances2.alice.encrypt32(214),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 4 (214, 210)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(214),
      this.instances2.alice.encrypt32(210),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 1 (254, 3644170480)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt32(3644170480),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 2 (250, 254)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(250),
      this.instances2.alice.encrypt32(254),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 3 (254, 254)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt32(254),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 4 (254, 250)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(254),
      this.instances2.alice.encrypt32(250),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 1 (200, 2966523441)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(200),
      this.instances2.alice.encrypt32(2966523441),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 2 (196, 200)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(196),
      this.instances2.alice.encrypt32(200),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 3 (200, 200)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(200),
      this.instances2.alice.encrypt32(200),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 4 (200, 196)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(200),
      this.instances2.alice.encrypt32(196),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 1 (80, 2205479823)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(80),
      this.instances2.alice.encrypt32(2205479823),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 2 (76, 80)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(76),
      this.instances2.alice.encrypt32(80),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 3 (80, 80)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(80),
      this.instances2.alice.encrypt32(80),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 4 (80, 76)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(80),
      this.instances2.alice.encrypt32(76),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 1 (253, 2965000543)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(253),
      this.instances2.alice.encrypt32(2965000543),
    );
    expect(res).to.equal(253n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 2 (249, 253)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(249),
      this.instances2.alice.encrypt32(253),
    );
    expect(res).to.equal(249n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 3 (253, 253)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(253),
      this.instances2.alice.encrypt32(253),
    );
    expect(res).to.equal(253n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 4 (253, 249)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(253),
      this.instances2.alice.encrypt32(249),
    );
    expect(res).to.equal(249n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 1 (130, 566960922)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(130),
      this.instances2.alice.encrypt32(566960922),
    );
    expect(res).to.equal(566960922n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 2 (126, 130)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(126),
      this.instances2.alice.encrypt32(130),
    );
    expect(res).to.equal(130n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 3 (130, 130)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(130),
      this.instances2.alice.encrypt32(130),
    );
    expect(res).to.equal(130n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 4 (130, 126)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(130),
      this.instances2.alice.encrypt32(126),
    );
    expect(res).to.equal(130n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 1 (2, 129)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt64(129),
    );
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 2 (115, 119)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(115),
      this.instances2.alice.encrypt64(119),
    );
    expect(res).to.equal(234n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 3 (119, 119)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(119),
      this.instances2.alice.encrypt64(119),
    );
    expect(res).to.equal(238n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 4 (119, 115)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(119),
      this.instances2.alice.encrypt64(115),
    );
    expect(res).to.equal(234n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 1 (168, 168)', async function () {
    const res = await this.contract2.sub_euint8_euint64(
      this.instances2.alice.encrypt8(168),
      this.instances2.alice.encrypt64(168),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 2 (168, 164)', async function () {
    const res = await this.contract2.sub_euint8_euint64(
      this.instances2.alice.encrypt8(168),
      this.instances2.alice.encrypt64(164),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 1 (2, 65)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt64(65),
    );
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 2 (10, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(10),
      this.instances2.alice.encrypt64(11),
    );
    expect(res).to.equal(110n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 3 (11, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt64(11),
    );
    expect(res).to.equal(121n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 4 (11, 10)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt64(10),
    );
    expect(res).to.equal(110n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 1 (187, 18444084668783699555)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(187),
      this.instances2.alice.encrypt64(18444084668783699555),
    );
    expect(res).to.equal(35n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 2 (183, 187)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(183),
      this.instances2.alice.encrypt64(187),
    );
    expect(res).to.equal(179n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 3 (187, 187)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(187),
      this.instances2.alice.encrypt64(187),
    );
    expect(res).to.equal(187n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 4 (187, 183)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(187),
      this.instances2.alice.encrypt64(183),
    );
    expect(res).to.equal(179n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 1 (135, 18439029290182698975)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(135),
      this.instances2.alice.encrypt64(18439029290182698975),
    );
    expect(res).to.equal(18439029290182698975n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 2 (131, 135)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(131),
      this.instances2.alice.encrypt64(135),
    );
    expect(res).to.equal(135n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 3 (135, 135)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(135),
      this.instances2.alice.encrypt64(135),
    );
    expect(res).to.equal(135n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 4 (135, 131)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(135),
      this.instances2.alice.encrypt64(131),
    );
    expect(res).to.equal(135n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 1 (84, 18444990477299490715)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(84),
      this.instances2.alice.encrypt64(18444990477299490715),
    );
    expect(res).to.equal(18444990477299490767n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 2 (80, 84)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(80),
      this.instances2.alice.encrypt64(84),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 3 (84, 84)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(84),
      this.instances2.alice.encrypt64(84),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 4 (84, 80)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(84),
      this.instances2.alice.encrypt64(80),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 1 (91, 18446457346943992227)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(91),
      this.instances2.alice.encrypt64(18446457346943992227),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 2 (87, 91)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(87),
      this.instances2.alice.encrypt64(91),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 3 (91, 91)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(91),
      this.instances2.alice.encrypt64(91),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 4 (91, 87)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(91),
      this.instances2.alice.encrypt64(87),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 1 (109, 18443318570639553087)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(109),
      this.instances2.alice.encrypt64(18443318570639553087),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 2 (105, 109)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(105),
      this.instances2.alice.encrypt64(109),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 3 (109, 109)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(109),
      this.instances2.alice.encrypt64(109),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 4 (109, 105)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(109),
      this.instances2.alice.encrypt64(105),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 1 (27, 18445930867214181117)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(27),
      this.instances2.alice.encrypt64(18445930867214181117),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 2 (23, 27)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(23),
      this.instances2.alice.encrypt64(27),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 3 (27, 27)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(27),
      this.instances2.alice.encrypt64(27),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 4 (27, 23)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(27),
      this.instances2.alice.encrypt64(23),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 1 (235, 18439120393033635471)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(235),
      this.instances2.alice.encrypt64(18439120393033635471),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 2 (231, 235)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(231),
      this.instances2.alice.encrypt64(235),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 3 (235, 235)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(235),
      this.instances2.alice.encrypt64(235),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 4 (235, 231)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(235),
      this.instances2.alice.encrypt64(231),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 1 (80, 18440486388708045995)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(80),
      this.instances2.alice.encrypt64(18440486388708045995),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 2 (76, 80)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(76),
      this.instances2.alice.encrypt64(80),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 3 (80, 80)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(80),
      this.instances2.alice.encrypt64(80),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 4 (80, 76)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(80),
      this.instances2.alice.encrypt64(76),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 1 (190, 18443665731691391943)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(190),
      this.instances2.alice.encrypt64(18443665731691391943),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 2 (186, 190)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(186),
      this.instances2.alice.encrypt64(190),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 3 (190, 190)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(190),
      this.instances2.alice.encrypt64(190),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 4 (190, 186)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(190),
      this.instances2.alice.encrypt64(186),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 1 (18, 18446036892619585799)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(18),
      this.instances2.alice.encrypt64(18446036892619585799),
    );
    expect(res).to.equal(18n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 2 (14, 18)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(14),
      this.instances2.alice.encrypt64(18),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 3 (18, 18)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(18),
      this.instances2.alice.encrypt64(18),
    );
    expect(res).to.equal(18n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 4 (18, 14)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(18),
      this.instances2.alice.encrypt64(14),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 1 (172, 18445195398017975891)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(172),
      this.instances2.alice.encrypt64(18445195398017975891),
    );
    expect(res).to.equal(18445195398017975891n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 2 (168, 172)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(168),
      this.instances2.alice.encrypt64(172),
    );
    expect(res).to.equal(172n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 3 (172, 172)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(172),
      this.instances2.alice.encrypt64(172),
    );
    expect(res).to.equal(172n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 4 (172, 168)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(172),
      this.instances2.alice.encrypt64(168),
    );
    expect(res).to.equal(172n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 1 (93, 112)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(93), 112);
    expect(res).to.equal(205n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 2 (91, 93)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(91), 93);
    expect(res).to.equal(184n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 3 (93, 93)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(93), 93);
    expect(res).to.equal(186n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 4 (93, 91)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(93), 91);
    expect(res).to.equal(184n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 1 (34, 112)', async function () {
    const res = await this.contract2.add_uint8_euint8(34, this.instances2.alice.encrypt8(112));
    expect(res).to.equal(146n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 2 (91, 93)', async function () {
    const res = await this.contract2.add_uint8_euint8(91, this.instances2.alice.encrypt8(93));
    expect(res).to.equal(184n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 3 (93, 93)', async function () {
    const res = await this.contract2.add_uint8_euint8(93, this.instances2.alice.encrypt8(93));
    expect(res).to.equal(186n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 4 (93, 91)', async function () {
    const res = await this.contract2.add_uint8_euint8(93, this.instances2.alice.encrypt8(91));
    expect(res).to.equal(184n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 1 (52, 52)', async function () {
    const res = await this.contract2.sub_euint8_uint8(this.instances2.alice.encrypt8(52), 52);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 2 (52, 48)', async function () {
    const res = await this.contract2.sub_euint8_uint8(this.instances2.alice.encrypt8(52), 48);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 1 (52, 52)', async function () {
    const res = await this.contract2.sub_uint8_euint8(52, this.instances2.alice.encrypt8(52));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 2 (52, 48)', async function () {
    const res = await this.contract2.sub_uint8_euint8(52, this.instances2.alice.encrypt8(48));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 1 (16, 7)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(16), 7);
    expect(res).to.equal(112n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 2 (15, 16)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(15), 16);
    expect(res).to.equal(240n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 3 (9, 9)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(9), 9);
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 4 (16, 15)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(16), 15);
    expect(res).to.equal(240n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 1 (14, 12)', async function () {
    const res = await this.contract2.mul_uint8_euint8(14, this.instances2.alice.encrypt8(12));
    expect(res).to.equal(168n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 2 (15, 16)', async function () {
    const res = await this.contract2.mul_uint8_euint8(15, this.instances2.alice.encrypt8(16));
    expect(res).to.equal(240n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 3 (9, 9)', async function () {
    const res = await this.contract2.mul_uint8_euint8(9, this.instances2.alice.encrypt8(9));
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 4 (16, 15)', async function () {
    const res = await this.contract2.mul_uint8_euint8(16, this.instances2.alice.encrypt8(15));
    expect(res).to.equal(240n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 1 (123, 214)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(123), 214);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 2 (20, 24)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(20), 24);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 3 (24, 24)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(24), 24);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 4 (24, 20)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(24), 20);
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 1 (144, 19)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(144), 19);
    expect(res).to.equal(11n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 2 (140, 144)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(140), 144);
    expect(res).to.equal(140n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 3 (144, 144)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(144), 144);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 4 (144, 140)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(144), 140);
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 1 (101, 175)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(101), 175);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 2 (97, 101)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(97), 101);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 3 (101, 101)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(101), 101);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 4 (101, 97)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(101), 97);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 1 (182, 175)', async function () {
    const res = await this.contract2.eq_uint8_euint8(182, this.instances2.alice.encrypt8(175));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 2 (97, 101)', async function () {
    const res = await this.contract2.eq_uint8_euint8(97, this.instances2.alice.encrypt8(101));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 3 (101, 101)', async function () {
    const res = await this.contract2.eq_uint8_euint8(101, this.instances2.alice.encrypt8(101));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 4 (101, 97)', async function () {
    const res = await this.contract2.eq_uint8_euint8(101, this.instances2.alice.encrypt8(97));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 1 (250, 114)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(250), 114);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 2 (139, 143)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(139), 143);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 3 (143, 143)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(143), 143);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 4 (143, 139)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(143), 139);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 1 (139, 114)', async function () {
    const res = await this.contract2.ne_uint8_euint8(139, this.instances2.alice.encrypt8(114));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 2 (139, 143)', async function () {
    const res = await this.contract2.ne_uint8_euint8(139, this.instances2.alice.encrypt8(143));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 3 (143, 143)', async function () {
    const res = await this.contract2.ne_uint8_euint8(143, this.instances2.alice.encrypt8(143));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 4 (143, 139)', async function () {
    const res = await this.contract2.ne_uint8_euint8(143, this.instances2.alice.encrypt8(139));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (41, 155)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(41), 155);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 2 (37, 41)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(37), 41);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 3 (41, 41)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(41), 41);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 4 (41, 37)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(41), 37);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 1 (28, 155)', async function () {
    const res = await this.contract2.ge_uint8_euint8(28, this.instances2.alice.encrypt8(155));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 2 (37, 41)', async function () {
    const res = await this.contract2.ge_uint8_euint8(37, this.instances2.alice.encrypt8(41));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 3 (41, 41)', async function () {
    const res = await this.contract2.ge_uint8_euint8(41, this.instances2.alice.encrypt8(41));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 4 (41, 37)', async function () {
    const res = await this.contract2.ge_uint8_euint8(41, this.instances2.alice.encrypt8(37));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (225, 176)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(225), 176);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 2 (92, 96)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(92), 96);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 3 (96, 96)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(96), 96);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 4 (96, 92)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(96), 92);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 1 (34, 176)', async function () {
    const res = await this.contract2.gt_uint8_euint8(34, this.instances2.alice.encrypt8(176));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 2 (92, 96)', async function () {
    const res = await this.contract2.gt_uint8_euint8(92, this.instances2.alice.encrypt8(96));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 3 (96, 96)', async function () {
    const res = await this.contract2.gt_uint8_euint8(96, this.instances2.alice.encrypt8(96));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 4 (96, 92)', async function () {
    const res = await this.contract2.gt_uint8_euint8(96, this.instances2.alice.encrypt8(92));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (102, 85)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(102), 85);
    expect(res).to.equal(false);
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

  it('test operator "le" overload (uint8, euint8) => ebool test 1 (46, 85)', async function () {
    const res = await this.contract2.le_uint8_euint8(46, this.instances2.alice.encrypt8(85));
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

  it('test operator "lt" overload (euint8, uint8) => ebool test 1 (253, 9)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(253), 9);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 2 (94, 98)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(94), 98);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 3 (98, 98)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(98), 98);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 4 (98, 94)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(98), 94);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (79, 9)', async function () {
    const res = await this.contract2.lt_uint8_euint8(79, this.instances2.alice.encrypt8(9));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 2 (94, 98)', async function () {
    const res = await this.contract2.lt_uint8_euint8(94, this.instances2.alice.encrypt8(98));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 3 (98, 98)', async function () {
    const res = await this.contract2.lt_uint8_euint8(98, this.instances2.alice.encrypt8(98));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 4 (98, 94)', async function () {
    const res = await this.contract2.lt_uint8_euint8(98, this.instances2.alice.encrypt8(94));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (53, 70)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(53), 70);
    expect(res).to.equal(53n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (49, 53)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(49), 53);
    expect(res).to.equal(49n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (53, 53)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(53), 53);
    expect(res).to.equal(53n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 4 (53, 49)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(53), 49);
    expect(res).to.equal(49n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (109, 70)', async function () {
    const res = await this.contract2.min_uint8_euint8(109, this.instances2.alice.encrypt8(70));
    expect(res).to.equal(70n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 2 (49, 53)', async function () {
    const res = await this.contract2.min_uint8_euint8(49, this.instances2.alice.encrypt8(53));
    expect(res).to.equal(49n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 3 (53, 53)', async function () {
    const res = await this.contract2.min_uint8_euint8(53, this.instances2.alice.encrypt8(53));
    expect(res).to.equal(53n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 4 (53, 49)', async function () {
    const res = await this.contract2.min_uint8_euint8(53, this.instances2.alice.encrypt8(49));
    expect(res).to.equal(49n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (198, 31)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(198), 31);
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (194, 198)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(194), 198);
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (198, 198)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(198), 198);
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 4 (198, 194)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(198), 194);
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 1 (102, 31)', async function () {
    const res = await this.contract2.max_uint8_euint8(102, this.instances2.alice.encrypt8(31));
    expect(res).to.equal(102n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 2 (194, 198)', async function () {
    const res = await this.contract2.max_uint8_euint8(194, this.instances2.alice.encrypt8(198));
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 3 (198, 198)', async function () {
    const res = await this.contract2.max_uint8_euint8(198, this.instances2.alice.encrypt8(198));
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 4 (198, 194)', async function () {
    const res = await this.contract2.max_uint8_euint8(198, this.instances2.alice.encrypt8(194));
    expect(res).to.equal(198n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 1 (13, 2)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(13),
      this.instances2.alice.encrypt4(2),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 3 (5, 5)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(5),
      this.instances2.alice.encrypt4(5),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint16, euint4) => euint16 test 1 (14, 14)', async function () {
    const res = await this.contract2.sub_euint16_euint4(
      this.instances2.alice.encrypt16(14),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint4) => euint16 test 2 (14, 10)', async function () {
    const res = await this.contract2.sub_euint16_euint4(
      this.instances2.alice.encrypt16(14),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 1 (7, 2)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(7),
      this.instances2.alice.encrypt4(2),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 2 (3, 5)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(3),
      this.instances2.alice.encrypt4(5),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 3 (3, 3)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(3),
      this.instances2.alice.encrypt4(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 4 (5, 3)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(5),
      this.instances2.alice.encrypt4(3),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 1 (6680, 4)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(6680),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 1 (57838, 2)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(57838),
      this.instances2.alice.encrypt4(2),
    );
    expect(res).to.equal(57838n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 1 (29564, 14)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(29564),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(29554n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 2 (10, 14)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(10),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 3 (14, 14)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(14),
      this.instances2.alice.encrypt4(14),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 4 (14, 10)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(14),
      this.instances2.alice.encrypt4(10),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 1 (43178, 6)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(43178),
      this.instances2.alice.encrypt4(6),
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

  it('test operator "ne" overload (euint16, euint4) => ebool test 1 (36210, 8)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(36210),
      this.instances2.alice.encrypt4(8),
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

  it('test operator "ge" overload (euint16, euint4) => ebool test 1 (9661, 2)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(9661),
      this.instances2.alice.encrypt4(2),
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

  it('test operator "gt" overload (euint16, euint4) => ebool test 1 (36260, 13)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(36260),
      this.instances2.alice.encrypt4(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(9),
      this.instances2.alice.encrypt4(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(13),
      this.instances2.alice.encrypt4(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(13),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 1 (7456, 9)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(7456),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(5),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(9),
      this.instances2.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(9),
      this.instances2.alice.encrypt4(5),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 1 (53504, 6)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(53504),
      this.instances2.alice.encrypt4(6),
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

  it('test operator "min" overload (euint16, euint4) => euint16 test 1 (37174, 5)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(37174),
      this.instances3.alice.encrypt4(5),
    );
    expect(res).to.equal(5n);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 1 (29272, 10)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(29272),
      this.instances3.alice.encrypt4(10),
    );
    expect(res).to.equal(29272n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 2 (6, 10)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(6),
      this.instances3.alice.encrypt4(10),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 3 (10, 10)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(10),
      this.instances3.alice.encrypt4(10),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 4 (10, 6)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(10),
      this.instances3.alice.encrypt4(6),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 1 (171, 2)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(171),
      this.instances3.alice.encrypt8(2),
    );
    expect(res).to.equal(173n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 2 (7, 11)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(7),
      this.instances3.alice.encrypt8(11),
    );
    expect(res).to.equal(18n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 3 (11, 11)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(11),
      this.instances3.alice.encrypt8(11),
    );
    expect(res).to.equal(22n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 4 (11, 7)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(11),
      this.instances3.alice.encrypt8(7),
    );
    expect(res).to.equal(18n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (166, 166)', async function () {
    const res = await this.contract3.sub_euint16_euint8(
      this.instances3.alice.encrypt16(166),
      this.instances3.alice.encrypt8(166),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (166, 162)', async function () {
    const res = await this.contract3.sub_euint16_euint8(
      this.instances3.alice.encrypt16(166),
      this.instances3.alice.encrypt8(162),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (110, 2)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(110),
      this.instances3.alice.encrypt8(2),
    );
    expect(res).to.equal(220n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 2 (13, 13)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(13),
      this.instances3.alice.encrypt8(13),
    );
    expect(res).to.equal(169n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 3 (13, 13)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(13),
      this.instances3.alice.encrypt8(13),
    );
    expect(res).to.equal(169n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 4 (13, 13)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(13),
      this.instances3.alice.encrypt8(13),
    );
    expect(res).to.equal(169n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 1 (18191, 188)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(18191),
      this.instances3.alice.encrypt8(188),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 2 (184, 188)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(184),
      this.instances3.alice.encrypt8(188),
    );
    expect(res).to.equal(184n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 3 (188, 188)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(188),
      this.instances3.alice.encrypt8(188),
    );
    expect(res).to.equal(188n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 4 (188, 184)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(188),
      this.instances3.alice.encrypt8(184),
    );
    expect(res).to.equal(184n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 1 (60745, 176)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(60745),
      this.instances3.alice.encrypt8(176),
    );
    expect(res).to.equal(60921n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 2 (172, 176)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(172),
      this.instances3.alice.encrypt8(176),
    );
    expect(res).to.equal(188n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 3 (176, 176)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(176),
      this.instances3.alice.encrypt8(176),
    );
    expect(res).to.equal(176n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 4 (176, 172)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(176),
      this.instances3.alice.encrypt8(172),
    );
    expect(res).to.equal(188n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (24120, 221)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(24120),
      this.instances3.alice.encrypt8(221),
    );
    expect(res).to.equal(24293n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (217, 221)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(217),
      this.instances3.alice.encrypt8(221),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 3 (221, 221)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(221),
      this.instances3.alice.encrypt8(221),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 4 (221, 217)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(221),
      this.instances3.alice.encrypt8(217),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 1 (52156, 49)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(52156),
      this.instances3.alice.encrypt8(49),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 2 (45, 49)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(45),
      this.instances3.alice.encrypt8(49),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 3 (49, 49)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(49),
      this.instances3.alice.encrypt8(49),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 4 (49, 45)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(49),
      this.instances3.alice.encrypt8(45),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (45762, 206)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(45762),
      this.instances3.alice.encrypt8(206),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (202, 206)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(202),
      this.instances3.alice.encrypt8(206),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 3 (206, 206)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(206),
      this.instances3.alice.encrypt8(206),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 4 (206, 202)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(206),
      this.instances3.alice.encrypt8(202),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (3763, 81)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(3763),
      this.instances3.alice.encrypt8(81),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (77, 81)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(77),
      this.instances3.alice.encrypt8(81),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (81, 81)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(81),
      this.instances3.alice.encrypt8(81),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 4 (81, 77)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(81),
      this.instances3.alice.encrypt8(77),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 1 (17649, 8)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(17649),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 1 (3242, 147)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(3242),
      this.instances3.alice.encrypt8(147),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 2 (143, 147)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(143),
      this.instances3.alice.encrypt8(147),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 3 (147, 147)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(147),
      this.instances3.alice.encrypt8(147),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 4 (147, 143)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(147),
      this.instances3.alice.encrypt8(143),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 1 (42518, 110)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(42518),
      this.instances3.alice.encrypt8(110),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 2 (106, 110)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(106),
      this.instances3.alice.encrypt8(110),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 3 (110, 110)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(110),
      this.instances3.alice.encrypt8(110),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 4 (110, 106)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(110),
      this.instances3.alice.encrypt8(106),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 1 (25144, 64)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(25144),
      this.instances3.alice.encrypt8(64),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 2 (60, 64)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(60),
      this.instances3.alice.encrypt8(64),
    );
    expect(res).to.equal(60n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 3 (64, 64)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(64),
      this.instances3.alice.encrypt8(64),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 4 (64, 60)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(64),
      this.instances3.alice.encrypt8(60),
    );
    expect(res).to.equal(60n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 1 (18336, 117)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(18336),
      this.instances3.alice.encrypt8(117),
    );
    expect(res).to.equal(18336n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 2 (113, 117)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(113),
      this.instances3.alice.encrypt8(117),
    );
    expect(res).to.equal(117n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 3 (117, 117)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(117),
      this.instances3.alice.encrypt8(117),
    );
    expect(res).to.equal(117n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 4 (117, 113)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(117),
      this.instances3.alice.encrypt8(113),
    );
    expect(res).to.equal(117n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 1 (27226, 22058)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(27226),
      this.instances3.alice.encrypt16(22058),
    );
    expect(res).to.equal(49284n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 2 (22056, 22058)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(22056),
      this.instances3.alice.encrypt16(22058),
    );
    expect(res).to.equal(44114n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 3 (22058, 22058)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(22058),
      this.instances3.alice.encrypt16(22058),
    );
    expect(res).to.equal(44116n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 4 (22058, 22056)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(22058),
      this.instances3.alice.encrypt16(22056),
    );
    expect(res).to.equal(44114n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 1 (56955, 56955)', async function () {
    const res = await this.contract3.sub_euint16_euint16(
      this.instances3.alice.encrypt16(56955),
      this.instances3.alice.encrypt16(56955),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 2 (56955, 56951)', async function () {
    const res = await this.contract3.sub_euint16_euint16(
      this.instances3.alice.encrypt16(56955),
      this.instances3.alice.encrypt16(56951),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 1 (120, 150)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(120),
      this.instances3.alice.encrypt16(150),
    );
    expect(res).to.equal(18000n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 2 (239, 239)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(239),
      this.instances3.alice.encrypt16(239),
    );
    expect(res).to.equal(57121n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 3 (239, 239)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(239),
      this.instances3.alice.encrypt16(239),
    );
    expect(res).to.equal(57121n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 4 (239, 239)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(239),
      this.instances3.alice.encrypt16(239),
    );
    expect(res).to.equal(57121n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 1 (63103, 25335)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(63103),
      this.instances3.alice.encrypt16(25335),
    );
    expect(res).to.equal(25207n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 2 (25331, 25335)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(25331),
      this.instances3.alice.encrypt16(25335),
    );
    expect(res).to.equal(25331n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 3 (25335, 25335)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(25335),
      this.instances3.alice.encrypt16(25335),
    );
    expect(res).to.equal(25335n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 4 (25335, 25331)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(25335),
      this.instances3.alice.encrypt16(25331),
    );
    expect(res).to.equal(25331n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 1 (53682, 14727)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(53682),
      this.instances3.alice.encrypt16(14727),
    );
    expect(res).to.equal(63927n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 2 (14723, 14727)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(14723),
      this.instances3.alice.encrypt16(14727),
    );
    expect(res).to.equal(14727n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 3 (14727, 14727)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(14727),
      this.instances3.alice.encrypt16(14727),
    );
    expect(res).to.equal(14727n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 4 (14727, 14723)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(14727),
      this.instances3.alice.encrypt16(14723),
    );
    expect(res).to.equal(14727n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (272, 42865)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(272),
      this.instances3.alice.encrypt16(42865),
    );
    expect(res).to.equal(42593n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (268, 272)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(268),
      this.instances3.alice.encrypt16(272),
    );
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 3 (272, 272)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(272),
      this.instances3.alice.encrypt16(272),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 4 (272, 268)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(272),
      this.instances3.alice.encrypt16(268),
    );
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 1 (53936, 55687)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(53936),
      this.instances3.alice.encrypt16(55687),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 2 (53932, 53936)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(53932),
      this.instances3.alice.encrypt16(53936),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 3 (53936, 53936)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(53936),
      this.instances3.alice.encrypt16(53936),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 4 (53936, 53932)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(53936),
      this.instances3.alice.encrypt16(53932),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 1 (64200, 17038)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(64200),
      this.instances3.alice.encrypt16(17038),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 2 (17034, 17038)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(17034),
      this.instances3.alice.encrypt16(17038),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 3 (17038, 17038)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(17038),
      this.instances3.alice.encrypt16(17038),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 4 (17038, 17034)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(17038),
      this.instances3.alice.encrypt16(17034),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 1 (18226, 11817)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(18226),
      this.instances3.alice.encrypt16(11817),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 2 (11813, 11817)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(11813),
      this.instances3.alice.encrypt16(11817),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 3 (11817, 11817)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(11817),
      this.instances3.alice.encrypt16(11817),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 4 (11817, 11813)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(11817),
      this.instances3.alice.encrypt16(11813),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 1 (46712, 53146)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(46712),
      this.instances3.alice.encrypt16(53146),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 2 (46708, 46712)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(46708),
      this.instances3.alice.encrypt16(46712),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 3 (46712, 46712)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(46712),
      this.instances3.alice.encrypt16(46712),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 4 (46712, 46708)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(46712),
      this.instances3.alice.encrypt16(46708),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 1 (9281, 48556)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(9281),
      this.instances3.alice.encrypt16(48556),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 2 (9277, 9281)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(9277),
      this.instances3.alice.encrypt16(9281),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 3 (9281, 9281)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(9281),
      this.instances3.alice.encrypt16(9281),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 4 (9281, 9277)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(9281),
      this.instances3.alice.encrypt16(9277),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 1 (44794, 23290)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(44794),
      this.instances3.alice.encrypt16(23290),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 2 (23286, 23290)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(23286),
      this.instances3.alice.encrypt16(23290),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 3 (23290, 23290)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(23290),
      this.instances3.alice.encrypt16(23290),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 4 (23290, 23286)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(23290),
      this.instances3.alice.encrypt16(23286),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 1 (30936, 9027)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(30936),
      this.instances3.alice.encrypt16(9027),
    );
    expect(res).to.equal(9027n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 2 (9023, 9027)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(9023),
      this.instances3.alice.encrypt16(9027),
    );
    expect(res).to.equal(9023n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 3 (9027, 9027)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(9027),
      this.instances3.alice.encrypt16(9027),
    );
    expect(res).to.equal(9027n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 4 (9027, 9023)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(9027),
      this.instances3.alice.encrypt16(9023),
    );
    expect(res).to.equal(9023n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 1 (34561, 34789)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(34561),
      this.instances3.alice.encrypt16(34789),
    );
    expect(res).to.equal(34789n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 2 (34557, 34561)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(34557),
      this.instances3.alice.encrypt16(34561),
    );
    expect(res).to.equal(34561n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 3 (34561, 34561)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(34561),
      this.instances3.alice.encrypt16(34561),
    );
    expect(res).to.equal(34561n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 4 (34561, 34557)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(34561),
      this.instances3.alice.encrypt16(34557),
    );
    expect(res).to.equal(34561n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 1 (2, 41075)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt32(41075),
    );
    expect(res).to.equal(41077n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 2 (25150, 25154)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(25150),
      this.instances3.alice.encrypt32(25154),
    );
    expect(res).to.equal(50304n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 3 (25154, 25154)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(25154),
      this.instances3.alice.encrypt32(25154),
    );
    expect(res).to.equal(50308n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 4 (25154, 25150)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(25154),
      this.instances3.alice.encrypt32(25150),
    );
    expect(res).to.equal(50304n);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (42408, 42408)', async function () {
    const res = await this.contract3.sub_euint16_euint32(
      this.instances3.alice.encrypt16(42408),
      this.instances3.alice.encrypt32(42408),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (42408, 42404)', async function () {
    const res = await this.contract3.sub_euint16_euint32(
      this.instances3.alice.encrypt16(42408),
      this.instances3.alice.encrypt32(42404),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (2, 32121)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt32(32121),
    );
    expect(res).to.equal(64242n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 2 (235, 235)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(235),
      this.instances3.alice.encrypt32(235),
    );
    expect(res).to.equal(55225n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 3 (235, 235)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(235),
      this.instances3.alice.encrypt32(235),
    );
    expect(res).to.equal(55225n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 4 (235, 235)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(235),
      this.instances3.alice.encrypt32(235),
    );
    expect(res).to.equal(55225n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (31764, 719791208)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(31764),
      this.instances3.alice.encrypt32(719791208),
    );
    expect(res).to.equal(9216n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (31760, 31764)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(31760),
      this.instances3.alice.encrypt32(31764),
    );
    expect(res).to.equal(31760n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (31764, 31764)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(31764),
      this.instances3.alice.encrypt32(31764),
    );
    expect(res).to.equal(31764n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (31764, 31760)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(31764),
      this.instances3.alice.encrypt32(31760),
    );
    expect(res).to.equal(31760n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (47569, 1867557342)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(47569),
      this.instances3.alice.encrypt32(1867557342),
    );
    expect(res).to.equal(1867561439n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (47565, 47569)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(47565),
      this.instances3.alice.encrypt32(47569),
    );
    expect(res).to.equal(47581n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 3 (47569, 47569)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(47569),
      this.instances3.alice.encrypt32(47569),
    );
    expect(res).to.equal(47569n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 4 (47569, 47565)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(47569),
      this.instances3.alice.encrypt32(47565),
    );
    expect(res).to.equal(47581n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 1 (49958, 2256953798)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(49958),
      this.instances3.alice.encrypt32(2256953798),
    );
    expect(res).to.equal(2256970464n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 2 (49954, 49958)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(49954),
      this.instances3.alice.encrypt32(49958),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 3 (49958, 49958)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(49958),
      this.instances3.alice.encrypt32(49958),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 4 (49958, 49954)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(49958),
      this.instances3.alice.encrypt32(49954),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (26213, 2142712247)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(26213),
      this.instances3.alice.encrypt32(2142712247),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (26209, 26213)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(26209),
      this.instances3.alice.encrypt32(26213),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 3 (26213, 26213)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(26213),
      this.instances3.alice.encrypt32(26213),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 4 (26213, 26209)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(26213),
      this.instances3.alice.encrypt32(26209),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 1 (52545, 2079966846)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(52545),
      this.instances3.alice.encrypt32(2079966846),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 2 (52541, 52545)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(52541),
      this.instances3.alice.encrypt32(52545),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 3 (52545, 52545)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(52545),
      this.instances3.alice.encrypt32(52545),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 4 (52545, 52541)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(52545),
      this.instances3.alice.encrypt32(52541),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (19364, 2908406445)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(19364),
      this.instances3.alice.encrypt32(2908406445),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (19360, 19364)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(19360),
      this.instances3.alice.encrypt32(19364),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (19364, 19364)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(19364),
      this.instances3.alice.encrypt32(19364),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 4 (19364, 19360)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(19364),
      this.instances3.alice.encrypt32(19360),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 1 (61440, 2523716364)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(61440),
      this.instances3.alice.encrypt32(2523716364),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 2 (61436, 61440)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(61436),
      this.instances3.alice.encrypt32(61440),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 3 (61440, 61440)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(61440),
      this.instances3.alice.encrypt32(61440),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 4 (61440, 61436)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(61440),
      this.instances3.alice.encrypt32(61436),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (42248, 3250361677)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(42248),
      this.instances3.alice.encrypt32(3250361677),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (42244, 42248)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(42244),
      this.instances3.alice.encrypt32(42248),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (42248, 42248)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(42248),
      this.instances3.alice.encrypt32(42248),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 4 (42248, 42244)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(42248),
      this.instances3.alice.encrypt32(42244),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 1 (45430, 522886914)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(45430),
      this.instances3.alice.encrypt32(522886914),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 2 (45426, 45430)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(45426),
      this.instances3.alice.encrypt32(45430),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 3 (45430, 45430)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(45430),
      this.instances3.alice.encrypt32(45430),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 4 (45430, 45426)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(45430),
      this.instances3.alice.encrypt32(45426),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 1 (3581, 3061947173)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(3581),
      this.instances3.alice.encrypt32(3061947173),
    );
    expect(res).to.equal(3581n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 2 (3577, 3581)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(3577),
      this.instances3.alice.encrypt32(3581),
    );
    expect(res).to.equal(3577n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 3 (3581, 3581)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(3581),
      this.instances3.alice.encrypt32(3581),
    );
    expect(res).to.equal(3581n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 4 (3581, 3577)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(3581),
      this.instances3.alice.encrypt32(3577),
    );
    expect(res).to.equal(3577n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (50723, 1432053137)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(50723),
      this.instances3.alice.encrypt32(1432053137),
    );
    expect(res).to.equal(1432053137n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (50719, 50723)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(50719),
      this.instances3.alice.encrypt32(50723),
    );
    expect(res).to.equal(50723n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (50723, 50723)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(50723),
      this.instances3.alice.encrypt32(50723),
    );
    expect(res).to.equal(50723n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 4 (50723, 50719)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(50723),
      this.instances3.alice.encrypt32(50719),
    );
    expect(res).to.equal(50723n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 1 (2, 65518)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt64(65518),
    );
    expect(res).to.equal(65520n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 2 (22742, 22744)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(22742),
      this.instances3.alice.encrypt64(22744),
    );
    expect(res).to.equal(45486n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 3 (22744, 22744)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(22744),
      this.instances3.alice.encrypt64(22744),
    );
    expect(res).to.equal(45488n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 4 (22744, 22742)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(22744),
      this.instances3.alice.encrypt64(22742),
    );
    expect(res).to.equal(45486n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 1 (8385, 8385)', async function () {
    const res = await this.contract3.sub_euint16_euint64(
      this.instances3.alice.encrypt16(8385),
      this.instances3.alice.encrypt64(8385),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 2 (8385, 8381)', async function () {
    const res = await this.contract3.sub_euint16_euint64(
      this.instances3.alice.encrypt16(8385),
      this.instances3.alice.encrypt64(8381),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 1 (2, 32761)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt64(32761),
    );
    expect(res).to.equal(65522n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 2 (251, 251)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(251),
      this.instances3.alice.encrypt64(251),
    );
    expect(res).to.equal(63001n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 3 (251, 251)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(251),
      this.instances3.alice.encrypt64(251),
    );
    expect(res).to.equal(63001n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 4 (251, 251)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(251),
      this.instances3.alice.encrypt64(251),
    );
    expect(res).to.equal(63001n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 1 (3617, 18445521461849474399)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(3617),
      this.instances3.alice.encrypt64(18445521461849474399),
    );
    expect(res).to.equal(1025n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 2 (3613, 3617)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(3613),
      this.instances3.alice.encrypt64(3617),
    );
    expect(res).to.equal(3585n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 3 (3617, 3617)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(3617),
      this.instances3.alice.encrypt64(3617),
    );
    expect(res).to.equal(3617n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 4 (3617, 3613)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(3617),
      this.instances3.alice.encrypt64(3613),
    );
    expect(res).to.equal(3585n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 1 (3594, 18443975303513898329)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(3594),
      this.instances3.alice.encrypt64(18443975303513898329),
    );
    expect(res).to.equal(18443975303513898843n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 2 (3590, 3594)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(3590),
      this.instances3.alice.encrypt64(3594),
    );
    expect(res).to.equal(3598n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 3 (3594, 3594)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(3594),
      this.instances3.alice.encrypt64(3594),
    );
    expect(res).to.equal(3594n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 4 (3594, 3590)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(3594),
      this.instances3.alice.encrypt64(3590),
    );
    expect(res).to.equal(3598n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 1 (31158, 18442847100019644985)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(31158),
      this.instances3.alice.encrypt64(18442847100019644985),
    );
    expect(res).to.equal(18442847100019663759n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 2 (31154, 31158)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(31154),
      this.instances3.alice.encrypt64(31158),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 3 (31158, 31158)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(31158),
      this.instances3.alice.encrypt64(31158),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 4 (31158, 31154)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(31158),
      this.instances3.alice.encrypt64(31154),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 1 (38256, 18442430973530036421)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(38256),
      this.instances3.alice.encrypt64(18442430973530036421),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 2 (38252, 38256)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(38252),
      this.instances3.alice.encrypt64(38256),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 3 (38256, 38256)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(38256),
      this.instances3.alice.encrypt64(38256),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 4 (38256, 38252)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(38256),
      this.instances3.alice.encrypt64(38252),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 1 (33238, 18440140341099634203)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(33238),
      this.instances3.alice.encrypt64(18440140341099634203),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 2 (33234, 33238)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(33234),
      this.instances3.alice.encrypt64(33238),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 3 (33238, 33238)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(33238),
      this.instances3.alice.encrypt64(33238),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 4 (33238, 33234)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(33238),
      this.instances3.alice.encrypt64(33234),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 1 (17006, 18440111639904808773)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(17006),
      this.instances3.alice.encrypt64(18440111639904808773),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 2 (17002, 17006)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(17002),
      this.instances3.alice.encrypt64(17006),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 3 (17006, 17006)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(17006),
      this.instances3.alice.encrypt64(17006),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 4 (17006, 17002)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(17006),
      this.instances3.alice.encrypt64(17002),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 1 (13915, 18437957835114177845)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(13915),
      this.instances3.alice.encrypt64(18437957835114177845),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 2 (13911, 13915)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(13911),
      this.instances3.alice.encrypt64(13915),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 3 (13915, 13915)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(13915),
      this.instances3.alice.encrypt64(13915),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 4 (13915, 13911)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(13915),
      this.instances3.alice.encrypt64(13911),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 1 (16281, 18438048148950562767)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(16281),
      this.instances3.alice.encrypt64(18438048148950562767),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 2 (16277, 16281)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(16277),
      this.instances3.alice.encrypt64(16281),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 3 (16281, 16281)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(16281),
      this.instances3.alice.encrypt64(16281),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 4 (16281, 16277)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(16281),
      this.instances3.alice.encrypt64(16277),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 1 (18995, 18443093417222594453)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(18995),
      this.instances3.alice.encrypt64(18443093417222594453),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 2 (18991, 18995)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(18991),
      this.instances3.alice.encrypt64(18995),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 3 (18995, 18995)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(18995),
      this.instances3.alice.encrypt64(18995),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 4 (18995, 18991)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(18995),
      this.instances3.alice.encrypt64(18991),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 1 (60945, 18442404212044065569)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(60945),
      this.instances3.alice.encrypt64(18442404212044065569),
    );
    expect(res).to.equal(60945n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 2 (60941, 60945)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(60941),
      this.instances3.alice.encrypt64(60945),
    );
    expect(res).to.equal(60941n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 3 (60945, 60945)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(60945),
      this.instances3.alice.encrypt64(60945),
    );
    expect(res).to.equal(60945n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 4 (60945, 60941)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(60945),
      this.instances3.alice.encrypt64(60941),
    );
    expect(res).to.equal(60941n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 1 (23644, 18446414081706845657)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(23644),
      this.instances3.alice.encrypt64(18446414081706845657),
    );
    expect(res).to.equal(18446414081706845657n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 2 (23640, 23644)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(23640),
      this.instances3.alice.encrypt64(23644),
    );
    expect(res).to.equal(23644n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 3 (23644, 23644)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(23644),
      this.instances3.alice.encrypt64(23644),
    );
    expect(res).to.equal(23644n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 4 (23644, 23640)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(23644),
      this.instances3.alice.encrypt64(23640),
    );
    expect(res).to.equal(23644n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (54451, 6303)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(54451), 6303);
    expect(res).to.equal(60754n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (22056, 22058)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(22056), 22058);
    expect(res).to.equal(44114n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (22058, 22058)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(22058), 22058);
    expect(res).to.equal(44116n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (22058, 22056)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(22058), 22056);
    expect(res).to.equal(44114n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (29751, 3152)', async function () {
    const res = await this.contract3.add_uint16_euint16(29751, this.instances3.alice.encrypt16(3152));
    expect(res).to.equal(32903n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 2 (22056, 22058)', async function () {
    const res = await this.contract3.add_uint16_euint16(22056, this.instances3.alice.encrypt16(22058));
    expect(res).to.equal(44114n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 3 (22058, 22058)', async function () {
    const res = await this.contract3.add_uint16_euint16(22058, this.instances3.alice.encrypt16(22058));
    expect(res).to.equal(44116n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 4 (22058, 22056)', async function () {
    const res = await this.contract3.add_uint16_euint16(22058, this.instances3.alice.encrypt16(22056));
    expect(res).to.equal(44114n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (56955, 56955)', async function () {
    const res = await this.contract3.sub_euint16_uint16(this.instances3.alice.encrypt16(56955), 56955);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 2 (56955, 56951)', async function () {
    const res = await this.contract3.sub_euint16_uint16(this.instances3.alice.encrypt16(56955), 56951);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (56955, 56955)', async function () {
    const res = await this.contract3.sub_uint16_euint16(56955, this.instances3.alice.encrypt16(56955));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (56955, 56951)', async function () {
    const res = await this.contract3.sub_uint16_euint16(56955, this.instances3.alice.encrypt16(56951));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (239, 131)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(239), 131);
    expect(res).to.equal(31309n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (239, 239)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(239), 239);
    expect(res).to.equal(57121n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (239, 239)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(239), 239);
    expect(res).to.equal(57121n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (239, 239)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(239), 239);
    expect(res).to.equal(57121n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (190, 131)', async function () {
    const res = await this.contract3.mul_uint16_euint16(190, this.instances3.alice.encrypt16(131));
    expect(res).to.equal(24890n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 2 (239, 239)', async function () {
    const res = await this.contract3.mul_uint16_euint16(239, this.instances3.alice.encrypt16(239));
    expect(res).to.equal(57121n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 3 (239, 239)', async function () {
    const res = await this.contract3.mul_uint16_euint16(239, this.instances3.alice.encrypt16(239));
    expect(res).to.equal(57121n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 4 (239, 239)', async function () {
    const res = await this.contract3.mul_uint16_euint16(239, this.instances3.alice.encrypt16(239));
    expect(res).to.equal(57121n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (61139, 60988)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(61139), 60988);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (59165, 59169)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(59165), 59169);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (59169, 59169)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(59169), 59169);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (59169, 59165)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(59169), 59165);
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (18856, 18330)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(18856), 18330);
    expect(res).to.equal(526n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 2 (18852, 18856)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(18852), 18856);
    expect(res).to.equal(18852n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 3 (18856, 18856)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(18856), 18856);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 4 (18856, 18852)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(18856), 18852);
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (53936, 62296)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(53936), 62296);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (53932, 53936)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(53932), 53936);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (53936, 53936)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(53936), 53936);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (53936, 53932)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(53936), 53932);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (49783, 62296)', async function () {
    const res = await this.contract3.eq_uint16_euint16(49783, this.instances3.alice.encrypt16(62296));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (53932, 53936)', async function () {
    const res = await this.contract3.eq_uint16_euint16(53932, this.instances3.alice.encrypt16(53936));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 3 (53936, 53936)', async function () {
    const res = await this.contract3.eq_uint16_euint16(53936, this.instances3.alice.encrypt16(53936));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 4 (53936, 53932)', async function () {
    const res = await this.contract3.eq_uint16_euint16(53936, this.instances3.alice.encrypt16(53932));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (64200, 15145)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(64200), 15145);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (17034, 17038)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(17034), 17038);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 3 (17038, 17038)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(17038), 17038);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 4 (17038, 17034)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(17038), 17034);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (35214, 15145)', async function () {
    const res = await this.contract3.ne_uint16_euint16(35214, this.instances3.alice.encrypt16(15145));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (17034, 17038)', async function () {
    const res = await this.contract3.ne_uint16_euint16(17034, this.instances3.alice.encrypt16(17038));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 3 (17038, 17038)', async function () {
    const res = await this.contract3.ne_uint16_euint16(17038, this.instances3.alice.encrypt16(17038));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 4 (17038, 17034)', async function () {
    const res = await this.contract3.ne_uint16_euint16(17038, this.instances3.alice.encrypt16(17034));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (18226, 26413)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(18226), 26413);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (11813, 11817)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(11813), 11817);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (11817, 11817)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(11817), 11817);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 4 (11817, 11813)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(11817), 11813);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (45131, 26413)', async function () {
    const res = await this.contract3.ge_uint16_euint16(45131, this.instances3.alice.encrypt16(26413));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (11813, 11817)', async function () {
    const res = await this.contract3.ge_uint16_euint16(11813, this.instances3.alice.encrypt16(11817));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (11817, 11817)', async function () {
    const res = await this.contract3.ge_uint16_euint16(11817, this.instances3.alice.encrypt16(11817));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 4 (11817, 11813)', async function () {
    const res = await this.contract3.ge_uint16_euint16(11817, this.instances3.alice.encrypt16(11813));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (46712, 65342)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(46712), 65342);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (46708, 46712)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(46708), 46712);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (46712, 46712)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(46712), 46712);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 4 (46712, 46708)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(46712), 46708);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (58255, 65342)', async function () {
    const res = await this.contract3.gt_uint16_euint16(58255, this.instances3.alice.encrypt16(65342));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (46708, 46712)', async function () {
    const res = await this.contract3.gt_uint16_euint16(46708, this.instances3.alice.encrypt16(46712));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (46712, 46712)', async function () {
    const res = await this.contract3.gt_uint16_euint16(46712, this.instances3.alice.encrypt16(46712));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 4 (46712, 46708)', async function () {
    const res = await this.contract3.gt_uint16_euint16(46712, this.instances3.alice.encrypt16(46708));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (9281, 23936)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(9281), 23936);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (9277, 9281)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(9277), 9281);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (9281, 9281)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(9281), 9281);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 4 (9281, 9277)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(9281), 9277);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (49675, 23936)', async function () {
    const res = await this.contract3.le_uint16_euint16(49675, this.instances3.alice.encrypt16(23936));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (9277, 9281)', async function () {
    const res = await this.contract3.le_uint16_euint16(9277, this.instances3.alice.encrypt16(9281));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (9281, 9281)', async function () {
    const res = await this.contract3.le_uint16_euint16(9281, this.instances3.alice.encrypt16(9281));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (9281, 9277)', async function () {
    const res = await this.contract3.le_uint16_euint16(9281, this.instances3.alice.encrypt16(9277));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (44794, 29106)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(44794), 29106);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (23286, 23290)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(23286), 23290);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (23290, 23290)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(23290), 23290);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 4 (23290, 23286)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(23290), 23286);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (11942, 29106)', async function () {
    const res = await this.contract3.lt_uint16_euint16(11942, this.instances3.alice.encrypt16(29106));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (23286, 23290)', async function () {
    const res = await this.contract3.lt_uint16_euint16(23286, this.instances3.alice.encrypt16(23290));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (23290, 23290)', async function () {
    const res = await this.contract3.lt_uint16_euint16(23290, this.instances3.alice.encrypt16(23290));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 4 (23290, 23286)', async function () {
    const res = await this.contract3.lt_uint16_euint16(23290, this.instances3.alice.encrypt16(23286));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (30936, 64470)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(30936), 64470);
    expect(res).to.equal(30936n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (9023, 9027)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(9023), 9027);
    expect(res).to.equal(9023n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (9027, 9027)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(9027), 9027);
    expect(res).to.equal(9027n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 4 (9027, 9023)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(9027), 9023);
    expect(res).to.equal(9023n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (10499, 64470)', async function () {
    const res = await this.contract3.min_uint16_euint16(10499, this.instances3.alice.encrypt16(64470));
    expect(res).to.equal(10499n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (9023, 9027)', async function () {
    const res = await this.contract3.min_uint16_euint16(9023, this.instances3.alice.encrypt16(9027));
    expect(res).to.equal(9023n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (9027, 9027)', async function () {
    const res = await this.contract3.min_uint16_euint16(9027, this.instances3.alice.encrypt16(9027));
    expect(res).to.equal(9027n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (9027, 9023)', async function () {
    const res = await this.contract3.min_uint16_euint16(9027, this.instances3.alice.encrypt16(9023));
    expect(res).to.equal(9023n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (34561, 63895)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(34561), 63895);
    expect(res).to.equal(63895n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (34557, 34561)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(34557), 34561);
    expect(res).to.equal(34561n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (34561, 34561)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(34561), 34561);
    expect(res).to.equal(34561n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (34561, 34557)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(34561), 34557);
    expect(res).to.equal(34561n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (52784, 63895)', async function () {
    const res = await this.contract3.max_uint16_euint16(52784, this.instances3.alice.encrypt16(63895));
    expect(res).to.equal(63895n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (34557, 34561)', async function () {
    const res = await this.contract3.max_uint16_euint16(34557, this.instances3.alice.encrypt16(34561));
    expect(res).to.equal(34561n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (34561, 34561)', async function () {
    const res = await this.contract3.max_uint16_euint16(34561, this.instances3.alice.encrypt16(34561));
    expect(res).to.equal(34561n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (34561, 34557)', async function () {
    const res = await this.contract3.max_uint16_euint16(34561, this.instances3.alice.encrypt16(34557));
    expect(res).to.equal(34561n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 1 (11, 2)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(11),
      this.instances3.alice.encrypt4(2),
    );
    expect(res).to.equal(13n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 3 (5, 5)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(5),
      this.instances3.alice.encrypt4(5),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint32, euint4) => euint32 test 1 (8, 8)', async function () {
    const res = await this.contract3.sub_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint4) => euint32 test 2 (8, 4)', async function () {
    const res = await this.contract3.sub_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 1 (5, 2)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(5),
      this.instances3.alice.encrypt4(2),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 2 (3, 3)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(3),
      this.instances3.alice.encrypt4(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 3 (3, 3)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(3),
      this.instances3.alice.encrypt4(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 4 (3, 3)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(3),
      this.instances3.alice.encrypt4(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 1 (1301921086, 9)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(1301921086),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 2 (5, 9)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(5),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 3 (9, 9)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 4 (9, 5)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(5),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 1 (2406735968, 5)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(2406735968),
      this.instances3.alice.encrypt4(5),
    );
    expect(res).to.equal(2406735973n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 1 (3625477729, 13)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(3625477729),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(3625477740n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 2 (9, 13)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 3 (13, 13)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(13),
      this.instances3.alice.encrypt4(13),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 4 (13, 9)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(13),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 1 (2724048740, 14)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(2724048740),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(10),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(14),
      this.instances3.alice.encrypt4(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(14),
      this.instances3.alice.encrypt4(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 1 (2506657954, 7)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(2506657954),
      this.instances3.alice.encrypt4(7),
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

  it('test operator "ge" overload (euint32, euint4) => ebool test 1 (4064050662, 13)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(4064050662),
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

  it('test operator "gt" overload (euint32, euint4) => ebool test 1 (2933730621, 7)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(2933730621),
      this.instances3.alice.encrypt4(7),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 1 (4064031115, 9)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(4064031115),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(5),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(5),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 1 (3623618060, 5)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(3623618060),
      this.instances3.alice.encrypt4(5),
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

  it('test operator "min" overload (euint32, euint4) => euint32 test 1 (3192029980, 3)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(3192029980),
      this.instances3.alice.encrypt4(3),
    );
    expect(res).to.equal(3n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 1 (681402629, 7)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(681402629),
      this.instances3.alice.encrypt4(7),
    );
    expect(res).to.equal(681402629n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (186, 2)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(186),
      this.instances3.alice.encrypt8(2),
    );
    expect(res).to.equal(188n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (69, 71)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(69),
      this.instances3.alice.encrypt8(71),
    );
    expect(res).to.equal(140n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (71, 71)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(71),
      this.instances3.alice.encrypt8(71),
    );
    expect(res).to.equal(142n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (71, 69)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(71),
      this.instances3.alice.encrypt8(69),
    );
    expect(res).to.equal(140n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (179, 179)', async function () {
    const res = await this.contract3.sub_euint32_euint8(
      this.instances3.alice.encrypt32(179),
      this.instances3.alice.encrypt8(179),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (179, 175)', async function () {
    const res = await this.contract3.sub_euint32_euint8(
      this.instances3.alice.encrypt32(179),
      this.instances3.alice.encrypt8(175),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (107, 2)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(107),
      this.instances3.alice.encrypt8(2),
    );
    expect(res).to.equal(214n);
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

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (2684075699, 16)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(2684075699),
      this.instances3.alice.encrypt8(16),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (12, 16)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(12),
      this.instances3.alice.encrypt8(16),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (16, 16)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(16),
      this.instances3.alice.encrypt8(16),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (16, 12)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(16),
      this.instances3.alice.encrypt8(12),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (3961011805, 118)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(3961011805),
      this.instances4.alice.encrypt8(118),
    );
    expect(res).to.equal(3961011839n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (114, 118)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(114),
      this.instances4.alice.encrypt8(118),
    );
    expect(res).to.equal(118n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (118, 118)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(118),
      this.instances4.alice.encrypt8(118),
    );
    expect(res).to.equal(118n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (118, 114)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(118),
      this.instances4.alice.encrypt8(114),
    );
    expect(res).to.equal(118n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (2615810877, 184)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(2615810877),
      this.instances4.alice.encrypt8(184),
    );
    expect(res).to.equal(2615810949n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (180, 184)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(180),
      this.instances4.alice.encrypt8(184),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (184, 184)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(184),
      this.instances4.alice.encrypt8(184),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (184, 180)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(184),
      this.instances4.alice.encrypt8(180),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (2890164202, 10)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(2890164202),
      this.instances4.alice.encrypt8(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(6),
      this.instances4.alice.encrypt8(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(10),
      this.instances4.alice.encrypt8(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(10),
      this.instances4.alice.encrypt8(6),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (874720156, 10)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(874720156),
      this.instances4.alice.encrypt8(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(6),
      this.instances4.alice.encrypt8(10),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(10),
      this.instances4.alice.encrypt8(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(10),
      this.instances4.alice.encrypt8(6),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (4107105674, 147)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(4107105674),
      this.instances4.alice.encrypt8(147),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (143, 147)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(143),
      this.instances4.alice.encrypt8(147),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (147, 147)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(147),
      this.instances4.alice.encrypt8(147),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 4 (147, 143)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(147),
      this.instances4.alice.encrypt8(143),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (1917780828, 163)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(1917780828),
      this.instances4.alice.encrypt8(163),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (159, 163)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(159),
      this.instances4.alice.encrypt8(163),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (163, 163)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(163),
      this.instances4.alice.encrypt8(163),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 4 (163, 159)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(163),
      this.instances4.alice.encrypt8(159),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (3743657855, 103)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(3743657855),
      this.instances4.alice.encrypt8(103),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (99, 103)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(99),
      this.instances4.alice.encrypt8(103),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (103, 103)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(103),
      this.instances4.alice.encrypt8(103),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (103, 99)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(103),
      this.instances4.alice.encrypt8(99),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (800790361, 82)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(800790361),
      this.instances4.alice.encrypt8(82),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (78, 82)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(78),
      this.instances4.alice.encrypt8(82),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (82, 82)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(82),
      this.instances4.alice.encrypt8(82),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 4 (82, 78)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(82),
      this.instances4.alice.encrypt8(78),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (2128865754, 127)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(2128865754),
      this.instances4.alice.encrypt8(127),
    );
    expect(res).to.equal(127n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (123, 127)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(123),
      this.instances4.alice.encrypt8(127),
    );
    expect(res).to.equal(123n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (127, 127)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(127),
      this.instances4.alice.encrypt8(127),
    );
    expect(res).to.equal(127n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (127, 123)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(127),
      this.instances4.alice.encrypt8(123),
    );
    expect(res).to.equal(123n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (1610798197, 160)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(1610798197),
      this.instances4.alice.encrypt8(160),
    );
    expect(res).to.equal(1610798197n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (156, 160)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(156),
      this.instances4.alice.encrypt8(160),
    );
    expect(res).to.equal(160n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (160, 160)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(160),
      this.instances4.alice.encrypt8(160),
    );
    expect(res).to.equal(160n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 4 (160, 156)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(160),
      this.instances4.alice.encrypt8(156),
    );
    expect(res).to.equal(160n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (40687, 4)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(40687),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(40691n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (18607, 18611)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(18607),
      this.instances4.alice.encrypt16(18611),
    );
    expect(res).to.equal(37218n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (18611, 18611)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(18611),
      this.instances4.alice.encrypt16(18611),
    );
    expect(res).to.equal(37222n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (18611, 18607)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(18611),
      this.instances4.alice.encrypt16(18607),
    );
    expect(res).to.equal(37218n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (47991, 47991)', async function () {
    const res = await this.contract4.sub_euint32_euint16(
      this.instances4.alice.encrypt32(47991),
      this.instances4.alice.encrypt16(47991),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (47991, 47987)', async function () {
    const res = await this.contract4.sub_euint32_euint16(
      this.instances4.alice.encrypt32(47991),
      this.instances4.alice.encrypt16(47987),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (30409, 2)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(30409),
      this.instances4.alice.encrypt16(2),
    );
    expect(res).to.equal(60818n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (152, 152)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(152),
      this.instances4.alice.encrypt16(152),
    );
    expect(res).to.equal(23104n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (152, 152)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(152),
      this.instances4.alice.encrypt16(152),
    );
    expect(res).to.equal(23104n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (152, 152)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(152),
      this.instances4.alice.encrypt16(152),
    );
    expect(res).to.equal(23104n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (1113519013, 16851)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(1113519013),
      this.instances4.alice.encrypt16(16851),
    );
    expect(res).to.equal(16769n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (16847, 16851)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(16847),
      this.instances4.alice.encrypt16(16851),
    );
    expect(res).to.equal(16835n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (16851, 16851)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(16851),
      this.instances4.alice.encrypt16(16851),
    );
    expect(res).to.equal(16851n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (16851, 16847)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(16851),
      this.instances4.alice.encrypt16(16847),
    );
    expect(res).to.equal(16835n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (2441286673, 26427)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(2441286673),
      this.instances4.alice.encrypt16(26427),
    );
    expect(res).to.equal(2441312059n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (26423, 26427)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(26423),
      this.instances4.alice.encrypt16(26427),
    );
    expect(res).to.equal(26431n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 3 (26427, 26427)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(26427),
      this.instances4.alice.encrypt16(26427),
    );
    expect(res).to.equal(26427n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 4 (26427, 26423)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(26427),
      this.instances4.alice.encrypt16(26423),
    );
    expect(res).to.equal(26431n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (2697992454, 47573)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(2697992454),
      this.instances4.alice.encrypt16(47573),
    );
    expect(res).to.equal(2698027219n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (47569, 47573)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(47569),
      this.instances4.alice.encrypt16(47573),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (47573, 47573)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(47573),
      this.instances4.alice.encrypt16(47573),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (47573, 47569)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(47573),
      this.instances4.alice.encrypt16(47569),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (3540063980, 53142)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(3540063980),
      this.instances4.alice.encrypt16(53142),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (53138, 53142)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(53138),
      this.instances4.alice.encrypt16(53142),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (53142, 53142)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(53142),
      this.instances4.alice.encrypt16(53142),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (53142, 53138)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(53142),
      this.instances4.alice.encrypt16(53138),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (2729404223, 63905)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(2729404223),
      this.instances4.alice.encrypt16(63905),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (63901, 63905)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(63901),
      this.instances4.alice.encrypt16(63905),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 3 (63905, 63905)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(63905),
      this.instances4.alice.encrypt16(63905),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 4 (63905, 63901)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(63905),
      this.instances4.alice.encrypt16(63901),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (3202817970, 50829)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(3202817970),
      this.instances4.alice.encrypt16(50829),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (50825, 50829)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(50825),
      this.instances4.alice.encrypt16(50829),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (50829, 50829)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(50829),
      this.instances4.alice.encrypt16(50829),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 4 (50829, 50825)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(50829),
      this.instances4.alice.encrypt16(50825),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (1221805876, 57752)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(1221805876),
      this.instances4.alice.encrypt16(57752),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (57748, 57752)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(57748),
      this.instances4.alice.encrypt16(57752),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (57752, 57752)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(57752),
      this.instances4.alice.encrypt16(57752),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 4 (57752, 57748)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(57752),
      this.instances4.alice.encrypt16(57748),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (1673073080, 44621)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(1673073080),
      this.instances4.alice.encrypt16(44621),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (44617, 44621)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(44617),
      this.instances4.alice.encrypt16(44621),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (44621, 44621)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(44621),
      this.instances4.alice.encrypt16(44621),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (44621, 44617)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(44621),
      this.instances4.alice.encrypt16(44617),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (2528767958, 64360)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(2528767958),
      this.instances4.alice.encrypt16(64360),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (64356, 64360)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(64356),
      this.instances4.alice.encrypt16(64360),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (64360, 64360)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(64360),
      this.instances4.alice.encrypt16(64360),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (64360, 64356)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(64360),
      this.instances4.alice.encrypt16(64356),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (2025213170, 65001)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(2025213170),
      this.instances4.alice.encrypt16(65001),
    );
    expect(res).to.equal(65001n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (64997, 65001)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(64997),
      this.instances4.alice.encrypt16(65001),
    );
    expect(res).to.equal(64997n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (65001, 65001)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(65001),
      this.instances4.alice.encrypt16(65001),
    );
    expect(res).to.equal(65001n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (65001, 64997)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(65001),
      this.instances4.alice.encrypt16(64997),
    );
    expect(res).to.equal(64997n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (4136615954, 9960)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(4136615954),
      this.instances4.alice.encrypt16(9960),
    );
    expect(res).to.equal(4136615954n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (9956, 9960)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(9956),
      this.instances4.alice.encrypt16(9960),
    );
    expect(res).to.equal(9960n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (9960, 9960)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(9960),
      this.instances4.alice.encrypt16(9960),
    );
    expect(res).to.equal(9960n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (9960, 9956)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(9960),
      this.instances4.alice.encrypt16(9956),
    );
    expect(res).to.equal(9960n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (2163343287, 829836787)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(2163343287),
      this.instances4.alice.encrypt32(829836787),
    );
    expect(res).to.equal(2993180074n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (829836783, 829836787)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(829836783),
      this.instances4.alice.encrypt32(829836787),
    );
    expect(res).to.equal(1659673570n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (829836787, 829836787)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(829836787),
      this.instances4.alice.encrypt32(829836787),
    );
    expect(res).to.equal(1659673574n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (829836787, 829836783)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(829836787),
      this.instances4.alice.encrypt32(829836783),
    );
    expect(res).to.equal(1659673570n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (3061205449, 3061205449)', async function () {
    const res = await this.contract4.sub_euint32_euint32(
      this.instances4.alice.encrypt32(3061205449),
      this.instances4.alice.encrypt32(3061205449),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (3061205449, 3061205445)', async function () {
    const res = await this.contract4.sub_euint32_euint32(
      this.instances4.alice.encrypt32(3061205449),
      this.instances4.alice.encrypt32(3061205445),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (42467, 41983)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(42467),
      this.instances4.alice.encrypt32(41983),
    );
    expect(res).to.equal(1782892061n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 2 (41983, 41983)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(41983),
      this.instances4.alice.encrypt32(41983),
    );
    expect(res).to.equal(1762572289n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 3 (41983, 41983)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(41983),
      this.instances4.alice.encrypt32(41983),
    );
    expect(res).to.equal(1762572289n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 4 (41983, 41983)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(41983),
      this.instances4.alice.encrypt32(41983),
    );
    expect(res).to.equal(1762572289n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (749010015, 1458345079)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(749010015),
      this.instances4.alice.encrypt32(1458345079),
    );
    expect(res).to.equal(77894743n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (749010011, 749010015)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(749010011),
      this.instances4.alice.encrypt32(749010015),
    );
    expect(res).to.equal(749010011n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 3 (749010015, 749010015)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(749010015),
      this.instances4.alice.encrypt32(749010015),
    );
    expect(res).to.equal(749010015n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 4 (749010015, 749010011)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(749010015),
      this.instances4.alice.encrypt32(749010011),
    );
    expect(res).to.equal(749010011n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (183558137, 3592149879)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(183558137),
      this.instances4.alice.encrypt32(3592149879),
    );
    expect(res).to.equal(3741048831n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (183558133, 183558137)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(183558133),
      this.instances4.alice.encrypt32(183558137),
    );
    expect(res).to.equal(183558141n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 3 (183558137, 183558137)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(183558137),
      this.instances4.alice.encrypt32(183558137),
    );
    expect(res).to.equal(183558137n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 4 (183558137, 183558133)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(183558137),
      this.instances4.alice.encrypt32(183558133),
    );
    expect(res).to.equal(183558141n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (3727740302, 3007364521)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(3727740302),
      this.instances4.alice.encrypt32(3007364521),
    );
    expect(res).to.equal(1836085287n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (3007364517, 3007364521)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(3007364517),
      this.instances4.alice.encrypt32(3007364521),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (3007364521, 3007364521)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(3007364521),
      this.instances4.alice.encrypt32(3007364521),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (3007364521, 3007364517)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(3007364521),
      this.instances4.alice.encrypt32(3007364517),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (3674813327, 3173886291)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(3674813327),
      this.instances4.alice.encrypt32(3173886291),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (3173886287, 3173886291)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(3173886287),
      this.instances4.alice.encrypt32(3173886291),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (3173886291, 3173886291)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(3173886291),
      this.instances4.alice.encrypt32(3173886291),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (3173886291, 3173886287)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(3173886291),
      this.instances4.alice.encrypt32(3173886287),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (40863923, 1726230330)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(40863923),
      this.instances4.alice.encrypt32(1726230330),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (40863919, 40863923)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(40863919),
      this.instances4.alice.encrypt32(40863923),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 3 (40863923, 40863923)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(40863923),
      this.instances4.alice.encrypt32(40863923),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 4 (40863923, 40863919)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(40863923),
      this.instances4.alice.encrypt32(40863919),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (1970784794, 1596956634)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(1970784794),
      this.instances4.alice.encrypt32(1596956634),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (1596956630, 1596956634)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(1596956630),
      this.instances4.alice.encrypt32(1596956634),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (1596956634, 1596956634)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(1596956634),
      this.instances4.alice.encrypt32(1596956634),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (1596956634, 1596956630)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(1596956634),
      this.instances4.alice.encrypt32(1596956630),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (2212514392, 854163759)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(2212514392),
      this.instances4.alice.encrypt32(854163759),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (854163755, 854163759)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(854163755),
      this.instances4.alice.encrypt32(854163759),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (854163759, 854163759)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(854163759),
      this.instances4.alice.encrypt32(854163759),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 4 (854163759, 854163755)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(854163759),
      this.instances4.alice.encrypt32(854163755),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (989604334, 3011475023)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(989604334),
      this.instances4.alice.encrypt32(3011475023),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (989604330, 989604334)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(989604330),
      this.instances4.alice.encrypt32(989604334),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (989604334, 989604334)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(989604334),
      this.instances4.alice.encrypt32(989604334),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 4 (989604334, 989604330)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(989604334),
      this.instances4.alice.encrypt32(989604330),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (3432727362, 2953663248)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(3432727362),
      this.instances4.alice.encrypt32(2953663248),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (2953663244, 2953663248)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(2953663244),
      this.instances4.alice.encrypt32(2953663248),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (2953663248, 2953663248)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(2953663248),
      this.instances4.alice.encrypt32(2953663248),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (2953663248, 2953663244)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(2953663248),
      this.instances4.alice.encrypt32(2953663244),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (1800381916, 3495424142)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(1800381916),
      this.instances4.alice.encrypt32(3495424142),
    );
    expect(res).to.equal(1800381916n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (1800381912, 1800381916)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(1800381912),
      this.instances4.alice.encrypt32(1800381916),
    );
    expect(res).to.equal(1800381912n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (1800381916, 1800381916)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(1800381916),
      this.instances4.alice.encrypt32(1800381916),
    );
    expect(res).to.equal(1800381916n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (1800381916, 1800381912)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(1800381916),
      this.instances4.alice.encrypt32(1800381912),
    );
    expect(res).to.equal(1800381912n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (2043312979, 3247295293)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(2043312979),
      this.instances4.alice.encrypt32(3247295293),
    );
    expect(res).to.equal(3247295293n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (2043312975, 2043312979)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(2043312975),
      this.instances4.alice.encrypt32(2043312979),
    );
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (2043312979, 2043312979)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(2043312979),
      this.instances4.alice.encrypt32(2043312979),
    );
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (2043312979, 2043312975)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(2043312979),
      this.instances4.alice.encrypt32(2043312975),
    );
    expect(res).to.equal(2043312979n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 1 (2, 4293304478)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(2),
      this.instances4.alice.encrypt64(4293304478),
    );
    expect(res).to.equal(4293304480n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 2 (2093298939, 2093298943)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(2093298939),
      this.instances4.alice.encrypt64(2093298943),
    );
    expect(res).to.equal(4186597882n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 3 (2093298943, 2093298943)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(2093298943),
      this.instances4.alice.encrypt64(2093298943),
    );
    expect(res).to.equal(4186597886n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 4 (2093298943, 2093298939)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(2093298943),
      this.instances4.alice.encrypt64(2093298939),
    );
    expect(res).to.equal(4186597882n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (1018084759, 1018084759)', async function () {
    const res = await this.contract4.sub_euint32_euint64(
      this.instances4.alice.encrypt32(1018084759),
      this.instances4.alice.encrypt64(1018084759),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (1018084759, 1018084755)', async function () {
    const res = await this.contract4.sub_euint32_euint64(
      this.instances4.alice.encrypt32(1018084759),
      this.instances4.alice.encrypt64(1018084755),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (2, 2147115345)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(2),
      this.instances4.alice.encrypt64(2147115345),
    );
    expect(res).to.equal(4294230690n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (46957, 46957)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(46957),
      this.instances4.alice.encrypt64(46957),
    );
    expect(res).to.equal(2204959849n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (46957, 46957)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(46957),
      this.instances4.alice.encrypt64(46957),
    );
    expect(res).to.equal(2204959849n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (46957, 46957)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(46957),
      this.instances4.alice.encrypt64(46957),
    );
    expect(res).to.equal(2204959849n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 1 (365928159, 18445926323394727831)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(365928159),
      this.instances4.alice.encrypt64(18445926323394727831),
    );
    expect(res).to.equal(13110935n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 2 (365928155, 365928159)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(365928155),
      this.instances4.alice.encrypt64(365928159),
    );
    expect(res).to.equal(365928155n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 3 (365928159, 365928159)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(365928159),
      this.instances4.alice.encrypt64(365928159),
    );
    expect(res).to.equal(365928159n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 4 (365928159, 365928155)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(365928159),
      this.instances4.alice.encrypt64(365928155),
    );
    expect(res).to.equal(365928155n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 1 (1857122149, 18440946789011191531)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(1857122149),
      this.instances4.alice.encrypt64(18440946789011191531),
    );
    expect(res).to.equal(18440946789112971247n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 2 (1857122145, 1857122149)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(1857122145),
      this.instances4.alice.encrypt64(1857122149),
    );
    expect(res).to.equal(1857122149n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 3 (1857122149, 1857122149)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(1857122149),
      this.instances4.alice.encrypt64(1857122149),
    );
    expect(res).to.equal(1857122149n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 4 (1857122149, 1857122145)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(1857122149),
      this.instances4.alice.encrypt64(1857122145),
    );
    expect(res).to.equal(1857122149n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (2536876532, 18445343517070927715)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(2536876532),
      this.instances4.alice.encrypt64(18445343517070927715),
    );
    expect(res).to.equal(18445343518833806999n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (2536876528, 2536876532)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(2536876528),
      this.instances4.alice.encrypt64(2536876532),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (2536876532, 2536876532)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(2536876532),
      this.instances4.alice.encrypt64(2536876532),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (2536876532, 2536876528)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(2536876532),
      this.instances4.alice.encrypt64(2536876528),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 1 (2805333302, 18443732701658209223)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(2805333302),
      this.instances4.alice.encrypt64(18443732701658209223),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 2 (2805333298, 2805333302)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(2805333298),
      this.instances4.alice.encrypt64(2805333302),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 3 (2805333302, 2805333302)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(2805333302),
      this.instances4.alice.encrypt64(2805333302),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 4 (2805333302, 2805333298)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(2805333302),
      this.instances4.alice.encrypt64(2805333298),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (734315994, 18437906324621631829)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(734315994),
      this.instances4.alice.encrypt64(18437906324621631829),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (734315990, 734315994)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(734315990),
      this.instances4.alice.encrypt64(734315994),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (734315994, 734315994)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(734315994),
      this.instances4.alice.encrypt64(734315994),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (734315994, 734315990)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(734315994),
      this.instances4.alice.encrypt64(734315990),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 1 (1126093045, 18444900127425225651)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(1126093045),
      this.instances4.alice.encrypt64(18444900127425225651),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 2 (1126093041, 1126093045)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(1126093041),
      this.instances4.alice.encrypt64(1126093045),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 3 (1126093045, 1126093045)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(1126093045),
      this.instances4.alice.encrypt64(1126093045),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 4 (1126093045, 1126093041)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(1126093045),
      this.instances4.alice.encrypt64(1126093041),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 1 (1296765186, 18439742981119094705)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(1296765186),
      this.instances4.alice.encrypt64(18439742981119094705),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 2 (1296765182, 1296765186)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(1296765182),
      this.instances4.alice.encrypt64(1296765186),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 3 (1296765186, 1296765186)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(1296765186),
      this.instances4.alice.encrypt64(1296765186),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 4 (1296765186, 1296765182)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(1296765186),
      this.instances4.alice.encrypt64(1296765182),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 1 (3652038615, 18440248369616077365)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(3652038615),
      this.instances4.alice.encrypt64(18440248369616077365),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 2 (3652038611, 3652038615)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(3652038611),
      this.instances4.alice.encrypt64(3652038615),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 3 (3652038615, 3652038615)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(3652038615),
      this.instances4.alice.encrypt64(3652038615),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 4 (3652038615, 3652038611)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(3652038615),
      this.instances4.alice.encrypt64(3652038611),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 1 (1728686329, 18442541149578902807)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(1728686329),
      this.instances4.alice.encrypt64(18442541149578902807),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 2 (1728686325, 1728686329)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(1728686325),
      this.instances4.alice.encrypt64(1728686329),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 3 (1728686329, 1728686329)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(1728686329),
      this.instances4.alice.encrypt64(1728686329),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 4 (1728686329, 1728686325)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(1728686329),
      this.instances4.alice.encrypt64(1728686325),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 1 (968740054, 18439525226452222255)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(968740054),
      this.instances4.alice.encrypt64(18439525226452222255),
    );
    expect(res).to.equal(968740054n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 2 (968740050, 968740054)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(968740050),
      this.instances4.alice.encrypt64(968740054),
    );
    expect(res).to.equal(968740050n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 3 (968740054, 968740054)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(968740054),
      this.instances4.alice.encrypt64(968740054),
    );
    expect(res).to.equal(968740054n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 4 (968740054, 968740050)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(968740054),
      this.instances4.alice.encrypt64(968740050),
    );
    expect(res).to.equal(968740050n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (340812031, 18443150842651286449)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(340812031),
      this.instances4.alice.encrypt64(18443150842651286449),
    );
    expect(res).to.equal(18443150842651286449n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (340812027, 340812031)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(340812027),
      this.instances4.alice.encrypt64(340812031),
    );
    expect(res).to.equal(340812031n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (340812031, 340812031)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(340812031),
      this.instances4.alice.encrypt64(340812031),
    );
    expect(res).to.equal(340812031n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (340812031, 340812027)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(340812031),
      this.instances4.alice.encrypt64(340812027),
    );
    expect(res).to.equal(340812031n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (1081671644, 1277295402)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(1081671644), 1277295402);
    expect(res).to.equal(2358967046n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (829836783, 829836787)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(829836783), 829836787);
    expect(res).to.equal(1659673570n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (829836787, 829836787)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(829836787), 829836787);
    expect(res).to.equal(1659673574n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (829836787, 829836783)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(829836787), 829836783);
    expect(res).to.equal(1659673570n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (1301306419, 1277295402)', async function () {
    const res = await this.contract4.add_uint32_euint32(1301306419, this.instances4.alice.encrypt32(1277295402));
    expect(res).to.equal(2578601821n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 2 (829836783, 829836787)', async function () {
    const res = await this.contract4.add_uint32_euint32(829836783, this.instances4.alice.encrypt32(829836787));
    expect(res).to.equal(1659673570n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 3 (829836787, 829836787)', async function () {
    const res = await this.contract4.add_uint32_euint32(829836787, this.instances4.alice.encrypt32(829836787));
    expect(res).to.equal(1659673574n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 4 (829836787, 829836783)', async function () {
    const res = await this.contract4.add_uint32_euint32(829836787, this.instances4.alice.encrypt32(829836783));
    expect(res).to.equal(1659673570n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (3061205449, 3061205449)', async function () {
    const res = await this.contract4.sub_euint32_uint32(this.instances4.alice.encrypt32(3061205449), 3061205449);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 2 (3061205449, 3061205445)', async function () {
    const res = await this.contract4.sub_euint32_uint32(this.instances4.alice.encrypt32(3061205449), 3061205445);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (3061205449, 3061205449)', async function () {
    const res = await this.contract4.sub_uint32_euint32(3061205449, this.instances4.alice.encrypt32(3061205449));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 2 (3061205449, 3061205445)', async function () {
    const res = await this.contract4.sub_uint32_euint32(3061205449, this.instances4.alice.encrypt32(3061205445));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (42467, 65037)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(42467), 65037);
    expect(res).to.equal(2761926279n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 2 (41983, 41983)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(41983), 41983);
    expect(res).to.equal(1762572289n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 3 (41983, 41983)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(41983), 41983);
    expect(res).to.equal(1762572289n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 4 (41983, 41983)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(41983), 41983);
    expect(res).to.equal(1762572289n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (17461, 65037)', async function () {
    const res = await this.contract4.mul_uint32_euint32(17461, this.instances4.alice.encrypt32(65037));
    expect(res).to.equal(1135611057n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (41983, 41983)', async function () {
    const res = await this.contract4.mul_uint32_euint32(41983, this.instances4.alice.encrypt32(41983));
    expect(res).to.equal(1762572289n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (41983, 41983)', async function () {
    const res = await this.contract4.mul_uint32_euint32(41983, this.instances4.alice.encrypt32(41983));
    expect(res).to.equal(1762572289n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (41983, 41983)', async function () {
    const res = await this.contract4.mul_uint32_euint32(41983, this.instances4.alice.encrypt32(41983));
    expect(res).to.equal(1762572289n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (1945845245, 755191882)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(1945845245), 755191882);
    expect(res).to.equal(2n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (1945845241, 1945845245)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(1945845241), 1945845245);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (1945845245, 1945845245)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(1945845245), 1945845245);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (1945845245, 1945845241)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(1945845245), 1945845241);
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (2521442787, 706504920)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(2521442787), 706504920);
    expect(res).to.equal(401928027n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 2 (2521442783, 2521442787)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(2521442783), 2521442787);
    expect(res).to.equal(2521442783n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 3 (2521442787, 2521442787)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(2521442787), 2521442787);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 4 (2521442787, 2521442783)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(2521442787), 2521442783);
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (3674813327, 340447461)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(3674813327), 340447461);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (3173886287, 3173886291)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(3173886287), 3173886291);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 3 (3173886291, 3173886291)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(3173886291), 3173886291);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 4 (3173886291, 3173886287)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(3173886291), 3173886287);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (630917914, 340447461)', async function () {
    const res = await this.contract4.eq_uint32_euint32(630917914, this.instances4.alice.encrypt32(340447461));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (3173886287, 3173886291)', async function () {
    const res = await this.contract4.eq_uint32_euint32(3173886287, this.instances4.alice.encrypt32(3173886291));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 3 (3173886291, 3173886291)', async function () {
    const res = await this.contract4.eq_uint32_euint32(3173886291, this.instances4.alice.encrypt32(3173886291));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 4 (3173886291, 3173886287)', async function () {
    const res = await this.contract4.eq_uint32_euint32(3173886291, this.instances4.alice.encrypt32(3173886287));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (40863923, 740881042)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(40863923), 740881042);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (40863919, 40863923)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(40863919), 40863923);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 3 (40863923, 40863923)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(40863923), 40863923);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 4 (40863923, 40863919)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(40863923), 40863919);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (3944008003, 740881042)', async function () {
    const res = await this.contract4.ne_uint32_euint32(3944008003, this.instances4.alice.encrypt32(740881042));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (40863919, 40863923)', async function () {
    const res = await this.contract4.ne_uint32_euint32(40863919, this.instances4.alice.encrypt32(40863923));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 3 (40863923, 40863923)', async function () {
    const res = await this.contract4.ne_uint32_euint32(40863923, this.instances4.alice.encrypt32(40863923));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 4 (40863923, 40863919)', async function () {
    const res = await this.contract4.ne_uint32_euint32(40863923, this.instances4.alice.encrypt32(40863919));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (1970784794, 2198890323)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(1970784794), 2198890323);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (1596956630, 1596956634)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(1596956630), 1596956634);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (1596956634, 1596956634)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(1596956634), 1596956634);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 4 (1596956634, 1596956630)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(1596956634), 1596956630);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (3128599756, 2198890323)', async function () {
    const res = await this.contract4.ge_uint32_euint32(3128599756, this.instances4.alice.encrypt32(2198890323));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (1596956630, 1596956634)', async function () {
    const res = await this.contract4.ge_uint32_euint32(1596956630, this.instances4.alice.encrypt32(1596956634));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (1596956634, 1596956634)', async function () {
    const res = await this.contract4.ge_uint32_euint32(1596956634, this.instances4.alice.encrypt32(1596956634));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (1596956634, 1596956630)', async function () {
    const res = await this.contract4.ge_uint32_euint32(1596956634, this.instances4.alice.encrypt32(1596956630));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (2212514392, 3587264713)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(2212514392), 3587264713);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (854163755, 854163759)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(854163755), 854163759);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (854163759, 854163759)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(854163759), 854163759);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 4 (854163759, 854163755)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(854163759), 854163755);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (405003775, 3587264713)', async function () {
    const res = await this.contract4.gt_uint32_euint32(405003775, this.instances4.alice.encrypt32(3587264713));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (854163755, 854163759)', async function () {
    const res = await this.contract4.gt_uint32_euint32(854163755, this.instances4.alice.encrypt32(854163759));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (854163759, 854163759)', async function () {
    const res = await this.contract4.gt_uint32_euint32(854163759, this.instances4.alice.encrypt32(854163759));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 4 (854163759, 854163755)', async function () {
    const res = await this.contract4.gt_uint32_euint32(854163759, this.instances4.alice.encrypt32(854163755));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (989604334, 2400461325)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(989604334), 2400461325);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (989604330, 989604334)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(989604330), 989604334);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (989604334, 989604334)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(989604334), 989604334);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 4 (989604334, 989604330)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(989604334), 989604330);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (3171491075, 2400461325)', async function () {
    const res = await this.contract4.le_uint32_euint32(3171491075, this.instances4.alice.encrypt32(2400461325));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (989604330, 989604334)', async function () {
    const res = await this.contract4.le_uint32_euint32(989604330, this.instances4.alice.encrypt32(989604334));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (989604334, 989604334)', async function () {
    const res = await this.contract4.le_uint32_euint32(989604334, this.instances4.alice.encrypt32(989604334));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 4 (989604334, 989604330)', async function () {
    const res = await this.contract4.le_uint32_euint32(989604334, this.instances4.alice.encrypt32(989604330));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (3432727362, 340267218)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(3432727362), 340267218);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (2953663244, 2953663248)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(2953663244), 2953663248);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (2953663248, 2953663248)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(2953663248), 2953663248);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 4 (2953663248, 2953663244)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(2953663248), 2953663244);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (254860119, 340267218)', async function () {
    const res = await this.contract4.lt_uint32_euint32(254860119, this.instances4.alice.encrypt32(340267218));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (2953663244, 2953663248)', async function () {
    const res = await this.contract4.lt_uint32_euint32(2953663244, this.instances4.alice.encrypt32(2953663248));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (2953663248, 2953663248)', async function () {
    const res = await this.contract4.lt_uint32_euint32(2953663248, this.instances4.alice.encrypt32(2953663248));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 4 (2953663248, 2953663244)', async function () {
    const res = await this.contract4.lt_uint32_euint32(2953663248, this.instances4.alice.encrypt32(2953663244));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (1800381916, 3488347882)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(1800381916), 3488347882);
    expect(res).to.equal(1800381916n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (1800381912, 1800381916)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(1800381912), 1800381916);
    expect(res).to.equal(1800381912n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (1800381916, 1800381916)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(1800381916), 1800381916);
    expect(res).to.equal(1800381916n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 4 (1800381916, 1800381912)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(1800381916), 1800381912);
    expect(res).to.equal(1800381912n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (2638635782, 3488347882)', async function () {
    const res = await this.contract4.min_uint32_euint32(2638635782, this.instances4.alice.encrypt32(3488347882));
    expect(res).to.equal(2638635782n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (1800381912, 1800381916)', async function () {
    const res = await this.contract4.min_uint32_euint32(1800381912, this.instances4.alice.encrypt32(1800381916));
    expect(res).to.equal(1800381912n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (1800381916, 1800381916)', async function () {
    const res = await this.contract4.min_uint32_euint32(1800381916, this.instances4.alice.encrypt32(1800381916));
    expect(res).to.equal(1800381916n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 4 (1800381916, 1800381912)', async function () {
    const res = await this.contract4.min_uint32_euint32(1800381916, this.instances4.alice.encrypt32(1800381912));
    expect(res).to.equal(1800381912n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (2043312979, 706283813)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(2043312979), 706283813);
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (2043312975, 2043312979)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(2043312975), 2043312979);
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (2043312979, 2043312979)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(2043312979), 2043312979);
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 4 (2043312979, 2043312975)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(2043312979), 2043312975);
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (1094445398, 706283813)', async function () {
    const res = await this.contract4.max_uint32_euint32(1094445398, this.instances4.alice.encrypt32(706283813));
    expect(res).to.equal(1094445398n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (2043312975, 2043312979)', async function () {
    const res = await this.contract4.max_uint32_euint32(2043312975, this.instances4.alice.encrypt32(2043312979));
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (2043312979, 2043312979)', async function () {
    const res = await this.contract4.max_uint32_euint32(2043312979, this.instances4.alice.encrypt32(2043312979));
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 4 (2043312979, 2043312975)', async function () {
    const res = await this.contract4.max_uint32_euint32(2043312979, this.instances4.alice.encrypt32(2043312975));
    expect(res).to.equal(2043312979n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 1 (9, 2)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(9),
      this.instances4.alice.encrypt4(2),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 2 (6, 8)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(6),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 3 (5, 5)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(5),
      this.instances4.alice.encrypt4(5),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 4 (8, 6)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(6),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "sub" overload (euint64, euint4) => euint64 test 1 (8, 8)', async function () {
    const res = await this.contract4.sub_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint4) => euint64 test 2 (8, 4)', async function () {
    const res = await this.contract4.sub_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 1 (5, 2)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(5),
      this.instances4.alice.encrypt4(2),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 2 (3, 5)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(3),
      this.instances4.alice.encrypt4(5),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 3 (3, 3)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(3),
      this.instances4.alice.encrypt4(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 4 (5, 3)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(5),
      this.instances4.alice.encrypt4(3),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 1 (18442416087216426369, 12)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(18442416087216426369),
      this.instances4.alice.encrypt4(12),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 2 (8, 12)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(12),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 3 (12, 12)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(12),
      this.instances4.alice.encrypt4(12),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 4 (12, 8)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(12),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 1 (18445233461885169423, 12)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(18445233461885169423),
      this.instances4.alice.encrypt4(12),
    );
    expect(res).to.equal(18445233461885169423n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 2 (8, 12)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(12),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 3 (12, 12)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(12),
      this.instances4.alice.encrypt4(12),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 4 (12, 8)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(12),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 1 (18444022948851042337, 14)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(18444022948851042337),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(18444022948851042351n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 2 (10, 14)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 3 (14, 14)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(14),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 4 (14, 10)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(14),
      this.instances4.alice.encrypt4(10),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 1 (18439121948970600941, 14)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(18439121948970600941),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(14),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(14),
      this.instances4.alice.encrypt4(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 1 (18442748864669273899, 3)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(18442748864669273899),
      this.instances4.alice.encrypt4(3),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 1 (18438518295027982843, 12)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(18438518295027982843),
      this.instances4.alice.encrypt4(12),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(12),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(12),
      this.instances4.alice.encrypt4(12),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(12),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 1 (18445028672073920305, 11)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(18445028672073920305),
      this.instances4.alice.encrypt4(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(7),
      this.instances4.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(11),
      this.instances4.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(11),
      this.instances4.alice.encrypt4(7),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 1 (18446127513373335953, 14)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(18446127513373335953),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(14),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(14),
      this.instances4.alice.encrypt4(10),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 1 (18440890716948813157, 11)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(18440890716948813157),
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

  it('test operator "min" overload (euint64, euint4) => euint64 test 1 (18437774930056492317, 14)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(18437774930056492317),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 2 (10, 14)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 3 (14, 14)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(14),
      this.instances4.alice.encrypt4(14),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 4 (14, 10)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(14),
      this.instances4.alice.encrypt4(10),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 1 (18441711596093702931, 1)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(18441711596093702931),
      this.instances4.alice.encrypt4(1),
    );
    expect(res).to.equal(18441711596093702931n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 1 (129, 2)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(129),
      this.instances4.alice.encrypt8(2),
    );
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 2 (98, 102)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(98),
      this.instances4.alice.encrypt8(102),
    );
    expect(res).to.equal(200n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 3 (102, 102)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(102),
      this.instances4.alice.encrypt8(102),
    );
    expect(res).to.equal(204n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 4 (102, 98)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(102),
      this.instances4.alice.encrypt8(98),
    );
    expect(res).to.equal(200n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 1 (101, 101)', async function () {
    const res = await this.contract4.sub_euint64_euint8(
      this.instances4.alice.encrypt64(101),
      this.instances4.alice.encrypt8(101),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 2 (101, 97)', async function () {
    const res = await this.contract4.sub_euint64_euint8(
      this.instances4.alice.encrypt64(101),
      this.instances4.alice.encrypt8(97),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 1 (65, 2)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(65),
      this.instances4.alice.encrypt8(2),
    );
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 2 (15, 15)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(15),
      this.instances4.alice.encrypt8(15),
    );
    expect(res).to.equal(225n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 3 (15, 15)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(15),
      this.instances4.alice.encrypt8(15),
    );
    expect(res).to.equal(225n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 4 (15, 15)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(15),
      this.instances4.alice.encrypt8(15),
    );
    expect(res).to.equal(225n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 1 (18438525099029165039, 71)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(18438525099029165039),
      this.instances4.alice.encrypt8(71),
    );
    expect(res).to.equal(71n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 2 (67, 71)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(67),
      this.instances4.alice.encrypt8(71),
    );
    expect(res).to.equal(67n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 3 (71, 71)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(71),
      this.instances4.alice.encrypt8(71),
    );
    expect(res).to.equal(71n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 4 (71, 67)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(71),
      this.instances4.alice.encrypt8(67),
    );
    expect(res).to.equal(67n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 1 (18445900286060653541, 17)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(18445900286060653541),
      this.instances4.alice.encrypt8(17),
    );
    expect(res).to.equal(18445900286060653557n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 2 (13, 17)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(13),
      this.instances4.alice.encrypt8(17),
    );
    expect(res).to.equal(29n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 3 (17, 17)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(17),
      this.instances4.alice.encrypt8(17),
    );
    expect(res).to.equal(17n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 4 (17, 13)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(17),
      this.instances4.alice.encrypt8(13),
    );
    expect(res).to.equal(29n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 1 (18439418694946486147, 205)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(18439418694946486147),
      this.instances4.alice.encrypt8(205),
    );
    expect(res).to.equal(18439418694946486094n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 2 (201, 205)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(201),
      this.instances4.alice.encrypt8(205),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 3 (205, 205)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(205),
      this.instances4.alice.encrypt8(205),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 4 (205, 201)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(205),
      this.instances4.alice.encrypt8(201),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 1 (18441725985045807501, 52)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(18441725985045807501),
      this.instances4.alice.encrypt8(52),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 2 (48, 52)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(48),
      this.instances4.alice.encrypt8(52),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 3 (52, 52)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(52),
      this.instances4.alice.encrypt8(52),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 4 (52, 48)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(52),
      this.instances4.alice.encrypt8(48),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 1 (18437879735793971605, 183)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(18437879735793971605),
      this.instances4.alice.encrypt8(183),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 2 (179, 183)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(179),
      this.instances4.alice.encrypt8(183),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 3 (183, 183)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(183),
      this.instances4.alice.encrypt8(183),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 4 (183, 179)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(183),
      this.instances4.alice.encrypt8(179),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 1 (18439034739003983767, 245)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(18439034739003983767),
      this.instances4.alice.encrypt8(245),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 2 (241, 245)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(241),
      this.instances4.alice.encrypt8(245),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 3 (245, 245)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(245),
      this.instances4.alice.encrypt8(245),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 4 (245, 241)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(245),
      this.instances4.alice.encrypt8(241),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 1 (18444472755157488819, 101)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(18444472755157488819),
      this.instances4.alice.encrypt8(101),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 2 (97, 101)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(97),
      this.instances4.alice.encrypt8(101),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 3 (101, 101)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(101),
      this.instances4.alice.encrypt8(101),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 4 (101, 97)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(101),
      this.instances4.alice.encrypt8(97),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 1 (18446424442125229129, 19)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(18446424442125229129),
      this.instances5.alice.encrypt8(19),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 2 (15, 19)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(15),
      this.instances5.alice.encrypt8(19),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 3 (19, 19)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(19),
      this.instances5.alice.encrypt8(19),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 4 (19, 15)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(19),
      this.instances5.alice.encrypt8(15),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 1 (18445717540030063977, 27)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(18445717540030063977),
      this.instances5.alice.encrypt8(27),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 2 (23, 27)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(23),
      this.instances5.alice.encrypt8(27),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 3 (27, 27)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(27),
      this.instances5.alice.encrypt8(27),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 4 (27, 23)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(27),
      this.instances5.alice.encrypt8(23),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 1 (18441639004244956281, 36)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(18441639004244956281),
      this.instances5.alice.encrypt8(36),
    );
    expect(res).to.equal(36n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 2 (32, 36)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(32),
      this.instances5.alice.encrypt8(36),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 3 (36, 36)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(36),
      this.instances5.alice.encrypt8(36),
    );
    expect(res).to.equal(36n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 4 (36, 32)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(36),
      this.instances5.alice.encrypt8(32),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 1 (18442614857754346699, 3)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(18442614857754346699),
      this.instances5.alice.encrypt8(3),
    );
    expect(res).to.equal(18442614857754346699n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 1 (65532, 2)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(65532),
      this.instances5.alice.encrypt16(2),
    );
    expect(res).to.equal(65534n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 2 (23282, 23284)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(23282),
      this.instances5.alice.encrypt16(23284),
    );
    expect(res).to.equal(46566n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 3 (23284, 23284)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(23284),
      this.instances5.alice.encrypt16(23284),
    );
    expect(res).to.equal(46568n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 4 (23284, 23282)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(23284),
      this.instances5.alice.encrypt16(23282),
    );
    expect(res).to.equal(46566n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 1 (25338, 25338)', async function () {
    const res = await this.contract5.sub_euint64_euint16(
      this.instances5.alice.encrypt64(25338),
      this.instances5.alice.encrypt16(25338),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 2 (25338, 25334)', async function () {
    const res = await this.contract5.sub_euint64_euint16(
      this.instances5.alice.encrypt64(25338),
      this.instances5.alice.encrypt16(25334),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 1 (32765, 2)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(32765),
      this.instances5.alice.encrypt16(2),
    );
    expect(res).to.equal(65530n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 2 (163, 163)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(163),
      this.instances5.alice.encrypt16(163),
    );
    expect(res).to.equal(26569n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 3 (163, 163)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(163),
      this.instances5.alice.encrypt16(163),
    );
    expect(res).to.equal(26569n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 4 (163, 163)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(163),
      this.instances5.alice.encrypt16(163),
    );
    expect(res).to.equal(26569n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 1 (18444131025157690381, 45908)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(18444131025157690381),
      this.instances5.alice.encrypt16(45908),
    );
    expect(res).to.equal(4100n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 2 (45904, 45908)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(45904),
      this.instances5.alice.encrypt16(45908),
    );
    expect(res).to.equal(45904n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 3 (45908, 45908)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(45908),
      this.instances5.alice.encrypt16(45908),
    );
    expect(res).to.equal(45908n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 4 (45908, 45904)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(45908),
      this.instances5.alice.encrypt16(45904),
    );
    expect(res).to.equal(45904n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 1 (18445329388989380329, 5327)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(18445329388989380329),
      this.instances5.alice.encrypt16(5327),
    );
    expect(res).to.equal(18445329388989380335n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 2 (5323, 5327)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(5323),
      this.instances5.alice.encrypt16(5327),
    );
    expect(res).to.equal(5327n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 3 (5327, 5327)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(5327),
      this.instances5.alice.encrypt16(5327),
    );
    expect(res).to.equal(5327n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 4 (5327, 5323)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(5327),
      this.instances5.alice.encrypt16(5323),
    );
    expect(res).to.equal(5327n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 1 (18443398919380985157, 45655)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(18443398919380985157),
      this.instances5.alice.encrypt16(45655),
    );
    expect(res).to.equal(18443398919380948754n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 2 (45651, 45655)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(45651),
      this.instances5.alice.encrypt16(45655),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 3 (45655, 45655)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(45655),
      this.instances5.alice.encrypt16(45655),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 4 (45655, 45651)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(45655),
      this.instances5.alice.encrypt16(45651),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 1 (18444068251063204057, 45460)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(18444068251063204057),
      this.instances5.alice.encrypt16(45460),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 2 (45456, 45460)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(45456),
      this.instances5.alice.encrypt16(45460),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 3 (45460, 45460)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(45460),
      this.instances5.alice.encrypt16(45460),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 4 (45460, 45456)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(45460),
      this.instances5.alice.encrypt16(45456),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 1 (18439766867707518465, 19155)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(18439766867707518465),
      this.instances5.alice.encrypt16(19155),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 2 (19151, 19155)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(19151),
      this.instances5.alice.encrypt16(19155),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 3 (19155, 19155)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(19155),
      this.instances5.alice.encrypt16(19155),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 4 (19155, 19151)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(19155),
      this.instances5.alice.encrypt16(19151),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 1 (18440840766291159069, 55918)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(18440840766291159069),
      this.instances5.alice.encrypt16(55918),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 2 (55914, 55918)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(55914),
      this.instances5.alice.encrypt16(55918),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 3 (55918, 55918)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(55918),
      this.instances5.alice.encrypt16(55918),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 4 (55918, 55914)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(55918),
      this.instances5.alice.encrypt16(55914),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 1 (18446239272421398075, 52612)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(18446239272421398075),
      this.instances5.alice.encrypt16(52612),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 2 (52608, 52612)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(52608),
      this.instances5.alice.encrypt16(52612),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 3 (52612, 52612)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(52612),
      this.instances5.alice.encrypt16(52612),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 4 (52612, 52608)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(52612),
      this.instances5.alice.encrypt16(52608),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 1 (18442809234541757919, 39205)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(18442809234541757919),
      this.instances5.alice.encrypt16(39205),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 2 (39201, 39205)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(39201),
      this.instances5.alice.encrypt16(39205),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 3 (39205, 39205)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(39205),
      this.instances5.alice.encrypt16(39205),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 4 (39205, 39201)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(39205),
      this.instances5.alice.encrypt16(39201),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 1 (18443664339829863559, 39059)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(18443664339829863559),
      this.instances5.alice.encrypt16(39059),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 2 (39055, 39059)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(39055),
      this.instances5.alice.encrypt16(39059),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 3 (39059, 39059)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(39059),
      this.instances5.alice.encrypt16(39059),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 4 (39059, 39055)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(39059),
      this.instances5.alice.encrypt16(39055),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 1 (18440273866551894469, 42779)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(18440273866551894469),
      this.instances5.alice.encrypt16(42779),
    );
    expect(res).to.equal(42779n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 2 (42775, 42779)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(42775),
      this.instances5.alice.encrypt16(42779),
    );
    expect(res).to.equal(42775n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 3 (42779, 42779)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(42779),
      this.instances5.alice.encrypt16(42779),
    );
    expect(res).to.equal(42779n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 4 (42779, 42775)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(42779),
      this.instances5.alice.encrypt16(42775),
    );
    expect(res).to.equal(42775n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (18439537518803733101, 41315)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(18439537518803733101),
      this.instances5.alice.encrypt16(41315),
    );
    expect(res).to.equal(18439537518803733101n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (41311, 41315)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(41311),
      this.instances5.alice.encrypt16(41315),
    );
    expect(res).to.equal(41315n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (41315, 41315)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(41315),
      this.instances5.alice.encrypt16(41315),
    );
    expect(res).to.equal(41315n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (41315, 41311)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(41315),
      this.instances5.alice.encrypt16(41311),
    );
    expect(res).to.equal(41315n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 1 (4293362093, 2)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(4293362093),
      this.instances5.alice.encrypt32(2),
    );
    expect(res).to.equal(4293362095n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 2 (1194292230, 1194292232)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(1194292230),
      this.instances5.alice.encrypt32(1194292232),
    );
    expect(res).to.equal(2388584462n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 3 (1194292232, 1194292232)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(1194292232),
      this.instances5.alice.encrypt32(1194292232),
    );
    expect(res).to.equal(2388584464n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 4 (1194292232, 1194292230)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(1194292232),
      this.instances5.alice.encrypt32(1194292230),
    );
    expect(res).to.equal(2388584462n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 1 (624820999, 624820999)', async function () {
    const res = await this.contract5.sub_euint64_euint32(
      this.instances5.alice.encrypt64(624820999),
      this.instances5.alice.encrypt32(624820999),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 2 (624820999, 624820995)', async function () {
    const res = await this.contract5.sub_euint64_euint32(
      this.instances5.alice.encrypt64(624820999),
      this.instances5.alice.encrypt32(624820995),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 1 (2147410658, 2)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(2147410658),
      this.instances5.alice.encrypt32(2),
    );
    expect(res).to.equal(4294821316n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 2 (32881, 32881)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(32881),
      this.instances5.alice.encrypt32(32881),
    );
    expect(res).to.equal(1081160161n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 3 (32881, 32881)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(32881),
      this.instances5.alice.encrypt32(32881),
    );
    expect(res).to.equal(1081160161n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 4 (32881, 32881)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(32881),
      this.instances5.alice.encrypt32(32881),
    );
    expect(res).to.equal(1081160161n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 1 (18444449709394842073, 3823819229)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(18444449709394842073),
      this.instances5.alice.encrypt32(3823819229),
    );
    expect(res).to.equal(2718454233n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 2 (3823819225, 3823819229)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(3823819225),
      this.instances5.alice.encrypt32(3823819229),
    );
    expect(res).to.equal(3823819225n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 3 (3823819229, 3823819229)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(3823819229),
      this.instances5.alice.encrypt32(3823819229),
    );
    expect(res).to.equal(3823819229n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 4 (3823819229, 3823819225)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(3823819229),
      this.instances5.alice.encrypt32(3823819225),
    );
    expect(res).to.equal(3823819225n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 1 (18444852954550836631, 3550708762)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(18444852954550836631),
      this.instances5.alice.encrypt32(3550708762),
    );
    expect(res).to.equal(18444852955920440735n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 2 (3550708758, 3550708762)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(3550708758),
      this.instances5.alice.encrypt32(3550708762),
    );
    expect(res).to.equal(3550708766n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 3 (3550708762, 3550708762)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(3550708762),
      this.instances5.alice.encrypt32(3550708762),
    );
    expect(res).to.equal(3550708762n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 4 (3550708762, 3550708758)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(3550708762),
      this.instances5.alice.encrypt32(3550708758),
    );
    expect(res).to.equal(3550708766n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 1 (18445532500811503869, 2578227181)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(18445532500811503869),
      this.instances5.alice.encrypt32(2578227181),
    );
    expect(res).to.equal(18445532498506434320n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 2 (2578227177, 2578227181)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(2578227177),
      this.instances5.alice.encrypt32(2578227181),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 3 (2578227181, 2578227181)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(2578227181),
      this.instances5.alice.encrypt32(2578227181),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 4 (2578227181, 2578227177)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(2578227181),
      this.instances5.alice.encrypt32(2578227177),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 1 (18444814847193612111, 3675625287)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(18444814847193612111),
      this.instances5.alice.encrypt32(3675625287),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 2 (3675625283, 3675625287)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(3675625283),
      this.instances5.alice.encrypt32(3675625287),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 3 (3675625287, 3675625287)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(3675625287),
      this.instances5.alice.encrypt32(3675625287),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 4 (3675625287, 3675625283)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(3675625287),
      this.instances5.alice.encrypt32(3675625283),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 1 (18442332740658370363, 1304399628)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(18442332740658370363),
      this.instances5.alice.encrypt32(1304399628),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 2 (1304399624, 1304399628)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(1304399624),
      this.instances5.alice.encrypt32(1304399628),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 3 (1304399628, 1304399628)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(1304399628),
      this.instances5.alice.encrypt32(1304399628),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 4 (1304399628, 1304399624)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(1304399628),
      this.instances5.alice.encrypt32(1304399624),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 1 (18443677328726027173, 2889413053)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(18443677328726027173),
      this.instances5.alice.encrypt32(2889413053),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 2 (2889413049, 2889413053)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(2889413049),
      this.instances5.alice.encrypt32(2889413053),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 3 (2889413053, 2889413053)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(2889413053),
      this.instances5.alice.encrypt32(2889413053),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 4 (2889413053, 2889413049)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(2889413053),
      this.instances5.alice.encrypt32(2889413049),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 1 (18445619408455524051, 2595159918)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(18445619408455524051),
      this.instances5.alice.encrypt32(2595159918),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 2 (2595159914, 2595159918)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(2595159914),
      this.instances5.alice.encrypt32(2595159918),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 3 (2595159918, 2595159918)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(2595159918),
      this.instances5.alice.encrypt32(2595159918),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 4 (2595159918, 2595159914)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(2595159918),
      this.instances5.alice.encrypt32(2595159914),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (18439009396144568585, 2271345781)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(18439009396144568585),
      this.instances5.alice.encrypt32(2271345781),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (2271345777, 2271345781)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(2271345777),
      this.instances5.alice.encrypt32(2271345781),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (2271345781, 2271345781)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(2271345781),
      this.instances5.alice.encrypt32(2271345781),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (2271345781, 2271345777)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(2271345781),
      this.instances5.alice.encrypt32(2271345777),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (18439663977987842087, 2158000734)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(18439663977987842087),
      this.instances5.alice.encrypt32(2158000734),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (2158000730, 2158000734)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(2158000730),
      this.instances5.alice.encrypt32(2158000734),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (2158000734, 2158000734)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(2158000734),
      this.instances5.alice.encrypt32(2158000734),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (2158000734, 2158000730)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(2158000734),
      this.instances5.alice.encrypt32(2158000730),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (18442182412102668107, 657148413)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(18442182412102668107),
      this.instances5.alice.encrypt32(657148413),
    );
    expect(res).to.equal(657148413n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (657148409, 657148413)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(657148409),
      this.instances5.alice.encrypt32(657148413),
    );
    expect(res).to.equal(657148409n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (657148413, 657148413)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(657148413),
      this.instances5.alice.encrypt32(657148413),
    );
    expect(res).to.equal(657148413n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (657148413, 657148409)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(657148413),
      this.instances5.alice.encrypt32(657148409),
    );
    expect(res).to.equal(657148409n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (18439662390589911437, 4249693397)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(18439662390589911437),
      this.instances5.alice.encrypt32(4249693397),
    );
    expect(res).to.equal(18439662390589911437n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (4249693393, 4249693397)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(4249693393),
      this.instances5.alice.encrypt32(4249693397),
    );
    expect(res).to.equal(4249693397n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (4249693397, 4249693397)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(4249693397),
      this.instances5.alice.encrypt32(4249693397),
    );
    expect(res).to.equal(4249693397n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (4249693397, 4249693393)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(4249693397),
      this.instances5.alice.encrypt32(4249693393),
    );
    expect(res).to.equal(4249693397n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (9223329218882461797, 9219964371310000511)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(9223329218882461797),
      this.instances5.alice.encrypt64(9219964371310000511),
    );
    expect(res).to.equal(18443293590192462308n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (9219964371310000509, 9219964371310000511)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(9219964371310000509),
      this.instances5.alice.encrypt64(9219964371310000511),
    );
    expect(res).to.equal(18439928742620001020n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (9219964371310000511, 9219964371310000511)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(9219964371310000511),
      this.instances5.alice.encrypt64(9219964371310000511),
    );
    expect(res).to.equal(18439928742620001022n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (9219964371310000511, 9219964371310000509)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(9219964371310000511),
      this.instances5.alice.encrypt64(9219964371310000509),
    );
    expect(res).to.equal(18439928742620001020n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (18445117613821089157, 18445117613821089157)', async function () {
    const res = await this.contract5.sub_euint64_euint64(
      this.instances5.alice.encrypt64(18445117613821089157),
      this.instances5.alice.encrypt64(18445117613821089157),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (18445117613821089157, 18445117613821089153)', async function () {
    const res = await this.contract5.sub_euint64_euint64(
      this.instances5.alice.encrypt64(18445117613821089157),
      this.instances5.alice.encrypt64(18445117613821089153),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (4294635170, 4293288604)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(4294635170),
      this.instances5.alice.encrypt64(4293288604),
    );
    expect(res).to.equal(18438108233698602680n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(4293288604),
      this.instances5.alice.encrypt64(4293288604),
    );
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(4293288604),
      this.instances5.alice.encrypt64(4293288604),
    );
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(4293288604),
      this.instances5.alice.encrypt64(4293288604),
    );
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (18441848963293247005, 18437762817766608073)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(18441848963293247005),
      this.instances5.alice.encrypt64(18437762817766608073),
    );
    expect(res).to.equal(18437758350371472393n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (18437762817766608069, 18437762817766608073)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(18437762817766608069),
      this.instances5.alice.encrypt64(18437762817766608073),
    );
    expect(res).to.equal(18437762817766608065n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (18437762817766608073, 18437762817766608073)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(18437762817766608073),
      this.instances5.alice.encrypt64(18437762817766608073),
    );
    expect(res).to.equal(18437762817766608073n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (18437762817766608073, 18437762817766608069)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(18437762817766608073),
      this.instances5.alice.encrypt64(18437762817766608069),
    );
    expect(res).to.equal(18437762817766608065n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 1 (18439947486770357681, 18442205516883919361)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(18439947486770357681),
      this.instances5.alice.encrypt64(18442205516883919361),
    );
    expect(res).to.equal(18442234697046943665n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 2 (18439947486770357677, 18439947486770357681)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(18439947486770357677),
      this.instances5.alice.encrypt64(18439947486770357681),
    );
    expect(res).to.equal(18439947486770357693n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 3 (18439947486770357681, 18439947486770357681)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(18439947486770357681),
      this.instances5.alice.encrypt64(18439947486770357681),
    );
    expect(res).to.equal(18439947486770357681n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 4 (18439947486770357681, 18439947486770357677)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(18439947486770357681),
      this.instances5.alice.encrypt64(18439947486770357677),
    );
    expect(res).to.equal(18439947486770357693n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 1 (18444700078916958431, 18443866708631144651)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(18444700078916958431),
      this.instances5.alice.encrypt64(18443866708631144651),
    );
    expect(res).to.equal(3795455568392212n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 2 (18443866708631144647, 18443866708631144651)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(18443866708631144647),
      this.instances5.alice.encrypt64(18443866708631144651),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 3 (18443866708631144651, 18443866708631144651)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(18443866708631144651),
      this.instances5.alice.encrypt64(18443866708631144651),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 4 (18443866708631144651, 18443866708631144647)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(18443866708631144651),
      this.instances5.alice.encrypt64(18443866708631144647),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 1 (18443330521266220729, 18438253731135327627)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(18443330521266220729),
      this.instances5.alice.encrypt64(18438253731135327627),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 2 (18438253731135327623, 18438253731135327627)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(18438253731135327623),
      this.instances5.alice.encrypt64(18438253731135327627),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 3 (18438253731135327627, 18438253731135327627)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(18438253731135327627),
      this.instances5.alice.encrypt64(18438253731135327627),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 4 (18438253731135327627, 18438253731135327623)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(18438253731135327627),
      this.instances5.alice.encrypt64(18438253731135327623),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (18445140354518938845, 18441391037965649995)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(18445140354518938845),
      this.instances5.alice.encrypt64(18441391037965649995),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (18441391037965649991, 18441391037965649995)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(18441391037965649991),
      this.instances5.alice.encrypt64(18441391037965649995),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (18441391037965649995, 18441391037965649995)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(18441391037965649995),
      this.instances5.alice.encrypt64(18441391037965649995),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (18441391037965649995, 18441391037965649991)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(18441391037965649995),
      this.instances5.alice.encrypt64(18441391037965649991),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 1 (18444991478795579145, 18445260307161364245)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(18444991478795579145),
      this.instances5.alice.encrypt64(18445260307161364245),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 2 (18444991478795579141, 18444991478795579145)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(18444991478795579141),
      this.instances5.alice.encrypt64(18444991478795579145),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 3 (18444991478795579145, 18444991478795579145)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(18444991478795579145),
      this.instances5.alice.encrypt64(18444991478795579145),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 4 (18444991478795579145, 18444991478795579141)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(18444991478795579145),
      this.instances5.alice.encrypt64(18444991478795579141),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (18439787790330435145, 18439484090308827429)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(18439787790330435145),
      this.instances5.alice.encrypt64(18439484090308827429),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (18439484090308827425, 18439484090308827429)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(18439484090308827425),
      this.instances5.alice.encrypt64(18439484090308827429),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (18439484090308827429, 18439484090308827429)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(18439484090308827429),
      this.instances5.alice.encrypt64(18439484090308827429),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (18439484090308827429, 18439484090308827425)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(18439484090308827429),
      this.instances5.alice.encrypt64(18439484090308827425),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (18440769778451615393, 18446070761608442971)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(18440769778451615393),
      this.instances5.alice.encrypt64(18446070761608442971),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (18440769778451615389, 18440769778451615393)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(18440769778451615389),
      this.instances5.alice.encrypt64(18440769778451615393),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (18440769778451615393, 18440769778451615393)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(18440769778451615393),
      this.instances5.alice.encrypt64(18440769778451615393),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (18440769778451615393, 18440769778451615389)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(18440769778451615393),
      this.instances5.alice.encrypt64(18440769778451615389),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 1 (18446718131340158589, 18444160910497783341)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(18446718131340158589),
      this.instances5.alice.encrypt64(18444160910497783341),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 2 (18444160910497783337, 18444160910497783341)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(18444160910497783337),
      this.instances5.alice.encrypt64(18444160910497783341),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 3 (18444160910497783341, 18444160910497783341)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(18444160910497783341),
      this.instances5.alice.encrypt64(18444160910497783341),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 4 (18444160910497783341, 18444160910497783337)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(18444160910497783341),
      this.instances5.alice.encrypt64(18444160910497783337),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 1 (18444400472074074345, 18442962239103377481)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(18444400472074074345),
      this.instances5.alice.encrypt64(18442962239103377481),
    );
    expect(res).to.equal(18442962239103377481n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 2 (18442962239103377477, 18442962239103377481)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(18442962239103377477),
      this.instances5.alice.encrypt64(18442962239103377481),
    );
    expect(res).to.equal(18442962239103377477n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 3 (18442962239103377481, 18442962239103377481)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(18442962239103377481),
      this.instances5.alice.encrypt64(18442962239103377481),
    );
    expect(res).to.equal(18442962239103377481n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 4 (18442962239103377481, 18442962239103377477)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(18442962239103377481),
      this.instances5.alice.encrypt64(18442962239103377477),
    );
    expect(res).to.equal(18442962239103377477n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (18440739371866435289, 18438298584940765731)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18440739371866435289),
      this.instances5.alice.encrypt64(18438298584940765731),
    );
    expect(res).to.equal(18440739371866435289n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (18438298584940765727, 18438298584940765731)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18438298584940765727),
      this.instances5.alice.encrypt64(18438298584940765731),
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (18438298584940765731, 18438298584940765731)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18438298584940765731),
      this.instances5.alice.encrypt64(18438298584940765731),
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (18438298584940765731, 18438298584940765727)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18438298584940765731),
      this.instances5.alice.encrypt64(18438298584940765727),
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (9223329218882461797, 9220956803715422232)', async function () {
    const res = await this.contract5.add_euint64_uint64(
      this.instances5.alice.encrypt64(9223329218882461797),
      9220956803715422232,
    );
    expect(res).to.equal(18444286022597884029n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (9219964371310000509, 9219964371310000511)', async function () {
    const res = await this.contract5.add_euint64_uint64(
      this.instances5.alice.encrypt64(9219964371310000509),
      9219964371310000511,
    );
    expect(res).to.equal(18439928742620001020n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (9219964371310000511, 9219964371310000511)', async function () {
    const res = await this.contract5.add_euint64_uint64(
      this.instances5.alice.encrypt64(9219964371310000511),
      9219964371310000511,
    );
    expect(res).to.equal(18439928742620001022n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (9219964371310000511, 9219964371310000509)', async function () {
    const res = await this.contract5.add_euint64_uint64(
      this.instances5.alice.encrypt64(9219964371310000511),
      9219964371310000509,
    );
    expect(res).to.equal(18439928742620001020n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 1 (9219177655732910821, 9220956803715422232)', async function () {
    const res = await this.contract5.add_uint64_euint64(
      9219177655732910821,
      this.instances5.alice.encrypt64(9220956803715422232),
    );
    expect(res).to.equal(18440134459448333053n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 2 (9219964371310000509, 9219964371310000511)', async function () {
    const res = await this.contract5.add_uint64_euint64(
      9219964371310000509,
      this.instances5.alice.encrypt64(9219964371310000511),
    );
    expect(res).to.equal(18439928742620001020n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 3 (9219964371310000511, 9219964371310000511)', async function () {
    const res = await this.contract5.add_uint64_euint64(
      9219964371310000511,
      this.instances5.alice.encrypt64(9219964371310000511),
    );
    expect(res).to.equal(18439928742620001022n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 4 (9219964371310000511, 9219964371310000509)', async function () {
    const res = await this.contract5.add_uint64_euint64(
      9219964371310000511,
      this.instances5.alice.encrypt64(9219964371310000509),
    );
    expect(res).to.equal(18439928742620001020n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (18445117613821089157, 18445117613821089157)', async function () {
    const res = await this.contract5.sub_euint64_uint64(
      this.instances5.alice.encrypt64(18445117613821089157),
      18445117613821089157,
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (18445117613821089157, 18445117613821089153)', async function () {
    const res = await this.contract5.sub_euint64_uint64(
      this.instances5.alice.encrypt64(18445117613821089157),
      18445117613821089153,
    );
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (18445117613821089157, 18445117613821089157)', async function () {
    const res = await this.contract5.sub_uint64_euint64(
      18445117613821089157,
      this.instances5.alice.encrypt64(18445117613821089157),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (18445117613821089157, 18445117613821089153)', async function () {
    const res = await this.contract5.sub_uint64_euint64(
      18445117613821089157,
      this.instances5.alice.encrypt64(18445117613821089153),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (4294635170, 4293232253)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(4294635170), 4293232253);
    expect(res).to.equal(18437866226712138010n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(4293288604), 4293288604);
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(4293288604), 4293288604);
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(4293288604), 4293288604);
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 1 (4294226236, 4293232253)', async function () {
    const res = await this.contract5.mul_uint64_euint64(4294226236, this.instances5.alice.encrypt64(4293232253));
    expect(res).to.equal(18436110578073989708n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 2 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_uint64_euint64(4293288604, this.instances5.alice.encrypt64(4293288604));
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 3 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_uint64_euint64(4293288604, this.instances5.alice.encrypt64(4293288604));
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 4 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_uint64_euint64(4293288604, this.instances5.alice.encrypt64(4293288604));
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (18441976837575510865, 18441212274805422577)', async function () {
    const res = await this.contract5.div_euint64_uint64(
      this.instances5.alice.encrypt64(18441976837575510865),
      18441212274805422577,
    );
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (18441976837575510861, 18441976837575510865)', async function () {
    const res = await this.contract5.div_euint64_uint64(
      this.instances5.alice.encrypt64(18441976837575510861),
      18441976837575510865,
    );
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (18441976837575510865, 18441976837575510865)', async function () {
    const res = await this.contract5.div_euint64_uint64(
      this.instances5.alice.encrypt64(18441976837575510865),
      18441976837575510865,
    );
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (18441976837575510865, 18441976837575510861)', async function () {
    const res = await this.contract5.div_euint64_uint64(
      this.instances5.alice.encrypt64(18441976837575510865),
      18441976837575510861,
    );
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 1 (18443785129295236141, 18441307989286811147)', async function () {
    const res = await this.contract5.rem_euint64_uint64(
      this.instances5.alice.encrypt64(18443785129295236141),
      18441307989286811147,
    );
    expect(res).to.equal(2477140008424994n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 2 (18438390548915069819, 18438390548915069823)', async function () {
    const res = await this.contract5.rem_euint64_uint64(
      this.instances5.alice.encrypt64(18438390548915069819),
      18438390548915069823,
    );
    expect(res).to.equal(18438390548915069819n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 3 (18438390548915069823, 18438390548915069823)', async function () {
    const res = await this.contract5.rem_euint64_uint64(
      this.instances5.alice.encrypt64(18438390548915069823),
      18438390548915069823,
    );
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 4 (18438390548915069823, 18438390548915069819)', async function () {
    const res = await this.contract5.rem_euint64_uint64(
      this.instances5.alice.encrypt64(18438390548915069823),
      18438390548915069819,
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (18443330521266220729, 18446706410531688277)', async function () {
    const res = await this.contract5.eq_euint64_uint64(
      this.instances5.alice.encrypt64(18443330521266220729),
      18446706410531688277,
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (18438253731135327623, 18438253731135327627)', async function () {
    const res = await this.contract5.eq_euint64_uint64(
      this.instances5.alice.encrypt64(18438253731135327623),
      18438253731135327627,
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (18438253731135327627, 18438253731135327627)', async function () {
    const res = await this.contract5.eq_euint64_uint64(
      this.instances5.alice.encrypt64(18438253731135327627),
      18438253731135327627,
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (18438253731135327627, 18438253731135327623)', async function () {
    const res = await this.contract5.eq_euint64_uint64(
      this.instances5.alice.encrypt64(18438253731135327627),
      18438253731135327623,
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (18444395277752785729, 18446706410531688277)', async function () {
    const res = await this.contract5.eq_uint64_euint64(
      18444395277752785729,
      this.instances5.alice.encrypt64(18446706410531688277),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (18438253731135327623, 18438253731135327627)', async function () {
    const res = await this.contract5.eq_uint64_euint64(
      18438253731135327623,
      this.instances5.alice.encrypt64(18438253731135327627),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (18438253731135327627, 18438253731135327627)', async function () {
    const res = await this.contract5.eq_uint64_euint64(
      18438253731135327627,
      this.instances5.alice.encrypt64(18438253731135327627),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (18438253731135327627, 18438253731135327623)', async function () {
    const res = await this.contract5.eq_uint64_euint64(
      18438253731135327627,
      this.instances5.alice.encrypt64(18438253731135327623),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (18445140354518938845, 18438176226766160787)', async function () {
    const res = await this.contract5.ne_euint64_uint64(
      this.instances5.alice.encrypt64(18445140354518938845),
      18438176226766160787,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (18441391037965649991, 18441391037965649995)', async function () {
    const res = await this.contract5.ne_euint64_uint64(
      this.instances5.alice.encrypt64(18441391037965649991),
      18441391037965649995,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (18441391037965649995, 18441391037965649995)', async function () {
    const res = await this.contract5.ne_euint64_uint64(
      this.instances5.alice.encrypt64(18441391037965649995),
      18441391037965649995,
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (18441391037965649995, 18441391037965649991)', async function () {
    const res = await this.contract5.ne_euint64_uint64(
      this.instances5.alice.encrypt64(18441391037965649995),
      18441391037965649991,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (18443547473224968383, 18438176226766160787)', async function () {
    const res = await this.contract5.ne_uint64_euint64(
      18443547473224968383,
      this.instances5.alice.encrypt64(18438176226766160787),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (18441391037965649991, 18441391037965649995)', async function () {
    const res = await this.contract5.ne_uint64_euint64(
      18441391037965649991,
      this.instances5.alice.encrypt64(18441391037965649995),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (18441391037965649995, 18441391037965649995)', async function () {
    const res = await this.contract5.ne_uint64_euint64(
      18441391037965649995,
      this.instances5.alice.encrypt64(18441391037965649995),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (18441391037965649995, 18441391037965649991)', async function () {
    const res = await this.contract5.ne_uint64_euint64(
      18441391037965649995,
      this.instances5.alice.encrypt64(18441391037965649991),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (18444991478795579145, 18439567451994245465)', async function () {
    const res = await this.contract5.ge_euint64_uint64(
      this.instances5.alice.encrypt64(18444991478795579145),
      18439567451994245465,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (18444991478795579141, 18444991478795579145)', async function () {
    const res = await this.contract5.ge_euint64_uint64(
      this.instances5.alice.encrypt64(18444991478795579141),
      18444991478795579145,
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (18444991478795579145, 18444991478795579145)', async function () {
    const res = await this.contract5.ge_euint64_uint64(
      this.instances5.alice.encrypt64(18444991478795579145),
      18444991478795579145,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (18444991478795579145, 18444991478795579141)', async function () {
    const res = await this.contract5.ge_euint64_uint64(
      this.instances5.alice.encrypt64(18444991478795579145),
      18444991478795579141,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (18444429093181704535, 18439567451994245465)', async function () {
    const res = await this.contract5.ge_uint64_euint64(
      18444429093181704535,
      this.instances5.alice.encrypt64(18439567451994245465),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (18444991478795579141, 18444991478795579145)', async function () {
    const res = await this.contract5.ge_uint64_euint64(
      18444991478795579141,
      this.instances5.alice.encrypt64(18444991478795579145),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (18444991478795579145, 18444991478795579145)', async function () {
    const res = await this.contract5.ge_uint64_euint64(
      18444991478795579145,
      this.instances5.alice.encrypt64(18444991478795579145),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (18444991478795579145, 18444991478795579141)', async function () {
    const res = await this.contract5.ge_uint64_euint64(
      18444991478795579145,
      this.instances5.alice.encrypt64(18444991478795579141),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (18439787790330435145, 18441907321511169065)', async function () {
    const res = await this.contract5.gt_euint64_uint64(
      this.instances5.alice.encrypt64(18439787790330435145),
      18441907321511169065,
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (18439484090308827425, 18439484090308827429)', async function () {
    const res = await this.contract5.gt_euint64_uint64(
      this.instances5.alice.encrypt64(18439484090308827425),
      18439484090308827429,
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (18439484090308827429, 18439484090308827429)', async function () {
    const res = await this.contract5.gt_euint64_uint64(
      this.instances5.alice.encrypt64(18439484090308827429),
      18439484090308827429,
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (18439484090308827429, 18439484090308827425)', async function () {
    const res = await this.contract5.gt_euint64_uint64(
      this.instances5.alice.encrypt64(18439484090308827429),
      18439484090308827425,
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 1 (18438935380134710315, 18441907321511169065)', async function () {
    const res = await this.contract5.gt_uint64_euint64(
      18438935380134710315,
      this.instances5.alice.encrypt64(18441907321511169065),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 2 (18439484090308827425, 18439484090308827429)', async function () {
    const res = await this.contract5.gt_uint64_euint64(
      18439484090308827425,
      this.instances5.alice.encrypt64(18439484090308827429),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 3 (18439484090308827429, 18439484090308827429)', async function () {
    const res = await this.contract5.gt_uint64_euint64(
      18439484090308827429,
      this.instances5.alice.encrypt64(18439484090308827429),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 4 (18439484090308827429, 18439484090308827425)', async function () {
    const res = await this.contract5.gt_uint64_euint64(
      18439484090308827429,
      this.instances5.alice.encrypt64(18439484090308827425),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (18440769778451615393, 18439065451314752761)', async function () {
    const res = await this.contract5.le_euint64_uint64(
      this.instances5.alice.encrypt64(18440769778451615393),
      18439065451314752761,
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (18440769778451615389, 18440769778451615393)', async function () {
    const res = await this.contract5.le_euint64_uint64(
      this.instances5.alice.encrypt64(18440769778451615389),
      18440769778451615393,
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (18440769778451615393, 18440769778451615393)', async function () {
    const res = await this.contract5.le_euint64_uint64(
      this.instances5.alice.encrypt64(18440769778451615393),
      18440769778451615393,
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (18440769778451615393, 18440769778451615389)', async function () {
    const res = await this.contract5.le_euint64_uint64(
      this.instances5.alice.encrypt64(18440769778451615393),
      18440769778451615389,
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (18440980092932624951, 18439065451314752761)', async function () {
    const res = await this.contract5.le_uint64_euint64(
      18440980092932624951,
      this.instances5.alice.encrypt64(18439065451314752761),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (18440769778451615389, 18440769778451615393)', async function () {
    const res = await this.contract5.le_uint64_euint64(
      18440769778451615389,
      this.instances5.alice.encrypt64(18440769778451615393),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (18440769778451615393, 18440769778451615393)', async function () {
    const res = await this.contract5.le_uint64_euint64(
      18440769778451615393,
      this.instances5.alice.encrypt64(18440769778451615393),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (18440769778451615393, 18440769778451615389)', async function () {
    const res = await this.contract5.le_uint64_euint64(
      18440769778451615393,
      this.instances5.alice.encrypt64(18440769778451615389),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (18446718131340158589, 18438438177494413269)', async function () {
    const res = await this.contract5.lt_euint64_uint64(
      this.instances5.alice.encrypt64(18446718131340158589),
      18438438177494413269,
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (18444160910497783337, 18444160910497783341)', async function () {
    const res = await this.contract5.lt_euint64_uint64(
      this.instances5.alice.encrypt64(18444160910497783337),
      18444160910497783341,
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (18444160910497783341, 18444160910497783341)', async function () {
    const res = await this.contract5.lt_euint64_uint64(
      this.instances5.alice.encrypt64(18444160910497783341),
      18444160910497783341,
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (18444160910497783341, 18444160910497783337)', async function () {
    const res = await this.contract5.lt_euint64_uint64(
      this.instances5.alice.encrypt64(18444160910497783341),
      18444160910497783337,
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (18445719507413937869, 18438438177494413269)', async function () {
    const res = await this.contract5.lt_uint64_euint64(
      18445719507413937869,
      this.instances5.alice.encrypt64(18438438177494413269),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (18444160910497783337, 18444160910497783341)', async function () {
    const res = await this.contract5.lt_uint64_euint64(
      18444160910497783337,
      this.instances5.alice.encrypt64(18444160910497783341),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (18444160910497783341, 18444160910497783341)', async function () {
    const res = await this.contract5.lt_uint64_euint64(
      18444160910497783341,
      this.instances5.alice.encrypt64(18444160910497783341),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (18444160910497783341, 18444160910497783337)', async function () {
    const res = await this.contract5.lt_uint64_euint64(
      18444160910497783341,
      this.instances5.alice.encrypt64(18444160910497783337),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 1 (18444400472074074345, 18445675871085860653)', async function () {
    const res = await this.contract5.min_euint64_uint64(
      this.instances5.alice.encrypt64(18444400472074074345),
      18445675871085860653,
    );
    expect(res).to.equal(18444400472074074345n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 2 (18442962239103377477, 18442962239103377481)', async function () {
    const res = await this.contract5.min_euint64_uint64(
      this.instances5.alice.encrypt64(18442962239103377477),
      18442962239103377481,
    );
    expect(res).to.equal(18442962239103377477n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 3 (18442962239103377481, 18442962239103377481)', async function () {
    const res = await this.contract5.min_euint64_uint64(
      this.instances5.alice.encrypt64(18442962239103377481),
      18442962239103377481,
    );
    expect(res).to.equal(18442962239103377481n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 4 (18442962239103377481, 18442962239103377477)', async function () {
    const res = await this.contract5.min_euint64_uint64(
      this.instances5.alice.encrypt64(18442962239103377481),
      18442962239103377477,
    );
    expect(res).to.equal(18442962239103377477n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (18443908139931756717, 18445675871085860653)', async function () {
    const res = await this.contract5.min_uint64_euint64(
      18443908139931756717,
      this.instances5.alice.encrypt64(18445675871085860653),
    );
    expect(res).to.equal(18443908139931756717n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (18442962239103377477, 18442962239103377481)', async function () {
    const res = await this.contract5.min_uint64_euint64(
      18442962239103377477,
      this.instances5.alice.encrypt64(18442962239103377481),
    );
    expect(res).to.equal(18442962239103377477n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (18442962239103377481, 18442962239103377481)', async function () {
    const res = await this.contract5.min_uint64_euint64(
      18442962239103377481,
      this.instances5.alice.encrypt64(18442962239103377481),
    );
    expect(res).to.equal(18442962239103377481n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (18442962239103377481, 18442962239103377477)', async function () {
    const res = await this.contract5.min_uint64_euint64(
      18442962239103377481,
      this.instances5.alice.encrypt64(18442962239103377477),
    );
    expect(res).to.equal(18442962239103377477n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (18440739371866435289, 18440643015791741637)', async function () {
    const res = await this.contract5.max_euint64_uint64(
      this.instances5.alice.encrypt64(18440739371866435289),
      18440643015791741637,
    );
    expect(res).to.equal(18440739371866435289n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (18438298584940765727, 18438298584940765731)', async function () {
    const res = await this.contract5.max_euint64_uint64(
      this.instances5.alice.encrypt64(18438298584940765727),
      18438298584940765731,
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (18438298584940765731, 18438298584940765731)', async function () {
    const res = await this.contract5.max_euint64_uint64(
      this.instances5.alice.encrypt64(18438298584940765731),
      18438298584940765731,
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (18438298584940765731, 18438298584940765727)', async function () {
    const res = await this.contract5.max_euint64_uint64(
      this.instances5.alice.encrypt64(18438298584940765731),
      18438298584940765727,
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (18441357041435050863, 18440643015791741637)', async function () {
    const res = await this.contract5.max_uint64_euint64(
      18441357041435050863,
      this.instances5.alice.encrypt64(18440643015791741637),
    );
    expect(res).to.equal(18441357041435050863n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18438298584940765727, 18438298584940765731)', async function () {
    const res = await this.contract5.max_uint64_euint64(
      18438298584940765727,
      this.instances5.alice.encrypt64(18438298584940765731),
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18438298584940765731, 18438298584940765731)', async function () {
    const res = await this.contract5.max_uint64_euint64(
      18438298584940765731,
      this.instances5.alice.encrypt64(18438298584940765731),
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18438298584940765731, 18438298584940765727)', async function () {
    const res = await this.contract5.max_uint64_euint64(
      18438298584940765731,
      this.instances5.alice.encrypt64(18438298584940765727),
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 1 (13, 5)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(13), 5);
    expect(res).to.equal(10);
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

  it('test operator "shr" overload (euint4, uint8) => euint4 test 1 (14, 4)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(14), 4);
    expect(res).to.equal(14);
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

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (34, 4)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(34),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(32);
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

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (34, 4)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(34), 4);
    expect(res).to.equal(32);
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

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (68, 4)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(68),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(4);
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

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (68, 4)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(68), 4);
    expect(res).to.equal(4);
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

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (15362, 2)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(15362),
      this.instances5.alice.encrypt8(2),
    );
    expect(res).to.equal(61448);
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

  it('test operator "shl" overload (euint16, uint8) => euint16 test 1 (15362, 2)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(15362), 2);
    expect(res).to.equal(61448);
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

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (25648, 5)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(25648),
      this.instances5.alice.encrypt8(5),
    );
    expect(res).to.equal(801);
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

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (25648, 5)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(25648), 5);
    expect(res).to.equal(801);
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

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (833510670, 1)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(833510670),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(1667021340);
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

  it('test operator "shl" overload (euint32, uint8) => euint32 test 1 (833510670, 1)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(833510670), 1);
    expect(res).to.equal(1667021340);
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

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (3957313401, 6)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(3957313401),
      this.instances5.alice.encrypt8(6),
    );
    expect(res).to.equal(61833021);
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

  it('test operator "shr" overload (euint32, uint8) => euint32 test 1 (3957313401, 6)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(3957313401), 6);
    expect(res).to.equal(61833021);
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

  it('test operator "shl" overload (euint64, euint8) => euint64 test 1 (18445451452906630791, 5)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(18445451452906630791),
      this.instances5.alice.encrypt8(5),
    );
    expect(res).to.equal(18405380208016085000);
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

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (18445451452906630791, 5)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(18445451452906630791), 5);
    expect(res).to.equal(18405380208016085000);
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

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (18439569308403000305, 1)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(18439569308403000305),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(9219784654201500000);
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

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (18439569308403000305, 1)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(18439569308403000305), 1);
    expect(res).to.equal(9219784654201500000);
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

  it('test operator "not" overload (euint4) => euint4 test 1 (13)', async function () {
    const res = await this.contract5.not_euint4(this.instances5.alice.encrypt4(13));
    expect(res).to.equal(2n);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (251)', async function () {
    const res = await this.contract5.neg_euint8(this.instances5.alice.encrypt8(251));
    expect(res).to.equal(5n);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (187)', async function () {
    const res = await this.contract5.not_euint8(this.instances5.alice.encrypt8(187));
    expect(res).to.equal(68n);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (25161)', async function () {
    const res = await this.contract5.neg_euint16(this.instances5.alice.encrypt16(25161));
    expect(res).to.equal(40375n);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (60538)', async function () {
    const res = await this.contract5.not_euint16(this.instances5.alice.encrypt16(60538));
    expect(res).to.equal(4997n);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (2156295218)', async function () {
    const res = await this.contract5.neg_euint32(this.instances5.alice.encrypt32(2156295218));
    expect(res).to.equal(2138672078n);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (2475211657)', async function () {
    const res = await this.contract5.not_euint32(this.instances5.alice.encrypt32(2475211657));
    expect(res).to.equal(1819755638n);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (18438244346234461619)', async function () {
    const res = await this.contract5.neg_euint64(this.instances5.alice.encrypt64(18438244346234461619));
    expect(res).to.equal(8499727475089997n);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (18443955194473722291)', async function () {
    const res = await this.contract5.not_euint64(this.instances5.alice.encrypt64(18443955194473722291));
    expect(res).to.equal(2788879235829324n);
  });
});
