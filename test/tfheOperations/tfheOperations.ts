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

  it('test operator "add" overload (euint4, euint4) => euint4 test 1 (8, 2)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(10n);
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

  it('test operator "mul" overload (euint4, euint4) => euint4 test 1 (1, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 1 (1, 15)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt4(15),
    );
    expect(res).to.equal(1n);
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

  it('test operator "or" overload (euint4, euint4) => euint4 test 1 (1, 2)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(3n);
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

  it('test operator "xor" overload (euint4, euint4) => euint4 test 1 (3, 1)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(2n);
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

  it('test operator "eq" overload (euint4, euint4) => ebool test 1 (15, 1)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(15),
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

  it('test operator "ne" overload (euint4, euint4) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(false);
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

  it('test operator "ge" overload (euint4, euint4) => ebool test 1 (2, 1)', async function () {
    const res = await this.contract1.ge_euint4_euint4(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt4(1),
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

  it('test operator "gt" overload (euint4, euint4) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt4(3),
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

  it('test operator "le" overload (euint4, euint4) => ebool test 1 (1, 3)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt4(3),
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

  it('test operator "lt" overload (euint4, euint4) => ebool test 1 (2, 5)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt4(5),
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

  it('test operator "min" overload (euint4, euint4) => euint4 test 1 (1, 1)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(1n);
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

  it('test operator "max" overload (euint4, euint4) => euint4 test 1 (1, 2)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt4(2),
    );
    expect(res).to.equal(2n);
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

  it('test operator "add" overload (euint4, euint8) => euint8 test 1 (8, 3)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(3),
    );
    expect(res).to.equal(11n);
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

  it('test operator "mul" overload (euint4, euint8) => euint8 test 1 (1, 1)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(1),
    );
    expect(res).to.equal(1n);
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

  it('test operator "and" overload (euint4, euint8) => euint8 test 1 (1, 1)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 1 (1, 1)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 1 (3, 1)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt8(1),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 1 (15, 5)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(15),
      this.instances1.alice.encrypt8(5),
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

  it('test operator "ne" overload (euint4, euint8) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt8(3),
    );
    expect(res).to.equal(false);
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

  it('test operator "ge" overload (euint4, euint8) => ebool test 1 (2, 3)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt8(3),
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

  it('test operator "gt" overload (euint4, euint8) => ebool test 1 (3, 4)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt8(4),
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

  it('test operator "le" overload (euint4, euint8) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(1),
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

  it('test operator "lt" overload (euint4, euint8) => ebool test 1 (2, 255)', async function () {
    const res = await this.contract1.lt_euint4_euint8(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt8(255),
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

  it('test operator "min" overload (euint4, euint8) => euint8 test 1 (1, 2)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(2),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 1 (1, 5)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt8(5),
    );
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt8(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 1 (8, 1)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(1),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt16(5),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(12n);
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

  it('test operator "mul" overload (euint4, euint16) => euint16 test 1 (1, 12)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(12),
    );
    expect(res).to.equal(12n);
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

  it('test operator "and" overload (euint4, euint16) => euint16 test 1 (1, 3)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(3),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 1 (1, 1)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(1),
    );
    expect(res).to.equal(1n);
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

  it('test operator "xor" overload (euint4, euint16) => euint16 test 1 (3, 1)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt16(1),
    );
    expect(res).to.equal(2n);
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

  it('test operator "eq" overload (euint4, euint16) => ebool test 1 (15, 2)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(15),
      this.instances1.alice.encrypt16(2),
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

  it('test operator "ne" overload (euint4, euint16) => ebool test 1 (3, 41)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt16(41),
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

  it('test operator "ge" overload (euint4, euint16) => ebool test 1 (2, 10)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt16(10),
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

  it('test operator "gt" overload (euint4, euint16) => ebool test 1 (3, 2)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt16(2),
    );
    expect(res).to.equal(true);
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

  it('test operator "le" overload (euint4, euint16) => ebool test 1 (1, 17)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(17),
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

  it('test operator "lt" overload (euint4, euint16) => ebool test 1 (2, 1)', async function () {
    const res = await this.contract1.lt_euint4_euint16(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt16(1),
    );
    expect(res).to.equal(false);
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

  it('test operator "min" overload (euint4, euint16) => euint16 test 1 (1, 2)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(2),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt16(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 1 (1, 1)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt16(1),
    );
    expect(res).to.equal(1n);
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

  it('test operator "add" overload (euint4, euint32) => euint32 test 1 (8, 1)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(9n);
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

  it('test operator "mul" overload (euint4, euint32) => euint32 test 1 (1, 2)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(2),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt32(5),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt32(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt32(3),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 1 (1, 2)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(2),
    );
    expect(res).to.equal(0n);
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

  it('test operator "or" overload (euint4, euint32) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(1n);
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

  it('test operator "xor" overload (euint4, euint32) => euint32 test 1 (3, 5)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt32(5),
    );
    expect(res).to.equal(6n);
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

  it('test operator "eq" overload (euint4, euint32) => ebool test 1 (15, 16)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(15),
      this.instances1.alice.encrypt32(16),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 2 (11, 15)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt32(15),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 3 (15, 15)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(15),
      this.instances1.alice.encrypt32(15),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 4 (15, 11)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(15),
      this.instances1.alice.encrypt32(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 1 (3, 1)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt32(1),
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

  it('test operator "ge" overload (euint4, euint32) => ebool test 1 (2, 3)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt32(3),
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

  it('test operator "gt" overload (euint4, euint32) => ebool test 1 (3, 4)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt32(4),
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

  it('test operator "le" overload (euint4, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(1),
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

  it('test operator "lt" overload (euint4, euint32) => ebool test 1 (2, 1)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(false);
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

  it('test operator "min" overload (euint4, euint32) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(1),
    );
    expect(res).to.equal(1n);
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

  it('test operator "max" overload (euint4, euint32) => euint32 test 1 (1, 6)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt32(6),
    );
    expect(res).to.equal(6n);
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

  it('test operator "add" overload (euint4, euint64) => euint64 test 1 (2, 10)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt64(10),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 2 (6, 8)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(6),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(5),
      this.instances1.alice.encrypt64(5),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 4 (8, 6)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(6),
    );
    expect(res).to.equal(14n);
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

  it('test operator "mul" overload (euint4, euint64) => euint64 test 1 (1, 13)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(13),
    );
    expect(res).to.equal(13n);
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

  it('test operator "and" overload (euint4, euint64) => euint64 test 1 (1, 13187)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(13187),
    );
    expect(res).to.equal(1n);
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

  it('test operator "or" overload (euint4, euint64) => euint64 test 1 (1, 57469)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(57469),
    );
    expect(res).to.equal(57469n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(4),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(8),
      this.instances1.alice.encrypt64(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 1 (3, 2792)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt64(2792),
    );
    expect(res).to.equal(2795n);
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

  it('test operator "eq" overload (euint4, euint64) => ebool test 1 (15, 191715)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(15),
      this.instances1.alice.encrypt64(191715),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 2 (11, 15)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(11),
      this.instances1.alice.encrypt64(15),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 3 (15, 15)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(15),
      this.instances1.alice.encrypt64(15),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 4 (15, 11)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(15),
      this.instances1.alice.encrypt64(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 1 (3, 2558)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt64(2558),
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

  it('test operator "ge" overload (euint4, euint64) => ebool test 1 (2, 3677)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt64(3677),
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

  it('test operator "gt" overload (euint4, euint64) => ebool test 1 (3, 6288)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(3),
      this.instances1.alice.encrypt64(6288),
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

  it('test operator "le" overload (euint4, euint64) => ebool test 1 (1, 10377)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(10377),
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

  it('test operator "lt" overload (euint4, euint64) => ebool test 1 (2, 8139)', async function () {
    const res = await this.contract1.lt_euint4_euint64(
      this.instances1.alice.encrypt4(2),
      this.instances1.alice.encrypt64(8139),
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

  it('test operator "min" overload (euint4, euint64) => euint64 test 1 (1, 2202)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(2202),
    );
    expect(res).to.equal(1n);
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

  it('test operator "max" overload (euint4, euint64) => euint64 test 1 (1, 2520)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(1),
      this.instances1.alice.encrypt64(2520),
    );
    expect(res).to.equal(2520n);
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

  it('test operator "add" overload (euint4, uint8) => euint4 test 1 (8, 2)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(8), 2);
    expect(res).to.equal(10n);
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

  it('test operator "add" overload (uint8, euint4) => euint4 test 1 (8, 1)', async function () {
    const res = await this.contract1.add_uint8_euint4(8, this.instances1.alice.encrypt4(1));
    expect(res).to.equal(9n);
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

  it('test operator "mul" overload (euint4, uint8) => euint4 test 1 (1, 3)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(1), 3);
    expect(res).to.equal(3n);
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

  it('test operator "mul" overload (uint8, euint4) => euint4 test 1 (2, 5)', async function () {
    const res = await this.contract1.mul_uint8_euint4(2, this.instances1.alice.encrypt4(5));
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_uint8_euint4(3, this.instances1.alice.encrypt4(5));
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_uint8_euint4(3, this.instances1.alice.encrypt4(3));
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_uint8_euint4(5, this.instances1.alice.encrypt4(3));
    expect(res).to.equal(15n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 1 (1, 2)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(1), 2);
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

  it('test operator "rem" overload (euint4, uint8) => euint4 test 1 (15, 2)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(15), 2);
    expect(res).to.equal(1n);
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

  it('test operator "eq" overload (euint4, uint8) => ebool test 1 (15, 15)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(15), 15);
    expect(res).to.equal(true);
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

  it('test operator "eq" overload (uint8, euint4) => ebool test 1 (1, 2)', async function () {
    const res = await this.contract1.eq_uint8_euint4(1, this.instances1.alice.encrypt4(2));
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

  it('test operator "ne" overload (euint4, uint8) => ebool test 1 (3, 1)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(3), 1);
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

  it('test operator "ne" overload (uint8, euint4) => ebool test 1 (2, 2)', async function () {
    const res = await this.contract1.ne_uint8_euint4(2, this.instances1.alice.encrypt4(2));
    expect(res).to.equal(false);
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

  it('test operator "ge" overload (euint4, uint8) => ebool test 1 (2, 1)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(2), 1);
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

  it('test operator "ge" overload (uint8, euint4) => ebool test 1 (2, 15)', async function () {
    const res = await this.contract1.ge_uint8_euint4(2, this.instances1.alice.encrypt4(15));
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

  it('test operator "gt" overload (euint4, uint8) => ebool test 1 (3, 1)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(3), 1);
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

  it('test operator "gt" overload (uint8, euint4) => ebool test 1 (5, 15)', async function () {
    const res = await this.contract1.gt_uint8_euint4(5, this.instances1.alice.encrypt4(15));
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

  it('test operator "le" overload (euint4, uint8) => ebool test 1 (1, 7)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(1), 7);
    expect(res).to.equal(true);
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

  it('test operator "le" overload (uint8, euint4) => ebool test 1 (1, 2)', async function () {
    const res = await this.contract1.le_uint8_euint4(1, this.instances1.alice.encrypt4(2));
    expect(res).to.equal(true);
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

  it('test operator "lt" overload (euint4, uint8) => ebool test 1 (2, 3)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(2), 3);
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

  it('test operator "lt" overload (uint8, euint4) => ebool test 1 (15, 1)', async function () {
    const res = await this.contract1.lt_uint8_euint4(15, this.instances1.alice.encrypt4(1));
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

  it('test operator "min" overload (euint4, uint8) => euint4 test 1 (1, 15)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(1), 15);
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 1 (2, 15)', async function () {
    const res = await this.contract1.min_uint8_euint4(2, this.instances1.alice.encrypt4(15));
    expect(res).to.equal(2n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_uint8_euint4(4, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_uint8_euint4(8, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 1 (1, 7)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(1), 7);
    expect(res).to.equal(7n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(4), 8);
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(8), 8);
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(8), 4);
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 1 (3, 1)', async function () {
    const res = await this.contract1.max_uint8_euint4(3, this.instances1.alice.encrypt4(1));
    expect(res).to.equal(3n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_uint8_euint4(4, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_uint8_euint4(8, this.instances1.alice.encrypt4(8));
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_uint8_euint4(8, this.instances1.alice.encrypt4(4));
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 1 (2, 1)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(2),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(3n);
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

  it('test operator "mul" overload (euint8, euint4) => euint8 test 1 (2, 5)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(2),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt4(5),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(3),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(5),
      this.instances1.alice.encrypt4(3),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 1 (2, 1)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(2),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(0n);
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

  it('test operator "or" overload (euint8, euint4) => euint8 test 1 (1, 1)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(1),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(1n);
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

  it('test operator "xor" overload (euint8, euint4) => euint8 test 1 (2, 1)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(2),
      this.instances1.alice.encrypt4(1),
    );
    expect(res).to.equal(3n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(4),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(8),
      this.instances1.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 1 (1, 2)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(1),
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

  it('test operator "ne" overload (euint8, euint4) => ebool test 1 (2, 2)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt4(2),
    );
    expect(res).to.equal(false);
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

  it('test operator "ge" overload (euint8, euint4) => ebool test 1 (2, 15)', async function () {
    const res = await this.contract2.ge_euint8_euint4(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt4(15),
    );
    expect(res).to.equal(false);
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

  it('test operator "gt" overload (euint8, euint4) => ebool test 1 (1, 15)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt4(15),
    );
    expect(res).to.equal(false);
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

  it('test operator "le" overload (euint8, euint4) => ebool test 1 (1, 2)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt4(2),
    );
    expect(res).to.equal(true);
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

  it('test operator "lt" overload (euint8, euint4) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(1),
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

  it('test operator "min" overload (euint8, euint4) => euint8 test 1 (8, 15)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(15),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 1 (1, 1)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 1 (15, 3)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(15),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(18n);
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

  it('test operator "sub" overload (euint8, euint8) => euint8 test 1 (8, 8)', async function () {
    const res = await this.contract2.sub_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 2 (8, 4)', async function () {
    const res = await this.contract2.sub_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 1 (2, 2)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt8(2),
    );
    expect(res).to.equal(4n);
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

  it('test operator "and" overload (euint8, euint8) => euint8 test 1 (2, 1)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt8(1),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 1 (1, 1)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt8(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 1 (2, 1)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt8(1),
    );
    expect(res).to.equal(3n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt8(1),
    );
    expect(res).to.equal(true);
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

  it('test operator "ne" overload (euint8, euint8) => ebool test 1 (2, 1)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt8(1),
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

  it('test operator "ge" overload (euint8, euint8) => ebool test 1 (2, 2)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt8(2),
    );
    expect(res).to.equal(true);
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

  it('test operator "gt" overload (euint8, euint8) => ebool test 1 (5, 1)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt8(1),
    );
    expect(res).to.equal(true);
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

  it('test operator "le" overload (euint8, euint8) => ebool test 1 (1, 4)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt8(4),
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

  it('test operator "lt" overload (euint8, euint8) => ebool test 1 (15, 1)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(15),
      this.instances2.alice.encrypt8(1),
    );
    expect(res).to.equal(false);
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

  it('test operator "min" overload (euint8, euint8) => euint8 test 1 (2, 2)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt8(2),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 1 (3, 3)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(3),
      this.instances2.alice.encrypt8(3),
    );
    expect(res).to.equal(3n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt8(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 1 (36, 1)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(36),
      this.instances2.alice.encrypt16(1),
    );
    expect(res).to.equal(37n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 1 (8, 8)', async function () {
    const res = await this.contract2.sub_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 2 (8, 4)', async function () {
    const res = await this.contract2.sub_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 1 (1, 1)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 1 (2, 2)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt16(2),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 1 (1, 1)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 1 (2, 1)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt16(1),
    );
    expect(res).to.equal(3n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 1 (5, 1)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt16(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 1 (3, 1)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(3),
      this.instances2.alice.encrypt16(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 1 (1, 26)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(26),
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

  it('test operator "gt" overload (euint8, euint16) => ebool test 1 (2, 2)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt16(2),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 1 (15, 1)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(15),
      this.instances2.alice.encrypt16(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 1 (1, 3)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(3),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt16(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 1 (1, 1)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt16(1),
    );
    expect(res).to.equal(1n);
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

  it('test operator "add" overload (euint8, euint32) => euint32 test 1 (36, 2)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(36),
      this.instances2.alice.encrypt32(2),
    );
    expect(res).to.equal(38n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 1 (8, 8)', async function () {
    const res = await this.contract2.sub_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 2 (8, 4)', async function () {
    const res = await this.contract2.sub_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 1 (2, 4)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 1 (2, 1)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt32(1),
    );
    expect(res).to.equal(3n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 1 (5, 3)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt32(3),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 1 (3, 1)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(3),
      this.instances2.alice.encrypt32(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(1),
    );
    expect(res).to.equal(true);
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

  it('test operator "gt" overload (euint8, euint32) => ebool test 1 (2, 1)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt32(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 1 (15, 1)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(15),
      this.instances2.alice.encrypt32(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 1 (1, 18)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(18),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 1 (1, 3)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt32(3),
    );
    expect(res).to.equal(3n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt32(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 1 (3, 168)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(3),
      this.instances2.alice.encrypt64(168),
    );
    expect(res).to.equal(171n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 2 (32, 36)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(32),
      this.instances2.alice.encrypt64(36),
    );
    expect(res).to.equal(68n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 3 (36, 36)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(36),
      this.instances2.alice.encrypt64(36),
    );
    expect(res).to.equal(72n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 4 (36, 32)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(36),
      this.instances2.alice.encrypt64(32),
    );
    expect(res).to.equal(68n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 1 (8, 8)', async function () {
    const res = await this.contract2.sub_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 2 (8, 4)', async function () {
    const res = await this.contract2.sub_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 1 (1, 140)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(140),
    );
    expect(res).to.equal(140n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(4),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 1 (2, 2049)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt64(2049),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 1 (1, 6021)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(6021),
    );
    expect(res).to.equal(6021n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 1 (2, 4046)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt64(4046),
    );
    expect(res).to.equal(4044n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 1 (5, 5764)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(5),
      this.instances2.alice.encrypt64(5764),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 1 (3, 2082)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(3),
      this.instances2.alice.encrypt64(2082),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 1 (1, 7637)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(7637),
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

  it('test operator "gt" overload (euint8, euint64) => ebool test 1 (2, 2952)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(2),
      this.instances2.alice.encrypt64(2952),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 1 (15, 3500)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(15),
      this.instances2.alice.encrypt64(3500),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 2 (11, 15)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(11),
      this.instances2.alice.encrypt64(15),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 3 (15, 15)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(15),
      this.instances2.alice.encrypt64(15),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 4 (15, 11)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(15),
      this.instances2.alice.encrypt64(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 1 (1, 2371)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(2371),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 1 (1, 5533)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(5533),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 1 (1, 9155)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(1),
      this.instances2.alice.encrypt64(9155),
    );
    expect(res).to.equal(9155n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(4),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(8),
      this.instances2.alice.encrypt64(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 1 (15, 1)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(15), 1);
    expect(res).to.equal(16n);
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

  it('test operator "add" overload (uint8, euint8) => euint8 test 1 (36, 1)', async function () {
    const res = await this.contract2.add_uint8_euint8(36, this.instances2.alice.encrypt8(1));
    expect(res).to.equal(37n);
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

  it('test operator "sub" overload (euint8, uint8) => euint8 test 1 (8, 8)', async function () {
    const res = await this.contract2.sub_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 2 (8, 4)', async function () {
    const res = await this.contract2.sub_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 1 (8, 8)', async function () {
    const res = await this.contract2.sub_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 2 (8, 4)', async function () {
    const res = await this.contract2.sub_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 1 (2, 2)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(2), 2);
    expect(res).to.equal(4n);
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

  it('test operator "mul" overload (uint8, euint8) => euint8 test 1 (1, 2)', async function () {
    const res = await this.contract2.mul_uint8_euint8(1, this.instances2.alice.encrypt8(2));
    expect(res).to.equal(2n);
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

  it('test operator "div" overload (euint8, uint8) => euint8 test 1 (1, 7)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(1), 7);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(2n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 1 (1, 1)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(1), 1);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(4n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(0n);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(1), 1);
    expect(res).to.equal(true);
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

  it('test operator "eq" overload (uint8, euint8) => ebool test 1 (5, 1)', async function () {
    const res = await this.contract2.eq_uint8_euint8(5, this.instances2.alice.encrypt8(1));
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

  it('test operator "ne" overload (euint8, uint8) => ebool test 1 (2, 2)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(2), 2);
    expect(res).to.equal(false);
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

  it('test operator "ne" overload (uint8, euint8) => ebool test 1 (3, 2)', async function () {
    const res = await this.contract2.ne_uint8_euint8(3, this.instances2.alice.encrypt8(2));
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

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (2, 11)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(2), 11);
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

  it('test operator "ge" overload (uint8, euint8) => ebool test 1 (1, 11)', async function () {
    const res = await this.contract2.ge_uint8_euint8(1, this.instances2.alice.encrypt8(11));
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

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (5, 1)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(5), 1);
    expect(res).to.equal(true);
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

  it('test operator "gt" overload (uint8, euint8) => ebool test 1 (2, 1)', async function () {
    const res = await this.contract2.gt_uint8_euint8(2, this.instances2.alice.encrypt8(1));
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

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (1, 2)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(1), 2);
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

  it('test operator "le" overload (uint8, euint8) => ebool test 1 (15, 2)', async function () {
    const res = await this.contract2.le_uint8_euint8(15, this.instances2.alice.encrypt8(2));
    expect(res).to.equal(false);
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

  it('test operator "lt" overload (euint8, uint8) => ebool test 1 (15, 1)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(15), 1);
    expect(res).to.equal(false);
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

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract2.lt_uint8_euint8(1, this.instances2.alice.encrypt8(1));
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

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (2, 1)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(2), 1);
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (1, 1)', async function () {
    const res = await this.contract2.min_uint8_euint8(1, this.instances2.alice.encrypt8(1));
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.min_uint8_euint8(4, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.min_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.min_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (3, 14)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(3), 14);
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(4), 8);
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(8), 8);
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(8), 4);
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 1 (1, 14)', async function () {
    const res = await this.contract2.max_uint8_euint8(1, this.instances2.alice.encrypt8(14));
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.max_uint8_euint8(4, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.max_uint8_euint8(8, this.instances2.alice.encrypt8(8));
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.max_uint8_euint8(8, this.instances2.alice.encrypt8(4));
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 1 (3, 2)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(3),
      this.instances2.alice.encrypt4(2),
    );
    expect(res).to.equal(5n);
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

  it('test operator "sub" overload (euint16, euint4) => euint16 test 1 (8, 8)', async function () {
    const res = await this.contract2.sub_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint4) => euint16 test 2 (8, 4)', async function () {
    const res = await this.contract2.sub_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 1 (1, 1)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(1),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(1n);
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

  it('test operator "and" overload (euint16, euint4) => euint16 test 1 (1, 7)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(1),
      this.instances2.alice.encrypt4(7),
    );
    expect(res).to.equal(1n);
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

  it('test operator "or" overload (euint16, euint4) => euint16 test 1 (2, 15)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(2),
      this.instances2.alice.encrypt4(15),
    );
    expect(res).to.equal(15n);
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

  it('test operator "xor" overload (euint16, euint4) => euint16 test 1 (9, 3)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(9),
      this.instances2.alice.encrypt4(3),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(4),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(8),
      this.instances2.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 1 (5, 7)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(5),
      this.instances2.alice.encrypt4(7),
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

  it('test operator "ne" overload (euint16, euint4) => ebool test 1 (3, 3)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(3),
      this.instances2.alice.encrypt4(3),
    );
    expect(res).to.equal(false);
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

  it('test operator "ge" overload (euint16, euint4) => ebool test 1 (1, 7)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(1),
      this.instances2.alice.encrypt4(7),
    );
    expect(res).to.equal(false);
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

  it('test operator "gt" overload (euint16, euint4) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(1),
      this.instances2.alice.encrypt4(1),
    );
    expect(res).to.equal(false);
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

  it('test operator "le" overload (euint16, euint4) => ebool test 1 (23, 1)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(23),
      this.instances2.alice.encrypt4(1),
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

  it('test operator "lt" overload (euint16, euint4) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(1),
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

  it('test operator "min" overload (euint16, euint4) => euint16 test 1 (1, 1)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt4(1),
    );
    expect(res).to.equal(1n);
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

  it('test operator "max" overload (euint16, euint4) => euint16 test 1 (2, 1)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt4(1),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 1 (3, 1)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(3),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (8, 8)', async function () {
    const res = await this.contract3.sub_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (8, 4)', async function () {
    const res = await this.contract3.sub_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (1, 21)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt8(21),
    );
    expect(res).to.equal(21n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 1 (1, 1)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 1 (2, 4)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (9, 7)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(9),
      this.instances3.alice.encrypt8(7),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 1 (5, 3)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(5),
      this.instances3.alice.encrypt8(3),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (3, 1)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(3),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 1 (1, 3)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt8(3),
    );
    expect(res).to.equal(false);
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

  it('test operator "le" overload (euint16, euint8) => ebool test 1 (23, 2)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(23),
      this.instances3.alice.encrypt8(2),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 1 (1, 5)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt8(5),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 1 (1, 3)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt8(3),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 1 (2, 1)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 1 (3, 3)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(3),
      this.instances3.alice.encrypt16(3),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 1 (8, 8)', async function () {
    const res = await this.contract3.sub_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 2 (8, 4)', async function () {
    const res = await this.contract3.sub_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 1 (1, 2)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt16(2),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(4),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 1 (1, 1)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt16(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 1 (2, 5)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt16(5),
    );
    expect(res).to.equal(7n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (9, 1)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(9),
      this.instances3.alice.encrypt16(1),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 1 (5, 6)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(5),
      this.instances3.alice.encrypt16(6),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 1 (3, 2)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(3),
      this.instances3.alice.encrypt16(2),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 1 (1, 2)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt16(2),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 1 (1, 2)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt16(2),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 1 (23, 1)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(23),
      this.instances3.alice.encrypt16(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt16(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 1 (1, 1)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt16(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 1 (2, 1)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt16(1),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt16(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 1 (2, 2)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt32(2),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (8, 8)', async function () {
    const res = await this.contract3.sub_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (8, 4)', async function () {
    const res = await this.contract3.sub_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (2, 1)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt32(1),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt32(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (2, 4)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 1 (9, 147)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(9),
      this.instances3.alice.encrypt32(147),
    );
    expect(res).to.equal(154n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 2 (5, 9)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(5),
      this.instances3.alice.encrypt32(9),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 3 (9, 9)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(9),
      this.instances3.alice.encrypt32(9),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 4 (9, 5)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(9),
      this.instances3.alice.encrypt32(5),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (1, 9)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt32(9),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt32(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt32(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt32(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt32(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 1 (2, 4)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 1 (1, 5)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt32(5),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt32(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt32(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 1 (2, 5596)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt64(5596),
    );
    expect(res).to.equal(5598n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 1 (8, 8)', async function () {
    const res = await this.contract3.sub_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 2 (8, 4)', async function () {
    const res = await this.contract3.sub_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 1 (2, 2601)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt64(2601),
    );
    expect(res).to.equal(5202n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(4),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 1 (1, 4630)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt64(4630),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 1 (2, 17420)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt64(17420),
    );
    expect(res).to.equal(17422n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 1 (9, 28164)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(9),
      this.instances3.alice.encrypt64(28164),
    );
    expect(res).to.equal(28173n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 2 (5, 9)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(5),
      this.instances3.alice.encrypt64(9),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 3 (9, 9)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(9),
      this.instances3.alice.encrypt64(9),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 4 (9, 5)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(9),
      this.instances3.alice.encrypt64(5),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 1 (1, 4154)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt64(4154),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 1 (1, 14889)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt64(14889),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 1 (1, 3917)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt64(3917),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 1 (1, 23898)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt64(23898),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 1 (1, 7578)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt64(7578),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 1 (2, 2714)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(2),
      this.instances3.alice.encrypt64(2714),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 1 (1, 3602)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt64(3602),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 1 (1, 3622)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(1),
      this.instances3.alice.encrypt64(3622),
    );
    expect(res).to.equal(3622n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(4),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(8),
      this.instances3.alice.encrypt64(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (3, 2)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(3), 2);
    expect(res).to.equal(5n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(4), 8);
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(8), 8);
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(8), 4);
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (2, 2)', async function () {
    const res = await this.contract3.add_uint16_euint16(2, this.instances3.alice.encrypt16(2));
    expect(res).to.equal(4n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.add_uint16_euint16(4, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.add_uint16_euint16(8, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.add_uint16_euint16(8, this.instances3.alice.encrypt16(4));
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (8, 8)', async function () {
    const res = await this.contract3.sub_euint16_uint16(this.instances3.alice.encrypt16(8), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 2 (8, 4)', async function () {
    const res = await this.contract3.sub_euint16_uint16(this.instances3.alice.encrypt16(8), 4);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (8, 8)', async function () {
    const res = await this.contract3.sub_uint16_euint16(8, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (8, 4)', async function () {
    const res = await this.contract3.sub_uint16_euint16(8, this.instances3.alice.encrypt16(4));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (1, 14)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(1), 14);
    expect(res).to.equal(14n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(4), 8);
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(8), 8);
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(8), 4);
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (2, 14)', async function () {
    const res = await this.contract3.mul_uint16_euint16(2, this.instances3.alice.encrypt16(14));
    expect(res).to.equal(28n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.mul_uint16_euint16(4, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.mul_uint16_euint16(8, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.mul_uint16_euint16(8, this.instances3.alice.encrypt16(4));
    expect(res).to.equal(32n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (1, 1)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(1), 1);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(4), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(8), 8);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(8), 4);
    expect(res).to.equal(2n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (1, 3)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(1), 3);
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(4), 8);
    expect(res).to.equal(4n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(8), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(8), 4);
    expect(res).to.equal(0n);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (5, 1)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(5), 1);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(4), 8);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(8), 8);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(8), 4);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract3.eq_uint16_euint16(1, this.instances3.alice.encrypt16(1));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.eq_uint16_euint16(4, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.eq_uint16_euint16(8, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.eq_uint16_euint16(8, this.instances3.alice.encrypt16(4));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (3, 6)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(3), 6);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(4), 8);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(8), 8);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(8), 4);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (1, 6)', async function () {
    const res = await this.contract3.ne_uint16_euint16(1, this.instances3.alice.encrypt16(6));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ne_uint16_euint16(4, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ne_uint16_euint16(8, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ne_uint16_euint16(8, this.instances3.alice.encrypt16(4));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(1), 1);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(4), 8);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(8), 8);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(8), 4);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract3.ge_uint16_euint16(1, this.instances3.alice.encrypt16(1));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ge_uint16_euint16(4, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ge_uint16_euint16(8, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ge_uint16_euint16(8, this.instances3.alice.encrypt16(4));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(1), 1);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(4), 8);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(8), 8);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(8), 4);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract3.gt_uint16_euint16(1, this.instances3.alice.encrypt16(1));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.gt_uint16_euint16(4, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.gt_uint16_euint16(8, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.gt_uint16_euint16(8, this.instances3.alice.encrypt16(4));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (23, 1)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(23), 1);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(4), 8);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(8), 8);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(8), 4);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract3.le_uint16_euint16(1, this.instances3.alice.encrypt16(1));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.le_uint16_euint16(4, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.le_uint16_euint16(8, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.le_uint16_euint16(8, this.instances3.alice.encrypt16(4));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (1, 8)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(1), 8);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(4), 8);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(8), 8);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(8), 4);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (2, 8)', async function () {
    const res = await this.contract3.lt_uint16_euint16(2, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.lt_uint16_euint16(4, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.lt_uint16_euint16(8, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.lt_uint16_euint16(8, this.instances3.alice.encrypt16(4));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (1, 2)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(1), 2);
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(4), 8);
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(8), 8);
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(8), 4);
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (1, 2)', async function () {
    const res = await this.contract3.min_uint16_euint16(1, this.instances3.alice.encrypt16(2));
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.min_uint16_euint16(4, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.min_uint16_euint16(8, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.min_uint16_euint16(8, this.instances3.alice.encrypt16(4));
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (2, 34)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(2), 34);
    expect(res).to.equal(34n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(4), 8);
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(8), 8);
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(8), 4);
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (1, 34)', async function () {
    const res = await this.contract3.max_uint16_euint16(1, this.instances3.alice.encrypt16(34));
    expect(res).to.equal(34n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.max_uint16_euint16(4, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.max_uint16_euint16(8, this.instances3.alice.encrypt16(8));
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.max_uint16_euint16(8, this.instances3.alice.encrypt16(4));
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 1 (2, 8)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(2),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(10n);
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

  it('test operator "mul" overload (euint32, euint4) => euint32 test 1 (1, 2)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(1),
      this.instances3.alice.encrypt4(2),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 2 (3, 5)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(3),
      this.instances3.alice.encrypt4(5),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 3 (3, 3)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(3),
      this.instances3.alice.encrypt4(3),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 4 (5, 3)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(5),
      this.instances3.alice.encrypt4(3),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 1 (1, 2)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(1),
      this.instances3.alice.encrypt4(2),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 1 (2, 15)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(2),
      this.instances3.alice.encrypt4(15),
    );
    expect(res).to.equal(15n);
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

  it('test operator "xor" overload (euint32, euint4) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(1),
      this.instances3.alice.encrypt4(1),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 1 (1, 3)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(1),
      this.instances3.alice.encrypt4(3),
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

  it('test operator "ne" overload (euint32, euint4) => ebool test 1 (24, 15)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(24),
      this.instances3.alice.encrypt4(15),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 2 (11, 15)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(11),
      this.instances3.alice.encrypt4(15),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 3 (15, 15)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(15),
      this.instances3.alice.encrypt4(15),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 4 (15, 11)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(15),
      this.instances3.alice.encrypt4(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 1 (1, 15)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(1),
      this.instances3.alice.encrypt4(15),
    );
    expect(res).to.equal(false);
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

  it('test operator "gt" overload (euint32, euint4) => ebool test 1 (1, 15)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(1),
      this.instances3.alice.encrypt4(15),
    );
    expect(res).to.equal(false);
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

  it('test operator "le" overload (euint32, euint4) => ebool test 1 (25, 15)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(25),
      this.instances3.alice.encrypt4(15),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 2 (11, 15)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(11),
      this.instances3.alice.encrypt4(15),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 3 (15, 15)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(15),
      this.instances3.alice.encrypt4(15),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 4 (15, 11)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(15),
      this.instances3.alice.encrypt4(11),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 1 (2, 1)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(2),
      this.instances3.alice.encrypt4(1),
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

  it('test operator "min" overload (euint32, euint4) => euint32 test 1 (3, 15)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(3),
      this.instances3.alice.encrypt4(15),
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

  it('test operator "max" overload (euint32, euint4) => euint32 test 1 (9, 5)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(9),
      this.instances3.alice.encrypt4(5),
    );
    expect(res).to.equal(9n);
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

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (3, 1)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(3),
      this.instances3.alice.encrypt8(1),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (8, 8)', async function () {
    const res = await this.contract3.sub_euint32_euint8(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (8, 4)', async function () {
    const res = await this.contract3.sub_euint32_euint8(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (1, 3)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(1),
      this.instances3.alice.encrypt8(3),
    );
    expect(res).to.equal(3n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (1, 85)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(1),
      this.instances3.alice.encrypt8(85),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(4),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(8),
      this.instances3.alice.encrypt8(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (2, 1)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(2),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(3n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (24, 13)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(24),
      this.instances4.alice.encrypt8(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(9),
      this.instances4.alice.encrypt8(13),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(13),
      this.instances4.alice.encrypt8(13),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(13),
      this.instances4.alice.encrypt8(9),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt8(1),
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

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (1, 7)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt8(7),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (25, 1)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(25),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (2, 4)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(2),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (3, 2)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(3),
      this.instances4.alice.encrypt8(2),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (9, 2)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(9),
      this.instances4.alice.encrypt8(2),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (3, 5)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(3),
      this.instances4.alice.encrypt16(5),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (8, 8)', async function () {
    const res = await this.contract4.sub_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (8, 4)', async function () {
    const res = await this.contract4.sub_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (1, 4)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt16(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (2, 1)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(2),
      this.instances4.alice.encrypt16(1),
    );
    expect(res).to.equal(3n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt16(1),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (1, 5)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt16(5),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (24, 2)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(24),
      this.instances4.alice.encrypt16(2),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt16(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt16(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (25, 1)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(25),
      this.instances4.alice.encrypt16(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (2, 6)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(2),
      this.instances4.alice.encrypt16(6),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (3, 3)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(3),
      this.instances4.alice.encrypt16(3),
    );
    expect(res).to.equal(3n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (9, 2)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(9),
      this.instances4.alice.encrypt16(2),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt16(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (3, 1)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(3),
      this.instances4.alice.encrypt32(1),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (8, 8)', async function () {
    const res = await this.contract4.sub_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (8, 4)', async function () {
    const res = await this.contract4.sub_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt32(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(4),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt32(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (2, 2)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(2),
      this.instances4.alice.encrypt32(2),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt32(1),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (1, 54)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt32(54),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (24, 1)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(24),
      this.instances4.alice.encrypt32(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (1, 22)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt32(22),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt32(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (25, 1)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(25),
      this.instances4.alice.encrypt32(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (2, 25)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(2),
      this.instances4.alice.encrypt32(25),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (3, 5)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(3),
      this.instances4.alice.encrypt32(5),
    );
    expect(res).to.equal(3n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (9, 6)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(9),
      this.instances4.alice.encrypt32(6),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt32(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 1 (8, 2055)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(2055),
    );
    expect(res).to.equal(2063n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (8, 8)', async function () {
    const res = await this.contract4.sub_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (8, 4)', async function () {
    const res = await this.contract4.sub_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (1, 2410)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt64(2410),
    );
    expect(res).to.equal(2410n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(4),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 1 (1, 3270)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt64(3270),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 1 (2, 3577)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(2),
      this.instances4.alice.encrypt64(3577),
    );
    expect(res).to.equal(3579n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (1, 2447)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt64(2447),
    );
    expect(res).to.equal(2446n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 1 (1, 2530)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt64(2530),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (1, 2583)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt64(2583),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 1 (2, 2385)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(2),
      this.instances4.alice.encrypt64(2385),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 1 (1, 3010)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(1),
      this.instances4.alice.encrypt64(3010),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 1 (2, 2235)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(2),
      this.instances4.alice.encrypt64(2235),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 1 (4, 5352)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt64(5352),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 1 (45, 5751)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(45),
      this.instances4.alice.encrypt64(5751),
    );
    expect(res).to.equal(45n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 2 (41, 45)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(41),
      this.instances4.alice.encrypt64(45),
    );
    expect(res).to.equal(41n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 3 (45, 45)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(45),
      this.instances4.alice.encrypt64(45),
    );
    expect(res).to.equal(45n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 4 (45, 41)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(45),
      this.instances4.alice.encrypt64(41),
    );
    expect(res).to.equal(41n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (4, 2425)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt64(2425),
    );
    expect(res).to.equal(2425n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(4),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(8),
      this.instances4.alice.encrypt64(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (3, 1)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(3), 1);
    expect(res).to.equal(4n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(4), 8);
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(8), 8);
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(8), 4);
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (8, 1)', async function () {
    const res = await this.contract4.add_uint32_euint32(8, this.instances4.alice.encrypt32(1));
    expect(res).to.equal(9n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.add_uint32_euint32(4, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.add_uint32_euint32(8, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.add_uint32_euint32(8, this.instances4.alice.encrypt32(4));
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (8, 8)', async function () {
    const res = await this.contract4.sub_euint32_uint32(this.instances4.alice.encrypt32(8), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 2 (8, 4)', async function () {
    const res = await this.contract4.sub_euint32_uint32(this.instances4.alice.encrypt32(8), 4);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (8, 8)', async function () {
    const res = await this.contract4.sub_uint32_euint32(8, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 2 (8, 4)', async function () {
    const res = await this.contract4.sub_uint32_euint32(8, this.instances4.alice.encrypt32(4));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (1, 4)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(1), 4);
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(4), 8);
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(8), 8);
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(8), 4);
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (1, 4)', async function () {
    const res = await this.contract4.mul_uint32_euint32(1, this.instances4.alice.encrypt32(4));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.mul_uint32_euint32(4, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.mul_uint32_euint32(8, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.mul_uint32_euint32(8, this.instances4.alice.encrypt32(4));
    expect(res).to.equal(32n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (1, 12)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(1), 12);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(4), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(8), 8);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(8), 4);
    expect(res).to.equal(2n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (1, 1)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(1), 1);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(4), 8);
    expect(res).to.equal(4n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(8), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(8), 4);
    expect(res).to.equal(0n);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (1, 3)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(1), 3);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(4), 8);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(8), 8);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(8), 4);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (1, 3)', async function () {
    const res = await this.contract4.eq_uint32_euint32(1, this.instances4.alice.encrypt32(3));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.eq_uint32_euint32(4, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.eq_uint32_euint32(8, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.eq_uint32_euint32(8, this.instances4.alice.encrypt32(4));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (24, 1)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(24), 1);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(4), 8);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(8), 8);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(8), 4);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract4.ne_uint32_euint32(1, this.instances4.alice.encrypt32(1));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ne_uint32_euint32(4, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ne_uint32_euint32(8, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ne_uint32_euint32(8, this.instances4.alice.encrypt32(4));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (1, 2)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(1), 2);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(4), 8);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(8), 8);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(8), 4);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (2, 2)', async function () {
    const res = await this.contract4.ge_uint32_euint32(2, this.instances4.alice.encrypt32(2));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ge_uint32_euint32(4, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ge_uint32_euint32(8, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ge_uint32_euint32(8, this.instances4.alice.encrypt32(4));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (1, 3)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(1), 3);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(4), 8);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(8), 8);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(8), 4);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (1, 3)', async function () {
    const res = await this.contract4.gt_uint32_euint32(1, this.instances4.alice.encrypt32(3));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.gt_uint32_euint32(4, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.gt_uint32_euint32(8, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.gt_uint32_euint32(8, this.instances4.alice.encrypt32(4));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (25, 3)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(25), 3);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(4), 8);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(8), 8);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(8), 4);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (2, 3)', async function () {
    const res = await this.contract4.le_uint32_euint32(2, this.instances4.alice.encrypt32(3));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.le_uint32_euint32(4, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.le_uint32_euint32(8, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.le_uint32_euint32(8, this.instances4.alice.encrypt32(4));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (2, 2)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(2), 2);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(4), 8);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(8), 8);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(8), 4);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (4, 2)', async function () {
    const res = await this.contract4.lt_uint32_euint32(4, this.instances4.alice.encrypt32(2));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.lt_uint32_euint32(4, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.lt_uint32_euint32(8, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.lt_uint32_euint32(8, this.instances4.alice.encrypt32(4));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (3, 3)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(3), 3);
    expect(res).to.equal(3n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(4), 8);
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(8), 8);
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(8), 4);
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (45, 3)', async function () {
    const res = await this.contract4.min_uint32_euint32(45, this.instances4.alice.encrypt32(3));
    expect(res).to.equal(3n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.min_uint32_euint32(4, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.min_uint32_euint32(8, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.min_uint32_euint32(8, this.instances4.alice.encrypt32(4));
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (9, 12)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(9), 12);
    expect(res).to.equal(12n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(4), 8);
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(8), 8);
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(8), 4);
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (4, 12)', async function () {
    const res = await this.contract4.max_uint32_euint32(4, this.instances4.alice.encrypt32(12));
    expect(res).to.equal(12n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract4.max_uint32_euint32(4, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract4.max_uint32_euint32(8, this.instances4.alice.encrypt32(8));
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract4.max_uint32_euint32(8, this.instances4.alice.encrypt32(4));
    expect(res).to.equal(8n);
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

  it('test operator "add" overload (euint64, euint4) => euint64 test 3 (5, 5)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(5),
      this.instances4.alice.encrypt4(5),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
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

  it('test operator "mul" overload (euint64, euint4) => euint64 test 1 (15, 1)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(15),
      this.instances4.alice.encrypt4(1),
    );
    expect(res).to.equal(15n);
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

  it('test operator "and" overload (euint64, euint4) => euint64 test 1 (2380, 1)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(2380),
      this.instances4.alice.encrypt4(1),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 1 (3328, 3)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(3328),
      this.instances4.alice.encrypt4(3),
    );
    expect(res).to.equal(3331n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 1 (5602, 3)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(5602),
      this.instances4.alice.encrypt4(3),
    );
    expect(res).to.equal(5601n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 1 (55032, 1)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(55032),
      this.instances4.alice.encrypt4(1),
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

  it('test operator "ne" overload (euint64, euint4) => ebool test 1 (3163, 15)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(3163),
      this.instances4.alice.encrypt4(15),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 2 (11, 15)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(11),
      this.instances4.alice.encrypt4(15),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 3 (15, 15)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(15),
      this.instances4.alice.encrypt4(15),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 4 (15, 11)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(15),
      this.instances4.alice.encrypt4(11),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 1 (3383, 2)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(3383),
      this.instances4.alice.encrypt4(2),
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

  it('test operator "gt" overload (euint64, euint4) => ebool test 1 (2050, 1)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(2050),
      this.instances4.alice.encrypt4(1),
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

  it('test operator "le" overload (euint64, euint4) => ebool test 1 (4742, 7)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(4742),
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

  it('test operator "lt" overload (euint64, euint4) => ebool test 1 (16329, 1)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(16329),
      this.instances4.alice.encrypt4(1),
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

  it('test operator "min" overload (euint64, euint4) => euint64 test 1 (2618, 3)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(2618),
      this.instances4.alice.encrypt4(3),
    );
    expect(res).to.equal(3n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt4(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 1 (2374, 3)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(2374),
      this.instances4.alice.encrypt4(3),
    );
    expect(res).to.equal(2374n);
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

  it('test operator "add" overload (euint64, euint8) => euint64 test 1 (141, 2)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(141),
      this.instances4.alice.encrypt8(2),
    );
    expect(res).to.equal(143n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 2 (10, 14)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(10),
      this.instances4.alice.encrypt8(14),
    );
    expect(res).to.equal(24n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 3 (14, 14)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(14),
      this.instances4.alice.encrypt8(14),
    );
    expect(res).to.equal(28n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 4 (14, 10)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(14),
      this.instances4.alice.encrypt8(10),
    );
    expect(res).to.equal(24n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 1 (8, 8)', async function () {
    const res = await this.contract4.sub_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 2 (8, 4)', async function () {
    const res = await this.contract4.sub_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 1 (109, 2)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(109),
      this.instances4.alice.encrypt8(2),
    );
    expect(res).to.equal(218n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 1 (2380, 1)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(2380),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 1 (3328, 1)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(3328),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(3329n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 1 (5602, 5)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(5602),
      this.instances4.alice.encrypt8(5),
    );
    expect(res).to.equal(5607n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 1 (55032, 1)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(55032),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 1 (3163, 2)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(3163),
      this.instances4.alice.encrypt8(2),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 1 (3383, 1)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(3383),
      this.instances4.alice.encrypt8(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(4),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 1 (2050, 12)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(2050),
      this.instances4.alice.encrypt8(12),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(8),
      this.instances4.alice.encrypt8(12),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(12),
      this.instances4.alice.encrypt8(12),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(12),
      this.instances4.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 1 (4742, 1)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(4742),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 1 (16329, 1)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(16329),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 1 (2618, 1)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(2618),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt8(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 1 (2374, 1)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(2374),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(2374n);
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

  it('test operator "add" overload (euint64, euint16) => euint64 test 1 (17823, 1)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(17823),
      this.instances5.alice.encrypt16(1),
    );
    expect(res).to.equal(17824n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 1 (8, 8)', async function () {
    const res = await this.contract5.sub_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 2 (8, 4)', async function () {
    const res = await this.contract5.sub_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 1 (3449, 2)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(3449),
      this.instances5.alice.encrypt16(2),
    );
    expect(res).to.equal(6898n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(4),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 1 (2380, 9)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(2380),
      this.instances5.alice.encrypt16(9),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 2 (5, 9)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(5),
      this.instances5.alice.encrypt16(9),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 3 (9, 9)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(9),
      this.instances5.alice.encrypt16(9),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 4 (9, 5)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(9),
      this.instances5.alice.encrypt16(5),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 1 (3328, 2)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(3328),
      this.instances5.alice.encrypt16(2),
    );
    expect(res).to.equal(3330n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 1 (5602, 9)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(5602),
      this.instances5.alice.encrypt16(9),
    );
    expect(res).to.equal(5611n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 2 (5, 9)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(5),
      this.instances5.alice.encrypt16(9),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 3 (9, 9)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(9),
      this.instances5.alice.encrypt16(9),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 4 (9, 5)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(9),
      this.instances5.alice.encrypt16(5),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 1 (55032, 3)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(55032),
      this.instances5.alice.encrypt16(3),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 1 (3163, 2)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(3163),
      this.instances5.alice.encrypt16(2),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 1 (3383, 2)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(3383),
      this.instances5.alice.encrypt16(2),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 1 (2050, 6)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(2050),
      this.instances5.alice.encrypt16(6),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 1 (4742, 1)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(4742),
      this.instances5.alice.encrypt16(1),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 1 (16329, 3)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(16329),
      this.instances5.alice.encrypt16(3),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 1 (2618, 5)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(2618),
      this.instances5.alice.encrypt16(5),
    );
    expect(res).to.equal(5n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (2374, 1)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(2374),
      this.instances5.alice.encrypt16(1),
    );
    expect(res).to.equal(2374n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt16(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 1 (17823, 2)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(17823),
      this.instances5.alice.encrypt32(2),
    );
    expect(res).to.equal(17825n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 1 (8, 8)', async function () {
    const res = await this.contract5.sub_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 2 (8, 4)', async function () {
    const res = await this.contract5.sub_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 1 (3449, 9)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(3449),
      this.instances5.alice.encrypt32(9),
    );
    expect(res).to.equal(31041n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 2 (5, 9)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(5),
      this.instances5.alice.encrypt32(9),
    );
    expect(res).to.equal(45n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 3 (9, 9)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(9),
      this.instances5.alice.encrypt32(9),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 4 (9, 5)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(9),
      this.instances5.alice.encrypt32(5),
    );
    expect(res).to.equal(45n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 1 (2380, 4)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(2380),
      this.instances5.alice.encrypt32(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(4),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 1 (3328, 3)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(3328),
      this.instances5.alice.encrypt32(3),
    );
    expect(res).to.equal(3331n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 1 (5602, 2)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(5602),
      this.instances5.alice.encrypt32(2),
    );
    expect(res).to.equal(5600n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(4),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 1 (55032, 3)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(55032),
      this.instances5.alice.encrypt32(3),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 1 (3163, 1)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(3163),
      this.instances5.alice.encrypt32(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 1 (3383, 8)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(3383),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 1 (2050, 1)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(2050),
      this.instances5.alice.encrypt32(1),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(4),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (4742, 21)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(4742),
      this.instances5.alice.encrypt32(21),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (17, 21)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(17),
      this.instances5.alice.encrypt32(21),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (21, 21)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(21),
      this.instances5.alice.encrypt32(21),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (21, 17)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(21),
      this.instances5.alice.encrypt32(17),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (16329, 2)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(16329),
      this.instances5.alice.encrypt32(2),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(4),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (2618, 1)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(2618),
      this.instances5.alice.encrypt32(1),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(4),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (2374, 2)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(2374),
      this.instances5.alice.encrypt32(2),
    );
    expect(res).to.equal(2374n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(4),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(8),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(8),
      this.instances5.alice.encrypt32(4),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (17823, 3432)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(17823),
      this.instances5.alice.encrypt64(3432),
    );
    expect(res).to.equal(21255n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (3428, 3432)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(3428),
      this.instances5.alice.encrypt64(3432),
    );
    expect(res).to.equal(6860n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (3432, 3432)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(3432),
      this.instances5.alice.encrypt64(3432),
    );
    expect(res).to.equal(6864n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (3432, 3428)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(3432),
      this.instances5.alice.encrypt64(3428),
    );
    expect(res).to.equal(6860n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (3660, 3660)', async function () {
    const res = await this.contract5.sub_euint64_euint64(
      this.instances5.alice.encrypt64(3660),
      this.instances5.alice.encrypt64(3660),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (3660, 3656)', async function () {
    const res = await this.contract5.sub_euint64_euint64(
      this.instances5.alice.encrypt64(3660),
      this.instances5.alice.encrypt64(3656),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (3449, 2396)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(3449),
      this.instances5.alice.encrypt64(2396),
    );
    expect(res).to.equal(8263804n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (2392, 2396)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(2392),
      this.instances5.alice.encrypt64(2396),
    );
    expect(res).to.equal(5731232n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (2396, 2396)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(2396),
      this.instances5.alice.encrypt64(2396),
    );
    expect(res).to.equal(5740816n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (2396, 2392)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(2396),
      this.instances5.alice.encrypt64(2392),
    );
    expect(res).to.equal(5731232n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (2380, 2068)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(2380),
      this.instances5.alice.encrypt64(2068),
    );
    expect(res).to.equal(2052n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (2064, 2068)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(2064),
      this.instances5.alice.encrypt64(2068),
    );
    expect(res).to.equal(2064n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (2068, 2068)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(2068),
      this.instances5.alice.encrypt64(2068),
    );
    expect(res).to.equal(2068n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (2068, 2064)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(2068),
      this.instances5.alice.encrypt64(2064),
    );
    expect(res).to.equal(2064n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 1 (3328, 2096)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(3328),
      this.instances5.alice.encrypt64(2096),
    );
    expect(res).to.equal(3376n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 2 (2092, 2096)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(2092),
      this.instances5.alice.encrypt64(2096),
    );
    expect(res).to.equal(2108n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 3 (2096, 2096)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(2096),
      this.instances5.alice.encrypt64(2096),
    );
    expect(res).to.equal(2096n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 4 (2096, 2092)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(2096),
      this.instances5.alice.encrypt64(2092),
    );
    expect(res).to.equal(2108n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 1 (5602, 4581)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(5602),
      this.instances5.alice.encrypt64(4581),
    );
    expect(res).to.equal(1031n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 2 (4577, 4581)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(4577),
      this.instances5.alice.encrypt64(4581),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 3 (4581, 4581)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(4581),
      this.instances5.alice.encrypt64(4581),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 4 (4581, 4577)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(4581),
      this.instances5.alice.encrypt64(4577),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 1 (55032, 3655)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(55032),
      this.instances5.alice.encrypt64(3655),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 2 (3651, 3655)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(3651),
      this.instances5.alice.encrypt64(3655),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 3 (3655, 3655)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(3655),
      this.instances5.alice.encrypt64(3655),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 4 (3655, 3651)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(3655),
      this.instances5.alice.encrypt64(3651),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (3163, 2116)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(3163),
      this.instances5.alice.encrypt64(2116),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (2112, 2116)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(2112),
      this.instances5.alice.encrypt64(2116),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (2116, 2116)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(2116),
      this.instances5.alice.encrypt64(2116),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (2116, 2112)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(2116),
      this.instances5.alice.encrypt64(2112),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 1 (3383, 7363)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(3383),
      this.instances5.alice.encrypt64(7363),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 2 (3379, 3383)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(3379),
      this.instances5.alice.encrypt64(3383),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 3 (3383, 3383)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(3383),
      this.instances5.alice.encrypt64(3383),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 4 (3383, 3379)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(3383),
      this.instances5.alice.encrypt64(3379),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (2050, 2503)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(2050),
      this.instances5.alice.encrypt64(2503),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (2046, 2050)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(2046),
      this.instances5.alice.encrypt64(2050),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (2050, 2050)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(2050),
      this.instances5.alice.encrypt64(2050),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (2050, 2046)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(2050),
      this.instances5.alice.encrypt64(2046),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (4742, 3626)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(4742),
      this.instances5.alice.encrypt64(3626),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (3622, 3626)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(3622),
      this.instances5.alice.encrypt64(3626),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (3626, 3626)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(3626),
      this.instances5.alice.encrypt64(3626),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (3626, 3622)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(3626),
      this.instances5.alice.encrypt64(3622),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 1 (16329, 2082)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(16329),
      this.instances5.alice.encrypt64(2082),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 2 (2078, 2082)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(2078),
      this.instances5.alice.encrypt64(2082),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 3 (2082, 2082)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(2082),
      this.instances5.alice.encrypt64(2082),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 4 (2082, 2078)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(2082),
      this.instances5.alice.encrypt64(2078),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 1 (2618, 6448)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(2618),
      this.instances5.alice.encrypt64(6448),
    );
    expect(res).to.equal(2618n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 2 (2614, 2618)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(2614),
      this.instances5.alice.encrypt64(2618),
    );
    expect(res).to.equal(2614n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 3 (2618, 2618)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(2618),
      this.instances5.alice.encrypt64(2618),
    );
    expect(res).to.equal(2618n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 4 (2618, 2614)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(2618),
      this.instances5.alice.encrypt64(2614),
    );
    expect(res).to.equal(2614n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (2374, 2762)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(2374),
      this.instances5.alice.encrypt64(2762),
    );
    expect(res).to.equal(2762n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (2370, 2374)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(2370),
      this.instances5.alice.encrypt64(2374),
    );
    expect(res).to.equal(2374n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (2374, 2374)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(2374),
      this.instances5.alice.encrypt64(2374),
    );
    expect(res).to.equal(2374n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (2374, 2370)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(2374),
      this.instances5.alice.encrypt64(2370),
    );
    expect(res).to.equal(2374n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (17823, 7318)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(17823), 7318);
    expect(res).to.equal(25141n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (3428, 3432)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(3428), 3432);
    expect(res).to.equal(6860n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (3432, 3432)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(3432), 3432);
    expect(res).to.equal(6864n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (3432, 3428)', async function () {
    const res = await this.contract5.add_euint64_uint64(this.instances5.alice.encrypt64(3432), 3428);
    expect(res).to.equal(6860n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 1 (5752, 7318)', async function () {
    const res = await this.contract5.add_uint64_euint64(5752, this.instances5.alice.encrypt64(7318));
    expect(res).to.equal(13070n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 2 (3428, 3432)', async function () {
    const res = await this.contract5.add_uint64_euint64(3428, this.instances5.alice.encrypt64(3432));
    expect(res).to.equal(6860n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 3 (3432, 3432)', async function () {
    const res = await this.contract5.add_uint64_euint64(3432, this.instances5.alice.encrypt64(3432));
    expect(res).to.equal(6864n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 4 (3432, 3428)', async function () {
    const res = await this.contract5.add_uint64_euint64(3432, this.instances5.alice.encrypt64(3428));
    expect(res).to.equal(6860n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (3660, 3660)', async function () {
    const res = await this.contract5.sub_euint64_uint64(this.instances5.alice.encrypt64(3660), 3660);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (3660, 3656)', async function () {
    const res = await this.contract5.sub_euint64_uint64(this.instances5.alice.encrypt64(3660), 3656);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (3660, 3660)', async function () {
    const res = await this.contract5.sub_uint64_euint64(3660, this.instances5.alice.encrypt64(3660));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (3660, 3656)', async function () {
    const res = await this.contract5.sub_uint64_euint64(3660, this.instances5.alice.encrypt64(3656));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (3449, 4919)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(3449), 4919);
    expect(res).to.equal(16965631n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (2392, 2396)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(2392), 2396);
    expect(res).to.equal(5731232n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (2396, 2396)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(2396), 2396);
    expect(res).to.equal(5740816n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (2396, 2392)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(2396), 2392);
    expect(res).to.equal(5731232n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 1 (2383, 4919)', async function () {
    const res = await this.contract5.mul_uint64_euint64(2383, this.instances5.alice.encrypt64(4919));
    expect(res).to.equal(11721977n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 2 (2392, 2396)', async function () {
    const res = await this.contract5.mul_uint64_euint64(2392, this.instances5.alice.encrypt64(2396));
    expect(res).to.equal(5731232n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 3 (2396, 2396)', async function () {
    const res = await this.contract5.mul_uint64_euint64(2396, this.instances5.alice.encrypt64(2396));
    expect(res).to.equal(5740816n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 4 (2396, 2392)', async function () {
    const res = await this.contract5.mul_uint64_euint64(2396, this.instances5.alice.encrypt64(2392));
    expect(res).to.equal(5731232n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (3692, 2899)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(3692), 2899);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (3688, 3692)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(3688), 3692);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (3692, 3692)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(3692), 3692);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (3692, 3688)', async function () {
    const res = await this.contract5.div_euint64_uint64(this.instances5.alice.encrypt64(3692), 3688);
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 1 (2467, 230368)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(2467), 230368);
    expect(res).to.equal(2467n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 2 (2168, 2172)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(2168), 2172);
    expect(res).to.equal(2168n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 3 (2172, 2172)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(2172), 2172);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 4 (2172, 2168)', async function () {
    const res = await this.contract5.rem_euint64_uint64(this.instances5.alice.encrypt64(2172), 2168);
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (55032, 2077)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(55032), 2077);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (3651, 3655)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(3651), 3655);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (3655, 3655)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(3655), 3655);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (3655, 3651)', async function () {
    const res = await this.contract5.eq_euint64_uint64(this.instances5.alice.encrypt64(3655), 3651);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (6350, 2077)', async function () {
    const res = await this.contract5.eq_uint64_euint64(6350, this.instances5.alice.encrypt64(2077));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (3651, 3655)', async function () {
    const res = await this.contract5.eq_uint64_euint64(3651, this.instances5.alice.encrypt64(3655));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (3655, 3655)', async function () {
    const res = await this.contract5.eq_uint64_euint64(3655, this.instances5.alice.encrypt64(3655));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (3655, 3651)', async function () {
    const res = await this.contract5.eq_uint64_euint64(3655, this.instances5.alice.encrypt64(3651));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (3163, 3329)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(3163), 3329);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (2112, 2116)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(2112), 2116);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (2116, 2116)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(2116), 2116);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (2116, 2112)', async function () {
    const res = await this.contract5.ne_euint64_uint64(this.instances5.alice.encrypt64(2116), 2112);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (2985, 3329)', async function () {
    const res = await this.contract5.ne_uint64_euint64(2985, this.instances5.alice.encrypt64(3329));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (2112, 2116)', async function () {
    const res = await this.contract5.ne_uint64_euint64(2112, this.instances5.alice.encrypt64(2116));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (2116, 2116)', async function () {
    const res = await this.contract5.ne_uint64_euint64(2116, this.instances5.alice.encrypt64(2116));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (2116, 2112)', async function () {
    const res = await this.contract5.ne_uint64_euint64(2116, this.instances5.alice.encrypt64(2112));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (3383, 3479)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(3383), 3479);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (3379, 3383)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(3379), 3383);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (3383, 3383)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(3383), 3383);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (3383, 3379)', async function () {
    const res = await this.contract5.ge_euint64_uint64(this.instances5.alice.encrypt64(3383), 3379);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (27028, 3479)', async function () {
    const res = await this.contract5.ge_uint64_euint64(27028, this.instances5.alice.encrypt64(3479));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (3379, 3383)', async function () {
    const res = await this.contract5.ge_uint64_euint64(3379, this.instances5.alice.encrypt64(3383));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (3383, 3383)', async function () {
    const res = await this.contract5.ge_uint64_euint64(3383, this.instances5.alice.encrypt64(3383));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (3383, 3379)', async function () {
    const res = await this.contract5.ge_uint64_euint64(3383, this.instances5.alice.encrypt64(3379));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (2050, 4117)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(2050), 4117);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (2046, 2050)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(2046), 2050);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (2050, 2050)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(2050), 2050);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (2050, 2046)', async function () {
    const res = await this.contract5.gt_euint64_uint64(this.instances5.alice.encrypt64(2050), 2046);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 1 (44963, 4117)', async function () {
    const res = await this.contract5.gt_uint64_euint64(44963, this.instances5.alice.encrypt64(4117));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 2 (2046, 2050)', async function () {
    const res = await this.contract5.gt_uint64_euint64(2046, this.instances5.alice.encrypt64(2050));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 3 (2050, 2050)', async function () {
    const res = await this.contract5.gt_uint64_euint64(2050, this.instances5.alice.encrypt64(2050));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 4 (2050, 2046)', async function () {
    const res = await this.contract5.gt_uint64_euint64(2050, this.instances5.alice.encrypt64(2046));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (4742, 42694)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(4742), 42694);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (3622, 3626)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(3622), 3626);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (3626, 3626)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(3626), 3626);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (3626, 3622)', async function () {
    const res = await this.contract5.le_euint64_uint64(this.instances5.alice.encrypt64(3626), 3622);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (2150, 42694)', async function () {
    const res = await this.contract5.le_uint64_euint64(2150, this.instances5.alice.encrypt64(42694));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (3622, 3626)', async function () {
    const res = await this.contract5.le_uint64_euint64(3622, this.instances5.alice.encrypt64(3626));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (3626, 3626)', async function () {
    const res = await this.contract5.le_uint64_euint64(3626, this.instances5.alice.encrypt64(3626));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (3626, 3622)', async function () {
    const res = await this.contract5.le_uint64_euint64(3626, this.instances5.alice.encrypt64(3622));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (16329, 2853)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(16329), 2853);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (2078, 2082)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(2078), 2082);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (2082, 2082)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(2082), 2082);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (2082, 2078)', async function () {
    const res = await this.contract5.lt_euint64_uint64(this.instances5.alice.encrypt64(2082), 2078);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (27593, 2853)', async function () {
    const res = await this.contract5.lt_uint64_euint64(27593, this.instances5.alice.encrypt64(2853));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (2078, 2082)', async function () {
    const res = await this.contract5.lt_uint64_euint64(2078, this.instances5.alice.encrypt64(2082));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (2082, 2082)', async function () {
    const res = await this.contract5.lt_uint64_euint64(2082, this.instances5.alice.encrypt64(2082));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (2082, 2078)', async function () {
    const res = await this.contract5.lt_uint64_euint64(2082, this.instances5.alice.encrypt64(2078));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 1 (2618, 4687)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(2618), 4687);
    expect(res).to.equal(2618n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 2 (2614, 2618)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(2614), 2618);
    expect(res).to.equal(2614n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 3 (2618, 2618)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(2618), 2618);
    expect(res).to.equal(2618n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 4 (2618, 2614)', async function () {
    const res = await this.contract5.min_euint64_uint64(this.instances5.alice.encrypt64(2618), 2614);
    expect(res).to.equal(2614n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (3232, 4687)', async function () {
    const res = await this.contract5.min_uint64_euint64(3232, this.instances5.alice.encrypt64(4687));
    expect(res).to.equal(3232n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (2614, 2618)', async function () {
    const res = await this.contract5.min_uint64_euint64(2614, this.instances5.alice.encrypt64(2618));
    expect(res).to.equal(2614n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (2618, 2618)', async function () {
    const res = await this.contract5.min_uint64_euint64(2618, this.instances5.alice.encrypt64(2618));
    expect(res).to.equal(2618n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (2618, 2614)', async function () {
    const res = await this.contract5.min_uint64_euint64(2618, this.instances5.alice.encrypt64(2614));
    expect(res).to.equal(2614n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (2374, 5605)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(2374), 5605);
    expect(res).to.equal(5605n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (2370, 2374)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(2370), 2374);
    expect(res).to.equal(2374n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (2374, 2374)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(2374), 2374);
    expect(res).to.equal(2374n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (2374, 2370)', async function () {
    const res = await this.contract5.max_euint64_uint64(this.instances5.alice.encrypt64(2374), 2370);
    expect(res).to.equal(2374n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (7998, 5605)', async function () {
    const res = await this.contract5.max_uint64_euint64(7998, this.instances5.alice.encrypt64(5605));
    expect(res).to.equal(7998n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (2370, 2374)', async function () {
    const res = await this.contract5.max_uint64_euint64(2370, this.instances5.alice.encrypt64(2374));
    expect(res).to.equal(2374n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (2374, 2374)', async function () {
    const res = await this.contract5.max_uint64_euint64(2374, this.instances5.alice.encrypt64(2374));
    expect(res).to.equal(2374n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (2374, 2370)', async function () {
    const res = await this.contract5.max_uint64_euint64(2374, this.instances5.alice.encrypt64(2370));
    expect(res).to.equal(2374n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 1 (1, 3)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(1), 3);
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

  it('test operator "shr" overload (euint4, uint8) => euint4 test 1 (5, 6)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(5), 6);
    expect(res).to.equal(1);
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

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (1, 2)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(1),
      this.instances5.alice.encrypt8(2),
    );
    expect(res).to.equal(4);
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

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (1, 2)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(1), 2);
    expect(res).to.equal(4);
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

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (1, 1)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(1),
      this.instances5.alice.encrypt8(1),
    );
    expect(res).to.equal(0);
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

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (1, 1)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(1), 1);
    expect(res).to.equal(0);
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

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (1, 5)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(1),
      this.instances5.alice.encrypt8(5),
    );
    expect(res).to.equal(32);
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

  it('test operator "shl" overload (euint16, uint8) => euint16 test 1 (1, 5)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(1), 5);
    expect(res).to.equal(32);
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

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (278, 4)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(278),
      this.instances5.alice.encrypt8(4),
    );
    expect(res).to.equal(17);
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

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (278, 4)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(278), 4);
    expect(res).to.equal(17);
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

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (1, 6)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(1),
      this.instances5.alice.encrypt8(6),
    );
    expect(res).to.equal(64);
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

  it('test operator "shl" overload (euint32, uint8) => euint32 test 1 (1, 6)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(1), 6);
    expect(res).to.equal(64);
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

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (2, 6)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(2),
      this.instances5.alice.encrypt8(6),
    );
    expect(res).to.equal(0);
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

  it('test operator "shr" overload (euint32, uint8) => euint32 test 1 (2, 6)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(2), 6);
    expect(res).to.equal(0);
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

  it('test operator "shl" overload (euint64, euint8) => euint64 test 1 (4744, 2)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(4744),
      this.instances5.alice.encrypt8(2),
    );
    expect(res).to.equal(18976);
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

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (4744, 2)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(4744), 2);
    expect(res).to.equal(18976);
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

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (23325, 5)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(23325),
      this.instances5.alice.encrypt8(5),
    );
    expect(res).to.equal(728);
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

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (23325, 5)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(23325), 5);
    expect(res).to.equal(728);
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

  it('test operator "neg" overload (euint4) => euint4 test 1 (1)', async function () {
    const res = await this.contract5.neg_euint4(this.instances5.alice.encrypt4(1));
    expect(res).to.equal(15n);
  });

  it('test operator "not" overload (euint4) => euint4 test 1 (1)', async function () {
    const res = await this.contract5.not_euint4(this.instances5.alice.encrypt4(1));
    expect(res).to.equal(14n);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (1)', async function () {
    const res = await this.contract5.neg_euint8(this.instances5.alice.encrypt8(1));
    expect(res).to.equal(255n);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (1)', async function () {
    const res = await this.contract5.not_euint8(this.instances5.alice.encrypt8(1));
    expect(res).to.equal(254n);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (1)', async function () {
    const res = await this.contract5.neg_euint16(this.instances5.alice.encrypt16(1));
    expect(res).to.equal(65535n);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (5)', async function () {
    const res = await this.contract5.not_euint16(this.instances5.alice.encrypt16(5));
    expect(res).to.equal(65530n);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (1)', async function () {
    const res = await this.contract5.neg_euint32(this.instances5.alice.encrypt32(1));
    expect(res).to.equal(4294967295n);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (1)', async function () {
    const res = await this.contract5.not_euint32(this.instances5.alice.encrypt32(1));
    expect(res).to.equal(4294967294n);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (40511)', async function () {
    const res = await this.contract5.neg_euint64(this.instances5.alice.encrypt64(40511));
    expect(res).to.equal(18446744073709511105n);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (51608)', async function () {
    const res = await this.contract5.not_euint64(this.instances5.alice.encrypt64(51608));
    expect(res).to.equal(18446744073709500007n);
  });
});
