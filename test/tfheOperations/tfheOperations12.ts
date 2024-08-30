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

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "rotr" overload (euint4, uint8) => euint4 test 1 (14, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint4_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(13n);
  });

  it('test operator "rotr" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "rotr" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "rotr" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint4_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (201, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(201n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (201, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(201n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint8_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (15, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(15n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (15, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(15n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 1 (52, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(52n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(104n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 1 (52, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(52n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint8_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(104n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 1 (50, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(50n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(35n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 1 (50, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(50n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(35n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (43733, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(43733n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(21930n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 1 (43733, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(43733n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint16_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(21930n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (55114, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(55114n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(27557n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (55114, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(55114n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint16_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(27557n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 1 (12143, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(12143n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(31609n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 1 (12143, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(12143n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint16_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(31609n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 1 (63583, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(63583n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(64559n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(32768n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 1 (63583, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(63583n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint16_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(64559n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(32768n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (1317463952, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1317463952n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(3899554048n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 1 (1317463952, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1317463952n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(3899554048n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (3048369776, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3048369776n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1524184888n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 1 (3048369776, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3048369776n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint32_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1524184888n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 1 (2219902125, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2219902125n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(289673910n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 1 (2219902125, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2219902125n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint32_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(289673910n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 1 (4127207673, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4127207673n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(3484418439n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(67108864n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(134217728n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(2147483648n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 1 (4127207673, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4127207673n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint32_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(3484418439n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(67108864n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(134217728n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(2147483648n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 1 (18445637206389853441, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445637206389853441n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18375904565248868416n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (18445637206389853441, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445637206389853441n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18375904565248868416n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (18438869893773583255, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438869893773583255n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(576214684180424476n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (18438869893773583255, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438869893773583255n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.shr_euint64_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(576214684180424476n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.shr_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.shr_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.shr_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 1 (18442278686771288481, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18442278686771288481n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18437813299833025347n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 1 (18442278686771288481, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18442278686771288481n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotl_euint64_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18437813299833025347n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotl_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 1 (18441051342909802901, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441051342909802901n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(13834346463932195122n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(288230376151711744n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(576460752303423488n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(9223372036854775808n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 1 (18441051342909802901, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441051342909802901n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotr_euint64_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(13834346463932195122n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotr_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(288230376151711744n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotr_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(576460752303423488n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rotr_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(9223372036854775808n);
  });

  it('test operator "neg" overload (euint4) => euint4 test 1 (6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.neg_euint4(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract6.res4());
    expect(res).to.equal(10n);
  });

  it('test operator "not" overload (euint4) => euint4 test 1 (4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.not_euint4(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract6.res4());
    expect(res).to.equal(11n);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (186)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add8(186n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.neg_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract6.res8());
    expect(res).to.equal(70n);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.not_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract6.res8());
    expect(res).to.equal(226n);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (3993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(3993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.neg_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.res16());
    expect(res).to.equal(61543n);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (47885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(47885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.not_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.res16());
    expect(res).to.equal(17650n);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (2747701293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2747701293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.neg_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.res32());
    expect(res).to.equal(1547266003n);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (3731607282)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3731607282n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.not_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.res32());
    expect(res).to.equal(563360013n);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (18446646312005372999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18446646312005372999n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.neg_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(97761704178617n);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (18443812023863287611)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18443812023863287611n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.not_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(2932049846264004n);
  });
});
