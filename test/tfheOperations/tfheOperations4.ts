import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHETestSuite1 } from '../../types/contracts/tests/TFHETestSuite1';
import type { TFHETestSuite2 } from '../../types/contracts/tests/TFHETestSuite2';
import type { TFHETestSuite3 } from '../../types/contracts/tests/TFHETestSuite3';
import type { TFHETestSuite4 } from '../../types/contracts/tests/TFHETestSuite4';
import type { TFHETestSuite5 } from '../../types/contracts/tests/TFHETestSuite5';
import type { TFHETestSuite6 } from '../../types/contracts/tests/TFHETestSuite6';
import { createInstances, decrypt4, decrypt8, decrypt16, decrypt32, decrypt64, decryptBool } from '../instance';
import { getSigners, initSigners } from '../signers';

async function deployTfheTestFixture1(): Promise<TFHETestSuite1> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite1');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture2(): Promise<TFHETestSuite2> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite2');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture3(): Promise<TFHETestSuite3> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite3');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture4(): Promise<TFHETestSuite4> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite4');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture5(): Promise<TFHETestSuite5> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite5');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture6(): Promise<TFHETestSuite6> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite6');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

describe('TFHE operations 4', function () {
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

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 1 (201, 56595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(201n);
    input.add16(56595n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 2 (197, 201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(197n);
    input.add16(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 3 (201, 201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(201n);
    input.add16(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 4 (201, 197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(201n);
    input.add16(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 1 (12, 41571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(12n);
    input.add16(41571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(8n);
    input.add16(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(12n);
    input.add16(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(12n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 1 (67, 34612)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(67n);
    input.add16(34612n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 2 (63, 67)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(63n);
    input.add16(67n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 3 (67, 67)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(67n);
    input.add16(67n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 4 (67, 63)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(67n);
    input.add16(63n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 1 (157, 25391)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(157n);
    input.add16(25391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 2 (153, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(153n);
    input.add16(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 3 (157, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(157n);
    input.add16(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 4 (157, 153)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(157n);
    input.add16(153n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 1 (115, 9566)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(115n);
    input.add16(9566n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 2 (111, 115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(111n);
    input.add16(115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 3 (115, 115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(115n);
    input.add16(115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 4 (115, 111)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(115n);
    input.add16(111n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 1 (59, 17033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(59n);
    input.add16(17033n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(59n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 2 (55, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(55n);
    input.add16(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(55n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 3 (59, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(59n);
    input.add16(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(59n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 4 (59, 55)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(59n);
    input.add16(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(55n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 1 (76, 60931)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(76n);
    input.add16(60931n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(60931n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 2 (72, 76)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(72n);
    input.add16(76n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(76n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 3 (76, 76)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(76n);
    input.add16(76n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(76n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 4 (76, 72)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(76n);
    input.add16(72n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(76n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 1 (2, 131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(2n);
    input.add32(131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(133n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 2 (88, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(88n);
    input.add32(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(180n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 3 (92, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(92n);
    input.add32(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(184n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 4 (92, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(92n);
    input.add32(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(180n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 1 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(10n);
    input.add32(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 2 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(10n);
    input.add32(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 1 (2, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(2n);
    input.add32(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(158n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 2 (10, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(10n);
    input.add32(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(110n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(11n);
    input.add32(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(121n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 4 (11, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(11n);
    input.add32(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(110n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 1 (39, 58691136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(39n);
    input.add32(58691136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 2 (35, 39)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(35n);
    input.add32(39n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(35n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 3 (39, 39)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(39n);
    input.add32(39n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(39n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 4 (39, 35)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(39n);
    input.add32(35n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(35n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 1 (99, 1264632675)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(99n);
    input.add32(1264632675n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(1264632675n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 2 (95, 99)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(95n);
    input.add32(99n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(127n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 3 (99, 99)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(99n);
    input.add32(99n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(99n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 4 (99, 95)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(99n);
    input.add32(95n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(127n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 1 (184, 3590065955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(184n);
    input.add32(3590065955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(3590066075n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 2 (180, 184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(180n);
    input.add32(184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 3 (184, 184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(184n);
    input.add32(184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 4 (184, 180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(184n);
    input.add32(180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 1 (110, 224862953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(110n);
    input.add32(224862953n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 2 (106, 110)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(106n);
    input.add32(110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 3 (110, 110)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(110n);
    input.add32(110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 4 (110, 106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(110n);
    input.add32(106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 1 (189, 472792825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(189n);
    input.add32(472792825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 2 (185, 189)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(185n);
    input.add32(189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 3 (189, 189)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(189n);
    input.add32(189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 4 (189, 185)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(189n);
    input.add32(185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 1 (222, 4025894176)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(222n);
    input.add32(4025894176n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 2 (218, 222)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(218n);
    input.add32(222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 3 (222, 222)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(222n);
    input.add32(222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 4 (222, 218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(222n);
    input.add32(218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 1 (193, 1536979513)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(193n);
    input.add32(1536979513n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 2 (189, 193)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(189n);
    input.add32(193n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 3 (193, 193)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(193n);
    input.add32(193n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 4 (193, 189)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(193n);
    input.add32(189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 1 (217, 193633124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(217n);
    input.add32(193633124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 2 (213, 217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(213n);
    input.add32(217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 3 (217, 217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(217n);
    input.add32(217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 4 (217, 213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(217n);
    input.add32(213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 1 (97, 73387652)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(97n);
    input.add32(73387652n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 2 (93, 97)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(93n);
    input.add32(97n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 3 (97, 97)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(97n);
    input.add32(97n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 4 (97, 93)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(97n);
    input.add32(93n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 1 (47, 1833760772)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(47n);
    input.add32(1833760772n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(47n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 2 (43, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(43n);
    input.add32(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(43n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 3 (47, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(47n);
    input.add32(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(47n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 4 (47, 43)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(47n);
    input.add32(43n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(43n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 1 (103, 990460263)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(103n);
    input.add32(990460263n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(990460263n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 2 (99, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(99n);
    input.add32(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(103n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 3 (103, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(103n);
    input.add32(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(103n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 4 (103, 99)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(103n);
    input.add32(99n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.res32());
    expect(res).to.equal(103n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 1 (2, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(2n);
    input.add64(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 2 (12, 16)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(12n);
    input.add64(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(28n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 3 (16, 16)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(16n);
    input.add64(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(32n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 4 (16, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(16n);
    input.add64(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(28n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 1 (15, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(15n);
    input.add64(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 2 (15, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(15n);
    input.add64(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 1 (2, 65)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(2n);
    input.add64(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 2 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(13n);
    input.add64(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(169n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(13n);
    input.add64(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(169n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 4 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(13n);
    input.add64(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(169n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 1 (62, 18441742416037218435)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(62n);
    input.add64(18441742416037218435n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(2n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 2 (58, 62)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(58n);
    input.add64(62n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(58n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 3 (62, 62)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(62n);
    input.add64(62n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(62n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 4 (62, 58)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(62n);
    input.add64(58n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(58n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 1 (66, 18439693911625769587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(66n);
    input.add64(18439693911625769587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(18439693911625769587n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 2 (62, 66)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(62n);
    input.add64(66n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(126n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 3 (66, 66)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(66n);
    input.add64(66n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(66n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 4 (66, 62)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(66n);
    input.add64(62n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(126n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 1 (212, 18437987864856647089)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(212n);
    input.add64(18437987864856647089n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(18437987864856647013n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 2 (208, 212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(208n);
    input.add64(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 3 (212, 212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(212n);
    input.add64(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 4 (212, 208)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(212n);
    input.add64(208n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 1 (151, 18441484264494857597)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(151n);
    input.add64(18441484264494857597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 2 (147, 151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(147n);
    input.add64(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 3 (151, 151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(151n);
    input.add64(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 4 (151, 147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(151n);
    input.add64(147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 1 (174, 18439836363494569405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(174n);
    input.add64(18439836363494569405n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 2 (170, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(170n);
    input.add64(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 3 (174, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(174n);
    input.add64(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 4 (174, 170)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(174n);
    input.add64(170n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 1 (201, 18443640492228529453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(201n);
    input.add64(18443640492228529453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 2 (197, 201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(197n);
    input.add64(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 3 (201, 201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(201n);
    input.add64(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 4 (201, 197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(201n);
    input.add64(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 1 (237, 18441103586480031265)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(237n);
    input.add64(18441103586480031265n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 2 (233, 237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(233n);
    input.add64(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 3 (237, 237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(237n);
    input.add64(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 4 (237, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(237n);
    input.add64(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 1 (18, 18442364533783886901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(18n);
    input.add64(18442364533783886901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 2 (14, 18)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(14n);
    input.add64(18n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 3 (18, 18)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(18n);
    input.add64(18n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 4 (18, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(18n);
    input.add64(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 1 (1, 18442662886837590601)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(1n);
    input.add64(18442662886837590601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(4n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(8n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(8n);
    input.add64(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 1 (240, 18439345293201198903)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(240n);
    input.add64(18439345293201198903n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(240n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 2 (236, 240)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(236n);
    input.add64(240n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(236n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 3 (240, 240)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(240n);
    input.add64(240n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(240n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 4 (240, 236)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(240n);
    input.add64(236n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(236n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 1 (180, 18445785341899355785)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(180n);
    input.add64(18445785341899355785n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(18445785341899355785n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 2 (176, 180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(176n);
    input.add64(180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(180n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 3 (180, 180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(180n);
    input.add64(180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(180n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 4 (180, 176)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(180n);
    input.add64(176n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.res64());
    expect(res).to.equal(180n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 1 (56, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(56n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(67n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 2 (52, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(52n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_uint8(encryptedAmount.handles[0], 56n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(108n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 3 (56, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(56n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_uint8(encryptedAmount.handles[0], 56n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(112n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 4 (56, 52)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(56n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_uint8(encryptedAmount.handles[0], 52n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(108n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 1 (69, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_uint8_euint8(69n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(80n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 2 (52, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(56n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_uint8_euint8(52n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(108n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 3 (56, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(56n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_uint8_euint8(56n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(112n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 4 (56, 52)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(52n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_uint8_euint8(56n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(108n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 1 (64, 64)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(64n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint8_uint8(encryptedAmount.handles[0], 64n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 2 (64, 60)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(64n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint8_uint8(encryptedAmount.handles[0], 60n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 1 (64, 64)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(64n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_uint8_euint8(64n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 2 (64, 60)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(60n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_uint8_euint8(64n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 1 (8, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_uint8(encryptedAmount.handles[0], 14n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(112n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 2 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_uint8(encryptedAmount.handles[0], 14n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_uint8(encryptedAmount.handles[0], 14n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 4 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_uint8(encryptedAmount.handles[0], 14n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 1 (8, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_uint8_euint8(8n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(112n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 2 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_uint8_euint8(14n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_uint8_euint8(14n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 4 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_uint8_euint8(14n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(196n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 1 (168, 148)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(168n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.div_euint8_uint8(encryptedAmount.handles[0], 148n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 2 (164, 168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(164n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.div_euint8_uint8(encryptedAmount.handles[0], 168n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 3 (168, 168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(168n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.div_euint8_uint8(encryptedAmount.handles[0], 168n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 4 (168, 164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(168n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.div_euint8_uint8(encryptedAmount.handles[0], 164n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 1 (225, 242)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(225n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.rem_euint8_uint8(encryptedAmount.handles[0], 242n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(225n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 2 (183, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(183n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.rem_euint8_uint8(encryptedAmount.handles[0], 187n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(183n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 3 (187, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(187n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.rem_euint8_uint8(encryptedAmount.handles[0], 187n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 4 (187, 183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(187n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.rem_euint8_uint8(encryptedAmount.handles[0], 183n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(4n);
  });
});
