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

describe('TFHE operations 5', function () {
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

  it('test operator "eq" overload (euint8, uint8) => ebool test 1 (51, 215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(51n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_uint8(encryptedAmount.handles[0], 215n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 2 (47, 51)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(47n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_uint8(encryptedAmount.handles[0], 51n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 3 (51, 51)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(51n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_uint8(encryptedAmount.handles[0], 51n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 4 (51, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(51n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_uint8(encryptedAmount.handles[0], 47n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 1 (31, 215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_uint8_euint8(31n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 2 (47, 51)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(51n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_uint8_euint8(47n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 3 (51, 51)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(51n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_uint8_euint8(51n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 4 (51, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_uint8_euint8(51n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 1 (236, 213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(236n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_uint8(encryptedAmount.handles[0], 213n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 2 (22, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(22n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_uint8(encryptedAmount.handles[0], 26n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 3 (26, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(26n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_uint8(encryptedAmount.handles[0], 26n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 4 (26, 22)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(26n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_uint8(encryptedAmount.handles[0], 22n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 1 (66, 213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_uint8_euint8(66n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 2 (22, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_uint8_euint8(22n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 3 (26, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_uint8_euint8(26n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 4 (26, 22)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(22n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_uint8_euint8(26n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (89, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(89n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_uint8(encryptedAmount.handles[0], 157n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 2 (85, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(85n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_uint8(encryptedAmount.handles[0], 89n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 3 (89, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(89n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_uint8(encryptedAmount.handles[0], 89n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 4 (89, 85)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(89n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_uint8(encryptedAmount.handles[0], 85n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 1 (164, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_uint8_euint8(164n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 2 (85, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_uint8_euint8(85n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 3 (89, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_uint8_euint8(89n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 4 (89, 85)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(85n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_uint8_euint8(89n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (247, 141)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(247n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_uint8(encryptedAmount.handles[0], 141n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 2 (52, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(52n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_uint8(encryptedAmount.handles[0], 56n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 3 (56, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(56n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_uint8(encryptedAmount.handles[0], 56n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 4 (56, 52)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(56n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_uint8(encryptedAmount.handles[0], 52n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 1 (186, 141)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(141n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_uint8_euint8(186n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 2 (52, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(56n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_uint8_euint8(52n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 3 (56, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(56n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_uint8_euint8(56n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 4 (56, 52)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(52n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_uint8_euint8(56n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (182, 83)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(182n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_uint8(encryptedAmount.handles[0], 83n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (178, 182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(178n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_uint8(encryptedAmount.handles[0], 182n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (182, 182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(182n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_uint8(encryptedAmount.handles[0], 182n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 4 (182, 178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(182n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_uint8(encryptedAmount.handles[0], 178n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 1 (14, 83)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_uint8_euint8(14n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 2 (178, 182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_uint8_euint8(178n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 3 (182, 182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_uint8_euint8(182n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 4 (182, 178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_uint8_euint8(182n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 1 (44, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(44n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_uint8(encryptedAmount.handles[0], 12n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 2 (40, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(40n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_uint8(encryptedAmount.handles[0], 44n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 3 (44, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(44n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_uint8(encryptedAmount.handles[0], 44n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 4 (44, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(44n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_uint8(encryptedAmount.handles[0], 40n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (106, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_uint8_euint8(106n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 2 (40, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_uint8_euint8(40n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 3 (44, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_uint8_euint8(44n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 4 (44, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_uint8_euint8(44n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (254, 134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(254n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_uint8(encryptedAmount.handles[0], 134n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(134n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (69, 73)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(69n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_uint8(encryptedAmount.handles[0], 73n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(69n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (73, 73)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(73n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_uint8(encryptedAmount.handles[0], 73n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(73n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 4 (73, 69)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(73n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_uint8(encryptedAmount.handles[0], 69n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(69n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (48, 134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(134n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_uint8_euint8(48n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(48n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 2 (69, 73)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(73n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_uint8_euint8(69n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(69n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 3 (73, 73)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(73n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_uint8_euint8(73n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(73n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 4 (73, 69)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(69n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_uint8_euint8(73n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(69n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (227, 27)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(227n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_uint8(encryptedAmount.handles[0], 27n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(227n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (25, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(25n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_uint8(encryptedAmount.handles[0], 29n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(29n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (29, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(29n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_uint8(encryptedAmount.handles[0], 29n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(29n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 4 (29, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(29n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_uint8(encryptedAmount.handles[0], 25n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(29n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 1 (174, 27)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(27n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_uint8_euint8(174n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(174n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 2 (25, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_uint8_euint8(25n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(29n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 3 (29, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_uint8_euint8(29n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(29n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 4 (29, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);

    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_uint8_euint8(29n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(29n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 1 (8, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(5n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint16, euint4) => euint16 test 1 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(14n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint4) => euint16 test 2 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(14n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 1 (5, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(5n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 2 (3, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint16, euint4) => euint16 test 4 (4, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 1 (62185, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(62185n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(7n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(3n);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(11n);
  });

  it('test operator "and" overload (euint16, euint4) => euint16 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11n);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(3n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 1 (40123, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(40123n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(40123n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(7n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(15n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(11n);
  });

  it('test operator "or" overload (euint16, euint4) => euint16 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11n);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(15n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 1 (34235, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(34235n);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(34236n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint4) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 1 (57152, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(57152n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 1 (24920, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(24920n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 1 (27268, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27268n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 1 (6870, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6870n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 1 (39104, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39104n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 1 (57762, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(57762n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 1 (10884, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(10884n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(9n);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(5n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(5n);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(9n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(9n);
  });

  it('test operator "min" overload (euint16, euint4) => euint16 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(9n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 1 (58569, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(58569n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(58569n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(6n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(10n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(10n);
  });

  it('test operator "max" overload (euint16, euint4) => euint16 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(10n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 1 (147, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(147n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(150n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 2 (116, 118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(116n);
    input.add8(118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(234n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 3 (118, 118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(118n);
    input.add8(118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(236n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 4 (118, 116)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(118n);
    input.add8(116n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(234n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (143, 143)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(143n);
    input.add8(143n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (143, 139)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(143n);
    input.add8(139n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (127, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(127n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(254n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 2 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 4 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(100n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 1 (19261, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(19261n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(5n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 1 (37227, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(37227n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(37243n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 2 (84, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(84n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(92n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 3 (88, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(88n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(88n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 4 (88, 84)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(88n);
    input.add8(84n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(92n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (43168, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(43168n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(43081n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (229, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(229n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 3 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(233n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 4 (233, 229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(233n);
    input.add8(229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 1 (64589, 145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(64589n);
    input.add8(145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 2 (141, 145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(141n);
    input.add8(145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 3 (145, 145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(145n);
    input.add8(145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 4 (145, 141)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(145n);
    input.add8(141n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (59614, 149)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(59614n);
    input.add8(149n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (145, 149)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(145n);
    input.add8(149n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 3 (149, 149)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(149n);
    input.add8(149n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 4 (149, 145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(149n);
    input.add8(145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (24994, 118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(24994n);
    input.add8(118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (114, 118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(114n);
    input.add8(118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (118, 118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(118n);
    input.add8(118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 4 (118, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(118n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 1 (15548, 171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(15548n);
    input.add8(171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 2 (167, 171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(167n);
    input.add8(171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 3 (171, 171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(171n);
    input.add8(171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 4 (171, 167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(171n);
    input.add8(167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 1 (53893, 183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(53893n);
    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 2 (179, 183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(179n);
    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 3 (183, 183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(183n);
    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 4 (183, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(183n);
    input.add8(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 1 (14349, 199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(14349n);
    input.add8(199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 2 (195, 199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(195n);
    input.add8(199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 3 (199, 199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(199n);
    input.add8(199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 4 (199, 195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(199n);
    input.add8(195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 1 (29386, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(29386n);
    input.add8(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(59n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 2 (55, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(55n);
    input.add8(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(55n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 3 (59, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(59n);
    input.add8(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(59n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 4 (59, 55)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(59n);
    input.add8(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(55n);
  });
});
