import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHETestSuite1 } from '../../types/contracts/tests/TFHETestSuite1';
import type { TFHETestSuite2 } from '../../types/contracts/tests/TFHETestSuite2';
import type { TFHETestSuite3 } from '../../types/contracts/tests/TFHETestSuite3';
import type { TFHETestSuite4 } from '../../types/contracts/tests/TFHETestSuite4';
import type { TFHETestSuite5 } from '../../types/contracts/tests/TFHETestSuite5';
import type { TFHETestSuite6 } from '../../types/contracts/tests/TFHETestSuite6';
import type { TFHETestSuite7 } from '../../types/contracts/tests/TFHETestSuite7';
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

async function deployTfheTestFixture6(): Promise<TFHETestSuite6> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture7(): Promise<TFHETestSuite7> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite7');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('TFHE operations 3', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract1 = await deployTfheTestFixture1();
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployTfheTestFixture2();
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployTfheTestFixture3();
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployTfheTestFixture4();
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployTfheTestFixture5();
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployTfheTestFixture6();
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const contract7 = await deployTfheTestFixture7();
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (31778, 31778)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(31778n);
    input.add32(31778n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (31778, 31774)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(31778n);
    input.add32(31774n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (15, 3773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15n);
    input.add32(3773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(56595n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 2 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(221n);
    input.add32(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(48841n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 3 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(221n);
    input.add32(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(48841n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 4 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(221n);
    input.add32(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(48841n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (12664, 4109236459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12664n);
    input.add32(4109236459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(12392n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (12660, 12664)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12660n);
    input.add32(12664n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(12656n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (12664, 12664)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12664n);
    input.add32(12664n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(12664n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (12664, 12660)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12664n);
    input.add32(12660n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(12656n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (47545, 900201727)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(47545n);
    input.add32(900201727n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(900201983n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (47541, 47545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(47541n);
    input.add32(47545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(47549n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 3 (47545, 47545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(47545n);
    input.add32(47545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(47545n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 4 (47545, 47541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(47545n);
    input.add32(47541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(47549n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 1 (63943, 7766924)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(63943n);
    input.add32(7766924n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(7764555n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 2 (63939, 63943)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(63939n);
    input.add32(63943n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 3 (63943, 63943)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(63943n);
    input.add32(63943n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 4 (63943, 63939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(63943n);
    input.add32(63939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (52780, 619670550)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(52780n);
    input.add32(619670550n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (52776, 52780)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(52776n);
    input.add32(52780n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 3 (52780, 52780)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(52780n);
    input.add32(52780n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 4 (52780, 52776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(52780n);
    input.add32(52776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 1 (23202, 2793435036)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23202n);
    input.add32(2793435036n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 2 (23198, 23202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23198n);
    input.add32(23202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 3 (23202, 23202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23202n);
    input.add32(23202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 4 (23202, 23198)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23202n);
    input.add32(23198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (34395, 2933225745)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(34395n);
    input.add32(2933225745n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (34391, 34395)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(34391n);
    input.add32(34395n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (34395, 34395)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(34395n);
    input.add32(34395n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 4 (34395, 34391)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(34395n);
    input.add32(34391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 1 (25653, 3752814797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(25653n);
    input.add32(3752814797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 2 (25649, 25653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(25649n);
    input.add32(25653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 3 (25653, 25653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(25653n);
    input.add32(25653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 4 (25653, 25649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(25653n);
    input.add32(25649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (40006, 3253389966)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(40006n);
    input.add32(3253389966n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (40002, 40006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(40002n);
    input.add32(40006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (40006, 40006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(40006n);
    input.add32(40006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 4 (40006, 40002)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(40006n);
    input.add32(40002n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 1 (54513, 3183400497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(54513n);
    input.add32(3183400497n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 2 (54509, 54513)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(54509n);
    input.add32(54513n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 3 (54513, 54513)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(54513n);
    input.add32(54513n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 4 (54513, 54509)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(54513n);
    input.add32(54509n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 1 (11315, 3401969836)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11315n);
    input.add32(3401969836n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(11315n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 2 (11311, 11315)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11311n);
    input.add32(11315n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(11311n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 3 (11315, 11315)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11315n);
    input.add32(11315n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(11315n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 4 (11315, 11311)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11315n);
    input.add32(11311n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(11311n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (36879, 1164171425)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36879n);
    input.add32(1164171425n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(1164171425n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (36875, 36879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36875n);
    input.add32(36879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(36879n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (36879, 36879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36879n);
    input.add32(36879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(36879n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 4 (36879, 36875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36879n);
    input.add32(36875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(36879n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 1 (2, 65508)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add64(65508n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(65510n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 2 (8609, 8613)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8609n);
    input.add64(8613n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(17222n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 3 (8613, 8613)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8613n);
    input.add64(8613n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(17226n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 4 (8613, 8609)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8613n);
    input.add64(8609n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(17222n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 1 (4243, 4243)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4243n);
    input.add64(4243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 2 (4243, 4239)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4243n);
    input.add64(4239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 1 (2, 32760)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add64(32760n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(65520n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 2 (254, 254)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(254n);
    input.add64(254n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(64516n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 3 (254, 254)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(254n);
    input.add64(254n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(64516n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 4 (254, 254)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(254n);
    input.add64(254n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(64516n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 1 (11701, 18443476785227006423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11701n);
    input.add64(18443476785227006423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(405n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 2 (11697, 11701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11697n);
    input.add64(11701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(11697n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 3 (11701, 11701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11701n);
    input.add64(11701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(11701n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 4 (11701, 11697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11701n);
    input.add64(11697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(11697n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 1 (24480, 18444312620584977271)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(24480n);
    input.add64(18444312620584977271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(18444312620584984567n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 2 (24476, 24480)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(24476n);
    input.add64(24480n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(24508n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 3 (24480, 24480)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(24480n);
    input.add64(24480n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(24480n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 4 (24480, 24476)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(24480n);
    input.add64(24476n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(24508n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 1 (34332, 18441617090617625573)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(34332n);
    input.add64(18441617090617625573n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(18441617090617658873n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 2 (34328, 34332)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(34328n);
    input.add64(34332n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 3 (34332, 34332)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(34332n);
    input.add64(34332n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 4 (34332, 34328)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(34332n);
    input.add64(34328n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 1 (64371, 18446517594874919791)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64371n);
    input.add64(18446517594874919791n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 2 (64367, 64371)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64367n);
    input.add64(64371n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 3 (64371, 64371)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64371n);
    input.add64(64371n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 4 (64371, 64367)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64371n);
    input.add64(64367n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 1 (49515, 18441910583086092305)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49515n);
    input.add64(18441910583086092305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 2 (49511, 49515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49511n);
    input.add64(49515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 3 (49515, 49515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49515n);
    input.add64(49515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 4 (49515, 49511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49515n);
    input.add64(49511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 1 (35274, 18443093442503920945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35274n);
    input.add64(18443093442503920945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 2 (35270, 35274)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35270n);
    input.add64(35274n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 3 (35274, 35274)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35274n);
    input.add64(35274n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 4 (35274, 35270)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35274n);
    input.add64(35270n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 1 (15832, 18437903660814007619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15832n);
    input.add64(18437903660814007619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 2 (15828, 15832)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15828n);
    input.add64(15832n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 3 (15832, 15832)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15832n);
    input.add64(15832n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 4 (15832, 15828)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15832n);
    input.add64(15828n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 1 (62115, 18440782657382405041)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(62115n);
    input.add64(18440782657382405041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 2 (62111, 62115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(62111n);
    input.add64(62115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 3 (62115, 62115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(62115n);
    input.add64(62115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 4 (62115, 62111)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(62115n);
    input.add64(62111n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 1 (64237, 18439509072941368087)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64237n);
    input.add64(18439509072941368087n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 2 (64233, 64237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64233n);
    input.add64(64237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 3 (64237, 64237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64237n);
    input.add64(64237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 4 (64237, 64233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64237n);
    input.add64(64233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 1 (22265, 18444689840619920341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22265n);
    input.add64(18444689840619920341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(22265n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 2 (22261, 22265)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22261n);
    input.add64(22265n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(22261n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 3 (22265, 22265)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22265n);
    input.add64(22265n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(22265n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 4 (22265, 22261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22265n);
    input.add64(22261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(22261n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 1 (37365, 18442742256566102069)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37365n);
    input.add64(18442742256566102069n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(18442742256566102069n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 2 (37361, 37365)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37361n);
    input.add64(37365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(37365n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 3 (37365, 37365)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37365n);
    input.add64(37365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(37365n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 4 (37365, 37361)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37365n);
    input.add64(37361n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(37365n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 1 (2, 32769)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add128(32769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(32771n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 2 (28211, 28213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(28211n);
    input.add128(28213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(56424n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 3 (28213, 28213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(28213n);
    input.add128(28213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(56426n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 4 (28213, 28211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(28213n);
    input.add128(28211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(56424n);
  });

  it('test operator "sub" overload (euint16, euint128) => euint128 test 1 (26994, 26994)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(26994n);
    input.add128(26994n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint128) => euint128 test 2 (26994, 26990)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(26994n);
    input.add128(26990n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 1 (2, 16385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add128(16385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 2 (177, 177)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(177n);
    input.add128(177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(31329n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 3 (177, 177)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(177n);
    input.add128(177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(31329n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 4 (177, 177)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(177n);
    input.add128(177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(31329n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 1 (22669, 340282366920938463463371889060299981967)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22669n);
    input.add128(340282366920938463463371889060299981967n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(16525n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 2 (22665, 22669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22665n);
    input.add128(22669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(22665n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 3 (22669, 22669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22669n);
    input.add128(22669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(22669n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 4 (22669, 22665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22669n);
    input.add128(22665n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(22665n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 1 (16016, 340282366920938463463366325963408619473)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16016n);
    input.add128(340282366920938463463366325963408619473n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(340282366920938463463366325963408621521n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 2 (16012, 16016)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16012n);
    input.add128(16016n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(16028n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 3 (16016, 16016)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16016n);
    input.add128(16016n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(16016n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 4 (16016, 16012)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16016n);
    input.add128(16012n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(16028n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 1 (31475, 340282366920938463463367712287147099321)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(31475n);
    input.add128(340282366920938463463367712287147099321n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(340282366920938463463367712287147130442n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 2 (31471, 31475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(31471n);
    input.add128(31475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 3 (31475, 31475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(31475n);
    input.add128(31475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 4 (31475, 31471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(31475n);
    input.add128(31471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 1 (46681, 340282366920938463463368201473039648441)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(46681n);
    input.add128(340282366920938463463368201473039648441n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 2 (46677, 46681)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(46677n);
    input.add128(46681n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 3 (46681, 46681)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(46681n);
    input.add128(46681n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 4 (46681, 46677)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(46681n);
    input.add128(46677n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 1 (15248, 340282366920938463463372916113924916259)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15248n);
    input.add128(340282366920938463463372916113924916259n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 2 (15244, 15248)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15244n);
    input.add128(15248n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 3 (15248, 15248)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15248n);
    input.add128(15248n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 4 (15248, 15244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15248n);
    input.add128(15244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 1 (57597, 340282366920938463463366179809798315405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(57597n);
    input.add128(340282366920938463463366179809798315405n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 2 (57593, 57597)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(57593n);
    input.add128(57597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 3 (57597, 57597)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(57597n);
    input.add128(57597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 4 (57597, 57593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(57597n);
    input.add128(57593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 1 (37899, 340282366920938463463370005098281423301)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37899n);
    input.add128(340282366920938463463370005098281423301n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 2 (37895, 37899)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37895n);
    input.add128(37899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 3 (37899, 37899)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37899n);
    input.add128(37899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 4 (37899, 37895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37899n);
    input.add128(37895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 1 (13823, 340282366920938463463372986881227080645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(13823n);
    input.add128(340282366920938463463372986881227080645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 2 (13819, 13823)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(13819n);
    input.add128(13823n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 3 (13823, 13823)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(13823n);
    input.add128(13823n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 4 (13823, 13819)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(13823n);
    input.add128(13819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 1 (52749, 340282366920938463463366414388575349703)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(52749n);
    input.add128(340282366920938463463366414388575349703n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 2 (52745, 52749)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(52745n);
    input.add128(52749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 3 (52749, 52749)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(52749n);
    input.add128(52749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 4 (52749, 52745)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(52749n);
    input.add128(52745n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 1 (29011, 340282366920938463463369429524255507491)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(29011n);
    input.add128(340282366920938463463369429524255507491n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(29011n);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 2 (29007, 29011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(29007n);
    input.add128(29011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(29007n);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 3 (29011, 29011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(29011n);
    input.add128(29011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(29011n);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 4 (29011, 29007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(29011n);
    input.add128(29007n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(29007n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 1 (8801, 340282366920938463463367430956536802917)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8801n);
    input.add128(340282366920938463463367430956536802917n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(340282366920938463463367430956536802917n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 2 (8797, 8801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8797n);
    input.add128(8801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(8801n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 3 (8801, 8801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8801n);
    input.add128(8801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(8801n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 4 (8801, 8797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8801n);
    input.add128(8797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(8801n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 1 (17876, 115792089237316195423570985008687907853269984665640564039457579526748766431153)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17876n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579526748766431153n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(17808n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 2 (17872, 17876)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17872n);
    input.add256(17876n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(17872n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 3 (17876, 17876)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17876n);
    input.add256(17876n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(17876n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 4 (17876, 17872)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17876n);
    input.add256(17872n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(17872n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 1 (16968, 115792089237316195423570985008687907853269984665640564039457579206759381929277)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16968n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579206759381929277n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579206759381946237n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 2 (16964, 16968)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16964n);
    input.add256(16968n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(16972n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 3 (16968, 16968)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16968n);
    input.add256(16968n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(16968n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 4 (16968, 16964)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16968n);
    input.add256(16964n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(16972n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 1 (27885, 115792089237316195423570985008687907853269984665640564039457575343002741924923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27885n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575343002741924923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575343002741901526n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 2 (27881, 27885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27881n);
    input.add256(27885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 3 (27885, 27885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27885n);
    input.add256(27885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 4 (27885, 27881)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27885n);
    input.add256(27881n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 1 (50079, 115792089237316195423570985008687907853269984665640564039457583340063328919269)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50079n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583340063328919269n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 2 (50075, 50079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50075n);
    input.add256(50079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 3 (50079, 50079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50079n);
    input.add256(50079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 4 (50079, 50075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50079n);
    input.add256(50075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 1 (9712, 115792089237316195423570985008687907853269984665640564039457583144402544760499)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9712n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583144402544760499n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 2 (9708, 9712)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9708n);
    input.add256(9712n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 3 (9712, 9712)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9712n);
    input.add256(9712n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 4 (9712, 9708)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9712n);
    input.add256(9708n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (171, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(171n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(173n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (77, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(77n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(156n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (79, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(79n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(158n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (79, 77)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(79n);
    input.add8(77n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(156n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (111, 111)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(111n);
    input.add8(111n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (111, 107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(111n);
    input.add8(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (71, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(71n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(142n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(5n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(25n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(5n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (2451169455, 242)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2451169455n);
    input.add8(242n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(162n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (238, 242)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(238n);
    input.add8(242n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(226n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (242, 242)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(242n);
    input.add8(242n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(242n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (242, 238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(242n);
    input.add8(238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(226n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (1012140051, 138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1012140051n);
    input.add8(138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(1012140187n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (134, 138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(134n);
    input.add8(138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(142n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (138, 138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(138n);
    input.add8(138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(138n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (138, 134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(138n);
    input.add8(134n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(142n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (3138828732, 178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3138828732n);
    input.add8(178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(3138828558n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (174, 178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(174n);
    input.add8(178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (178, 178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(178n);
    input.add8(178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (178, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(178n);
    input.add8(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(28n);
  });
});
