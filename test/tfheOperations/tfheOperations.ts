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

  it('test operator "add" overload (euint4, euint4) => euint4 test 1 (3, 7)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt4(7n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt4(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, euint4) => euint4 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint4) => euint4 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 1 (2, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt4(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt4(5n),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt4(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt4(3n),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 1 (3, 2)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt4(2n),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 1 (10, 3)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt4(3n),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 1 (14, 3)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt4(3n),
    );
    expect(res).to.equal(13n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 1 (1, 7)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(1n),
      this.instances1.alice.encrypt4(7n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 1 (9, 10)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt4(10n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt4(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt4(9n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt4(5n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 1 (3, 11)', async function () {
    const res = await this.contract1.ge_euint4_euint4(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt4(11n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint4(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 1 (11, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 1 (12, 8)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 1 (4, 11)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt4(11n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 1 (7, 1)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt4(1n),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 1 (2, 6)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt4(6n),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 1 (2, 10)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt8(10n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt8(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, euint8) => euint8 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint8) => euint8 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 1 (2, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt8(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt8(5n),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt8(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt8(3n),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 1 (4, 185)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt8(185n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(4n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 1 (3, 157)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt8(157n),
    );
    expect(res).to.equal(159n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 1 (4, 125)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt8(125n),
    );
    expect(res).to.equal(121n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 1 (7, 32)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt8(32n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 1 (11, 19)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt8(19n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt8(11n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt8(11n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt8(7n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 1 (1, 51)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(1n),
      this.instances1.alice.encrypt8(51n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 1 (2, 182)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt8(182n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 1 (10, 19)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt8(19n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt8(10n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt8(10n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt8(6n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 1 (3, 72)', async function () {
    const res = await this.contract1.lt_euint4_euint8(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt8(72n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint8(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 1 (14, 100)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt8(100n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 2 (10, 14)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt8(14n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 3 (14, 14)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt8(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 4 (14, 10)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt8(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 1 (12, 169)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt8(169n),
    );
    expect(res).to.equal(169n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 2 (8, 12)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(12n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 3 (12, 12)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt8(12n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 4 (12, 8)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 1 (2, 9)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt16(9n),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 2 (5, 7)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt16(7n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 3 (7, 7)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt16(7n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 4 (7, 5)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt16(5n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, euint16) => euint16 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint16) => euint16 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 1 (2, 6)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt16(6n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 2 (3, 4)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt16(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt16(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 4 (4, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt16(3n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 1 (12, 5602)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt16(5602n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 2 (8, 12)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(12n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 3 (12, 12)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt16(12n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 4 (12, 8)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 1 (3, 2214)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt16(2214n),
    );
    expect(res).to.equal(2215n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 1 (14, 55222)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt16(55222n),
    );
    expect(res).to.equal(55224n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 2 (10, 14)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt16(14n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 3 (14, 14)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt16(14n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 4 (14, 10)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt16(10n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 1 (4, 42082)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt16(42082n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 1 (14, 36271)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt16(36271n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt16(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt16(14n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt16(10n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 1 (6, 33197)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt16(33197n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 1 (3, 64214)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt16(64214n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 1 (8, 1689)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(1689n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 1 (8, 38833)', async function () {
    const res = await this.contract1.lt_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(38833n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint16(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 1 (7, 1092)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt16(1092n),
    );
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 1 (8, 7918)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(7918n),
    );
    expect(res).to.equal(7918n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(4n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 1 (2, 11)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt32(11n),
    );
    expect(res).to.equal(13n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 2 (5, 7)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt32(7n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 3 (7, 7)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt32(7n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 4 (7, 5)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt32(5n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, euint32) => euint32 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint32) => euint32 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 1 (2, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt32(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 2 (3, 4)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt32(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt32(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 4 (4, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt32(3n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 1 (14, 4070009968)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt32(4070009968n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 2 (10, 14)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt32(14n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 3 (14, 14)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt32(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 4 (14, 10)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt32(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 1 (9, 1970104792)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt32(1970104792n),
    );
    expect(res).to.equal(1970104793n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 2 (5, 9)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt32(9n),
    );
    expect(res).to.equal(13n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 3 (9, 9)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt32(9n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 4 (9, 5)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt32(5n),
    );
    expect(res).to.equal(13n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 1 (13, 2792893769)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(13n),
      this.instances1.alice.encrypt32(2792893769n),
    );
    expect(res).to.equal(2792893764n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 2 (9, 13)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt32(13n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 3 (13, 13)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(13n),
      this.instances1.alice.encrypt32(13n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 4 (13, 9)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(13n),
      this.instances1.alice.encrypt32(9n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 1 (14, 615427293)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt32(615427293n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt32(14n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt32(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt32(10n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 1 (12, 2043447934)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt32(2043447934n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(12n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt32(12n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 1 (1, 2311666703)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(1n),
      this.instances1.alice.encrypt32(2311666703n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 1 (4, 3962197569)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt32(3962197569n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 1 (8, 2524314152)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(2524314152n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 1 (9, 3957590987)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt32(3957590987n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt32(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt32(9n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt32(5n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 1 (14, 3097088189)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt32(3097088189n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 2 (10, 14)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt32(14n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 3 (14, 14)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt32(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 4 (14, 10)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt32(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 1 (2, 1707655364)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt32(1707655364n),
    );
    expect(res).to.equal(1707655364n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(4n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 1 (1, 9)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(1n),
      this.instances1.alice.encrypt64(9n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt64(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, euint64) => euint64 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint64) => euint64 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 1 (2, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt64(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 2 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt64(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt64(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 4 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt64(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 1 (14, 18445389920785975855)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt64(18445389920785975855n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 2 (10, 14)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt64(14n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 3 (14, 14)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt64(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 4 (14, 10)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt64(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 1 (1, 18440781624161714963)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(1n),
      this.instances1.alice.encrypt64(18440781624161714963n),
    );
    expect(res).to.equal(18440781624161714963n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 1 (11, 18443657406056982117)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt64(18443657406056982117n),
    );
    expect(res).to.equal(18443657406056982126n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 2 (7, 11)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt64(11n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 3 (11, 11)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt64(11n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 4 (11, 7)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt64(7n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 1 (13, 18440956134656481891)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(13n),
      this.instances1.alice.encrypt64(18440956134656481891n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt64(13n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(13n),
      this.instances1.alice.encrypt64(13n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(13n),
      this.instances1.alice.encrypt64(9n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 1 (6, 18440752300602820577)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt64(18440752300602820577n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 1 (2, 18441476332811684089)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt64(18441476332811684089n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 1 (2, 18440642707187411485)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt64(18440642707187411485n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 1 (5, 18443766393745666501)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt64(18443766393745666501n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 1 (5, 18441528681833028629)', async function () {
    const res = await this.contract1.lt_euint4_euint64(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt64(18441528681833028629n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint64(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 1 (12, 18441447130341341655)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt64(18441447130341341655n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 2 (8, 12)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(12n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 3 (12, 12)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt64(12n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 4 (12, 8)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 1 (1, 18441933496763425437)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(1n),
      this.instances1.alice.encrypt64(18441933496763425437n),
    );
    expect(res).to.equal(18441933496763425437n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(4n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 1 (6, 9)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(6n), 9n);
    expect(res).to.equal(15n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(4n), 8n);
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(5n), 5n);
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(8n), 4n);
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 1 (6, 5)', async function () {
    const res = await this.contract1.add_uint8_euint4(6n, this.instances1.alice.encrypt4(5n));
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_uint8_euint4(4n, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_uint8_euint4(5n, this.instances1.alice.encrypt4(5n));
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_uint8_euint4(8n, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, uint8) => euint4 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_uint8(this.instances1.alice.encrypt4(8n), 8n);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, uint8) => euint4 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_uint8(this.instances1.alice.encrypt4(8n), 4n);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint8, euint4) => euint4 test 1 (14, 14)', async function () {
    const res = await this.contract1.sub_uint8_euint4(14n, this.instances1.alice.encrypt4(14n));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint8, euint4) => euint4 test 2 (14, 10)', async function () {
    const res = await this.contract1.sub_uint8_euint4(14n, this.instances1.alice.encrypt4(10n));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 1 (3, 4)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(3n), 4n);
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(3n), 5n);
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(3n), 3n);
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(5n), 3n);
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 1 (7, 1)', async function () {
    const res = await this.contract1.mul_uint8_euint4(7n, this.instances1.alice.encrypt4(1n));
    expect(res).to.equal(7n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_uint8_euint4(3n, this.instances1.alice.encrypt4(5n));
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_uint8_euint4(3n, this.instances1.alice.encrypt4(3n));
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_uint8_euint4(5n, this.instances1.alice.encrypt4(3n));
    expect(res).to.equal(15n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 1 (5, 12)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(5n), 12n);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(4n), 8n);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(8n), 8n);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(8n), 4n);
    expect(res).to.equal(2n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 1 (8, 14)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(8n), 14n);
    expect(res).to.equal(8n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(4n), 8n);
    expect(res).to.equal(4n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(8n), 8n);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(8n), 4n);
    expect(res).to.equal(0n);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 1 (7, 11)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(7n), 11n);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(4n), 8n);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(8n), 8n);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(8n), 4n);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 1 (11, 1)', async function () {
    const res = await this.contract1.eq_uint8_euint4(11n, this.instances1.alice.encrypt4(1n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_uint8_euint4(4n, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_uint8_euint4(8n, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_uint8_euint4(8n, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 1 (11, 5)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(11n), 5n);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(7n), 11n);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(11n), 11n);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(11n), 7n);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 1 (10, 3)', async function () {
    const res = await this.contract1.ne_uint8_euint4(10n, this.instances1.alice.encrypt4(3n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ne_uint8_euint4(4n, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ne_uint8_euint4(8n, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ne_uint8_euint4(8n, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 1 (1, 1)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(1n), 1n);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(4n), 8n);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(8n), 8n);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(8n), 4n);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 1 (14, 1)', async function () {
    const res = await this.contract1.ge_uint8_euint4(14n, this.instances1.alice.encrypt4(1n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_uint8_euint4(4n, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_uint8_euint4(8n, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_uint8_euint4(8n, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 1 (2, 8)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(2n), 8n);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(4n), 8n);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(8n), 8n);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(8n), 4n);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 1 (6, 9)', async function () {
    const res = await this.contract1.gt_uint8_euint4(6n, this.instances1.alice.encrypt4(9n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.gt_uint8_euint4(5n, this.instances1.alice.encrypt4(9n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.gt_uint8_euint4(9n, this.instances1.alice.encrypt4(9n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.gt_uint8_euint4(9n, this.instances1.alice.encrypt4(5n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 1 (10, 3)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(10n), 3n);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(6n), 10n);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(10n), 10n);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(10n), 6n);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 1 (12, 8)', async function () {
    const res = await this.contract1.le_uint8_euint4(12n, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.le_uint8_euint4(4n, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.le_uint8_euint4(8n, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.le_uint8_euint4(8n, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 1 (3, 6)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(3n), 6n);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(4n), 8n);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(8n), 8n);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(8n), 4n);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 1 (13, 5)', async function () {
    const res = await this.contract1.lt_uint8_euint4(13n, this.instances1.alice.encrypt4(5n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_uint8_euint4(4n, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_uint8_euint4(8n, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_uint8_euint4(8n, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 1 (14, 14)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(14n), 14n);
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 2 (10, 14)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(10n), 14n);
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 3 (14, 14)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(14n), 14n);
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 4 (14, 10)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(14n), 10n);
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 1 (5, 14)', async function () {
    const res = await this.contract1.min_uint8_euint4(5n, this.instances1.alice.encrypt4(14n));
    expect(res).to.equal(5n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 2 (10, 14)', async function () {
    const res = await this.contract1.min_uint8_euint4(10n, this.instances1.alice.encrypt4(14n));
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 3 (14, 14)', async function () {
    const res = await this.contract1.min_uint8_euint4(14n, this.instances1.alice.encrypt4(14n));
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 4 (14, 10)', async function () {
    const res = await this.contract1.min_uint8_euint4(14n, this.instances1.alice.encrypt4(10n));
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 1 (12, 11)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(12n), 11n);
    expect(res).to.equal(12n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 2 (8, 12)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(8n), 12n);
    expect(res).to.equal(12n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 3 (12, 12)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(12n), 12n);
    expect(res).to.equal(12n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 4 (12, 8)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(12n), 8n);
    expect(res).to.equal(12n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 1 (8, 10)', async function () {
    const res = await this.contract1.max_uint8_euint4(8n, this.instances1.alice.encrypt4(10n));
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 2 (6, 10)', async function () {
    const res = await this.contract1.max_uint8_euint4(6n, this.instances1.alice.encrypt4(10n));
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 3 (10, 10)', async function () {
    const res = await this.contract1.max_uint8_euint4(10n, this.instances1.alice.encrypt4(10n));
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 4 (10, 6)', async function () {
    const res = await this.contract1.max_uint8_euint4(10n, this.instances1.alice.encrypt4(6n));
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 1 (9, 2)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(9n),
      this.instances1.alice.encrypt4(2n),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(4n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(5n),
      this.instances1.alice.encrypt4(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint8_euint4(
      this.instances1.alice.encrypt8(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint8, euint4) => euint8 test 1 (14, 14)', async function () {
    const res = await this.contract1.sub_euint8_euint4(
      this.instances1.alice.encrypt8(14n),
      this.instances1.alice.encrypt4(14n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint4) => euint8 test 2 (14, 10)', async function () {
    const res = await this.contract1.sub_euint8_euint4(
      this.instances1.alice.encrypt8(14n),
      this.instances1.alice.encrypt4(10n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 1 (2, 1)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(2n),
      this.instances1.alice.encrypt4(1n),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(3n),
      this.instances1.alice.encrypt4(5n),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(3n),
      this.instances1.alice.encrypt4(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(5n),
      this.instances1.alice.encrypt4(3n),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 1 (42, 14)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(42n),
      this.instances1.alice.encrypt4(14n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 2 (10, 14)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(10n),
      this.instances1.alice.encrypt4(14n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 3 (14, 14)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(14n),
      this.instances1.alice.encrypt4(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 4 (14, 10)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(14n),
      this.instances1.alice.encrypt4(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 1 (107, 10)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(107n),
      this.instances1.alice.encrypt4(10n),
    );
    expect(res).to.equal(107n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 2 (6, 10)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(6n),
      this.instances1.alice.encrypt4(10n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 3 (10, 10)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(10n),
      this.instances1.alice.encrypt4(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 4 (10, 6)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(10n),
      this.instances1.alice.encrypt4(6n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 1 (71, 13)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(71n),
      this.instances1.alice.encrypt4(13n),
    );
    expect(res).to.equal(74n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 2 (9, 13)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(9n),
      this.instances1.alice.encrypt4(13n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 3 (13, 13)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(13n),
      this.instances1.alice.encrypt4(13n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 4 (13, 9)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(13n),
      this.instances1.alice.encrypt4(9n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 1 (183, 1)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(183n),
      this.instances2.alice.encrypt4(1n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(4n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 1 (129, 3)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(129n),
      this.instances2.alice.encrypt4(3n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(4n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint4) => ebool test 1 (228, 1)', async function () {
    const res = await this.contract2.ge_euint8_euint4(
      this.instances2.alice.encrypt8(228n),
      this.instances2.alice.encrypt4(1n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ge_euint8_euint4(
      this.instances2.alice.encrypt8(4n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ge_euint8_euint4(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ge_euint8_euint4(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 1 (171, 9)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(171n),
      this.instances2.alice.encrypt4(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(5n),
      this.instances2.alice.encrypt4(9n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(9n),
      this.instances2.alice.encrypt4(9n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(9n),
      this.instances2.alice.encrypt4(5n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 1 (226, 8)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(226n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(4n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 1 (124, 5)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(124n),
      this.instances2.alice.encrypt4(5n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(4n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 1 (60, 14)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(60n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 2 (10, 14)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(10n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 3 (14, 14)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(14n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 4 (14, 10)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(14n),
      this.instances2.alice.encrypt4(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 1 (12, 10)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(12n),
      this.instances2.alice.encrypt4(10n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 2 (6, 10)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(6n),
      this.instances2.alice.encrypt4(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 3 (10, 10)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(10n),
      this.instances2.alice.encrypt4(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 4 (10, 6)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(10n),
      this.instances2.alice.encrypt4(6n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 1 (194, 52)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(194n),
      this.instances2.alice.encrypt8(52n),
    );
    expect(res).to.equal(246n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 2 (48, 52)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(48n),
      this.instances2.alice.encrypt8(52n),
    );
    expect(res).to.equal(100n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 3 (52, 52)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(52n),
      this.instances2.alice.encrypt8(52n),
    );
    expect(res).to.equal(104n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 4 (52, 48)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(52n),
      this.instances2.alice.encrypt8(48n),
    );
    expect(res).to.equal(100n);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 1 (65, 65)', async function () {
    const res = await this.contract2.sub_euint8_euint8(
      this.instances2.alice.encrypt8(65n),
      this.instances2.alice.encrypt8(65n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 2 (65, 61)', async function () {
    const res = await this.contract2.sub_euint8_euint8(
      this.instances2.alice.encrypt8(65n),
      this.instances2.alice.encrypt8(61n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 1 (32, 4)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(32n),
      this.instances2.alice.encrypt8(4n),
    );
    expect(res).to.equal(128n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 2 (14, 18)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(14n),
      this.instances2.alice.encrypt8(18n),
    );
    expect(res).to.equal(252n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 3 (10, 10)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(10n),
      this.instances2.alice.encrypt8(10n),
    );
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 4 (18, 14)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(18n),
      this.instances2.alice.encrypt8(14n),
    );
    expect(res).to.equal(252n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 1 (9, 119)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(9n),
      this.instances2.alice.encrypt8(119n),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 2 (5, 9)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(5n),
      this.instances2.alice.encrypt8(9n),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 3 (9, 9)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(9n),
      this.instances2.alice.encrypt8(9n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 4 (9, 5)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(9n),
      this.instances2.alice.encrypt8(5n),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 1 (89, 78)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(89n),
      this.instances2.alice.encrypt8(78n),
    );
    expect(res).to.equal(95n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 2 (74, 78)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(74n),
      this.instances2.alice.encrypt8(78n),
    );
    expect(res).to.equal(78n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 3 (78, 78)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(78n),
      this.instances2.alice.encrypt8(78n),
    );
    expect(res).to.equal(78n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 4 (78, 74)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(78n),
      this.instances2.alice.encrypt8(74n),
    );
    expect(res).to.equal(78n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 1 (20, 124)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(20n),
      this.instances2.alice.encrypt8(124n),
    );
    expect(res).to.equal(104n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 2 (16, 20)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(16n),
      this.instances2.alice.encrypt8(20n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 3 (20, 20)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(20n),
      this.instances2.alice.encrypt8(20n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 4 (20, 16)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(20n),
      this.instances2.alice.encrypt8(16n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 1 (21, 228)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(21n),
      this.instances2.alice.encrypt8(228n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 2 (17, 21)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(17n),
      this.instances2.alice.encrypt8(21n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 3 (21, 21)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(21n),
      this.instances2.alice.encrypt8(21n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 4 (21, 17)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(21n),
      this.instances2.alice.encrypt8(17n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 1 (164, 34)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(164n),
      this.instances2.alice.encrypt8(34n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 2 (30, 34)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(30n),
      this.instances2.alice.encrypt8(34n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 3 (34, 34)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(34n),
      this.instances2.alice.encrypt8(34n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 4 (34, 30)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(34n),
      this.instances2.alice.encrypt8(30n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 1 (163, 148)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(163n),
      this.instances2.alice.encrypt8(148n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 2 (144, 148)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(144n),
      this.instances2.alice.encrypt8(148n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 3 (148, 148)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(148n),
      this.instances2.alice.encrypt8(148n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 4 (148, 144)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(148n),
      this.instances2.alice.encrypt8(144n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 1 (77, 115)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(77n),
      this.instances2.alice.encrypt8(115n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 2 (73, 77)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(73n),
      this.instances2.alice.encrypt8(77n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 3 (77, 77)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(77n),
      this.instances2.alice.encrypt8(77n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 4 (77, 73)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(77n),
      this.instances2.alice.encrypt8(73n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 1 (149, 249)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(149n),
      this.instances2.alice.encrypt8(249n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 2 (145, 149)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(145n),
      this.instances2.alice.encrypt8(149n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 3 (149, 149)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(149n),
      this.instances2.alice.encrypt8(149n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 4 (149, 145)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(149n),
      this.instances2.alice.encrypt8(145n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 1 (247, 51)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(247n),
      this.instances2.alice.encrypt8(51n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 2 (47, 51)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(47n),
      this.instances2.alice.encrypt8(51n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 3 (51, 51)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(51n),
      this.instances2.alice.encrypt8(51n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 4 (51, 47)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(51n),
      this.instances2.alice.encrypt8(47n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 1 (85, 101)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(85n),
      this.instances2.alice.encrypt8(101n),
    );
    expect(res).to.equal(85n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 2 (81, 85)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(81n),
      this.instances2.alice.encrypt8(85n),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 3 (85, 85)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(85n),
      this.instances2.alice.encrypt8(85n),
    );
    expect(res).to.equal(85n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 4 (85, 81)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(85n),
      this.instances2.alice.encrypt8(81n),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 1 (67, 194)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(67n),
      this.instances2.alice.encrypt8(194n),
    );
    expect(res).to.equal(194n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 2 (63, 67)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(63n),
      this.instances2.alice.encrypt8(67n),
    );
    expect(res).to.equal(67n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 3 (67, 67)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(67n),
      this.instances2.alice.encrypt8(67n),
    );
    expect(res).to.equal(67n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 4 (67, 63)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(67n),
      this.instances2.alice.encrypt8(63n),
    );
    expect(res).to.equal(67n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 1 (3, 153)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(3n),
      this.instances2.alice.encrypt16(153n),
    );
    expect(res).to.equal(156n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 2 (76, 78)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(76n),
      this.instances2.alice.encrypt16(78n),
    );
    expect(res).to.equal(154n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 3 (78, 78)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(78n),
      this.instances2.alice.encrypt16(78n),
    );
    expect(res).to.equal(156n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 4 (78, 76)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(78n),
      this.instances2.alice.encrypt16(76n),
    );
    expect(res).to.equal(154n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 1 (126, 126)', async function () {
    const res = await this.contract2.sub_euint8_euint16(
      this.instances2.alice.encrypt8(126n),
      this.instances2.alice.encrypt16(126n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 2 (126, 122)', async function () {
    const res = await this.contract2.sub_euint8_euint16(
      this.instances2.alice.encrypt8(126n),
      this.instances2.alice.encrypt16(122n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 1 (2, 101)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(2n),
      this.instances2.alice.encrypt16(101n),
    );
    expect(res).to.equal(202n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 2 (11, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(11n),
      this.instances2.alice.encrypt16(11n),
    );
    expect(res).to.equal(121n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 3 (11, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(11n),
      this.instances2.alice.encrypt16(11n),
    );
    expect(res).to.equal(121n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 4 (11, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(11n),
      this.instances2.alice.encrypt16(11n),
    );
    expect(res).to.equal(121n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 1 (206, 3237)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(206n),
      this.instances2.alice.encrypt16(3237n),
    );
    expect(res).to.equal(132n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 2 (202, 206)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(202n),
      this.instances2.alice.encrypt16(206n),
    );
    expect(res).to.equal(202n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 3 (206, 206)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(206n),
      this.instances2.alice.encrypt16(206n),
    );
    expect(res).to.equal(206n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 4 (206, 202)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(206n),
      this.instances2.alice.encrypt16(202n),
    );
    expect(res).to.equal(202n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 1 (193, 5250)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(193n),
      this.instances2.alice.encrypt16(5250n),
    );
    expect(res).to.equal(5315n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 2 (189, 193)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(189n),
      this.instances2.alice.encrypt16(193n),
    );
    expect(res).to.equal(253n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 3 (193, 193)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(193n),
      this.instances2.alice.encrypt16(193n),
    );
    expect(res).to.equal(193n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 4 (193, 189)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(193n),
      this.instances2.alice.encrypt16(189n),
    );
    expect(res).to.equal(253n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 1 (75, 41617)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(75n),
      this.instances2.alice.encrypt16(41617n),
    );
    expect(res).to.equal(41690n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 2 (71, 75)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(71n),
      this.instances2.alice.encrypt16(75n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 3 (75, 75)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(75n),
      this.instances2.alice.encrypt16(75n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 4 (75, 71)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(75n),
      this.instances2.alice.encrypt16(71n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 1 (6, 23875)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(6n),
      this.instances2.alice.encrypt16(23875n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(4n),
      this.instances2.alice.encrypt16(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt16(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt16(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 1 (113, 51087)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(113n),
      this.instances2.alice.encrypt16(51087n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 2 (109, 113)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(109n),
      this.instances2.alice.encrypt16(113n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 3 (113, 113)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(113n),
      this.instances2.alice.encrypt16(113n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 4 (113, 109)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(113n),
      this.instances2.alice.encrypt16(109n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 1 (50, 56012)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(50n),
      this.instances2.alice.encrypt16(56012n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 2 (46, 50)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(46n),
      this.instances2.alice.encrypt16(50n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 3 (50, 50)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(50n),
      this.instances2.alice.encrypt16(50n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 4 (50, 46)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(50n),
      this.instances2.alice.encrypt16(46n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 1 (240, 10469)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(240n),
      this.instances2.alice.encrypt16(10469n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 2 (236, 240)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(236n),
      this.instances2.alice.encrypt16(240n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 3 (240, 240)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(240n),
      this.instances2.alice.encrypt16(240n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 4 (240, 236)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(240n),
      this.instances2.alice.encrypt16(236n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 1 (24, 35982)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(24n),
      this.instances2.alice.encrypt16(35982n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 2 (20, 24)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(20n),
      this.instances2.alice.encrypt16(24n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 3 (24, 24)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(24n),
      this.instances2.alice.encrypt16(24n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 4 (24, 20)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(24n),
      this.instances2.alice.encrypt16(20n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 1 (144, 10345)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(144n),
      this.instances2.alice.encrypt16(10345n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 2 (140, 144)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(140n),
      this.instances2.alice.encrypt16(144n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 3 (144, 144)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(144n),
      this.instances2.alice.encrypt16(144n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 4 (144, 140)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(144n),
      this.instances2.alice.encrypt16(140n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 1 (214, 34157)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(214n),
      this.instances2.alice.encrypt16(34157n),
    );
    expect(res).to.equal(214n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 2 (210, 214)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(210n),
      this.instances2.alice.encrypt16(214n),
    );
    expect(res).to.equal(210n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 3 (214, 214)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(214n),
      this.instances2.alice.encrypt16(214n),
    );
    expect(res).to.equal(214n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 4 (214, 210)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(214n),
      this.instances2.alice.encrypt16(210n),
    );
    expect(res).to.equal(210n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 1 (175, 2073)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(175n),
      this.instances2.alice.encrypt16(2073n),
    );
    expect(res).to.equal(2073n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 2 (171, 175)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(171n),
      this.instances2.alice.encrypt16(175n),
    );
    expect(res).to.equal(175n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 3 (175, 175)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(175n),
      this.instances2.alice.encrypt16(175n),
    );
    expect(res).to.equal(175n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 4 (175, 171)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(175n),
      this.instances2.alice.encrypt16(171n),
    );
    expect(res).to.equal(175n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 1 (2, 188)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(2n),
      this.instances2.alice.encrypt32(188n),
    );
    expect(res).to.equal(190n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 2 (118, 122)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(118n),
      this.instances2.alice.encrypt32(122n),
    );
    expect(res).to.equal(240n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 3 (122, 122)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(122n),
      this.instances2.alice.encrypt32(122n),
    );
    expect(res).to.equal(244n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 4 (122, 118)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(122n),
      this.instances2.alice.encrypt32(118n),
    );
    expect(res).to.equal(240n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 1 (78, 78)', async function () {
    const res = await this.contract2.sub_euint8_euint32(
      this.instances2.alice.encrypt8(78n),
      this.instances2.alice.encrypt32(78n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 2 (78, 74)', async function () {
    const res = await this.contract2.sub_euint8_euint32(
      this.instances2.alice.encrypt8(78n),
      this.instances2.alice.encrypt32(74n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 1 (2, 94)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(2n),
      this.instances2.alice.encrypt32(94n),
    );
    expect(res).to.equal(188n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 2 (9, 9)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(9n),
      this.instances2.alice.encrypt32(9n),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 3 (9, 9)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(9n),
      this.instances2.alice.encrypt32(9n),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 4 (9, 9)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(9n),
      this.instances2.alice.encrypt32(9n),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 1 (123, 418827486)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(123n),
      this.instances2.alice.encrypt32(418827486n),
    );
    expect(res).to.equal(90n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 2 (119, 123)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(119n),
      this.instances2.alice.encrypt32(123n),
    );
    expect(res).to.equal(115n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 3 (123, 123)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(123n),
      this.instances2.alice.encrypt32(123n),
    );
    expect(res).to.equal(123n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 4 (123, 119)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(123n),
      this.instances2.alice.encrypt32(119n),
    );
    expect(res).to.equal(115n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 1 (68, 3455465274)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(68n),
      this.instances2.alice.encrypt32(3455465274n),
    );
    expect(res).to.equal(3455465342n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 2 (64, 68)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(64n),
      this.instances2.alice.encrypt32(68n),
    );
    expect(res).to.equal(68n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 3 (68, 68)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(68n),
      this.instances2.alice.encrypt32(68n),
    );
    expect(res).to.equal(68n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 4 (68, 64)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(68n),
      this.instances2.alice.encrypt32(64n),
    );
    expect(res).to.equal(68n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 1 (109, 952561135)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(109n),
      this.instances2.alice.encrypt32(952561135n),
    );
    expect(res).to.equal(952561026n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 2 (105, 109)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(105n),
      this.instances2.alice.encrypt32(109n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 3 (109, 109)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(109n),
      this.instances2.alice.encrypt32(109n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 4 (109, 105)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(109n),
      this.instances2.alice.encrypt32(105n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 1 (211, 984363883)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(211n),
      this.instances2.alice.encrypt32(984363883n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 2 (207, 211)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(207n),
      this.instances2.alice.encrypt32(211n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 3 (211, 211)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(211n),
      this.instances2.alice.encrypt32(211n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 4 (211, 207)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(211n),
      this.instances2.alice.encrypt32(207n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 1 (29, 4141598810)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(29n),
      this.instances2.alice.encrypt32(4141598810n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 2 (25, 29)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(25n),
      this.instances2.alice.encrypt32(29n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 3 (29, 29)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(29n),
      this.instances2.alice.encrypt32(29n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 4 (29, 25)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(29n),
      this.instances2.alice.encrypt32(25n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 1 (40, 375178711)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(40n),
      this.instances2.alice.encrypt32(375178711n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 2 (36, 40)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(36n),
      this.instances2.alice.encrypt32(40n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 3 (40, 40)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(40n),
      this.instances2.alice.encrypt32(40n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 4 (40, 36)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(40n),
      this.instances2.alice.encrypt32(36n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 1 (250, 1468050188)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(250n),
      this.instances2.alice.encrypt32(1468050188n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 2 (246, 250)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(246n),
      this.instances2.alice.encrypt32(250n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 3 (250, 250)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(250n),
      this.instances2.alice.encrypt32(250n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 4 (250, 246)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(250n),
      this.instances2.alice.encrypt32(246n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 1 (247, 2062561284)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(247n),
      this.instances2.alice.encrypt32(2062561284n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 2 (243, 247)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(243n),
      this.instances2.alice.encrypt32(247n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 3 (247, 247)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(247n),
      this.instances2.alice.encrypt32(247n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 4 (247, 243)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(247n),
      this.instances2.alice.encrypt32(243n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 1 (242, 1342396181)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(242n),
      this.instances2.alice.encrypt32(1342396181n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 2 (238, 242)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(238n),
      this.instances2.alice.encrypt32(242n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 3 (242, 242)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(242n),
      this.instances2.alice.encrypt32(242n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 4 (242, 238)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(242n),
      this.instances2.alice.encrypt32(238n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 1 (209, 3879162726)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(209n),
      this.instances2.alice.encrypt32(3879162726n),
    );
    expect(res).to.equal(209n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 2 (205, 209)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(205n),
      this.instances2.alice.encrypt32(209n),
    );
    expect(res).to.equal(205n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 3 (209, 209)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(209n),
      this.instances2.alice.encrypt32(209n),
    );
    expect(res).to.equal(209n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 4 (209, 205)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(209n),
      this.instances2.alice.encrypt32(205n),
    );
    expect(res).to.equal(205n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 1 (137, 4020329182)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(137n),
      this.instances2.alice.encrypt32(4020329182n),
    );
    expect(res).to.equal(4020329182n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 2 (133, 137)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(133n),
      this.instances2.alice.encrypt32(137n),
    );
    expect(res).to.equal(137n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 3 (137, 137)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(137n),
      this.instances2.alice.encrypt32(137n),
    );
    expect(res).to.equal(137n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 4 (137, 133)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(137n),
      this.instances2.alice.encrypt32(133n),
    );
    expect(res).to.equal(137n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 1 (2, 129)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(2n),
      this.instances2.alice.encrypt64(129n),
    );
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 2 (115, 117)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(115n),
      this.instances2.alice.encrypt64(117n),
    );
    expect(res).to.equal(232n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 3 (117, 117)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(117n),
      this.instances2.alice.encrypt64(117n),
    );
    expect(res).to.equal(234n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 4 (117, 115)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(117n),
      this.instances2.alice.encrypt64(115n),
    );
    expect(res).to.equal(232n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 1 (112, 112)', async function () {
    const res = await this.contract2.sub_euint8_euint64(
      this.instances2.alice.encrypt8(112n),
      this.instances2.alice.encrypt64(112n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 2 (112, 108)', async function () {
    const res = await this.contract2.sub_euint8_euint64(
      this.instances2.alice.encrypt8(112n),
      this.instances2.alice.encrypt64(108n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 1 (2, 65)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(2n),
      this.instances2.alice.encrypt64(65n),
    );
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 2 (14, 16)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(14n),
      this.instances2.alice.encrypt64(16n),
    );
    expect(res).to.equal(224n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 3 (9, 9)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(9n),
      this.instances2.alice.encrypt64(9n),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 4 (16, 14)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(16n),
      this.instances2.alice.encrypt64(14n),
    );
    expect(res).to.equal(224n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 1 (57, 18441523438200809921)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(57n),
      this.instances2.alice.encrypt64(18441523438200809921n),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 2 (53, 57)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(53n),
      this.instances2.alice.encrypt64(57n),
    );
    expect(res).to.equal(49n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 3 (57, 57)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(57n),
      this.instances2.alice.encrypt64(57n),
    );
    expect(res).to.equal(57n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 4 (57, 53)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(57n),
      this.instances2.alice.encrypt64(53n),
    );
    expect(res).to.equal(49n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 1 (212, 18446560023502465713)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(212n),
      this.instances2.alice.encrypt64(18446560023502465713n),
    );
    expect(res).to.equal(18446560023502465781n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 2 (208, 212)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(208n),
      this.instances2.alice.encrypt64(212n),
    );
    expect(res).to.equal(212n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 3 (212, 212)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(212n),
      this.instances2.alice.encrypt64(212n),
    );
    expect(res).to.equal(212n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 4 (212, 208)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(212n),
      this.instances2.alice.encrypt64(208n),
    );
    expect(res).to.equal(212n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 1 (130, 18440006477954789963)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(130n),
      this.instances2.alice.encrypt64(18440006477954789963n),
    );
    expect(res).to.equal(18440006477954790089n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 2 (126, 130)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(126n),
      this.instances2.alice.encrypt64(130n),
    );
    expect(res).to.equal(252n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 3 (130, 130)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(130n),
      this.instances2.alice.encrypt64(130n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 4 (130, 126)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(130n),
      this.instances2.alice.encrypt64(126n),
    );
    expect(res).to.equal(252n);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 1 (195, 18439742818261570443)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(195n),
      this.instances2.alice.encrypt64(18439742818261570443n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 2 (191, 195)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(191n),
      this.instances2.alice.encrypt64(195n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 3 (195, 195)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(195n),
      this.instances2.alice.encrypt64(195n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 4 (195, 191)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(195n),
      this.instances2.alice.encrypt64(191n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 1 (135, 18440123185345504193)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(135n),
      this.instances2.alice.encrypt64(18440123185345504193n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 2 (131, 135)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(131n),
      this.instances2.alice.encrypt64(135n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 3 (135, 135)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(135n),
      this.instances2.alice.encrypt64(135n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 4 (135, 131)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(135n),
      this.instances2.alice.encrypt64(131n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 1 (93, 18440306937797900841)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(93n),
      this.instances2.alice.encrypt64(18440306937797900841n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 2 (89, 93)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(89n),
      this.instances2.alice.encrypt64(93n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 3 (93, 93)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(93n),
      this.instances2.alice.encrypt64(93n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 4 (93, 89)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(93n),
      this.instances2.alice.encrypt64(89n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 1 (52, 18439570337276424885)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(52n),
      this.instances2.alice.encrypt64(18439570337276424885n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 2 (48, 52)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(48n),
      this.instances2.alice.encrypt64(52n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 3 (52, 52)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(52n),
      this.instances2.alice.encrypt64(52n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 4 (52, 48)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(52n),
      this.instances2.alice.encrypt64(48n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 1 (227, 18443685234711580931)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(227n),
      this.instances2.alice.encrypt64(18443685234711580931n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 2 (223, 227)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(223n),
      this.instances2.alice.encrypt64(227n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 3 (227, 227)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(227n),
      this.instances2.alice.encrypt64(227n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 4 (227, 223)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(227n),
      this.instances2.alice.encrypt64(223n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 1 (173, 18439436156575213631)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(173n),
      this.instances2.alice.encrypt64(18439436156575213631n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 2 (169, 173)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(169n),
      this.instances2.alice.encrypt64(173n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 3 (173, 173)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(173n),
      this.instances2.alice.encrypt64(173n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 4 (173, 169)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(173n),
      this.instances2.alice.encrypt64(169n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 1 (166, 18444798929769499509)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(166n),
      this.instances2.alice.encrypt64(18444798929769499509n),
    );
    expect(res).to.equal(166n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 2 (162, 166)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(162n),
      this.instances2.alice.encrypt64(166n),
    );
    expect(res).to.equal(162n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 3 (166, 166)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(166n),
      this.instances2.alice.encrypt64(166n),
    );
    expect(res).to.equal(166n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 4 (166, 162)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(166n),
      this.instances2.alice.encrypt64(162n),
    );
    expect(res).to.equal(162n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 1 (192, 18438449977824161913)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(192n),
      this.instances2.alice.encrypt64(18438449977824161913n),
    );
    expect(res).to.equal(18438449977824161913n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 2 (188, 192)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(188n),
      this.instances2.alice.encrypt64(192n),
    );
    expect(res).to.equal(192n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 3 (192, 192)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(192n),
      this.instances2.alice.encrypt64(192n),
    );
    expect(res).to.equal(192n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 4 (192, 188)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(192n),
      this.instances2.alice.encrypt64(188n),
    );
    expect(res).to.equal(192n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 1 (98, 104)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(98n), 104n);
    expect(res).to.equal(202n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 2 (48, 52)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(48n), 52n);
    expect(res).to.equal(100n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 3 (52, 52)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(52n), 52n);
    expect(res).to.equal(104n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 4 (52, 48)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(52n), 48n);
    expect(res).to.equal(100n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 1 (62, 104)', async function () {
    const res = await this.contract2.add_uint8_euint8(62n, this.instances2.alice.encrypt8(104n));
    expect(res).to.equal(166n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 2 (48, 52)', async function () {
    const res = await this.contract2.add_uint8_euint8(48n, this.instances2.alice.encrypt8(52n));
    expect(res).to.equal(100n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 3 (52, 52)', async function () {
    const res = await this.contract2.add_uint8_euint8(52n, this.instances2.alice.encrypt8(52n));
    expect(res).to.equal(104n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 4 (52, 48)', async function () {
    const res = await this.contract2.add_uint8_euint8(52n, this.instances2.alice.encrypt8(48n));
    expect(res).to.equal(100n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 1 (65, 65)', async function () {
    const res = await this.contract2.sub_euint8_uint8(this.instances2.alice.encrypt8(65n), 65n);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 2 (65, 61)', async function () {
    const res = await this.contract2.sub_euint8_uint8(this.instances2.alice.encrypt8(65n), 61n);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 1 (65, 65)', async function () {
    const res = await this.contract2.sub_uint8_euint8(65n, this.instances2.alice.encrypt8(65n));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 2 (65, 61)', async function () {
    const res = await this.contract2.sub_uint8_euint8(65n, this.instances2.alice.encrypt8(61n));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 1 (17, 12)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(17n), 12n);
    expect(res).to.equal(204n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 2 (14, 18)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(14n), 18n);
    expect(res).to.equal(252n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 3 (10, 10)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(10n), 10n);
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 4 (18, 14)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(18n), 14n);
    expect(res).to.equal(252n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 1 (17, 12)', async function () {
    const res = await this.contract2.mul_uint8_euint8(17n, this.instances2.alice.encrypt8(12n));
    expect(res).to.equal(204n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 2 (14, 18)', async function () {
    const res = await this.contract2.mul_uint8_euint8(14n, this.instances2.alice.encrypt8(18n));
    expect(res).to.equal(252n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 3 (10, 10)', async function () {
    const res = await this.contract2.mul_uint8_euint8(10n, this.instances2.alice.encrypt8(10n));
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 4 (18, 14)', async function () {
    const res = await this.contract2.mul_uint8_euint8(18n, this.instances2.alice.encrypt8(14n));
    expect(res).to.equal(252n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 1 (9, 251)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(9n), 251n);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 2 (5, 9)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(5n), 9n);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 3 (9, 9)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(9n), 9n);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 4 (9, 5)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(9n), 5n);
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 1 (8, 10)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(8n), 10n);
    expect(res).to.equal(8n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(4n), 8n);
    expect(res).to.equal(4n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(8n), 8n);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(8n), 4n);
    expect(res).to.equal(0n);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 1 (21, 77)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(21n), 77n);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 2 (17, 21)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(17n), 21n);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 3 (21, 21)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(21n), 21n);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 4 (21, 17)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(21n), 17n);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 1 (199, 77)', async function () {
    const res = await this.contract2.eq_uint8_euint8(199n, this.instances2.alice.encrypt8(77n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 2 (17, 21)', async function () {
    const res = await this.contract2.eq_uint8_euint8(17n, this.instances2.alice.encrypt8(21n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 3 (21, 21)', async function () {
    const res = await this.contract2.eq_uint8_euint8(21n, this.instances2.alice.encrypt8(21n));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 4 (21, 17)', async function () {
    const res = await this.contract2.eq_uint8_euint8(21n, this.instances2.alice.encrypt8(17n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 1 (164, 216)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(164n), 216n);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 2 (30, 34)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(30n), 34n);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 3 (34, 34)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(34n), 34n);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 4 (34, 30)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(34n), 30n);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 1 (61, 216)', async function () {
    const res = await this.contract2.ne_uint8_euint8(61n, this.instances2.alice.encrypt8(216n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 2 (30, 34)', async function () {
    const res = await this.contract2.ne_uint8_euint8(30n, this.instances2.alice.encrypt8(34n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 3 (34, 34)', async function () {
    const res = await this.contract2.ne_uint8_euint8(34n, this.instances2.alice.encrypt8(34n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 4 (34, 30)', async function () {
    const res = await this.contract2.ne_uint8_euint8(34n, this.instances2.alice.encrypt8(30n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (163, 227)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(163n), 227n);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 2 (144, 148)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(144n), 148n);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 3 (148, 148)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(148n), 148n);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 4 (148, 144)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(148n), 144n);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 1 (175, 227)', async function () {
    const res = await this.contract2.ge_uint8_euint8(175n, this.instances2.alice.encrypt8(227n));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 2 (144, 148)', async function () {
    const res = await this.contract2.ge_uint8_euint8(144n, this.instances2.alice.encrypt8(148n));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 3 (148, 148)', async function () {
    const res = await this.contract2.ge_uint8_euint8(148n, this.instances2.alice.encrypt8(148n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 4 (148, 144)', async function () {
    const res = await this.contract2.ge_uint8_euint8(148n, this.instances2.alice.encrypt8(144n));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (77, 237)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(77n), 237n);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 2 (73, 77)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(73n), 77n);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 3 (77, 77)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(77n), 77n);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 4 (77, 73)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(77n), 73n);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 1 (137, 237)', async function () {
    const res = await this.contract2.gt_uint8_euint8(137n, this.instances2.alice.encrypt8(237n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 2 (73, 77)', async function () {
    const res = await this.contract2.gt_uint8_euint8(73n, this.instances2.alice.encrypt8(77n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 3 (77, 77)', async function () {
    const res = await this.contract2.gt_uint8_euint8(77n, this.instances2.alice.encrypt8(77n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 4 (77, 73)', async function () {
    const res = await this.contract2.gt_uint8_euint8(77n, this.instances2.alice.encrypt8(73n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (149, 13)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(149n), 13n);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (145, 149)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(145n), 149n);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (149, 149)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(149n), 149n);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 4 (149, 145)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(149n), 145n);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 1 (226, 13)', async function () {
    const res = await this.contract2.le_uint8_euint8(226n, this.instances2.alice.encrypt8(13n));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 2 (145, 149)', async function () {
    const res = await this.contract2.le_uint8_euint8(145n, this.instances2.alice.encrypt8(149n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 3 (149, 149)', async function () {
    const res = await this.contract2.le_uint8_euint8(149n, this.instances2.alice.encrypt8(149n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 4 (149, 145)', async function () {
    const res = await this.contract2.le_uint8_euint8(149n, this.instances2.alice.encrypt8(145n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 1 (247, 208)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(247n), 208n);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 2 (47, 51)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(47n), 51n);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 3 (51, 51)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(51n), 51n);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 4 (51, 47)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(51n), 47n);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (30, 208)', async function () {
    const res = await this.contract2.lt_uint8_euint8(30n, this.instances2.alice.encrypt8(208n));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 2 (47, 51)', async function () {
    const res = await this.contract2.lt_uint8_euint8(47n, this.instances2.alice.encrypt8(51n));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 3 (51, 51)', async function () {
    const res = await this.contract2.lt_uint8_euint8(51n, this.instances2.alice.encrypt8(51n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 4 (51, 47)', async function () {
    const res = await this.contract2.lt_uint8_euint8(51n, this.instances2.alice.encrypt8(47n));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (85, 106)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(85n), 106n);
    expect(res).to.equal(85n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (81, 85)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(81n), 85n);
    expect(res).to.equal(81n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (85, 85)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(85n), 85n);
    expect(res).to.equal(85n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 4 (85, 81)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(85n), 81n);
    expect(res).to.equal(81n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (233, 106)', async function () {
    const res = await this.contract2.min_uint8_euint8(233n, this.instances2.alice.encrypt8(106n));
    expect(res).to.equal(106n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 2 (81, 85)', async function () {
    const res = await this.contract2.min_uint8_euint8(81n, this.instances2.alice.encrypt8(85n));
    expect(res).to.equal(81n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 3 (85, 85)', async function () {
    const res = await this.contract2.min_uint8_euint8(85n, this.instances2.alice.encrypt8(85n));
    expect(res).to.equal(85n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 4 (85, 81)', async function () {
    const res = await this.contract2.min_uint8_euint8(85n, this.instances2.alice.encrypt8(81n));
    expect(res).to.equal(81n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (67, 11)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(67n), 11n);
    expect(res).to.equal(67n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (63, 67)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(63n), 67n);
    expect(res).to.equal(67n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (67, 67)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(67n), 67n);
    expect(res).to.equal(67n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 4 (67, 63)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(67n), 63n);
    expect(res).to.equal(67n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 1 (246, 11)', async function () {
    const res = await this.contract2.max_uint8_euint8(246n, this.instances2.alice.encrypt8(11n));
    expect(res).to.equal(246n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 2 (63, 67)', async function () {
    const res = await this.contract2.max_uint8_euint8(63n, this.instances2.alice.encrypt8(67n));
    expect(res).to.equal(67n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 3 (67, 67)', async function () {
    const res = await this.contract2.max_uint8_euint8(67n, this.instances2.alice.encrypt8(67n));
    expect(res).to.equal(67n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 4 (67, 63)', async function () {
    const res = await this.contract2.max_uint8_euint8(67n, this.instances2.alice.encrypt8(63n));
    expect(res).to.equal(67n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 1 (10, 2)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(10n),
      this.instances2.alice.encrypt4(2n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(4n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 3 (5, 5)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(5n),
      this.instances2.alice.encrypt4(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint16, euint4) => euint16 test 1 (8, 8)', async function () {
    const res = await this.contract2.sub_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint4) => euint16 test 2 (8, 4)', async function () {
    const res = await this.contract2.sub_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 1 (5, 2)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(5n),
      this.instances2.alice.encrypt4(2n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 2 (3, 4)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(3n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 3 (3, 3)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(3n),
      this.instances2.alice.encrypt4(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 4 (4, 3)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(4n),
      this.instances2.alice.encrypt4(3n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 1 (38968, 6)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(38968n),
      this.instances2.alice.encrypt4(6n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(4n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 1 (23856, 14)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(23856n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(23870n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 2 (10, 14)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(10n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 3 (14, 14)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(14n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 4 (14, 10)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(14n),
      this.instances2.alice.encrypt4(10n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 1 (62909, 9)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(62909n),
      this.instances2.alice.encrypt4(9n),
    );
    expect(res).to.equal(62900n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 2 (5, 9)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(5n),
      this.instances2.alice.encrypt4(9n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 3 (9, 9)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(9n),
      this.instances2.alice.encrypt4(9n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 4 (9, 5)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(9n),
      this.instances2.alice.encrypt4(5n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 1 (38016, 5)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(38016n),
      this.instances2.alice.encrypt4(5n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(4n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 1 (52490, 4)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(52490n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(4n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 1 (19532, 14)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(19532n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(10n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(14n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(14n),
      this.instances2.alice.encrypt4(10n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 1 (61155, 3)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(61155n),
      this.instances2.alice.encrypt4(3n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(4n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 1 (53692, 7)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(53692n),
      this.instances2.alice.encrypt4(7n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(4n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 1 (54836, 11)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(54836n),
      this.instances2.alice.encrypt4(11n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(7n),
      this.instances2.alice.encrypt4(11n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(11n),
      this.instances2.alice.encrypt4(11n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(11n),
      this.instances2.alice.encrypt4(7n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 1 (28521, 8)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(28521n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(4n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(8n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(8n),
      this.instances3.alice.encrypt4(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 1 (365, 8)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(365n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(365n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(4n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(8n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(8n),
      this.instances3.alice.encrypt4(4n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 1 (216, 2)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(216n),
      this.instances3.alice.encrypt8(2n),
    );
    expect(res).to.equal(218n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 2 (121, 125)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(121n),
      this.instances3.alice.encrypt8(125n),
    );
    expect(res).to.equal(246n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 3 (125, 125)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(125n),
      this.instances3.alice.encrypt8(125n),
    );
    expect(res).to.equal(250n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 4 (125, 121)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(125n),
      this.instances3.alice.encrypt8(121n),
    );
    expect(res).to.equal(246n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (127, 127)', async function () {
    const res = await this.contract3.sub_euint16_euint8(
      this.instances3.alice.encrypt16(127n),
      this.instances3.alice.encrypt8(127n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (127, 123)', async function () {
    const res = await this.contract3.sub_euint16_euint8(
      this.instances3.alice.encrypt16(127n),
      this.instances3.alice.encrypt8(123n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (85, 2)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(85n),
      this.instances3.alice.encrypt8(2n),
    );
    expect(res).to.equal(170n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 2 (15, 15)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(15n),
      this.instances3.alice.encrypt8(15n),
    );
    expect(res).to.equal(225n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 3 (15, 15)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(15n),
      this.instances3.alice.encrypt8(15n),
    );
    expect(res).to.equal(225n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 4 (15, 15)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(15n),
      this.instances3.alice.encrypt8(15n),
    );
    expect(res).to.equal(225n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 1 (63714, 17)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(63714n),
      this.instances3.alice.encrypt8(17n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 2 (13, 17)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(13n),
      this.instances3.alice.encrypt8(17n),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 3 (17, 17)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(17n),
      this.instances3.alice.encrypt8(17n),
    );
    expect(res).to.equal(17n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 4 (17, 13)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(17n),
      this.instances3.alice.encrypt8(13n),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 1 (30873, 188)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(30873n),
      this.instances3.alice.encrypt8(188n),
    );
    expect(res).to.equal(30909n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 2 (184, 188)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(184n),
      this.instances3.alice.encrypt8(188n),
    );
    expect(res).to.equal(188n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 3 (188, 188)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(188n),
      this.instances3.alice.encrypt8(188n),
    );
    expect(res).to.equal(188n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 4 (188, 184)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(188n),
      this.instances3.alice.encrypt8(184n),
    );
    expect(res).to.equal(188n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (6800, 30)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(6800n),
      this.instances3.alice.encrypt8(30n),
    );
    expect(res).to.equal(6798n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (26, 30)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(26n),
      this.instances3.alice.encrypt8(30n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 3 (30, 30)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(30n),
      this.instances3.alice.encrypt8(30n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 4 (30, 26)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(30n),
      this.instances3.alice.encrypt8(26n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 1 (32370, 81)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(32370n),
      this.instances3.alice.encrypt8(81n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 2 (77, 81)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(77n),
      this.instances3.alice.encrypt8(81n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 3 (81, 81)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(81n),
      this.instances3.alice.encrypt8(81n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 4 (81, 77)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(81n),
      this.instances3.alice.encrypt8(77n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (22182, 38)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(22182n),
      this.instances3.alice.encrypt8(38n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (34, 38)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(34n),
      this.instances3.alice.encrypt8(38n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 3 (38, 38)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(38n),
      this.instances3.alice.encrypt8(38n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 4 (38, 34)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(38n),
      this.instances3.alice.encrypt8(34n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (49741, 85)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(49741n),
      this.instances3.alice.encrypt8(85n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (81, 85)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(81n),
      this.instances3.alice.encrypt8(85n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (85, 85)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(85n),
      this.instances3.alice.encrypt8(85n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 4 (85, 81)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(85n),
      this.instances3.alice.encrypt8(81n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 1 (50975, 81)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(50975n),
      this.instances3.alice.encrypt8(81n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 2 (77, 81)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(77n),
      this.instances3.alice.encrypt8(81n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 3 (81, 81)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(81n),
      this.instances3.alice.encrypt8(81n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 4 (81, 77)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(81n),
      this.instances3.alice.encrypt8(77n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 1 (704, 121)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(704n),
      this.instances3.alice.encrypt8(121n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 2 (117, 121)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(117n),
      this.instances3.alice.encrypt8(121n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 3 (121, 121)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(121n),
      this.instances3.alice.encrypt8(121n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 4 (121, 117)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(121n),
      this.instances3.alice.encrypt8(117n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 1 (7923, 2)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(7923n),
      this.instances3.alice.encrypt8(2n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(4n),
      this.instances3.alice.encrypt8(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(8n),
      this.instances3.alice.encrypt8(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(8n),
      this.instances3.alice.encrypt8(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 1 (53149, 105)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(53149n),
      this.instances3.alice.encrypt8(105n),
    );
    expect(res).to.equal(105n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 2 (101, 105)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(101n),
      this.instances3.alice.encrypt8(105n),
    );
    expect(res).to.equal(101n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 3 (105, 105)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(105n),
      this.instances3.alice.encrypt8(105n),
    );
    expect(res).to.equal(105n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 4 (105, 101)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(105n),
      this.instances3.alice.encrypt8(101n),
    );
    expect(res).to.equal(101n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 1 (18799, 98)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(18799n),
      this.instances3.alice.encrypt8(98n),
    );
    expect(res).to.equal(18799n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 2 (94, 98)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(94n),
      this.instances3.alice.encrypt8(98n),
    );
    expect(res).to.equal(98n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 3 (98, 98)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(98n),
      this.instances3.alice.encrypt8(98n),
    );
    expect(res).to.equal(98n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 4 (98, 94)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(98n),
      this.instances3.alice.encrypt8(94n),
    );
    expect(res).to.equal(98n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 1 (11119, 48315)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(11119n),
      this.instances3.alice.encrypt16(48315n),
    );
    expect(res).to.equal(59434n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 2 (11115, 11119)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(11115n),
      this.instances3.alice.encrypt16(11119n),
    );
    expect(res).to.equal(22234n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 3 (11119, 11119)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(11119n),
      this.instances3.alice.encrypt16(11119n),
    );
    expect(res).to.equal(22238n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 4 (11119, 11115)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(11119n),
      this.instances3.alice.encrypt16(11115n),
    );
    expect(res).to.equal(22234n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 1 (8949, 8949)', async function () {
    const res = await this.contract3.sub_euint16_euint16(
      this.instances3.alice.encrypt16(8949n),
      this.instances3.alice.encrypt16(8949n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 2 (8949, 8945)', async function () {
    const res = await this.contract3.sub_euint16_euint16(
      this.instances3.alice.encrypt16(8949n),
      this.instances3.alice.encrypt16(8945n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 1 (126, 444)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(126n),
      this.instances3.alice.encrypt16(444n),
    );
    expect(res).to.equal(55944n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 2 (250, 250)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(250n),
      this.instances3.alice.encrypt16(250n),
    );
    expect(res).to.equal(62500n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 3 (250, 250)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(250n),
      this.instances3.alice.encrypt16(250n),
    );
    expect(res).to.equal(62500n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 4 (250, 250)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(250n),
      this.instances3.alice.encrypt16(250n),
    );
    expect(res).to.equal(62500n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 1 (9705, 867)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(9705n),
      this.instances3.alice.encrypt16(867n),
    );
    expect(res).to.equal(353n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 2 (863, 867)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(863n),
      this.instances3.alice.encrypt16(867n),
    );
    expect(res).to.equal(835n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 3 (867, 867)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(867n),
      this.instances3.alice.encrypt16(867n),
    );
    expect(res).to.equal(867n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 4 (867, 863)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(867n),
      this.instances3.alice.encrypt16(863n),
    );
    expect(res).to.equal(835n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 1 (45122, 17016)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(45122n),
      this.instances3.alice.encrypt16(17016n),
    );
    expect(res).to.equal(62074n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 2 (17012, 17016)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(17012n),
      this.instances3.alice.encrypt16(17016n),
    );
    expect(res).to.equal(17020n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 3 (17016, 17016)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(17016n),
      this.instances3.alice.encrypt16(17016n),
    );
    expect(res).to.equal(17016n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 4 (17016, 17012)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(17016n),
      this.instances3.alice.encrypt16(17012n),
    );
    expect(res).to.equal(17020n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (4910, 59435)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(4910n),
      this.instances3.alice.encrypt16(59435n),
    );
    expect(res).to.equal(64261n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (4906, 4910)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(4906n),
      this.instances3.alice.encrypt16(4910n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 3 (4910, 4910)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(4910n),
      this.instances3.alice.encrypt16(4910n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 4 (4910, 4906)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(4910n),
      this.instances3.alice.encrypt16(4906n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 1 (63163, 52416)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(63163n),
      this.instances3.alice.encrypt16(52416n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 2 (52412, 52416)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(52412n),
      this.instances3.alice.encrypt16(52416n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 3 (52416, 52416)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(52416n),
      this.instances3.alice.encrypt16(52416n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 4 (52416, 52412)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(52416n),
      this.instances3.alice.encrypt16(52412n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 1 (25480, 29585)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(25480n),
      this.instances3.alice.encrypt16(29585n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 2 (25476, 25480)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(25476n),
      this.instances3.alice.encrypt16(25480n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 3 (25480, 25480)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(25480n),
      this.instances3.alice.encrypt16(25480n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 4 (25480, 25476)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(25480n),
      this.instances3.alice.encrypt16(25476n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 1 (29422, 34903)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(29422n),
      this.instances3.alice.encrypt16(34903n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 2 (29418, 29422)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(29418n),
      this.instances3.alice.encrypt16(29422n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 3 (29422, 29422)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(29422n),
      this.instances3.alice.encrypt16(29422n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 4 (29422, 29418)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(29422n),
      this.instances3.alice.encrypt16(29418n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 1 (54686, 46157)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(54686n),
      this.instances3.alice.encrypt16(46157n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 2 (46153, 46157)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(46153n),
      this.instances3.alice.encrypt16(46157n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 3 (46157, 46157)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(46157n),
      this.instances3.alice.encrypt16(46157n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 4 (46157, 46153)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(46157n),
      this.instances3.alice.encrypt16(46153n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 1 (27418, 31738)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(27418n),
      this.instances3.alice.encrypt16(31738n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 2 (27414, 27418)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(27414n),
      this.instances3.alice.encrypt16(27418n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 3 (27418, 27418)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(27418n),
      this.instances3.alice.encrypt16(27418n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 4 (27418, 27414)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(27418n),
      this.instances3.alice.encrypt16(27414n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 1 (42423, 21335)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(42423n),
      this.instances3.alice.encrypt16(21335n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 2 (21331, 21335)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(21331n),
      this.instances3.alice.encrypt16(21335n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 3 (21335, 21335)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(21335n),
      this.instances3.alice.encrypt16(21335n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 4 (21335, 21331)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(21335n),
      this.instances3.alice.encrypt16(21331n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 1 (16764, 28435)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(16764n),
      this.instances3.alice.encrypt16(28435n),
    );
    expect(res).to.equal(16764n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 2 (16760, 16764)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(16760n),
      this.instances3.alice.encrypt16(16764n),
    );
    expect(res).to.equal(16760n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 3 (16764, 16764)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(16764n),
      this.instances3.alice.encrypt16(16764n),
    );
    expect(res).to.equal(16764n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 4 (16764, 16760)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(16764n),
      this.instances3.alice.encrypt16(16760n),
    );
    expect(res).to.equal(16760n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 1 (34467, 11766)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(34467n),
      this.instances3.alice.encrypt16(11766n),
    );
    expect(res).to.equal(34467n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 2 (11762, 11766)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(11762n),
      this.instances3.alice.encrypt16(11766n),
    );
    expect(res).to.equal(11766n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 3 (11766, 11766)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(11766n),
      this.instances3.alice.encrypt16(11766n),
    );
    expect(res).to.equal(11766n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 4 (11766, 11762)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(11766n),
      this.instances3.alice.encrypt16(11762n),
    );
    expect(res).to.equal(11766n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 1 (2, 50704)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(2n),
      this.instances3.alice.encrypt32(50704n),
    );
    expect(res).to.equal(50706n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 2 (15242, 15246)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(15242n),
      this.instances3.alice.encrypt32(15246n),
    );
    expect(res).to.equal(30488n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 3 (15246, 15246)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(15246n),
      this.instances3.alice.encrypt32(15246n),
    );
    expect(res).to.equal(30492n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 4 (15246, 15242)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(15246n),
      this.instances3.alice.encrypt32(15242n),
    );
    expect(res).to.equal(30488n);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (37742, 37742)', async function () {
    const res = await this.contract3.sub_euint16_euint32(
      this.instances3.alice.encrypt16(37742n),
      this.instances3.alice.encrypt32(37742n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (37742, 37738)', async function () {
    const res = await this.contract3.sub_euint16_euint32(
      this.instances3.alice.encrypt16(37742n),
      this.instances3.alice.encrypt32(37738n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (2, 27763)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(2n),
      this.instances3.alice.encrypt32(27763n),
    );
    expect(res).to.equal(55526n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 2 (179, 179)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(179n),
      this.instances3.alice.encrypt32(179n),
    );
    expect(res).to.equal(32041n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 3 (179, 179)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(179n),
      this.instances3.alice.encrypt32(179n),
    );
    expect(res).to.equal(32041n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 4 (179, 179)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(179n),
      this.instances3.alice.encrypt32(179n),
    );
    expect(res).to.equal(32041n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (36913, 4249867996)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(36913n),
      this.instances3.alice.encrypt32(4249867996n),
    );
    expect(res).to.equal(36880n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (36909, 36913)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(36909n),
      this.instances3.alice.encrypt32(36913n),
    );
    expect(res).to.equal(36897n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (36913, 36913)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(36913n),
      this.instances3.alice.encrypt32(36913n),
    );
    expect(res).to.equal(36913n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (36913, 36909)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(36913n),
      this.instances3.alice.encrypt32(36909n),
    );
    expect(res).to.equal(36897n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (4456, 1748158582)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(4456n),
      this.instances3.alice.encrypt32(1748158582n),
    );
    expect(res).to.equal(1748162942n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (4452, 4456)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(4452n),
      this.instances3.alice.encrypt32(4456n),
    );
    expect(res).to.equal(4460n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 3 (4456, 4456)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(4456n),
      this.instances3.alice.encrypt32(4456n),
    );
    expect(res).to.equal(4456n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 4 (4456, 4452)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(4456n),
      this.instances3.alice.encrypt32(4452n),
    );
    expect(res).to.equal(4460n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 1 (63959, 233439801)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(63959n),
      this.instances3.alice.encrypt32(233439801n),
    );
    expect(res).to.equal(233503726n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 2 (63955, 63959)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(63955n),
      this.instances3.alice.encrypt32(63959n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 3 (63959, 63959)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(63959n),
      this.instances3.alice.encrypt32(63959n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 4 (63959, 63955)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(63959n),
      this.instances3.alice.encrypt32(63955n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (59226, 1338987968)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(59226n),
      this.instances3.alice.encrypt32(1338987968n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (59222, 59226)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(59222n),
      this.instances3.alice.encrypt32(59226n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 3 (59226, 59226)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(59226n),
      this.instances3.alice.encrypt32(59226n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 4 (59226, 59222)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(59226n),
      this.instances3.alice.encrypt32(59222n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 1 (46360, 20214536)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(46360n),
      this.instances3.alice.encrypt32(20214536n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 2 (46356, 46360)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(46356n),
      this.instances3.alice.encrypt32(46360n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 3 (46360, 46360)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(46360n),
      this.instances3.alice.encrypt32(46360n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 4 (46360, 46356)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(46360n),
      this.instances3.alice.encrypt32(46356n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (18479, 3153529612)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(18479n),
      this.instances3.alice.encrypt32(3153529612n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (18475, 18479)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(18475n),
      this.instances3.alice.encrypt32(18479n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (18479, 18479)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(18479n),
      this.instances3.alice.encrypt32(18479n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 4 (18479, 18475)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(18479n),
      this.instances3.alice.encrypt32(18475n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 1 (1724, 3845491325)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(1724n),
      this.instances3.alice.encrypt32(3845491325n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 2 (1720, 1724)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(1720n),
      this.instances3.alice.encrypt32(1724n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 3 (1724, 1724)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(1724n),
      this.instances3.alice.encrypt32(1724n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 4 (1724, 1720)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(1724n),
      this.instances3.alice.encrypt32(1720n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (64249, 2852093696)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(64249n),
      this.instances3.alice.encrypt32(2852093696n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (64245, 64249)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(64245n),
      this.instances3.alice.encrypt32(64249n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (64249, 64249)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(64249n),
      this.instances3.alice.encrypt32(64249n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 4 (64249, 64245)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(64249n),
      this.instances3.alice.encrypt32(64245n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 1 (64831, 1889205542)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(64831n),
      this.instances3.alice.encrypt32(1889205542n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 2 (64827, 64831)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(64827n),
      this.instances3.alice.encrypt32(64831n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 3 (64831, 64831)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(64831n),
      this.instances3.alice.encrypt32(64831n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 4 (64831, 64827)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(64831n),
      this.instances3.alice.encrypt32(64827n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 1 (10503, 4039671425)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(10503n),
      this.instances3.alice.encrypt32(4039671425n),
    );
    expect(res).to.equal(10503n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 2 (10499, 10503)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(10499n),
      this.instances3.alice.encrypt32(10503n),
    );
    expect(res).to.equal(10499n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 3 (10503, 10503)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(10503n),
      this.instances3.alice.encrypt32(10503n),
    );
    expect(res).to.equal(10503n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 4 (10503, 10499)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(10503n),
      this.instances3.alice.encrypt32(10499n),
    );
    expect(res).to.equal(10499n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (6124, 649522722)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(6124n),
      this.instances3.alice.encrypt32(649522722n),
    );
    expect(res).to.equal(649522722n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (6120, 6124)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(6120n),
      this.instances3.alice.encrypt32(6124n),
    );
    expect(res).to.equal(6124n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (6124, 6124)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(6124n),
      this.instances3.alice.encrypt32(6124n),
    );
    expect(res).to.equal(6124n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 4 (6124, 6120)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(6124n),
      this.instances3.alice.encrypt32(6120n),
    );
    expect(res).to.equal(6124n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 1 (2, 65518)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(2n),
      this.instances3.alice.encrypt64(65518n),
    );
    expect(res).to.equal(65520n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 2 (5709, 5713)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(5709n),
      this.instances3.alice.encrypt64(5713n),
    );
    expect(res).to.equal(11422n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 3 (5713, 5713)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(5713n),
      this.instances3.alice.encrypt64(5713n),
    );
    expect(res).to.equal(11426n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 4 (5713, 5709)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(5713n),
      this.instances3.alice.encrypt64(5709n),
    );
    expect(res).to.equal(11422n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 1 (19465, 19465)', async function () {
    const res = await this.contract3.sub_euint16_euint64(
      this.instances3.alice.encrypt16(19465n),
      this.instances3.alice.encrypt64(19465n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 2 (19465, 19461)', async function () {
    const res = await this.contract3.sub_euint16_euint64(
      this.instances3.alice.encrypt16(19465n),
      this.instances3.alice.encrypt64(19461n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 1 (2, 32762)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(2n),
      this.instances3.alice.encrypt64(32762n),
    );
    expect(res).to.equal(65524n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 2 (192, 192)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(192n),
      this.instances3.alice.encrypt64(192n),
    );
    expect(res).to.equal(36864n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 3 (192, 192)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(192n),
      this.instances3.alice.encrypt64(192n),
    );
    expect(res).to.equal(36864n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 4 (192, 192)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(192n),
      this.instances3.alice.encrypt64(192n),
    );
    expect(res).to.equal(36864n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 1 (37650, 18445096100929808349)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(37650n),
      this.instances3.alice.encrypt64(18445096100929808349n),
    );
    expect(res).to.equal(784n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 2 (37646, 37650)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(37646n),
      this.instances3.alice.encrypt64(37650n),
    );
    expect(res).to.equal(37634n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 3 (37650, 37650)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(37650n),
      this.instances3.alice.encrypt64(37650n),
    );
    expect(res).to.equal(37650n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 4 (37650, 37646)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(37650n),
      this.instances3.alice.encrypt64(37646n),
    );
    expect(res).to.equal(37634n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 1 (40790, 18439654160476847847)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(40790n),
      this.instances3.alice.encrypt64(18439654160476847847n),
    );
    expect(res).to.equal(18439654160476848119n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 2 (40786, 40790)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(40786n),
      this.instances3.alice.encrypt64(40790n),
    );
    expect(res).to.equal(40790n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 3 (40790, 40790)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(40790n),
      this.instances3.alice.encrypt64(40790n),
    );
    expect(res).to.equal(40790n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 4 (40790, 40786)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(40790n),
      this.instances3.alice.encrypt64(40786n),
    );
    expect(res).to.equal(40790n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 1 (40629, 18444797253466788605)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(40629n),
      this.instances3.alice.encrypt64(18444797253466788605n),
    );
    expect(res).to.equal(18444797253466819656n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 2 (40625, 40629)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(40625n),
      this.instances3.alice.encrypt64(40629n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 3 (40629, 40629)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(40629n),
      this.instances3.alice.encrypt64(40629n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 4 (40629, 40625)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(40629n),
      this.instances3.alice.encrypt64(40625n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 1 (48894, 18445756232384346451)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(48894n),
      this.instances3.alice.encrypt64(18445756232384346451n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 2 (48890, 48894)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(48890n),
      this.instances3.alice.encrypt64(48894n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 3 (48894, 48894)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(48894n),
      this.instances3.alice.encrypt64(48894n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 4 (48894, 48890)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(48894n),
      this.instances3.alice.encrypt64(48890n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 1 (10282, 18444621055179699415)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(10282n),
      this.instances3.alice.encrypt64(18444621055179699415n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 2 (10278, 10282)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(10278n),
      this.instances3.alice.encrypt64(10282n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 3 (10282, 10282)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(10282n),
      this.instances3.alice.encrypt64(10282n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 4 (10282, 10278)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(10282n),
      this.instances3.alice.encrypt64(10278n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 1 (10286, 18442616435224963159)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(10286n),
      this.instances3.alice.encrypt64(18442616435224963159n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 2 (10282, 10286)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(10282n),
      this.instances3.alice.encrypt64(10286n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 3 (10286, 10286)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(10286n),
      this.instances3.alice.encrypt64(10286n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 4 (10286, 10282)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(10286n),
      this.instances3.alice.encrypt64(10282n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 1 (54449, 18438546423338342951)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(54449n),
      this.instances3.alice.encrypt64(18438546423338342951n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 2 (54445, 54449)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(54445n),
      this.instances3.alice.encrypt64(54449n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 3 (54449, 54449)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(54449n),
      this.instances3.alice.encrypt64(54449n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 4 (54449, 54445)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(54449n),
      this.instances3.alice.encrypt64(54445n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 1 (58853, 18440531451848815977)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(58853n),
      this.instances3.alice.encrypt64(18440531451848815977n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 2 (58849, 58853)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(58849n),
      this.instances3.alice.encrypt64(58853n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 3 (58853, 58853)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(58853n),
      this.instances3.alice.encrypt64(58853n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 4 (58853, 58849)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(58853n),
      this.instances3.alice.encrypt64(58849n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 1 (18476, 18443022011750886647)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(18476n),
      this.instances3.alice.encrypt64(18443022011750886647n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 2 (18472, 18476)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(18472n),
      this.instances3.alice.encrypt64(18476n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 3 (18476, 18476)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(18476n),
      this.instances3.alice.encrypt64(18476n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 4 (18476, 18472)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(18476n),
      this.instances3.alice.encrypt64(18472n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 1 (12487, 18437967600930526653)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(12487n),
      this.instances3.alice.encrypt64(18437967600930526653n),
    );
    expect(res).to.equal(12487n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 2 (12483, 12487)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(12483n),
      this.instances3.alice.encrypt64(12487n),
    );
    expect(res).to.equal(12483n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 3 (12487, 12487)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(12487n),
      this.instances3.alice.encrypt64(12487n),
    );
    expect(res).to.equal(12487n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 4 (12487, 12483)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(12487n),
      this.instances3.alice.encrypt64(12483n),
    );
    expect(res).to.equal(12483n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 1 (18320, 18439734967706381877)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(18320n),
      this.instances3.alice.encrypt64(18439734967706381877n),
    );
    expect(res).to.equal(18439734967706381877n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 2 (18316, 18320)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(18316n),
      this.instances3.alice.encrypt64(18320n),
    );
    expect(res).to.equal(18320n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 3 (18320, 18320)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(18320n),
      this.instances3.alice.encrypt64(18320n),
    );
    expect(res).to.equal(18320n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 4 (18320, 18316)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(18320n),
      this.instances3.alice.encrypt64(18316n),
    );
    expect(res).to.equal(18320n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (11119, 41026)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(11119n), 41026n);
    expect(res).to.equal(52145n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (11115, 11119)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(11115n), 11119n);
    expect(res).to.equal(22234n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (11119, 11119)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(11119n), 11119n);
    expect(res).to.equal(22238n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (11119, 11115)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(11119n), 11115n);
    expect(res).to.equal(22234n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (29634, 20514)', async function () {
    const res = await this.contract3.add_uint16_euint16(29634n, this.instances3.alice.encrypt16(20514n));
    expect(res).to.equal(50148n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 2 (11115, 11119)', async function () {
    const res = await this.contract3.add_uint16_euint16(11115n, this.instances3.alice.encrypt16(11119n));
    expect(res).to.equal(22234n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 3 (11119, 11119)', async function () {
    const res = await this.contract3.add_uint16_euint16(11119n, this.instances3.alice.encrypt16(11119n));
    expect(res).to.equal(22238n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 4 (11119, 11115)', async function () {
    const res = await this.contract3.add_uint16_euint16(11119n, this.instances3.alice.encrypt16(11115n));
    expect(res).to.equal(22234n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (8949, 8949)', async function () {
    const res = await this.contract3.sub_euint16_uint16(this.instances3.alice.encrypt16(8949n), 8949n);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 2 (8949, 8945)', async function () {
    const res = await this.contract3.sub_euint16_uint16(this.instances3.alice.encrypt16(8949n), 8945n);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (8949, 8949)', async function () {
    const res = await this.contract3.sub_uint16_euint16(8949n, this.instances3.alice.encrypt16(8949n));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (8949, 8945)', async function () {
    const res = await this.contract3.sub_uint16_euint16(8949n, this.instances3.alice.encrypt16(8945n));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (64, 440)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(64n), 440n);
    expect(res).to.equal(28160n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (250, 250)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(250n), 250n);
    expect(res).to.equal(62500n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (250, 250)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(250n), 250n);
    expect(res).to.equal(62500n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (250, 250)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(250n), 250n);
    expect(res).to.equal(62500n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (124, 221)', async function () {
    const res = await this.contract3.mul_uint16_euint16(124n, this.instances3.alice.encrypt16(221n));
    expect(res).to.equal(27404n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 2 (250, 250)', async function () {
    const res = await this.contract3.mul_uint16_euint16(250n, this.instances3.alice.encrypt16(250n));
    expect(res).to.equal(62500n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 3 (250, 250)', async function () {
    const res = await this.contract3.mul_uint16_euint16(250n, this.instances3.alice.encrypt16(250n));
    expect(res).to.equal(62500n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 4 (250, 250)', async function () {
    const res = await this.contract3.mul_uint16_euint16(250n, this.instances3.alice.encrypt16(250n));
    expect(res).to.equal(62500n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (5178, 48082)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(5178n), 48082n);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (5174, 5178)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(5174n), 5178n);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (5178, 5178)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(5178n), 5178n);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (5178, 5174)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(5178n), 5174n);
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (34639, 53051)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(34639n), 53051n);
    expect(res).to.equal(34639n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 2 (34635, 34639)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(34635n), 34639n);
    expect(res).to.equal(34635n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 3 (34639, 34639)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(34639n), 34639n);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 4 (34639, 34635)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(34639n), 34635n);
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (63163, 55058)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(63163n), 55058n);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (52412, 52416)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(52412n), 52416n);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (52416, 52416)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(52416n), 52416n);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (52416, 52412)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(52416n), 52412n);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (18577, 55058)', async function () {
    const res = await this.contract3.eq_uint16_euint16(18577n, this.instances3.alice.encrypt16(55058n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (52412, 52416)', async function () {
    const res = await this.contract3.eq_uint16_euint16(52412n, this.instances3.alice.encrypt16(52416n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 3 (52416, 52416)', async function () {
    const res = await this.contract3.eq_uint16_euint16(52416n, this.instances3.alice.encrypt16(52416n));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 4 (52416, 52412)', async function () {
    const res = await this.contract3.eq_uint16_euint16(52416n, this.instances3.alice.encrypt16(52412n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (25480, 4458)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(25480n), 4458n);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (25476, 25480)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(25476n), 25480n);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 3 (25480, 25480)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(25480n), 25480n);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 4 (25480, 25476)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(25480n), 25476n);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (14335, 4458)', async function () {
    const res = await this.contract3.ne_uint16_euint16(14335n, this.instances3.alice.encrypt16(4458n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (25476, 25480)', async function () {
    const res = await this.contract3.ne_uint16_euint16(25476n, this.instances3.alice.encrypt16(25480n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 3 (25480, 25480)', async function () {
    const res = await this.contract3.ne_uint16_euint16(25480n, this.instances3.alice.encrypt16(25480n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 4 (25480, 25476)', async function () {
    const res = await this.contract3.ne_uint16_euint16(25480n, this.instances3.alice.encrypt16(25476n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (29422, 60978)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(29422n), 60978n);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (29418, 29422)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(29418n), 29422n);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (29422, 29422)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(29422n), 29422n);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 4 (29422, 29418)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(29422n), 29418n);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (30097, 60978)', async function () {
    const res = await this.contract3.ge_uint16_euint16(30097n, this.instances3.alice.encrypt16(60978n));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (29418, 29422)', async function () {
    const res = await this.contract3.ge_uint16_euint16(29418n, this.instances3.alice.encrypt16(29422n));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (29422, 29422)', async function () {
    const res = await this.contract3.ge_uint16_euint16(29422n, this.instances3.alice.encrypt16(29422n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 4 (29422, 29418)', async function () {
    const res = await this.contract3.ge_uint16_euint16(29422n, this.instances3.alice.encrypt16(29418n));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (54686, 12705)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(54686n), 12705n);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (46153, 46157)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(46153n), 46157n);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (46157, 46157)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(46157n), 46157n);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 4 (46157, 46153)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(46157n), 46153n);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (56674, 12705)', async function () {
    const res = await this.contract3.gt_uint16_euint16(56674n, this.instances3.alice.encrypt16(12705n));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (46153, 46157)', async function () {
    const res = await this.contract3.gt_uint16_euint16(46153n, this.instances3.alice.encrypt16(46157n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (46157, 46157)', async function () {
    const res = await this.contract3.gt_uint16_euint16(46157n, this.instances3.alice.encrypt16(46157n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 4 (46157, 46153)', async function () {
    const res = await this.contract3.gt_uint16_euint16(46157n, this.instances3.alice.encrypt16(46153n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (27418, 34933)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(27418n), 34933n);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (27414, 27418)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(27414n), 27418n);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (27418, 27418)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(27418n), 27418n);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 4 (27418, 27414)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(27418n), 27414n);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (11511, 34933)', async function () {
    const res = await this.contract3.le_uint16_euint16(11511n, this.instances3.alice.encrypt16(34933n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (27414, 27418)', async function () {
    const res = await this.contract3.le_uint16_euint16(27414n, this.instances3.alice.encrypt16(27418n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (27418, 27418)', async function () {
    const res = await this.contract3.le_uint16_euint16(27418n, this.instances3.alice.encrypt16(27418n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (27418, 27414)', async function () {
    const res = await this.contract3.le_uint16_euint16(27418n, this.instances3.alice.encrypt16(27414n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (42423, 4870)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(42423n), 4870n);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (21331, 21335)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(21331n), 21335n);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (21335, 21335)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(21335n), 21335n);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 4 (21335, 21331)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(21335n), 21331n);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (8043, 4870)', async function () {
    const res = await this.contract3.lt_uint16_euint16(8043n, this.instances3.alice.encrypt16(4870n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (21331, 21335)', async function () {
    const res = await this.contract3.lt_uint16_euint16(21331n, this.instances3.alice.encrypt16(21335n));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (21335, 21335)', async function () {
    const res = await this.contract3.lt_uint16_euint16(21335n, this.instances3.alice.encrypt16(21335n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 4 (21335, 21331)', async function () {
    const res = await this.contract3.lt_uint16_euint16(21335n, this.instances3.alice.encrypt16(21331n));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (16764, 32518)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(16764n), 32518n);
    expect(res).to.equal(16764n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (16760, 16764)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(16760n), 16764n);
    expect(res).to.equal(16760n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (16764, 16764)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(16764n), 16764n);
    expect(res).to.equal(16764n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 4 (16764, 16760)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(16764n), 16760n);
    expect(res).to.equal(16760n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (39233, 32518)', async function () {
    const res = await this.contract3.min_uint16_euint16(39233n, this.instances3.alice.encrypt16(32518n));
    expect(res).to.equal(32518n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (16760, 16764)', async function () {
    const res = await this.contract3.min_uint16_euint16(16760n, this.instances3.alice.encrypt16(16764n));
    expect(res).to.equal(16760n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (16764, 16764)', async function () {
    const res = await this.contract3.min_uint16_euint16(16764n, this.instances3.alice.encrypt16(16764n));
    expect(res).to.equal(16764n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (16764, 16760)', async function () {
    const res = await this.contract3.min_uint16_euint16(16764n, this.instances3.alice.encrypt16(16760n));
    expect(res).to.equal(16760n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (34467, 41318)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(34467n), 41318n);
    expect(res).to.equal(41318n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (11762, 11766)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(11762n), 11766n);
    expect(res).to.equal(11766n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (11766, 11766)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(11766n), 11766n);
    expect(res).to.equal(11766n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (11766, 11762)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(11766n), 11762n);
    expect(res).to.equal(11766n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (19449, 41318)', async function () {
    const res = await this.contract3.max_uint16_euint16(19449n, this.instances3.alice.encrypt16(41318n));
    expect(res).to.equal(41318n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (11762, 11766)', async function () {
    const res = await this.contract3.max_uint16_euint16(11762n, this.instances3.alice.encrypt16(11766n));
    expect(res).to.equal(11766n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (11766, 11766)', async function () {
    const res = await this.contract3.max_uint16_euint16(11766n, this.instances3.alice.encrypt16(11766n));
    expect(res).to.equal(11766n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (11766, 11762)', async function () {
    const res = await this.contract3.max_uint16_euint16(11766n, this.instances3.alice.encrypt16(11762n));
    expect(res).to.equal(11766n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 1 (12, 2)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(12n),
      this.instances3.alice.encrypt4(2n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 2 (5, 7)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(5n),
      this.instances3.alice.encrypt4(7n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 3 (7, 7)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(7n),
      this.instances3.alice.encrypt4(7n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 4 (7, 5)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(7n),
      this.instances3.alice.encrypt4(5n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint32, euint4) => euint32 test 1 (11, 11)', async function () {
    const res = await this.contract3.sub_euint32_euint4(
      this.instances3.alice.encrypt32(11n),
      this.instances3.alice.encrypt4(11n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint4) => euint32 test 2 (11, 7)', async function () {
    const res = await this.contract3.sub_euint32_euint4(
      this.instances3.alice.encrypt32(11n),
      this.instances3.alice.encrypt4(7n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 1 (7, 2)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(7n),
      this.instances3.alice.encrypt4(2n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 2 (3, 3)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(3n),
      this.instances3.alice.encrypt4(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 3 (3, 3)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(3n),
      this.instances3.alice.encrypt4(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 4 (3, 3)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(3n),
      this.instances3.alice.encrypt4(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 1 (1985406057, 9)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(1985406057n),
      this.instances3.alice.encrypt4(9n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 2 (5, 9)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(5n),
      this.instances3.alice.encrypt4(9n),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 3 (9, 9)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(9n),
      this.instances3.alice.encrypt4(9n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 4 (9, 5)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(9n),
      this.instances3.alice.encrypt4(5n),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 1 (508692335, 6)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(508692335n),
      this.instances3.alice.encrypt4(6n),
    );
    expect(res).to.equal(508692335n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(4n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 1 (967011552, 13)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(967011552n),
      this.instances3.alice.encrypt4(13n),
    );
    expect(res).to.equal(967011565n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 2 (9, 13)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(9n),
      this.instances3.alice.encrypt4(13n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 3 (13, 13)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(13n),
      this.instances3.alice.encrypt4(13n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 4 (13, 9)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(13n),
      this.instances3.alice.encrypt4(9n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 1 (3265717055, 10)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(3265717055n),
      this.instances3.alice.encrypt4(10n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(6n),
      this.instances3.alice.encrypt4(10n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(10n),
      this.instances3.alice.encrypt4(10n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(10n),
      this.instances3.alice.encrypt4(6n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 1 (357432318, 2)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(357432318n),
      this.instances3.alice.encrypt4(2n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(4n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 1 (2495129196, 13)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(2495129196n),
      this.instances3.alice.encrypt4(13n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(9n),
      this.instances3.alice.encrypt4(13n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(13n),
      this.instances3.alice.encrypt4(13n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(13n),
      this.instances3.alice.encrypt4(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 1 (2671970490, 2)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(2671970490n),
      this.instances3.alice.encrypt4(2n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(4n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 1 (4109122236, 4)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(4109122236n),
      this.instances3.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(4n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 1 (2705187582, 2)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(2705187582n),
      this.instances3.alice.encrypt4(2n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(4n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 1 (3409257759, 14)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(3409257759n),
      this.instances3.alice.encrypt4(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 2 (10, 14)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(10n),
      this.instances3.alice.encrypt4(14n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 3 (14, 14)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(14n),
      this.instances3.alice.encrypt4(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 4 (14, 10)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(14n),
      this.instances3.alice.encrypt4(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 1 (1630864445, 5)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(1630864445n),
      this.instances3.alice.encrypt4(5n),
    );
    expect(res).to.equal(1630864445n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(4n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(4n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (135, 2)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(135n),
      this.instances3.alice.encrypt8(2n),
    );
    expect(res).to.equal(137n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (113, 117)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(113n),
      this.instances3.alice.encrypt8(117n),
    );
    expect(res).to.equal(230n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (117, 117)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(117n),
      this.instances3.alice.encrypt8(117n),
    );
    expect(res).to.equal(234n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (117, 113)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(117n),
      this.instances3.alice.encrypt8(113n),
    );
    expect(res).to.equal(230n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (199, 199)', async function () {
    const res = await this.contract3.sub_euint32_euint8(
      this.instances3.alice.encrypt32(199n),
      this.instances3.alice.encrypt8(199n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (199, 195)', async function () {
    const res = await this.contract3.sub_euint32_euint8(
      this.instances3.alice.encrypt32(199n),
      this.instances3.alice.encrypt8(195n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (92, 2)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(92n),
      this.instances3.alice.encrypt8(2n),
    );
    expect(res).to.equal(184n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(4n),
      this.instances3.alice.encrypt8(8n),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt8(8n),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt8(4n),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (3414772297, 150)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(3414772297n),
      this.instances3.alice.encrypt8(150n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (146, 150)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(146n),
      this.instances3.alice.encrypt8(150n),
    );
    expect(res).to.equal(146n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (150, 150)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(150n),
      this.instances3.alice.encrypt8(150n),
    );
    expect(res).to.equal(150n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (150, 146)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(150n),
      this.instances3.alice.encrypt8(146n),
    );
    expect(res).to.equal(146n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (2068340748, 254)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(2068340748n),
      this.instances4.alice.encrypt8(254n),
    );
    expect(res).to.equal(2068340990n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (250, 254)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(250n),
      this.instances4.alice.encrypt8(254n),
    );
    expect(res).to.equal(254n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (254, 254)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(254n),
      this.instances4.alice.encrypt8(254n),
    );
    expect(res).to.equal(254n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (254, 250)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(254n),
      this.instances4.alice.encrypt8(250n),
    );
    expect(res).to.equal(254n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (2832110231, 71)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(2832110231n),
      this.instances4.alice.encrypt8(71n),
    );
    expect(res).to.equal(2832110288n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (67, 71)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(67n),
      this.instances4.alice.encrypt8(71n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (71, 71)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(71n),
      this.instances4.alice.encrypt8(71n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (71, 67)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(71n),
      this.instances4.alice.encrypt8(67n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (3290600270, 147)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(3290600270n),
      this.instances4.alice.encrypt8(147n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (143, 147)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(143n),
      this.instances4.alice.encrypt8(147n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 3 (147, 147)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(147n),
      this.instances4.alice.encrypt8(147n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 4 (147, 143)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(147n),
      this.instances4.alice.encrypt8(143n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (1247872648, 174)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(1247872648n),
      this.instances4.alice.encrypt8(174n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (170, 174)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(170n),
      this.instances4.alice.encrypt8(174n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 3 (174, 174)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(174n),
      this.instances4.alice.encrypt8(174n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 4 (174, 170)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(174n),
      this.instances4.alice.encrypt8(170n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (1853718080, 165)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(1853718080n),
      this.instances4.alice.encrypt8(165n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (161, 165)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(161n),
      this.instances4.alice.encrypt8(165n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (165, 165)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(165n),
      this.instances4.alice.encrypt8(165n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 4 (165, 161)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(165n),
      this.instances4.alice.encrypt8(161n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (2765170765, 138)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(2765170765n),
      this.instances4.alice.encrypt8(138n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (134, 138)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(134n),
      this.instances4.alice.encrypt8(138n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (138, 138)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(138n),
      this.instances4.alice.encrypt8(138n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 4 (138, 134)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(138n),
      this.instances4.alice.encrypt8(134n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (4269282942, 205)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(4269282942n),
      this.instances4.alice.encrypt8(205n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (201, 205)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(201n),
      this.instances4.alice.encrypt8(205n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (205, 205)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(205n),
      this.instances4.alice.encrypt8(205n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (205, 201)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(205n),
      this.instances4.alice.encrypt8(201n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (3904403618, 55)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(3904403618n),
      this.instances4.alice.encrypt8(55n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (51, 55)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(51n),
      this.instances4.alice.encrypt8(55n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (55, 55)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(55n),
      this.instances4.alice.encrypt8(55n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 4 (55, 51)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(55n),
      this.instances4.alice.encrypt8(51n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (790659700, 49)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(790659700n),
      this.instances4.alice.encrypt8(49n),
    );
    expect(res).to.equal(49n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (45, 49)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(45n),
      this.instances4.alice.encrypt8(49n),
    );
    expect(res).to.equal(45n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (49, 49)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(49n),
      this.instances4.alice.encrypt8(49n),
    );
    expect(res).to.equal(49n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (49, 45)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(49n),
      this.instances4.alice.encrypt8(45n),
    );
    expect(res).to.equal(45n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (2791780632, 46)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(2791780632n),
      this.instances4.alice.encrypt8(46n),
    );
    expect(res).to.equal(2791780632n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (42, 46)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(42n),
      this.instances4.alice.encrypt8(46n),
    );
    expect(res).to.equal(46n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (46, 46)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(46n),
      this.instances4.alice.encrypt8(46n),
    );
    expect(res).to.equal(46n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 4 (46, 42)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(46n),
      this.instances4.alice.encrypt8(42n),
    );
    expect(res).to.equal(46n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (44289, 2)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(44289n),
      this.instances4.alice.encrypt16(2n),
    );
    expect(res).to.equal(44291n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (21146, 21148)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(21146n),
      this.instances4.alice.encrypt16(21148n),
    );
    expect(res).to.equal(42294n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (21148, 21148)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(21148n),
      this.instances4.alice.encrypt16(21148n),
    );
    expect(res).to.equal(42296n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (21148, 21146)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(21148n),
      this.instances4.alice.encrypt16(21146n),
    );
    expect(res).to.equal(42294n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (5260, 5260)', async function () {
    const res = await this.contract4.sub_euint32_euint16(
      this.instances4.alice.encrypt32(5260n),
      this.instances4.alice.encrypt16(5260n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (5260, 5256)', async function () {
    const res = await this.contract4.sub_euint32_euint16(
      this.instances4.alice.encrypt32(5260n),
      this.instances4.alice.encrypt16(5256n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (21288, 2)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(21288n),
      this.instances4.alice.encrypt16(2n),
    );
    expect(res).to.equal(42576n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (154, 154)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(154n),
      this.instances4.alice.encrypt16(154n),
    );
    expect(res).to.equal(23716n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (154, 154)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(154n),
      this.instances4.alice.encrypt16(154n),
    );
    expect(res).to.equal(23716n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (154, 154)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(154n),
      this.instances4.alice.encrypt16(154n),
    );
    expect(res).to.equal(23716n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (1973938263, 52332)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(1973938263n),
      this.instances4.alice.encrypt16(52332n),
    );
    expect(res).to.equal(51268n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (52328, 52332)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(52328n),
      this.instances4.alice.encrypt16(52332n),
    );
    expect(res).to.equal(52328n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (52332, 52332)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(52332n),
      this.instances4.alice.encrypt16(52332n),
    );
    expect(res).to.equal(52332n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (52332, 52328)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(52332n),
      this.instances4.alice.encrypt16(52328n),
    );
    expect(res).to.equal(52328n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (1582044597, 3191)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(1582044597n),
      this.instances4.alice.encrypt16(3191n),
    );
    expect(res).to.equal(1582046711n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (3187, 3191)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(3187n),
      this.instances4.alice.encrypt16(3191n),
    );
    expect(res).to.equal(3191n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 3 (3191, 3191)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(3191n),
      this.instances4.alice.encrypt16(3191n),
    );
    expect(res).to.equal(3191n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 4 (3191, 3187)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(3191n),
      this.instances4.alice.encrypt16(3187n),
    );
    expect(res).to.equal(3191n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (2978626751, 41335)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(2978626751n),
      this.instances4.alice.encrypt16(41335n),
    );
    expect(res).to.equal(2978651592n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (41331, 41335)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(41331n),
      this.instances4.alice.encrypt16(41335n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (41335, 41335)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(41335n),
      this.instances4.alice.encrypt16(41335n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (41335, 41331)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(41335n),
      this.instances4.alice.encrypt16(41331n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (1823907444, 56958)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(1823907444n),
      this.instances4.alice.encrypt16(56958n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (56954, 56958)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(56954n),
      this.instances4.alice.encrypt16(56958n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (56958, 56958)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(56958n),
      this.instances4.alice.encrypt16(56958n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (56958, 56954)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(56958n),
      this.instances4.alice.encrypt16(56954n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (3638524537, 28898)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(3638524537n),
      this.instances4.alice.encrypt16(28898n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (28894, 28898)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(28894n),
      this.instances4.alice.encrypt16(28898n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 3 (28898, 28898)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(28898n),
      this.instances4.alice.encrypt16(28898n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 4 (28898, 28894)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(28898n),
      this.instances4.alice.encrypt16(28894n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (244234121, 57423)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(244234121n),
      this.instances4.alice.encrypt16(57423n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (57419, 57423)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(57419n),
      this.instances4.alice.encrypt16(57423n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (57423, 57423)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(57423n),
      this.instances4.alice.encrypt16(57423n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 4 (57423, 57419)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(57423n),
      this.instances4.alice.encrypt16(57419n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (190008396, 29281)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(190008396n),
      this.instances4.alice.encrypt16(29281n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (29277, 29281)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(29277n),
      this.instances4.alice.encrypt16(29281n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (29281, 29281)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(29281n),
      this.instances4.alice.encrypt16(29281n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 4 (29281, 29277)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(29281n),
      this.instances4.alice.encrypt16(29277n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (2809640232, 65084)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(2809640232n),
      this.instances4.alice.encrypt16(65084n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (65080, 65084)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(65080n),
      this.instances4.alice.encrypt16(65084n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (65084, 65084)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(65084n),
      this.instances4.alice.encrypt16(65084n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (65084, 65080)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(65084n),
      this.instances4.alice.encrypt16(65080n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (4280437825, 3080)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(4280437825n),
      this.instances4.alice.encrypt16(3080n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (3076, 3080)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(3076n),
      this.instances4.alice.encrypt16(3080n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (3080, 3080)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(3080n),
      this.instances4.alice.encrypt16(3080n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (3080, 3076)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(3080n),
      this.instances4.alice.encrypt16(3076n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (2343096104, 27257)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(2343096104n),
      this.instances4.alice.encrypt16(27257n),
    );
    expect(res).to.equal(27257n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (27253, 27257)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(27253n),
      this.instances4.alice.encrypt16(27257n),
    );
    expect(res).to.equal(27253n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (27257, 27257)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(27257n),
      this.instances4.alice.encrypt16(27257n),
    );
    expect(res).to.equal(27257n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (27257, 27253)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(27257n),
      this.instances4.alice.encrypt16(27253n),
    );
    expect(res).to.equal(27253n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (1916594677, 65516)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(1916594677n),
      this.instances4.alice.encrypt16(65516n),
    );
    expect(res).to.equal(1916594677n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (65512, 65516)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(65512n),
      this.instances4.alice.encrypt16(65516n),
    );
    expect(res).to.equal(65516n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (65516, 65516)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(65516n),
      this.instances4.alice.encrypt16(65516n),
    );
    expect(res).to.equal(65516n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (65516, 65512)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(65516n),
      this.instances4.alice.encrypt16(65512n),
    );
    expect(res).to.equal(65516n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (1114542222, 1859394045)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(1114542222n),
      this.instances4.alice.encrypt32(1859394045n),
    );
    expect(res).to.equal(2973936267n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (1114542220, 1114542222)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(1114542220n),
      this.instances4.alice.encrypt32(1114542222n),
    );
    expect(res).to.equal(2229084442n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (1114542222, 1114542222)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(1114542222n),
      this.instances4.alice.encrypt32(1114542222n),
    );
    expect(res).to.equal(2229084444n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (1114542222, 1114542220)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(1114542222n),
      this.instances4.alice.encrypt32(1114542220n),
    );
    expect(res).to.equal(2229084442n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (2227924198, 2227924198)', async function () {
    const res = await this.contract4.sub_euint32_euint32(
      this.instances4.alice.encrypt32(2227924198n),
      this.instances4.alice.encrypt32(2227924198n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (2227924198, 2227924194)', async function () {
    const res = await this.contract4.sub_euint32_euint32(
      this.instances4.alice.encrypt32(2227924198n),
      this.instances4.alice.encrypt32(2227924194n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (23578, 92355)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(23578n),
      this.instances4.alice.encrypt32(92355n),
    );
    expect(res).to.equal(2177546190n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 2 (47155, 47155)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(47155n),
      this.instances4.alice.encrypt32(47155n),
    );
    expect(res).to.equal(2223594025n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 3 (47155, 47155)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(47155n),
      this.instances4.alice.encrypt32(47155n),
    );
    expect(res).to.equal(2223594025n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 4 (47155, 47155)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(47155n),
      this.instances4.alice.encrypt32(47155n),
    );
    expect(res).to.equal(2223594025n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (1210734052, 3917577435)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(1210734052n),
      this.instances4.alice.encrypt32(3917577435n),
    );
    expect(res).to.equal(1207981248n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (1210734048, 1210734052)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(1210734048n),
      this.instances4.alice.encrypt32(1210734052n),
    );
    expect(res).to.equal(1210734048n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 3 (1210734052, 1210734052)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(1210734052n),
      this.instances4.alice.encrypt32(1210734052n),
    );
    expect(res).to.equal(1210734052n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 4 (1210734052, 1210734048)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(1210734052n),
      this.instances4.alice.encrypt32(1210734048n),
    );
    expect(res).to.equal(1210734048n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (3225478949, 1660008798)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(3225478949n),
      this.instances4.alice.encrypt32(1660008798n),
    );
    expect(res).to.equal(3807510399n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (1660008794, 1660008798)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(1660008794n),
      this.instances4.alice.encrypt32(1660008798n),
    );
    expect(res).to.equal(1660008798n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 3 (1660008798, 1660008798)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(1660008798n),
      this.instances4.alice.encrypt32(1660008798n),
    );
    expect(res).to.equal(1660008798n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 4 (1660008798, 1660008794)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(1660008798n),
      this.instances4.alice.encrypt32(1660008794n),
    );
    expect(res).to.equal(1660008798n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (1530698392, 3094509188)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(1530698392n),
      this.instances4.alice.encrypt32(3094509188n),
    );
    expect(res).to.equal(3813600284n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (1530698388, 1530698392)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(1530698388n),
      this.instances4.alice.encrypt32(1530698392n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (1530698392, 1530698392)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(1530698392n),
      this.instances4.alice.encrypt32(1530698392n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (1530698392, 1530698388)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(1530698392n),
      this.instances4.alice.encrypt32(1530698388n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (936892683, 2097167145)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(936892683n),
      this.instances4.alice.encrypt32(2097167145n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (936892679, 936892683)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(936892679n),
      this.instances4.alice.encrypt32(936892683n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (936892683, 936892683)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(936892683n),
      this.instances4.alice.encrypt32(936892683n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (936892683, 936892679)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(936892683n),
      this.instances4.alice.encrypt32(936892679n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (3125035557, 2092315500)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(3125035557n),
      this.instances4.alice.encrypt32(2092315500n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (2092315496, 2092315500)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(2092315496n),
      this.instances4.alice.encrypt32(2092315500n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 3 (2092315500, 2092315500)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(2092315500n),
      this.instances4.alice.encrypt32(2092315500n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 4 (2092315500, 2092315496)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(2092315500n),
      this.instances4.alice.encrypt32(2092315496n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (2871799948, 327585458)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(2871799948n),
      this.instances4.alice.encrypt32(327585458n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (327585454, 327585458)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(327585454n),
      this.instances4.alice.encrypt32(327585458n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (327585458, 327585458)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(327585458n),
      this.instances4.alice.encrypt32(327585458n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (327585458, 327585454)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(327585458n),
      this.instances4.alice.encrypt32(327585454n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (3656408513, 757192239)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(3656408513n),
      this.instances4.alice.encrypt32(757192239n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (757192235, 757192239)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(757192235n),
      this.instances4.alice.encrypt32(757192239n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (757192239, 757192239)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(757192239n),
      this.instances4.alice.encrypt32(757192239n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 4 (757192239, 757192235)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(757192239n),
      this.instances4.alice.encrypt32(757192235n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (419990657, 3512269933)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(419990657n),
      this.instances4.alice.encrypt32(3512269933n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (419990653, 419990657)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(419990653n),
      this.instances4.alice.encrypt32(419990657n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (419990657, 419990657)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(419990657n),
      this.instances4.alice.encrypt32(419990657n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 4 (419990657, 419990653)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(419990657n),
      this.instances4.alice.encrypt32(419990653n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (3206439576, 362263886)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(3206439576n),
      this.instances4.alice.encrypt32(362263886n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (362263882, 362263886)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(362263882n),
      this.instances4.alice.encrypt32(362263886n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (362263886, 362263886)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(362263886n),
      this.instances4.alice.encrypt32(362263886n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (362263886, 362263882)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(362263886n),
      this.instances4.alice.encrypt32(362263882n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (1560047482, 2374882166)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(1560047482n),
      this.instances4.alice.encrypt32(2374882166n),
    );
    expect(res).to.equal(1560047482n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (1560047478, 1560047482)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(1560047478n),
      this.instances4.alice.encrypt32(1560047482n),
    );
    expect(res).to.equal(1560047478n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (1560047482, 1560047482)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(1560047482n),
      this.instances4.alice.encrypt32(1560047482n),
    );
    expect(res).to.equal(1560047482n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (1560047482, 1560047478)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(1560047482n),
      this.instances4.alice.encrypt32(1560047478n),
    );
    expect(res).to.equal(1560047478n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (267116805, 2009338128)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(267116805n),
      this.instances4.alice.encrypt32(2009338128n),
    );
    expect(res).to.equal(2009338128n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (267116801, 267116805)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(267116801n),
      this.instances4.alice.encrypt32(267116805n),
    );
    expect(res).to.equal(267116805n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (267116805, 267116805)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(267116805n),
      this.instances4.alice.encrypt32(267116805n),
    );
    expect(res).to.equal(267116805n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (267116805, 267116801)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(267116805n),
      this.instances4.alice.encrypt32(267116801n),
    );
    expect(res).to.equal(267116805n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 1 (2, 4293885415)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(2n),
      this.instances4.alice.encrypt64(4293885415n),
    );
    expect(res).to.equal(4293885417n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 2 (235867974, 235867978)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(235867974n),
      this.instances4.alice.encrypt64(235867978n),
    );
    expect(res).to.equal(471735952n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 3 (235867978, 235867978)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(235867978n),
      this.instances4.alice.encrypt64(235867978n),
    );
    expect(res).to.equal(471735956n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 4 (235867978, 235867974)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(235867978n),
      this.instances4.alice.encrypt64(235867974n),
    );
    expect(res).to.equal(471735952n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (2215563243, 2215563243)', async function () {
    const res = await this.contract4.sub_euint32_euint64(
      this.instances4.alice.encrypt32(2215563243n),
      this.instances4.alice.encrypt64(2215563243n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (2215563243, 2215563239)', async function () {
    const res = await this.contract4.sub_euint32_euint64(
      this.instances4.alice.encrypt32(2215563243n),
      this.instances4.alice.encrypt64(2215563239n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (2, 2147178617)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(2n),
      this.instances4.alice.encrypt64(2147178617n),
    );
    expect(res).to.equal(4294357234n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (63982, 63982)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(63982n),
      this.instances4.alice.encrypt64(63982n),
    );
    expect(res).to.equal(4093696324n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (63982, 63982)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(63982n),
      this.instances4.alice.encrypt64(63982n),
    );
    expect(res).to.equal(4093696324n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (63982, 63982)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(63982n),
      this.instances4.alice.encrypt64(63982n),
    );
    expect(res).to.equal(4093696324n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 1 (2691900932, 18443792408865565825)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(2691900932n),
      this.instances4.alice.encrypt64(18443792408865565825n),
    );
    expect(res).to.equal(2691834880n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 2 (2691900928, 2691900932)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(2691900928n),
      this.instances4.alice.encrypt64(2691900932n),
    );
    expect(res).to.equal(2691900928n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 3 (2691900932, 2691900932)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(2691900932n),
      this.instances4.alice.encrypt64(2691900932n),
    );
    expect(res).to.equal(2691900932n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 4 (2691900932, 2691900928)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(2691900932n),
      this.instances4.alice.encrypt64(2691900928n),
    );
    expect(res).to.equal(2691900928n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 1 (2366610015, 18438916292667690081)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(2366610015n),
      this.instances4.alice.encrypt64(18438916292667690081n),
    );
    expect(res).to.equal(18438916292869086847n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 2 (2366610011, 2366610015)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(2366610011n),
      this.instances4.alice.encrypt64(2366610015n),
    );
    expect(res).to.equal(2366610015n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 3 (2366610015, 2366610015)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(2366610015n),
      this.instances4.alice.encrypt64(2366610015n),
    );
    expect(res).to.equal(2366610015n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 4 (2366610015, 2366610011)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(2366610015n),
      this.instances4.alice.encrypt64(2366610011n),
    );
    expect(res).to.equal(2366610015n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (1560937853, 18439758140674320695)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(1560937853n),
      this.instances4.alice.encrypt64(18439758140674320695n),
    );
    expect(res).to.equal(18439758141530352714n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (1560937849, 1560937853)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(1560937849n),
      this.instances4.alice.encrypt64(1560937853n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (1560937853, 1560937853)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(1560937853n),
      this.instances4.alice.encrypt64(1560937853n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (1560937853, 1560937849)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(1560937853n),
      this.instances4.alice.encrypt64(1560937849n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 1 (202687275, 18441470248109604137)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(202687275n),
      this.instances4.alice.encrypt64(18441470248109604137n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 2 (202687271, 202687275)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(202687271n),
      this.instances4.alice.encrypt64(202687275n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 3 (202687275, 202687275)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(202687275n),
      this.instances4.alice.encrypt64(202687275n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 4 (202687275, 202687271)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(202687275n),
      this.instances4.alice.encrypt64(202687271n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (183060501, 18444145979610672717)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(183060501n),
      this.instances4.alice.encrypt64(18444145979610672717n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (183060497, 183060501)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(183060497n),
      this.instances4.alice.encrypt64(183060501n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (183060501, 183060501)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(183060501n),
      this.instances4.alice.encrypt64(183060501n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (183060501, 183060497)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(183060501n),
      this.instances4.alice.encrypt64(183060497n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 1 (920534420, 18445111792944685137)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(920534420n),
      this.instances4.alice.encrypt64(18445111792944685137n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 2 (920534416, 920534420)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(920534416n),
      this.instances4.alice.encrypt64(920534420n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 3 (920534420, 920534420)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(920534420n),
      this.instances4.alice.encrypt64(920534420n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 4 (920534420, 920534416)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(920534420n),
      this.instances4.alice.encrypt64(920534416n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 1 (2098301212, 18442675480743402157)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(2098301212n),
      this.instances4.alice.encrypt64(18442675480743402157n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 2 (2098301208, 2098301212)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(2098301208n),
      this.instances4.alice.encrypt64(2098301212n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 3 (2098301212, 2098301212)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(2098301212n),
      this.instances4.alice.encrypt64(2098301212n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 4 (2098301212, 2098301208)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(2098301212n),
      this.instances4.alice.encrypt64(2098301208n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 1 (1012978392, 18445874202420647401)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(1012978392n),
      this.instances4.alice.encrypt64(18445874202420647401n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 2 (1012978388, 1012978392)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(1012978388n),
      this.instances4.alice.encrypt64(1012978392n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 3 (1012978392, 1012978392)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(1012978392n),
      this.instances4.alice.encrypt64(1012978392n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 4 (1012978392, 1012978388)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(1012978392n),
      this.instances4.alice.encrypt64(1012978388n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 1 (752929415, 18443916236902180471)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(752929415n),
      this.instances4.alice.encrypt64(18443916236902180471n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 2 (752929411, 752929415)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(752929411n),
      this.instances4.alice.encrypt64(752929415n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 3 (752929415, 752929415)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(752929415n),
      this.instances4.alice.encrypt64(752929415n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 4 (752929415, 752929411)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(752929415n),
      this.instances4.alice.encrypt64(752929411n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 1 (3551198442, 18441306633676294663)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(3551198442n),
      this.instances4.alice.encrypt64(18441306633676294663n),
    );
    expect(res).to.equal(3551198442n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 2 (3551198438, 3551198442)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(3551198438n),
      this.instances4.alice.encrypt64(3551198442n),
    );
    expect(res).to.equal(3551198438n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 3 (3551198442, 3551198442)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(3551198442n),
      this.instances4.alice.encrypt64(3551198442n),
    );
    expect(res).to.equal(3551198442n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 4 (3551198442, 3551198438)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(3551198442n),
      this.instances4.alice.encrypt64(3551198438n),
    );
    expect(res).to.equal(3551198438n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (2673994445, 18440801377188498231)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(2673994445n),
      this.instances4.alice.encrypt64(18440801377188498231n),
    );
    expect(res).to.equal(18440801377188498231n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (2673994441, 2673994445)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(2673994441n),
      this.instances4.alice.encrypt64(2673994445n),
    );
    expect(res).to.equal(2673994445n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (2673994445, 2673994445)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(2673994445n),
      this.instances4.alice.encrypt64(2673994445n),
    );
    expect(res).to.equal(2673994445n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (2673994445, 2673994441)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(2673994445n),
      this.instances4.alice.encrypt64(2673994441n),
    );
    expect(res).to.equal(2673994445n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (2229084442, 831779538)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(2229084442n), 831779538n);
    expect(res).to.equal(3060863980n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (1114542220, 1114542222)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(1114542220n), 1114542222n);
    expect(res).to.equal(2229084442n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (1114542222, 1114542222)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(1114542222n), 1114542222n);
    expect(res).to.equal(2229084444n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (1114542222, 1114542220)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(1114542222n), 1114542220n);
    expect(res).to.equal(2229084442n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (2606635759, 831779538)', async function () {
    const res = await this.contract4.add_uint32_euint32(2606635759n, this.instances4.alice.encrypt32(831779538n));
    expect(res).to.equal(3438415297n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 2 (1114542220, 1114542222)', async function () {
    const res = await this.contract4.add_uint32_euint32(1114542220n, this.instances4.alice.encrypt32(1114542222n));
    expect(res).to.equal(2229084442n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 3 (1114542222, 1114542222)', async function () {
    const res = await this.contract4.add_uint32_euint32(1114542222n, this.instances4.alice.encrypt32(1114542222n));
    expect(res).to.equal(2229084444n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 4 (1114542222, 1114542220)', async function () {
    const res = await this.contract4.add_uint32_euint32(1114542222n, this.instances4.alice.encrypt32(1114542220n));
    expect(res).to.equal(2229084442n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (2227924198, 2227924198)', async function () {
    const res = await this.contract4.sub_euint32_uint32(this.instances4.alice.encrypt32(2227924198n), 2227924198n);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 2 (2227924198, 2227924194)', async function () {
    const res = await this.contract4.sub_euint32_uint32(this.instances4.alice.encrypt32(2227924198n), 2227924194n);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (2227924198, 2227924198)', async function () {
    const res = await this.contract4.sub_uint32_euint32(2227924198n, this.instances4.alice.encrypt32(2227924198n));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 2 (2227924198, 2227924194)', async function () {
    const res = await this.contract4.sub_uint32_euint32(2227924198n, this.instances4.alice.encrypt32(2227924194n));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (23578, 115302)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(23578n), 115302n);
    expect(res).to.equal(2718590556n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 2 (47155, 47155)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(47155n), 47155n);
    expect(res).to.equal(2223594025n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 3 (47155, 47155)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(47155n), 47155n);
    expect(res).to.equal(2223594025n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 4 (47155, 47155)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(47155n), 47155n);
    expect(res).to.equal(2223594025n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (61158, 57652)', async function () {
    const res = await this.contract4.mul_uint32_euint32(61158n, this.instances4.alice.encrypt32(57652n));
    expect(res).to.equal(3525881016n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (47155, 47155)', async function () {
    const res = await this.contract4.mul_uint32_euint32(47155n, this.instances4.alice.encrypt32(47155n));
    expect(res).to.equal(2223594025n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (47155, 47155)', async function () {
    const res = await this.contract4.mul_uint32_euint32(47155n, this.instances4.alice.encrypt32(47155n));
    expect(res).to.equal(2223594025n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (47155, 47155)', async function () {
    const res = await this.contract4.mul_uint32_euint32(47155n, this.instances4.alice.encrypt32(47155n));
    expect(res).to.equal(2223594025n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (2118241942, 2564265476)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(2118241942n), 2564265476n);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (2118241938, 2118241942)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(2118241938n), 2118241942n);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (2118241942, 2118241942)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(2118241942n), 2118241942n);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (2118241942, 2118241938)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(2118241942n), 2118241938n);
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (3451190912, 2132331803)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(3451190912n), 2132331803n);
    expect(res).to.equal(1318859109n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 2 (1071530833, 1071530837)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(1071530833n), 1071530837n);
    expect(res).to.equal(1071530833n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 3 (1071530837, 1071530837)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(1071530837n), 1071530837n);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 4 (1071530837, 1071530833)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(1071530837n), 1071530833n);
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (936892683, 562516695)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(936892683n), 562516695n);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (936892679, 936892683)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(936892679n), 936892683n);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 3 (936892683, 936892683)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(936892683n), 936892683n);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 4 (936892683, 936892679)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(936892683n), 936892679n);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (1342862361, 562516695)', async function () {
    const res = await this.contract4.eq_uint32_euint32(1342862361n, this.instances4.alice.encrypt32(562516695n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (936892679, 936892683)', async function () {
    const res = await this.contract4.eq_uint32_euint32(936892679n, this.instances4.alice.encrypt32(936892683n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 3 (936892683, 936892683)', async function () {
    const res = await this.contract4.eq_uint32_euint32(936892683n, this.instances4.alice.encrypt32(936892683n));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 4 (936892683, 936892679)', async function () {
    const res = await this.contract4.eq_uint32_euint32(936892683n, this.instances4.alice.encrypt32(936892679n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (3125035557, 3970623572)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(3125035557n), 3970623572n);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (2092315496, 2092315500)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(2092315496n), 2092315500n);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 3 (2092315500, 2092315500)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(2092315500n), 2092315500n);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 4 (2092315500, 2092315496)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(2092315500n), 2092315496n);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (3964920568, 3970623572)', async function () {
    const res = await this.contract4.ne_uint32_euint32(3964920568n, this.instances4.alice.encrypt32(3970623572n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (2092315496, 2092315500)', async function () {
    const res = await this.contract4.ne_uint32_euint32(2092315496n, this.instances4.alice.encrypt32(2092315500n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 3 (2092315500, 2092315500)', async function () {
    const res = await this.contract4.ne_uint32_euint32(2092315500n, this.instances4.alice.encrypt32(2092315500n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 4 (2092315500, 2092315496)', async function () {
    const res = await this.contract4.ne_uint32_euint32(2092315500n, this.instances4.alice.encrypt32(2092315496n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (2871799948, 190762344)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(2871799948n), 190762344n);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (327585454, 327585458)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(327585454n), 327585458n);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (327585458, 327585458)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(327585458n), 327585458n);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 4 (327585458, 327585454)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(327585458n), 327585454n);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (4095559129, 190762344)', async function () {
    const res = await this.contract4.ge_uint32_euint32(4095559129n, this.instances4.alice.encrypt32(190762344n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (327585454, 327585458)', async function () {
    const res = await this.contract4.ge_uint32_euint32(327585454n, this.instances4.alice.encrypt32(327585458n));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (327585458, 327585458)', async function () {
    const res = await this.contract4.ge_uint32_euint32(327585458n, this.instances4.alice.encrypt32(327585458n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (327585458, 327585454)', async function () {
    const res = await this.contract4.ge_uint32_euint32(327585458n, this.instances4.alice.encrypt32(327585454n));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (3656408513, 2219426017)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(3656408513n), 2219426017n);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (757192235, 757192239)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(757192235n), 757192239n);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (757192239, 757192239)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(757192239n), 757192239n);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 4 (757192239, 757192235)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(757192239n), 757192235n);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (2002645436, 2219426017)', async function () {
    const res = await this.contract4.gt_uint32_euint32(2002645436n, this.instances4.alice.encrypt32(2219426017n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (757192235, 757192239)', async function () {
    const res = await this.contract4.gt_uint32_euint32(757192235n, this.instances4.alice.encrypt32(757192239n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (757192239, 757192239)', async function () {
    const res = await this.contract4.gt_uint32_euint32(757192239n, this.instances4.alice.encrypt32(757192239n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 4 (757192239, 757192235)', async function () {
    const res = await this.contract4.gt_uint32_euint32(757192239n, this.instances4.alice.encrypt32(757192235n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (419990657, 3083955967)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(419990657n), 3083955967n);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (419990653, 419990657)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(419990653n), 419990657n);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (419990657, 419990657)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(419990657n), 419990657n);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 4 (419990657, 419990653)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(419990657n), 419990653n);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (3765055757, 3083955967)', async function () {
    const res = await this.contract4.le_uint32_euint32(3765055757n, this.instances4.alice.encrypt32(3083955967n));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (419990653, 419990657)', async function () {
    const res = await this.contract4.le_uint32_euint32(419990653n, this.instances4.alice.encrypt32(419990657n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (419990657, 419990657)', async function () {
    const res = await this.contract4.le_uint32_euint32(419990657n, this.instances4.alice.encrypt32(419990657n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 4 (419990657, 419990653)', async function () {
    const res = await this.contract4.le_uint32_euint32(419990657n, this.instances4.alice.encrypt32(419990653n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (3206439576, 1387743217)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(3206439576n), 1387743217n);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (362263882, 362263886)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(362263882n), 362263886n);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (362263886, 362263886)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(362263886n), 362263886n);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 4 (362263886, 362263882)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(362263886n), 362263882n);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (1026858228, 1387743217)', async function () {
    const res = await this.contract4.lt_uint32_euint32(1026858228n, this.instances4.alice.encrypt32(1387743217n));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (362263882, 362263886)', async function () {
    const res = await this.contract4.lt_uint32_euint32(362263882n, this.instances4.alice.encrypt32(362263886n));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (362263886, 362263886)', async function () {
    const res = await this.contract4.lt_uint32_euint32(362263886n, this.instances4.alice.encrypt32(362263886n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 4 (362263886, 362263882)', async function () {
    const res = await this.contract4.lt_uint32_euint32(362263886n, this.instances4.alice.encrypt32(362263882n));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (1560047482, 1226912561)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(1560047482n), 1226912561n);
    expect(res).to.equal(1226912561n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (1560047478, 1560047482)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(1560047478n), 1560047482n);
    expect(res).to.equal(1560047478n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (1560047482, 1560047482)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(1560047482n), 1560047482n);
    expect(res).to.equal(1560047482n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 4 (1560047482, 1560047478)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(1560047482n), 1560047478n);
    expect(res).to.equal(1560047478n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (4019036246, 1226912561)', async function () {
    const res = await this.contract4.min_uint32_euint32(4019036246n, this.instances4.alice.encrypt32(1226912561n));
    expect(res).to.equal(1226912561n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (1560047478, 1560047482)', async function () {
    const res = await this.contract4.min_uint32_euint32(1560047478n, this.instances4.alice.encrypt32(1560047482n));
    expect(res).to.equal(1560047478n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (1560047482, 1560047482)', async function () {
    const res = await this.contract4.min_uint32_euint32(1560047482n, this.instances4.alice.encrypt32(1560047482n));
    expect(res).to.equal(1560047482n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 4 (1560047482, 1560047478)', async function () {
    const res = await this.contract4.min_uint32_euint32(1560047482n, this.instances4.alice.encrypt32(1560047478n));
    expect(res).to.equal(1560047478n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (267116805, 2729936821)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(267116805n), 2729936821n);
    expect(res).to.equal(2729936821n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (267116801, 267116805)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(267116801n), 267116805n);
    expect(res).to.equal(267116805n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (267116805, 267116805)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(267116805n), 267116805n);
    expect(res).to.equal(267116805n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 4 (267116805, 267116801)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(267116805n), 267116801n);
    expect(res).to.equal(267116805n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (1528010013, 2729936821)', async function () {
    const res = await this.contract4.max_uint32_euint32(1528010013n, this.instances4.alice.encrypt32(2729936821n));
    expect(res).to.equal(2729936821n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (267116801, 267116805)', async function () {
    const res = await this.contract4.max_uint32_euint32(267116801n, this.instances4.alice.encrypt32(267116805n));
    expect(res).to.equal(267116805n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (267116805, 267116805)', async function () {
    const res = await this.contract4.max_uint32_euint32(267116805n, this.instances4.alice.encrypt32(267116805n));
    expect(res).to.equal(267116805n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 4 (267116805, 267116801)', async function () {
    const res = await this.contract4.max_uint32_euint32(267116805n, this.instances4.alice.encrypt32(267116801n));
    expect(res).to.equal(267116805n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 1 (9, 2)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(9n),
      this.instances4.alice.encrypt4(2n),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(4n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 3 (5, 5)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(5n),
      this.instances4.alice.encrypt4(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint64, euint4) => euint64 test 1 (8, 8)', async function () {
    const res = await this.contract4.sub_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint4) => euint64 test 2 (8, 4)', async function () {
    const res = await this.contract4.sub_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 1 (5, 2)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(5n),
      this.instances4.alice.encrypt4(2n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 2 (3, 5)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(3n),
      this.instances4.alice.encrypt4(5n),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 3 (3, 3)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(3n),
      this.instances4.alice.encrypt4(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 4 (5, 3)', async function () {
    const res = await this.contract4.mul_euint64_euint4(
      this.instances4.alice.encrypt64(5n),
      this.instances4.alice.encrypt4(3n),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 1 (18438385621874912119, 13)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(18438385621874912119n),
      this.instances4.alice.encrypt4(13n),
    );
    expect(res).to.equal(5n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 2 (9, 13)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(9n),
      this.instances4.alice.encrypt4(13n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 3 (13, 13)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(13n),
      this.instances4.alice.encrypt4(13n),
    );
    expect(res).to.equal(13n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 4 (13, 9)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(13n),
      this.instances4.alice.encrypt4(9n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 1 (18441517670030272959, 12)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(18441517670030272959n),
      this.instances4.alice.encrypt4(12n),
    );
    expect(res).to.equal(18441517670030272959n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 2 (8, 12)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(12n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 3 (12, 12)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(12n),
      this.instances4.alice.encrypt4(12n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 4 (12, 8)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(12n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 1 (18441793080321119931, 9)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(18441793080321119931n),
      this.instances4.alice.encrypt4(9n),
    );
    expect(res).to.equal(18441793080321119922n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 2 (5, 9)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(5n),
      this.instances4.alice.encrypt4(9n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 3 (9, 9)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(9n),
      this.instances4.alice.encrypt4(9n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 4 (9, 5)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(9n),
      this.instances4.alice.encrypt4(5n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 1 (18439404178919589045, 5)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(18439404178919589045n),
      this.instances4.alice.encrypt4(5n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(4n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 1 (18445431420464402821, 14)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(18445431420464402821n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(10n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(14n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(14n),
      this.instances4.alice.encrypt4(10n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 1 (18439304491127934949, 6)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(18439304491127934949n),
      this.instances4.alice.encrypt4(6n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(4n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 1 (18438978913305655493, 5)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(18438978913305655493n),
      this.instances4.alice.encrypt4(5n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(4n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 1 (18445161087311088075, 10)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(18445161087311088075n),
      this.instances4.alice.encrypt4(10n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(6n),
      this.instances4.alice.encrypt4(10n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(10n),
      this.instances4.alice.encrypt4(10n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(10n),
      this.instances4.alice.encrypt4(6n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 1 (18444555938475269429, 4)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(18444555938475269429n),
      this.instances4.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(4n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 1 (18444946054883221371, 5)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(18444946054883221371n),
      this.instances4.alice.encrypt4(5n),
    );
    expect(res).to.equal(5n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(4n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 1 (18443614046966819915, 14)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(18443614046966819915n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(18443614046966819915n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 2 (10, 14)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(10n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 3 (14, 14)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(14n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 4 (14, 10)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(14n),
      this.instances4.alice.encrypt4(10n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 1 (129, 2)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(129n),
      this.instances4.alice.encrypt8(2n),
    );
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 2 (68, 70)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(68n),
      this.instances4.alice.encrypt8(70n),
    );
    expect(res).to.equal(138n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 3 (70, 70)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(70n),
      this.instances4.alice.encrypt8(70n),
    );
    expect(res).to.equal(140n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 4 (70, 68)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(70n),
      this.instances4.alice.encrypt8(68n),
    );
    expect(res).to.equal(138n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 1 (84, 84)', async function () {
    const res = await this.contract4.sub_euint64_euint8(
      this.instances4.alice.encrypt64(84n),
      this.instances4.alice.encrypt8(84n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 2 (84, 80)', async function () {
    const res = await this.contract4.sub_euint64_euint8(
      this.instances4.alice.encrypt64(84n),
      this.instances4.alice.encrypt8(80n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 1 (65, 2)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(65n),
      this.instances4.alice.encrypt8(2n),
    );
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 2 (9, 10)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(9n),
      this.instances4.alice.encrypt8(10n),
    );
    expect(res).to.equal(90n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 3 (10, 10)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(10n),
      this.instances4.alice.encrypt8(10n),
    );
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 4 (10, 9)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(10n),
      this.instances4.alice.encrypt8(9n),
    );
    expect(res).to.equal(90n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 1 (18442357649689732045, 154)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(18442357649689732045n),
      this.instances4.alice.encrypt8(154n),
    );
    expect(res).to.equal(136n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 2 (150, 154)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(150n),
      this.instances4.alice.encrypt8(154n),
    );
    expect(res).to.equal(146n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 3 (154, 154)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(154n),
      this.instances4.alice.encrypt8(154n),
    );
    expect(res).to.equal(154n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 4 (154, 150)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(154n),
      this.instances4.alice.encrypt8(150n),
    );
    expect(res).to.equal(146n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 1 (18443183887478024691, 189)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(18443183887478024691n),
      this.instances4.alice.encrypt8(189n),
    );
    expect(res).to.equal(18443183887478024703n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 2 (185, 189)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(185n),
      this.instances4.alice.encrypt8(189n),
    );
    expect(res).to.equal(189n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 3 (189, 189)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(189n),
      this.instances4.alice.encrypt8(189n),
    );
    expect(res).to.equal(189n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 4 (189, 185)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(189n),
      this.instances4.alice.encrypt8(185n),
    );
    expect(res).to.equal(189n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 1 (18445815661885535957, 27)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(18445815661885535957n),
      this.instances4.alice.encrypt8(27n),
    );
    expect(res).to.equal(18445815661885535950n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 2 (23, 27)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(23n),
      this.instances4.alice.encrypt8(27n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 3 (27, 27)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(27n),
      this.instances4.alice.encrypt8(27n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 4 (27, 23)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(27n),
      this.instances4.alice.encrypt8(23n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 1 (18441464536800333931, 61)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(18441464536800333931n),
      this.instances4.alice.encrypt8(61n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 2 (57, 61)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(57n),
      this.instances4.alice.encrypt8(61n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 3 (61, 61)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(61n),
      this.instances4.alice.encrypt8(61n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 4 (61, 57)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(61n),
      this.instances4.alice.encrypt8(57n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 1 (18445535581731799625, 206)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(18445535581731799625n),
      this.instances4.alice.encrypt8(206n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 2 (202, 206)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(202n),
      this.instances4.alice.encrypt8(206n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 3 (206, 206)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(206n),
      this.instances4.alice.encrypt8(206n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 4 (206, 202)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(206n),
      this.instances4.alice.encrypt8(202n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 1 (18438747736171818973, 36)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(18438747736171818973n),
      this.instances4.alice.encrypt8(36n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 2 (32, 36)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(32n),
      this.instances4.alice.encrypt8(36n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 3 (36, 36)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(36n),
      this.instances4.alice.encrypt8(36n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 4 (36, 32)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(36n),
      this.instances4.alice.encrypt8(32n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 1 (18440518059209241675, 65)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(18440518059209241675n),
      this.instances4.alice.encrypt8(65n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 2 (61, 65)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(61n),
      this.instances4.alice.encrypt8(65n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 3 (65, 65)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(65n),
      this.instances4.alice.encrypt8(65n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 4 (65, 61)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(65n),
      this.instances4.alice.encrypt8(61n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 1 (18438408285593999781, 1)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(18438408285593999781n),
      this.instances5.alice.encrypt8(1n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 1 (18441962709425103459, 182)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(18441962709425103459n),
      this.instances5.alice.encrypt8(182n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 2 (178, 182)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(178n),
      this.instances5.alice.encrypt8(182n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 3 (182, 182)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(182n),
      this.instances5.alice.encrypt8(182n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 4 (182, 178)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(182n),
      this.instances5.alice.encrypt8(178n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 1 (18442010296993772325, 1)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(18442010296993772325n),
      this.instances5.alice.encrypt8(1n),
    );
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 1 (18443582207772596877, 242)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(18443582207772596877n),
      this.instances5.alice.encrypt8(242n),
    );
    expect(res).to.equal(18443582207772596877n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 2 (238, 242)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(238n),
      this.instances5.alice.encrypt8(242n),
    );
    expect(res).to.equal(242n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 3 (242, 242)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(242n),
      this.instances5.alice.encrypt8(242n),
    );
    expect(res).to.equal(242n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 4 (242, 238)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(242n),
      this.instances5.alice.encrypt8(238n),
    );
    expect(res).to.equal(242n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 1 (32768, 2)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(32768n),
      this.instances5.alice.encrypt16(2n),
    );
    expect(res).to.equal(32770n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 2 (22003, 22007)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(22003n),
      this.instances5.alice.encrypt16(22007n),
    );
    expect(res).to.equal(44010n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 3 (22007, 22007)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(22007n),
      this.instances5.alice.encrypt16(22007n),
    );
    expect(res).to.equal(44014n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 4 (22007, 22003)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(22007n),
      this.instances5.alice.encrypt16(22003n),
    );
    expect(res).to.equal(44010n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 1 (26454, 26454)', async function () {
    const res = await this.contract5.sub_euint64_euint16(
      this.instances5.alice.encrypt64(26454n),
      this.instances5.alice.encrypt16(26454n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 2 (26454, 26450)', async function () {
    const res = await this.contract5.sub_euint64_euint16(
      this.instances5.alice.encrypt64(26454n),
      this.instances5.alice.encrypt16(26450n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 1 (32760, 2)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(32760n),
      this.instances5.alice.encrypt16(2n),
    );
    expect(res).to.equal(65520n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 2 (133, 133)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(133n),
      this.instances5.alice.encrypt16(133n),
    );
    expect(res).to.equal(17689n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 3 (133, 133)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(133n),
      this.instances5.alice.encrypt16(133n),
    );
    expect(res).to.equal(17689n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 4 (133, 133)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(133n),
      this.instances5.alice.encrypt16(133n),
    );
    expect(res).to.equal(17689n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 1 (18440659024486493899, 49786)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(18440659024486493899n),
      this.instances5.alice.encrypt16(49786n),
    );
    expect(res).to.equal(16970n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 2 (49782, 49786)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(49782n),
      this.instances5.alice.encrypt16(49786n),
    );
    expect(res).to.equal(49778n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 3 (49786, 49786)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(49786n),
      this.instances5.alice.encrypt16(49786n),
    );
    expect(res).to.equal(49786n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 4 (49786, 49782)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(49786n),
      this.instances5.alice.encrypt16(49782n),
    );
    expect(res).to.equal(49778n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 1 (18441498621886349737, 58007)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(18441498621886349737n),
      this.instances5.alice.encrypt16(58007n),
    );
    expect(res).to.equal(18441498621886391231n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 2 (58003, 58007)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(58003n),
      this.instances5.alice.encrypt16(58007n),
    );
    expect(res).to.equal(58007n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 3 (58007, 58007)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(58007n),
      this.instances5.alice.encrypt16(58007n),
    );
    expect(res).to.equal(58007n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 4 (58007, 58003)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(58007n),
      this.instances5.alice.encrypt16(58003n),
    );
    expect(res).to.equal(58007n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 1 (18438525085683773351, 14371)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(18438525085683773351n),
      this.instances5.alice.encrypt16(14371n),
    );
    expect(res).to.equal(18438525085683767172n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 2 (14367, 14371)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(14367n),
      this.instances5.alice.encrypt16(14371n),
    );
    expect(res).to.equal(60n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 3 (14371, 14371)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(14371n),
      this.instances5.alice.encrypt16(14371n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 4 (14371, 14367)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(14371n),
      this.instances5.alice.encrypt16(14367n),
    );
    expect(res).to.equal(60n);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 1 (18441759567832827065, 47235)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(18441759567832827065n),
      this.instances5.alice.encrypt16(47235n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 2 (47231, 47235)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(47231n),
      this.instances5.alice.encrypt16(47235n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 3 (47235, 47235)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(47235n),
      this.instances5.alice.encrypt16(47235n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 4 (47235, 47231)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(47235n),
      this.instances5.alice.encrypt16(47231n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 1 (18438198467601571499, 50309)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(18438198467601571499n),
      this.instances5.alice.encrypt16(50309n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 2 (50305, 50309)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(50305n),
      this.instances5.alice.encrypt16(50309n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 3 (50309, 50309)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(50309n),
      this.instances5.alice.encrypt16(50309n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 4 (50309, 50305)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(50309n),
      this.instances5.alice.encrypt16(50305n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 1 (18444761807759616923, 39078)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(18444761807759616923n),
      this.instances5.alice.encrypt16(39078n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 2 (39074, 39078)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(39074n),
      this.instances5.alice.encrypt16(39078n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 3 (39078, 39078)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(39078n),
      this.instances5.alice.encrypt16(39078n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 4 (39078, 39074)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(39078n),
      this.instances5.alice.encrypt16(39074n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 1 (18443499982556129285, 40011)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(18443499982556129285n),
      this.instances5.alice.encrypt16(40011n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 2 (40007, 40011)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(40007n),
      this.instances5.alice.encrypt16(40011n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 3 (40011, 40011)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(40011n),
      this.instances5.alice.encrypt16(40011n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 4 (40011, 40007)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(40011n),
      this.instances5.alice.encrypt16(40007n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 1 (18438700253908998303, 54224)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(18438700253908998303n),
      this.instances5.alice.encrypt16(54224n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 2 (54220, 54224)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(54220n),
      this.instances5.alice.encrypt16(54224n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 3 (54224, 54224)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(54224n),
      this.instances5.alice.encrypt16(54224n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 4 (54224, 54220)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(54224n),
      this.instances5.alice.encrypt16(54220n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 1 (18438154654269388321, 41644)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(18438154654269388321n),
      this.instances5.alice.encrypt16(41644n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 2 (41640, 41644)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(41640n),
      this.instances5.alice.encrypt16(41644n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 3 (41644, 41644)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(41644n),
      this.instances5.alice.encrypt16(41644n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 4 (41644, 41640)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(41644n),
      this.instances5.alice.encrypt16(41640n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 1 (18440798283313050593, 4514)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(18440798283313050593n),
      this.instances5.alice.encrypt16(4514n),
    );
    expect(res).to.equal(4514n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 2 (4510, 4514)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(4510n),
      this.instances5.alice.encrypt16(4514n),
    );
    expect(res).to.equal(4510n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 3 (4514, 4514)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(4514n),
      this.instances5.alice.encrypt16(4514n),
    );
    expect(res).to.equal(4514n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 4 (4514, 4510)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(4514n),
      this.instances5.alice.encrypt16(4510n),
    );
    expect(res).to.equal(4510n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (18439993166888239399, 38438)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(18439993166888239399n),
      this.instances5.alice.encrypt16(38438n),
    );
    expect(res).to.equal(18439993166888239399n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (38434, 38438)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(38434n),
      this.instances5.alice.encrypt16(38438n),
    );
    expect(res).to.equal(38438n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (38438, 38438)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(38438n),
      this.instances5.alice.encrypt16(38438n),
    );
    expect(res).to.equal(38438n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (38438, 38434)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(38438n),
      this.instances5.alice.encrypt16(38434n),
    );
    expect(res).to.equal(38438n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 1 (4293324292, 2)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(4293324292n),
      this.instances5.alice.encrypt32(2n),
    );
    expect(res).to.equal(4293324294n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 2 (1577057496, 1577057498)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(1577057496n),
      this.instances5.alice.encrypt32(1577057498n),
    );
    expect(res).to.equal(3154114994n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 3 (1577057498, 1577057498)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(1577057498n),
      this.instances5.alice.encrypt32(1577057498n),
    );
    expect(res).to.equal(3154114996n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 4 (1577057498, 1577057496)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(1577057498n),
      this.instances5.alice.encrypt32(1577057496n),
    );
    expect(res).to.equal(3154114994n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 1 (1759976836, 1759976836)', async function () {
    const res = await this.contract5.sub_euint64_euint32(
      this.instances5.alice.encrypt64(1759976836n),
      this.instances5.alice.encrypt32(1759976836n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 2 (1759976836, 1759976832)', async function () {
    const res = await this.contract5.sub_euint64_euint32(
      this.instances5.alice.encrypt64(1759976836n),
      this.instances5.alice.encrypt32(1759976832n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 1 (2146804614, 2)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(2146804614n),
      this.instances5.alice.encrypt32(2n),
    );
    expect(res).to.equal(4293609228n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 2 (36035, 36035)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(36035n),
      this.instances5.alice.encrypt32(36035n),
    );
    expect(res).to.equal(1298521225n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 3 (36035, 36035)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(36035n),
      this.instances5.alice.encrypt32(36035n),
    );
    expect(res).to.equal(1298521225n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 4 (36035, 36035)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(36035n),
      this.instances5.alice.encrypt32(36035n),
    );
    expect(res).to.equal(1298521225n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 1 (18446338251263313345, 1815122101)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(18446338251263313345n),
      this.instances5.alice.encrypt32(1815122101n),
    );
    expect(res).to.equal(134217857n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 2 (1815122097, 1815122101)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(1815122097n),
      this.instances5.alice.encrypt32(1815122101n),
    );
    expect(res).to.equal(1815122097n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 3 (1815122101, 1815122101)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(1815122101n),
      this.instances5.alice.encrypt32(1815122101n),
    );
    expect(res).to.equal(1815122101n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 4 (1815122101, 1815122097)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(1815122101n),
      this.instances5.alice.encrypt32(1815122097n),
    );
    expect(res).to.equal(1815122097n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 1 (18442735320913618739, 2243204562)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(18442735320913618739n),
      this.instances5.alice.encrypt32(2243204562n),
    );
    expect(res).to.equal(18442735320983123955n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 2 (2243204558, 2243204562)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(2243204558n),
      this.instances5.alice.encrypt32(2243204562n),
    );
    expect(res).to.equal(2243204574n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 3 (2243204562, 2243204562)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(2243204562n),
      this.instances5.alice.encrypt32(2243204562n),
    );
    expect(res).to.equal(2243204562n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 4 (2243204562, 2243204558)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(2243204562n),
      this.instances5.alice.encrypt32(2243204558n),
    );
    expect(res).to.equal(2243204574n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 1 (18444693408203973473, 816979867)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(18444693408203973473n),
      this.instances5.alice.encrypt32(816979867n),
    );
    expect(res).to.equal(18444693407387270394n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 2 (816979863, 816979867)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(816979863n),
      this.instances5.alice.encrypt32(816979867n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 3 (816979867, 816979867)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(816979867n),
      this.instances5.alice.encrypt32(816979867n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 4 (816979867, 816979863)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(816979867n),
      this.instances5.alice.encrypt32(816979863n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 1 (18438001907010629715, 3787224253)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(18438001907010629715n),
      this.instances5.alice.encrypt32(3787224253n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 2 (3787224249, 3787224253)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(3787224249n),
      this.instances5.alice.encrypt32(3787224253n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 3 (3787224253, 3787224253)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(3787224253n),
      this.instances5.alice.encrypt32(3787224253n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 4 (3787224253, 3787224249)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(3787224253n),
      this.instances5.alice.encrypt32(3787224249n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 1 (18443529529517282445, 2935860126)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(18443529529517282445n),
      this.instances5.alice.encrypt32(2935860126n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 2 (2935860122, 2935860126)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(2935860122n),
      this.instances5.alice.encrypt32(2935860126n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 3 (2935860126, 2935860126)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(2935860126n),
      this.instances5.alice.encrypt32(2935860126n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 4 (2935860126, 2935860122)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(2935860126n),
      this.instances5.alice.encrypt32(2935860122n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 1 (18437765775691825677, 2620353033)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(18437765775691825677n),
      this.instances5.alice.encrypt32(2620353033n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 2 (2620353029, 2620353033)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(2620353029n),
      this.instances5.alice.encrypt32(2620353033n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 3 (2620353033, 2620353033)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(2620353033n),
      this.instances5.alice.encrypt32(2620353033n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 4 (2620353033, 2620353029)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(2620353033n),
      this.instances5.alice.encrypt32(2620353029n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 1 (18444733203279770853, 1238323239)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(18444733203279770853n),
      this.instances5.alice.encrypt32(1238323239n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 2 (1238323235, 1238323239)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(1238323235n),
      this.instances5.alice.encrypt32(1238323239n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 3 (1238323239, 1238323239)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(1238323239n),
      this.instances5.alice.encrypt32(1238323239n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 4 (1238323239, 1238323235)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(1238323239n),
      this.instances5.alice.encrypt32(1238323235n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (18442923053218646747, 3072637935)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(18442923053218646747n),
      this.instances5.alice.encrypt32(3072637935n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (3072637931, 3072637935)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(3072637931n),
      this.instances5.alice.encrypt32(3072637935n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (3072637935, 3072637935)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(3072637935n),
      this.instances5.alice.encrypt32(3072637935n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (3072637935, 3072637931)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(3072637935n),
      this.instances5.alice.encrypt32(3072637931n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (18444299937827990993, 3506217873)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(18444299937827990993n),
      this.instances5.alice.encrypt32(3506217873n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (3506217869, 3506217873)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(3506217869n),
      this.instances5.alice.encrypt32(3506217873n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (3506217873, 3506217873)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(3506217873n),
      this.instances5.alice.encrypt32(3506217873n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (3506217873, 3506217869)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(3506217873n),
      this.instances5.alice.encrypt32(3506217869n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (18438338712483817671, 2439887652)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(18438338712483817671n),
      this.instances5.alice.encrypt32(2439887652n),
    );
    expect(res).to.equal(2439887652n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (2439887648, 2439887652)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(2439887648n),
      this.instances5.alice.encrypt32(2439887652n),
    );
    expect(res).to.equal(2439887648n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (2439887652, 2439887652)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(2439887652n),
      this.instances5.alice.encrypt32(2439887652n),
    );
    expect(res).to.equal(2439887652n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (2439887652, 2439887648)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(2439887652n),
      this.instances5.alice.encrypt32(2439887648n),
    );
    expect(res).to.equal(2439887648n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (18438951033743985703, 179529597)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(18438951033743985703n),
      this.instances5.alice.encrypt32(179529597n),
    );
    expect(res).to.equal(18438951033743985703n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (179529593, 179529597)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(179529593n),
      this.instances5.alice.encrypt32(179529597n),
    );
    expect(res).to.equal(179529597n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (179529597, 179529597)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(179529597n),
      this.instances5.alice.encrypt32(179529597n),
    );
    expect(res).to.equal(179529597n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (179529597, 179529593)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(179529597n),
      this.instances5.alice.encrypt32(179529593n),
    );
    expect(res).to.equal(179529597n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (9219782672957507892, 9221460954706528411)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(9219782672957507892n),
      this.instances5.alice.encrypt64(9221460954706528411n),
    );
    expect(res).to.equal(18441243627664036303n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (9219782672957507890, 9219782672957507892)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(9219782672957507890n),
      this.instances5.alice.encrypt64(9219782672957507892n),
    );
    expect(res).to.equal(18439565345915015782n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (9219782672957507892, 9219782672957507892)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(9219782672957507892n),
      this.instances5.alice.encrypt64(9219782672957507892n),
    );
    expect(res).to.equal(18439565345915015784n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (9219782672957507892, 9219782672957507890)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(9219782672957507892n),
      this.instances5.alice.encrypt64(9219782672957507890n),
    );
    expect(res).to.equal(18439565345915015782n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (18441341593220211387, 18441341593220211387)', async function () {
    const res = await this.contract5.sub_euint64_euint64(
      this.instances5.alice.encrypt64(18441341593220211387n),
      this.instances5.alice.encrypt64(18441341593220211387n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (18441341593220211387, 18441341593220211383)', async function () {
    const res = await this.contract5.sub_euint64_euint64(
      this.instances5.alice.encrypt64(18441341593220211387n),
      this.instances5.alice.encrypt64(18441341593220211383n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (4294682660, 4293693962)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(4294682660n),
      this.instances5.alice.encrypt64(4293693962n),
    );
    expect(res).to.equal(18440053005948098920n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (4293693962, 4293693962)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(4293693962n),
      this.instances5.alice.encrypt64(4293693962n),
    );
    expect(res).to.equal(18435807839315257444n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (4293693962, 4293693962)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(4293693962n),
      this.instances5.alice.encrypt64(4293693962n),
    );
    expect(res).to.equal(18435807839315257444n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (4293693962, 4293693962)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(4293693962n),
      this.instances5.alice.encrypt64(4293693962n),
    );
    expect(res).to.equal(18435807839315257444n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (18440531577290780663, 18438982738167832105)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(18440531577290780663n),
      this.instances5.alice.encrypt64(18438982738167832105n),
    );
    expect(res).to.equal(18437856722296507937n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (18438982738167832101, 18438982738167832105)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(18438982738167832101n),
      this.instances5.alice.encrypt64(18438982738167832105n),
    );
    expect(res).to.equal(18438982738167832097n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (18438982738167832105, 18438982738167832105)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(18438982738167832105n),
      this.instances5.alice.encrypt64(18438982738167832105n),
    );
    expect(res).to.equal(18438982738167832105n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (18438982738167832105, 18438982738167832101)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(18438982738167832105n),
      this.instances5.alice.encrypt64(18438982738167832101n),
    );
    expect(res).to.equal(18438982738167832097n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 1 (18442354486709475395, 18441755799323420511)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(18442354486709475395n),
      this.instances5.alice.encrypt64(18441755799323420511n),
    );
    expect(res).to.equal(18446295137944239967n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 2 (18441755799323420507, 18441755799323420511)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(18441755799323420507n),
      this.instances5.alice.encrypt64(18441755799323420511n),
    );
    expect(res).to.equal(18441755799323420511n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 3 (18441755799323420511, 18441755799323420511)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(18441755799323420511n),
      this.instances5.alice.encrypt64(18441755799323420511n),
    );
    expect(res).to.equal(18441755799323420511n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 4 (18441755799323420511, 18441755799323420507)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(18441755799323420511n),
      this.instances5.alice.encrypt64(18441755799323420507n),
    );
    expect(res).to.equal(18441755799323420511n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 1 (18438969927177317847, 18443687297949478061)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(18438969927177317847n),
      this.instances5.alice.encrypt64(18443687297949478061n),
    );
    expect(res).to.equal(4858280629895546n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 2 (18438969927177317843, 18438969927177317847)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(18438969927177317843n),
      this.instances5.alice.encrypt64(18438969927177317847n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 3 (18438969927177317847, 18438969927177317847)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(18438969927177317847n),
      this.instances5.alice.encrypt64(18438969927177317847n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 4 (18438969927177317847, 18438969927177317843)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(18438969927177317847n),
      this.instances5.alice.encrypt64(18438969927177317843n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 1 (18445774084346646289, 18439398756395415043)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(18445774084346646289n),
      this.instances5.alice.encrypt64(18439398756395415043n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 2 (18439398756395415039, 18439398756395415043)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(18439398756395415039n),
      this.instances5.alice.encrypt64(18439398756395415043n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 3 (18439398756395415043, 18439398756395415043)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(18439398756395415043n),
      this.instances5.alice.encrypt64(18439398756395415043n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 4 (18439398756395415043, 18439398756395415039)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(18439398756395415043n),
      this.instances5.alice.encrypt64(18439398756395415039n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (18443560085267937099, 18438958130327077841)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(18443560085267937099n),
      this.instances5.alice.encrypt64(18438958130327077841n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (18438958130327077837, 18438958130327077841)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(18438958130327077837n),
      this.instances5.alice.encrypt64(18438958130327077841n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (18438958130327077841, 18438958130327077841)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(18438958130327077841n),
      this.instances5.alice.encrypt64(18438958130327077841n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (18438958130327077841, 18438958130327077837)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(18438958130327077841n),
      this.instances5.alice.encrypt64(18438958130327077837n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 1 (18440422587099648863, 18442535963906799301)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(18440422587099648863n),
      this.instances5.alice.encrypt64(18442535963906799301n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 2 (18440422587099648859, 18440422587099648863)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(18440422587099648859n),
      this.instances5.alice.encrypt64(18440422587099648863n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 3 (18440422587099648863, 18440422587099648863)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(18440422587099648863n),
      this.instances5.alice.encrypt64(18440422587099648863n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 4 (18440422587099648863, 18440422587099648859)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(18440422587099648863n),
      this.instances5.alice.encrypt64(18440422587099648859n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (18438097221695737185, 18440705423878875443)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(18438097221695737185n),
      this.instances5.alice.encrypt64(18440705423878875443n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (18438097221695737181, 18438097221695737185)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(18438097221695737181n),
      this.instances5.alice.encrypt64(18438097221695737185n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (18438097221695737185, 18438097221695737185)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(18438097221695737185n),
      this.instances5.alice.encrypt64(18438097221695737185n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (18438097221695737185, 18438097221695737181)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(18438097221695737185n),
      this.instances5.alice.encrypt64(18438097221695737181n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (18443440103034891291, 18439639983201094797)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(18443440103034891291n),
      this.instances5.alice.encrypt64(18439639983201094797n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (18439639983201094793, 18439639983201094797)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(18439639983201094793n),
      this.instances5.alice.encrypt64(18439639983201094797n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (18439639983201094797, 18439639983201094797)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(18439639983201094797n),
      this.instances5.alice.encrypt64(18439639983201094797n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (18439639983201094797, 18439639983201094793)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(18439639983201094797n),
      this.instances5.alice.encrypt64(18439639983201094793n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 1 (18444348850784574539, 18444982452771330491)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(18444348850784574539n),
      this.instances5.alice.encrypt64(18444982452771330491n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 2 (18444348850784574535, 18444348850784574539)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(18444348850784574535n),
      this.instances5.alice.encrypt64(18444348850784574539n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 3 (18444348850784574539, 18444348850784574539)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(18444348850784574539n),
      this.instances5.alice.encrypt64(18444348850784574539n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 4 (18444348850784574539, 18444348850784574535)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(18444348850784574539n),
      this.instances5.alice.encrypt64(18444348850784574535n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 1 (18440065418633042573, 18446119246420221527)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(18440065418633042573n),
      this.instances5.alice.encrypt64(18446119246420221527n),
    );
    expect(res).to.equal(18440065418633042573n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 2 (18440065418633042569, 18440065418633042573)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(18440065418633042569n),
      this.instances5.alice.encrypt64(18440065418633042573n),
    );
    expect(res).to.equal(18440065418633042569n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 3 (18440065418633042573, 18440065418633042573)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(18440065418633042573n),
      this.instances5.alice.encrypt64(18440065418633042573n),
    );
    expect(res).to.equal(18440065418633042573n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 4 (18440065418633042573, 18440065418633042569)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(18440065418633042573n),
      this.instances5.alice.encrypt64(18440065418633042569n),
    );
    expect(res).to.equal(18440065418633042569n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (18438041369519163839, 18443250928081110863)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18438041369519163839n),
      this.instances5.alice.encrypt64(18443250928081110863n),
    );
    expect(res).to.equal(18443250928081110863n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (18438041369519163835, 18438041369519163839)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18438041369519163835n),
      this.instances5.alice.encrypt64(18438041369519163839n),
    );
    expect(res).to.equal(18438041369519163839n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (18438041369519163839, 18438041369519163839)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18438041369519163839n),
      this.instances5.alice.encrypt64(18438041369519163839n),
    );
    expect(res).to.equal(18438041369519163839n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (18438041369519163839, 18438041369519163835)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18438041369519163839n),
      this.instances5.alice.encrypt64(18438041369519163835n),
    );
    expect(res).to.equal(18438041369519163839n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (9219782672957507892, 9223276956668541432)', async function () {
    const res = await this.contract5.add_euint64_uint64(
      this.instances5.alice.encrypt64(9219782672957507892n),
      9223276956668541432n,
    );
    expect(res).to.equal(18443059629626049324n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (9219782672957507890, 9219782672957507892)', async function () {
    const res = await this.contract5.add_euint64_uint64(
      this.instances5.alice.encrypt64(9219782672957507890n),
      9219782672957507892n,
    );
    expect(res).to.equal(18439565345915015782n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (9219782672957507892, 9219782672957507892)', async function () {
    const res = await this.contract5.add_euint64_uint64(
      this.instances5.alice.encrypt64(9219782672957507892n),
      9219782672957507892n,
    );
    expect(res).to.equal(18439565345915015784n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (9219782672957507892, 9219782672957507890)', async function () {
    const res = await this.contract5.add_euint64_uint64(
      this.instances5.alice.encrypt64(9219782672957507892n),
      9219782672957507890n,
    );
    expect(res).to.equal(18439565345915015782n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 1 (9221604079573718002, 9223276956668541432)', async function () {
    const res = await this.contract5.add_uint64_euint64(
      9221604079573718002n,
      this.instances5.alice.encrypt64(9223276956668541432n),
    );
    expect(res).to.equal(18444881036242259434n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 2 (9219782672957507890, 9219782672957507892)', async function () {
    const res = await this.contract5.add_uint64_euint64(
      9219782672957507890n,
      this.instances5.alice.encrypt64(9219782672957507892n),
    );
    expect(res).to.equal(18439565345915015782n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 3 (9219782672957507892, 9219782672957507892)', async function () {
    const res = await this.contract5.add_uint64_euint64(
      9219782672957507892n,
      this.instances5.alice.encrypt64(9219782672957507892n),
    );
    expect(res).to.equal(18439565345915015784n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 4 (9219782672957507892, 9219782672957507890)', async function () {
    const res = await this.contract5.add_uint64_euint64(
      9219782672957507892n,
      this.instances5.alice.encrypt64(9219782672957507890n),
    );
    expect(res).to.equal(18439565345915015782n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (18441341593220211387, 18441341593220211387)', async function () {
    const res = await this.contract5.sub_euint64_uint64(
      this.instances5.alice.encrypt64(18441341593220211387n),
      18441341593220211387n,
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (18441341593220211387, 18441341593220211383)', async function () {
    const res = await this.contract5.sub_euint64_uint64(
      this.instances5.alice.encrypt64(18441341593220211387n),
      18441341593220211383n,
    );
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (18441341593220211387, 18441341593220211387)', async function () {
    const res = await this.contract5.sub_uint64_euint64(
      18441341593220211387n,
      this.instances5.alice.encrypt64(18441341593220211387n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (18441341593220211387, 18441341593220211383)', async function () {
    const res = await this.contract5.sub_uint64_euint64(
      18441341593220211387n,
      this.instances5.alice.encrypt64(18441341593220211383n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (4294682660, 4294366726)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(4294682660n), 4294366726n);
    expect(res).to.equal(18442942313833171160n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (4293693962, 4293693962)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(4293693962n), 4293693962n);
    expect(res).to.equal(18435807839315257444n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (4293693962, 4293693962)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(4293693962n), 4293693962n);
    expect(res).to.equal(18435807839315257444n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (4293693962, 4293693962)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(4293693962n), 4293693962n);
    expect(res).to.equal(18435807839315257444n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 1 (4294769578, 4294366726)', async function () {
    const res = await this.contract5.mul_uint64_euint64(4294769578n, this.instances5.alice.encrypt64(4294366726n));
    expect(res).to.equal(18443315571600261628n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 2 (4293693962, 4293693962)', async function () {
    const res = await this.contract5.mul_uint64_euint64(4293693962n, this.instances5.alice.encrypt64(4293693962n));
    expect(res).to.equal(18435807839315257444n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 3 (4293693962, 4293693962)', async function () {
    const res = await this.contract5.mul_uint64_euint64(4293693962n, this.instances5.alice.encrypt64(4293693962n));
    expect(res).to.equal(18435807839315257444n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 4 (4293693962, 4293693962)', async function () {
    const res = await this.contract5.mul_uint64_euint64(4293693962n, this.instances5.alice.encrypt64(4293693962n));
    expect(res).to.equal(18435807839315257444n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (18444029057751200103, 18443244070000667869)', async function () {
    const res = await this.contract5.div_euint64_uint64(
      this.instances5.alice.encrypt64(18444029057751200103n),
      18443244070000667869n,
    );
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (18443602089809486423, 18443602089809486427)', async function () {
    const res = await this.contract5.div_euint64_uint64(
      this.instances5.alice.encrypt64(18443602089809486423n),
      18443602089809486427n,
    );
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (18443602089809486427, 18443602089809486427)', async function () {
    const res = await this.contract5.div_euint64_uint64(
      this.instances5.alice.encrypt64(18443602089809486427n),
      18443602089809486427n,
    );
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (18443602089809486427, 18443602089809486423)', async function () {
    const res = await this.contract5.div_euint64_uint64(
      this.instances5.alice.encrypt64(18443602089809486427n),
      18443602089809486423n,
    );
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 1 (18441483509629816441, 18442745641427984083)', async function () {
    const res = await this.contract5.rem_euint64_uint64(
      this.instances5.alice.encrypt64(18441483509629816441n),
      18442745641427984083n,
    );
    expect(res).to.equal(18441483509629816441n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 2 (18439036027124742149, 18439036027124742153)', async function () {
    const res = await this.contract5.rem_euint64_uint64(
      this.instances5.alice.encrypt64(18439036027124742149n),
      18439036027124742153n,
    );
    expect(res).to.equal(18439036027124742149n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 3 (18439036027124742153, 18439036027124742153)', async function () {
    const res = await this.contract5.rem_euint64_uint64(
      this.instances5.alice.encrypt64(18439036027124742153n),
      18439036027124742153n,
    );
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 4 (18439036027124742153, 18439036027124742149)', async function () {
    const res = await this.contract5.rem_euint64_uint64(
      this.instances5.alice.encrypt64(18439036027124742153n),
      18439036027124742149n,
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (18445774084346646289, 18440029891503033273)', async function () {
    const res = await this.contract5.eq_euint64_uint64(
      this.instances5.alice.encrypt64(18445774084346646289n),
      18440029891503033273n,
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (18439398756395415039, 18439398756395415043)', async function () {
    const res = await this.contract5.eq_euint64_uint64(
      this.instances5.alice.encrypt64(18439398756395415039n),
      18439398756395415043n,
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (18439398756395415043, 18439398756395415043)', async function () {
    const res = await this.contract5.eq_euint64_uint64(
      this.instances5.alice.encrypt64(18439398756395415043n),
      18439398756395415043n,
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (18439398756395415043, 18439398756395415039)', async function () {
    const res = await this.contract5.eq_euint64_uint64(
      this.instances5.alice.encrypt64(18439398756395415043n),
      18439398756395415039n,
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (18444025261552469241, 18440029891503033273)', async function () {
    const res = await this.contract5.eq_uint64_euint64(
      18444025261552469241n,
      this.instances5.alice.encrypt64(18440029891503033273n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (18439398756395415039, 18439398756395415043)', async function () {
    const res = await this.contract5.eq_uint64_euint64(
      18439398756395415039n,
      this.instances5.alice.encrypt64(18439398756395415043n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (18439398756395415043, 18439398756395415043)', async function () {
    const res = await this.contract5.eq_uint64_euint64(
      18439398756395415043n,
      this.instances5.alice.encrypt64(18439398756395415043n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (18439398756395415043, 18439398756395415039)', async function () {
    const res = await this.contract5.eq_uint64_euint64(
      18439398756395415043n,
      this.instances5.alice.encrypt64(18439398756395415039n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (18443560085267937099, 18438802284685869287)', async function () {
    const res = await this.contract5.ne_euint64_uint64(
      this.instances5.alice.encrypt64(18443560085267937099n),
      18438802284685869287n,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (18438958130327077837, 18438958130327077841)', async function () {
    const res = await this.contract5.ne_euint64_uint64(
      this.instances5.alice.encrypt64(18438958130327077837n),
      18438958130327077841n,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (18438958130327077841, 18438958130327077841)', async function () {
    const res = await this.contract5.ne_euint64_uint64(
      this.instances5.alice.encrypt64(18438958130327077841n),
      18438958130327077841n,
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (18438958130327077841, 18438958130327077837)', async function () {
    const res = await this.contract5.ne_euint64_uint64(
      this.instances5.alice.encrypt64(18438958130327077841n),
      18438958130327077837n,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (18445997854105153061, 18438802284685869287)', async function () {
    const res = await this.contract5.ne_uint64_euint64(
      18445997854105153061n,
      this.instances5.alice.encrypt64(18438802284685869287n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (18438958130327077837, 18438958130327077841)', async function () {
    const res = await this.contract5.ne_uint64_euint64(
      18438958130327077837n,
      this.instances5.alice.encrypt64(18438958130327077841n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (18438958130327077841, 18438958130327077841)', async function () {
    const res = await this.contract5.ne_uint64_euint64(
      18438958130327077841n,
      this.instances5.alice.encrypt64(18438958130327077841n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (18438958130327077841, 18438958130327077837)', async function () {
    const res = await this.contract5.ne_uint64_euint64(
      18438958130327077841n,
      this.instances5.alice.encrypt64(18438958130327077837n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (18440422587099648863, 18441247043482344217)', async function () {
    const res = await this.contract5.ge_euint64_uint64(
      this.instances5.alice.encrypt64(18440422587099648863n),
      18441247043482344217n,
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (18440422587099648859, 18440422587099648863)', async function () {
    const res = await this.contract5.ge_euint64_uint64(
      this.instances5.alice.encrypt64(18440422587099648859n),
      18440422587099648863n,
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (18440422587099648863, 18440422587099648863)', async function () {
    const res = await this.contract5.ge_euint64_uint64(
      this.instances5.alice.encrypt64(18440422587099648863n),
      18440422587099648863n,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (18440422587099648863, 18440422587099648859)', async function () {
    const res = await this.contract5.ge_euint64_uint64(
      this.instances5.alice.encrypt64(18440422587099648863n),
      18440422587099648859n,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (18440419934786413523, 18441247043482344217)', async function () {
    const res = await this.contract5.ge_uint64_euint64(
      18440419934786413523n,
      this.instances5.alice.encrypt64(18441247043482344217n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (18440422587099648859, 18440422587099648863)', async function () {
    const res = await this.contract5.ge_uint64_euint64(
      18440422587099648859n,
      this.instances5.alice.encrypt64(18440422587099648863n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (18440422587099648863, 18440422587099648863)', async function () {
    const res = await this.contract5.ge_uint64_euint64(
      18440422587099648863n,
      this.instances5.alice.encrypt64(18440422587099648863n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (18440422587099648863, 18440422587099648859)', async function () {
    const res = await this.contract5.ge_uint64_euint64(
      18440422587099648863n,
      this.instances5.alice.encrypt64(18440422587099648859n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (18438097221695737185, 18446155734799970265)', async function () {
    const res = await this.contract5.gt_euint64_uint64(
      this.instances5.alice.encrypt64(18438097221695737185n),
      18446155734799970265n,
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (18438097221695737181, 18438097221695737185)', async function () {
    const res = await this.contract5.gt_euint64_uint64(
      this.instances5.alice.encrypt64(18438097221695737181n),
      18438097221695737185n,
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (18438097221695737185, 18438097221695737185)', async function () {
    const res = await this.contract5.gt_euint64_uint64(
      this.instances5.alice.encrypt64(18438097221695737185n),
      18438097221695737185n,
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (18438097221695737185, 18438097221695737181)', async function () {
    const res = await this.contract5.gt_euint64_uint64(
      this.instances5.alice.encrypt64(18438097221695737185n),
      18438097221695737181n,
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 1 (18445790806120595617, 18446155734799970265)', async function () {
    const res = await this.contract5.gt_uint64_euint64(
      18445790806120595617n,
      this.instances5.alice.encrypt64(18446155734799970265n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 2 (18438097221695737181, 18438097221695737185)', async function () {
    const res = await this.contract5.gt_uint64_euint64(
      18438097221695737181n,
      this.instances5.alice.encrypt64(18438097221695737185n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 3 (18438097221695737185, 18438097221695737185)', async function () {
    const res = await this.contract5.gt_uint64_euint64(
      18438097221695737185n,
      this.instances5.alice.encrypt64(18438097221695737185n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 4 (18438097221695737185, 18438097221695737181)', async function () {
    const res = await this.contract5.gt_uint64_euint64(
      18438097221695737185n,
      this.instances5.alice.encrypt64(18438097221695737181n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (18443440103034891291, 18446381446809506033)', async function () {
    const res = await this.contract5.le_euint64_uint64(
      this.instances5.alice.encrypt64(18443440103034891291n),
      18446381446809506033n,
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (18439639983201094793, 18439639983201094797)', async function () {
    const res = await this.contract5.le_euint64_uint64(
      this.instances5.alice.encrypt64(18439639983201094793n),
      18439639983201094797n,
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (18439639983201094797, 18439639983201094797)', async function () {
    const res = await this.contract5.le_euint64_uint64(
      this.instances5.alice.encrypt64(18439639983201094797n),
      18439639983201094797n,
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (18439639983201094797, 18439639983201094793)', async function () {
    const res = await this.contract5.le_euint64_uint64(
      this.instances5.alice.encrypt64(18439639983201094797n),
      18439639983201094793n,
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (18442636031210964339, 18446381446809506033)', async function () {
    const res = await this.contract5.le_uint64_euint64(
      18442636031210964339n,
      this.instances5.alice.encrypt64(18446381446809506033n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (18439639983201094793, 18439639983201094797)', async function () {
    const res = await this.contract5.le_uint64_euint64(
      18439639983201094793n,
      this.instances5.alice.encrypt64(18439639983201094797n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (18439639983201094797, 18439639983201094797)', async function () {
    const res = await this.contract5.le_uint64_euint64(
      18439639983201094797n,
      this.instances5.alice.encrypt64(18439639983201094797n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (18439639983201094797, 18439639983201094793)', async function () {
    const res = await this.contract5.le_uint64_euint64(
      18439639983201094797n,
      this.instances5.alice.encrypt64(18439639983201094793n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (18444348850784574539, 18440820311074541547)', async function () {
    const res = await this.contract5.lt_euint64_uint64(
      this.instances5.alice.encrypt64(18444348850784574539n),
      18440820311074541547n,
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (18444348850784574535, 18444348850784574539)', async function () {
    const res = await this.contract5.lt_euint64_uint64(
      this.instances5.alice.encrypt64(18444348850784574535n),
      18444348850784574539n,
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (18444348850784574539, 18444348850784574539)', async function () {
    const res = await this.contract5.lt_euint64_uint64(
      this.instances5.alice.encrypt64(18444348850784574539n),
      18444348850784574539n,
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (18444348850784574539, 18444348850784574535)', async function () {
    const res = await this.contract5.lt_euint64_uint64(
      this.instances5.alice.encrypt64(18444348850784574539n),
      18444348850784574535n,
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (18439237911505883611, 18440820311074541547)', async function () {
    const res = await this.contract5.lt_uint64_euint64(
      18439237911505883611n,
      this.instances5.alice.encrypt64(18440820311074541547n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (18444348850784574535, 18444348850784574539)', async function () {
    const res = await this.contract5.lt_uint64_euint64(
      18444348850784574535n,
      this.instances5.alice.encrypt64(18444348850784574539n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (18444348850784574539, 18444348850784574539)', async function () {
    const res = await this.contract5.lt_uint64_euint64(
      18444348850784574539n,
      this.instances5.alice.encrypt64(18444348850784574539n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (18444348850784574539, 18444348850784574535)', async function () {
    const res = await this.contract5.lt_uint64_euint64(
      18444348850784574539n,
      this.instances5.alice.encrypt64(18444348850784574535n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 1 (18440065418633042573, 18438575790803191203)', async function () {
    const res = await this.contract5.min_euint64_uint64(
      this.instances5.alice.encrypt64(18440065418633042573n),
      18438575790803191203n,
    );
    expect(res).to.equal(18438575790803191203n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 2 (18440065418633042569, 18440065418633042573)', async function () {
    const res = await this.contract5.min_euint64_uint64(
      this.instances5.alice.encrypt64(18440065418633042569n),
      18440065418633042573n,
    );
    expect(res).to.equal(18440065418633042569n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 3 (18440065418633042573, 18440065418633042573)', async function () {
    const res = await this.contract5.min_euint64_uint64(
      this.instances5.alice.encrypt64(18440065418633042573n),
      18440065418633042573n,
    );
    expect(res).to.equal(18440065418633042573n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 4 (18440065418633042573, 18440065418633042569)', async function () {
    const res = await this.contract5.min_euint64_uint64(
      this.instances5.alice.encrypt64(18440065418633042573n),
      18440065418633042569n,
    );
    expect(res).to.equal(18440065418633042569n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (18438233715164129421, 18438575790803191203)', async function () {
    const res = await this.contract5.min_uint64_euint64(
      18438233715164129421n,
      this.instances5.alice.encrypt64(18438575790803191203n),
    );
    expect(res).to.equal(18438233715164129421n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (18440065418633042569, 18440065418633042573)', async function () {
    const res = await this.contract5.min_uint64_euint64(
      18440065418633042569n,
      this.instances5.alice.encrypt64(18440065418633042573n),
    );
    expect(res).to.equal(18440065418633042569n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (18440065418633042573, 18440065418633042573)', async function () {
    const res = await this.contract5.min_uint64_euint64(
      18440065418633042573n,
      this.instances5.alice.encrypt64(18440065418633042573n),
    );
    expect(res).to.equal(18440065418633042573n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (18440065418633042573, 18440065418633042569)', async function () {
    const res = await this.contract5.min_uint64_euint64(
      18440065418633042573n,
      this.instances5.alice.encrypt64(18440065418633042569n),
    );
    expect(res).to.equal(18440065418633042569n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (18438041369519163839, 18446420001661732973)', async function () {
    const res = await this.contract5.max_euint64_uint64(
      this.instances5.alice.encrypt64(18438041369519163839n),
      18446420001661732973n,
    );
    expect(res).to.equal(18446420001661732973n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (18438041369519163835, 18438041369519163839)', async function () {
    const res = await this.contract5.max_euint64_uint64(
      this.instances5.alice.encrypt64(18438041369519163835n),
      18438041369519163839n,
    );
    expect(res).to.equal(18438041369519163839n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (18438041369519163839, 18438041369519163839)', async function () {
    const res = await this.contract5.max_euint64_uint64(
      this.instances5.alice.encrypt64(18438041369519163839n),
      18438041369519163839n,
    );
    expect(res).to.equal(18438041369519163839n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (18438041369519163839, 18438041369519163835)', async function () {
    const res = await this.contract5.max_euint64_uint64(
      this.instances5.alice.encrypt64(18438041369519163839n),
      18438041369519163835n,
    );
    expect(res).to.equal(18438041369519163839n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (18439939633614308253, 18446420001661732973)', async function () {
    const res = await this.contract5.max_uint64_euint64(
      18439939633614308253n,
      this.instances5.alice.encrypt64(18446420001661732973n),
    );
    expect(res).to.equal(18446420001661732973n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18438041369519163835, 18438041369519163839)', async function () {
    const res = await this.contract5.max_uint64_euint64(
      18438041369519163835n,
      this.instances5.alice.encrypt64(18438041369519163839n),
    );
    expect(res).to.equal(18438041369519163839n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18438041369519163839, 18438041369519163839)', async function () {
    const res = await this.contract5.max_uint64_euint64(
      18438041369519163839n,
      this.instances5.alice.encrypt64(18438041369519163839n),
    );
    expect(res).to.equal(18438041369519163839n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18438041369519163839, 18438041369519163835)', async function () {
    const res = await this.contract5.max_uint64_euint64(
      18438041369519163839n,
      this.instances5.alice.encrypt64(18438041369519163835n),
    );
    expect(res).to.equal(18438041369519163839n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 1 (1, 5)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(1n), 5n);
    expect(res).to.equal(2n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(4n), 8n);
    expect(res).to.equal(4n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(8n), 8n);
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(8n), 4n);
    expect(res).to.equal(8n);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 1 (3, 7)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(3n), 7n);
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(4n), 8n);
    expect(res).to.equal(4n);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(8n), 8n);
    expect(res).to.equal(8n);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(8n), 4n);
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (190, 3)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(190n),
      this.instances5.alice.encrypt8(3n),
    );
    expect(res).to.equal(240n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (190, 3)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(190n), 3n);
    expect(res).to.equal(240n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(4n), 8n);
    expect(res).to.equal(4n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(8n), 8n);
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(8n), 4n);
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (141, 6)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(141n),
      this.instances5.alice.encrypt8(6n),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (141, 6)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(141n), 6n);
    expect(res).to.equal(2n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(4n), 8n);
    expect(res).to.equal(4n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(8n), 8n);
    expect(res).to.equal(8n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(8n), 4n);
    expect(res).to.equal(0n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (23171, 3)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(23171n),
      this.instances5.alice.encrypt8(3n),
    );
    expect(res).to.equal(54296n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 1 (23171, 3)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(23171n), 3n);
    expect(res).to.equal(54296n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(4n), 8n);
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(8n), 8n);
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(8n), 4n);
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (19736, 3)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(19736n),
      this.instances5.alice.encrypt8(3n),
    );
    expect(res).to.equal(2467n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (19736, 3)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(19736n), 3n);
    expect(res).to.equal(2467n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(4n), 8n);
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(8n), 8n);
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(8n), 4n);
    expect(res).to.equal(0n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (3941404411, 2)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(3941404411n),
      this.instances5.alice.encrypt8(2n),
    );
    expect(res).to.equal(2880715756n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 1 (3941404411, 2)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(3941404411n), 2n);
    expect(res).to.equal(2880715756n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(4n), 8n);
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(8n), 8n);
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(8n), 4n);
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (643205149, 7)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(643205149n),
      this.instances5.alice.encrypt8(7n),
    );
    expect(res).to.equal(5025040n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 1 (643205149, 7)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(643205149n), 7n);
    expect(res).to.equal(5025040n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(4n), 8n);
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(8n), 8n);
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(8n), 4n);
    expect(res).to.equal(0n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 1 (18443917851207927997, 3)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(18443917851207927997n),
      this.instances5.alice.encrypt8(3n),
    );
    expect(res).to.equal(18424134293696562664n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (18443917851207927997, 3)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(18443917851207927997n), 3n);
    expect(res).to.equal(18424134293696562664n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(4n), 8n);
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(8n), 8n);
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(8n), 4n);
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (18439818227985179917, 5)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(18439818227985179917n),
      this.instances5.alice.encrypt8(5n),
    );
    expect(res).to.equal(576244319624536872n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (18439818227985179917, 5)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(18439818227985179917n), 5n);
    expect(res).to.equal(576244319624536872n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(4n), 8n);
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(8n), 8n);
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(8n), 4n);
    expect(res).to.equal(0n);
  });

  it('test operator "neg" overload (euint4) => euint4 test 1 (3)', async function () {
    const res = await this.contract5.neg_euint4(this.instances5.alice.encrypt4(3n));
    expect(res).to.equal(13n);
  });

  it('test operator "not" overload (euint4) => euint4 test 1 (3)', async function () {
    const res = await this.contract5.not_euint4(this.instances5.alice.encrypt4(3n));
    expect(res).to.equal(12n);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (62)', async function () {
    const res = await this.contract5.neg_euint8(this.instances5.alice.encrypt8(62n));
    expect(res).to.equal(194n);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (236)', async function () {
    const res = await this.contract5.not_euint8(this.instances5.alice.encrypt8(236n));
    expect(res).to.equal(19n);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (47549)', async function () {
    const res = await this.contract5.neg_euint16(this.instances5.alice.encrypt16(47549n));
    expect(res).to.equal(17987n);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (38757)', async function () {
    const res = await this.contract5.not_euint16(this.instances5.alice.encrypt16(38757n));
    expect(res).to.equal(26778n);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (4149698786)', async function () {
    const res = await this.contract5.neg_euint32(this.instances5.alice.encrypt32(4149698786n));
    expect(res).to.equal(145268510n);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (1521520195)', async function () {
    const res = await this.contract5.not_euint32(this.instances5.alice.encrypt32(1521520195n));
    expect(res).to.equal(2773447100n);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (18444493835715762809)', async function () {
    const res = await this.contract5.neg_euint64(this.instances5.alice.encrypt64(18444493835715762809n));
    expect(res).to.equal(2250237993788807n);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (18443718942921088825)', async function () {
    const res = await this.contract5.not_euint64(this.instances5.alice.encrypt64(18443718942921088825n));
    expect(res).to.equal(3025130788462790n);
  });
});
