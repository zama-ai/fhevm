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
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt4(1n),
    );
    expect(res).to.equal(12n);
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

  it('test operator "mul" overload (euint4, euint4) => euint4 test 1 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt4(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 2 (3, 4)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt4(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 4 (4, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint4(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt4(3n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 1 (5, 2)', async function () {
    const res = await this.contract1.and_euint4_euint4(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt4(2n),
    );
    expect(res).to.equal(0n);
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

  it('test operator "or" overload (euint4, euint4) => euint4 test 1 (6, 5)', async function () {
    const res = await this.contract1.or_euint4_euint4(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt4(5n),
    );
    expect(res).to.equal(7n);
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

  it('test operator "xor" overload (euint4, euint4) => euint4 test 1 (2, 10)', async function () {
    const res = await this.contract1.xor_euint4_euint4(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt4(10n),
    );
    expect(res).to.equal(8n);
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

  it('test operator "eq" overload (euint4, euint4) => ebool test 1 (8, 2)', async function () {
    const res = await this.contract1.eq_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(2n),
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

  it('test operator "ne" overload (euint4, euint4) => ebool test 1 (11, 12)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt4(12n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt4(11n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt4(11n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.ne_euint4_euint4(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt4(7n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 1 (3, 4)', async function () {
    const res = await this.contract1.ge_euint4_euint4(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt4(4n),
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

  it('test operator "gt" overload (euint4, euint4) => ebool test 1 (9, 12)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt4(12n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt4(9n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt4(9n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.gt_euint4_euint4(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt4(5n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 1 (8, 6)', async function () {
    const res = await this.contract1.le_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(6n),
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

  it('test operator "lt" overload (euint4, euint4) => ebool test 1 (10, 9)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt4(9n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt4(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt4(9n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.lt_euint4_euint4(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt4(5n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 1 (6, 5)', async function () {
    const res = await this.contract1.min_euint4_euint4(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt4(5n),
    );
    expect(res).to.equal(5n);
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

  it('test operator "max" overload (euint4, euint4) => euint4 test 1 (8, 3)', async function () {
    const res = await this.contract1.max_euint4_euint4(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt4(3n),
    );
    expect(res).to.equal(8n);
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

  it('test operator "add" overload (euint4, euint8) => euint8 test 1 (2, 12)', async function () {
    const res = await this.contract1.add_euint4_euint8(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt8(12n),
    );
    expect(res).to.equal(14n);
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

  it('test operator "mul" overload (euint4, euint8) => euint8 test 1 (1, 14)', async function () {
    const res = await this.contract1.mul_euint4_euint8(
      this.instances1.alice.encrypt4(1n),
      this.instances1.alice.encrypt8(14n),
    );
    expect(res).to.equal(14n);
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

  it('test operator "and" overload (euint4, euint8) => euint8 test 1 (12, 110)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt8(110n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 2 (8, 12)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(12n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 3 (12, 12)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt8(12n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 4 (12, 8)', async function () {
    const res = await this.contract1.and_euint4_euint8(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 1 (14, 110)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt8(110n),
    );
    expect(res).to.equal(110n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 2 (10, 14)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt8(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 3 (14, 14)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt8(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 4 (14, 10)', async function () {
    const res = await this.contract1.or_euint4_euint8(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt8(10n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 1 (12, 244)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt8(244n),
    );
    expect(res).to.equal(248n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 2 (8, 12)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(12n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 3 (12, 12)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt8(12n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 4 (12, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint8(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 1 (14, 180)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt8(180n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt8(14n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt8(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.eq_euint4_euint8(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt8(10n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 1 (7, 94)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt8(94n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ne_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 1 (12, 141)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt8(141n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt8(12n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt8(12n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract1.ge_euint4_euint8(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt8(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 1 (7, 49)', async function () {
    const res = await this.contract1.gt_euint4_euint8(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt8(49n),
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

  it('test operator "le" overload (euint4, euint8) => ebool test 1 (14, 151)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt8(151n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt8(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt8(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.le_euint4_euint8(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt8(10n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 1 (7, 92)', async function () {
    const res = await this.contract1.lt_euint4_euint8(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt8(92n),
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

  it('test operator "min" overload (euint4, euint8) => euint8 test 1 (11, 237)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt8(237n),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 2 (7, 11)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt8(11n),
    );
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 3 (11, 11)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt8(11n),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 4 (11, 7)', async function () {
    const res = await this.contract1.min_euint4_euint8(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt8(7n),
    );
    expect(res).to.equal(7n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 1 (11, 181)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt8(181n),
    );
    expect(res).to.equal(181n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 2 (7, 11)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt8(11n),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 3 (11, 11)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt8(11n),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 4 (11, 7)', async function () {
    const res = await this.contract1.max_euint4_euint8(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt8(7n),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 1 (2, 9)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt16(9n),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 2 (6, 8)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt16(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 4 (8, 6)', async function () {
    const res = await this.contract1.add_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(6n),
    );
    expect(res).to.equal(14n);
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

  it('test operator "mul" overload (euint4, euint16) => euint16 test 1 (2, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt16(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt16(5n),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt16(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint16(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt16(3n),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 1 (10, 16702)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt16(16702n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 2 (6, 10)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt16(10n),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 3 (10, 10)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt16(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 4 (10, 6)', async function () {
    const res = await this.contract1.and_euint4_euint16(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt16(6n),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 1 (3, 60460)', async function () {
    const res = await this.contract1.or_euint4_euint16(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt16(60460n),
    );
    expect(res).to.equal(60463n);
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

  it('test operator "xor" overload (euint4, euint16) => euint16 test 1 (6, 58722)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt16(58722n),
    );
    expect(res).to.equal(58724n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract1.xor_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 1 (9, 7831)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt16(7831n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt16(9n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt16(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.eq_euint4_euint16(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt16(5n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 1 (2, 63787)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt16(63787n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ne_euint4_euint16(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt16(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 1 (9, 63554)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt16(63554n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt16(9n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt16(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.ge_euint4_euint16(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt16(5n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 1 (13, 28270)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(13n),
      this.instances1.alice.encrypt16(28270n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt16(13n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(13n),
      this.instances1.alice.encrypt16(13n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract1.gt_euint4_euint16(
      this.instances1.alice.encrypt4(13n),
      this.instances1.alice.encrypt16(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 1 (1, 51341)', async function () {
    const res = await this.contract1.le_euint4_euint16(
      this.instances1.alice.encrypt4(1n),
      this.instances1.alice.encrypt16(51341n),
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

  it('test operator "lt" overload (euint4, euint16) => ebool test 1 (6, 15496)', async function () {
    const res = await this.contract1.lt_euint4_euint16(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt16(15496n),
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

  it('test operator "min" overload (euint4, euint16) => euint16 test 1 (10, 26263)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt16(26263n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 2 (6, 10)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt16(10n),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 3 (10, 10)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt16(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 4 (10, 6)', async function () {
    const res = await this.contract1.min_euint4_euint16(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt16(6n),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 1 (2, 61770)', async function () {
    const res = await this.contract1.max_euint4_euint16(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt16(61770n),
    );
    expect(res).to.equal(61770n);
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

  it('test operator "add" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt32(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(4n),
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

  it('test operator "mul" overload (euint4, euint32) => euint32 test 1 (2, 7)', async function () {
    const res = await this.contract1.mul_euint4_euint32(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt32(7n),
    );
    expect(res).to.equal(14n);
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

  it('test operator "and" overload (euint4, euint32) => euint32 test 1 (6, 3534750602)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt32(3534750602n),
    );
    expect(res).to.equal(2n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(4n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 1 (3, 2840015564)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt32(2840015564n),
    );
    expect(res).to.equal(2840015567n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 1 (6, 2183122437)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt32(2183122437n),
    );
    expect(res).to.equal(2183122435n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.xor_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 1 (6, 2902824357)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt32(2902824357n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 1 (13, 3287915476)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(13n),
      this.instances1.alice.encrypt32(3287915476n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt32(13n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(13n),
      this.instances1.alice.encrypt32(13n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract1.ne_euint4_euint32(
      this.instances1.alice.encrypt4(13n),
      this.instances1.alice.encrypt32(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 1 (9, 3023026099)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt32(3023026099n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt32(9n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt32(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.ge_euint4_euint32(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt32(5n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 1 (10, 2302702329)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt32(2302702329n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt32(10n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt32(10n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract1.gt_euint4_euint32(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt32(6n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 1 (9, 467708048)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt32(467708048n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt32(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt32(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract1.le_euint4_euint32(
      this.instances1.alice.encrypt4(9n),
      this.instances1.alice.encrypt32(5n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 1 (11, 1378961804)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt32(1378961804n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt32(11n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt32(11n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.lt_euint4_euint32(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt32(7n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 1 (5, 2670542696)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt32(2670542696n),
    );
    expect(res).to.equal(5n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_euint32(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt32(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 1 (2, 1031795645)', async function () {
    const res = await this.contract1.max_euint4_euint32(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt32(1031795645n),
    );
    expect(res).to.equal(1031795645n);
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

  it('test operator "add" overload (euint4, euint64) => euint64 test 1 (2, 9)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(2n),
      this.instances1.alice.encrypt64(9n),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 2 (4, 6)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt64(6n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 3 (6, 6)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt64(6n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 4 (6, 4)', async function () {
    const res = await this.contract1.add_euint4_euint64(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt64(4n),
    );
    expect(res).to.equal(10n);
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

  it('test operator "mul" overload (euint4, euint64) => euint64 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt64(5n),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt64(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_euint64(
      this.instances1.alice.encrypt4(5n),
      this.instances1.alice.encrypt64(3n),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 1 (7, 18444452142912700231)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt64(18444452142912700231n),
    );
    expect(res).to.equal(7n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(4n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 1 (10, 18439589984533574615)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt64(18439589984533574615n),
    );
    expect(res).to.equal(18439589984533574623n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 2 (6, 10)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt64(10n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 3 (10, 10)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt64(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 4 (10, 6)', async function () {
    const res = await this.contract1.or_euint4_euint64(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt64(6n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 1 (7, 18438536745518625067)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt64(18438536745518625067n),
    );
    expect(res).to.equal(18438536745518625068n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.xor_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 1 (4, 18441783865246079825)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt64(18441783865246079825n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 1 (11, 18442031616327904827)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt64(18442031616327904827n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(7n),
      this.instances1.alice.encrypt64(11n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt64(11n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract1.ne_euint4_euint64(
      this.instances1.alice.encrypt4(11n),
      this.instances1.alice.encrypt64(7n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 1 (14, 18441505983320830973)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt64(18441505983320830973n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(10n),
      this.instances1.alice.encrypt64(14n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt64(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.ge_euint4_euint64(
      this.instances1.alice.encrypt4(14n),
      this.instances1.alice.encrypt64(10n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 1 (8, 18443962302044821049)', async function () {
    const res = await this.contract1.gt_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(18443962302044821049n),
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

  it('test operator "le" overload (euint4, euint64) => ebool test 1 (12, 18438657387678135029)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt64(18438657387678135029n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(12n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt64(12n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract1.le_euint4_euint64(
      this.instances1.alice.encrypt4(12n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 1 (8, 18442139041620940861)', async function () {
    const res = await this.contract1.lt_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(18442139041620940861n),
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

  it('test operator "min" overload (euint4, euint64) => euint64 test 1 (6, 18437937380503493287)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(6n),
      this.instances1.alice.encrypt64(18437937380503493287n),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(4n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract1.min_euint4_euint64(
      this.instances1.alice.encrypt4(8n),
      this.instances1.alice.encrypt64(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 1 (3, 18443395042624107977)', async function () {
    const res = await this.contract1.max_euint4_euint64(
      this.instances1.alice.encrypt4(3n),
      this.instances1.alice.encrypt64(18443395042624107977n),
    );
    expect(res).to.equal(18443395042624107977n);
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

  it('test operator "add" overload (euint4, uint8) => euint4 test 1 (7, 7)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(7n), 7);
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(4n), 8);
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(5n), 5);
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_euint4_uint8(this.instances1.alice.encrypt4(8n), 4);
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 1 (12, 3)', async function () {
    const res = await this.contract1.add_uint8_euint4(12, this.instances1.alice.encrypt4(3n));
    expect(res).to.equal(15n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.add_uint8_euint4(4, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 3 (5, 5)', async function () {
    const res = await this.contract1.add_uint8_euint4(5, this.instances1.alice.encrypt4(5n));
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.add_uint8_euint4(8, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, uint8) => euint4 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint4_uint8(this.instances1.alice.encrypt4(8n), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, uint8) => euint4 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint4_uint8(this.instances1.alice.encrypt4(8n), 4);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint8, euint4) => euint4 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_uint8_euint4(8, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint8, euint4) => euint4 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_uint8_euint4(8, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 1 (1, 10)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(1n), 10);
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 2 (3, 5)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(3n), 5);
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(3n), 3);
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 4 (5, 3)', async function () {
    const res = await this.contract1.mul_euint4_uint8(this.instances1.alice.encrypt4(5n), 3);
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 1 (1, 10)', async function () {
    const res = await this.contract1.mul_uint8_euint4(1, this.instances1.alice.encrypt4(10n));
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 2 (3, 4)', async function () {
    const res = await this.contract1.mul_uint8_euint4(3, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_uint8_euint4(3, this.instances1.alice.encrypt4(3n));
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 4 (4, 3)', async function () {
    const res = await this.contract1.mul_uint8_euint4(4, this.instances1.alice.encrypt4(3n));
    expect(res).to.equal(12n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 1 (3, 6)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(3n), 6);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(4n), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(8n), 8);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.div_euint4_uint8(this.instances1.alice.encrypt4(8n), 4);
    expect(res).to.equal(2n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 1 (4, 10)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(4n), 10);
    expect(res).to.equal(4n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(4n), 8);
    expect(res).to.equal(4n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(8n), 8);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract1.rem_euint4_uint8(this.instances1.alice.encrypt4(8n), 4);
    expect(res).to.equal(0n);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 1 (14, 4)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(14n), 4);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(10n), 14);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(14n), 14);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.eq_euint4_uint8(this.instances1.alice.encrypt4(14n), 10);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 1 (11, 1)', async function () {
    const res = await this.contract1.eq_uint8_euint4(11, this.instances1.alice.encrypt4(1n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.eq_uint8_euint4(4, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.eq_uint8_euint4(8, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.eq_uint8_euint4(8, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 1 (7, 12)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(7n), 12);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(4n), 8);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(8n), 8);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ne_euint4_uint8(this.instances1.alice.encrypt4(8n), 4);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 1 (3, 1)', async function () {
    const res = await this.contract1.ne_uint8_euint4(3, this.instances1.alice.encrypt4(1n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ne_uint8_euint4(4, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ne_uint8_euint4(8, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ne_uint8_euint4(8, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 1 (12, 11)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(12n), 11);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(8n), 12);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(12n), 12);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract1.ge_euint4_uint8(this.instances1.alice.encrypt4(12n), 8);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 1 (2, 4)', async function () {
    const res = await this.contract1.ge_uint8_euint4(2, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.ge_uint8_euint4(4, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.ge_uint8_euint4(8, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.ge_uint8_euint4(8, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 1 (7, 10)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(7n), 10);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(4n), 8);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(8n), 8);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.gt_euint4_uint8(this.instances1.alice.encrypt4(8n), 4);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 1 (4, 14)', async function () {
    const res = await this.contract1.gt_uint8_euint4(4, this.instances1.alice.encrypt4(14n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.gt_uint8_euint4(10, this.instances1.alice.encrypt4(14n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.gt_uint8_euint4(14, this.instances1.alice.encrypt4(14n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.gt_uint8_euint4(14, this.instances1.alice.encrypt4(10n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 1 (14, 8)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(14n), 8);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(10n), 14);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(14n), 14);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract1.le_euint4_uint8(this.instances1.alice.encrypt4(14n), 10);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 1 (11, 8)', async function () {
    const res = await this.contract1.le_uint8_euint4(11, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.le_uint8_euint4(4, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.le_uint8_euint4(8, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.le_uint8_euint4(8, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 1 (7, 14)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(7n), 14);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(4n), 8);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(8n), 8);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_euint4_uint8(this.instances1.alice.encrypt4(8n), 4);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 1 (12, 5)', async function () {
    const res = await this.contract1.lt_uint8_euint4(12, this.instances1.alice.encrypt4(5n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract1.lt_uint8_euint4(4, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract1.lt_uint8_euint4(8, this.instances1.alice.encrypt4(8n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract1.lt_uint8_euint4(8, this.instances1.alice.encrypt4(4n));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 1 (11, 10)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(11n), 10);
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 2 (7, 11)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(7n), 11);
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 3 (11, 11)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(11n), 11);
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 4 (11, 7)', async function () {
    const res = await this.contract1.min_euint4_uint8(this.instances1.alice.encrypt4(11n), 7);
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 1 (8, 11)', async function () {
    const res = await this.contract1.min_uint8_euint4(8, this.instances1.alice.encrypt4(11n));
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 2 (7, 11)', async function () {
    const res = await this.contract1.min_uint8_euint4(7, this.instances1.alice.encrypt4(11n));
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 3 (11, 11)', async function () {
    const res = await this.contract1.min_uint8_euint4(11, this.instances1.alice.encrypt4(11n));
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 4 (11, 7)', async function () {
    const res = await this.contract1.min_uint8_euint4(11, this.instances1.alice.encrypt4(7n));
    expect(res).to.equal(7n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 1 (11, 7)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(11n), 7);
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 2 (7, 11)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(7n), 11);
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 3 (11, 11)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(11n), 11);
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 4 (11, 7)', async function () {
    const res = await this.contract1.max_euint4_uint8(this.instances1.alice.encrypt4(11n), 7);
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 1 (7, 14)', async function () {
    const res = await this.contract1.max_uint8_euint4(7, this.instances1.alice.encrypt4(14n));
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 2 (10, 14)', async function () {
    const res = await this.contract1.max_uint8_euint4(10, this.instances1.alice.encrypt4(14n));
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 3 (14, 14)', async function () {
    const res = await this.contract1.max_uint8_euint4(14, this.instances1.alice.encrypt4(14n));
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 4 (14, 10)', async function () {
    const res = await this.contract1.max_uint8_euint4(14, this.instances1.alice.encrypt4(10n));
    expect(res).to.equal(14n);
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

  it('test operator "sub" overload (euint8, euint4) => euint8 test 1 (8, 8)', async function () {
    const res = await this.contract1.sub_euint8_euint4(
      this.instances1.alice.encrypt8(8n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint4) => euint8 test 2 (8, 4)', async function () {
    const res = await this.contract1.sub_euint8_euint4(
      this.instances1.alice.encrypt8(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 1 (5, 2)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(5n),
      this.instances1.alice.encrypt4(2n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 2 (3, 4)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(3n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 3 (3, 3)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(3n),
      this.instances1.alice.encrypt4(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 4 (4, 3)', async function () {
    const res = await this.contract1.mul_euint8_euint4(
      this.instances1.alice.encrypt8(4n),
      this.instances1.alice.encrypt4(3n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 1 (143, 6)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(143n),
      this.instances1.alice.encrypt4(6n),
    );
    expect(res).to.equal(6n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(4n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(8n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.and_euint8_euint4(
      this.instances1.alice.encrypt8(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 1 (89, 1)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(89n),
      this.instances1.alice.encrypt4(1n),
    );
    expect(res).to.equal(89n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(4n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(8n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract1.or_euint8_euint4(
      this.instances1.alice.encrypt8(8n),
      this.instances1.alice.encrypt4(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 1 (100, 12)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(100n),
      this.instances1.alice.encrypt4(12n),
    );
    expect(res).to.equal(104n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 2 (8, 12)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(8n),
      this.instances1.alice.encrypt4(12n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 3 (12, 12)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(12n),
      this.instances1.alice.encrypt4(12n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 4 (12, 8)', async function () {
    const res = await this.contract1.xor_euint8_euint4(
      this.instances1.alice.encrypt8(12n),
      this.instances1.alice.encrypt4(8n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 1 (168, 1)', async function () {
    const res = await this.contract2.eq_euint8_euint4(
      this.instances2.alice.encrypt8(168n),
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

  it('test operator "ne" overload (euint8, euint4) => ebool test 1 (237, 1)', async function () {
    const res = await this.contract2.ne_euint8_euint4(
      this.instances2.alice.encrypt8(237n),
      this.instances2.alice.encrypt4(1n),
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

  it('test operator "ge" overload (euint8, euint4) => ebool test 1 (228, 4)', async function () {
    const res = await this.contract2.ge_euint8_euint4(
      this.instances2.alice.encrypt8(228n),
      this.instances2.alice.encrypt4(4n),
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

  it('test operator "gt" overload (euint8, euint4) => ebool test 1 (53, 14)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(53n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(10n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(14n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract2.gt_euint8_euint4(
      this.instances2.alice.encrypt8(14n),
      this.instances2.alice.encrypt4(10n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 1 (194, 8)', async function () {
    const res = await this.contract2.le_euint8_euint4(
      this.instances2.alice.encrypt8(194n),
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

  it('test operator "lt" overload (euint8, euint4) => ebool test 1 (130, 5)', async function () {
    const res = await this.contract2.lt_euint8_euint4(
      this.instances2.alice.encrypt8(130n),
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

  it('test operator "min" overload (euint8, euint4) => euint8 test 1 (58, 11)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(58n),
      this.instances2.alice.encrypt4(11n),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 2 (7, 11)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(7n),
      this.instances2.alice.encrypt4(11n),
    );
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 3 (11, 11)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(11n),
      this.instances2.alice.encrypt4(11n),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 4 (11, 7)', async function () {
    const res = await this.contract2.min_euint8_euint4(
      this.instances2.alice.encrypt8(11n),
      this.instances2.alice.encrypt4(7n),
    );
    expect(res).to.equal(7n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 1 (211, 14)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(211n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(211n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 2 (10, 14)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(10n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 3 (14, 14)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(14n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 4 (14, 10)', async function () {
    const res = await this.contract2.max_euint8_euint4(
      this.instances2.alice.encrypt8(14n),
      this.instances2.alice.encrypt4(10n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 1 (93, 114)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(93n),
      this.instances2.alice.encrypt8(114n),
    );
    expect(res).to.equal(207n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 2 (91, 93)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(91n),
      this.instances2.alice.encrypt8(93n),
    );
    expect(res).to.equal(184n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 3 (93, 93)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(93n),
      this.instances2.alice.encrypt8(93n),
    );
    expect(res).to.equal(186n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 4 (93, 91)', async function () {
    const res = await this.contract2.add_euint8_euint8(
      this.instances2.alice.encrypt8(93n),
      this.instances2.alice.encrypt8(91n),
    );
    expect(res).to.equal(184n);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 1 (52, 52)', async function () {
    const res = await this.contract2.sub_euint8_euint8(
      this.instances2.alice.encrypt8(52n),
      this.instances2.alice.encrypt8(52n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 2 (52, 48)', async function () {
    const res = await this.contract2.sub_euint8_euint8(
      this.instances2.alice.encrypt8(52n),
      this.instances2.alice.encrypt8(48n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 1 (9, 17)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(9n),
      this.instances2.alice.encrypt8(17n),
    );
    expect(res).to.equal(153n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 2 (15, 16)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(15n),
      this.instances2.alice.encrypt8(16n),
    );
    expect(res).to.equal(240n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 3 (9, 9)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(9n),
      this.instances2.alice.encrypt8(9n),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 4 (16, 15)', async function () {
    const res = await this.contract2.mul_euint8_euint8(
      this.instances2.alice.encrypt8(16n),
      this.instances2.alice.encrypt8(15n),
    );
    expect(res).to.equal(240n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 1 (254, 204)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(254n),
      this.instances2.alice.encrypt8(204n),
    );
    expect(res).to.equal(204n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 2 (200, 204)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(200n),
      this.instances2.alice.encrypt8(204n),
    );
    expect(res).to.equal(200n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 3 (204, 204)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(204n),
      this.instances2.alice.encrypt8(204n),
    );
    expect(res).to.equal(204n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 4 (204, 200)', async function () {
    const res = await this.contract2.and_euint8_euint8(
      this.instances2.alice.encrypt8(204n),
      this.instances2.alice.encrypt8(200n),
    );
    expect(res).to.equal(200n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 1 (174, 137)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(174n),
      this.instances2.alice.encrypt8(137n),
    );
    expect(res).to.equal(175n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 2 (133, 137)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(133n),
      this.instances2.alice.encrypt8(137n),
    );
    expect(res).to.equal(141n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 3 (137, 137)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(137n),
      this.instances2.alice.encrypt8(137n),
    );
    expect(res).to.equal(137n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 4 (137, 133)', async function () {
    const res = await this.contract2.or_euint8_euint8(
      this.instances2.alice.encrypt8(137n),
      this.instances2.alice.encrypt8(133n),
    );
    expect(res).to.equal(141n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 1 (183, 10)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(183n),
      this.instances2.alice.encrypt8(10n),
    );
    expect(res).to.equal(189n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 2 (6, 10)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(6n),
      this.instances2.alice.encrypt8(10n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 3 (10, 10)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(10n),
      this.instances2.alice.encrypt8(10n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 4 (10, 6)', async function () {
    const res = await this.contract2.xor_euint8_euint8(
      this.instances2.alice.encrypt8(10n),
      this.instances2.alice.encrypt8(6n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 1 (101, 188)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(101n),
      this.instances2.alice.encrypt8(188n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 2 (97, 101)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(97n),
      this.instances2.alice.encrypt8(101n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 3 (101, 101)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(101n),
      this.instances2.alice.encrypt8(101n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 4 (101, 97)', async function () {
    const res = await this.contract2.eq_euint8_euint8(
      this.instances2.alice.encrypt8(101n),
      this.instances2.alice.encrypt8(97n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 1 (250, 143)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(250n),
      this.instances2.alice.encrypt8(143n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 2 (139, 143)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(139n),
      this.instances2.alice.encrypt8(143n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 3 (143, 143)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(143n),
      this.instances2.alice.encrypt8(143n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 4 (143, 139)', async function () {
    const res = await this.contract2.ne_euint8_euint8(
      this.instances2.alice.encrypt8(143n),
      this.instances2.alice.encrypt8(139n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 1 (41, 173)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(41n),
      this.instances2.alice.encrypt8(173n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 2 (37, 41)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(37n),
      this.instances2.alice.encrypt8(41n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 3 (41, 41)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(41n),
      this.instances2.alice.encrypt8(41n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 4 (41, 37)', async function () {
    const res = await this.contract2.ge_euint8_euint8(
      this.instances2.alice.encrypt8(41n),
      this.instances2.alice.encrypt8(37n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 1 (225, 96)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(225n),
      this.instances2.alice.encrypt8(96n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 2 (92, 96)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(92n),
      this.instances2.alice.encrypt8(96n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 3 (96, 96)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(96n),
      this.instances2.alice.encrypt8(96n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 4 (96, 92)', async function () {
    const res = await this.contract2.gt_euint8_euint8(
      this.instances2.alice.encrypt8(96n),
      this.instances2.alice.encrypt8(92n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 1 (102, 4)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(102n),
      this.instances2.alice.encrypt8(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(4n),
      this.instances2.alice.encrypt8(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt8(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.le_euint8_euint8(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt8(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 1 (253, 98)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(253n),
      this.instances2.alice.encrypt8(98n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 2 (94, 98)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(94n),
      this.instances2.alice.encrypt8(98n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 3 (98, 98)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(98n),
      this.instances2.alice.encrypt8(98n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 4 (98, 94)', async function () {
    const res = await this.contract2.lt_euint8_euint8(
      this.instances2.alice.encrypt8(98n),
      this.instances2.alice.encrypt8(94n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 1 (53, 97)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(53n),
      this.instances2.alice.encrypt8(97n),
    );
    expect(res).to.equal(53n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 2 (49, 53)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(49n),
      this.instances2.alice.encrypt8(53n),
    );
    expect(res).to.equal(49n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 3 (53, 53)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(53n),
      this.instances2.alice.encrypt8(53n),
    );
    expect(res).to.equal(53n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 4 (53, 49)', async function () {
    const res = await this.contract2.min_euint8_euint8(
      this.instances2.alice.encrypt8(53n),
      this.instances2.alice.encrypt8(49n),
    );
    expect(res).to.equal(49n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 1 (198, 211)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(198n),
      this.instances2.alice.encrypt8(211n),
    );
    expect(res).to.equal(211n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 2 (194, 198)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(194n),
      this.instances2.alice.encrypt8(198n),
    );
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 3 (198, 198)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(198n),
      this.instances2.alice.encrypt8(198n),
    );
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 4 (198, 194)', async function () {
    const res = await this.contract2.max_euint8_euint8(
      this.instances2.alice.encrypt8(198n),
      this.instances2.alice.encrypt8(194n),
    );
    expect(res).to.equal(198n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 1 (2, 208)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(2n),
      this.instances2.alice.encrypt16(208n),
    );
    expect(res).to.equal(210n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 2 (83, 85)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(83n),
      this.instances2.alice.encrypt16(85n),
    );
    expect(res).to.equal(168n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 3 (85, 85)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(85n),
      this.instances2.alice.encrypt16(85n),
    );
    expect(res).to.equal(170n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 4 (85, 83)', async function () {
    const res = await this.contract2.add_euint8_euint16(
      this.instances2.alice.encrypt8(85n),
      this.instances2.alice.encrypt16(83n),
    );
    expect(res).to.equal(168n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 1 (236, 236)', async function () {
    const res = await this.contract2.sub_euint8_euint16(
      this.instances2.alice.encrypt8(236n),
      this.instances2.alice.encrypt16(236n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 2 (236, 232)', async function () {
    const res = await this.contract2.sub_euint8_euint16(
      this.instances2.alice.encrypt8(236n),
      this.instances2.alice.encrypt16(232n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 1 (3, 60)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(3n),
      this.instances2.alice.encrypt16(60n),
    );
    expect(res).to.equal(180n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 2 (9, 9)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(9n),
      this.instances2.alice.encrypt16(9n),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 3 (9, 9)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(9n),
      this.instances2.alice.encrypt16(9n),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 4 (9, 9)', async function () {
    const res = await this.contract2.mul_euint8_euint16(
      this.instances2.alice.encrypt8(9n),
      this.instances2.alice.encrypt16(9n),
    );
    expect(res).to.equal(81n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 1 (112, 22367)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(112n),
      this.instances2.alice.encrypt16(22367n),
    );
    expect(res).to.equal(80n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 2 (108, 112)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(108n),
      this.instances2.alice.encrypt16(112n),
    );
    expect(res).to.equal(96n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 3 (112, 112)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(112n),
      this.instances2.alice.encrypt16(112n),
    );
    expect(res).to.equal(112n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 4 (112, 108)', async function () {
    const res = await this.contract2.and_euint8_euint16(
      this.instances2.alice.encrypt8(112n),
      this.instances2.alice.encrypt16(108n),
    );
    expect(res).to.equal(96n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 1 (99, 46031)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(99n),
      this.instances2.alice.encrypt16(46031n),
    );
    expect(res).to.equal(46063n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 2 (95, 99)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(95n),
      this.instances2.alice.encrypt16(99n),
    );
    expect(res).to.equal(127n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 3 (99, 99)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(99n),
      this.instances2.alice.encrypt16(99n),
    );
    expect(res).to.equal(99n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 4 (99, 95)', async function () {
    const res = await this.contract2.or_euint8_euint16(
      this.instances2.alice.encrypt8(99n),
      this.instances2.alice.encrypt16(95n),
    );
    expect(res).to.equal(127n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 1 (129, 2322)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(129n),
      this.instances2.alice.encrypt16(2322n),
    );
    expect(res).to.equal(2451n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 2 (125, 129)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(125n),
      this.instances2.alice.encrypt16(129n),
    );
    expect(res).to.equal(252n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 3 (129, 129)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(129n),
      this.instances2.alice.encrypt16(129n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 4 (129, 125)', async function () {
    const res = await this.contract2.xor_euint8_euint16(
      this.instances2.alice.encrypt8(129n),
      this.instances2.alice.encrypt16(125n),
    );
    expect(res).to.equal(252n);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 1 (111, 58517)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(111n),
      this.instances2.alice.encrypt16(58517n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 2 (107, 111)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(107n),
      this.instances2.alice.encrypt16(111n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 3 (111, 111)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(111n),
      this.instances2.alice.encrypt16(111n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 4 (111, 107)', async function () {
    const res = await this.contract2.eq_euint8_euint16(
      this.instances2.alice.encrypt8(111n),
      this.instances2.alice.encrypt16(107n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 1 (86, 21243)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(86n),
      this.instances2.alice.encrypt16(21243n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 2 (82, 86)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(82n),
      this.instances2.alice.encrypt16(86n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 3 (86, 86)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(86n),
      this.instances2.alice.encrypt16(86n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 4 (86, 82)', async function () {
    const res = await this.contract2.ne_euint8_euint16(
      this.instances2.alice.encrypt8(86n),
      this.instances2.alice.encrypt16(82n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 1 (204, 4953)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(204n),
      this.instances2.alice.encrypt16(4953n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 2 (200, 204)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(200n),
      this.instances2.alice.encrypt16(204n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 3 (204, 204)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(204n),
      this.instances2.alice.encrypt16(204n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 4 (204, 200)', async function () {
    const res = await this.contract2.ge_euint8_euint16(
      this.instances2.alice.encrypt8(204n),
      this.instances2.alice.encrypt16(200n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 1 (254, 24172)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(254n),
      this.instances2.alice.encrypt16(24172n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 2 (250, 254)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(250n),
      this.instances2.alice.encrypt16(254n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 3 (254, 254)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(254n),
      this.instances2.alice.encrypt16(254n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 4 (254, 250)', async function () {
    const res = await this.contract2.gt_euint8_euint16(
      this.instances2.alice.encrypt8(254n),
      this.instances2.alice.encrypt16(250n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 1 (31, 28651)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(31n),
      this.instances2.alice.encrypt16(28651n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 2 (27, 31)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(27n),
      this.instances2.alice.encrypt16(31n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 3 (31, 31)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(31n),
      this.instances2.alice.encrypt16(31n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 4 (31, 27)', async function () {
    const res = await this.contract2.le_euint8_euint16(
      this.instances2.alice.encrypt8(31n),
      this.instances2.alice.encrypt16(27n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 1 (113, 2877)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(113n),
      this.instances2.alice.encrypt16(2877n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 2 (109, 113)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(109n),
      this.instances2.alice.encrypt16(113n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 3 (113, 113)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(113n),
      this.instances2.alice.encrypt16(113n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 4 (113, 109)', async function () {
    const res = await this.contract2.lt_euint8_euint16(
      this.instances2.alice.encrypt8(113n),
      this.instances2.alice.encrypt16(109n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 1 (190, 58049)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(190n),
      this.instances2.alice.encrypt16(58049n),
    );
    expect(res).to.equal(190n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 2 (186, 190)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(186n),
      this.instances2.alice.encrypt16(190n),
    );
    expect(res).to.equal(186n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 3 (190, 190)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(190n),
      this.instances2.alice.encrypt16(190n),
    );
    expect(res).to.equal(190n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 4 (190, 186)', async function () {
    const res = await this.contract2.min_euint8_euint16(
      this.instances2.alice.encrypt8(190n),
      this.instances2.alice.encrypt16(186n),
    );
    expect(res).to.equal(186n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 1 (8, 11440)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt16(11440n),
    );
    expect(res).to.equal(11440n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(4n),
      this.instances2.alice.encrypt16(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt16(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.max_euint8_euint16(
      this.instances2.alice.encrypt8(8n),
      this.instances2.alice.encrypt16(4n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 1 (2, 161)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(2n),
      this.instances2.alice.encrypt32(161n),
    );
    expect(res).to.equal(163n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 2 (96, 100)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(96n),
      this.instances2.alice.encrypt32(100n),
    );
    expect(res).to.equal(196n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 3 (100, 100)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(100n),
      this.instances2.alice.encrypt32(100n),
    );
    expect(res).to.equal(200n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 4 (100, 96)', async function () {
    const res = await this.contract2.add_euint8_euint32(
      this.instances2.alice.encrypt8(100n),
      this.instances2.alice.encrypt32(96n),
    );
    expect(res).to.equal(196n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 1 (83, 83)', async function () {
    const res = await this.contract2.sub_euint8_euint32(
      this.instances2.alice.encrypt8(83n),
      this.instances2.alice.encrypt32(83n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 2 (83, 79)', async function () {
    const res = await this.contract2.sub_euint8_euint32(
      this.instances2.alice.encrypt8(83n),
      this.instances2.alice.encrypt32(79n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 1 (2, 96)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(2n),
      this.instances2.alice.encrypt32(96n),
    );
    expect(res).to.equal(192n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 2 (10, 12)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(10n),
      this.instances2.alice.encrypt32(12n),
    );
    expect(res).to.equal(120n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 3 (12, 12)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(12n),
      this.instances2.alice.encrypt32(12n),
    );
    expect(res).to.equal(144n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 4 (12, 10)', async function () {
    const res = await this.contract2.mul_euint8_euint32(
      this.instances2.alice.encrypt8(12n),
      this.instances2.alice.encrypt32(10n),
    );
    expect(res).to.equal(120n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 1 (36, 664013992)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(36n),
      this.instances2.alice.encrypt32(664013992n),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 2 (32, 36)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(32n),
      this.instances2.alice.encrypt32(36n),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 3 (36, 36)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(36n),
      this.instances2.alice.encrypt32(36n),
    );
    expect(res).to.equal(36n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 4 (36, 32)', async function () {
    const res = await this.contract2.and_euint8_euint32(
      this.instances2.alice.encrypt8(36n),
      this.instances2.alice.encrypt32(32n),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 1 (50, 1388677537)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(50n),
      this.instances2.alice.encrypt32(1388677537n),
    );
    expect(res).to.equal(1388677555n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 2 (46, 50)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(46n),
      this.instances2.alice.encrypt32(50n),
    );
    expect(res).to.equal(62n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 3 (50, 50)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(50n),
      this.instances2.alice.encrypt32(50n),
    );
    expect(res).to.equal(50n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 4 (50, 46)', async function () {
    const res = await this.contract2.or_euint8_euint32(
      this.instances2.alice.encrypt8(50n),
      this.instances2.alice.encrypt32(46n),
    );
    expect(res).to.equal(62n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 1 (67, 2386754441)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(67n),
      this.instances2.alice.encrypt32(2386754441n),
    );
    expect(res).to.equal(2386754506n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 2 (63, 67)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(63n),
      this.instances2.alice.encrypt32(67n),
    );
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 3 (67, 67)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(67n),
      this.instances2.alice.encrypt32(67n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 4 (67, 63)', async function () {
    const res = await this.contract2.xor_euint8_euint32(
      this.instances2.alice.encrypt8(67n),
      this.instances2.alice.encrypt32(63n),
    );
    expect(res).to.equal(124n);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 1 (161, 1325601812)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(161n),
      this.instances2.alice.encrypt32(1325601812n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 2 (157, 161)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(157n),
      this.instances2.alice.encrypt32(161n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 3 (161, 161)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(161n),
      this.instances2.alice.encrypt32(161n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 4 (161, 157)', async function () {
    const res = await this.contract2.eq_euint8_euint32(
      this.instances2.alice.encrypt8(161n),
      this.instances2.alice.encrypt32(157n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 1 (185, 1521229668)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(185n),
      this.instances2.alice.encrypt32(1521229668n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 2 (181, 185)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(181n),
      this.instances2.alice.encrypt32(185n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 3 (185, 185)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(185n),
      this.instances2.alice.encrypt32(185n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 4 (185, 181)', async function () {
    const res = await this.contract2.ne_euint8_euint32(
      this.instances2.alice.encrypt8(185n),
      this.instances2.alice.encrypt32(181n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 1 (214, 2636986545)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(214n),
      this.instances2.alice.encrypt32(2636986545n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 2 (210, 214)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(210n),
      this.instances2.alice.encrypt32(214n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 3 (214, 214)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(214n),
      this.instances2.alice.encrypt32(214n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 4 (214, 210)', async function () {
    const res = await this.contract2.ge_euint8_euint32(
      this.instances2.alice.encrypt8(214n),
      this.instances2.alice.encrypt32(210n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 1 (254, 3644170480)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(254n),
      this.instances2.alice.encrypt32(3644170480n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 2 (250, 254)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(250n),
      this.instances2.alice.encrypt32(254n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 3 (254, 254)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(254n),
      this.instances2.alice.encrypt32(254n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 4 (254, 250)', async function () {
    const res = await this.contract2.gt_euint8_euint32(
      this.instances2.alice.encrypt8(254n),
      this.instances2.alice.encrypt32(250n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 1 (200, 2966523441)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(200n),
      this.instances2.alice.encrypt32(2966523441n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 2 (196, 200)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(196n),
      this.instances2.alice.encrypt32(200n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 3 (200, 200)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(200n),
      this.instances2.alice.encrypt32(200n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 4 (200, 196)', async function () {
    const res = await this.contract2.le_euint8_euint32(
      this.instances2.alice.encrypt8(200n),
      this.instances2.alice.encrypt32(196n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 1 (80, 2205479823)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(80n),
      this.instances2.alice.encrypt32(2205479823n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 2 (76, 80)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(76n),
      this.instances2.alice.encrypt32(80n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 3 (80, 80)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(80n),
      this.instances2.alice.encrypt32(80n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 4 (80, 76)', async function () {
    const res = await this.contract2.lt_euint8_euint32(
      this.instances2.alice.encrypt8(80n),
      this.instances2.alice.encrypt32(76n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 1 (253, 2965000543)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(253n),
      this.instances2.alice.encrypt32(2965000543n),
    );
    expect(res).to.equal(253n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 2 (249, 253)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(249n),
      this.instances2.alice.encrypt32(253n),
    );
    expect(res).to.equal(249n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 3 (253, 253)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(253n),
      this.instances2.alice.encrypt32(253n),
    );
    expect(res).to.equal(253n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 4 (253, 249)', async function () {
    const res = await this.contract2.min_euint8_euint32(
      this.instances2.alice.encrypt8(253n),
      this.instances2.alice.encrypt32(249n),
    );
    expect(res).to.equal(249n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 1 (130, 566960922)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(130n),
      this.instances2.alice.encrypt32(566960922n),
    );
    expect(res).to.equal(566960922n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 2 (126, 130)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(126n),
      this.instances2.alice.encrypt32(130n),
    );
    expect(res).to.equal(130n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 3 (130, 130)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(130n),
      this.instances2.alice.encrypt32(130n),
    );
    expect(res).to.equal(130n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 4 (130, 126)', async function () {
    const res = await this.contract2.max_euint8_euint32(
      this.instances2.alice.encrypt8(130n),
      this.instances2.alice.encrypt32(126n),
    );
    expect(res).to.equal(130n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 1 (2, 129)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(2n),
      this.instances2.alice.encrypt64(129n),
    );
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 2 (115, 119)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(115n),
      this.instances2.alice.encrypt64(119n),
    );
    expect(res).to.equal(234n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 3 (119, 119)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(119n),
      this.instances2.alice.encrypt64(119n),
    );
    expect(res).to.equal(238n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 4 (119, 115)', async function () {
    const res = await this.contract2.add_euint8_euint64(
      this.instances2.alice.encrypt8(119n),
      this.instances2.alice.encrypt64(115n),
    );
    expect(res).to.equal(234n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 1 (168, 168)', async function () {
    const res = await this.contract2.sub_euint8_euint64(
      this.instances2.alice.encrypt8(168n),
      this.instances2.alice.encrypt64(168n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 2 (168, 164)', async function () {
    const res = await this.contract2.sub_euint8_euint64(
      this.instances2.alice.encrypt8(168n),
      this.instances2.alice.encrypt64(164n),
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

  it('test operator "mul" overload (euint8, euint64) => euint64 test 2 (10, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(10n),
      this.instances2.alice.encrypt64(11n),
    );
    expect(res).to.equal(110n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 3 (11, 11)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(11n),
      this.instances2.alice.encrypt64(11n),
    );
    expect(res).to.equal(121n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 4 (11, 10)', async function () {
    const res = await this.contract2.mul_euint8_euint64(
      this.instances2.alice.encrypt8(11n),
      this.instances2.alice.encrypt64(10n),
    );
    expect(res).to.equal(110n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 1 (187, 18444084668783699555)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(187n),
      this.instances2.alice.encrypt64(18444084668783699555n),
    );
    expect(res).to.equal(35n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 2 (183, 187)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(183n),
      this.instances2.alice.encrypt64(187n),
    );
    expect(res).to.equal(179n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 3 (187, 187)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(187n),
      this.instances2.alice.encrypt64(187n),
    );
    expect(res).to.equal(187n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 4 (187, 183)', async function () {
    const res = await this.contract2.and_euint8_euint64(
      this.instances2.alice.encrypt8(187n),
      this.instances2.alice.encrypt64(183n),
    );
    expect(res).to.equal(179n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 1 (135, 18439029290182698975)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(135n),
      this.instances2.alice.encrypt64(18439029290182698975n),
    );
    expect(res).to.equal(18439029290182698975n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 2 (131, 135)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(131n),
      this.instances2.alice.encrypt64(135n),
    );
    expect(res).to.equal(135n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 3 (135, 135)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(135n),
      this.instances2.alice.encrypt64(135n),
    );
    expect(res).to.equal(135n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 4 (135, 131)', async function () {
    const res = await this.contract2.or_euint8_euint64(
      this.instances2.alice.encrypt8(135n),
      this.instances2.alice.encrypt64(131n),
    );
    expect(res).to.equal(135n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 1 (84, 18444990477299490715)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(84n),
      this.instances2.alice.encrypt64(18444990477299490715n),
    );
    expect(res).to.equal(18444990477299490767n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 2 (80, 84)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(80n),
      this.instances2.alice.encrypt64(84n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 3 (84, 84)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(84n),
      this.instances2.alice.encrypt64(84n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 4 (84, 80)', async function () {
    const res = await this.contract2.xor_euint8_euint64(
      this.instances2.alice.encrypt8(84n),
      this.instances2.alice.encrypt64(80n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 1 (91, 18446457346943992227)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(91n),
      this.instances2.alice.encrypt64(18446457346943992227n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 2 (87, 91)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(87n),
      this.instances2.alice.encrypt64(91n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 3 (91, 91)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(91n),
      this.instances2.alice.encrypt64(91n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 4 (91, 87)', async function () {
    const res = await this.contract2.eq_euint8_euint64(
      this.instances2.alice.encrypt8(91n),
      this.instances2.alice.encrypt64(87n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 1 (109, 18443318570639553087)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(109n),
      this.instances2.alice.encrypt64(18443318570639553087n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 2 (105, 109)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(105n),
      this.instances2.alice.encrypt64(109n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 3 (109, 109)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(109n),
      this.instances2.alice.encrypt64(109n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 4 (109, 105)', async function () {
    const res = await this.contract2.ne_euint8_euint64(
      this.instances2.alice.encrypt8(109n),
      this.instances2.alice.encrypt64(105n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 1 (27, 18445930867214181117)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(27n),
      this.instances2.alice.encrypt64(18445930867214181117n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 2 (23, 27)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(23n),
      this.instances2.alice.encrypt64(27n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 3 (27, 27)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(27n),
      this.instances2.alice.encrypt64(27n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 4 (27, 23)', async function () {
    const res = await this.contract2.ge_euint8_euint64(
      this.instances2.alice.encrypt8(27n),
      this.instances2.alice.encrypt64(23n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 1 (235, 18439120393033635471)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(235n),
      this.instances2.alice.encrypt64(18439120393033635471n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 2 (231, 235)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(231n),
      this.instances2.alice.encrypt64(235n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 3 (235, 235)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(235n),
      this.instances2.alice.encrypt64(235n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 4 (235, 231)', async function () {
    const res = await this.contract2.gt_euint8_euint64(
      this.instances2.alice.encrypt8(235n),
      this.instances2.alice.encrypt64(231n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 1 (80, 18440486388708045995)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(80n),
      this.instances2.alice.encrypt64(18440486388708045995n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 2 (76, 80)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(76n),
      this.instances2.alice.encrypt64(80n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 3 (80, 80)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(80n),
      this.instances2.alice.encrypt64(80n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 4 (80, 76)', async function () {
    const res = await this.contract2.le_euint8_euint64(
      this.instances2.alice.encrypt8(80n),
      this.instances2.alice.encrypt64(76n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 1 (190, 18443665731691391943)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(190n),
      this.instances2.alice.encrypt64(18443665731691391943n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 2 (186, 190)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(186n),
      this.instances2.alice.encrypt64(190n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 3 (190, 190)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(190n),
      this.instances2.alice.encrypt64(190n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 4 (190, 186)', async function () {
    const res = await this.contract2.lt_euint8_euint64(
      this.instances2.alice.encrypt8(190n),
      this.instances2.alice.encrypt64(186n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 1 (18, 18446036892619585799)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(18n),
      this.instances2.alice.encrypt64(18446036892619585799n),
    );
    expect(res).to.equal(18n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 2 (14, 18)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(14n),
      this.instances2.alice.encrypt64(18n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 3 (18, 18)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(18n),
      this.instances2.alice.encrypt64(18n),
    );
    expect(res).to.equal(18n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 4 (18, 14)', async function () {
    const res = await this.contract2.min_euint8_euint64(
      this.instances2.alice.encrypt8(18n),
      this.instances2.alice.encrypt64(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 1 (172, 18445195398017975891)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(172n),
      this.instances2.alice.encrypt64(18445195398017975891n),
    );
    expect(res).to.equal(18445195398017975891n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 2 (168, 172)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(168n),
      this.instances2.alice.encrypt64(172n),
    );
    expect(res).to.equal(172n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 3 (172, 172)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(172n),
      this.instances2.alice.encrypt64(172n),
    );
    expect(res).to.equal(172n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 4 (172, 168)', async function () {
    const res = await this.contract2.max_euint8_euint64(
      this.instances2.alice.encrypt8(172n),
      this.instances2.alice.encrypt64(168n),
    );
    expect(res).to.equal(172n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 1 (93, 112)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(93n), 112);
    expect(res).to.equal(205n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 2 (91, 93)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(91n), 93);
    expect(res).to.equal(184n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 3 (93, 93)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(93n), 93);
    expect(res).to.equal(186n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 4 (93, 91)', async function () {
    const res = await this.contract2.add_euint8_uint8(this.instances2.alice.encrypt8(93n), 91);
    expect(res).to.equal(184n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 1 (34, 112)', async function () {
    const res = await this.contract2.add_uint8_euint8(34, this.instances2.alice.encrypt8(112n));
    expect(res).to.equal(146n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 2 (91, 93)', async function () {
    const res = await this.contract2.add_uint8_euint8(91, this.instances2.alice.encrypt8(93n));
    expect(res).to.equal(184n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 3 (93, 93)', async function () {
    const res = await this.contract2.add_uint8_euint8(93, this.instances2.alice.encrypt8(93n));
    expect(res).to.equal(186n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 4 (93, 91)', async function () {
    const res = await this.contract2.add_uint8_euint8(93, this.instances2.alice.encrypt8(91n));
    expect(res).to.equal(184n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 1 (52, 52)', async function () {
    const res = await this.contract2.sub_euint8_uint8(this.instances2.alice.encrypt8(52n), 52);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 2 (52, 48)', async function () {
    const res = await this.contract2.sub_euint8_uint8(this.instances2.alice.encrypt8(52n), 48);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 1 (52, 52)', async function () {
    const res = await this.contract2.sub_uint8_euint8(52, this.instances2.alice.encrypt8(52n));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 2 (52, 48)', async function () {
    const res = await this.contract2.sub_uint8_euint8(52, this.instances2.alice.encrypt8(48n));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 1 (16, 7)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(16n), 7);
    expect(res).to.equal(112n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 2 (15, 16)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(15n), 16);
    expect(res).to.equal(240n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 3 (9, 9)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(9n), 9);
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 4 (16, 15)', async function () {
    const res = await this.contract2.mul_euint8_uint8(this.instances2.alice.encrypt8(16n), 15);
    expect(res).to.equal(240n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 1 (14, 12)', async function () {
    const res = await this.contract2.mul_uint8_euint8(14, this.instances2.alice.encrypt8(12n));
    expect(res).to.equal(168n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 2 (15, 16)', async function () {
    const res = await this.contract2.mul_uint8_euint8(15, this.instances2.alice.encrypt8(16n));
    expect(res).to.equal(240n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 3 (9, 9)', async function () {
    const res = await this.contract2.mul_uint8_euint8(9, this.instances2.alice.encrypt8(9n));
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 4 (16, 15)', async function () {
    const res = await this.contract2.mul_uint8_euint8(16, this.instances2.alice.encrypt8(15n));
    expect(res).to.equal(240n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 1 (123, 214)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(123n), 214);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 2 (20, 24)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(20n), 24);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 3 (24, 24)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(24n), 24);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 4 (24, 20)', async function () {
    const res = await this.contract2.div_euint8_uint8(this.instances2.alice.encrypt8(24n), 20);
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 1 (144, 19)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(144n), 19);
    expect(res).to.equal(11n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 2 (140, 144)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(140n), 144);
    expect(res).to.equal(140n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 3 (144, 144)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(144n), 144);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 4 (144, 140)', async function () {
    const res = await this.contract2.rem_euint8_uint8(this.instances2.alice.encrypt8(144n), 140);
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 1 (101, 175)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(101n), 175);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 2 (97, 101)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(97n), 101);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 3 (101, 101)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(101n), 101);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 4 (101, 97)', async function () {
    const res = await this.contract2.eq_euint8_uint8(this.instances2.alice.encrypt8(101n), 97);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 1 (182, 175)', async function () {
    const res = await this.contract2.eq_uint8_euint8(182, this.instances2.alice.encrypt8(175n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 2 (97, 101)', async function () {
    const res = await this.contract2.eq_uint8_euint8(97, this.instances2.alice.encrypt8(101n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 3 (101, 101)', async function () {
    const res = await this.contract2.eq_uint8_euint8(101, this.instances2.alice.encrypt8(101n));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 4 (101, 97)', async function () {
    const res = await this.contract2.eq_uint8_euint8(101, this.instances2.alice.encrypt8(97n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 1 (250, 114)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(250n), 114);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 2 (139, 143)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(139n), 143);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 3 (143, 143)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(143n), 143);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 4 (143, 139)', async function () {
    const res = await this.contract2.ne_euint8_uint8(this.instances2.alice.encrypt8(143n), 139);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 1 (139, 114)', async function () {
    const res = await this.contract2.ne_uint8_euint8(139, this.instances2.alice.encrypt8(114n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 2 (139, 143)', async function () {
    const res = await this.contract2.ne_uint8_euint8(139, this.instances2.alice.encrypt8(143n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 3 (143, 143)', async function () {
    const res = await this.contract2.ne_uint8_euint8(143, this.instances2.alice.encrypt8(143n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 4 (143, 139)', async function () {
    const res = await this.contract2.ne_uint8_euint8(143, this.instances2.alice.encrypt8(139n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (41, 155)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(41n), 155);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 2 (37, 41)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(37n), 41);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 3 (41, 41)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(41n), 41);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 4 (41, 37)', async function () {
    const res = await this.contract2.ge_euint8_uint8(this.instances2.alice.encrypt8(41n), 37);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 1 (28, 155)', async function () {
    const res = await this.contract2.ge_uint8_euint8(28, this.instances2.alice.encrypt8(155n));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 2 (37, 41)', async function () {
    const res = await this.contract2.ge_uint8_euint8(37, this.instances2.alice.encrypt8(41n));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 3 (41, 41)', async function () {
    const res = await this.contract2.ge_uint8_euint8(41, this.instances2.alice.encrypt8(41n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 4 (41, 37)', async function () {
    const res = await this.contract2.ge_uint8_euint8(41, this.instances2.alice.encrypt8(37n));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (225, 176)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(225n), 176);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 2 (92, 96)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(92n), 96);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 3 (96, 96)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(96n), 96);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 4 (96, 92)', async function () {
    const res = await this.contract2.gt_euint8_uint8(this.instances2.alice.encrypt8(96n), 92);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 1 (34, 176)', async function () {
    const res = await this.contract2.gt_uint8_euint8(34, this.instances2.alice.encrypt8(176n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 2 (92, 96)', async function () {
    const res = await this.contract2.gt_uint8_euint8(92, this.instances2.alice.encrypt8(96n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 3 (96, 96)', async function () {
    const res = await this.contract2.gt_uint8_euint8(96, this.instances2.alice.encrypt8(96n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 4 (96, 92)', async function () {
    const res = await this.contract2.gt_uint8_euint8(96, this.instances2.alice.encrypt8(92n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (102, 85)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(102n), 85);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(4n), 8);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(8n), 8);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.le_euint8_uint8(this.instances2.alice.encrypt8(8n), 4);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 1 (46, 85)', async function () {
    const res = await this.contract2.le_uint8_euint8(46, this.instances2.alice.encrypt8(85n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.le_uint8_euint8(4, this.instances2.alice.encrypt8(8n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.le_uint8_euint8(8, this.instances2.alice.encrypt8(8n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.le_uint8_euint8(8, this.instances2.alice.encrypt8(4n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 1 (253, 9)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(253n), 9);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 2 (94, 98)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(94n), 98);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 3 (98, 98)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(98n), 98);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 4 (98, 94)', async function () {
    const res = await this.contract2.lt_euint8_uint8(this.instances2.alice.encrypt8(98n), 94);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (79, 9)', async function () {
    const res = await this.contract2.lt_uint8_euint8(79, this.instances2.alice.encrypt8(9n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 2 (94, 98)', async function () {
    const res = await this.contract2.lt_uint8_euint8(94, this.instances2.alice.encrypt8(98n));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 3 (98, 98)', async function () {
    const res = await this.contract2.lt_uint8_euint8(98, this.instances2.alice.encrypt8(98n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 4 (98, 94)', async function () {
    const res = await this.contract2.lt_uint8_euint8(98, this.instances2.alice.encrypt8(94n));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (53, 70)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(53n), 70);
    expect(res).to.equal(53n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (49, 53)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(49n), 53);
    expect(res).to.equal(49n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (53, 53)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(53n), 53);
    expect(res).to.equal(53n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 4 (53, 49)', async function () {
    const res = await this.contract2.min_euint8_uint8(this.instances2.alice.encrypt8(53n), 49);
    expect(res).to.equal(49n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (109, 70)', async function () {
    const res = await this.contract2.min_uint8_euint8(109, this.instances2.alice.encrypt8(70n));
    expect(res).to.equal(70n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 2 (49, 53)', async function () {
    const res = await this.contract2.min_uint8_euint8(49, this.instances2.alice.encrypt8(53n));
    expect(res).to.equal(49n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 3 (53, 53)', async function () {
    const res = await this.contract2.min_uint8_euint8(53, this.instances2.alice.encrypt8(53n));
    expect(res).to.equal(53n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 4 (53, 49)', async function () {
    const res = await this.contract2.min_uint8_euint8(53, this.instances2.alice.encrypt8(49n));
    expect(res).to.equal(49n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (198, 31)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(198n), 31);
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (194, 198)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(194n), 198);
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (198, 198)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(198n), 198);
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 4 (198, 194)', async function () {
    const res = await this.contract2.max_euint8_uint8(this.instances2.alice.encrypt8(198n), 194);
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 1 (102, 31)', async function () {
    const res = await this.contract2.max_uint8_euint8(102, this.instances2.alice.encrypt8(31n));
    expect(res).to.equal(102n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 2 (194, 198)', async function () {
    const res = await this.contract2.max_uint8_euint8(194, this.instances2.alice.encrypt8(198n));
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 3 (198, 198)', async function () {
    const res = await this.contract2.max_uint8_euint8(198, this.instances2.alice.encrypt8(198n));
    expect(res).to.equal(198n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 4 (198, 194)', async function () {
    const res = await this.contract2.max_uint8_euint8(198, this.instances2.alice.encrypt8(194n));
    expect(res).to.equal(198n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 1 (13, 2)', async function () {
    const res = await this.contract2.add_euint16_euint4(
      this.instances2.alice.encrypt16(13n),
      this.instances2.alice.encrypt4(2n),
    );
    expect(res).to.equal(15n);
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

  it('test operator "sub" overload (euint16, euint4) => euint16 test 1 (14, 14)', async function () {
    const res = await this.contract2.sub_euint16_euint4(
      this.instances2.alice.encrypt16(14n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint4) => euint16 test 2 (14, 10)', async function () {
    const res = await this.contract2.sub_euint16_euint4(
      this.instances2.alice.encrypt16(14n),
      this.instances2.alice.encrypt4(10n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 1 (7, 2)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(7n),
      this.instances2.alice.encrypt4(2n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 2 (3, 5)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(3n),
      this.instances2.alice.encrypt4(5n),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 3 (3, 3)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(3n),
      this.instances2.alice.encrypt4(3n),
    );
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 4 (5, 3)', async function () {
    const res = await this.contract2.mul_euint16_euint4(
      this.instances2.alice.encrypt16(5n),
      this.instances2.alice.encrypt4(3n),
    );
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 1 (6680, 4)', async function () {
    const res = await this.contract2.and_euint16_euint4(
      this.instances2.alice.encrypt16(6680n),
      this.instances2.alice.encrypt4(4n),
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

  it('test operator "or" overload (euint16, euint4) => euint16 test 1 (57838, 2)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(57838n),
      this.instances2.alice.encrypt4(2n),
    );
    expect(res).to.equal(57838n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(4n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract2.or_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 1 (29564, 14)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(29564n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(29554n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 2 (10, 14)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(10n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 3 (14, 14)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(14n),
      this.instances2.alice.encrypt4(14n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 4 (14, 10)', async function () {
    const res = await this.contract2.xor_euint16_euint4(
      this.instances2.alice.encrypt16(14n),
      this.instances2.alice.encrypt4(10n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 1 (43178, 6)', async function () {
    const res = await this.contract2.eq_euint16_euint4(
      this.instances2.alice.encrypt16(43178n),
      this.instances2.alice.encrypt4(6n),
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

  it('test operator "ne" overload (euint16, euint4) => ebool test 1 (36210, 8)', async function () {
    const res = await this.contract2.ne_euint16_euint4(
      this.instances2.alice.encrypt16(36210n),
      this.instances2.alice.encrypt4(8n),
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

  it('test operator "ge" overload (euint16, euint4) => ebool test 1 (9661, 2)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(9661n),
      this.instances2.alice.encrypt4(2n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(4n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.ge_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 1 (36260, 13)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(36260n),
      this.instances2.alice.encrypt4(13n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 2 (9, 13)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(9n),
      this.instances2.alice.encrypt4(13n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 3 (13, 13)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(13n),
      this.instances2.alice.encrypt4(13n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 4 (13, 9)', async function () {
    const res = await this.contract2.gt_euint16_euint4(
      this.instances2.alice.encrypt16(13n),
      this.instances2.alice.encrypt4(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 1 (7456, 9)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(7456n),
      this.instances2.alice.encrypt4(9n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(5n),
      this.instances2.alice.encrypt4(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(9n),
      this.instances2.alice.encrypt4(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract2.le_euint16_euint4(
      this.instances2.alice.encrypt16(9n),
      this.instances2.alice.encrypt4(5n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 1 (53504, 6)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(53504n),
      this.instances2.alice.encrypt4(6n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(4n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract2.lt_euint16_euint4(
      this.instances2.alice.encrypt16(8n),
      this.instances2.alice.encrypt4(4n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 1 (37174, 5)', async function () {
    const res = await this.contract3.min_euint16_euint4(
      this.instances3.alice.encrypt16(37174n),
      this.instances3.alice.encrypt4(5n),
    );
    expect(res).to.equal(5n);
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

  it('test operator "max" overload (euint16, euint4) => euint16 test 1 (29272, 10)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(29272n),
      this.instances3.alice.encrypt4(10n),
    );
    expect(res).to.equal(29272n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 2 (6, 10)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(6n),
      this.instances3.alice.encrypt4(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 3 (10, 10)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(10n),
      this.instances3.alice.encrypt4(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 4 (10, 6)', async function () {
    const res = await this.contract3.max_euint16_euint4(
      this.instances3.alice.encrypt16(10n),
      this.instances3.alice.encrypt4(6n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 1 (171, 2)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(171n),
      this.instances3.alice.encrypt8(2n),
    );
    expect(res).to.equal(173n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 2 (7, 11)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(7n),
      this.instances3.alice.encrypt8(11n),
    );
    expect(res).to.equal(18n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 3 (11, 11)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(11n),
      this.instances3.alice.encrypt8(11n),
    );
    expect(res).to.equal(22n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 4 (11, 7)', async function () {
    const res = await this.contract3.add_euint16_euint8(
      this.instances3.alice.encrypt16(11n),
      this.instances3.alice.encrypt8(7n),
    );
    expect(res).to.equal(18n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (166, 166)', async function () {
    const res = await this.contract3.sub_euint16_euint8(
      this.instances3.alice.encrypt16(166n),
      this.instances3.alice.encrypt8(166n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (166, 162)', async function () {
    const res = await this.contract3.sub_euint16_euint8(
      this.instances3.alice.encrypt16(166n),
      this.instances3.alice.encrypt8(162n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (110, 2)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(110n),
      this.instances3.alice.encrypt8(2n),
    );
    expect(res).to.equal(220n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 2 (13, 13)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(13n),
      this.instances3.alice.encrypt8(13n),
    );
    expect(res).to.equal(169n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 3 (13, 13)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(13n),
      this.instances3.alice.encrypt8(13n),
    );
    expect(res).to.equal(169n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 4 (13, 13)', async function () {
    const res = await this.contract3.mul_euint16_euint8(
      this.instances3.alice.encrypt16(13n),
      this.instances3.alice.encrypt8(13n),
    );
    expect(res).to.equal(169n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 1 (18191, 188)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(18191n),
      this.instances3.alice.encrypt8(188n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 2 (184, 188)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(184n),
      this.instances3.alice.encrypt8(188n),
    );
    expect(res).to.equal(184n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 3 (188, 188)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(188n),
      this.instances3.alice.encrypt8(188n),
    );
    expect(res).to.equal(188n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 4 (188, 184)', async function () {
    const res = await this.contract3.and_euint16_euint8(
      this.instances3.alice.encrypt16(188n),
      this.instances3.alice.encrypt8(184n),
    );
    expect(res).to.equal(184n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 1 (60745, 176)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(60745n),
      this.instances3.alice.encrypt8(176n),
    );
    expect(res).to.equal(60921n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 2 (172, 176)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(172n),
      this.instances3.alice.encrypt8(176n),
    );
    expect(res).to.equal(188n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 3 (176, 176)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(176n),
      this.instances3.alice.encrypt8(176n),
    );
    expect(res).to.equal(176n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 4 (176, 172)', async function () {
    const res = await this.contract3.or_euint16_euint8(
      this.instances3.alice.encrypt16(176n),
      this.instances3.alice.encrypt8(172n),
    );
    expect(res).to.equal(188n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (24120, 221)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(24120n),
      this.instances3.alice.encrypt8(221n),
    );
    expect(res).to.equal(24293n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (217, 221)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(217n),
      this.instances3.alice.encrypt8(221n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 3 (221, 221)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(221n),
      this.instances3.alice.encrypt8(221n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 4 (221, 217)', async function () {
    const res = await this.contract3.xor_euint16_euint8(
      this.instances3.alice.encrypt16(221n),
      this.instances3.alice.encrypt8(217n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 1 (52156, 49)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(52156n),
      this.instances3.alice.encrypt8(49n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 2 (45, 49)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(45n),
      this.instances3.alice.encrypt8(49n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 3 (49, 49)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(49n),
      this.instances3.alice.encrypt8(49n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 4 (49, 45)', async function () {
    const res = await this.contract3.eq_euint16_euint8(
      this.instances3.alice.encrypt16(49n),
      this.instances3.alice.encrypt8(45n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (45762, 206)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(45762n),
      this.instances3.alice.encrypt8(206n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (202, 206)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(202n),
      this.instances3.alice.encrypt8(206n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 3 (206, 206)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(206n),
      this.instances3.alice.encrypt8(206n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 4 (206, 202)', async function () {
    const res = await this.contract3.ne_euint16_euint8(
      this.instances3.alice.encrypt16(206n),
      this.instances3.alice.encrypt8(202n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (3763, 81)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(3763n),
      this.instances3.alice.encrypt8(81n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (77, 81)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(77n),
      this.instances3.alice.encrypt8(81n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (81, 81)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(81n),
      this.instances3.alice.encrypt8(81n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 4 (81, 77)', async function () {
    const res = await this.contract3.ge_euint16_euint8(
      this.instances3.alice.encrypt16(81n),
      this.instances3.alice.encrypt8(77n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 1 (17649, 8)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(17649n),
      this.instances3.alice.encrypt8(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(4n),
      this.instances3.alice.encrypt8(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(8n),
      this.instances3.alice.encrypt8(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract3.gt_euint16_euint8(
      this.instances3.alice.encrypt16(8n),
      this.instances3.alice.encrypt8(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 1 (3242, 147)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(3242n),
      this.instances3.alice.encrypt8(147n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 2 (143, 147)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(143n),
      this.instances3.alice.encrypt8(147n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 3 (147, 147)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(147n),
      this.instances3.alice.encrypt8(147n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 4 (147, 143)', async function () {
    const res = await this.contract3.le_euint16_euint8(
      this.instances3.alice.encrypt16(147n),
      this.instances3.alice.encrypt8(143n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 1 (42518, 110)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(42518n),
      this.instances3.alice.encrypt8(110n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 2 (106, 110)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(106n),
      this.instances3.alice.encrypt8(110n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 3 (110, 110)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(110n),
      this.instances3.alice.encrypt8(110n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 4 (110, 106)', async function () {
    const res = await this.contract3.lt_euint16_euint8(
      this.instances3.alice.encrypt16(110n),
      this.instances3.alice.encrypt8(106n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 1 (25144, 64)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(25144n),
      this.instances3.alice.encrypt8(64n),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 2 (60, 64)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(60n),
      this.instances3.alice.encrypt8(64n),
    );
    expect(res).to.equal(60n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 3 (64, 64)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(64n),
      this.instances3.alice.encrypt8(64n),
    );
    expect(res).to.equal(64n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 4 (64, 60)', async function () {
    const res = await this.contract3.min_euint16_euint8(
      this.instances3.alice.encrypt16(64n),
      this.instances3.alice.encrypt8(60n),
    );
    expect(res).to.equal(60n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 1 (18336, 117)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(18336n),
      this.instances3.alice.encrypt8(117n),
    );
    expect(res).to.equal(18336n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 2 (113, 117)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(113n),
      this.instances3.alice.encrypt8(117n),
    );
    expect(res).to.equal(117n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 3 (117, 117)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(117n),
      this.instances3.alice.encrypt8(117n),
    );
    expect(res).to.equal(117n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 4 (117, 113)', async function () {
    const res = await this.contract3.max_euint16_euint8(
      this.instances3.alice.encrypt16(117n),
      this.instances3.alice.encrypt8(113n),
    );
    expect(res).to.equal(117n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 1 (27226, 22058)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(27226n),
      this.instances3.alice.encrypt16(22058n),
    );
    expect(res).to.equal(49284n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 2 (22056, 22058)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(22056n),
      this.instances3.alice.encrypt16(22058n),
    );
    expect(res).to.equal(44114n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 3 (22058, 22058)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(22058n),
      this.instances3.alice.encrypt16(22058n),
    );
    expect(res).to.equal(44116n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 4 (22058, 22056)', async function () {
    const res = await this.contract3.add_euint16_euint16(
      this.instances3.alice.encrypt16(22058n),
      this.instances3.alice.encrypt16(22056n),
    );
    expect(res).to.equal(44114n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 1 (56955, 56955)', async function () {
    const res = await this.contract3.sub_euint16_euint16(
      this.instances3.alice.encrypt16(56955n),
      this.instances3.alice.encrypt16(56955n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 2 (56955, 56951)', async function () {
    const res = await this.contract3.sub_euint16_euint16(
      this.instances3.alice.encrypt16(56955n),
      this.instances3.alice.encrypt16(56951n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 1 (120, 150)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(120n),
      this.instances3.alice.encrypt16(150n),
    );
    expect(res).to.equal(18000n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 2 (239, 239)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(239n),
      this.instances3.alice.encrypt16(239n),
    );
    expect(res).to.equal(57121n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 3 (239, 239)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(239n),
      this.instances3.alice.encrypt16(239n),
    );
    expect(res).to.equal(57121n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 4 (239, 239)', async function () {
    const res = await this.contract3.mul_euint16_euint16(
      this.instances3.alice.encrypt16(239n),
      this.instances3.alice.encrypt16(239n),
    );
    expect(res).to.equal(57121n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 1 (63103, 25335)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(63103n),
      this.instances3.alice.encrypt16(25335n),
    );
    expect(res).to.equal(25207n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 2 (25331, 25335)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(25331n),
      this.instances3.alice.encrypt16(25335n),
    );
    expect(res).to.equal(25331n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 3 (25335, 25335)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(25335n),
      this.instances3.alice.encrypt16(25335n),
    );
    expect(res).to.equal(25335n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 4 (25335, 25331)', async function () {
    const res = await this.contract3.and_euint16_euint16(
      this.instances3.alice.encrypt16(25335n),
      this.instances3.alice.encrypt16(25331n),
    );
    expect(res).to.equal(25331n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 1 (53682, 14727)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(53682n),
      this.instances3.alice.encrypt16(14727n),
    );
    expect(res).to.equal(63927n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 2 (14723, 14727)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(14723n),
      this.instances3.alice.encrypt16(14727n),
    );
    expect(res).to.equal(14727n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 3 (14727, 14727)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(14727n),
      this.instances3.alice.encrypt16(14727n),
    );
    expect(res).to.equal(14727n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 4 (14727, 14723)', async function () {
    const res = await this.contract3.or_euint16_euint16(
      this.instances3.alice.encrypt16(14727n),
      this.instances3.alice.encrypt16(14723n),
    );
    expect(res).to.equal(14727n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (272, 42865)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(272n),
      this.instances3.alice.encrypt16(42865n),
    );
    expect(res).to.equal(42593n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (268, 272)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(268n),
      this.instances3.alice.encrypt16(272n),
    );
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 3 (272, 272)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(272n),
      this.instances3.alice.encrypt16(272n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 4 (272, 268)', async function () {
    const res = await this.contract3.xor_euint16_euint16(
      this.instances3.alice.encrypt16(272n),
      this.instances3.alice.encrypt16(268n),
    );
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 1 (53936, 55687)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(53936n),
      this.instances3.alice.encrypt16(55687n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 2 (53932, 53936)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(53932n),
      this.instances3.alice.encrypt16(53936n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 3 (53936, 53936)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(53936n),
      this.instances3.alice.encrypt16(53936n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 4 (53936, 53932)', async function () {
    const res = await this.contract3.eq_euint16_euint16(
      this.instances3.alice.encrypt16(53936n),
      this.instances3.alice.encrypt16(53932n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 1 (64200, 17038)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(64200n),
      this.instances3.alice.encrypt16(17038n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 2 (17034, 17038)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(17034n),
      this.instances3.alice.encrypt16(17038n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 3 (17038, 17038)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(17038n),
      this.instances3.alice.encrypt16(17038n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 4 (17038, 17034)', async function () {
    const res = await this.contract3.ne_euint16_euint16(
      this.instances3.alice.encrypt16(17038n),
      this.instances3.alice.encrypt16(17034n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 1 (18226, 11817)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(18226n),
      this.instances3.alice.encrypt16(11817n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 2 (11813, 11817)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(11813n),
      this.instances3.alice.encrypt16(11817n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 3 (11817, 11817)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(11817n),
      this.instances3.alice.encrypt16(11817n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 4 (11817, 11813)', async function () {
    const res = await this.contract3.ge_euint16_euint16(
      this.instances3.alice.encrypt16(11817n),
      this.instances3.alice.encrypt16(11813n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 1 (46712, 53146)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(46712n),
      this.instances3.alice.encrypt16(53146n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 2 (46708, 46712)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(46708n),
      this.instances3.alice.encrypt16(46712n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 3 (46712, 46712)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(46712n),
      this.instances3.alice.encrypt16(46712n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 4 (46712, 46708)', async function () {
    const res = await this.contract3.gt_euint16_euint16(
      this.instances3.alice.encrypt16(46712n),
      this.instances3.alice.encrypt16(46708n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 1 (9281, 48556)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(9281n),
      this.instances3.alice.encrypt16(48556n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 2 (9277, 9281)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(9277n),
      this.instances3.alice.encrypt16(9281n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 3 (9281, 9281)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(9281n),
      this.instances3.alice.encrypt16(9281n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 4 (9281, 9277)', async function () {
    const res = await this.contract3.le_euint16_euint16(
      this.instances3.alice.encrypt16(9281n),
      this.instances3.alice.encrypt16(9277n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 1 (44794, 23290)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(44794n),
      this.instances3.alice.encrypt16(23290n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 2 (23286, 23290)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(23286n),
      this.instances3.alice.encrypt16(23290n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 3 (23290, 23290)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(23290n),
      this.instances3.alice.encrypt16(23290n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 4 (23290, 23286)', async function () {
    const res = await this.contract3.lt_euint16_euint16(
      this.instances3.alice.encrypt16(23290n),
      this.instances3.alice.encrypt16(23286n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 1 (30936, 9027)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(30936n),
      this.instances3.alice.encrypt16(9027n),
    );
    expect(res).to.equal(9027n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 2 (9023, 9027)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(9023n),
      this.instances3.alice.encrypt16(9027n),
    );
    expect(res).to.equal(9023n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 3 (9027, 9027)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(9027n),
      this.instances3.alice.encrypt16(9027n),
    );
    expect(res).to.equal(9027n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 4 (9027, 9023)', async function () {
    const res = await this.contract3.min_euint16_euint16(
      this.instances3.alice.encrypt16(9027n),
      this.instances3.alice.encrypt16(9023n),
    );
    expect(res).to.equal(9023n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 1 (34561, 34789)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(34561n),
      this.instances3.alice.encrypt16(34789n),
    );
    expect(res).to.equal(34789n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 2 (34557, 34561)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(34557n),
      this.instances3.alice.encrypt16(34561n),
    );
    expect(res).to.equal(34561n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 3 (34561, 34561)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(34561n),
      this.instances3.alice.encrypt16(34561n),
    );
    expect(res).to.equal(34561n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 4 (34561, 34557)', async function () {
    const res = await this.contract3.max_euint16_euint16(
      this.instances3.alice.encrypt16(34561n),
      this.instances3.alice.encrypt16(34557n),
    );
    expect(res).to.equal(34561n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 1 (2, 41075)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(2n),
      this.instances3.alice.encrypt32(41075n),
    );
    expect(res).to.equal(41077n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 2 (25150, 25154)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(25150n),
      this.instances3.alice.encrypt32(25154n),
    );
    expect(res).to.equal(50304n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 3 (25154, 25154)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(25154n),
      this.instances3.alice.encrypt32(25154n),
    );
    expect(res).to.equal(50308n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 4 (25154, 25150)', async function () {
    const res = await this.contract3.add_euint16_euint32(
      this.instances3.alice.encrypt16(25154n),
      this.instances3.alice.encrypt32(25150n),
    );
    expect(res).to.equal(50304n);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (42408, 42408)', async function () {
    const res = await this.contract3.sub_euint16_euint32(
      this.instances3.alice.encrypt16(42408n),
      this.instances3.alice.encrypt32(42408n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (42408, 42404)', async function () {
    const res = await this.contract3.sub_euint16_euint32(
      this.instances3.alice.encrypt16(42408n),
      this.instances3.alice.encrypt32(42404n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (2, 32121)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(2n),
      this.instances3.alice.encrypt32(32121n),
    );
    expect(res).to.equal(64242n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 2 (235, 235)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(235n),
      this.instances3.alice.encrypt32(235n),
    );
    expect(res).to.equal(55225n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 3 (235, 235)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(235n),
      this.instances3.alice.encrypt32(235n),
    );
    expect(res).to.equal(55225n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 4 (235, 235)', async function () {
    const res = await this.contract3.mul_euint16_euint32(
      this.instances3.alice.encrypt16(235n),
      this.instances3.alice.encrypt32(235n),
    );
    expect(res).to.equal(55225n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (31764, 719791208)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(31764n),
      this.instances3.alice.encrypt32(719791208n),
    );
    expect(res).to.equal(9216n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (31760, 31764)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(31760n),
      this.instances3.alice.encrypt32(31764n),
    );
    expect(res).to.equal(31760n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (31764, 31764)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(31764n),
      this.instances3.alice.encrypt32(31764n),
    );
    expect(res).to.equal(31764n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (31764, 31760)', async function () {
    const res = await this.contract3.and_euint16_euint32(
      this.instances3.alice.encrypt16(31764n),
      this.instances3.alice.encrypt32(31760n),
    );
    expect(res).to.equal(31760n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (47569, 1867557342)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(47569n),
      this.instances3.alice.encrypt32(1867557342n),
    );
    expect(res).to.equal(1867561439n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (47565, 47569)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(47565n),
      this.instances3.alice.encrypt32(47569n),
    );
    expect(res).to.equal(47581n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 3 (47569, 47569)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(47569n),
      this.instances3.alice.encrypt32(47569n),
    );
    expect(res).to.equal(47569n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 4 (47569, 47565)', async function () {
    const res = await this.contract3.or_euint16_euint32(
      this.instances3.alice.encrypt16(47569n),
      this.instances3.alice.encrypt32(47565n),
    );
    expect(res).to.equal(47581n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 1 (49958, 2256953798)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(49958n),
      this.instances3.alice.encrypt32(2256953798n),
    );
    expect(res).to.equal(2256970464n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 2 (49954, 49958)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(49954n),
      this.instances3.alice.encrypt32(49958n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 3 (49958, 49958)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(49958n),
      this.instances3.alice.encrypt32(49958n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 4 (49958, 49954)', async function () {
    const res = await this.contract3.xor_euint16_euint32(
      this.instances3.alice.encrypt16(49958n),
      this.instances3.alice.encrypt32(49954n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (26213, 2142712247)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(26213n),
      this.instances3.alice.encrypt32(2142712247n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (26209, 26213)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(26209n),
      this.instances3.alice.encrypt32(26213n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 3 (26213, 26213)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(26213n),
      this.instances3.alice.encrypt32(26213n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 4 (26213, 26209)', async function () {
    const res = await this.contract3.eq_euint16_euint32(
      this.instances3.alice.encrypt16(26213n),
      this.instances3.alice.encrypt32(26209n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 1 (52545, 2079966846)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(52545n),
      this.instances3.alice.encrypt32(2079966846n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 2 (52541, 52545)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(52541n),
      this.instances3.alice.encrypt32(52545n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 3 (52545, 52545)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(52545n),
      this.instances3.alice.encrypt32(52545n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 4 (52545, 52541)', async function () {
    const res = await this.contract3.ne_euint16_euint32(
      this.instances3.alice.encrypt16(52545n),
      this.instances3.alice.encrypt32(52541n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (19364, 2908406445)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(19364n),
      this.instances3.alice.encrypt32(2908406445n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (19360, 19364)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(19360n),
      this.instances3.alice.encrypt32(19364n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (19364, 19364)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(19364n),
      this.instances3.alice.encrypt32(19364n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 4 (19364, 19360)', async function () {
    const res = await this.contract3.ge_euint16_euint32(
      this.instances3.alice.encrypt16(19364n),
      this.instances3.alice.encrypt32(19360n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 1 (61440, 2523716364)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(61440n),
      this.instances3.alice.encrypt32(2523716364n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 2 (61436, 61440)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(61436n),
      this.instances3.alice.encrypt32(61440n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 3 (61440, 61440)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(61440n),
      this.instances3.alice.encrypt32(61440n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 4 (61440, 61436)', async function () {
    const res = await this.contract3.gt_euint16_euint32(
      this.instances3.alice.encrypt16(61440n),
      this.instances3.alice.encrypt32(61436n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (42248, 3250361677)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(42248n),
      this.instances3.alice.encrypt32(3250361677n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (42244, 42248)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(42244n),
      this.instances3.alice.encrypt32(42248n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (42248, 42248)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(42248n),
      this.instances3.alice.encrypt32(42248n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 4 (42248, 42244)', async function () {
    const res = await this.contract3.le_euint16_euint32(
      this.instances3.alice.encrypt16(42248n),
      this.instances3.alice.encrypt32(42244n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 1 (45430, 522886914)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(45430n),
      this.instances3.alice.encrypt32(522886914n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 2 (45426, 45430)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(45426n),
      this.instances3.alice.encrypt32(45430n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 3 (45430, 45430)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(45430n),
      this.instances3.alice.encrypt32(45430n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 4 (45430, 45426)', async function () {
    const res = await this.contract3.lt_euint16_euint32(
      this.instances3.alice.encrypt16(45430n),
      this.instances3.alice.encrypt32(45426n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 1 (3581, 3061947173)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(3581n),
      this.instances3.alice.encrypt32(3061947173n),
    );
    expect(res).to.equal(3581n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 2 (3577, 3581)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(3577n),
      this.instances3.alice.encrypt32(3581n),
    );
    expect(res).to.equal(3577n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 3 (3581, 3581)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(3581n),
      this.instances3.alice.encrypt32(3581n),
    );
    expect(res).to.equal(3581n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 4 (3581, 3577)', async function () {
    const res = await this.contract3.min_euint16_euint32(
      this.instances3.alice.encrypt16(3581n),
      this.instances3.alice.encrypt32(3577n),
    );
    expect(res).to.equal(3577n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (50723, 1432053137)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(50723n),
      this.instances3.alice.encrypt32(1432053137n),
    );
    expect(res).to.equal(1432053137n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (50719, 50723)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(50719n),
      this.instances3.alice.encrypt32(50723n),
    );
    expect(res).to.equal(50723n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (50723, 50723)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(50723n),
      this.instances3.alice.encrypt32(50723n),
    );
    expect(res).to.equal(50723n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 4 (50723, 50719)', async function () {
    const res = await this.contract3.max_euint16_euint32(
      this.instances3.alice.encrypt16(50723n),
      this.instances3.alice.encrypt32(50719n),
    );
    expect(res).to.equal(50723n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 1 (2, 65518)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(2n),
      this.instances3.alice.encrypt64(65518n),
    );
    expect(res).to.equal(65520n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 2 (22742, 22744)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(22742n),
      this.instances3.alice.encrypt64(22744n),
    );
    expect(res).to.equal(45486n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 3 (22744, 22744)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(22744n),
      this.instances3.alice.encrypt64(22744n),
    );
    expect(res).to.equal(45488n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 4 (22744, 22742)', async function () {
    const res = await this.contract3.add_euint16_euint64(
      this.instances3.alice.encrypt16(22744n),
      this.instances3.alice.encrypt64(22742n),
    );
    expect(res).to.equal(45486n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 1 (8385, 8385)', async function () {
    const res = await this.contract3.sub_euint16_euint64(
      this.instances3.alice.encrypt16(8385n),
      this.instances3.alice.encrypt64(8385n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 2 (8385, 8381)', async function () {
    const res = await this.contract3.sub_euint16_euint64(
      this.instances3.alice.encrypt16(8385n),
      this.instances3.alice.encrypt64(8381n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 1 (2, 32761)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(2n),
      this.instances3.alice.encrypt64(32761n),
    );
    expect(res).to.equal(65522n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 2 (251, 251)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(251n),
      this.instances3.alice.encrypt64(251n),
    );
    expect(res).to.equal(63001n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 3 (251, 251)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(251n),
      this.instances3.alice.encrypt64(251n),
    );
    expect(res).to.equal(63001n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 4 (251, 251)', async function () {
    const res = await this.contract3.mul_euint16_euint64(
      this.instances3.alice.encrypt16(251n),
      this.instances3.alice.encrypt64(251n),
    );
    expect(res).to.equal(63001n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 1 (3617, 18445521461849474399)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(3617n),
      this.instances3.alice.encrypt64(18445521461849474399n),
    );
    expect(res).to.equal(1025n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 2 (3613, 3617)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(3613n),
      this.instances3.alice.encrypt64(3617n),
    );
    expect(res).to.equal(3585n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 3 (3617, 3617)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(3617n),
      this.instances3.alice.encrypt64(3617n),
    );
    expect(res).to.equal(3617n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 4 (3617, 3613)', async function () {
    const res = await this.contract3.and_euint16_euint64(
      this.instances3.alice.encrypt16(3617n),
      this.instances3.alice.encrypt64(3613n),
    );
    expect(res).to.equal(3585n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 1 (3594, 18443975303513898329)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(3594n),
      this.instances3.alice.encrypt64(18443975303513898329n),
    );
    expect(res).to.equal(18443975303513898843n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 2 (3590, 3594)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(3590n),
      this.instances3.alice.encrypt64(3594n),
    );
    expect(res).to.equal(3598n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 3 (3594, 3594)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(3594n),
      this.instances3.alice.encrypt64(3594n),
    );
    expect(res).to.equal(3594n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 4 (3594, 3590)', async function () {
    const res = await this.contract3.or_euint16_euint64(
      this.instances3.alice.encrypt16(3594n),
      this.instances3.alice.encrypt64(3590n),
    );
    expect(res).to.equal(3598n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 1 (31158, 18442847100019644985)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(31158n),
      this.instances3.alice.encrypt64(18442847100019644985n),
    );
    expect(res).to.equal(18442847100019663759n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 2 (31154, 31158)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(31154n),
      this.instances3.alice.encrypt64(31158n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 3 (31158, 31158)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(31158n),
      this.instances3.alice.encrypt64(31158n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 4 (31158, 31154)', async function () {
    const res = await this.contract3.xor_euint16_euint64(
      this.instances3.alice.encrypt16(31158n),
      this.instances3.alice.encrypt64(31154n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 1 (38256, 18442430973530036421)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(38256n),
      this.instances3.alice.encrypt64(18442430973530036421n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 2 (38252, 38256)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(38252n),
      this.instances3.alice.encrypt64(38256n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 3 (38256, 38256)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(38256n),
      this.instances3.alice.encrypt64(38256n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 4 (38256, 38252)', async function () {
    const res = await this.contract3.eq_euint16_euint64(
      this.instances3.alice.encrypt16(38256n),
      this.instances3.alice.encrypt64(38252n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 1 (33238, 18440140341099634203)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(33238n),
      this.instances3.alice.encrypt64(18440140341099634203n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 2 (33234, 33238)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(33234n),
      this.instances3.alice.encrypt64(33238n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 3 (33238, 33238)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(33238n),
      this.instances3.alice.encrypt64(33238n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 4 (33238, 33234)', async function () {
    const res = await this.contract3.ne_euint16_euint64(
      this.instances3.alice.encrypt16(33238n),
      this.instances3.alice.encrypt64(33234n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 1 (17006, 18440111639904808773)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(17006n),
      this.instances3.alice.encrypt64(18440111639904808773n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 2 (17002, 17006)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(17002n),
      this.instances3.alice.encrypt64(17006n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 3 (17006, 17006)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(17006n),
      this.instances3.alice.encrypt64(17006n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 4 (17006, 17002)', async function () {
    const res = await this.contract3.ge_euint16_euint64(
      this.instances3.alice.encrypt16(17006n),
      this.instances3.alice.encrypt64(17002n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 1 (13915, 18437957835114177845)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(13915n),
      this.instances3.alice.encrypt64(18437957835114177845n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 2 (13911, 13915)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(13911n),
      this.instances3.alice.encrypt64(13915n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 3 (13915, 13915)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(13915n),
      this.instances3.alice.encrypt64(13915n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 4 (13915, 13911)', async function () {
    const res = await this.contract3.gt_euint16_euint64(
      this.instances3.alice.encrypt16(13915n),
      this.instances3.alice.encrypt64(13911n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 1 (16281, 18438048148950562767)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(16281n),
      this.instances3.alice.encrypt64(18438048148950562767n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 2 (16277, 16281)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(16277n),
      this.instances3.alice.encrypt64(16281n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 3 (16281, 16281)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(16281n),
      this.instances3.alice.encrypt64(16281n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 4 (16281, 16277)', async function () {
    const res = await this.contract3.le_euint16_euint64(
      this.instances3.alice.encrypt16(16281n),
      this.instances3.alice.encrypt64(16277n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 1 (18995, 18443093417222594453)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(18995n),
      this.instances3.alice.encrypt64(18443093417222594453n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 2 (18991, 18995)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(18991n),
      this.instances3.alice.encrypt64(18995n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 3 (18995, 18995)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(18995n),
      this.instances3.alice.encrypt64(18995n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 4 (18995, 18991)', async function () {
    const res = await this.contract3.lt_euint16_euint64(
      this.instances3.alice.encrypt16(18995n),
      this.instances3.alice.encrypt64(18991n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 1 (60945, 18442404212044065569)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(60945n),
      this.instances3.alice.encrypt64(18442404212044065569n),
    );
    expect(res).to.equal(60945n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 2 (60941, 60945)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(60941n),
      this.instances3.alice.encrypt64(60945n),
    );
    expect(res).to.equal(60941n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 3 (60945, 60945)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(60945n),
      this.instances3.alice.encrypt64(60945n),
    );
    expect(res).to.equal(60945n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 4 (60945, 60941)', async function () {
    const res = await this.contract3.min_euint16_euint64(
      this.instances3.alice.encrypt16(60945n),
      this.instances3.alice.encrypt64(60941n),
    );
    expect(res).to.equal(60941n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 1 (23644, 18446414081706845657)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(23644n),
      this.instances3.alice.encrypt64(18446414081706845657n),
    );
    expect(res).to.equal(18446414081706845657n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 2 (23640, 23644)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(23640n),
      this.instances3.alice.encrypt64(23644n),
    );
    expect(res).to.equal(23644n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 3 (23644, 23644)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(23644n),
      this.instances3.alice.encrypt64(23644n),
    );
    expect(res).to.equal(23644n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 4 (23644, 23640)', async function () {
    const res = await this.contract3.max_euint16_euint64(
      this.instances3.alice.encrypt16(23644n),
      this.instances3.alice.encrypt64(23640n),
    );
    expect(res).to.equal(23644n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (54451, 6303)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(54451n), 6303);
    expect(res).to.equal(60754n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (22056, 22058)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(22056n), 22058);
    expect(res).to.equal(44114n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (22058, 22058)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(22058n), 22058);
    expect(res).to.equal(44116n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (22058, 22056)', async function () {
    const res = await this.contract3.add_euint16_uint16(this.instances3.alice.encrypt16(22058n), 22056);
    expect(res).to.equal(44114n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (29751, 3152)', async function () {
    const res = await this.contract3.add_uint16_euint16(29751, this.instances3.alice.encrypt16(3152n));
    expect(res).to.equal(32903n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 2 (22056, 22058)', async function () {
    const res = await this.contract3.add_uint16_euint16(22056, this.instances3.alice.encrypt16(22058n));
    expect(res).to.equal(44114n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 3 (22058, 22058)', async function () {
    const res = await this.contract3.add_uint16_euint16(22058, this.instances3.alice.encrypt16(22058n));
    expect(res).to.equal(44116n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 4 (22058, 22056)', async function () {
    const res = await this.contract3.add_uint16_euint16(22058, this.instances3.alice.encrypt16(22056n));
    expect(res).to.equal(44114n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (56955, 56955)', async function () {
    const res = await this.contract3.sub_euint16_uint16(this.instances3.alice.encrypt16(56955n), 56955);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 2 (56955, 56951)', async function () {
    const res = await this.contract3.sub_euint16_uint16(this.instances3.alice.encrypt16(56955n), 56951);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (56955, 56955)', async function () {
    const res = await this.contract3.sub_uint16_euint16(56955, this.instances3.alice.encrypt16(56955n));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (56955, 56951)', async function () {
    const res = await this.contract3.sub_uint16_euint16(56955, this.instances3.alice.encrypt16(56951n));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (239, 131)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(239n), 131);
    expect(res).to.equal(31309n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (239, 239)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(239n), 239);
    expect(res).to.equal(57121n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (239, 239)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(239n), 239);
    expect(res).to.equal(57121n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (239, 239)', async function () {
    const res = await this.contract3.mul_euint16_uint16(this.instances3.alice.encrypt16(239n), 239);
    expect(res).to.equal(57121n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (190, 131)', async function () {
    const res = await this.contract3.mul_uint16_euint16(190, this.instances3.alice.encrypt16(131n));
    expect(res).to.equal(24890n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 2 (239, 239)', async function () {
    const res = await this.contract3.mul_uint16_euint16(239, this.instances3.alice.encrypt16(239n));
    expect(res).to.equal(57121n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 3 (239, 239)', async function () {
    const res = await this.contract3.mul_uint16_euint16(239, this.instances3.alice.encrypt16(239n));
    expect(res).to.equal(57121n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 4 (239, 239)', async function () {
    const res = await this.contract3.mul_uint16_euint16(239, this.instances3.alice.encrypt16(239n));
    expect(res).to.equal(57121n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (61139, 60988)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(61139n), 60988);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (59165, 59169)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(59165n), 59169);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (59169, 59169)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(59169n), 59169);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (59169, 59165)', async function () {
    const res = await this.contract3.div_euint16_uint16(this.instances3.alice.encrypt16(59169n), 59165);
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (18856, 18330)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(18856n), 18330);
    expect(res).to.equal(526n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 2 (18852, 18856)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(18852n), 18856);
    expect(res).to.equal(18852n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 3 (18856, 18856)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(18856n), 18856);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 4 (18856, 18852)', async function () {
    const res = await this.contract3.rem_euint16_uint16(this.instances3.alice.encrypt16(18856n), 18852);
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (53936, 62296)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(53936n), 62296);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (53932, 53936)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(53932n), 53936);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (53936, 53936)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(53936n), 53936);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (53936, 53932)', async function () {
    const res = await this.contract3.eq_euint16_uint16(this.instances3.alice.encrypt16(53936n), 53932);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (49783, 62296)', async function () {
    const res = await this.contract3.eq_uint16_euint16(49783, this.instances3.alice.encrypt16(62296n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (53932, 53936)', async function () {
    const res = await this.contract3.eq_uint16_euint16(53932, this.instances3.alice.encrypt16(53936n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 3 (53936, 53936)', async function () {
    const res = await this.contract3.eq_uint16_euint16(53936, this.instances3.alice.encrypt16(53936n));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 4 (53936, 53932)', async function () {
    const res = await this.contract3.eq_uint16_euint16(53936, this.instances3.alice.encrypt16(53932n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (64200, 15145)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(64200n), 15145);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (17034, 17038)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(17034n), 17038);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 3 (17038, 17038)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(17038n), 17038);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 4 (17038, 17034)', async function () {
    const res = await this.contract3.ne_euint16_uint16(this.instances3.alice.encrypt16(17038n), 17034);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (35214, 15145)', async function () {
    const res = await this.contract3.ne_uint16_euint16(35214, this.instances3.alice.encrypt16(15145n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (17034, 17038)', async function () {
    const res = await this.contract3.ne_uint16_euint16(17034, this.instances3.alice.encrypt16(17038n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 3 (17038, 17038)', async function () {
    const res = await this.contract3.ne_uint16_euint16(17038, this.instances3.alice.encrypt16(17038n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 4 (17038, 17034)', async function () {
    const res = await this.contract3.ne_uint16_euint16(17038, this.instances3.alice.encrypt16(17034n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (18226, 26413)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(18226n), 26413);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (11813, 11817)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(11813n), 11817);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (11817, 11817)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(11817n), 11817);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 4 (11817, 11813)', async function () {
    const res = await this.contract3.ge_euint16_uint16(this.instances3.alice.encrypt16(11817n), 11813);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (45131, 26413)', async function () {
    const res = await this.contract3.ge_uint16_euint16(45131, this.instances3.alice.encrypt16(26413n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (11813, 11817)', async function () {
    const res = await this.contract3.ge_uint16_euint16(11813, this.instances3.alice.encrypt16(11817n));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (11817, 11817)', async function () {
    const res = await this.contract3.ge_uint16_euint16(11817, this.instances3.alice.encrypt16(11817n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 4 (11817, 11813)', async function () {
    const res = await this.contract3.ge_uint16_euint16(11817, this.instances3.alice.encrypt16(11813n));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (46712, 65342)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(46712n), 65342);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (46708, 46712)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(46708n), 46712);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (46712, 46712)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(46712n), 46712);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 4 (46712, 46708)', async function () {
    const res = await this.contract3.gt_euint16_uint16(this.instances3.alice.encrypt16(46712n), 46708);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (58255, 65342)', async function () {
    const res = await this.contract3.gt_uint16_euint16(58255, this.instances3.alice.encrypt16(65342n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (46708, 46712)', async function () {
    const res = await this.contract3.gt_uint16_euint16(46708, this.instances3.alice.encrypt16(46712n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (46712, 46712)', async function () {
    const res = await this.contract3.gt_uint16_euint16(46712, this.instances3.alice.encrypt16(46712n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 4 (46712, 46708)', async function () {
    const res = await this.contract3.gt_uint16_euint16(46712, this.instances3.alice.encrypt16(46708n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (9281, 23936)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(9281n), 23936);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (9277, 9281)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(9277n), 9281);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (9281, 9281)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(9281n), 9281);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 4 (9281, 9277)', async function () {
    const res = await this.contract3.le_euint16_uint16(this.instances3.alice.encrypt16(9281n), 9277);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (49675, 23936)', async function () {
    const res = await this.contract3.le_uint16_euint16(49675, this.instances3.alice.encrypt16(23936n));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (9277, 9281)', async function () {
    const res = await this.contract3.le_uint16_euint16(9277, this.instances3.alice.encrypt16(9281n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (9281, 9281)', async function () {
    const res = await this.contract3.le_uint16_euint16(9281, this.instances3.alice.encrypt16(9281n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (9281, 9277)', async function () {
    const res = await this.contract3.le_uint16_euint16(9281, this.instances3.alice.encrypt16(9277n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (44794, 29106)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(44794n), 29106);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (23286, 23290)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(23286n), 23290);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (23290, 23290)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(23290n), 23290);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 4 (23290, 23286)', async function () {
    const res = await this.contract3.lt_euint16_uint16(this.instances3.alice.encrypt16(23290n), 23286);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (11942, 29106)', async function () {
    const res = await this.contract3.lt_uint16_euint16(11942, this.instances3.alice.encrypt16(29106n));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (23286, 23290)', async function () {
    const res = await this.contract3.lt_uint16_euint16(23286, this.instances3.alice.encrypt16(23290n));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (23290, 23290)', async function () {
    const res = await this.contract3.lt_uint16_euint16(23290, this.instances3.alice.encrypt16(23290n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 4 (23290, 23286)', async function () {
    const res = await this.contract3.lt_uint16_euint16(23290, this.instances3.alice.encrypt16(23286n));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (30936, 64470)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(30936n), 64470);
    expect(res).to.equal(30936n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (9023, 9027)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(9023n), 9027);
    expect(res).to.equal(9023n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (9027, 9027)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(9027n), 9027);
    expect(res).to.equal(9027n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 4 (9027, 9023)', async function () {
    const res = await this.contract3.min_euint16_uint16(this.instances3.alice.encrypt16(9027n), 9023);
    expect(res).to.equal(9023n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (10499, 64470)', async function () {
    const res = await this.contract3.min_uint16_euint16(10499, this.instances3.alice.encrypt16(64470n));
    expect(res).to.equal(10499n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (9023, 9027)', async function () {
    const res = await this.contract3.min_uint16_euint16(9023, this.instances3.alice.encrypt16(9027n));
    expect(res).to.equal(9023n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (9027, 9027)', async function () {
    const res = await this.contract3.min_uint16_euint16(9027, this.instances3.alice.encrypt16(9027n));
    expect(res).to.equal(9027n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (9027, 9023)', async function () {
    const res = await this.contract3.min_uint16_euint16(9027, this.instances3.alice.encrypt16(9023n));
    expect(res).to.equal(9023n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (34561, 63895)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(34561n), 63895);
    expect(res).to.equal(63895n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (34557, 34561)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(34557n), 34561);
    expect(res).to.equal(34561n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (34561, 34561)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(34561n), 34561);
    expect(res).to.equal(34561n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (34561, 34557)', async function () {
    const res = await this.contract3.max_euint16_uint16(this.instances3.alice.encrypt16(34561n), 34557);
    expect(res).to.equal(34561n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (52784, 63895)', async function () {
    const res = await this.contract3.max_uint16_euint16(52784, this.instances3.alice.encrypt16(63895n));
    expect(res).to.equal(63895n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (34557, 34561)', async function () {
    const res = await this.contract3.max_uint16_euint16(34557, this.instances3.alice.encrypt16(34561n));
    expect(res).to.equal(34561n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (34561, 34561)', async function () {
    const res = await this.contract3.max_uint16_euint16(34561, this.instances3.alice.encrypt16(34561n));
    expect(res).to.equal(34561n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (34561, 34557)', async function () {
    const res = await this.contract3.max_uint16_euint16(34561, this.instances3.alice.encrypt16(34557n));
    expect(res).to.equal(34561n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 1 (11, 2)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(11n),
      this.instances3.alice.encrypt4(2n),
    );
    expect(res).to.equal(13n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(4n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 3 (5, 5)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(5n),
      this.instances3.alice.encrypt4(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.add_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(4n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint32, euint4) => euint32 test 1 (8, 8)', async function () {
    const res = await this.contract3.sub_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint4) => euint32 test 2 (8, 4)', async function () {
    const res = await this.contract3.sub_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 1 (5, 2)', async function () {
    const res = await this.contract3.mul_euint32_euint4(
      this.instances3.alice.encrypt32(5n),
      this.instances3.alice.encrypt4(2n),
    );
    expect(res).to.equal(10n);
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

  it('test operator "and" overload (euint32, euint4) => euint32 test 1 (1301921086, 9)', async function () {
    const res = await this.contract3.and_euint32_euint4(
      this.instances3.alice.encrypt32(1301921086n),
      this.instances3.alice.encrypt4(9n),
    );
    expect(res).to.equal(8n);
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

  it('test operator "or" overload (euint32, euint4) => euint32 test 1 (2406735968, 5)', async function () {
    const res = await this.contract3.or_euint32_euint4(
      this.instances3.alice.encrypt32(2406735968n),
      this.instances3.alice.encrypt4(5n),
    );
    expect(res).to.equal(2406735973n);
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

  it('test operator "xor" overload (euint32, euint4) => euint32 test 1 (3625477729, 13)', async function () {
    const res = await this.contract3.xor_euint32_euint4(
      this.instances3.alice.encrypt32(3625477729n),
      this.instances3.alice.encrypt4(13n),
    );
    expect(res).to.equal(3625477740n);
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

  it('test operator "eq" overload (euint32, euint4) => ebool test 1 (2724048740, 14)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(2724048740n),
      this.instances3.alice.encrypt4(14n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(10n),
      this.instances3.alice.encrypt4(14n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(14n),
      this.instances3.alice.encrypt4(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract3.eq_euint32_euint4(
      this.instances3.alice.encrypt32(14n),
      this.instances3.alice.encrypt4(10n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 1 (2506657954, 7)', async function () {
    const res = await this.contract3.ne_euint32_euint4(
      this.instances3.alice.encrypt32(2506657954n),
      this.instances3.alice.encrypt4(7n),
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

  it('test operator "ge" overload (euint32, euint4) => ebool test 1 (4064050662, 13)', async function () {
    const res = await this.contract3.ge_euint32_euint4(
      this.instances3.alice.encrypt32(4064050662n),
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

  it('test operator "gt" overload (euint32, euint4) => ebool test 1 (2933730621, 7)', async function () {
    const res = await this.contract3.gt_euint32_euint4(
      this.instances3.alice.encrypt32(2933730621n),
      this.instances3.alice.encrypt4(7n),
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

  it('test operator "le" overload (euint32, euint4) => ebool test 1 (4064031115, 9)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(4064031115n),
      this.instances3.alice.encrypt4(9n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 2 (5, 9)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(5n),
      this.instances3.alice.encrypt4(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 3 (9, 9)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(9n),
      this.instances3.alice.encrypt4(9n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 4 (9, 5)', async function () {
    const res = await this.contract3.le_euint32_euint4(
      this.instances3.alice.encrypt32(9n),
      this.instances3.alice.encrypt4(5n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 1 (3623618060, 5)', async function () {
    const res = await this.contract3.lt_euint32_euint4(
      this.instances3.alice.encrypt32(3623618060n),
      this.instances3.alice.encrypt4(5n),
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

  it('test operator "min" overload (euint32, euint4) => euint32 test 1 (3192029980, 3)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(3192029980n),
      this.instances3.alice.encrypt4(3n),
    );
    expect(res).to.equal(3n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(4n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract3.min_euint32_euint4(
      this.instances3.alice.encrypt32(8n),
      this.instances3.alice.encrypt4(4n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 1 (681402629, 7)', async function () {
    const res = await this.contract3.max_euint32_euint4(
      this.instances3.alice.encrypt32(681402629n),
      this.instances3.alice.encrypt4(7n),
    );
    expect(res).to.equal(681402629n);
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

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (186, 2)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(186n),
      this.instances3.alice.encrypt8(2n),
    );
    expect(res).to.equal(188n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (69, 71)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(69n),
      this.instances3.alice.encrypt8(71n),
    );
    expect(res).to.equal(140n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (71, 71)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(71n),
      this.instances3.alice.encrypt8(71n),
    );
    expect(res).to.equal(142n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (71, 69)', async function () {
    const res = await this.contract3.add_euint32_euint8(
      this.instances3.alice.encrypt32(71n),
      this.instances3.alice.encrypt8(69n),
    );
    expect(res).to.equal(140n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (179, 179)', async function () {
    const res = await this.contract3.sub_euint32_euint8(
      this.instances3.alice.encrypt32(179n),
      this.instances3.alice.encrypt8(179n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (179, 175)', async function () {
    const res = await this.contract3.sub_euint32_euint8(
      this.instances3.alice.encrypt32(179n),
      this.instances3.alice.encrypt8(175n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (107, 2)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(107n),
      this.instances3.alice.encrypt8(2n),
    );
    expect(res).to.equal(214n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 2 (12, 12)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(12n),
      this.instances3.alice.encrypt8(12n),
    );
    expect(res).to.equal(144n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 3 (12, 12)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(12n),
      this.instances3.alice.encrypt8(12n),
    );
    expect(res).to.equal(144n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 4 (12, 12)', async function () {
    const res = await this.contract3.mul_euint32_euint8(
      this.instances3.alice.encrypt32(12n),
      this.instances3.alice.encrypt8(12n),
    );
    expect(res).to.equal(144n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (2684075699, 16)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(2684075699n),
      this.instances3.alice.encrypt8(16n),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (12, 16)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(12n),
      this.instances3.alice.encrypt8(16n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (16, 16)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(16n),
      this.instances3.alice.encrypt8(16n),
    );
    expect(res).to.equal(16n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (16, 12)', async function () {
    const res = await this.contract3.and_euint32_euint8(
      this.instances3.alice.encrypt32(16n),
      this.instances3.alice.encrypt8(12n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (3961011805, 118)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(3961011805n),
      this.instances4.alice.encrypt8(118n),
    );
    expect(res).to.equal(3961011839n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (114, 118)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(114n),
      this.instances4.alice.encrypt8(118n),
    );
    expect(res).to.equal(118n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (118, 118)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(118n),
      this.instances4.alice.encrypt8(118n),
    );
    expect(res).to.equal(118n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (118, 114)', async function () {
    const res = await this.contract4.or_euint32_euint8(
      this.instances4.alice.encrypt32(118n),
      this.instances4.alice.encrypt8(114n),
    );
    expect(res).to.equal(118n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (2615810877, 184)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(2615810877n),
      this.instances4.alice.encrypt8(184n),
    );
    expect(res).to.equal(2615810949n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (180, 184)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(180n),
      this.instances4.alice.encrypt8(184n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (184, 184)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(184n),
      this.instances4.alice.encrypt8(184n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (184, 180)', async function () {
    const res = await this.contract4.xor_euint32_euint8(
      this.instances4.alice.encrypt32(184n),
      this.instances4.alice.encrypt8(180n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (2890164202, 10)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(2890164202n),
      this.instances4.alice.encrypt8(10n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(6n),
      this.instances4.alice.encrypt8(10n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(10n),
      this.instances4.alice.encrypt8(10n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract4.eq_euint32_euint8(
      this.instances4.alice.encrypt32(10n),
      this.instances4.alice.encrypt8(6n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (874720156, 10)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(874720156n),
      this.instances4.alice.encrypt8(10n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (6, 10)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(6n),
      this.instances4.alice.encrypt8(10n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 3 (10, 10)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(10n),
      this.instances4.alice.encrypt8(10n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 4 (10, 6)', async function () {
    const res = await this.contract4.ne_euint32_euint8(
      this.instances4.alice.encrypt32(10n),
      this.instances4.alice.encrypt8(6n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (4107105674, 147)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(4107105674n),
      this.instances4.alice.encrypt8(147n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (143, 147)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(143n),
      this.instances4.alice.encrypt8(147n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (147, 147)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(147n),
      this.instances4.alice.encrypt8(147n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 4 (147, 143)', async function () {
    const res = await this.contract4.ge_euint32_euint8(
      this.instances4.alice.encrypt32(147n),
      this.instances4.alice.encrypt8(143n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (1917780828, 163)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(1917780828n),
      this.instances4.alice.encrypt8(163n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (159, 163)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(159n),
      this.instances4.alice.encrypt8(163n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (163, 163)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(163n),
      this.instances4.alice.encrypt8(163n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 4 (163, 159)', async function () {
    const res = await this.contract4.gt_euint32_euint8(
      this.instances4.alice.encrypt32(163n),
      this.instances4.alice.encrypt8(159n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (3743657855, 103)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(3743657855n),
      this.instances4.alice.encrypt8(103n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (99, 103)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(99n),
      this.instances4.alice.encrypt8(103n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (103, 103)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(103n),
      this.instances4.alice.encrypt8(103n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (103, 99)', async function () {
    const res = await this.contract4.le_euint32_euint8(
      this.instances4.alice.encrypt32(103n),
      this.instances4.alice.encrypt8(99n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (800790361, 82)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(800790361n),
      this.instances4.alice.encrypt8(82n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (78, 82)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(78n),
      this.instances4.alice.encrypt8(82n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (82, 82)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(82n),
      this.instances4.alice.encrypt8(82n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 4 (82, 78)', async function () {
    const res = await this.contract4.lt_euint32_euint8(
      this.instances4.alice.encrypt32(82n),
      this.instances4.alice.encrypt8(78n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (2128865754, 127)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(2128865754n),
      this.instances4.alice.encrypt8(127n),
    );
    expect(res).to.equal(127n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (123, 127)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(123n),
      this.instances4.alice.encrypt8(127n),
    );
    expect(res).to.equal(123n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (127, 127)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(127n),
      this.instances4.alice.encrypt8(127n),
    );
    expect(res).to.equal(127n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (127, 123)', async function () {
    const res = await this.contract4.min_euint32_euint8(
      this.instances4.alice.encrypt32(127n),
      this.instances4.alice.encrypt8(123n),
    );
    expect(res).to.equal(123n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (1610798197, 160)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(1610798197n),
      this.instances4.alice.encrypt8(160n),
    );
    expect(res).to.equal(1610798197n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (156, 160)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(156n),
      this.instances4.alice.encrypt8(160n),
    );
    expect(res).to.equal(160n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (160, 160)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(160n),
      this.instances4.alice.encrypt8(160n),
    );
    expect(res).to.equal(160n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 4 (160, 156)', async function () {
    const res = await this.contract4.max_euint32_euint8(
      this.instances4.alice.encrypt32(160n),
      this.instances4.alice.encrypt8(156n),
    );
    expect(res).to.equal(160n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (40687, 4)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(40687n),
      this.instances4.alice.encrypt16(4n),
    );
    expect(res).to.equal(40691n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (18607, 18611)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(18607n),
      this.instances4.alice.encrypt16(18611n),
    );
    expect(res).to.equal(37218n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (18611, 18611)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(18611n),
      this.instances4.alice.encrypt16(18611n),
    );
    expect(res).to.equal(37222n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (18611, 18607)', async function () {
    const res = await this.contract4.add_euint32_euint16(
      this.instances4.alice.encrypt32(18611n),
      this.instances4.alice.encrypt16(18607n),
    );
    expect(res).to.equal(37218n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (47991, 47991)', async function () {
    const res = await this.contract4.sub_euint32_euint16(
      this.instances4.alice.encrypt32(47991n),
      this.instances4.alice.encrypt16(47991n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (47991, 47987)', async function () {
    const res = await this.contract4.sub_euint32_euint16(
      this.instances4.alice.encrypt32(47991n),
      this.instances4.alice.encrypt16(47987n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (30409, 2)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(30409n),
      this.instances4.alice.encrypt16(2n),
    );
    expect(res).to.equal(60818n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (152, 152)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(152n),
      this.instances4.alice.encrypt16(152n),
    );
    expect(res).to.equal(23104n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (152, 152)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(152n),
      this.instances4.alice.encrypt16(152n),
    );
    expect(res).to.equal(23104n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (152, 152)', async function () {
    const res = await this.contract4.mul_euint32_euint16(
      this.instances4.alice.encrypt32(152n),
      this.instances4.alice.encrypt16(152n),
    );
    expect(res).to.equal(23104n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (1113519013, 16851)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(1113519013n),
      this.instances4.alice.encrypt16(16851n),
    );
    expect(res).to.equal(16769n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (16847, 16851)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(16847n),
      this.instances4.alice.encrypt16(16851n),
    );
    expect(res).to.equal(16835n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (16851, 16851)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(16851n),
      this.instances4.alice.encrypt16(16851n),
    );
    expect(res).to.equal(16851n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (16851, 16847)', async function () {
    const res = await this.contract4.and_euint32_euint16(
      this.instances4.alice.encrypt32(16851n),
      this.instances4.alice.encrypt16(16847n),
    );
    expect(res).to.equal(16835n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (2441286673, 26427)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(2441286673n),
      this.instances4.alice.encrypt16(26427n),
    );
    expect(res).to.equal(2441312059n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (26423, 26427)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(26423n),
      this.instances4.alice.encrypt16(26427n),
    );
    expect(res).to.equal(26431n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 3 (26427, 26427)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(26427n),
      this.instances4.alice.encrypt16(26427n),
    );
    expect(res).to.equal(26427n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 4 (26427, 26423)', async function () {
    const res = await this.contract4.or_euint32_euint16(
      this.instances4.alice.encrypt32(26427n),
      this.instances4.alice.encrypt16(26423n),
    );
    expect(res).to.equal(26431n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (2697992454, 47573)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(2697992454n),
      this.instances4.alice.encrypt16(47573n),
    );
    expect(res).to.equal(2698027219n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (47569, 47573)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(47569n),
      this.instances4.alice.encrypt16(47573n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (47573, 47573)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(47573n),
      this.instances4.alice.encrypt16(47573n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (47573, 47569)', async function () {
    const res = await this.contract4.xor_euint32_euint16(
      this.instances4.alice.encrypt32(47573n),
      this.instances4.alice.encrypt16(47569n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (3540063980, 53142)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(3540063980n),
      this.instances4.alice.encrypt16(53142n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (53138, 53142)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(53138n),
      this.instances4.alice.encrypt16(53142n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (53142, 53142)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(53142n),
      this.instances4.alice.encrypt16(53142n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (53142, 53138)', async function () {
    const res = await this.contract4.eq_euint32_euint16(
      this.instances4.alice.encrypt32(53142n),
      this.instances4.alice.encrypt16(53138n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (2729404223, 63905)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(2729404223n),
      this.instances4.alice.encrypt16(63905n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (63901, 63905)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(63901n),
      this.instances4.alice.encrypt16(63905n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 3 (63905, 63905)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(63905n),
      this.instances4.alice.encrypt16(63905n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 4 (63905, 63901)', async function () {
    const res = await this.contract4.ne_euint32_euint16(
      this.instances4.alice.encrypt32(63905n),
      this.instances4.alice.encrypt16(63901n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (3202817970, 50829)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(3202817970n),
      this.instances4.alice.encrypt16(50829n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (50825, 50829)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(50825n),
      this.instances4.alice.encrypt16(50829n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (50829, 50829)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(50829n),
      this.instances4.alice.encrypt16(50829n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 4 (50829, 50825)', async function () {
    const res = await this.contract4.ge_euint32_euint16(
      this.instances4.alice.encrypt32(50829n),
      this.instances4.alice.encrypt16(50825n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (1221805876, 57752)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(1221805876n),
      this.instances4.alice.encrypt16(57752n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (57748, 57752)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(57748n),
      this.instances4.alice.encrypt16(57752n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (57752, 57752)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(57752n),
      this.instances4.alice.encrypt16(57752n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 4 (57752, 57748)', async function () {
    const res = await this.contract4.gt_euint32_euint16(
      this.instances4.alice.encrypt32(57752n),
      this.instances4.alice.encrypt16(57748n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (1673073080, 44621)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(1673073080n),
      this.instances4.alice.encrypt16(44621n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (44617, 44621)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(44617n),
      this.instances4.alice.encrypt16(44621n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (44621, 44621)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(44621n),
      this.instances4.alice.encrypt16(44621n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (44621, 44617)', async function () {
    const res = await this.contract4.le_euint32_euint16(
      this.instances4.alice.encrypt32(44621n),
      this.instances4.alice.encrypt16(44617n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (2528767958, 64360)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(2528767958n),
      this.instances4.alice.encrypt16(64360n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (64356, 64360)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(64356n),
      this.instances4.alice.encrypt16(64360n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (64360, 64360)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(64360n),
      this.instances4.alice.encrypt16(64360n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (64360, 64356)', async function () {
    const res = await this.contract4.lt_euint32_euint16(
      this.instances4.alice.encrypt32(64360n),
      this.instances4.alice.encrypt16(64356n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (2025213170, 65001)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(2025213170n),
      this.instances4.alice.encrypt16(65001n),
    );
    expect(res).to.equal(65001n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (64997, 65001)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(64997n),
      this.instances4.alice.encrypt16(65001n),
    );
    expect(res).to.equal(64997n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (65001, 65001)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(65001n),
      this.instances4.alice.encrypt16(65001n),
    );
    expect(res).to.equal(65001n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (65001, 64997)', async function () {
    const res = await this.contract4.min_euint32_euint16(
      this.instances4.alice.encrypt32(65001n),
      this.instances4.alice.encrypt16(64997n),
    );
    expect(res).to.equal(64997n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (4136615954, 9960)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(4136615954n),
      this.instances4.alice.encrypt16(9960n),
    );
    expect(res).to.equal(4136615954n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (9956, 9960)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(9956n),
      this.instances4.alice.encrypt16(9960n),
    );
    expect(res).to.equal(9960n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (9960, 9960)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(9960n),
      this.instances4.alice.encrypt16(9960n),
    );
    expect(res).to.equal(9960n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (9960, 9956)', async function () {
    const res = await this.contract4.max_euint32_euint16(
      this.instances4.alice.encrypt32(9960n),
      this.instances4.alice.encrypt16(9956n),
    );
    expect(res).to.equal(9960n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (2163343287, 829836787)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(2163343287n),
      this.instances4.alice.encrypt32(829836787n),
    );
    expect(res).to.equal(2993180074n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (829836783, 829836787)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(829836783n),
      this.instances4.alice.encrypt32(829836787n),
    );
    expect(res).to.equal(1659673570n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (829836787, 829836787)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(829836787n),
      this.instances4.alice.encrypt32(829836787n),
    );
    expect(res).to.equal(1659673574n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (829836787, 829836783)', async function () {
    const res = await this.contract4.add_euint32_euint32(
      this.instances4.alice.encrypt32(829836787n),
      this.instances4.alice.encrypt32(829836783n),
    );
    expect(res).to.equal(1659673570n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (3061205449, 3061205449)', async function () {
    const res = await this.contract4.sub_euint32_euint32(
      this.instances4.alice.encrypt32(3061205449n),
      this.instances4.alice.encrypt32(3061205449n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (3061205449, 3061205445)', async function () {
    const res = await this.contract4.sub_euint32_euint32(
      this.instances4.alice.encrypt32(3061205449n),
      this.instances4.alice.encrypt32(3061205445n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (42467, 41983)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(42467n),
      this.instances4.alice.encrypt32(41983n),
    );
    expect(res).to.equal(1782892061n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 2 (41983, 41983)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(41983n),
      this.instances4.alice.encrypt32(41983n),
    );
    expect(res).to.equal(1762572289n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 3 (41983, 41983)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(41983n),
      this.instances4.alice.encrypt32(41983n),
    );
    expect(res).to.equal(1762572289n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 4 (41983, 41983)', async function () {
    const res = await this.contract4.mul_euint32_euint32(
      this.instances4.alice.encrypt32(41983n),
      this.instances4.alice.encrypt32(41983n),
    );
    expect(res).to.equal(1762572289n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (749010015, 1458345079)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(749010015n),
      this.instances4.alice.encrypt32(1458345079n),
    );
    expect(res).to.equal(77894743n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (749010011, 749010015)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(749010011n),
      this.instances4.alice.encrypt32(749010015n),
    );
    expect(res).to.equal(749010011n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 3 (749010015, 749010015)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(749010015n),
      this.instances4.alice.encrypt32(749010015n),
    );
    expect(res).to.equal(749010015n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 4 (749010015, 749010011)', async function () {
    const res = await this.contract4.and_euint32_euint32(
      this.instances4.alice.encrypt32(749010015n),
      this.instances4.alice.encrypt32(749010011n),
    );
    expect(res).to.equal(749010011n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (183558137, 3592149879)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(183558137n),
      this.instances4.alice.encrypt32(3592149879n),
    );
    expect(res).to.equal(3741048831n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (183558133, 183558137)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(183558133n),
      this.instances4.alice.encrypt32(183558137n),
    );
    expect(res).to.equal(183558141n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 3 (183558137, 183558137)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(183558137n),
      this.instances4.alice.encrypt32(183558137n),
    );
    expect(res).to.equal(183558137n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 4 (183558137, 183558133)', async function () {
    const res = await this.contract4.or_euint32_euint32(
      this.instances4.alice.encrypt32(183558137n),
      this.instances4.alice.encrypt32(183558133n),
    );
    expect(res).to.equal(183558141n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (3727740302, 3007364521)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(3727740302n),
      this.instances4.alice.encrypt32(3007364521n),
    );
    expect(res).to.equal(1836085287n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (3007364517, 3007364521)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(3007364517n),
      this.instances4.alice.encrypt32(3007364521n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (3007364521, 3007364521)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(3007364521n),
      this.instances4.alice.encrypt32(3007364521n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (3007364521, 3007364517)', async function () {
    const res = await this.contract4.xor_euint32_euint32(
      this.instances4.alice.encrypt32(3007364521n),
      this.instances4.alice.encrypt32(3007364517n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (3674813327, 3173886291)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(3674813327n),
      this.instances4.alice.encrypt32(3173886291n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (3173886287, 3173886291)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(3173886287n),
      this.instances4.alice.encrypt32(3173886291n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (3173886291, 3173886291)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(3173886291n),
      this.instances4.alice.encrypt32(3173886291n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (3173886291, 3173886287)', async function () {
    const res = await this.contract4.eq_euint32_euint32(
      this.instances4.alice.encrypt32(3173886291n),
      this.instances4.alice.encrypt32(3173886287n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (40863923, 1726230330)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(40863923n),
      this.instances4.alice.encrypt32(1726230330n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (40863919, 40863923)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(40863919n),
      this.instances4.alice.encrypt32(40863923n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 3 (40863923, 40863923)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(40863923n),
      this.instances4.alice.encrypt32(40863923n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 4 (40863923, 40863919)', async function () {
    const res = await this.contract4.ne_euint32_euint32(
      this.instances4.alice.encrypt32(40863923n),
      this.instances4.alice.encrypt32(40863919n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (1970784794, 1596956634)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(1970784794n),
      this.instances4.alice.encrypt32(1596956634n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (1596956630, 1596956634)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(1596956630n),
      this.instances4.alice.encrypt32(1596956634n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (1596956634, 1596956634)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(1596956634n),
      this.instances4.alice.encrypt32(1596956634n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (1596956634, 1596956630)', async function () {
    const res = await this.contract4.ge_euint32_euint32(
      this.instances4.alice.encrypt32(1596956634n),
      this.instances4.alice.encrypt32(1596956630n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (2212514392, 854163759)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(2212514392n),
      this.instances4.alice.encrypt32(854163759n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (854163755, 854163759)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(854163755n),
      this.instances4.alice.encrypt32(854163759n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (854163759, 854163759)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(854163759n),
      this.instances4.alice.encrypt32(854163759n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 4 (854163759, 854163755)', async function () {
    const res = await this.contract4.gt_euint32_euint32(
      this.instances4.alice.encrypt32(854163759n),
      this.instances4.alice.encrypt32(854163755n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (989604334, 3011475023)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(989604334n),
      this.instances4.alice.encrypt32(3011475023n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (989604330, 989604334)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(989604330n),
      this.instances4.alice.encrypt32(989604334n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (989604334, 989604334)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(989604334n),
      this.instances4.alice.encrypt32(989604334n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 4 (989604334, 989604330)', async function () {
    const res = await this.contract4.le_euint32_euint32(
      this.instances4.alice.encrypt32(989604334n),
      this.instances4.alice.encrypt32(989604330n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (3432727362, 2953663248)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(3432727362n),
      this.instances4.alice.encrypt32(2953663248n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (2953663244, 2953663248)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(2953663244n),
      this.instances4.alice.encrypt32(2953663248n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (2953663248, 2953663248)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(2953663248n),
      this.instances4.alice.encrypt32(2953663248n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (2953663248, 2953663244)', async function () {
    const res = await this.contract4.lt_euint32_euint32(
      this.instances4.alice.encrypt32(2953663248n),
      this.instances4.alice.encrypt32(2953663244n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (1800381916, 3495424142)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(1800381916n),
      this.instances4.alice.encrypt32(3495424142n),
    );
    expect(res).to.equal(1800381916n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (1800381912, 1800381916)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(1800381912n),
      this.instances4.alice.encrypt32(1800381916n),
    );
    expect(res).to.equal(1800381912n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (1800381916, 1800381916)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(1800381916n),
      this.instances4.alice.encrypt32(1800381916n),
    );
    expect(res).to.equal(1800381916n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (1800381916, 1800381912)', async function () {
    const res = await this.contract4.min_euint32_euint32(
      this.instances4.alice.encrypt32(1800381916n),
      this.instances4.alice.encrypt32(1800381912n),
    );
    expect(res).to.equal(1800381912n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (2043312979, 3247295293)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(2043312979n),
      this.instances4.alice.encrypt32(3247295293n),
    );
    expect(res).to.equal(3247295293n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (2043312975, 2043312979)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(2043312975n),
      this.instances4.alice.encrypt32(2043312979n),
    );
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (2043312979, 2043312979)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(2043312979n),
      this.instances4.alice.encrypt32(2043312979n),
    );
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (2043312979, 2043312975)', async function () {
    const res = await this.contract4.max_euint32_euint32(
      this.instances4.alice.encrypt32(2043312979n),
      this.instances4.alice.encrypt32(2043312975n),
    );
    expect(res).to.equal(2043312979n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 1 (2, 4293304478)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(2n),
      this.instances4.alice.encrypt64(4293304478n),
    );
    expect(res).to.equal(4293304480n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 2 (2093298939, 2093298943)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(2093298939n),
      this.instances4.alice.encrypt64(2093298943n),
    );
    expect(res).to.equal(4186597882n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 3 (2093298943, 2093298943)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(2093298943n),
      this.instances4.alice.encrypt64(2093298943n),
    );
    expect(res).to.equal(4186597886n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 4 (2093298943, 2093298939)', async function () {
    const res = await this.contract4.add_euint32_euint64(
      this.instances4.alice.encrypt32(2093298943n),
      this.instances4.alice.encrypt64(2093298939n),
    );
    expect(res).to.equal(4186597882n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (1018084759, 1018084759)', async function () {
    const res = await this.contract4.sub_euint32_euint64(
      this.instances4.alice.encrypt32(1018084759n),
      this.instances4.alice.encrypt64(1018084759n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (1018084759, 1018084755)', async function () {
    const res = await this.contract4.sub_euint32_euint64(
      this.instances4.alice.encrypt32(1018084759n),
      this.instances4.alice.encrypt64(1018084755n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (2, 2147115345)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(2n),
      this.instances4.alice.encrypt64(2147115345n),
    );
    expect(res).to.equal(4294230690n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (46957, 46957)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(46957n),
      this.instances4.alice.encrypt64(46957n),
    );
    expect(res).to.equal(2204959849n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (46957, 46957)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(46957n),
      this.instances4.alice.encrypt64(46957n),
    );
    expect(res).to.equal(2204959849n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (46957, 46957)', async function () {
    const res = await this.contract4.mul_euint32_euint64(
      this.instances4.alice.encrypt32(46957n),
      this.instances4.alice.encrypt64(46957n),
    );
    expect(res).to.equal(2204959849n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 1 (365928159, 18445926323394727831)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(365928159n),
      this.instances4.alice.encrypt64(18445926323394727831n),
    );
    expect(res).to.equal(13110935n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 2 (365928155, 365928159)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(365928155n),
      this.instances4.alice.encrypt64(365928159n),
    );
    expect(res).to.equal(365928155n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 3 (365928159, 365928159)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(365928159n),
      this.instances4.alice.encrypt64(365928159n),
    );
    expect(res).to.equal(365928159n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 4 (365928159, 365928155)', async function () {
    const res = await this.contract4.and_euint32_euint64(
      this.instances4.alice.encrypt32(365928159n),
      this.instances4.alice.encrypt64(365928155n),
    );
    expect(res).to.equal(365928155n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 1 (1857122149, 18440946789011191531)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(1857122149n),
      this.instances4.alice.encrypt64(18440946789011191531n),
    );
    expect(res).to.equal(18440946789112971247n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 2 (1857122145, 1857122149)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(1857122145n),
      this.instances4.alice.encrypt64(1857122149n),
    );
    expect(res).to.equal(1857122149n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 3 (1857122149, 1857122149)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(1857122149n),
      this.instances4.alice.encrypt64(1857122149n),
    );
    expect(res).to.equal(1857122149n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 4 (1857122149, 1857122145)', async function () {
    const res = await this.contract4.or_euint32_euint64(
      this.instances4.alice.encrypt32(1857122149n),
      this.instances4.alice.encrypt64(1857122145n),
    );
    expect(res).to.equal(1857122149n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (2536876532, 18445343517070927715)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(2536876532n),
      this.instances4.alice.encrypt64(18445343517070927715n),
    );
    expect(res).to.equal(18445343518833806999n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (2536876528, 2536876532)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(2536876528n),
      this.instances4.alice.encrypt64(2536876532n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (2536876532, 2536876532)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(2536876532n),
      this.instances4.alice.encrypt64(2536876532n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (2536876532, 2536876528)', async function () {
    const res = await this.contract4.xor_euint32_euint64(
      this.instances4.alice.encrypt32(2536876532n),
      this.instances4.alice.encrypt64(2536876528n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 1 (2805333302, 18443732701658209223)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(2805333302n),
      this.instances4.alice.encrypt64(18443732701658209223n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 2 (2805333298, 2805333302)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(2805333298n),
      this.instances4.alice.encrypt64(2805333302n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 3 (2805333302, 2805333302)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(2805333302n),
      this.instances4.alice.encrypt64(2805333302n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 4 (2805333302, 2805333298)', async function () {
    const res = await this.contract4.eq_euint32_euint64(
      this.instances4.alice.encrypt32(2805333302n),
      this.instances4.alice.encrypt64(2805333298n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (734315994, 18437906324621631829)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(734315994n),
      this.instances4.alice.encrypt64(18437906324621631829n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (734315990, 734315994)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(734315990n),
      this.instances4.alice.encrypt64(734315994n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (734315994, 734315994)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(734315994n),
      this.instances4.alice.encrypt64(734315994n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (734315994, 734315990)', async function () {
    const res = await this.contract4.ne_euint32_euint64(
      this.instances4.alice.encrypt32(734315994n),
      this.instances4.alice.encrypt64(734315990n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 1 (1126093045, 18444900127425225651)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(1126093045n),
      this.instances4.alice.encrypt64(18444900127425225651n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 2 (1126093041, 1126093045)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(1126093041n),
      this.instances4.alice.encrypt64(1126093045n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 3 (1126093045, 1126093045)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(1126093045n),
      this.instances4.alice.encrypt64(1126093045n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 4 (1126093045, 1126093041)', async function () {
    const res = await this.contract4.ge_euint32_euint64(
      this.instances4.alice.encrypt32(1126093045n),
      this.instances4.alice.encrypt64(1126093041n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 1 (1296765186, 18439742981119094705)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(1296765186n),
      this.instances4.alice.encrypt64(18439742981119094705n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 2 (1296765182, 1296765186)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(1296765182n),
      this.instances4.alice.encrypt64(1296765186n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 3 (1296765186, 1296765186)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(1296765186n),
      this.instances4.alice.encrypt64(1296765186n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 4 (1296765186, 1296765182)', async function () {
    const res = await this.contract4.gt_euint32_euint64(
      this.instances4.alice.encrypt32(1296765186n),
      this.instances4.alice.encrypt64(1296765182n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 1 (3652038615, 18440248369616077365)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(3652038615n),
      this.instances4.alice.encrypt64(18440248369616077365n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 2 (3652038611, 3652038615)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(3652038611n),
      this.instances4.alice.encrypt64(3652038615n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 3 (3652038615, 3652038615)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(3652038615n),
      this.instances4.alice.encrypt64(3652038615n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 4 (3652038615, 3652038611)', async function () {
    const res = await this.contract4.le_euint32_euint64(
      this.instances4.alice.encrypt32(3652038615n),
      this.instances4.alice.encrypt64(3652038611n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 1 (1728686329, 18442541149578902807)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(1728686329n),
      this.instances4.alice.encrypt64(18442541149578902807n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 2 (1728686325, 1728686329)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(1728686325n),
      this.instances4.alice.encrypt64(1728686329n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 3 (1728686329, 1728686329)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(1728686329n),
      this.instances4.alice.encrypt64(1728686329n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 4 (1728686329, 1728686325)', async function () {
    const res = await this.contract4.lt_euint32_euint64(
      this.instances4.alice.encrypt32(1728686329n),
      this.instances4.alice.encrypt64(1728686325n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 1 (968740054, 18439525226452222255)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(968740054n),
      this.instances4.alice.encrypt64(18439525226452222255n),
    );
    expect(res).to.equal(968740054n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 2 (968740050, 968740054)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(968740050n),
      this.instances4.alice.encrypt64(968740054n),
    );
    expect(res).to.equal(968740050n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 3 (968740054, 968740054)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(968740054n),
      this.instances4.alice.encrypt64(968740054n),
    );
    expect(res).to.equal(968740054n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 4 (968740054, 968740050)', async function () {
    const res = await this.contract4.min_euint32_euint64(
      this.instances4.alice.encrypt32(968740054n),
      this.instances4.alice.encrypt64(968740050n),
    );
    expect(res).to.equal(968740050n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (340812031, 18443150842651286449)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(340812031n),
      this.instances4.alice.encrypt64(18443150842651286449n),
    );
    expect(res).to.equal(18443150842651286449n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (340812027, 340812031)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(340812027n),
      this.instances4.alice.encrypt64(340812031n),
    );
    expect(res).to.equal(340812031n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (340812031, 340812031)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(340812031n),
      this.instances4.alice.encrypt64(340812031n),
    );
    expect(res).to.equal(340812031n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (340812031, 340812027)', async function () {
    const res = await this.contract4.max_euint32_euint64(
      this.instances4.alice.encrypt32(340812031n),
      this.instances4.alice.encrypt64(340812027n),
    );
    expect(res).to.equal(340812031n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (1081671644, 1277295402)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(1081671644n), 1277295402);
    expect(res).to.equal(2358967046n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (829836783, 829836787)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(829836783n), 829836787);
    expect(res).to.equal(1659673570n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (829836787, 829836787)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(829836787n), 829836787);
    expect(res).to.equal(1659673574n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (829836787, 829836783)', async function () {
    const res = await this.contract4.add_euint32_uint32(this.instances4.alice.encrypt32(829836787n), 829836783);
    expect(res).to.equal(1659673570n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (1301306419, 1277295402)', async function () {
    const res = await this.contract4.add_uint32_euint32(1301306419, this.instances4.alice.encrypt32(1277295402n));
    expect(res).to.equal(2578601821n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 2 (829836783, 829836787)', async function () {
    const res = await this.contract4.add_uint32_euint32(829836783, this.instances4.alice.encrypt32(829836787n));
    expect(res).to.equal(1659673570n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 3 (829836787, 829836787)', async function () {
    const res = await this.contract4.add_uint32_euint32(829836787, this.instances4.alice.encrypt32(829836787n));
    expect(res).to.equal(1659673574n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 4 (829836787, 829836783)', async function () {
    const res = await this.contract4.add_uint32_euint32(829836787, this.instances4.alice.encrypt32(829836783n));
    expect(res).to.equal(1659673570n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (3061205449, 3061205449)', async function () {
    const res = await this.contract4.sub_euint32_uint32(this.instances4.alice.encrypt32(3061205449n), 3061205449);
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 2 (3061205449, 3061205445)', async function () {
    const res = await this.contract4.sub_euint32_uint32(this.instances4.alice.encrypt32(3061205449n), 3061205445);
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (3061205449, 3061205449)', async function () {
    const res = await this.contract4.sub_uint32_euint32(3061205449, this.instances4.alice.encrypt32(3061205449n));
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 2 (3061205449, 3061205445)', async function () {
    const res = await this.contract4.sub_uint32_euint32(3061205449, this.instances4.alice.encrypt32(3061205445n));
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (42467, 65037)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(42467n), 65037);
    expect(res).to.equal(2761926279n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 2 (41983, 41983)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(41983n), 41983);
    expect(res).to.equal(1762572289n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 3 (41983, 41983)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(41983n), 41983);
    expect(res).to.equal(1762572289n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 4 (41983, 41983)', async function () {
    const res = await this.contract4.mul_euint32_uint32(this.instances4.alice.encrypt32(41983n), 41983);
    expect(res).to.equal(1762572289n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (17461, 65037)', async function () {
    const res = await this.contract4.mul_uint32_euint32(17461, this.instances4.alice.encrypt32(65037n));
    expect(res).to.equal(1135611057n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (41983, 41983)', async function () {
    const res = await this.contract4.mul_uint32_euint32(41983, this.instances4.alice.encrypt32(41983n));
    expect(res).to.equal(1762572289n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (41983, 41983)', async function () {
    const res = await this.contract4.mul_uint32_euint32(41983, this.instances4.alice.encrypt32(41983n));
    expect(res).to.equal(1762572289n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (41983, 41983)', async function () {
    const res = await this.contract4.mul_uint32_euint32(41983, this.instances4.alice.encrypt32(41983n));
    expect(res).to.equal(1762572289n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (1945845245, 755191882)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(1945845245n), 755191882);
    expect(res).to.equal(2n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (1945845241, 1945845245)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(1945845241n), 1945845245);
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (1945845245, 1945845245)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(1945845245n), 1945845245);
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (1945845245, 1945845241)', async function () {
    const res = await this.contract4.div_euint32_uint32(this.instances4.alice.encrypt32(1945845245n), 1945845241);
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (2521442787, 706504920)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(2521442787n), 706504920);
    expect(res).to.equal(401928027n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 2 (2521442783, 2521442787)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(2521442783n), 2521442787);
    expect(res).to.equal(2521442783n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 3 (2521442787, 2521442787)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(2521442787n), 2521442787);
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 4 (2521442787, 2521442783)', async function () {
    const res = await this.contract4.rem_euint32_uint32(this.instances4.alice.encrypt32(2521442787n), 2521442783);
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (3674813327, 340447461)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(3674813327n), 340447461);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (3173886287, 3173886291)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(3173886287n), 3173886291);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 3 (3173886291, 3173886291)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(3173886291n), 3173886291);
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 4 (3173886291, 3173886287)', async function () {
    const res = await this.contract4.eq_euint32_uint32(this.instances4.alice.encrypt32(3173886291n), 3173886287);
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (630917914, 340447461)', async function () {
    const res = await this.contract4.eq_uint32_euint32(630917914, this.instances4.alice.encrypt32(340447461n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (3173886287, 3173886291)', async function () {
    const res = await this.contract4.eq_uint32_euint32(3173886287, this.instances4.alice.encrypt32(3173886291n));
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 3 (3173886291, 3173886291)', async function () {
    const res = await this.contract4.eq_uint32_euint32(3173886291, this.instances4.alice.encrypt32(3173886291n));
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 4 (3173886291, 3173886287)', async function () {
    const res = await this.contract4.eq_uint32_euint32(3173886291, this.instances4.alice.encrypt32(3173886287n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (40863923, 740881042)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(40863923n), 740881042);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (40863919, 40863923)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(40863919n), 40863923);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 3 (40863923, 40863923)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(40863923n), 40863923);
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 4 (40863923, 40863919)', async function () {
    const res = await this.contract4.ne_euint32_uint32(this.instances4.alice.encrypt32(40863923n), 40863919);
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (3944008003, 740881042)', async function () {
    const res = await this.contract4.ne_uint32_euint32(3944008003, this.instances4.alice.encrypt32(740881042n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (40863919, 40863923)', async function () {
    const res = await this.contract4.ne_uint32_euint32(40863919, this.instances4.alice.encrypt32(40863923n));
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 3 (40863923, 40863923)', async function () {
    const res = await this.contract4.ne_uint32_euint32(40863923, this.instances4.alice.encrypt32(40863923n));
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 4 (40863923, 40863919)', async function () {
    const res = await this.contract4.ne_uint32_euint32(40863923, this.instances4.alice.encrypt32(40863919n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (1970784794, 2198890323)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(1970784794n), 2198890323);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (1596956630, 1596956634)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(1596956630n), 1596956634);
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (1596956634, 1596956634)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(1596956634n), 1596956634);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 4 (1596956634, 1596956630)', async function () {
    const res = await this.contract4.ge_euint32_uint32(this.instances4.alice.encrypt32(1596956634n), 1596956630);
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (3128599756, 2198890323)', async function () {
    const res = await this.contract4.ge_uint32_euint32(3128599756, this.instances4.alice.encrypt32(2198890323n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (1596956630, 1596956634)', async function () {
    const res = await this.contract4.ge_uint32_euint32(1596956630, this.instances4.alice.encrypt32(1596956634n));
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (1596956634, 1596956634)', async function () {
    const res = await this.contract4.ge_uint32_euint32(1596956634, this.instances4.alice.encrypt32(1596956634n));
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (1596956634, 1596956630)', async function () {
    const res = await this.contract4.ge_uint32_euint32(1596956634, this.instances4.alice.encrypt32(1596956630n));
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (2212514392, 3587264713)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(2212514392n), 3587264713);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (854163755, 854163759)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(854163755n), 854163759);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (854163759, 854163759)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(854163759n), 854163759);
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 4 (854163759, 854163755)', async function () {
    const res = await this.contract4.gt_euint32_uint32(this.instances4.alice.encrypt32(854163759n), 854163755);
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (405003775, 3587264713)', async function () {
    const res = await this.contract4.gt_uint32_euint32(405003775, this.instances4.alice.encrypt32(3587264713n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (854163755, 854163759)', async function () {
    const res = await this.contract4.gt_uint32_euint32(854163755, this.instances4.alice.encrypt32(854163759n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (854163759, 854163759)', async function () {
    const res = await this.contract4.gt_uint32_euint32(854163759, this.instances4.alice.encrypt32(854163759n));
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 4 (854163759, 854163755)', async function () {
    const res = await this.contract4.gt_uint32_euint32(854163759, this.instances4.alice.encrypt32(854163755n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (989604334, 2400461325)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(989604334n), 2400461325);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (989604330, 989604334)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(989604330n), 989604334);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (989604334, 989604334)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(989604334n), 989604334);
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 4 (989604334, 989604330)', async function () {
    const res = await this.contract4.le_euint32_uint32(this.instances4.alice.encrypt32(989604334n), 989604330);
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (3171491075, 2400461325)', async function () {
    const res = await this.contract4.le_uint32_euint32(3171491075, this.instances4.alice.encrypt32(2400461325n));
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (989604330, 989604334)', async function () {
    const res = await this.contract4.le_uint32_euint32(989604330, this.instances4.alice.encrypt32(989604334n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (989604334, 989604334)', async function () {
    const res = await this.contract4.le_uint32_euint32(989604334, this.instances4.alice.encrypt32(989604334n));
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 4 (989604334, 989604330)', async function () {
    const res = await this.contract4.le_uint32_euint32(989604334, this.instances4.alice.encrypt32(989604330n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (3432727362, 340267218)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(3432727362n), 340267218);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (2953663244, 2953663248)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(2953663244n), 2953663248);
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (2953663248, 2953663248)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(2953663248n), 2953663248);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 4 (2953663248, 2953663244)', async function () {
    const res = await this.contract4.lt_euint32_uint32(this.instances4.alice.encrypt32(2953663248n), 2953663244);
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (254860119, 340267218)', async function () {
    const res = await this.contract4.lt_uint32_euint32(254860119, this.instances4.alice.encrypt32(340267218n));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (2953663244, 2953663248)', async function () {
    const res = await this.contract4.lt_uint32_euint32(2953663244, this.instances4.alice.encrypt32(2953663248n));
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (2953663248, 2953663248)', async function () {
    const res = await this.contract4.lt_uint32_euint32(2953663248, this.instances4.alice.encrypt32(2953663248n));
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 4 (2953663248, 2953663244)', async function () {
    const res = await this.contract4.lt_uint32_euint32(2953663248, this.instances4.alice.encrypt32(2953663244n));
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (1800381916, 3488347882)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(1800381916n), 3488347882);
    expect(res).to.equal(1800381916n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (1800381912, 1800381916)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(1800381912n), 1800381916);
    expect(res).to.equal(1800381912n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (1800381916, 1800381916)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(1800381916n), 1800381916);
    expect(res).to.equal(1800381916n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 4 (1800381916, 1800381912)', async function () {
    const res = await this.contract4.min_euint32_uint32(this.instances4.alice.encrypt32(1800381916n), 1800381912);
    expect(res).to.equal(1800381912n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (2638635782, 3488347882)', async function () {
    const res = await this.contract4.min_uint32_euint32(2638635782, this.instances4.alice.encrypt32(3488347882n));
    expect(res).to.equal(2638635782n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (1800381912, 1800381916)', async function () {
    const res = await this.contract4.min_uint32_euint32(1800381912, this.instances4.alice.encrypt32(1800381916n));
    expect(res).to.equal(1800381912n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (1800381916, 1800381916)', async function () {
    const res = await this.contract4.min_uint32_euint32(1800381916, this.instances4.alice.encrypt32(1800381916n));
    expect(res).to.equal(1800381916n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 4 (1800381916, 1800381912)', async function () {
    const res = await this.contract4.min_uint32_euint32(1800381916, this.instances4.alice.encrypt32(1800381912n));
    expect(res).to.equal(1800381912n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (2043312979, 706283813)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(2043312979n), 706283813);
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (2043312975, 2043312979)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(2043312975n), 2043312979);
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (2043312979, 2043312979)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(2043312979n), 2043312979);
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 4 (2043312979, 2043312975)', async function () {
    const res = await this.contract4.max_euint32_uint32(this.instances4.alice.encrypt32(2043312979n), 2043312975);
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (1094445398, 706283813)', async function () {
    const res = await this.contract4.max_uint32_euint32(1094445398, this.instances4.alice.encrypt32(706283813n));
    expect(res).to.equal(1094445398n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (2043312975, 2043312979)', async function () {
    const res = await this.contract4.max_uint32_euint32(2043312975, this.instances4.alice.encrypt32(2043312979n));
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (2043312979, 2043312979)', async function () {
    const res = await this.contract4.max_uint32_euint32(2043312979, this.instances4.alice.encrypt32(2043312979n));
    expect(res).to.equal(2043312979n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 4 (2043312979, 2043312975)', async function () {
    const res = await this.contract4.max_uint32_euint32(2043312979, this.instances4.alice.encrypt32(2043312975n));
    expect(res).to.equal(2043312979n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 1 (9, 2)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(9n),
      this.instances4.alice.encrypt4(2n),
    );
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 2 (6, 8)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(6n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 3 (5, 5)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(5n),
      this.instances4.alice.encrypt4(5n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 4 (8, 6)', async function () {
    const res = await this.contract4.add_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(6n),
    );
    expect(res).to.equal(14n);
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

  it('test operator "and" overload (euint64, euint4) => euint64 test 1 (18442416087216426369, 12)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(18442416087216426369n),
      this.instances4.alice.encrypt4(12n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 2 (8, 12)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(12n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 3 (12, 12)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(12n),
      this.instances4.alice.encrypt4(12n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 4 (12, 8)', async function () {
    const res = await this.contract4.and_euint64_euint4(
      this.instances4.alice.encrypt64(12n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 1 (18445233461885169423, 12)', async function () {
    const res = await this.contract4.or_euint64_euint4(
      this.instances4.alice.encrypt64(18445233461885169423n),
      this.instances4.alice.encrypt4(12n),
    );
    expect(res).to.equal(18445233461885169423n);
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

  it('test operator "xor" overload (euint64, euint4) => euint64 test 1 (18444022948851042337, 14)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(18444022948851042337n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(18444022948851042351n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 2 (10, 14)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(10n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 3 (14, 14)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(14n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 4 (14, 10)', async function () {
    const res = await this.contract4.xor_euint64_euint4(
      this.instances4.alice.encrypt64(14n),
      this.instances4.alice.encrypt4(10n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 1 (18439121948970600941, 14)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(18439121948970600941n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(10n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(14n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract4.eq_euint64_euint4(
      this.instances4.alice.encrypt64(14n),
      this.instances4.alice.encrypt4(10n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 1 (18442748864669273899, 3)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(18442748864669273899n),
      this.instances4.alice.encrypt4(3n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(4n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const res = await this.contract4.ne_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(4n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 1 (18438518295027982843, 12)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(18438518295027982843n),
      this.instances4.alice.encrypt4(12n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 2 (8, 12)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(12n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 3 (12, 12)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(12n),
      this.instances4.alice.encrypt4(12n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 4 (12, 8)', async function () {
    const res = await this.contract4.ge_euint64_euint4(
      this.instances4.alice.encrypt64(12n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 1 (18445028672073920305, 11)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(18445028672073920305n),
      this.instances4.alice.encrypt4(11n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(7n),
      this.instances4.alice.encrypt4(11n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(11n),
      this.instances4.alice.encrypt4(11n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract4.gt_euint64_euint4(
      this.instances4.alice.encrypt64(11n),
      this.instances4.alice.encrypt4(7n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 1 (18446127513373335953, 14)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(18446127513373335953n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 2 (10, 14)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(10n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 3 (14, 14)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(14n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 4 (14, 10)', async function () {
    const res = await this.contract4.le_euint64_euint4(
      this.instances4.alice.encrypt64(14n),
      this.instances4.alice.encrypt4(10n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 1 (18440890716948813157, 11)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(18440890716948813157n),
      this.instances4.alice.encrypt4(11n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 2 (7, 11)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(7n),
      this.instances4.alice.encrypt4(11n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 3 (11, 11)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(11n),
      this.instances4.alice.encrypt4(11n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 4 (11, 7)', async function () {
    const res = await this.contract4.lt_euint64_euint4(
      this.instances4.alice.encrypt64(11n),
      this.instances4.alice.encrypt4(7n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 1 (18437774930056492317, 14)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(18437774930056492317n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 2 (10, 14)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(10n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 3 (14, 14)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(14n),
      this.instances4.alice.encrypt4(14n),
    );
    expect(res).to.equal(14n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 4 (14, 10)', async function () {
    const res = await this.contract4.min_euint64_euint4(
      this.instances4.alice.encrypt64(14n),
      this.instances4.alice.encrypt4(10n),
    );
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 1 (18441711596093702931, 1)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(18441711596093702931n),
      this.instances4.alice.encrypt4(1n),
    );
    expect(res).to.equal(18441711596093702931n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(4n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract4.max_euint64_euint4(
      this.instances4.alice.encrypt64(8n),
      this.instances4.alice.encrypt4(4n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 1 (129, 2)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(129n),
      this.instances4.alice.encrypt8(2n),
    );
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 2 (98, 102)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(98n),
      this.instances4.alice.encrypt8(102n),
    );
    expect(res).to.equal(200n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 3 (102, 102)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(102n),
      this.instances4.alice.encrypt8(102n),
    );
    expect(res).to.equal(204n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 4 (102, 98)', async function () {
    const res = await this.contract4.add_euint64_euint8(
      this.instances4.alice.encrypt64(102n),
      this.instances4.alice.encrypt8(98n),
    );
    expect(res).to.equal(200n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 1 (101, 101)', async function () {
    const res = await this.contract4.sub_euint64_euint8(
      this.instances4.alice.encrypt64(101n),
      this.instances4.alice.encrypt8(101n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 2 (101, 97)', async function () {
    const res = await this.contract4.sub_euint64_euint8(
      this.instances4.alice.encrypt64(101n),
      this.instances4.alice.encrypt8(97n),
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

  it('test operator "mul" overload (euint64, euint8) => euint64 test 2 (15, 15)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(15n),
      this.instances4.alice.encrypt8(15n),
    );
    expect(res).to.equal(225n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 3 (15, 15)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(15n),
      this.instances4.alice.encrypt8(15n),
    );
    expect(res).to.equal(225n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 4 (15, 15)', async function () {
    const res = await this.contract4.mul_euint64_euint8(
      this.instances4.alice.encrypt64(15n),
      this.instances4.alice.encrypt8(15n),
    );
    expect(res).to.equal(225n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 1 (18438525099029165039, 71)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(18438525099029165039n),
      this.instances4.alice.encrypt8(71n),
    );
    expect(res).to.equal(71n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 2 (67, 71)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(67n),
      this.instances4.alice.encrypt8(71n),
    );
    expect(res).to.equal(67n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 3 (71, 71)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(71n),
      this.instances4.alice.encrypt8(71n),
    );
    expect(res).to.equal(71n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 4 (71, 67)', async function () {
    const res = await this.contract4.and_euint64_euint8(
      this.instances4.alice.encrypt64(71n),
      this.instances4.alice.encrypt8(67n),
    );
    expect(res).to.equal(67n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 1 (18445900286060653541, 17)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(18445900286060653541n),
      this.instances4.alice.encrypt8(17n),
    );
    expect(res).to.equal(18445900286060653557n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 2 (13, 17)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(13n),
      this.instances4.alice.encrypt8(17n),
    );
    expect(res).to.equal(29n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 3 (17, 17)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(17n),
      this.instances4.alice.encrypt8(17n),
    );
    expect(res).to.equal(17n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 4 (17, 13)', async function () {
    const res = await this.contract4.or_euint64_euint8(
      this.instances4.alice.encrypt64(17n),
      this.instances4.alice.encrypt8(13n),
    );
    expect(res).to.equal(29n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 1 (18439418694946486147, 205)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(18439418694946486147n),
      this.instances4.alice.encrypt8(205n),
    );
    expect(res).to.equal(18439418694946486094n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 2 (201, 205)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(201n),
      this.instances4.alice.encrypt8(205n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 3 (205, 205)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(205n),
      this.instances4.alice.encrypt8(205n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 4 (205, 201)', async function () {
    const res = await this.contract4.xor_euint64_euint8(
      this.instances4.alice.encrypt64(205n),
      this.instances4.alice.encrypt8(201n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 1 (18441725985045807501, 52)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(18441725985045807501n),
      this.instances4.alice.encrypt8(52n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 2 (48, 52)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(48n),
      this.instances4.alice.encrypt8(52n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 3 (52, 52)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(52n),
      this.instances4.alice.encrypt8(52n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 4 (52, 48)', async function () {
    const res = await this.contract4.eq_euint64_euint8(
      this.instances4.alice.encrypt64(52n),
      this.instances4.alice.encrypt8(48n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 1 (18437879735793971605, 183)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(18437879735793971605n),
      this.instances4.alice.encrypt8(183n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 2 (179, 183)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(179n),
      this.instances4.alice.encrypt8(183n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 3 (183, 183)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(183n),
      this.instances4.alice.encrypt8(183n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 4 (183, 179)', async function () {
    const res = await this.contract4.ne_euint64_euint8(
      this.instances4.alice.encrypt64(183n),
      this.instances4.alice.encrypt8(179n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 1 (18439034739003983767, 245)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(18439034739003983767n),
      this.instances4.alice.encrypt8(245n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 2 (241, 245)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(241n),
      this.instances4.alice.encrypt8(245n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 3 (245, 245)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(245n),
      this.instances4.alice.encrypt8(245n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 4 (245, 241)', async function () {
    const res = await this.contract4.ge_euint64_euint8(
      this.instances4.alice.encrypt64(245n),
      this.instances4.alice.encrypt8(241n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 1 (18444472755157488819, 101)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(18444472755157488819n),
      this.instances4.alice.encrypt8(101n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 2 (97, 101)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(97n),
      this.instances4.alice.encrypt8(101n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 3 (101, 101)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(101n),
      this.instances4.alice.encrypt8(101n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 4 (101, 97)', async function () {
    const res = await this.contract4.gt_euint64_euint8(
      this.instances4.alice.encrypt64(101n),
      this.instances4.alice.encrypt8(97n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 1 (18446424442125229129, 19)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(18446424442125229129n),
      this.instances5.alice.encrypt8(19n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 2 (15, 19)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(15n),
      this.instances5.alice.encrypt8(19n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 3 (19, 19)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(19n),
      this.instances5.alice.encrypt8(19n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 4 (19, 15)', async function () {
    const res = await this.contract5.le_euint64_euint8(
      this.instances5.alice.encrypt64(19n),
      this.instances5.alice.encrypt8(15n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 1 (18445717540030063977, 27)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(18445717540030063977n),
      this.instances5.alice.encrypt8(27n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 2 (23, 27)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(23n),
      this.instances5.alice.encrypt8(27n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 3 (27, 27)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(27n),
      this.instances5.alice.encrypt8(27n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 4 (27, 23)', async function () {
    const res = await this.contract5.lt_euint64_euint8(
      this.instances5.alice.encrypt64(27n),
      this.instances5.alice.encrypt8(23n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 1 (18441639004244956281, 36)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(18441639004244956281n),
      this.instances5.alice.encrypt8(36n),
    );
    expect(res).to.equal(36n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 2 (32, 36)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(32n),
      this.instances5.alice.encrypt8(36n),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 3 (36, 36)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(36n),
      this.instances5.alice.encrypt8(36n),
    );
    expect(res).to.equal(36n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 4 (36, 32)', async function () {
    const res = await this.contract5.min_euint64_euint8(
      this.instances5.alice.encrypt64(36n),
      this.instances5.alice.encrypt8(32n),
    );
    expect(res).to.equal(32n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 1 (18442614857754346699, 3)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(18442614857754346699n),
      this.instances5.alice.encrypt8(3n),
    );
    expect(res).to.equal(18442614857754346699n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.max_euint64_euint8(
      this.instances5.alice.encrypt64(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 1 (65532, 2)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(65532n),
      this.instances5.alice.encrypt16(2n),
    );
    expect(res).to.equal(65534n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 2 (23282, 23284)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(23282n),
      this.instances5.alice.encrypt16(23284n),
    );
    expect(res).to.equal(46566n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 3 (23284, 23284)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(23284n),
      this.instances5.alice.encrypt16(23284n),
    );
    expect(res).to.equal(46568n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 4 (23284, 23282)', async function () {
    const res = await this.contract5.add_euint64_euint16(
      this.instances5.alice.encrypt64(23284n),
      this.instances5.alice.encrypt16(23282n),
    );
    expect(res).to.equal(46566n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 1 (25338, 25338)', async function () {
    const res = await this.contract5.sub_euint64_euint16(
      this.instances5.alice.encrypt64(25338n),
      this.instances5.alice.encrypt16(25338n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 2 (25338, 25334)', async function () {
    const res = await this.contract5.sub_euint64_euint16(
      this.instances5.alice.encrypt64(25338n),
      this.instances5.alice.encrypt16(25334n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 1 (32765, 2)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(32765n),
      this.instances5.alice.encrypt16(2n),
    );
    expect(res).to.equal(65530n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 2 (163, 163)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(163n),
      this.instances5.alice.encrypt16(163n),
    );
    expect(res).to.equal(26569n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 3 (163, 163)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(163n),
      this.instances5.alice.encrypt16(163n),
    );
    expect(res).to.equal(26569n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 4 (163, 163)', async function () {
    const res = await this.contract5.mul_euint64_euint16(
      this.instances5.alice.encrypt64(163n),
      this.instances5.alice.encrypt16(163n),
    );
    expect(res).to.equal(26569n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 1 (18444131025157690381, 45908)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(18444131025157690381n),
      this.instances5.alice.encrypt16(45908n),
    );
    expect(res).to.equal(4100n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 2 (45904, 45908)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(45904n),
      this.instances5.alice.encrypt16(45908n),
    );
    expect(res).to.equal(45904n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 3 (45908, 45908)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(45908n),
      this.instances5.alice.encrypt16(45908n),
    );
    expect(res).to.equal(45908n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 4 (45908, 45904)', async function () {
    const res = await this.contract5.and_euint64_euint16(
      this.instances5.alice.encrypt64(45908n),
      this.instances5.alice.encrypt16(45904n),
    );
    expect(res).to.equal(45904n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 1 (18445329388989380329, 5327)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(18445329388989380329n),
      this.instances5.alice.encrypt16(5327n),
    );
    expect(res).to.equal(18445329388989380335n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 2 (5323, 5327)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(5323n),
      this.instances5.alice.encrypt16(5327n),
    );
    expect(res).to.equal(5327n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 3 (5327, 5327)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(5327n),
      this.instances5.alice.encrypt16(5327n),
    );
    expect(res).to.equal(5327n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 4 (5327, 5323)', async function () {
    const res = await this.contract5.or_euint64_euint16(
      this.instances5.alice.encrypt64(5327n),
      this.instances5.alice.encrypt16(5323n),
    );
    expect(res).to.equal(5327n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 1 (18443398919380985157, 45655)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(18443398919380985157n),
      this.instances5.alice.encrypt16(45655n),
    );
    expect(res).to.equal(18443398919380948754n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 2 (45651, 45655)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(45651n),
      this.instances5.alice.encrypt16(45655n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 3 (45655, 45655)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(45655n),
      this.instances5.alice.encrypt16(45655n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 4 (45655, 45651)', async function () {
    const res = await this.contract5.xor_euint64_euint16(
      this.instances5.alice.encrypt64(45655n),
      this.instances5.alice.encrypt16(45651n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 1 (18444068251063204057, 45460)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(18444068251063204057n),
      this.instances5.alice.encrypt16(45460n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 2 (45456, 45460)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(45456n),
      this.instances5.alice.encrypt16(45460n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 3 (45460, 45460)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(45460n),
      this.instances5.alice.encrypt16(45460n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 4 (45460, 45456)', async function () {
    const res = await this.contract5.eq_euint64_euint16(
      this.instances5.alice.encrypt64(45460n),
      this.instances5.alice.encrypt16(45456n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 1 (18439766867707518465, 19155)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(18439766867707518465n),
      this.instances5.alice.encrypt16(19155n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 2 (19151, 19155)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(19151n),
      this.instances5.alice.encrypt16(19155n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 3 (19155, 19155)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(19155n),
      this.instances5.alice.encrypt16(19155n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 4 (19155, 19151)', async function () {
    const res = await this.contract5.ne_euint64_euint16(
      this.instances5.alice.encrypt64(19155n),
      this.instances5.alice.encrypt16(19151n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 1 (18440840766291159069, 55918)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(18440840766291159069n),
      this.instances5.alice.encrypt16(55918n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 2 (55914, 55918)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(55914n),
      this.instances5.alice.encrypt16(55918n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 3 (55918, 55918)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(55918n),
      this.instances5.alice.encrypt16(55918n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 4 (55918, 55914)', async function () {
    const res = await this.contract5.ge_euint64_euint16(
      this.instances5.alice.encrypt64(55918n),
      this.instances5.alice.encrypt16(55914n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 1 (18446239272421398075, 52612)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(18446239272421398075n),
      this.instances5.alice.encrypt16(52612n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 2 (52608, 52612)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(52608n),
      this.instances5.alice.encrypt16(52612n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 3 (52612, 52612)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(52612n),
      this.instances5.alice.encrypt16(52612n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 4 (52612, 52608)', async function () {
    const res = await this.contract5.gt_euint64_euint16(
      this.instances5.alice.encrypt64(52612n),
      this.instances5.alice.encrypt16(52608n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 1 (18442809234541757919, 39205)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(18442809234541757919n),
      this.instances5.alice.encrypt16(39205n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 2 (39201, 39205)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(39201n),
      this.instances5.alice.encrypt16(39205n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 3 (39205, 39205)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(39205n),
      this.instances5.alice.encrypt16(39205n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 4 (39205, 39201)', async function () {
    const res = await this.contract5.le_euint64_euint16(
      this.instances5.alice.encrypt64(39205n),
      this.instances5.alice.encrypt16(39201n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 1 (18443664339829863559, 39059)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(18443664339829863559n),
      this.instances5.alice.encrypt16(39059n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 2 (39055, 39059)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(39055n),
      this.instances5.alice.encrypt16(39059n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 3 (39059, 39059)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(39059n),
      this.instances5.alice.encrypt16(39059n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 4 (39059, 39055)', async function () {
    const res = await this.contract5.lt_euint64_euint16(
      this.instances5.alice.encrypt64(39059n),
      this.instances5.alice.encrypt16(39055n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 1 (18440273866551894469, 42779)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(18440273866551894469n),
      this.instances5.alice.encrypt16(42779n),
    );
    expect(res).to.equal(42779n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 2 (42775, 42779)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(42775n),
      this.instances5.alice.encrypt16(42779n),
    );
    expect(res).to.equal(42775n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 3 (42779, 42779)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(42779n),
      this.instances5.alice.encrypt16(42779n),
    );
    expect(res).to.equal(42779n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 4 (42779, 42775)', async function () {
    const res = await this.contract5.min_euint64_euint16(
      this.instances5.alice.encrypt64(42779n),
      this.instances5.alice.encrypt16(42775n),
    );
    expect(res).to.equal(42775n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (18439537518803733101, 41315)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(18439537518803733101n),
      this.instances5.alice.encrypt16(41315n),
    );
    expect(res).to.equal(18439537518803733101n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (41311, 41315)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(41311n),
      this.instances5.alice.encrypt16(41315n),
    );
    expect(res).to.equal(41315n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (41315, 41315)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(41315n),
      this.instances5.alice.encrypt16(41315n),
    );
    expect(res).to.equal(41315n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (41315, 41311)', async function () {
    const res = await this.contract5.max_euint64_euint16(
      this.instances5.alice.encrypt64(41315n),
      this.instances5.alice.encrypt16(41311n),
    );
    expect(res).to.equal(41315n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 1 (4293362093, 2)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(4293362093n),
      this.instances5.alice.encrypt32(2n),
    );
    expect(res).to.equal(4293362095n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 2 (1194292230, 1194292232)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(1194292230n),
      this.instances5.alice.encrypt32(1194292232n),
    );
    expect(res).to.equal(2388584462n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 3 (1194292232, 1194292232)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(1194292232n),
      this.instances5.alice.encrypt32(1194292232n),
    );
    expect(res).to.equal(2388584464n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 4 (1194292232, 1194292230)', async function () {
    const res = await this.contract5.add_euint64_euint32(
      this.instances5.alice.encrypt64(1194292232n),
      this.instances5.alice.encrypt32(1194292230n),
    );
    expect(res).to.equal(2388584462n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 1 (624820999, 624820999)', async function () {
    const res = await this.contract5.sub_euint64_euint32(
      this.instances5.alice.encrypt64(624820999n),
      this.instances5.alice.encrypt32(624820999n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 2 (624820999, 624820995)', async function () {
    const res = await this.contract5.sub_euint64_euint32(
      this.instances5.alice.encrypt64(624820999n),
      this.instances5.alice.encrypt32(624820995n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 1 (2147410658, 2)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(2147410658n),
      this.instances5.alice.encrypt32(2n),
    );
    expect(res).to.equal(4294821316n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 2 (32881, 32881)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(32881n),
      this.instances5.alice.encrypt32(32881n),
    );
    expect(res).to.equal(1081160161n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 3 (32881, 32881)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(32881n),
      this.instances5.alice.encrypt32(32881n),
    );
    expect(res).to.equal(1081160161n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 4 (32881, 32881)', async function () {
    const res = await this.contract5.mul_euint64_euint32(
      this.instances5.alice.encrypt64(32881n),
      this.instances5.alice.encrypt32(32881n),
    );
    expect(res).to.equal(1081160161n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 1 (18444449709394842073, 3823819229)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(18444449709394842073n),
      this.instances5.alice.encrypt32(3823819229n),
    );
    expect(res).to.equal(2718454233n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 2 (3823819225, 3823819229)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(3823819225n),
      this.instances5.alice.encrypt32(3823819229n),
    );
    expect(res).to.equal(3823819225n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 3 (3823819229, 3823819229)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(3823819229n),
      this.instances5.alice.encrypt32(3823819229n),
    );
    expect(res).to.equal(3823819229n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 4 (3823819229, 3823819225)', async function () {
    const res = await this.contract5.and_euint64_euint32(
      this.instances5.alice.encrypt64(3823819229n),
      this.instances5.alice.encrypt32(3823819225n),
    );
    expect(res).to.equal(3823819225n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 1 (18444852954550836631, 3550708762)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(18444852954550836631n),
      this.instances5.alice.encrypt32(3550708762n),
    );
    expect(res).to.equal(18444852955920440735n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 2 (3550708758, 3550708762)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(3550708758n),
      this.instances5.alice.encrypt32(3550708762n),
    );
    expect(res).to.equal(3550708766n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 3 (3550708762, 3550708762)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(3550708762n),
      this.instances5.alice.encrypt32(3550708762n),
    );
    expect(res).to.equal(3550708762n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 4 (3550708762, 3550708758)', async function () {
    const res = await this.contract5.or_euint64_euint32(
      this.instances5.alice.encrypt64(3550708762n),
      this.instances5.alice.encrypt32(3550708758n),
    );
    expect(res).to.equal(3550708766n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 1 (18445532500811503869, 2578227181)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(18445532500811503869n),
      this.instances5.alice.encrypt32(2578227181n),
    );
    expect(res).to.equal(18445532498506434320n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 2 (2578227177, 2578227181)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(2578227177n),
      this.instances5.alice.encrypt32(2578227181n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 3 (2578227181, 2578227181)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(2578227181n),
      this.instances5.alice.encrypt32(2578227181n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 4 (2578227181, 2578227177)', async function () {
    const res = await this.contract5.xor_euint64_euint32(
      this.instances5.alice.encrypt64(2578227181n),
      this.instances5.alice.encrypt32(2578227177n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 1 (18444814847193612111, 3675625287)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(18444814847193612111n),
      this.instances5.alice.encrypt32(3675625287n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 2 (3675625283, 3675625287)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(3675625283n),
      this.instances5.alice.encrypt32(3675625287n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 3 (3675625287, 3675625287)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(3675625287n),
      this.instances5.alice.encrypt32(3675625287n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 4 (3675625287, 3675625283)', async function () {
    const res = await this.contract5.eq_euint64_euint32(
      this.instances5.alice.encrypt64(3675625287n),
      this.instances5.alice.encrypt32(3675625283n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 1 (18442332740658370363, 1304399628)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(18442332740658370363n),
      this.instances5.alice.encrypt32(1304399628n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 2 (1304399624, 1304399628)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(1304399624n),
      this.instances5.alice.encrypt32(1304399628n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 3 (1304399628, 1304399628)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(1304399628n),
      this.instances5.alice.encrypt32(1304399628n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 4 (1304399628, 1304399624)', async function () {
    const res = await this.contract5.ne_euint64_euint32(
      this.instances5.alice.encrypt64(1304399628n),
      this.instances5.alice.encrypt32(1304399624n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 1 (18443677328726027173, 2889413053)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(18443677328726027173n),
      this.instances5.alice.encrypt32(2889413053n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 2 (2889413049, 2889413053)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(2889413049n),
      this.instances5.alice.encrypt32(2889413053n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 3 (2889413053, 2889413053)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(2889413053n),
      this.instances5.alice.encrypt32(2889413053n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 4 (2889413053, 2889413049)', async function () {
    const res = await this.contract5.ge_euint64_euint32(
      this.instances5.alice.encrypt64(2889413053n),
      this.instances5.alice.encrypt32(2889413049n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 1 (18445619408455524051, 2595159918)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(18445619408455524051n),
      this.instances5.alice.encrypt32(2595159918n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 2 (2595159914, 2595159918)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(2595159914n),
      this.instances5.alice.encrypt32(2595159918n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 3 (2595159918, 2595159918)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(2595159918n),
      this.instances5.alice.encrypt32(2595159918n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 4 (2595159918, 2595159914)', async function () {
    const res = await this.contract5.gt_euint64_euint32(
      this.instances5.alice.encrypt64(2595159918n),
      this.instances5.alice.encrypt32(2595159914n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (18439009396144568585, 2271345781)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(18439009396144568585n),
      this.instances5.alice.encrypt32(2271345781n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (2271345777, 2271345781)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(2271345777n),
      this.instances5.alice.encrypt32(2271345781n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (2271345781, 2271345781)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(2271345781n),
      this.instances5.alice.encrypt32(2271345781n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (2271345781, 2271345777)', async function () {
    const res = await this.contract5.le_euint64_euint32(
      this.instances5.alice.encrypt64(2271345781n),
      this.instances5.alice.encrypt32(2271345777n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (18439663977987842087, 2158000734)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(18439663977987842087n),
      this.instances5.alice.encrypt32(2158000734n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (2158000730, 2158000734)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(2158000730n),
      this.instances5.alice.encrypt32(2158000734n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (2158000734, 2158000734)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(2158000734n),
      this.instances5.alice.encrypt32(2158000734n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (2158000734, 2158000730)', async function () {
    const res = await this.contract5.lt_euint64_euint32(
      this.instances5.alice.encrypt64(2158000734n),
      this.instances5.alice.encrypt32(2158000730n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (18442182412102668107, 657148413)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(18442182412102668107n),
      this.instances5.alice.encrypt32(657148413n),
    );
    expect(res).to.equal(657148413n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (657148409, 657148413)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(657148409n),
      this.instances5.alice.encrypt32(657148413n),
    );
    expect(res).to.equal(657148409n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (657148413, 657148413)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(657148413n),
      this.instances5.alice.encrypt32(657148413n),
    );
    expect(res).to.equal(657148413n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (657148413, 657148409)', async function () {
    const res = await this.contract5.min_euint64_euint32(
      this.instances5.alice.encrypt64(657148413n),
      this.instances5.alice.encrypt32(657148409n),
    );
    expect(res).to.equal(657148409n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (18439662390589911437, 4249693397)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(18439662390589911437n),
      this.instances5.alice.encrypt32(4249693397n),
    );
    expect(res).to.equal(18439662390589911437n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (4249693393, 4249693397)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(4249693393n),
      this.instances5.alice.encrypt32(4249693397n),
    );
    expect(res).to.equal(4249693397n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (4249693397, 4249693397)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(4249693397n),
      this.instances5.alice.encrypt32(4249693397n),
    );
    expect(res).to.equal(4249693397n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (4249693397, 4249693393)', async function () {
    const res = await this.contract5.max_euint64_euint32(
      this.instances5.alice.encrypt64(4249693397n),
      this.instances5.alice.encrypt32(4249693393n),
    );
    expect(res).to.equal(4249693397n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (9223329218882461797, 9219964371310000511)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(9223329218882461797n),
      this.instances5.alice.encrypt64(9219964371310000511n),
    );
    expect(res).to.equal(18443293590192462308n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (9219964371310000509, 9219964371310000511)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(9219964371310000509n),
      this.instances5.alice.encrypt64(9219964371310000511n),
    );
    expect(res).to.equal(18439928742620001020n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (9219964371310000511, 9219964371310000511)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(9219964371310000511n),
      this.instances5.alice.encrypt64(9219964371310000511n),
    );
    expect(res).to.equal(18439928742620001022n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (9219964371310000511, 9219964371310000509)', async function () {
    const res = await this.contract5.add_euint64_euint64(
      this.instances5.alice.encrypt64(9219964371310000511n),
      this.instances5.alice.encrypt64(9219964371310000509n),
    );
    expect(res).to.equal(18439928742620001020n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (18445117613821089157, 18445117613821089157)', async function () {
    const res = await this.contract5.sub_euint64_euint64(
      this.instances5.alice.encrypt64(18445117613821089157n),
      this.instances5.alice.encrypt64(18445117613821089157n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (18445117613821089157, 18445117613821089153)', async function () {
    const res = await this.contract5.sub_euint64_euint64(
      this.instances5.alice.encrypt64(18445117613821089157n),
      this.instances5.alice.encrypt64(18445117613821089153n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (4294635170, 4293288604)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(4294635170n),
      this.instances5.alice.encrypt64(4293288604n),
    );
    expect(res).to.equal(18438108233698602680n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(4293288604n),
      this.instances5.alice.encrypt64(4293288604n),
    );
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(4293288604n),
      this.instances5.alice.encrypt64(4293288604n),
    );
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_euint64_euint64(
      this.instances5.alice.encrypt64(4293288604n),
      this.instances5.alice.encrypt64(4293288604n),
    );
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (18441848963293247005, 18437762817766608073)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(18441848963293247005n),
      this.instances5.alice.encrypt64(18437762817766608073n),
    );
    expect(res).to.equal(18437758350371472393n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (18437762817766608069, 18437762817766608073)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(18437762817766608069n),
      this.instances5.alice.encrypt64(18437762817766608073n),
    );
    expect(res).to.equal(18437762817766608065n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (18437762817766608073, 18437762817766608073)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(18437762817766608073n),
      this.instances5.alice.encrypt64(18437762817766608073n),
    );
    expect(res).to.equal(18437762817766608073n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (18437762817766608073, 18437762817766608069)', async function () {
    const res = await this.contract5.and_euint64_euint64(
      this.instances5.alice.encrypt64(18437762817766608073n),
      this.instances5.alice.encrypt64(18437762817766608069n),
    );
    expect(res).to.equal(18437762817766608065n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 1 (18439947486770357681, 18442205516883919361)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(18439947486770357681n),
      this.instances5.alice.encrypt64(18442205516883919361n),
    );
    expect(res).to.equal(18442234697046943665n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 2 (18439947486770357677, 18439947486770357681)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(18439947486770357677n),
      this.instances5.alice.encrypt64(18439947486770357681n),
    );
    expect(res).to.equal(18439947486770357693n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 3 (18439947486770357681, 18439947486770357681)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(18439947486770357681n),
      this.instances5.alice.encrypt64(18439947486770357681n),
    );
    expect(res).to.equal(18439947486770357681n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 4 (18439947486770357681, 18439947486770357677)', async function () {
    const res = await this.contract5.or_euint64_euint64(
      this.instances5.alice.encrypt64(18439947486770357681n),
      this.instances5.alice.encrypt64(18439947486770357677n),
    );
    expect(res).to.equal(18439947486770357693n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 1 (18444700078916958431, 18443866708631144651)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(18444700078916958431n),
      this.instances5.alice.encrypt64(18443866708631144651n),
    );
    expect(res).to.equal(3795455568392212n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 2 (18443866708631144647, 18443866708631144651)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(18443866708631144647n),
      this.instances5.alice.encrypt64(18443866708631144651n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 3 (18443866708631144651, 18443866708631144651)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(18443866708631144651n),
      this.instances5.alice.encrypt64(18443866708631144651n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 4 (18443866708631144651, 18443866708631144647)', async function () {
    const res = await this.contract5.xor_euint64_euint64(
      this.instances5.alice.encrypt64(18443866708631144651n),
      this.instances5.alice.encrypt64(18443866708631144647n),
    );
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 1 (18443330521266220729, 18438253731135327627)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(18443330521266220729n),
      this.instances5.alice.encrypt64(18438253731135327627n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 2 (18438253731135327623, 18438253731135327627)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(18438253731135327623n),
      this.instances5.alice.encrypt64(18438253731135327627n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 3 (18438253731135327627, 18438253731135327627)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(18438253731135327627n),
      this.instances5.alice.encrypt64(18438253731135327627n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 4 (18438253731135327627, 18438253731135327623)', async function () {
    const res = await this.contract5.eq_euint64_euint64(
      this.instances5.alice.encrypt64(18438253731135327627n),
      this.instances5.alice.encrypt64(18438253731135327623n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (18445140354518938845, 18441391037965649995)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(18445140354518938845n),
      this.instances5.alice.encrypt64(18441391037965649995n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (18441391037965649991, 18441391037965649995)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(18441391037965649991n),
      this.instances5.alice.encrypt64(18441391037965649995n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (18441391037965649995, 18441391037965649995)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(18441391037965649995n),
      this.instances5.alice.encrypt64(18441391037965649995n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (18441391037965649995, 18441391037965649991)', async function () {
    const res = await this.contract5.ne_euint64_euint64(
      this.instances5.alice.encrypt64(18441391037965649995n),
      this.instances5.alice.encrypt64(18441391037965649991n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 1 (18444991478795579145, 18445260307161364245)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(18444991478795579145n),
      this.instances5.alice.encrypt64(18445260307161364245n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 2 (18444991478795579141, 18444991478795579145)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(18444991478795579141n),
      this.instances5.alice.encrypt64(18444991478795579145n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 3 (18444991478795579145, 18444991478795579145)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(18444991478795579145n),
      this.instances5.alice.encrypt64(18444991478795579145n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 4 (18444991478795579145, 18444991478795579141)', async function () {
    const res = await this.contract5.ge_euint64_euint64(
      this.instances5.alice.encrypt64(18444991478795579145n),
      this.instances5.alice.encrypt64(18444991478795579141n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (18439787790330435145, 18439484090308827429)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(18439787790330435145n),
      this.instances5.alice.encrypt64(18439484090308827429n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (18439484090308827425, 18439484090308827429)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(18439484090308827425n),
      this.instances5.alice.encrypt64(18439484090308827429n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (18439484090308827429, 18439484090308827429)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(18439484090308827429n),
      this.instances5.alice.encrypt64(18439484090308827429n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (18439484090308827429, 18439484090308827425)', async function () {
    const res = await this.contract5.gt_euint64_euint64(
      this.instances5.alice.encrypt64(18439484090308827429n),
      this.instances5.alice.encrypt64(18439484090308827425n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (18440769778451615393, 18446070761608442971)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(18440769778451615393n),
      this.instances5.alice.encrypt64(18446070761608442971n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (18440769778451615389, 18440769778451615393)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(18440769778451615389n),
      this.instances5.alice.encrypt64(18440769778451615393n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (18440769778451615393, 18440769778451615393)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(18440769778451615393n),
      this.instances5.alice.encrypt64(18440769778451615393n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (18440769778451615393, 18440769778451615389)', async function () {
    const res = await this.contract5.le_euint64_euint64(
      this.instances5.alice.encrypt64(18440769778451615393n),
      this.instances5.alice.encrypt64(18440769778451615389n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 1 (18446718131340158589, 18444160910497783341)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(18446718131340158589n),
      this.instances5.alice.encrypt64(18444160910497783341n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 2 (18444160910497783337, 18444160910497783341)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(18444160910497783337n),
      this.instances5.alice.encrypt64(18444160910497783341n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 3 (18444160910497783341, 18444160910497783341)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(18444160910497783341n),
      this.instances5.alice.encrypt64(18444160910497783341n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 4 (18444160910497783341, 18444160910497783337)', async function () {
    const res = await this.contract5.lt_euint64_euint64(
      this.instances5.alice.encrypt64(18444160910497783341n),
      this.instances5.alice.encrypt64(18444160910497783337n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 1 (18444400472074074345, 18442962239103377481)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(18444400472074074345n),
      this.instances5.alice.encrypt64(18442962239103377481n),
    );
    expect(res).to.equal(18442962239103377481n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 2 (18442962239103377477, 18442962239103377481)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(18442962239103377477n),
      this.instances5.alice.encrypt64(18442962239103377481n),
    );
    expect(res).to.equal(18442962239103377477n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 3 (18442962239103377481, 18442962239103377481)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(18442962239103377481n),
      this.instances5.alice.encrypt64(18442962239103377481n),
    );
    expect(res).to.equal(18442962239103377481n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 4 (18442962239103377481, 18442962239103377477)', async function () {
    const res = await this.contract5.min_euint64_euint64(
      this.instances5.alice.encrypt64(18442962239103377481n),
      this.instances5.alice.encrypt64(18442962239103377477n),
    );
    expect(res).to.equal(18442962239103377477n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (18440739371866435289, 18438298584940765731)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18440739371866435289n),
      this.instances5.alice.encrypt64(18438298584940765731n),
    );
    expect(res).to.equal(18440739371866435289n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (18438298584940765727, 18438298584940765731)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18438298584940765727n),
      this.instances5.alice.encrypt64(18438298584940765731n),
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (18438298584940765731, 18438298584940765731)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18438298584940765731n),
      this.instances5.alice.encrypt64(18438298584940765731n),
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (18438298584940765731, 18438298584940765727)', async function () {
    const res = await this.contract5.max_euint64_euint64(
      this.instances5.alice.encrypt64(18438298584940765731n),
      this.instances5.alice.encrypt64(18438298584940765727n),
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (9223329218882461797, 9220956803715422232)', async function () {
    const res = await this.contract5.add_euint64_uint64(
      this.instances5.alice.encrypt64(9223329218882461797n),
      9220956803715422232,
    );
    expect(res).to.equal(18444286022597884029n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (9219964371310000509, 9219964371310000511)', async function () {
    const res = await this.contract5.add_euint64_uint64(
      this.instances5.alice.encrypt64(9219964371310000509n),
      9219964371310000511,
    );
    expect(res).to.equal(18439928742620001020n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (9219964371310000511, 9219964371310000511)', async function () {
    const res = await this.contract5.add_euint64_uint64(
      this.instances5.alice.encrypt64(9219964371310000511n),
      9219964371310000511,
    );
    expect(res).to.equal(18439928742620001022n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (9219964371310000511, 9219964371310000509)', async function () {
    const res = await this.contract5.add_euint64_uint64(
      this.instances5.alice.encrypt64(9219964371310000511n),
      9219964371310000509,
    );
    expect(res).to.equal(18439928742620001020n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 1 (9219177655732910821, 9220956803715422232)', async function () {
    const res = await this.contract5.add_uint64_euint64(
      9219177655732910821,
      this.instances5.alice.encrypt64(9220956803715422232n),
    );
    expect(res).to.equal(18440134459448333053n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 2 (9219964371310000509, 9219964371310000511)', async function () {
    const res = await this.contract5.add_uint64_euint64(
      9219964371310000509,
      this.instances5.alice.encrypt64(9219964371310000511n),
    );
    expect(res).to.equal(18439928742620001020n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 3 (9219964371310000511, 9219964371310000511)', async function () {
    const res = await this.contract5.add_uint64_euint64(
      9219964371310000511,
      this.instances5.alice.encrypt64(9219964371310000511n),
    );
    expect(res).to.equal(18439928742620001022n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 4 (9219964371310000511, 9219964371310000509)', async function () {
    const res = await this.contract5.add_uint64_euint64(
      9219964371310000511,
      this.instances5.alice.encrypt64(9219964371310000509n),
    );
    expect(res).to.equal(18439928742620001020n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (18445117613821089157, 18445117613821089157)', async function () {
    const res = await this.contract5.sub_euint64_uint64(
      this.instances5.alice.encrypt64(18445117613821089157n),
      18445117613821089157,
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (18445117613821089157, 18445117613821089153)', async function () {
    const res = await this.contract5.sub_euint64_uint64(
      this.instances5.alice.encrypt64(18445117613821089157n),
      18445117613821089153,
    );
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (18445117613821089157, 18445117613821089157)', async function () {
    const res = await this.contract5.sub_uint64_euint64(
      18445117613821089157,
      this.instances5.alice.encrypt64(18445117613821089157n),
    );
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (18445117613821089157, 18445117613821089153)', async function () {
    const res = await this.contract5.sub_uint64_euint64(
      18445117613821089157,
      this.instances5.alice.encrypt64(18445117613821089153n),
    );
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (4294635170, 4293232253)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(4294635170n), 4293232253);
    expect(res).to.equal(18437866226712138010n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(4293288604n), 4293288604);
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(4293288604n), 4293288604);
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_euint64_uint64(this.instances5.alice.encrypt64(4293288604n), 4293288604);
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 1 (4294226236, 4293232253)', async function () {
    const res = await this.contract5.mul_uint64_euint64(4294226236, this.instances5.alice.encrypt64(4293232253n));
    expect(res).to.equal(18436110578073989708n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 2 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_uint64_euint64(4293288604, this.instances5.alice.encrypt64(4293288604n));
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 3 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_uint64_euint64(4293288604, this.instances5.alice.encrypt64(4293288604n));
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 4 (4293288604, 4293288604)', async function () {
    const res = await this.contract5.mul_uint64_euint64(4293288604, this.instances5.alice.encrypt64(4293288604n));
    expect(res).to.equal(18432327037236268816n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (18441976837575510865, 18441212274805422577)', async function () {
    const res = await this.contract5.div_euint64_uint64(
      this.instances5.alice.encrypt64(18441976837575510865n),
      18441212274805422577,
    );
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (18441976837575510861, 18441976837575510865)', async function () {
    const res = await this.contract5.div_euint64_uint64(
      this.instances5.alice.encrypt64(18441976837575510861n),
      18441976837575510865,
    );
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (18441976837575510865, 18441976837575510865)', async function () {
    const res = await this.contract5.div_euint64_uint64(
      this.instances5.alice.encrypt64(18441976837575510865n),
      18441976837575510865,
    );
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (18441976837575510865, 18441976837575510861)', async function () {
    const res = await this.contract5.div_euint64_uint64(
      this.instances5.alice.encrypt64(18441976837575510865n),
      18441976837575510861,
    );
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 1 (18443785129295236141, 18441307989286811147)', async function () {
    const res = await this.contract5.rem_euint64_uint64(
      this.instances5.alice.encrypt64(18443785129295236141n),
      18441307989286811147,
    );
    expect(res).to.equal(2477140008424994n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 2 (18438390548915069819, 18438390548915069823)', async function () {
    const res = await this.contract5.rem_euint64_uint64(
      this.instances5.alice.encrypt64(18438390548915069819n),
      18438390548915069823,
    );
    expect(res).to.equal(18438390548915069819n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 3 (18438390548915069823, 18438390548915069823)', async function () {
    const res = await this.contract5.rem_euint64_uint64(
      this.instances5.alice.encrypt64(18438390548915069823n),
      18438390548915069823,
    );
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 4 (18438390548915069823, 18438390548915069819)', async function () {
    const res = await this.contract5.rem_euint64_uint64(
      this.instances5.alice.encrypt64(18438390548915069823n),
      18438390548915069819,
    );
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (18443330521266220729, 18446706410531688277)', async function () {
    const res = await this.contract5.eq_euint64_uint64(
      this.instances5.alice.encrypt64(18443330521266220729n),
      18446706410531688277,
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (18438253731135327623, 18438253731135327627)', async function () {
    const res = await this.contract5.eq_euint64_uint64(
      this.instances5.alice.encrypt64(18438253731135327623n),
      18438253731135327627,
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (18438253731135327627, 18438253731135327627)', async function () {
    const res = await this.contract5.eq_euint64_uint64(
      this.instances5.alice.encrypt64(18438253731135327627n),
      18438253731135327627,
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (18438253731135327627, 18438253731135327623)', async function () {
    const res = await this.contract5.eq_euint64_uint64(
      this.instances5.alice.encrypt64(18438253731135327627n),
      18438253731135327623,
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (18444395277752785729, 18446706410531688277)', async function () {
    const res = await this.contract5.eq_uint64_euint64(
      18444395277752785729,
      this.instances5.alice.encrypt64(18446706410531688277n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (18438253731135327623, 18438253731135327627)', async function () {
    const res = await this.contract5.eq_uint64_euint64(
      18438253731135327623,
      this.instances5.alice.encrypt64(18438253731135327627n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (18438253731135327627, 18438253731135327627)', async function () {
    const res = await this.contract5.eq_uint64_euint64(
      18438253731135327627,
      this.instances5.alice.encrypt64(18438253731135327627n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (18438253731135327627, 18438253731135327623)', async function () {
    const res = await this.contract5.eq_uint64_euint64(
      18438253731135327627,
      this.instances5.alice.encrypt64(18438253731135327623n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (18445140354518938845, 18438176226766160787)', async function () {
    const res = await this.contract5.ne_euint64_uint64(
      this.instances5.alice.encrypt64(18445140354518938845n),
      18438176226766160787,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (18441391037965649991, 18441391037965649995)', async function () {
    const res = await this.contract5.ne_euint64_uint64(
      this.instances5.alice.encrypt64(18441391037965649991n),
      18441391037965649995,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (18441391037965649995, 18441391037965649995)', async function () {
    const res = await this.contract5.ne_euint64_uint64(
      this.instances5.alice.encrypt64(18441391037965649995n),
      18441391037965649995,
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (18441391037965649995, 18441391037965649991)', async function () {
    const res = await this.contract5.ne_euint64_uint64(
      this.instances5.alice.encrypt64(18441391037965649995n),
      18441391037965649991,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (18443547473224968383, 18438176226766160787)', async function () {
    const res = await this.contract5.ne_uint64_euint64(
      18443547473224968383,
      this.instances5.alice.encrypt64(18438176226766160787n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (18441391037965649991, 18441391037965649995)', async function () {
    const res = await this.contract5.ne_uint64_euint64(
      18441391037965649991,
      this.instances5.alice.encrypt64(18441391037965649995n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (18441391037965649995, 18441391037965649995)', async function () {
    const res = await this.contract5.ne_uint64_euint64(
      18441391037965649995,
      this.instances5.alice.encrypt64(18441391037965649995n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (18441391037965649995, 18441391037965649991)', async function () {
    const res = await this.contract5.ne_uint64_euint64(
      18441391037965649995,
      this.instances5.alice.encrypt64(18441391037965649991n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (18444991478795579145, 18439567451994245465)', async function () {
    const res = await this.contract5.ge_euint64_uint64(
      this.instances5.alice.encrypt64(18444991478795579145n),
      18439567451994245465,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (18444991478795579141, 18444991478795579145)', async function () {
    const res = await this.contract5.ge_euint64_uint64(
      this.instances5.alice.encrypt64(18444991478795579141n),
      18444991478795579145,
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (18444991478795579145, 18444991478795579145)', async function () {
    const res = await this.contract5.ge_euint64_uint64(
      this.instances5.alice.encrypt64(18444991478795579145n),
      18444991478795579145,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (18444991478795579145, 18444991478795579141)', async function () {
    const res = await this.contract5.ge_euint64_uint64(
      this.instances5.alice.encrypt64(18444991478795579145n),
      18444991478795579141,
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (18444429093181704535, 18439567451994245465)', async function () {
    const res = await this.contract5.ge_uint64_euint64(
      18444429093181704535,
      this.instances5.alice.encrypt64(18439567451994245465n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (18444991478795579141, 18444991478795579145)', async function () {
    const res = await this.contract5.ge_uint64_euint64(
      18444991478795579141,
      this.instances5.alice.encrypt64(18444991478795579145n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (18444991478795579145, 18444991478795579145)', async function () {
    const res = await this.contract5.ge_uint64_euint64(
      18444991478795579145,
      this.instances5.alice.encrypt64(18444991478795579145n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (18444991478795579145, 18444991478795579141)', async function () {
    const res = await this.contract5.ge_uint64_euint64(
      18444991478795579145,
      this.instances5.alice.encrypt64(18444991478795579141n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (18439787790330435145, 18441907321511169065)', async function () {
    const res = await this.contract5.gt_euint64_uint64(
      this.instances5.alice.encrypt64(18439787790330435145n),
      18441907321511169065,
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (18439484090308827425, 18439484090308827429)', async function () {
    const res = await this.contract5.gt_euint64_uint64(
      this.instances5.alice.encrypt64(18439484090308827425n),
      18439484090308827429,
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (18439484090308827429, 18439484090308827429)', async function () {
    const res = await this.contract5.gt_euint64_uint64(
      this.instances5.alice.encrypt64(18439484090308827429n),
      18439484090308827429,
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (18439484090308827429, 18439484090308827425)', async function () {
    const res = await this.contract5.gt_euint64_uint64(
      this.instances5.alice.encrypt64(18439484090308827429n),
      18439484090308827425,
    );
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 1 (18438935380134710315, 18441907321511169065)', async function () {
    const res = await this.contract5.gt_uint64_euint64(
      18438935380134710315,
      this.instances5.alice.encrypt64(18441907321511169065n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 2 (18439484090308827425, 18439484090308827429)', async function () {
    const res = await this.contract5.gt_uint64_euint64(
      18439484090308827425,
      this.instances5.alice.encrypt64(18439484090308827429n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 3 (18439484090308827429, 18439484090308827429)', async function () {
    const res = await this.contract5.gt_uint64_euint64(
      18439484090308827429,
      this.instances5.alice.encrypt64(18439484090308827429n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 4 (18439484090308827429, 18439484090308827425)', async function () {
    const res = await this.contract5.gt_uint64_euint64(
      18439484090308827429,
      this.instances5.alice.encrypt64(18439484090308827425n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (18440769778451615393, 18439065451314752761)', async function () {
    const res = await this.contract5.le_euint64_uint64(
      this.instances5.alice.encrypt64(18440769778451615393n),
      18439065451314752761,
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (18440769778451615389, 18440769778451615393)', async function () {
    const res = await this.contract5.le_euint64_uint64(
      this.instances5.alice.encrypt64(18440769778451615389n),
      18440769778451615393,
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (18440769778451615393, 18440769778451615393)', async function () {
    const res = await this.contract5.le_euint64_uint64(
      this.instances5.alice.encrypt64(18440769778451615393n),
      18440769778451615393,
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (18440769778451615393, 18440769778451615389)', async function () {
    const res = await this.contract5.le_euint64_uint64(
      this.instances5.alice.encrypt64(18440769778451615393n),
      18440769778451615389,
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (18440980092932624951, 18439065451314752761)', async function () {
    const res = await this.contract5.le_uint64_euint64(
      18440980092932624951,
      this.instances5.alice.encrypt64(18439065451314752761n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (18440769778451615389, 18440769778451615393)', async function () {
    const res = await this.contract5.le_uint64_euint64(
      18440769778451615389,
      this.instances5.alice.encrypt64(18440769778451615393n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (18440769778451615393, 18440769778451615393)', async function () {
    const res = await this.contract5.le_uint64_euint64(
      18440769778451615393,
      this.instances5.alice.encrypt64(18440769778451615393n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (18440769778451615393, 18440769778451615389)', async function () {
    const res = await this.contract5.le_uint64_euint64(
      18440769778451615393,
      this.instances5.alice.encrypt64(18440769778451615389n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (18446718131340158589, 18438438177494413269)', async function () {
    const res = await this.contract5.lt_euint64_uint64(
      this.instances5.alice.encrypt64(18446718131340158589n),
      18438438177494413269,
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (18444160910497783337, 18444160910497783341)', async function () {
    const res = await this.contract5.lt_euint64_uint64(
      this.instances5.alice.encrypt64(18444160910497783337n),
      18444160910497783341,
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (18444160910497783341, 18444160910497783341)', async function () {
    const res = await this.contract5.lt_euint64_uint64(
      this.instances5.alice.encrypt64(18444160910497783341n),
      18444160910497783341,
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (18444160910497783341, 18444160910497783337)', async function () {
    const res = await this.contract5.lt_euint64_uint64(
      this.instances5.alice.encrypt64(18444160910497783341n),
      18444160910497783337,
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (18445719507413937869, 18438438177494413269)', async function () {
    const res = await this.contract5.lt_uint64_euint64(
      18445719507413937869,
      this.instances5.alice.encrypt64(18438438177494413269n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (18444160910497783337, 18444160910497783341)', async function () {
    const res = await this.contract5.lt_uint64_euint64(
      18444160910497783337,
      this.instances5.alice.encrypt64(18444160910497783341n),
    );
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (18444160910497783341, 18444160910497783341)', async function () {
    const res = await this.contract5.lt_uint64_euint64(
      18444160910497783341,
      this.instances5.alice.encrypt64(18444160910497783341n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (18444160910497783341, 18444160910497783337)', async function () {
    const res = await this.contract5.lt_uint64_euint64(
      18444160910497783341,
      this.instances5.alice.encrypt64(18444160910497783337n),
    );
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 1 (18444400472074074345, 18445675871085860653)', async function () {
    const res = await this.contract5.min_euint64_uint64(
      this.instances5.alice.encrypt64(18444400472074074345n),
      18445675871085860653,
    );
    expect(res).to.equal(18444400472074074345n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 2 (18442962239103377477, 18442962239103377481)', async function () {
    const res = await this.contract5.min_euint64_uint64(
      this.instances5.alice.encrypt64(18442962239103377477n),
      18442962239103377481,
    );
    expect(res).to.equal(18442962239103377477n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 3 (18442962239103377481, 18442962239103377481)', async function () {
    const res = await this.contract5.min_euint64_uint64(
      this.instances5.alice.encrypt64(18442962239103377481n),
      18442962239103377481,
    );
    expect(res).to.equal(18442962239103377481n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 4 (18442962239103377481, 18442962239103377477)', async function () {
    const res = await this.contract5.min_euint64_uint64(
      this.instances5.alice.encrypt64(18442962239103377481n),
      18442962239103377477,
    );
    expect(res).to.equal(18442962239103377477n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (18443908139931756717, 18445675871085860653)', async function () {
    const res = await this.contract5.min_uint64_euint64(
      18443908139931756717,
      this.instances5.alice.encrypt64(18445675871085860653n),
    );
    expect(res).to.equal(18443908139931756717n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (18442962239103377477, 18442962239103377481)', async function () {
    const res = await this.contract5.min_uint64_euint64(
      18442962239103377477,
      this.instances5.alice.encrypt64(18442962239103377481n),
    );
    expect(res).to.equal(18442962239103377477n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (18442962239103377481, 18442962239103377481)', async function () {
    const res = await this.contract5.min_uint64_euint64(
      18442962239103377481,
      this.instances5.alice.encrypt64(18442962239103377481n),
    );
    expect(res).to.equal(18442962239103377481n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (18442962239103377481, 18442962239103377477)', async function () {
    const res = await this.contract5.min_uint64_euint64(
      18442962239103377481,
      this.instances5.alice.encrypt64(18442962239103377477n),
    );
    expect(res).to.equal(18442962239103377477n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (18440739371866435289, 18440643015791741637)', async function () {
    const res = await this.contract5.max_euint64_uint64(
      this.instances5.alice.encrypt64(18440739371866435289n),
      18440643015791741637,
    );
    expect(res).to.equal(18440739371866435289n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (18438298584940765727, 18438298584940765731)', async function () {
    const res = await this.contract5.max_euint64_uint64(
      this.instances5.alice.encrypt64(18438298584940765727n),
      18438298584940765731,
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (18438298584940765731, 18438298584940765731)', async function () {
    const res = await this.contract5.max_euint64_uint64(
      this.instances5.alice.encrypt64(18438298584940765731n),
      18438298584940765731,
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (18438298584940765731, 18438298584940765727)', async function () {
    const res = await this.contract5.max_euint64_uint64(
      this.instances5.alice.encrypt64(18438298584940765731n),
      18438298584940765727,
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (18441357041435050863, 18440643015791741637)', async function () {
    const res = await this.contract5.max_uint64_euint64(
      18441357041435050863,
      this.instances5.alice.encrypt64(18440643015791741637n),
    );
    expect(res).to.equal(18441357041435050863n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18438298584940765727, 18438298584940765731)', async function () {
    const res = await this.contract5.max_uint64_euint64(
      18438298584940765727,
      this.instances5.alice.encrypt64(18438298584940765731n),
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18438298584940765731, 18438298584940765731)', async function () {
    const res = await this.contract5.max_uint64_euint64(
      18438298584940765731,
      this.instances5.alice.encrypt64(18438298584940765731n),
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18438298584940765731, 18438298584940765727)', async function () {
    const res = await this.contract5.max_uint64_euint64(
      18438298584940765731,
      this.instances5.alice.encrypt64(18438298584940765727n),
    );
    expect(res).to.equal(18438298584940765731n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 1 (13, 5)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(13n), 5);
    expect(res).to.equal(10);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(4n), 8);
    expect(res).to.equal(4);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(8n), 8);
    expect(res).to.equal(8);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint4_uint8(this.instances5.alice.encrypt4(8n), 4);
    expect(res).to.equal(8);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 1 (14, 4)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(14n), 4);
    expect(res).to.equal(14);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(4n), 8);
    expect(res).to.equal(4);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(8n), 8);
    expect(res).to.equal(8);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint4_uint8(this.instances5.alice.encrypt4(8n), 4);
    expect(res).to.equal(8);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (34, 4)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(34n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(32);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(4);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(8);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint8_euint8(
      this.instances5.alice.encrypt8(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(128);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (34, 4)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(34n), 4);
    expect(res).to.equal(32);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(4n), 8);
    expect(res).to.equal(4);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(8n), 8);
    expect(res).to.equal(8);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint8_uint8(this.instances5.alice.encrypt8(8n), 4);
    expect(res).to.equal(128);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (68, 4)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(68n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(4);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(4);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(8);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint8_euint8(
      this.instances5.alice.encrypt8(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (68, 4)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(68n), 4);
    expect(res).to.equal(4);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(4n), 8);
    expect(res).to.equal(4);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(8n), 8);
    expect(res).to.equal(8);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint8_uint8(this.instances5.alice.encrypt8(8n), 4);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (15362, 2)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(15362n),
      this.instances5.alice.encrypt8(2n),
    );
    expect(res).to.equal(61448);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(1024);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(2048);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint16_euint8(
      this.instances5.alice.encrypt16(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(128);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 1 (15362, 2)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(15362n), 2);
    expect(res).to.equal(61448);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(4n), 8);
    expect(res).to.equal(1024);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(8n), 8);
    expect(res).to.equal(2048);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint16_uint8(this.instances5.alice.encrypt16(8n), 4);
    expect(res).to.equal(128);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (25648, 5)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(25648n),
      this.instances5.alice.encrypt8(5n),
    );
    expect(res).to.equal(801);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint16_euint8(
      this.instances5.alice.encrypt16(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (25648, 5)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(25648n), 5);
    expect(res).to.equal(801);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(4n), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(8n), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint16_uint8(this.instances5.alice.encrypt16(8n), 4);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (833510670, 1)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(833510670n),
      this.instances5.alice.encrypt8(1n),
    );
    expect(res).to.equal(1667021340);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(1024);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(2048);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint32_euint8(
      this.instances5.alice.encrypt32(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(128);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 1 (833510670, 1)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(833510670n), 1);
    expect(res).to.equal(1667021340);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(4n), 8);
    expect(res).to.equal(1024);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(8n), 8);
    expect(res).to.equal(2048);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint32_uint8(this.instances5.alice.encrypt32(8n), 4);
    expect(res).to.equal(128);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (3957313401, 6)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(3957313401n),
      this.instances5.alice.encrypt8(6n),
    );
    expect(res).to.equal(61833021);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint32_euint8(
      this.instances5.alice.encrypt32(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 1 (3957313401, 6)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(3957313401n), 6);
    expect(res).to.equal(61833021);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(4n), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(8n), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint32_uint8(this.instances5.alice.encrypt32(8n), 4);
    expect(res).to.equal(0);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 1 (18445451452906630791, 5)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(18445451452906630791n),
      this.instances5.alice.encrypt8(5n),
    );
    expect(res).to.equal(18405380208016085000);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(1024);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(2048);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint64_euint8(
      this.instances5.alice.encrypt64(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(128);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (18445451452906630791, 5)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(18445451452906630791n), 5);
    expect(res).to.equal(18405380208016085000);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(4n), 8);
    expect(res).to.equal(1024);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(8n), 8);
    expect(res).to.equal(2048);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.shl_euint64_uint8(this.instances5.alice.encrypt64(8n), 4);
    expect(res).to.equal(128);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (18439569308403000305, 1)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(18439569308403000305n),
      this.instances5.alice.encrypt8(1n),
    );
    expect(res).to.equal(9219784654201500000);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(4n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(8n),
      this.instances5.alice.encrypt8(8n),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint64_euint8(
      this.instances5.alice.encrypt64(8n),
      this.instances5.alice.encrypt8(4n),
    );
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (18439569308403000305, 1)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(18439569308403000305n), 1);
    expect(res).to.equal(9219784654201500000);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(4n), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(8n), 8);
    expect(res).to.equal(0);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const res = await this.contract5.shr_euint64_uint8(this.instances5.alice.encrypt64(8n), 4);
    expect(res).to.equal(0);
  });

  it('test operator "neg" overload (euint4) => euint4 test 1 (11)', async function () {
    const res = await this.contract5.neg_euint4(this.instances5.alice.encrypt4(11n));
    expect(res).to.equal(5n);
  });

  it('test operator "not" overload (euint4) => euint4 test 1 (13)', async function () {
    const res = await this.contract5.not_euint4(this.instances5.alice.encrypt4(13n));
    expect(res).to.equal(2n);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (251)', async function () {
    const res = await this.contract5.neg_euint8(this.instances5.alice.encrypt8(251n));
    expect(res).to.equal(5n);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (187)', async function () {
    const res = await this.contract5.not_euint8(this.instances5.alice.encrypt8(187n));
    expect(res).to.equal(68n);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (25161)', async function () {
    const res = await this.contract5.neg_euint16(this.instances5.alice.encrypt16(25161n));
    expect(res).to.equal(40375n);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (60538)', async function () {
    const res = await this.contract5.not_euint16(this.instances5.alice.encrypt16(60538n));
    expect(res).to.equal(4997n);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (2156295218)', async function () {
    const res = await this.contract5.neg_euint32(this.instances5.alice.encrypt32(2156295218n));
    expect(res).to.equal(2138672078n);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (2475211657)', async function () {
    const res = await this.contract5.not_euint32(this.instances5.alice.encrypt32(2475211657n));
    expect(res).to.equal(1819755638n);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (18438244346234461619)', async function () {
    const res = await this.contract5.neg_euint64(this.instances5.alice.encrypt64(18438244346234461619n));
    expect(res).to.equal(8499727475089997n);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (18443955194473722291)', async function () {
    const res = await this.contract5.not_euint64(this.instances5.alice.encrypt64(18443955194473722291n));
    expect(res).to.equal(2788879235829324n);
  });
});
