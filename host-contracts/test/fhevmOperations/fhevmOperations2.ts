import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { FHEVMTestSuite1 } from '../../types/contracts/tests/FHEVMTestSuite1';
import type { FHEVMTestSuite2 } from '../../types/contracts/tests/FHEVMTestSuite2';
import type { FHEVMTestSuite3 } from '../../types/contracts/tests/FHEVMTestSuite3';
import type { FHEVMTestSuite4 } from '../../types/contracts/tests/FHEVMTestSuite4';
import type { FHEVMTestSuite5 } from '../../types/contracts/tests/FHEVMTestSuite5';
import type { FHEVMTestSuite6 } from '../../types/contracts/tests/FHEVMTestSuite6';
import type { FHEVMTestSuite7 } from '../../types/contracts/tests/FHEVMTestSuite7';
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

async function deployFHEVMTestFixture1(): Promise<FHEVMTestSuite1> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture2(): Promise<FHEVMTestSuite2> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture3(): Promise<FHEVMTestSuite3> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture4(): Promise<FHEVMTestSuite4> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture5(): Promise<FHEVMTestSuite5> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture6(): Promise<FHEVMTestSuite6> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture7(): Promise<FHEVMTestSuite7> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite7');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('FHEVM operations 2', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract1 = await deployFHEVMTestFixture1();
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployFHEVMTestFixture2();
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployFHEVMTestFixture3();
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployFHEVMTestFixture4();
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployFHEVMTestFixture5();
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployFHEVMTestFixture6();
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const contract7 = await deployFHEVMTestFixture7();
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 1 (217, 18438487598126184331)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(217n);
    input.add64(18438487598126184331n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 2 (213, 217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(213n);
    input.add64(217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 3 (217, 217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(217n);
    input.add64(217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 4 (217, 213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(217n);
    input.add64(213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 1 (178, 18444808322147740501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(178n);
    input.add64(18444808322147740501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 2 (174, 178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(174n);
    input.add64(178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 3 (178, 178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(178n);
    input.add64(178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 4 (178, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(178n);
    input.add64(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 1 (231, 18441957729769617445)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(231n);
    input.add64(18441957729769617445n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(231n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 2 (227, 231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(227n);
    input.add64(231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(227n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 3 (231, 231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(231n);
    input.add64(231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(231n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 4 (231, 227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(231n);
    input.add64(227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(227n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 1 (175, 18444441477495097817)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(175n);
    input.add64(18444441477495097817n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(18444441477495097817n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 2 (171, 175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(171n);
    input.add64(175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(175n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 3 (175, 175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(175n);
    input.add64(175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(175n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 4 (175, 171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(175n);
    input.add64(171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(175n);
  });

  it('test operator "add" overload (euint8, euint128) => euint128 test 1 (2, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add128(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint8, euint128) => euint128 test 2 (22, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(22n);
    input.add128(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(48n);
  });

  it('test operator "add" overload (euint8, euint128) => euint128 test 3 (26, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(26n);
    input.add128(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(52n);
  });

  it('test operator "add" overload (euint8, euint128) => euint128 test 4 (26, 22)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(26n);
    input.add128(22n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(48n);
  });

  it('test operator "sub" overload (euint8, euint128) => euint128 test 1 (85, 85)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(85n);
    input.add128(85n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint128) => euint128 test 2 (85, 81)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(85n);
    input.add128(81n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint128) => euint128 test 1 (2, 65)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add128(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint8, euint128) => euint128 test 2 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(11n);
    input.add128(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(121n);
  });

  it('test operator "mul" overload (euint8, euint128) => euint128 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(11n);
    input.add128(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(121n);
  });

  it('test operator "mul" overload (euint8, euint128) => euint128 test 4 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(11n);
    input.add128(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(121n);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 1 (100, 340282366920938463463366068297266700057)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(100n);
    input.add128(340282366920938463463366068297266700057n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 2 (96, 100)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(96n);
    input.add128(100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(96n);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 3 (100, 100)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(100n);
    input.add128(100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(100n);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 4 (100, 96)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(100n);
    input.add128(96n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(96n);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 1 (158, 340282366920938463463372590921736204849)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(158n);
    input.add128(340282366920938463463372590921736204849n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(340282366920938463463372590921736204991n);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 2 (154, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(154n);
    input.add128(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(158n);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 3 (158, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(158n);
    input.add128(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(158n);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 4 (158, 154)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(158n);
    input.add128(154n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(158n);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 1 (26, 340282366920938463463372115481041226887)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(26n);
    input.add128(340282366920938463463372115481041226887n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(340282366920938463463372115481041226909n);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 2 (22, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(22n);
    input.add128(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 3 (26, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(26n);
    input.add128(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 4 (26, 22)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(26n);
    input.add128(22n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, euint128) => ebool test 1 (188, 340282366920938463463370622944712692871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(188n);
    input.add128(340282366920938463463370622944712692871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint128) => ebool test 2 (184, 188)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(184n);
    input.add128(188n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint128) => ebool test 3 (188, 188)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(188n);
    input.add128(188n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint128) => ebool test 4 (188, 184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(188n);
    input.add128(184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 1 (27, 340282366920938463463370317277162459171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(27n);
    input.add128(340282366920938463463370317277162459171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 2 (23, 27)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(23n);
    input.add128(27n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 3 (27, 27)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(27n);
    input.add128(27n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 4 (27, 23)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(27n);
    input.add128(23n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint128) => ebool test 1 (203, 340282366920938463463373719383951386955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(203n);
    input.add128(340282366920938463463373719383951386955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint128) => ebool test 2 (199, 203)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(199n);
    input.add128(203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint128) => ebool test 3 (203, 203)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(203n);
    input.add128(203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint128) => ebool test 4 (203, 199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(203n);
    input.add128(199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint128) => ebool test 1 (114, 340282366920938463463373925202094169321)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(114n);
    input.add128(340282366920938463463373925202094169321n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint128) => ebool test 2 (110, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(110n);
    input.add128(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint128) => ebool test 3 (114, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(114n);
    input.add128(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint128) => ebool test 4 (114, 110)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(114n);
    input.add128(110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint128) => ebool test 1 (247, 340282366920938463463371438987448754081)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(247n);
    input.add128(340282366920938463463371438987448754081n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint128) => ebool test 2 (243, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(243n);
    input.add128(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint128) => ebool test 3 (247, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(247n);
    input.add128(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint128) => ebool test 4 (247, 243)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(247n);
    input.add128(243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint128) => ebool test 1 (201, 340282366920938463463366008985670092061)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(201n);
    input.add128(340282366920938463463366008985670092061n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint128) => ebool test 2 (197, 201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(197n);
    input.add128(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint128) => ebool test 3 (201, 201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(201n);
    input.add128(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint128) => ebool test 4 (201, 197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(201n);
    input.add128(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint128) => euint128 test 1 (108, 340282366920938463463373043906671604877)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(108n);
    input.add128(340282366920938463463373043906671604877n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(108n);
  });

  it('test operator "min" overload (euint8, euint128) => euint128 test 2 (104, 108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(104n);
    input.add128(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(104n);
  });

  it('test operator "min" overload (euint8, euint128) => euint128 test 3 (108, 108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(108n);
    input.add128(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(108n);
  });

  it('test operator "min" overload (euint8, euint128) => euint128 test 4 (108, 104)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(108n);
    input.add128(104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(104n);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 1 (202, 340282366920938463463365858450728191317)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(202n);
    input.add128(340282366920938463463365858450728191317n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(340282366920938463463365858450728191317n);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 2 (198, 202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(198n);
    input.add128(202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(202n);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 3 (202, 202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(202n);
    input.add128(202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(202n);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 4 (202, 198)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(202n);
    input.add128(198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.resEuint128());
    expect(res).to.equal(202n);
  });

  it('test operator "and" overload (euint8, euint256) => euint256 test 1 (13, 115792089237316195423570985008687907853269984665640564039457575218153070149415)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(13n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575218153070149415n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(5n);
  });

  it('test operator "and" overload (euint8, euint256) => euint256 test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add256(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint8, euint256) => euint256 test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(13n);
    input.add256(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(13n);
  });

  it('test operator "and" overload (euint8, euint256) => euint256 test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(13n);
    input.add256(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(9n);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 1 (75, 115792089237316195423570985008687907853269984665640564039457580124085996979419)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(75n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580124085996979419n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457580124085996979419n);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 2 (71, 75)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(71n);
    input.add256(75n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(79n);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 3 (75, 75)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(75n);
    input.add256(75n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(75n);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 4 (75, 71)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(75n);
    input.add256(71n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(79n);
  });

  it('test operator "xor" overload (euint8, euint256) => euint256 test 1 (61, 115792089237316195423570985008687907853269984665640564039457583819463386448981)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(61n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583819463386448981n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583819463386449000n);
  });

  it('test operator "xor" overload (euint8, euint256) => euint256 test 2 (57, 61)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(57n);
    input.add256(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint256) => euint256 test 3 (61, 61)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(61n);
    input.add256(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint256) => euint256 test 4 (61, 57)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(61n);
    input.add256(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract1.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 1 (53, 115792089237316195423570985008687907853269984665640564039457578095327928626201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(53n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578095327928626201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 2 (49, 53)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(49n);
    input.add256(53n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 3 (53, 53)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(53n);
    input.add256(53n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 4 (53, 49)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(53n);
    input.add256(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint256) => ebool test 1 (114, 115792089237316195423570985008687907853269984665640564039457583831197830118433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(114n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583831197830118433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint256) => ebool test 2 (110, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(110n);
    input.add256(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint256) => ebool test 3 (114, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(114n);
    input.add256(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint256) => ebool test 4 (114, 110)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(114n);
    input.add256(110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 1 (135, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(135n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(137n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 2 (69, 73)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(69n);
    input.add8(73n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(142n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 3 (73, 73)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(73n);
    input.add8(73n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(146n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 4 (73, 69)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(73n);
    input.add8(69n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(142n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (147, 147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(147n);
    input.add8(147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (147, 143)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(147n);
    input.add8(143n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (66, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(66n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(132n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 2 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 4 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(196n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 1 (5404, 231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(5404n);
    input.add8(231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 2 (227, 231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(227n);
    input.add8(231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(227n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 3 (231, 231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(231n);
    input.add8(231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(231n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 4 (231, 227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(231n);
    input.add8(227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(227n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 1 (1200, 229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(1200n);
    input.add8(229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(1269n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 2 (225, 229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(225n);
    input.add8(229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(229n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 3 (229, 229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(229n);
    input.add8(229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(229n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 4 (229, 225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(229n);
    input.add8(225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(229n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (13394, 197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(13394n);
    input.add8(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(13463n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (193, 197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(193n);
    input.add8(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 3 (197, 197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(197n);
    input.add8(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 4 (197, 193)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(197n);
    input.add8(193n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 1 (25318, 151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(25318n);
    input.add8(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 2 (147, 151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(147n);
    input.add8(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 3 (151, 151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(151n);
    input.add8(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 4 (151, 147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(151n);
    input.add8(147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (2292, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(2292n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (40, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(40n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 3 (44, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(44n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 4 (44, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(44n);
    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (64995, 132)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(64995n);
    input.add8(132n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (128, 132)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(128n);
    input.add8(132n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (132, 132)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(132n);
    input.add8(132n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 4 (132, 128)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(132n);
    input.add8(128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 1 (2900, 218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(2900n);
    input.add8(218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 2 (214, 218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(214n);
    input.add8(218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 3 (218, 218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(218n);
    input.add8(218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 4 (218, 214)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(218n);
    input.add8(214n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 1 (12409, 190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(12409n);
    input.add8(190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 2 (186, 190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(186n);
    input.add8(190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 3 (190, 190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(190n);
    input.add8(190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 4 (190, 186)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(190n);
    input.add8(186n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 1 (60440, 23)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(60440n);
    input.add8(23n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 2 (19, 23)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(19n);
    input.add8(23n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 3 (23, 23)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(23n);
    input.add8(23n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 4 (23, 19)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(23n);
    input.add8(19n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 1 (13560, 156)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(13560n);
    input.add8(156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(156n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 2 (152, 156)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(152n);
    input.add8(156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(152n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 3 (156, 156)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(156n);
    input.add8(156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(156n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 4 (156, 152)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(156n);
    input.add8(152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(152n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 1 (30558, 133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(30558n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(30558n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 2 (129, 133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(129n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(133n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 3 (133, 133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(133n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(133n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 4 (133, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(133n);
    input.add8(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(133n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 1 (32611, 19452)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(32611n);
    input.add16(19452n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(52063n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 2 (19450, 19452)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(19450n);
    input.add16(19452n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(38902n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 3 (19452, 19452)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(19452n);
    input.add16(19452n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(38904n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 4 (19452, 19450)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(19452n);
    input.add16(19450n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(38902n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 1 (3298, 3298)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3298n);
    input.add16(3298n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 2 (3298, 3294)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3298n);
    input.add16(3294n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 1 (90, 467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(90n);
    input.add16(467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(42030n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 2 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(179n);
    input.add16(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(32041n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 3 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(179n);
    input.add16(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(32041n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 4 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(179n);
    input.add16(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(32041n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 1 (62109, 49287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(62109n);
    input.add16(49287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(49285n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 2 (49283, 49287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49283n);
    input.add16(49287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(49283n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 3 (49287, 49287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49287n);
    input.add16(49287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(49287n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 4 (49287, 49283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49287n);
    input.add16(49283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(49283n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 1 (63081, 15211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(63081n);
    input.add16(15211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(65387n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 2 (15207, 15211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15207n);
    input.add16(15211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(15215n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 3 (15211, 15211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15211n);
    input.add16(15211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(15211n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 4 (15211, 15207)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15211n);
    input.add16(15207n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(15215n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (65065, 61749)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(65065n);
    input.add16(61749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(3868n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (61745, 61749)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(61745n);
    input.add16(61749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 3 (61749, 61749)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(61749n);
    input.add16(61749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 4 (61749, 61745)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(61749n);
    input.add16(61745n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 1 (32412, 31798)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32412n);
    input.add16(31798n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 2 (31794, 31798)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(31794n);
    input.add16(31798n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 3 (31798, 31798)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(31798n);
    input.add16(31798n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 4 (31798, 31794)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(31798n);
    input.add16(31794n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 1 (37435, 63070)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37435n);
    input.add16(63070n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 2 (37431, 37435)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37431n);
    input.add16(37435n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 3 (37435, 37435)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37435n);
    input.add16(37435n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 4 (37435, 37431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37435n);
    input.add16(37431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 1 (38438, 38593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(38438n);
    input.add16(38593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 2 (38434, 38438)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(38434n);
    input.add16(38438n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 3 (38438, 38438)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(38438n);
    input.add16(38438n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 4 (38438, 38434)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(38438n);
    input.add16(38434n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 1 (62495, 23014)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(62495n);
    input.add16(23014n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 2 (23010, 23014)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23010n);
    input.add16(23014n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 3 (23014, 23014)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23014n);
    input.add16(23014n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 4 (23014, 23010)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23014n);
    input.add16(23010n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 1 (4364, 10146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4364n);
    input.add16(10146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 2 (4360, 4364)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4360n);
    input.add16(4364n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 3 (4364, 4364)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4364n);
    input.add16(4364n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 4 (4364, 4360)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4364n);
    input.add16(4360n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 1 (64494, 2879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64494n);
    input.add16(2879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 2 (2875, 2879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2875n);
    input.add16(2879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 3 (2879, 2879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2879n);
    input.add16(2879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 4 (2879, 2875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2879n);
    input.add16(2875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 1 (16825, 38046)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16825n);
    input.add16(38046n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(16825n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 2 (16821, 16825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16821n);
    input.add16(16825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(16821n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 3 (16825, 16825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16825n);
    input.add16(16825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(16825n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 4 (16825, 16821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16825n);
    input.add16(16821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(16821n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 1 (19895, 49192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(19895n);
    input.add16(49192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(49192n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 2 (19891, 19895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(19891n);
    input.add16(19895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(19895n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 3 (19895, 19895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(19895n);
    input.add16(19895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(19895n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 4 (19895, 19891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(19895n);
    input.add16(19891n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract2.resEuint16());
    expect(res).to.equal(19895n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 1 (3, 44648)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3n);
    input.add32(44648n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(44651n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 2 (19671, 19673)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(19671n);
    input.add32(19673n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(39344n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 3 (19673, 19673)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(19673n);
    input.add32(19673n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(39346n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 4 (19673, 19671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(19673n);
    input.add32(19671n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(39344n);
  });
});
