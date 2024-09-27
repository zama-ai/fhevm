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

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 1 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_uint8(encryptedAmount.handles[0], 13n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_uint8(encryptedAmount.handles[0], 13n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, uint8) => ebool test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 1 (8, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_uint8_euint4(8n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_uint8_euint4(9n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_uint8_euint4(13n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint4) => ebool test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_uint8_euint4(13n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 1 (13, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_uint8(encryptedAmount.handles[0], 13n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_uint8(encryptedAmount.handles[0], 13n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, uint8) => ebool test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 1 (11, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_uint8_euint4(11n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_uint8_euint4(10n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_uint8_euint4(14n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint4) => ebool test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_uint8_euint4(14n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 1 (8, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 1 (5, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_uint8_euint4(5n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(3n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_uint8_euint4(4n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_uint8_euint4(8n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (uint8, euint4) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_uint8_euint4(8n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 1 (1, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(2n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 1 (10, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_uint8_euint4(10n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_uint8_euint4(7n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_uint8_euint4(11n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (uint8, euint4) => euint4 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_uint8_euint4(11n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 1 (10, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(10n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint8, euint4) => euint8 test 1 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint4) => euint8 test 2 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 1 (5, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 2 (3, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(3n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(3n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint8, euint4) => euint8 test 4 (5, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 1 (185, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(185n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint8, euint4) => euint8 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 1 (37, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(37n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(39n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 1 (253, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(253n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(254n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 1 (37, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(37n);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 1 (145, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(145n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(10n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(14n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint4) => ebool test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(14n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint4) => ebool test 1 (85, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(85n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint4) => ebool test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(8n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint4) => ebool test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(12n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint4) => ebool test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(12n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 1 (248, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(248n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 1 (92, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(92n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(9n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(13n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint4) => ebool test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(13n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 1 (195, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(195n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(10n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(14n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint4) => ebool test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(14n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 1 (55, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(55n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(3n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint8, euint4) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 1 (41, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(41n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(41n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(7n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(11n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (euint8, euint4) => euint8 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(11n);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 1 (56, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(56n);
    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(145n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 2 (52, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(52n);
    input.add8(56n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(108n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 3 (56, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(56n);
    input.add8(56n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(112n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 4 (56, 52)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(56n);
    input.add8(52n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(108n);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 1 (64, 64)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(64n);
    input.add8(64n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 2 (64, 60)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(64n);
    input.add8(60n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 1 (14, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(14n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(210n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 2 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 4 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(196n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 1 (68, 245)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(68n);
    input.add8(245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(68n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 2 (64, 68)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(64n);
    input.add8(68n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(64n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 3 (68, 68)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(68n);
    input.add8(68n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(68n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 4 (68, 64)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(68n);
    input.add8(64n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(64n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 1 (245, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(245n);
    input.add8(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(255n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 2 (43, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(43n);
    input.add8(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(47n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 3 (47, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(47n);
    input.add8(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(47n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 4 (47, 43)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(47n);
    input.add8(43n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(47n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 1 (213, 62)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(213n);
    input.add8(62n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(235n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 2 (58, 62)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(58n);
    input.add8(62n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 3 (62, 62)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(62n);
    input.add8(62n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 4 (62, 58)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(62n);
    input.add8(58n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 1 (51, 70)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(51n);
    input.add8(70n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 2 (47, 51)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(47n);
    input.add8(51n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 3 (51, 51)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(51n);
    input.add8(51n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 4 (51, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(51n);
    input.add8(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 1 (236, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(236n);
    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 2 (22, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(22n);
    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 3 (26, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(26n);
    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 4 (26, 22)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(26n);
    input.add8(22n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 1 (89, 227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(89n);
    input.add8(227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 2 (85, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(85n);
    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 3 (89, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(89n);
    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 4 (89, 85)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(89n);
    input.add8(85n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 1 (247, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(247n);
    input.add8(56n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 2 (52, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(52n);
    input.add8(56n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 3 (56, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(56n);
    input.add8(56n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 4 (56, 52)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(56n);
    input.add8(52n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 1 (182, 238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(182n);
    input.add8(238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 2 (178, 182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(178n);
    input.add8(182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 3 (182, 182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(182n);
    input.add8(182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 4 (182, 178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(182n);
    input.add8(178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 1 (44, 62)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(44n);
    input.add8(62n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 2 (40, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(40n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 3 (44, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(44n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 4 (44, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(44n);
    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 1 (254, 73)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(254n);
    input.add8(73n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(73n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 2 (69, 73)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(69n);
    input.add8(73n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(69n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 3 (73, 73)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(73n);
    input.add8(73n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(73n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 4 (73, 69)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(73n);
    input.add8(69n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(69n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 1 (227, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(227n);
    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(227n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 2 (25, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(25n);
    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(29n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 3 (29, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(29n);
    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(29n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 4 (29, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(29n);
    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract2.res8());
    expect(res).to.equal(29n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 1 (2, 246)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(2n);
    input.add16(246n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(248n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 2 (104, 106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(104n);
    input.add16(106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(210n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 3 (106, 106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(106n);
    input.add16(106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(212n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 4 (106, 104)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(106n);
    input.add16(104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(210n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 1 (17, 17)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(17n);
    input.add16(17n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 2 (17, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(17n);
    input.add16(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 1 (2, 56)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(2n);
    input.add16(56n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(112n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 2 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(10n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(10n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 4 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(10n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(100n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 1 (140, 15292)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(140n);
    input.add16(15292n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(140n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 2 (136, 140)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(136n);
    input.add16(140n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(136n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 3 (140, 140)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(140n);
    input.add16(140n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(140n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 4 (140, 136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(140n);
    input.add16(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(136n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 1 (140, 39215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(140n);
    input.add16(39215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(39343n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 2 (136, 140)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(136n);
    input.add16(140n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(140n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 3 (140, 140)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(140n);
    input.add16(140n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(140n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 4 (140, 136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(140n);
    input.add16(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(140n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 1 (2, 64716)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(2n);
    input.add16(64716n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(64718n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(4n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(8n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(8n);
    input.add16(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 1 (22, 5993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(22n);
    input.add16(5993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 2 (18, 22)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(18n);
    input.add16(22n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 3 (22, 22)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(22n);
    input.add16(22n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 4 (22, 18)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add8(22n);
    input.add16(18n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resb());
    expect(res).to.equal(false);
  });
});
