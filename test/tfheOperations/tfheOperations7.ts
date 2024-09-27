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

describe('TFHE operations 7', function () {
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

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (12135, 52126)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12135n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_uint16(encryptedAmount.handles[0], 52126n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(64261n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (12131, 12135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12131n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_uint16(encryptedAmount.handles[0], 12135n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24266n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (12135, 12135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12135n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_uint16(encryptedAmount.handles[0], 12135n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24270n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (12135, 12131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12135n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_uint16(encryptedAmount.handles[0], 12131n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24266n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (12140, 26064)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(26064n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_uint16_euint16(12140n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(38204n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 2 (12131, 12135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(12135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_uint16_euint16(12131n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24266n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 3 (12135, 12135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(12135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_uint16_euint16(12135n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24270n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 4 (12135, 12131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(12131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_uint16_euint16(12135n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24266n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (3171, 3171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(3171n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint16_uint16(encryptedAmount.handles[0], 3171n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 2 (3171, 3167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(3171n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint16_uint16(encryptedAmount.handles[0], 3167n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (3171, 3171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(3171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_uint16_euint16(3171n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (3171, 3167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(3167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_uint16_euint16(3171n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (78, 344)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(78n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_uint16(encryptedAmount.handles[0], 344n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(26832n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (155, 155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(155n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_uint16(encryptedAmount.handles[0], 155n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24025n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (155, 155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(155n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_uint16(encryptedAmount.handles[0], 155n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24025n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (155, 155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(155n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_uint16(encryptedAmount.handles[0], 155n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24025n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (256, 173)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(173n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_uint16_euint16(256n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(44288n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 2 (155, 155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_uint16_euint16(155n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24025n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 3 (155, 155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_uint16_euint16(155n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24025n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 4 (155, 155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_uint16_euint16(155n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24025n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (8852, 38586)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(8852n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.div_euint16_uint16(encryptedAmount.handles[0], 38586n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (8848, 8852)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(8848n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.div_euint16_uint16(encryptedAmount.handles[0], 8852n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (8852, 8852)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(8852n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.div_euint16_uint16(encryptedAmount.handles[0], 8852n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (8852, 8848)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(8852n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.div_euint16_uint16(encryptedAmount.handles[0], 8848n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (53463, 2321)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(53463n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rem_euint16_uint16(encryptedAmount.handles[0], 2321n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(80n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 2 (20898, 20902)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(20898n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rem_euint16_uint16(encryptedAmount.handles[0], 20902n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(20898n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 3 (20902, 20902)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(20902n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rem_euint16_uint16(encryptedAmount.handles[0], 20902n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 4 (20902, 20898)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(20902n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rem_euint16_uint16(encryptedAmount.handles[0], 20898n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (63705, 39705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(63705n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_uint16(encryptedAmount.handles[0], 39705n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (3245, 3249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(3245n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_uint16(encryptedAmount.handles[0], 3249n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (3249, 3249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(3249n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_uint16(encryptedAmount.handles[0], 3249n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (3249, 3245)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(3249n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_uint16(encryptedAmount.handles[0], 3245n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (28254, 39705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(39705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint16_euint16(28254n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (3245, 3249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(3249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint16_euint16(3245n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 3 (3249, 3249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(3249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint16_euint16(3249n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 4 (3249, 3245)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(3245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint16_euint16(3249n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (27750, 3479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(27750n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_uint16(encryptedAmount.handles[0], 3479n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (23847, 23851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(23847n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_uint16(encryptedAmount.handles[0], 23851n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 3 (23851, 23851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(23851n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_uint16(encryptedAmount.handles[0], 23851n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 4 (23851, 23847)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(23851n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_uint16(encryptedAmount.handles[0], 23847n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (20385, 3479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(3479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_uint16_euint16(20385n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (23847, 23851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(23851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_uint16_euint16(23847n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 3 (23851, 23851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(23851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_uint16_euint16(23851n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 4 (23851, 23847)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(23847n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_uint16_euint16(23851n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (8825, 23737)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(8825n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_uint16(encryptedAmount.handles[0], 23737n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (8821, 8825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(8821n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_uint16(encryptedAmount.handles[0], 8825n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (8825, 8825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(8825n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_uint16(encryptedAmount.handles[0], 8825n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 4 (8825, 8821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(8825n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_uint16(encryptedAmount.handles[0], 8821n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (43508, 23737)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(23737n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_uint16_euint16(43508n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (8821, 8825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(8825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_uint16_euint16(8821n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (8825, 8825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(8825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_uint16_euint16(8825n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 4 (8825, 8821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(8821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_uint16_euint16(8825n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (33735, 40439)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(33735n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_uint16(encryptedAmount.handles[0], 40439n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (33731, 33735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(33731n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_uint16(encryptedAmount.handles[0], 33735n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (33735, 33735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(33735n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_uint16(encryptedAmount.handles[0], 33735n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 4 (33735, 33731)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(33735n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_uint16(encryptedAmount.handles[0], 33731n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (47103, 40439)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(40439n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_uint16_euint16(47103n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (33731, 33735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(33735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_uint16_euint16(33731n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (33735, 33735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(33735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_uint16_euint16(33735n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 4 (33735, 33731)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(33731n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_uint16_euint16(33735n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (38605, 15668)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(38605n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_uint16(encryptedAmount.handles[0], 15668n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (38601, 38605)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(38601n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_uint16(encryptedAmount.handles[0], 38605n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (38605, 38605)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(38605n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_uint16(encryptedAmount.handles[0], 38605n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 4 (38605, 38601)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(38605n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_uint16(encryptedAmount.handles[0], 38601n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (26564, 15668)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(15668n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_uint16_euint16(26564n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (38601, 38605)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(38605n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_uint16_euint16(38601n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (38605, 38605)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(38605n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_uint16_euint16(38605n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (38605, 38601)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(38601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_uint16_euint16(38605n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (59332, 5218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(59332n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_uint16(encryptedAmount.handles[0], 5218n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (12423, 12427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12423n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_uint16(encryptedAmount.handles[0], 12427n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (12427, 12427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12427n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_uint16(encryptedAmount.handles[0], 12427n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 4 (12427, 12423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12427n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_uint16(encryptedAmount.handles[0], 12423n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (8638, 5218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(5218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_uint16_euint16(8638n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (12423, 12427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(12427n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_uint16_euint16(12423n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (12427, 12427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(12427n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_uint16_euint16(12427n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 4 (12427, 12423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(12423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_uint16_euint16(12427n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (18209, 9598)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(18209n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_uint16(encryptedAmount.handles[0], 9598n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(9598n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (14692, 14696)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(14692n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_uint16(encryptedAmount.handles[0], 14696n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14692n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (14696, 14696)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(14696n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_uint16(encryptedAmount.handles[0], 14696n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14696n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 4 (14696, 14692)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(14696n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_uint16(encryptedAmount.handles[0], 14692n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14692n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (16454, 9598)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(9598n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_uint16_euint16(16454n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(9598n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (14692, 14696)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(14696n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_uint16_euint16(14692n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14692n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (14696, 14696)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(14696n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_uint16_euint16(14696n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14696n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (14696, 14692)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(14692n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_uint16_euint16(14696n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14692n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (35722, 5145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(35722n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_uint16(encryptedAmount.handles[0], 5145n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(35722n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (7684, 7688)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(7684n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_uint16(encryptedAmount.handles[0], 7688n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7688n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (7688, 7688)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(7688n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_uint16(encryptedAmount.handles[0], 7688n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7688n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (7688, 7684)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(7688n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_uint16(encryptedAmount.handles[0], 7684n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7688n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (56359, 5145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(5145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_uint16_euint16(56359n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(56359n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (7684, 7688)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(7688n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_uint16_euint16(7684n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7688n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (7688, 7688)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(7688n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_uint16_euint16(7688n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7688n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (7688, 7684)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(7684n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_uint16_euint16(7688n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7688n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 1 (12, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(12n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 2 (4, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(6n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 4 (6, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(6n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(10n);
  });

  it('test operator "sub" overload (euint32, euint4) => euint32 test 1 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint4) => euint32 test 2 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 1 (5, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(5n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 2 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 4 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 1 (2599330540, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2599330540n);
    input.add4(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 1 (2761097401, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2761097401n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(2761097405n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(9n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(13n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(13n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(13n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(13n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(13n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 1 (2588014920, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2588014920n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(2588014923n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 1 (3027950561, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3027950561n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(10n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(14n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(14n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 1 (2599901708, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2599901708n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 1 (1601227311, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1601227311n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(10n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(14n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(14n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 1 (3548820179, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3548820179n);
    input.add4(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 1 (681177958, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(681177958n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 1 (928445097, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(928445097n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 1 (2403125524, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2403125524n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(7n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(11n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(11n);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(7n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 1 (1272584917, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1272584917n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(1272584917n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(10n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(14n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(14n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (153, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(153n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(155n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (123, 125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(123n);
    input.add8(125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(248n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (125, 125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(125n);
    input.add8(125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(250n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (125, 123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(125n);
    input.add8(123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(248n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (52, 52)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(52n);
    input.add8(52n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (52, 48)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(52n);
    input.add8(48n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (102, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(102n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(204n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 2 (10, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(10n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(120n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(12n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(144n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 4 (12, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(12n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(120n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (2756559333, 33)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2756559333n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(33n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (29, 33)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(29n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (33, 33)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(33n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(33n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (33, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(33n);
    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (1062322837, 212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1062322837n);
    input.add8(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1062322901n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (208, 212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(208n);
    input.add8(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(212n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (212, 212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(212n);
    input.add8(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(212n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (212, 208)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(212n);
    input.add8(208n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(212n);
  });
});
