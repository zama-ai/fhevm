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

describe('TFHE operations 6', function () {
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

  it('test operator "ne" overload (euint64, euint32) => ebool test 1 (18444237487909908235, 180946232)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18444237487909908235n);
    input.add32(180946232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 2 (180946228, 180946232)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(180946228n);
    input.add32(180946232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 3 (180946232, 180946232)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(180946232n);
    input.add32(180946232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 4 (180946232, 180946228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(180946232n);
    input.add32(180946228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 1 (18441232457200920889, 1481511596)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18441232457200920889n);
    input.add32(1481511596n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 2 (1481511592, 1481511596)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1481511592n);
    input.add32(1481511596n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 3 (1481511596, 1481511596)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1481511596n);
    input.add32(1481511596n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 4 (1481511596, 1481511592)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1481511596n);
    input.add32(1481511592n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 1 (18437887851999879021, 2227687978)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18437887851999879021n);
    input.add32(2227687978n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 2 (2227687974, 2227687978)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2227687974n);
    input.add32(2227687978n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 3 (2227687978, 2227687978)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2227687978n);
    input.add32(2227687978n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 4 (2227687978, 2227687974)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2227687978n);
    input.add32(2227687974n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (18438351993231610687, 2019646336)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438351993231610687n);
    input.add32(2019646336n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (2019646332, 2019646336)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2019646332n);
    input.add32(2019646336n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (2019646336, 2019646336)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2019646336n);
    input.add32(2019646336n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (2019646336, 2019646332)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2019646336n);
    input.add32(2019646332n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (18440356757939195061, 3718245576)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18440356757939195061n);
    input.add32(3718245576n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (3718245572, 3718245576)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3718245572n);
    input.add32(3718245576n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (3718245576, 3718245576)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3718245576n);
    input.add32(3718245576n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (3718245576, 3718245572)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3718245576n);
    input.add32(3718245572n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (18438419075864898527, 406672084)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438419075864898527n);
    input.add32(406672084n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(406672084n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (406672080, 406672084)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(406672080n);
    input.add32(406672084n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(406672080n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (406672084, 406672084)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(406672084n);
    input.add32(406672084n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(406672084n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (406672084, 406672080)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(406672084n);
    input.add32(406672080n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(406672080n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (18440844527432445207, 564871090)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18440844527432445207n);
    input.add32(564871090n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18440844527432445207n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (564871086, 564871090)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(564871086n);
    input.add32(564871090n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(564871090n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (564871090, 564871090)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(564871090n);
    input.add32(564871090n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(564871090n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (564871090, 564871086)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(564871090n);
    input.add32(564871086n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(564871090n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (9221329629035408200, 9220732250900834197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9221329629035408200n);
    input.add64(9220732250900834197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18442061879936242397n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (9220732250900834195, 9220732250900834197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9220732250900834195n);
    input.add64(9220732250900834197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18441464501801668392n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (9220732250900834197, 9220732250900834197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9220732250900834197n);
    input.add64(9220732250900834197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18441464501801668394n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (9220732250900834197, 9220732250900834195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9220732250900834197n);
    input.add64(9220732250900834195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18441464501801668392n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (18440682051650300723, 18440682051650300723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18440682051650300723n);
    input.add64(18440682051650300723n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (18440682051650300723, 18440682051650300719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18440682051650300723n);
    input.add64(18440682051650300719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (4294324555, 4294830092)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4294324555n);
    input.add64(4294830092n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18443394323628509060n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (4294324555, 4294324555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4294324555n);
    input.add64(4294324555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18441223383675948025n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (4294324555, 4294324555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4294324555n);
    input.add64(4294324555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18441223383675948025n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (4294324555, 4294324555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4294324555n);
    input.add64(4294324555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18441223383675948025n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (18443089347779807029, 18439708968362228719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443089347779807029n);
    input.add64(18439708968362228719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18438581349918999333n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (18439708968362228715, 18439708968362228719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439708968362228715n);
    input.add64(18439708968362228719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18439708968362228715n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (18439708968362228719, 18439708968362228719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439708968362228719n);
    input.add64(18439708968362228719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18439708968362228719n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (18439708968362228719, 18439708968362228715)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439708968362228719n);
    input.add64(18439708968362228715n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18439708968362228715n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 1 (18441344925300235447, 18445742029678020969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441344925300235447n);
    input.add64(18445742029678020969n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18445883869035732479n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 2 (18441344925300235443, 18441344925300235447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441344925300235443n);
    input.add64(18441344925300235447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18441344925300235447n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 3 (18441344925300235447, 18441344925300235447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441344925300235447n);
    input.add64(18441344925300235447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18441344925300235447n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 4 (18441344925300235447, 18441344925300235443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441344925300235447n);
    input.add64(18441344925300235443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18441344925300235447n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 1 (18446152010884387051, 18441220256721527773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446152010884387051n);
    input.add64(18441220256721527773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(4931977568383798n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 2 (18441220256721527769, 18441220256721527773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441220256721527769n);
    input.add64(18441220256721527773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 3 (18441220256721527773, 18441220256721527773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441220256721527773n);
    input.add64(18441220256721527773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 4 (18441220256721527773, 18441220256721527769)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441220256721527773n);
    input.add64(18441220256721527769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 1 (18441937730309383741, 18438190080728092159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441937730309383741n);
    input.add64(18438190080728092159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 2 (18438190080728092155, 18438190080728092159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438190080728092155n);
    input.add64(18438190080728092159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 3 (18438190080728092159, 18438190080728092159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438190080728092159n);
    input.add64(18438190080728092159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 4 (18438190080728092159, 18438190080728092155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438190080728092159n);
    input.add64(18438190080728092155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (18442258317735800351, 18443603479994658319)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442258317735800351n);
    input.add64(18443603479994658319n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (18442258317735800347, 18442258317735800351)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442258317735800347n);
    input.add64(18442258317735800351n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (18442258317735800351, 18442258317735800351)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442258317735800351n);
    input.add64(18442258317735800351n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (18442258317735800351, 18442258317735800347)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442258317735800351n);
    input.add64(18442258317735800347n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 1 (18442691876989499033, 18439901620554113171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442691876989499033n);
    input.add64(18439901620554113171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 2 (18439901620554113167, 18439901620554113171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439901620554113167n);
    input.add64(18439901620554113171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 3 (18439901620554113171, 18439901620554113171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439901620554113171n);
    input.add64(18439901620554113171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 4 (18439901620554113171, 18439901620554113167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439901620554113171n);
    input.add64(18439901620554113167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (18441303612401362499, 18437763367601732741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441303612401362499n);
    input.add64(18437763367601732741n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (18437763367601732737, 18437763367601732741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437763367601732737n);
    input.add64(18437763367601732741n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (18437763367601732741, 18437763367601732741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437763367601732741n);
    input.add64(18437763367601732741n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (18437763367601732741, 18437763367601732737)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437763367601732741n);
    input.add64(18437763367601732737n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (18438136005104024903, 18440049246404113697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438136005104024903n);
    input.add64(18440049246404113697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (18438136005104024899, 18438136005104024903)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438136005104024899n);
    input.add64(18438136005104024903n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (18438136005104024903, 18438136005104024903)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438136005104024903n);
    input.add64(18438136005104024903n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (18438136005104024903, 18438136005104024899)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438136005104024903n);
    input.add64(18438136005104024899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 1 (18445604221905337531, 18441898882936062577)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445604221905337531n);
    input.add64(18441898882936062577n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 2 (18441898882936062573, 18441898882936062577)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441898882936062573n);
    input.add64(18441898882936062577n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 3 (18441898882936062577, 18441898882936062577)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441898882936062577n);
    input.add64(18441898882936062577n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 4 (18441898882936062577, 18441898882936062573)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441898882936062577n);
    input.add64(18441898882936062573n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 1 (18444304399887891779, 18443514127334359483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444304399887891779n);
    input.add64(18443514127334359483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18443514127334359483n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 2 (18443514127334359479, 18443514127334359483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443514127334359479n);
    input.add64(18443514127334359483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18443514127334359479n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 3 (18443514127334359483, 18443514127334359483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443514127334359483n);
    input.add64(18443514127334359483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18443514127334359483n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 4 (18443514127334359483, 18443514127334359479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443514127334359483n);
    input.add64(18443514127334359479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18443514127334359479n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (18445864526277383617, 18443501951499654855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445864526277383617n);
    input.add64(18443501951499654855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18445864526277383617n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (18443501951499654851, 18443501951499654855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443501951499654851n);
    input.add64(18443501951499654855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18443501951499654855n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (18443501951499654855, 18443501951499654855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443501951499654855n);
    input.add64(18443501951499654855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18443501951499654855n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (18443501951499654855, 18443501951499654851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443501951499654855n);
    input.add64(18443501951499654851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18443501951499654855n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 1 (2, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(2n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 2 (9223045752850366274, 9223045752850366276)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9223045752850366274n);
    input.add128(9223045752850366276n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446091505700732550n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 3 (9223045752850366276, 9223045752850366276)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9223045752850366276n);
    input.add128(9223045752850366276n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446091505700732552n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 4 (9223045752850366276, 9223045752850366274)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9223045752850366276n);
    input.add128(9223045752850366274n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446091505700732550n);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 1 (18438761960820256963, 18438761960820256963)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438761960820256963n);
    input.add128(18438761960820256963n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 2 (18438761960820256963, 18438761960820256959)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438761960820256963n);
    input.add128(18438761960820256959n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 1 (2, 4611686018427387905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(2n);
    input.add128(4611686018427387905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 2 (4293279767, 4293279767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4293279767n);
    input.add128(4293279767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18432251157731574289n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 3 (4293279767, 4293279767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4293279767n);
    input.add128(4293279767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18432251157731574289n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 4 (4293279767, 4293279767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4293279767n);
    input.add128(4293279767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18432251157731574289n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 1 (18439032601838193243, 340282366920938463463370621183379702223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439032601838193243n);
    input.add128(340282366920938463463370621183379702223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18437897424765780043n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 2 (18439032601838193239, 18439032601838193243)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439032601838193239n);
    input.add128(18439032601838193243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439032601838193235n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 3 (18439032601838193243, 18439032601838193243)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439032601838193243n);
    input.add128(18439032601838193243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439032601838193243n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 4 (18439032601838193243, 18439032601838193239)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439032601838193243n);
    input.add128(18439032601838193239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439032601838193235n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 1 (18438684999662964117, 340282366920938463463367449195057351543)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438684999662964117n);
    input.add128(340282366920938463463367449195057351543n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463367816709370908663n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 2 (18438684999662964113, 18438684999662964117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438684999662964113n);
    input.add128(18438684999662964117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438684999662964117n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 3 (18438684999662964117, 18438684999662964117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438684999662964117n);
    input.add128(18438684999662964117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438684999662964117n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 4 (18438684999662964117, 18438684999662964113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438684999662964117n);
    input.add128(18438684999662964113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438684999662964117n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 1 (18443367813223576899, 340282366920938463463373547012157288993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443367813223576899n);
    input.add128(340282366920938463463373547012157288993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463444930179766139963234n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 2 (18443367813223576895, 18443367813223576899)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443367813223576895n);
    input.add128(18443367813223576899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 3 (18443367813223576899, 18443367813223576899)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443367813223576899n);
    input.add128(18443367813223576899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 4 (18443367813223576899, 18443367813223576895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443367813223576899n);
    input.add128(18443367813223576895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(124n);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 1 (18438454206201490007, 340282366920938463463370600097949450139)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438454206201490007n);
    input.add128(340282366920938463463370600097949450139n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 2 (18438454206201490003, 18438454206201490007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438454206201490003n);
    input.add128(18438454206201490007n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 3 (18438454206201490007, 18438454206201490007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438454206201490007n);
    input.add128(18438454206201490007n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 4 (18438454206201490007, 18438454206201490003)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438454206201490007n);
    input.add128(18438454206201490003n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 1 (18439288059512052889, 340282366920938463463370885103871221261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439288059512052889n);
    input.add128(340282366920938463463370885103871221261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 2 (18439288059512052885, 18439288059512052889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439288059512052885n);
    input.add128(18439288059512052889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 3 (18439288059512052889, 18439288059512052889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439288059512052889n);
    input.add128(18439288059512052889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 4 (18439288059512052889, 18439288059512052885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439288059512052889n);
    input.add128(18439288059512052885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 1 (18443331628957680105, 340282366920938463463369929468204550277)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443331628957680105n);
    input.add128(340282366920938463463369929468204550277n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 2 (18443331628957680101, 18443331628957680105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443331628957680101n);
    input.add128(18443331628957680105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 3 (18443331628957680105, 18443331628957680105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443331628957680105n);
    input.add128(18443331628957680105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 4 (18443331628957680105, 18443331628957680101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443331628957680105n);
    input.add128(18443331628957680101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 1 (18440611168503379155, 340282366920938463463367742379216355283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440611168503379155n);
    input.add128(340282366920938463463367742379216355283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 2 (18440611168503379151, 18440611168503379155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440611168503379151n);
    input.add128(18440611168503379155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 3 (18440611168503379155, 18440611168503379155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440611168503379155n);
    input.add128(18440611168503379155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 4 (18440611168503379155, 18440611168503379151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440611168503379155n);
    input.add128(18440611168503379151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 1 (18438106060849333659, 340282366920938463463366750659853518527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438106060849333659n);
    input.add128(340282366920938463463366750659853518527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 2 (18438106060849333655, 18438106060849333659)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438106060849333655n);
    input.add128(18438106060849333659n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 3 (18438106060849333659, 18438106060849333659)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438106060849333659n);
    input.add128(18438106060849333659n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 4 (18438106060849333659, 18438106060849333655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438106060849333659n);
    input.add128(18438106060849333655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 1 (18441810833913661773, 340282366920938463463370982108294588983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441810833913661773n);
    input.add128(340282366920938463463370982108294588983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 2 (18441810833913661769, 18441810833913661773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441810833913661769n);
    input.add128(18441810833913661773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 3 (18441810833913661773, 18441810833913661773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441810833913661773n);
    input.add128(18441810833913661773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 4 (18441810833913661773, 18441810833913661769)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441810833913661773n);
    input.add128(18441810833913661769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 1 (18445457355849218963, 340282366920938463463373888643133690331)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445457355849218963n);
    input.add128(340282366920938463463373888643133690331n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18445457355849218963n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 2 (18445457355849218959, 18445457355849218963)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445457355849218959n);
    input.add128(18445457355849218963n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18445457355849218959n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 3 (18445457355849218963, 18445457355849218963)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445457355849218963n);
    input.add128(18445457355849218963n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18445457355849218963n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 4 (18445457355849218963, 18445457355849218959)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445457355849218963n);
    input.add128(18445457355849218959n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18445457355849218959n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 1 (18445272757837564437, 340282366920938463463370409180966629291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445272757837564437n);
    input.add128(340282366920938463463370409180966629291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370409180966629291n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 2 (18445272757837564433, 18445272757837564437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445272757837564433n);
    input.add128(18445272757837564437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18445272757837564437n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 3 (18445272757837564437, 18445272757837564437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445272757837564437n);
    input.add128(18445272757837564437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18445272757837564437n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 4 (18445272757837564437, 18445272757837564433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445272757837564437n);
    input.add128(18445272757837564433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18445272757837564437n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 1 (18438238803275084287, 115792089237316195423570985008687907853269984665640564039457582998772326628485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438238803275084287n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582998772326628485n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18437816039426781317n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 2 (18438238803275084283, 18438238803275084287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438238803275084283n);
    input.add256(18438238803275084287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18438238803275084283n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 3 (18438238803275084287, 18438238803275084287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438238803275084287n);
    input.add256(18438238803275084287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18438238803275084287n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 4 (18438238803275084287, 18438238803275084283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438238803275084287n);
    input.add256(18438238803275084283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18438238803275084283n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 1 (18439555963954625857, 115792089237316195423570985008687907853269984665640564039457579079135852315163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439555963954625857n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579079135852315163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579081550735572827n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 2 (18439555963954625853, 18439555963954625857)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439555963954625853n);
    input.add256(18439555963954625857n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18439555963954625917n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 3 (18439555963954625857, 18439555963954625857)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439555963954625857n);
    input.add256(18439555963954625857n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18439555963954625857n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 4 (18439555963954625857, 18439555963954625853)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439555963954625857n);
    input.add256(18439555963954625853n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18439555963954625917n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 1 (18437894907377184167, 115792089237316195423570985008687907853269984665640564039457583775321568146983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437894907377184167n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583775321568146983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039439146169674874100608n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 2 (18437894907377184163, 18437894907377184167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437894907377184163n);
    input.add256(18437894907377184167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 3 (18437894907377184167, 18437894907377184167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437894907377184167n);
    input.add256(18437894907377184167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 4 (18437894907377184167, 18437894907377184163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437894907377184167n);
    input.add256(18437894907377184163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 1 (18438285407905626077, 115792089237316195423570985008687907853269984665640564039457582312932532272151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438285407905626077n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582312932532272151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 2 (18438285407905626073, 18438285407905626077)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438285407905626073n);
    input.add256(18438285407905626077n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 3 (18438285407905626077, 18438285407905626077)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438285407905626077n);
    input.add256(18438285407905626077n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 4 (18438285407905626077, 18438285407905626073)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438285407905626077n);
    input.add256(18438285407905626073n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 1 (18441250512952089351, 115792089237316195423570985008687907853269984665640564039457575216459694617697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441250512952089351n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575216459694617697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 2 (18441250512952089347, 18441250512952089351)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441250512952089347n);
    input.add256(18441250512952089351n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 3 (18441250512952089351, 18441250512952089351)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441250512952089351n);
    input.add256(18441250512952089351n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 4 (18441250512952089351, 18441250512952089347)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441250512952089351n);
    input.add256(18441250512952089347n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 1 (129, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(8n);
  });

  it('test operator "sub" overload (euint128, euint8) => euint128 test 1 (248, 248)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(248n);
    input.add8(248n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint8) => euint128 test 2 (248, 244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(248n);
    input.add8(244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 1 (65, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(65n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 2 (15, 16)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(15n);
    input.add8(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(240n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 4 (16, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(16n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(240n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 1 (340282366920938463463368056812780716311, 107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368056812780716311n);
    input.add8(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 2 (103, 107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(103n);
    input.add8(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(99n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 3 (107, 107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(107n);
    input.add8(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(107n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 4 (107, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(107n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(99n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 1 (340282366920938463463366412026795708831, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366412026795708831n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463366412026795708831n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(15n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(11n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(15n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 1 (340282366920938463463370979756788862303, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370979756788862303n);
    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370979756788862278n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 2 (21, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(21n);
    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 3 (25, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(25n);
    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 4 (25, 21)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(25n);
    input.add8(21n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 1 (340282366920938463463374223974090635647, 184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463374223974090635647n);
    input.add8(184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 2 (180, 184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(180n);
    input.add8(184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 3 (184, 184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(184n);
    input.add8(184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 4 (184, 180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(184n);
    input.add8(180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 1 (340282366920938463463365958264677496853, 30)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463365958264677496853n);
    input.add8(30n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 2 (26, 30)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(26n);
    input.add8(30n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 3 (30, 30)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(30n);
    input.add8(30n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 4 (30, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(30n);
    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 1 (340282366920938463463367618550243346695, 107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367618550243346695n);
    input.add8(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 2 (103, 107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(103n);
    input.add8(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 3 (107, 107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(107n);
    input.add8(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 4 (107, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(107n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 1 (340282366920938463463367432613039037625, 37)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367432613039037625n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 2 (33, 37)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(33n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 3 (37, 37)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(37n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 4 (37, 33)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(37n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 1 (340282366920938463463372996501175183393, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372996501175183393n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 1 (340282366920938463463371574519418116419, 181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371574519418116419n);
    input.add8(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 2 (177, 181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(177n);
    input.add8(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 3 (181, 181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(181n);
    input.add8(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 4 (181, 177)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(181n);
    input.add8(177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });
});
