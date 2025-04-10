import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { HTTPZTestSuite1 } from '../../types/contracts/tests/HTTPZTestSuite1';
import type { HTTPZTestSuite2 } from '../../types/contracts/tests/HTTPZTestSuite2';
import type { HTTPZTestSuite3 } from '../../types/contracts/tests/HTTPZTestSuite3';
import type { HTTPZTestSuite4 } from '../../types/contracts/tests/HTTPZTestSuite4';
import type { HTTPZTestSuite5 } from '../../types/contracts/tests/HTTPZTestSuite5';
import type { HTTPZTestSuite6 } from '../../types/contracts/tests/HTTPZTestSuite6';
import type { HTTPZTestSuite7 } from '../../types/contracts/tests/HTTPZTestSuite7';
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

async function deployHTTPZTestFixture1(): Promise<HTTPZTestSuite1> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployHTTPZTestFixture2(): Promise<HTTPZTestSuite2> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployHTTPZTestFixture3(): Promise<HTTPZTestSuite3> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployHTTPZTestFixture4(): Promise<HTTPZTestSuite4> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployHTTPZTestFixture5(): Promise<HTTPZTestSuite5> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployHTTPZTestFixture6(): Promise<HTTPZTestSuite6> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployHTTPZTestFixture7(): Promise<HTTPZTestSuite7> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite7');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('HTTPZ operations 12', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract1 = await deployHTTPZTestFixture1();
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployHTTPZTestFixture2();
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployHTTPZTestFixture3();
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployHTTPZTestFixture4();
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployHTTPZTestFixture5();
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployHTTPZTestFixture6();
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const contract7 = await deployHTTPZTestFixture7();
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 1 (202, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(202n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(86n);
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

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (25152, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(25152n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(32768n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(2560n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(4608n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(288n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 1 (25152, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(25152n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(32768n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(2560n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(4608n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(288n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (5164, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(5164n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(5n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(6n);
    input.add8(10n);
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

  it('test operator "shr" overload (euint16, euint8) => euint16 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(10n);
    input.add8(10n);
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

  it('test operator "shr" overload (euint16, euint8) => euint16 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(10n);
    input.add8(6n);
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

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (5164, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(5164n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(5n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 1 (385, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(385n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(33025n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 1 (385, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(385n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(33025n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 1 (58216, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(58216n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(27932n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(224n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(352n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(5632n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 1 (58216, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(58216n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(27932n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(224n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(352n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(5632n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (2251087776, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2251087776n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(376426496n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3n);
    input.add8(7n);
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

  it('test operator "shl" overload (euint32, euint8) => euint32 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(896n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(56n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 1 (2251087776, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2251087776n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(376426496n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(384n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(896n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(56n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (1387288818, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1387288818n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(21676387n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2n);
    input.add8(6n);
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

  it('test operator "shr" overload (euint32, euint8) => euint32 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);
    input.add8(6n);
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

  it('test operator "shr" overload (euint32, euint8) => euint32 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(1n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 1 (1387288818, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1387288818n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(21676387n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(1n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 1 (5127928, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(5127928n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(164093696n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(32n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(160n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(10n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 1 (5127928, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(5127928n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(164093696n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(32n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(160n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(10n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 1 (3059269461, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3059269461n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(2876027262n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(100663296n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(234881024n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(3758096384n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 1 (3059269461, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3059269461n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(2876027262n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(100663296n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(234881024n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(3758096384n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 1 (18440487513552738617, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440487513552738617n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(17645904373637487744n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(3n);
    input.add8(7n);
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

  it('test operator "shl" overload (euint64, euint8) => euint64 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(896n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(56n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (18440487513552738617, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440487513552738617n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(17645904373637487744n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(384n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(896n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(56n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (18439831352521485649, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439831352521485649n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(144061182441574106n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(3n);
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

  it('test operator "shr" overload (euint64, euint8) => euint64 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);
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

  it('test operator "shr" overload (euint64, euint8) => euint64 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);
    input.add8(3n);
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

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (18439831352521485649, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439831352521485649n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(144061182441574106n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 1 (18439174508535240987, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439174508535240987n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(17477839731397791231n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(3n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(384n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(896n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(56n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 1 (18439174508535240987, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439174508535240987n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(17477839731397791231n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(384n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(896n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(56n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 1 (18444238912781749263, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444238912781749263n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(9223293750575781984n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(576460752303423488n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(2882303761517117440n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 1 (18444238912781749263, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444238912781749263n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(9223293750575781984n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(576460752303423488n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(2882303761517117440n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 1 (340282366920938463463369544615587422507, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369544615587422507n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463458190283662640327680n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(6n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(6144n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(10240n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(10n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(640n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463369544615587422507, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369544615587422507n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463458190283662640327680n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(6144n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(10240n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(640n);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 1 (340282366920938463463371927043448557835, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371927043448557835n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(1329227995784915872903796590013470929n);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(4n);
    input.add8(8n);
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

  it('test operator "shr" overload (euint128, euint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(8n);
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

  it('test operator "shr" overload (euint128, euint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(4n);
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

  it('test operator "shr" overload (euint128, uint8) => euint128 test 1 (340282366920938463463371927043448557835, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371927043448557835n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(1329227995784915872903796590013470929n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 1 (340282366920938463463367847308854996581, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367847308854996581n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463449529875705504149503n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(14336n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(22528n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(1408n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463367847308854996581, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367847308854996581n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463449529875705504149503n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(14336n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(22528n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(1408n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 1 (340282366920938463463372602031959617341, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372602031959617341n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(329648542954659136480144119615153445564n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(10633823966279326983230456482242756608n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(31901471898837980949691369446728269824n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(170141183460469231731687303715884105729n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 1 (340282366920938463463372602031959617341, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372602031959617341n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(329648542954659136480144119615153445564n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(10633823966279326983230456482242756608n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(31901471898837980949691369446728269824n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(170141183460469231731687303715884105729n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581192896563946673, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581192896563946673n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457403846852925271104n);
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

  it('test operator "shl" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581192896563946673, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581192896563946673n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457403846852925271104n);
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

  it('test operator "shr" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581496955386576627, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581496955386576627n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(3618502788666131106986593281521497120414687020801267626233049421779855830519n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1n);
    input.add8(5n);
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

  it('test operator "shr" overload (euint256, euint8) => euint256 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);
    input.add8(5n);
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

  it('test operator "shr" overload (euint256, euint8) => euint256 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(2n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581496955386576627, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581496955386576627n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(3618502788666131106986593281521497120414687020801267626233049421779855830519n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(2n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575936884980106787, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575936884980106787n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039453451641500568668159n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(2560n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(4608n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(288n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575936884980106787, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575936884980106787n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039453451641500568668159n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(2560n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(4608n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(288n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580170131885020177, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580170131885020177n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(58913748527970447085625471864771874991751623057420638539606837174027197187594n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(395773742510358089826658640166413747545356392900138646619239789089546829824n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(621930166801991284013320720261507317571274331700217873258805382855002161152n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(9950882668831860544213131524184117081140389307203485972140886125680034578432n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580170131885020177, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580170131885020177n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(58913748527970447085625471864771874991751623057420638539606837174027197187594n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(395773742510358089826658640166413747545356392900138646619239789089546829824n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(621930166801991284013320720261507317571274331700217873258805382855002161152n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(9950882668831860544213131524184117081140389307203485972140886125680034578432n);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(148n);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(24n);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (35292)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(35292n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(30244n);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (40866)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(40866n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(24669n);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (1294865746)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1294865746n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(3000101550n);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (1498283587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1498283587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(2796683708n);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (18445783143631910457)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445783143631910457n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(960930077641159n);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (18439015784596221655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439015784596221655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(7728289113329960n);
  });

  it('test operator "neg" overload (euint128) => euint128 test 1 (340282366920938463463366218444435554605)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366218444435554605n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint128(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(8388987332656851n);
  });

  it('test operator "not" overload (euint128) => euint128 test 1 (340282366920938463463369541007870072907)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369541007870072907n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint128(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(5066423898138548n);
  });

  it('test operator "neg" overload (euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575831742679297561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575831742679297561n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint256(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(8176170450342375n);
  });
});
