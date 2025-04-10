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

describe('TFHE operations 12', function () {
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

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 1 (177, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(177n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(141n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(8n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(40n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(130n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (31464, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(31464n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(59392n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 1 (31464, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(31464n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(59392n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (44257, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(44257n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(86n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (44257, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(44257n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(86n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 1 (37762, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(37762n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(1319n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(2560n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(4608n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(288n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 1 (37762, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(37762n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(1319n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(2560n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(4608n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(288n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 1 (61062, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(61062n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(14196n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(10240n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(32770n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 1 (61062, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(61062n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(14196n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(10240n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(32770n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (2910264043, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2910264043n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(1573305024n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(384n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(24n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 1 (2910264043, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2910264043n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(1573305024n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(384n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(24n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (3463956825, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3463956825n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(27062162n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 1 (3463956825, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3463956825n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(27062162n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 1 (1449324859, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1449324859n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(829988267n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(384n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(896n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(56n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 1 (1449324859, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1449324859n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(829988267n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(384n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(896n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(56n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 1 (3110216987, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3110216987n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(3721072936n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(134217728n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(671088640n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 1 (3110216987, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3110216987n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(3721072936n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(134217728n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(671088640n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 1 (18443283062149320265, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443283062149320265n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(18225239333854745152n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(384n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(24n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (18443283062149320265, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443283062149320265n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(18225239333854745152n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(384n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(24n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (18445354198316265791, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445354198316265791n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(9006520604646614n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (18445354198316265791, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445354198316265791n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(9006520604646614n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 1 (18437955581941029181, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18437955581941029181n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(9447328502742579199n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(6144n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(10240n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(10n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(640n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 1 (18437955581941029181, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18437955581941029181n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(9447328502742579199n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(6144n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(10240n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(640n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 1 (18444524543152379505, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444524543152379505n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(16429114100565091500n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(3n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(432345564227567616n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(1008806316530991104n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(16140901064495857664n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 1 (18444524543152379505, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444524543152379505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(16429114100565091500n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(432345564227567616n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(1008806316530991104n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(16140901064495857664n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 1 (340282366920938463463367252062772659433, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367252062772659433n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463139235623910546720n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(32n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(160n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(10n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463367252062772659433, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367252062772659433n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463463139235623910546720n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(32n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(160n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(10n);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 1 (340282366920938463463372655259489290069, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372655259489290069n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(166153499473114484112974929325922504n);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 1 (340282366920938463463372655259489290069, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372655259489290069n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(166153499473114484112974929325922504n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 1 (340282366920938463463369435548960748433, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369435548960748433n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463462050605433057677823n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463369435548960748433, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369435548960748433n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463462050605433057677823n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 1 (340282366920938463463366509695909904637, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366509695909904637n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(168811955464684315858783480839713412896n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(3323069989462289682259517650700861440n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(5981525981032121428067131771261550592n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(95704415696513942849074108340184809472n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 1 (340282366920938463463366509695909904637, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366509695909904637n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(168811955464684315858783480839713412896n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(3323069989462289682259517650700861440n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(5981525981032121428067131771261550592n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(95704415696513942849074108340184809472n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578647606015737085, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578647606015737085n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457240948257839857472n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(384n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(24n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578647606015737085, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578647606015737085n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457240948257839857472n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(384n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(24n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580756359767095869, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580756359767095869n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(452312848583266388373324160190187140051835877600158453279131174829530340218n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580756359767095869, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580756359767095869n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(452312848583266388373324160190187140051835877600158453279131174829530340218n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575177020426745525, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575177020426745525n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039455323299381188670975n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575177020426745525, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575177020426745525n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039455323299381188670975n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580318884071551605, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580318884071551605n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(53372916132825433828052250902442082526116633556818697486937480114237188435250n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(1809251394333065553493296640760748560207343510400633813116524750123642650624n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(3618502788666131106986593281521497120414687020801267626233049500247285301248n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(57896044618658097711785492504343953926634992332820282019728792003956564819968n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580318884071551605, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580318884071551605n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(53372916132825433828052250902442082526116633556818697486937480114237188435250n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(1809251394333065553493296640760748560207343510400633813116524750123642650624n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(3618502788666131106986593281521497120414687020801267626233049500247285301248n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(57896044618658097711785492504343953926634992332820282019728792003956564819968n);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (90)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(90n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(166n);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(90n);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (27862)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(27862n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(37674n);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (24002)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(24002n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(41533n);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (2406385524)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2406385524n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(1888581772n);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (250662384)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(250662384n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(4044304911n);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (18441141236810006711)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441141236810006711n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(5602836899544905n);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (18443352287354699631)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443352287354699631n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(3391786354851984n);
  });

  it('test operator "neg" overload (euint128) => euint128 test 1 (340282366920938463463366794193484648073)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366794193484648073n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint128(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(7813238283563383n);
  });

  it('test operator "not" overload (euint128) => euint128 test 1 (340282366920938463463368888989685938131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368888989685938131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint128(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(5718442082273324n);
  });

  it('test operator "neg" overload (euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576716327227954233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576716327227954233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint256(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(7291585901685703n);
  });
});
