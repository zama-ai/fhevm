import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { FHEVMTestSuite1 } from '../../typechain-types/examples/tests/FHEVMTestSuite1';
import type { FHEVMTestSuite2 } from '../../typechain-types/examples/tests/FHEVMTestSuite2';
import type { FHEVMTestSuite3 } from '../../typechain-types/examples/tests/FHEVMTestSuite3';
import type { FHEVMTestSuite4 } from '../../typechain-types/examples/tests/FHEVMTestSuite4';
import type { FHEVMTestSuite5 } from '../../typechain-types/examples/tests/FHEVMTestSuite5';
import type { FHEVMTestSuite6 } from '../../typechain-types/examples/tests/FHEVMTestSuite6';
import type { FHEVMTestSuite7 } from '../../typechain-types/examples/tests/FHEVMTestSuite7';
import {
  createInstances,
  decrypt8,
  decrypt16,
  decrypt32,
  decrypt64,
  decrypt128,
  decrypt256,
  decryptBool,
} from '../instance';
import { getSigners, initSigners } from '../signers';

async function deployFHEVMTestFixture1(): Promise<FHEVMTestSuite1> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract as unknown as FHEVMTestSuite1;
}

async function deployFHEVMTestFixture2(): Promise<FHEVMTestSuite2> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract as unknown as FHEVMTestSuite2;
}

async function deployFHEVMTestFixture3(): Promise<FHEVMTestSuite3> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract as unknown as FHEVMTestSuite3;
}

async function deployFHEVMTestFixture4(): Promise<FHEVMTestSuite4> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract as unknown as FHEVMTestSuite4;
}

async function deployFHEVMTestFixture5(): Promise<FHEVMTestSuite5> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract as unknown as FHEVMTestSuite5;
}

async function deployFHEVMTestFixture6(): Promise<FHEVMTestSuite6> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract as unknown as FHEVMTestSuite6;
}

async function deployFHEVMTestFixture7(): Promise<FHEVMTestSuite7> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite7');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract as unknown as FHEVMTestSuite7;
}

describe('FHEVM operations 2', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract1 = await deployFHEVMTestFixture1();
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployFHEVMTestFixture2();
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployFHEVMTestFixture3();
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployFHEVMTestFixture4();
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployFHEVMTestFixture5();
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployFHEVMTestFixture6();
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const contract7 = await deployFHEVMTestFixture7();
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 1 (57, 18437980839186346639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(57n);
    input.add64(18437980839186346639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 2 (53, 57)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(53n);
    input.add64(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 3 (57, 57)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(57n);
    input.add64(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 4 (57, 53)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(57n);
    input.add64(53n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 1 (120, 18438373286938013753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(120n);
    input.add64(18438373286938013753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 2 (116, 120)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(116n);
    input.add64(120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 3 (120, 120)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(120n);
    input.add64(120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 4 (120, 116)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(120n);
    input.add64(116n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 1 (13, 18442574269281988663)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(13n);
    input.add64(18442574269281988663n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(13n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add64(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(9n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(13n);
    input.add64(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(13n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(13n);
    input.add64(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(9n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 1 (250, 18439684605436224935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(250n);
    input.add64(18439684605436224935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(18439684605436224935n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 2 (246, 250)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(246n);
    input.add64(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(250n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 3 (250, 250)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(250n);
    input.add64(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(250n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 4 (250, 246)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(250n);
    input.add64(246n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(250n);
  });

  it('test operator "add" overload (euint8, euint128) => euint128 test 1 (2, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add128(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint8, euint128) => euint128 test 2 (115, 119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(115n);
    input.add128(119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(234n);
  });

  it('test operator "add" overload (euint8, euint128) => euint128 test 3 (119, 119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(119n);
    input.add128(119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(238n);
  });

  it('test operator "add" overload (euint8, euint128) => euint128 test 4 (119, 115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(119n);
    input.add128(115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(234n);
  });

  it('test operator "sub" overload (euint8, euint128) => euint128 test 1 (90, 90)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(90n);
    input.add128(90n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint128) => euint128 test 2 (90, 86)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(90n);
    input.add128(86n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint128) => euint128 test 1 (2, 65)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add128(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint8, euint128) => euint128 test 2 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add128(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint128) => euint128 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add128(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint128) => euint128 test 4 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add128(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(81n);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 1 (11, 340282366920938463463365821734732949511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(11n);
    input.add128(340282366920938463463365821734732949511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(3n);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(7n);
    input.add128(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(3n);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(11n);
    input.add128(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(11n);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(11n);
    input.add128(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(3n);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 1 (105, 340282366920938463463373526168922294789)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(105n);
    input.add128(340282366920938463463373526168922294789n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(340282366920938463463373526168922294893n);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 2 (101, 105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(101n);
    input.add128(105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(109n);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 3 (105, 105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(105n);
    input.add128(105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(105n);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 4 (105, 101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(105n);
    input.add128(101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(109n);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 1 (210, 340282366920938463463373913827547070557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(210n);
    input.add128(340282366920938463463373913827547070557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(340282366920938463463373913827547070607n);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 2 (206, 210)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(206n);
    input.add128(210n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 3 (210, 210)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(210n);
    input.add128(210n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 4 (210, 206)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(210n);
    input.add128(206n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint8, euint128) => ebool test 1 (57, 340282366920938463463373238229739312555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(57n);
    input.add128(340282366920938463463373238229739312555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint128) => ebool test 2 (53, 57)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(53n);
    input.add128(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint128) => ebool test 3 (57, 57)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(57n);
    input.add128(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint128) => ebool test 4 (57, 53)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(57n);
    input.add128(53n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 1 (90, 340282366920938463463371590597529804205)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(90n);
    input.add128(340282366920938463463371590597529804205n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 2 (86, 90)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(86n);
    input.add128(90n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 3 (90, 90)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(90n);
    input.add128(90n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 4 (90, 86)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(90n);
    input.add128(86n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint128) => ebool test 1 (244, 340282366920938463463374155923620006263)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(244n);
    input.add128(340282366920938463463374155923620006263n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint128) => ebool test 2 (240, 244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(240n);
    input.add128(244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint128) => ebool test 3 (244, 244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(244n);
    input.add128(244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint128) => ebool test 4 (244, 240)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(244n);
    input.add128(240n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint128) => ebool test 1 (145, 340282366920938463463368793491690468083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(145n);
    input.add128(340282366920938463463368793491690468083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint128) => ebool test 2 (141, 145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(141n);
    input.add128(145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint128) => ebool test 3 (145, 145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(145n);
    input.add128(145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint128) => ebool test 4 (145, 141)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(145n);
    input.add128(141n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint128) => ebool test 1 (219, 340282366920938463463374148550144340929)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(219n);
    input.add128(340282366920938463463374148550144340929n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint128) => ebool test 2 (215, 219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(215n);
    input.add128(219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint128) => ebool test 3 (219, 219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(219n);
    input.add128(219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint128) => ebool test 4 (219, 215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(219n);
    input.add128(215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint128) => ebool test 1 (60, 340282366920938463463372861542493365571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(60n);
    input.add128(340282366920938463463372861542493365571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint128) => ebool test 2 (56, 60)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(56n);
    input.add128(60n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint128) => ebool test 3 (60, 60)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(60n);
    input.add128(60n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint128) => ebool test 4 (60, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(60n);
    input.add128(56n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint128) => euint128 test 1 (124, 340282366920938463463372659629998993737)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(124n);
    input.add128(340282366920938463463372659629998993737n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(124n);
  });

  it('test operator "min" overload (euint8, euint128) => euint128 test 2 (120, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(120n);
    input.add128(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(120n);
  });

  it('test operator "min" overload (euint8, euint128) => euint128 test 3 (124, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(124n);
    input.add128(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(124n);
  });

  it('test operator "min" overload (euint8, euint128) => euint128 test 4 (124, 120)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(124n);
    input.add128(120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(120n);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 1 (234, 340282366920938463463371843680898827895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(234n);
    input.add128(340282366920938463463371843680898827895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(340282366920938463463371843680898827895n);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 2 (230, 234)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(230n);
    input.add128(234n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(234n);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 3 (234, 234)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(234n);
    input.add128(234n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(234n);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 4 (234, 230)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(234n);
    input.add128(230n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(234n);
  });

  it('test operator "and" overload (euint8, euint256) => euint256 test 1 (239, 115792089237316195423570985008687907853269984665640564039457583404717217059291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(239n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583404717217059291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(203n);
  });

  it('test operator "and" overload (euint8, euint256) => euint256 test 2 (235, 239)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(235n);
    input.add256(239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(235n);
  });

  it('test operator "and" overload (euint8, euint256) => euint256 test 3 (239, 239)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(239n);
    input.add256(239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(239n);
  });

  it('test operator "and" overload (euint8, euint256) => euint256 test 4 (239, 235)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(239n);
    input.add256(235n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(235n);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 1 (91, 115792089237316195423570985008687907853269984665640564039457576553263577864747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(91n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576553263577864747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576553263577864827n);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 2 (87, 91)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(87n);
    input.add256(91n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(95n);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 3 (91, 91)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(91n);
    input.add256(91n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(91n);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 4 (91, 87)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(91n);
    input.add256(87n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(95n);
  });

  it('test operator "xor" overload (euint8, euint256) => euint256 test 1 (161, 115792089237316195423570985008687907853269984665640564039457581736093342860495)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(161n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581736093342860495n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581736093342860398n);
  });

  it('test operator "xor" overload (euint8, euint256) => euint256 test 2 (157, 161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(157n);
    input.add256(161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(60n);
  });

  it('test operator "xor" overload (euint8, euint256) => euint256 test 3 (161, 161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(161n);
    input.add256(161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint256) => euint256 test 4 (161, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(161n);
    input.add256(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(60n);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 1 (27, 115792089237316195423570985008687907853269984665640564039457581607429569215397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(27n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581607429569215397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 2 (23, 27)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(23n);
    input.add256(27n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 3 (27, 27)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(27n);
    input.add256(27n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 4 (27, 23)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(27n);
    input.add256(23n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint256) => ebool test 1 (195, 115792089237316195423570985008687907853269984665640564039457583931279723295997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(195n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583931279723295997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint256) => ebool test 2 (191, 195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(191n);
    input.add256(195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint256) => ebool test 3 (195, 195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(195n);
    input.add256(195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint256) => ebool test 4 (195, 191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(195n);
    input.add256(191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 1 (147, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(147n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(150n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 2 (57, 61)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(57n);
    input.add8(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(118n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 3 (61, 61)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(61n);
    input.add8(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(122n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 4 (61, 57)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(61n);
    input.add8(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(118n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (205, 205)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(205n);
    input.add8(205n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (205, 201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(205n);
    input.add8(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (107, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(107n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(214n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 2 (14, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(14n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(210n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 3 (15, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(15n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(225n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 4 (15, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(15n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(210n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 1 (6219, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(6219n);
    input.add8(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(3n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 2 (175, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(175n);
    input.add8(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(163n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 3 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(179n);
    input.add8(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(179n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 4 (179, 175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(179n);
    input.add8(175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(163n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 1 (35728, 234)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(35728n);
    input.add8(234n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(35834n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 2 (230, 234)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(230n);
    input.add8(234n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(238n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 3 (234, 234)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(234n);
    input.add8(234n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(234n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 4 (234, 230)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(234n);
    input.add8(230n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(238n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (6305, 135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(6305n);
    input.add8(135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(6182n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (131, 135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(131n);
    input.add8(135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 3 (135, 135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(135n);
    input.add8(135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 4 (135, 131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(135n);
    input.add8(131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 1 (27664, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(27664n);
    input.add8(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 2 (170, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(170n);
    input.add8(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 3 (174, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(174n);
    input.add8(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 4 (174, 170)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(174n);
    input.add8(170n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (31229, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(31229n);
    input.add8(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (55, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(55n);
    input.add8(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 3 (59, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(59n);
    input.add8(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 4 (59, 55)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(59n);
    input.add8(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (18326, 60)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(18326n);
    input.add8(60n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (56, 60)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(56n);
    input.add8(60n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (60, 60)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(60n);
    input.add8(60n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 4 (60, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(60n);
    input.add8(56n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 1 (28534, 219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(28534n);
    input.add8(219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 2 (215, 219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(215n);
    input.add8(219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 3 (219, 219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(219n);
    input.add8(219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 4 (219, 215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(219n);
    input.add8(215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 1 (47768, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(47768n);
    input.add8(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 2 (120, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(120n);
    input.add8(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 3 (124, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(124n);
    input.add8(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 4 (124, 120)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(124n);
    input.add8(120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 1 (43001, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(43001n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 2 (88, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(88n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 3 (92, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(92n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 4 (92, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(92n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 1 (17490, 53)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(17490n);
    input.add8(53n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(53n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 2 (49, 53)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(49n);
    input.add8(53n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(49n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 3 (53, 53)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(53n);
    input.add8(53n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(53n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 4 (53, 49)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(53n);
    input.add8(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(49n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 1 (23196, 138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(23196n);
    input.add8(138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(23196n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 2 (134, 138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(134n);
    input.add8(138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(138n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 3 (138, 138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(138n);
    input.add8(138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(138n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 4 (138, 134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(138n);
    input.add8(134n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(138n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 1 (21898, 16862)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(21898n);
    input.add16(16862n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(38760n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 2 (16860, 16862)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(16860n);
    input.add16(16862n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(33722n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 3 (16862, 16862)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(16862n);
    input.add16(16862n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(33724n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 4 (16862, 16860)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(16862n);
    input.add16(16860n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(33722n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 1 (6587, 6587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6587n);
    input.add16(6587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 2 (6587, 6583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6587n);
    input.add16(6583n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 1 (221, 267)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(221n);
    input.add16(267n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(59007n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 2 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(221n);
    input.add16(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(48841n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 3 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(221n);
    input.add16(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(48841n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 4 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(221n);
    input.add16(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(48841n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 1 (36796, 21465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36796n);
    input.add16(21465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(920n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 2 (21461, 21465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21461n);
    input.add16(21465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(21457n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 3 (21465, 21465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21465n);
    input.add16(21465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(21465n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 4 (21465, 21461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21465n);
    input.add16(21461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(21457n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 1 (13841, 12872)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(13841n);
    input.add16(12872n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(13913n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 2 (12868, 12872)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12868n);
    input.add16(12872n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(12876n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 3 (12872, 12872)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12872n);
    input.add16(12872n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(12872n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 4 (12872, 12868)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12872n);
    input.add16(12868n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(12876n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (5643, 40329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(5643n);
    input.add16(40329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(35714n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (5639, 5643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(5639n);
    input.add16(5643n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 3 (5643, 5643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(5643n);
    input.add16(5643n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 4 (5643, 5639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(5643n);
    input.add16(5639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 1 (22634, 46877)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22634n);
    input.add16(46877n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 2 (22630, 22634)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22630n);
    input.add16(22634n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 3 (22634, 22634)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22634n);
    input.add16(22634n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 4 (22634, 22630)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22634n);
    input.add16(22630n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 1 (50588, 37592)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50588n);
    input.add16(37592n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 2 (37588, 37592)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37588n);
    input.add16(37592n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 3 (37592, 37592)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37592n);
    input.add16(37592n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 4 (37592, 37588)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37592n);
    input.add16(37588n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 1 (64742, 23128)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64742n);
    input.add16(23128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 2 (23124, 23128)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23124n);
    input.add16(23128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 3 (23128, 23128)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23128n);
    input.add16(23128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 4 (23128, 23124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23128n);
    input.add16(23124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 1 (16063, 32687)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16063n);
    input.add16(32687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 2 (16059, 16063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16059n);
    input.add16(16063n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 3 (16063, 16063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16063n);
    input.add16(16063n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 4 (16063, 16059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16063n);
    input.add16(16059n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 1 (39480, 18264)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39480n);
    input.add16(18264n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 2 (18260, 18264)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18260n);
    input.add16(18264n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 3 (18264, 18264)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18264n);
    input.add16(18264n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 4 (18264, 18260)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18264n);
    input.add16(18260n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 1 (40322, 23108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(40322n);
    input.add16(23108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 2 (23104, 23108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23104n);
    input.add16(23108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 3 (23108, 23108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23108n);
    input.add16(23108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 4 (23108, 23104)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23108n);
    input.add16(23104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 1 (4164, 53251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4164n);
    input.add16(53251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(4164n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 2 (4160, 4164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4160n);
    input.add16(4164n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(4160n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 3 (4164, 4164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4164n);
    input.add16(4164n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(4164n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 4 (4164, 4160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4164n);
    input.add16(4160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(4160n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 1 (56758, 51763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(56758n);
    input.add16(51763n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(56758n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 2 (51759, 51763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51759n);
    input.add16(51763n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(51763n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 3 (51763, 51763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51763n);
    input.add16(51763n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(51763n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 4 (51763, 51759)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51763n);
    input.add16(51759n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(51763n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 1 (3, 43140)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3n);
    input.add32(43140n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(43143n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 2 (29941, 29943)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(29941n);
    input.add32(29943n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(59884n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 3 (29943, 29943)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(29943n);
    input.add32(29943n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(59886n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 4 (29943, 29941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(29943n);
    input.add32(29941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(59884n);
  });
});
