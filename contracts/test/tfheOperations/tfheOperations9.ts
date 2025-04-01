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

describe('TFHE operations 9', function () {
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

  it('test operator "or" overload (uint8, euint8) => euint8 test 1 (129, 106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(129n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(235n);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 2 (157, 161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(157n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(189n);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 3 (161, 161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(161n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(161n);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 4 (161, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(161n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(189n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 1 (40, 184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(40n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 184n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(144n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 2 (36, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(36n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 40n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 3 (40, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(40n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 40n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 4 (40, 36)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(40n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 36n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 1 (101, 184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(101n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(221n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 2 (36, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(36n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 3 (40, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(40n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 4 (40, 36)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(36n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(40n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 1 (131, 19)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(131n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 19n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 2 (127, 131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(127n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 131n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 3 (131, 131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(131n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 131n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 4 (131, 127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(131n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 127n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 1 (231, 19)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(19n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(231n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 2 (127, 131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(127n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 3 (131, 131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(131n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 4 (131, 127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(131n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 1 (219, 116)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(219n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 116n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 2 (123, 127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(123n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 127n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 3 (127, 127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(127n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 127n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 4 (127, 123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(127n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 123n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 1 (43, 116)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(116n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(43n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 2 (123, 127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(123n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 3 (127, 127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(127n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 4 (127, 123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(127n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (216, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(216n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 103n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 2 (212, 216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(212n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 216n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 3 (216, 216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(216n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 216n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 4 (216, 212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(216n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 212n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 1 (112, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(112n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 2 (212, 216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(212n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 3 (216, 216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(216n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 4 (216, 212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(216n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (150, 50)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(150n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 50n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 2 (146, 150)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(146n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 150n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 3 (150, 150)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(150n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 150n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 4 (150, 146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(150n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 146n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 1 (141, 50)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(50n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(141n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 2 (146, 150)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(150n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(146n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 3 (150, 150)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(150n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(150n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 4 (150, 146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(150n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (235, 150)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(235n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 150n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (183, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(183n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 187n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (187, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(187n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 187n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 4 (187, 183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(187n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 183n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 1 (89, 150)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(150n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(89n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 2 (183, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(183n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 3 (187, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(187n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 4 (187, 183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(187n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 1 (75, 163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(75n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 163n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 2 (60, 64)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(60n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 64n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 3 (64, 64)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(64n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 64n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 4 (64, 60)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(64n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 60n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (5, 163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(5n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 2 (60, 64)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(64n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(60n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 3 (64, 64)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(64n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(64n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 4 (64, 60)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(60n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(64n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (29, 193)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(29n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 193n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(29n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (25, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(25n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 29n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(25n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (29, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(29n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 29n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(29n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 4 (29, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(29n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 25n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(25n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (27, 193)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(193n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(27n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(27n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 2 (25, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(25n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(25n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 3 (29, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(29n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(29n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 4 (29, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(29n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(25n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (5, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 25n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(25n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 1 (211, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(211n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(211n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(1n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(5n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(5n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (9409, 55880)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(9409n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 55880n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(65289n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (9405, 9409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(9405n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 9409n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(18814n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (9409, 9409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(9409n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 9409n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(18818n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (9409, 9405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(9409n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 9405n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(18814n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (15591, 27941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(27941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(15591n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(43532n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 2 (9405, 9409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(9409n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(9405n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(18814n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 3 (9409, 9409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(9409n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(9409n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(18818n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 4 (9409, 9405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(9405n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(9409n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(18814n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (3155, 3155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(3155n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint16_uint16(encryptedAmount.handles[0], 3155n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 2 (3155, 3151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(3155n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint16_uint16(encryptedAmount.handles[0], 3151n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (3155, 3155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(3155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint16_euint16(3155n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (3155, 3151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(3151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint16_euint16(3155n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (357, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(357n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 179n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(63903n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(179n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 179n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(32041n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(179n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 179n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(32041n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(179n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 179n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(32041n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (94, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(94n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(16826n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 2 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(179n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(32041n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 3 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(179n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(32041n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 4 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(179n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(32041n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (17227, 27037)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(17227n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 27037n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (17223, 17227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(17223n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 17227n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (17227, 17227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(17227n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 17227n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (17227, 17223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(17227n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 17223n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (2527, 47550)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(2527n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 47550n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(2527n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 2 (2523, 2527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(2523n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 2527n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(2523n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 3 (2527, 2527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(2527n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 2527n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 4 (2527, 2523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(2527n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 2523n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 1 (46275, 47937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(46275n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 47937n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(45121n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 2 (46271, 46275)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(46271n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 46275n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(46211n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 3 (46275, 46275)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(46275n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 46275n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(46275n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 4 (46275, 46271)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(46275n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 46271n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(46211n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 1 (45134, 47937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(47937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(45134n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(45120n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 2 (46271, 46275)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(46275n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(46271n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(46211n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 3 (46275, 46275)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(46275n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(46275n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(46275n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 4 (46275, 46271)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(46271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(46275n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(46211n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 1 (48494, 48891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(48494n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 48891n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(49151n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 2 (16969, 16973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16969n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 16973n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(16973n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 3 (16973, 16973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16973n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 16973n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(16973n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 4 (16973, 16969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16973n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 16969n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(16973n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 1 (62329, 48891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(48891n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(62329n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(65531n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 2 (16969, 16973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(16973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(16969n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(16973n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 3 (16973, 16973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(16973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(16973n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(16973n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 4 (16973, 16969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(16969n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(16973n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(16973n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 1 (7797, 826)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(7797n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 826n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(7503n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 2 (7793, 7797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(7793n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 7797n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 3 (7797, 7797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(7797n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 7797n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 4 (7797, 7793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(7797n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 7793n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 1 (15386, 826)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(826n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(15386n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(16160n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 2 (7793, 7797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(7797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(7793n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 3 (7797, 7797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(7797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(7797n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 4 (7797, 7793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(7793n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(7797n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (20280, 63639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(20280n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 63639n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (17248, 17252)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(17248n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 17252n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (17252, 17252)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(17252n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 17252n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (17252, 17248)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(17252n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 17248n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (57792, 63639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(63639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(57792n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (17248, 17252)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(17252n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(17248n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 3 (17252, 17252)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(17252n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(17252n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 4 (17252, 17248)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(17248n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(17252n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (62028, 26983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(62028n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 26983n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (45244, 45248)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(45244n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 45248n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 3 (45248, 45248)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(45248n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 45248n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 4 (45248, 45244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(45248n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 45244n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (12902, 26983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(26983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(12902n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (45244, 45248)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(45248n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(45244n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 3 (45248, 45248)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(45248n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(45248n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 4 (45248, 45244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(45244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(45248n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (20735, 37554)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(20735n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 37554n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (14993, 14997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(14993n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 14997n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (14997, 14997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(14997n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 14997n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 4 (14997, 14993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(14997n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 14993n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (41018, 37554)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(37554n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(41018n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (14993, 14997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(14997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(14993n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (14997, 14997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(14997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(14997n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 4 (14997, 14993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(14993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(14997n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (61346, 39113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(61346n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 39113n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (60441, 60445)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(60441n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 60445n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (60445, 60445)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(60445n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 60445n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 4 (60445, 60441)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(60445n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 60441n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (56639, 39113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(39113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(56639n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (60441, 60445)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(60445n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(60441n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (60445, 60445)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(60445n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(60445n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 4 (60445, 60441)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(60441n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(60445n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (27360, 4118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(27360n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 4118n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (27356, 27360)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(27356n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 27360n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (27360, 27360)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(27360n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 27360n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 4 (27360, 27356)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(27360n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 27356n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (6906, 4118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(4118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(6906n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (27356, 27360)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(27360n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(27356n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (27360, 27360)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(27360n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(27360n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (27360, 27356)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(27356n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(27360n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (23882, 31574)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(23882n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 31574n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (23878, 23882)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(23878n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 23882n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (23882, 23882)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(23882n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 23882n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 4 (23882, 23878)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(23882n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 23878n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (29068, 31574)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(31574n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(29068n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (23878, 23882)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(23882n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(23878n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (23882, 23882)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(23882n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(23882n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 4 (23882, 23878)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(23878n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(23882n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (36216, 39818)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(36216n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 39818n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(36216n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (25136, 25140)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(25136n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 25140n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(25136n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (25140, 25140)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(25140n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 25140n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(25140n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 4 (25140, 25136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(25140n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 25136n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(25136n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (45311, 39818)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(39818n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(45311n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(39818n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (25136, 25140)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(25140n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(25136n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(25136n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (25140, 25140)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(25140n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(25140n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(25140n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (25140, 25136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(25136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(25140n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(25136n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (42716, 64776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(42716n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 64776n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(64776n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (26662, 26666)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(26662n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 26666n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(26666n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (26666, 26666)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(26666n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 26666n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(26666n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (26666, 26662)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(26666n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 26662n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(26666n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (34108, 64776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(64776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(34108n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(64776n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (26662, 26666)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(26666n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(26662n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(26666n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (26666, 26666)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(26666n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(26666n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(26666n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (26666, 26662)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(26662n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(26666n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(26666n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (51600443, 2137576604)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(51600443n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      2137576604n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2189177047n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (103200881, 103200885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(103200881n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      103200885n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(206401766n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (103200885, 103200885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(103200885n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      103200885n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(206401770n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (103200885, 103200881)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(103200885n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      103200881n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(206401766n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (1113522715, 2137576604)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2137576604n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      1113522715n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3251099319n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 2 (103200881, 103200885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(103200885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      103200881n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(206401766n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 3 (103200885, 103200885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(103200885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      103200885n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(206401770n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 4 (103200885, 103200881)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(103200881n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      103200885n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(206401766n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (3448348036, 3448348036)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3448348036n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint32_uint32(
      encryptedAmount.handles[0],
      3448348036n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 2 (3448348036, 3448348032)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3448348036n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint32_uint32(
      encryptedAmount.handles[0],
      3448348032n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(4n);
  });
});
