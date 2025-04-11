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

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 1 (207, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(207n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(243n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(129n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(130n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(40n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (22188, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(22188n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(43776n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(2n);
    input.add8(6n);
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

  it('test operator "shl" overload (euint16, euint8) => euint16 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(384n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(24n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 1 (22188, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(22188n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(43776n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(384n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(24n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (33232, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(33232n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(64n);
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

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (33232, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(33232n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(64n);
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

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 1 (578, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(578n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(33796n);
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

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 1 (578, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(578n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(33796n);
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

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 1 (47934, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(47934n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(53166n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(6n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(384n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(640n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(10n);
    input.add8(6n);
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

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 1 (47934, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(47934n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(53166n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(384n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(640n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(10240n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (2217572133, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2217572133n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(2242831520n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(32n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(160n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(10n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 1 (2217572133, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2217572133n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(2242831520n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(32n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(160n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(10n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (3136631566, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3136631566n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(24504934n);
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

  it('test operator "shr" overload (euint32, uint8) => euint32 test 1 (3136631566, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3136631566n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(24504934n);
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

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 1 (3400120761, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3400120761n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(2859363954n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);
    input.add8(6n);
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

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(24n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 1 (3400120761, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3400120761n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(2859363954n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(384n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(24n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 1 (3180887700, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3180887700n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(695939325n);
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

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 1 (3180887700, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3180887700n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(695939325n);
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

  it('test operator "shl" overload (euint64, euint8) => euint64 test 1 (18439750008030732885, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439750008030732885n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(14865782446154361344n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(2560n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(4608n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(288n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (18439750008030732885, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439750008030732885n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(14865782446154361344n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(2560n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(4608n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(288n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (18438453478748547175, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438453478748547175n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(288100835605446049n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2n);
    input.add8(6n);
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

  it('test operator "shr" overload (euint64, euint8) => euint64 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);
    input.add8(6n);
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

  it('test operator "shr" overload (euint64, euint8) => euint64 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(1n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (18438453478748547175, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438453478748547175n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(288100835605446049n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(1n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 1 (18446627849061713287, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18446627849061713287n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(18443024884978725119n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(32n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(160n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(10n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 1 (18446627849061713287, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18446627849061713287n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(18443024884978725119n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(32n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(160n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(10n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 1 (18441280551657855437, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441280551657855437n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(4161323387954961499n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(63050394783186944n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(99079191802150912n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(1585267068834414592n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 1 (18441280551657855437, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441280551657855437n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(4161323387954961499n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(63050394783186944n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(99079191802150912n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(1585267068834414592n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 1 (340282366920938463463368350684132797115, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368350684132797115n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463456967697853103926272n);
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

  it('test operator "shl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463368350684132797115, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368350684132797115n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463456967697853103926272n);
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

  it('test operator "shr" overload (euint128, euint8) => euint128 test 1 (340282366920938463463368058593023349135, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368058593023349135n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(1329227995784915872903781478878997457n);
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

  it('test operator "shr" overload (euint128, uint8) => euint128 test 1 (340282366920938463463368058593023349135, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368058593023349135n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(1329227995784915872903781478878997457n);
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

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 1 (340282366920938463463372855129143239247, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372855129143239247n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463462926017959775326207n);
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

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463372855129143239247, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372855129143239247n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(340282366920938463462926017959775326207n);
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

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 1 (340282366920938463463370215088382094223, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463370215088382094223n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(191408831393027885698148199522778266923n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(5316911983139663491615228241121378304n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(8n);
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

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(170141183460469231731687303715884105728n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 1 (340282366920938463463370215088382094223, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463370215088382094223n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(191408831393027885698148199522778266923n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(5316911983139663491615228241121378304n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(10633823966279326983230456482242756608n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(170141183460469231731687303715884105728n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575533077292144017, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575533077292144017n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457312813166329770528n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(32n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(160n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(10n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575533077292144017, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575533077292144017n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457312813166329770528n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(32n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(160n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(10n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583424205570957441, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583424205570957441n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(904625697166532776746648320380374280103671755200316906558262370501606023105n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(3n);
    input.add8(7n);
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

  it('test operator "shr" overload (euint256, euint8) => euint256 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7n);
    input.add8(7n);
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

  it('test operator "shr" overload (euint256, euint8) => euint256 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7n);
    input.add8(3n);
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

  it('test operator "shr" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583424205570957441, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583424205570957441n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(904625697166532776746648320380374280103671755200316906558262370501606023105n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576986201167983539, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576986201167983539n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039456685228782037621247n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(3n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(384n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(896n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(56n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576986201167983539, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576986201167983539n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039456685228782037621247n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(384n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(896n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(56n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582037796693705747, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582037796693705747n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(72370055773322622139731865630429942408293740416025352524660989943379567402016n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1n);
    input.add8(5n);
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

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(18092513943330655534932966407607485602073435104006338131165247501236426506240n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(57896044618658097711785492504343953926634992332820282019728792003956564819970n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582037796693705747, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582037796693705747n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(72370055773322622139731865630429942408293740416025352524660989943379567402016n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(3618502788666131106986593281521497120414687020801267626233049500247285301248n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(18092513943330655534932966407607485602073435104006338131165247501236426506240n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(57896044618658097711785492504343953926634992332820282019728792003956564819970n);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(43n);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (70)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(70n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract7.resEuint8());
    expect(res).to.equal(185n);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (43386)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(43386n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(22150n);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (24528)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(24528n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract7.resEuint16());
    expect(res).to.equal(41007n);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (1581481030)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1581481030n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(2713486266n);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (3984966127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3984966127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract7.resEuint32());
    expect(res).to.equal(310001168n);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (18439558102475769465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439558102475769465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(7185971233782151n);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (18443964035879508429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443964035879508429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract7.resEuint64());
    expect(res).to.equal(2780037830043186n);
  });

  it('test operator "neg" overload (euint128) => euint128 test 1 (340282366920938463463372565413896228509)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372565413896228509n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint128(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(2042017871982947n);
  });

  it('test operator "not" overload (euint128) => euint128 test 1 (340282366920938463463368201446256659293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368201446256659293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint128(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract7.resEuint128());
    expect(res).to.equal(6405985511552162n);
  });

  it('test operator "neg" overload (euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581745298896656089)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581745298896656089n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint256(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract7.resEuint256());
    expect(res).to.equal(2262614232983847n);
  });
});
