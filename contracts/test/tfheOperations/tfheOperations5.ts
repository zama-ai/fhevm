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

    const contract7 = await deployTfheTestFixture7();
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 1 (2, 1073741825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add128(1073741825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 2 (43656, 43656)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(43656n);
    input.add128(43656n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(1905846336n);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 3 (43656, 43656)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(43656n);
    input.add128(43656n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(1905846336n);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 4 (43656, 43656)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(43656n);
    input.add128(43656n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(1905846336n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 1 (1221126130, 340282366920938463463367453770364138787)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1221126130n);
    input.add128(340282366920938463463367453770364138787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(1208526114n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 2 (1221126126, 1221126130)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1221126126n);
    input.add128(1221126130n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(1221126114n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 3 (1221126130, 1221126130)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1221126130n);
    input.add128(1221126130n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(1221126130n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 4 (1221126130, 1221126126)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1221126130n);
    input.add128(1221126126n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(1221126114n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 1 (2550467444, 340282366920938463463367107707936915327)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2550467444n);
    input.add128(340282366920938463463367107707936915327n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(340282366920938463463367107710084398975n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 2 (2550467440, 2550467444)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2550467440n);
    input.add128(2550467444n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(2550467444n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 3 (2550467444, 2550467444)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2550467444n);
    input.add128(2550467444n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(2550467444n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 4 (2550467444, 2550467440)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2550467444n);
    input.add128(2550467440n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(2550467444n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 1 (1117423640, 340282366920938463463370235378880074391)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1117423640n);
    input.add128(340282366920938463463370235378880074391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(340282366920938463463370235379979606671n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 2 (1117423636, 1117423640)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1117423636n);
    input.add128(1117423640n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 3 (1117423640, 1117423640)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1117423640n);
    input.add128(1117423640n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 4 (1117423640, 1117423636)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1117423640n);
    input.add128(1117423636n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 1 (3531028786, 340282366920938463463366811671598605485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3531028786n);
    input.add128(340282366920938463463366811671598605485n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 2 (3531028782, 3531028786)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3531028782n);
    input.add128(3531028786n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 3 (3531028786, 3531028786)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3531028786n);
    input.add128(3531028786n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 4 (3531028786, 3531028782)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3531028786n);
    input.add128(3531028782n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 1 (1536568261, 340282366920938463463374530401952821231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1536568261n);
    input.add128(340282366920938463463374530401952821231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 2 (1536568257, 1536568261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1536568257n);
    input.add128(1536568261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 3 (1536568261, 1536568261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1536568261n);
    input.add128(1536568261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 4 (1536568261, 1536568257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1536568261n);
    input.add128(1536568257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 1 (3860846754, 340282366920938463463369529720785392975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3860846754n);
    input.add128(340282366920938463463369529720785392975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 2 (3860846750, 3860846754)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3860846750n);
    input.add128(3860846754n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 3 (3860846754, 3860846754)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3860846754n);
    input.add128(3860846754n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 4 (3860846754, 3860846750)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3860846754n);
    input.add128(3860846750n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 1 (2944878484, 340282366920938463463373169734005657705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2944878484n);
    input.add128(340282366920938463463373169734005657705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 2 (2944878480, 2944878484)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2944878480n);
    input.add128(2944878484n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 3 (2944878484, 2944878484)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2944878484n);
    input.add128(2944878484n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 4 (2944878484, 2944878480)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2944878484n);
    input.add128(2944878480n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 1 (4006780942, 340282366920938463463373056740463490379)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4006780942n);
    input.add128(340282366920938463463373056740463490379n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 2 (4006780938, 4006780942)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4006780938n);
    input.add128(4006780942n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 3 (4006780942, 4006780942)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4006780942n);
    input.add128(4006780942n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 4 (4006780942, 4006780938)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4006780942n);
    input.add128(4006780938n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 1 (4093831401, 340282366920938463463370640742779314905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4093831401n);
    input.add128(340282366920938463463370640742779314905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 2 (4093831397, 4093831401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4093831397n);
    input.add128(4093831401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 3 (4093831401, 4093831401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4093831401n);
    input.add128(4093831401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 4 (4093831401, 4093831397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4093831401n);
    input.add128(4093831397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 1 (3142833719, 340282366920938463463367493887949702525)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3142833719n);
    input.add128(340282366920938463463367493887949702525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(3142833719n);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 2 (3142833715, 3142833719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3142833715n);
    input.add128(3142833719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(3142833715n);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 3 (3142833719, 3142833719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3142833719n);
    input.add128(3142833719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(3142833719n);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 4 (3142833719, 3142833715)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3142833719n);
    input.add128(3142833715n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(3142833715n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 1 (997780440, 340282366920938463463371685379323045809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(997780440n);
    input.add128(340282366920938463463371685379323045809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(340282366920938463463371685379323045809n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 2 (997780436, 997780440)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(997780436n);
    input.add128(997780440n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(997780440n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 3 (997780440, 997780440)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(997780440n);
    input.add128(997780440n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(997780440n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 4 (997780440, 997780436)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(997780440n);
    input.add128(997780436n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(997780440n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 1 (57146110, 115792089237316195423570985008687907853269984665640564039457578232583802901551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(57146110n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578232583802901551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(50647086n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 2 (57146106, 57146110)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(57146106n);
    input.add256(57146110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(57146106n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 3 (57146110, 57146110)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(57146110n);
    input.add256(57146110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(57146110n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 4 (57146110, 57146106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(57146110n);
    input.add256(57146106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(57146106n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 1 (1462351308, 115792089237316195423570985008687907853269984665640564039457580566314915181255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1462351308n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580566314915181255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457580566315033147343n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 2 (1462351304, 1462351308)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1462351304n);
    input.add256(1462351308n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(1462351308n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 3 (1462351308, 1462351308)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1462351308n);
    input.add256(1462351308n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(1462351308n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 4 (1462351308, 1462351304)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1462351308n);
    input.add256(1462351304n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(1462351308n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 1 (4100331626, 115792089237316195423570985008687907853269984665640564039457579104573394395227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4100331626n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579104573394395227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579104575338068017n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 2 (4100331622, 4100331626)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4100331622n);
    input.add256(4100331626n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 3 (4100331626, 4100331626)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4100331626n);
    input.add256(4100331626n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 4 (4100331626, 4100331622)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4100331626n);
    input.add256(4100331622n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 1 (186975223, 115792089237316195423570985008687907853269984665640564039457575092791715143817)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(186975223n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575092791715143817n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 2 (186975219, 186975223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(186975219n);
    input.add256(186975223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 3 (186975223, 186975223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(186975223n);
    input.add256(186975223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 4 (186975223, 186975219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(186975223n);
    input.add256(186975219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 1 (1311509969, 115792089237316195423570985008687907853269984665640564039457581429136404368565)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1311509969n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581429136404368565n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 2 (1311509965, 1311509969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1311509965n);
    input.add256(1311509969n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 3 (1311509969, 1311509969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1311509969n);
    input.add256(1311509969n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 4 (1311509969, 1311509965)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1311509969n);
    input.add256(1311509965n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 1 (129, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 2 (101, 105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(101n);
    input.add8(105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(206n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 3 (105, 105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(105n);
    input.add8(105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(210n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 4 (105, 101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(105n);
    input.add8(101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(206n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 1 (47, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(47n);
    input.add8(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 2 (47, 43)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(47n);
    input.add8(43n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 1 (65, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(65n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 2 (15, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(15n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(225n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 3 (15, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(15n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(225n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 4 (15, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(15n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(225n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 1 (18438440372159966899, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438440372159966899n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(16n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 2 (88, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(88n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(88n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 3 (92, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(92n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(92n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 4 (92, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(92n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(88n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 1 (18444787798790127143, 195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18444787798790127143n);
    input.add8(195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18444787798790127335n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 2 (191, 195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(191n);
    input.add8(195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(255n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 3 (195, 195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(195n);
    input.add8(195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(195n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 4 (195, 191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(195n);
    input.add8(191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(255n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 1 (18442480708808444555, 86)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442480708808444555n);
    input.add8(86n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18442480708808444637n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 2 (82, 86)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(82n);
    input.add8(86n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 3 (86, 86)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(86n);
    input.add8(86n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 4 (86, 82)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(86n);
    input.add8(82n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 1 (18440565881742808063, 237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18440565881742808063n);
    input.add8(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 2 (233, 237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(233n);
    input.add8(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 3 (237, 237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(237n);
    input.add8(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 4 (237, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(237n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 1 (18445018242261622013, 46)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445018242261622013n);
    input.add8(46n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 2 (42, 46)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(42n);
    input.add8(46n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 3 (46, 46)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(46n);
    input.add8(46n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 4 (46, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(46n);
    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 1 (18442177379903303423, 216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442177379903303423n);
    input.add8(216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 2 (212, 216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(212n);
    input.add8(216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 3 (216, 216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(216n);
    input.add8(216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 4 (216, 212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(216n);
    input.add8(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 1 (18441374570153895733, 181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18441374570153895733n);
    input.add8(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 2 (177, 181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(177n);
    input.add8(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 3 (181, 181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(181n);
    input.add8(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 4 (181, 177)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(181n);
    input.add8(177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 1 (18445400825795397061, 81)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445400825795397061n);
    input.add8(81n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 2 (77, 81)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(77n);
    input.add8(81n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 3 (81, 81)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(81n);
    input.add8(81n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 4 (81, 77)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(81n);
    input.add8(77n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 1 (18445472397917602557, 126)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445472397917602557n);
    input.add8(126n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 2 (122, 126)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(122n);
    input.add8(126n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 3 (126, 126)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(126n);
    input.add8(126n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 4 (126, 122)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(126n);
    input.add8(122n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 1 (18439603251132400909, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18439603251132400909n);
    input.add8(28n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(28n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 2 (24, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(24n);
    input.add8(28n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(24n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 3 (28, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(28n);
    input.add8(28n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(28n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 4 (28, 24)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(28n);
    input.add8(24n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(24n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 1 (18438611224096970641, 241)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438611224096970641n);
    input.add8(241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18438611224096970641n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 2 (237, 241)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(237n);
    input.add8(241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(241n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 3 (241, 241)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(241n);
    input.add8(241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(241n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 4 (241, 237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(241n);
    input.add8(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(241n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 1 (65519, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(65519n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(65521n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 2 (5175, 5179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(5175n);
    input.add16(5179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(10354n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 3 (5179, 5179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(5179n);
    input.add16(5179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(10358n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 4 (5179, 5175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(5179n);
    input.add16(5175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(10354n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 1 (65307, 65307)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(65307n);
    input.add16(65307n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 2 (65307, 65303)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(65307n);
    input.add16(65303n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 1 (16385, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(16385n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 2 (231, 231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(231n);
    input.add16(231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(53361n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 3 (231, 231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(231n);
    input.add16(231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(53361n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 4 (231, 231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(231n);
    input.add16(231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(53361n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 1 (18440491305727720617, 3097)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18440491305727720617n);
    input.add16(3097n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3081n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 2 (3093, 3097)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3093n);
    input.add16(3097n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3089n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 3 (3097, 3097)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3097n);
    input.add16(3097n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3097n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 4 (3097, 3093)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3097n);
    input.add16(3093n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3089n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 1 (18440793108071415095, 60842)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18440793108071415095n);
    input.add16(60842n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18440793108071447999n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 2 (60838, 60842)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(60838n);
    input.add16(60842n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(60846n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 3 (60842, 60842)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(60842n);
    input.add16(60842n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(60842n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 4 (60842, 60838)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(60842n);
    input.add16(60838n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(60846n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 1 (18438586875105638233, 36431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438586875105638233n);
    input.add16(36431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18438586875105605910n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 2 (36427, 36431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(36427n);
    input.add16(36431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 3 (36431, 36431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(36431n);
    input.add16(36431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 4 (36431, 36427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(36431n);
    input.add16(36427n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 1 (18437889848008869003, 51179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18437889848008869003n);
    input.add16(51179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 2 (51175, 51179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(51175n);
    input.add16(51179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 3 (51179, 51179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(51179n);
    input.add16(51179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 4 (51179, 51175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(51179n);
    input.add16(51175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 1 (18443299240954649069, 10646)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18443299240954649069n);
    input.add16(10646n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 2 (10642, 10646)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(10642n);
    input.add16(10646n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 3 (10646, 10646)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(10646n);
    input.add16(10646n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 4 (10646, 10642)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(10646n);
    input.add16(10642n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 1 (18438401013641950553, 28364)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438401013641950553n);
    input.add16(28364n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 2 (28360, 28364)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(28360n);
    input.add16(28364n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 3 (28364, 28364)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(28364n);
    input.add16(28364n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 4 (28364, 28360)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(28364n);
    input.add16(28360n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 1 (18440973805046632507, 38000)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18440973805046632507n);
    input.add16(38000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 2 (37996, 38000)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(37996n);
    input.add16(38000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 3 (38000, 38000)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(38000n);
    input.add16(38000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 4 (38000, 37996)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(38000n);
    input.add16(37996n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 1 (18444746103570142687, 55027)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18444746103570142687n);
    input.add16(55027n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 2 (55023, 55027)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(55023n);
    input.add16(55027n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 3 (55027, 55027)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(55027n);
    input.add16(55027n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 4 (55027, 55023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(55027n);
    input.add16(55023n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 1 (18438585229002067113, 62377)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438585229002067113n);
    input.add16(62377n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 2 (62373, 62377)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(62373n);
    input.add16(62377n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 3 (62377, 62377)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(62377n);
    input.add16(62377n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 4 (62377, 62373)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(62377n);
    input.add16(62373n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 1 (18445731688500701953, 25217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445731688500701953n);
    input.add16(25217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(25217n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 2 (25213, 25217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(25213n);
    input.add16(25217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(25213n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 3 (25217, 25217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(25217n);
    input.add16(25217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(25217n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 4 (25217, 25213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(25217n);
    input.add16(25213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(25213n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (18443644310242825963, 41202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18443644310242825963n);
    input.add16(41202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18443644310242825963n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (41198, 41202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(41198n);
    input.add16(41202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(41202n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (41202, 41202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(41202n);
    input.add16(41202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(41202n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (41202, 41198)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(41202n);
    input.add16(41198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(41202n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 1 (4294811725, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4294811725n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4294811727n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 2 (1320163939, 1320163941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1320163939n);
    input.add32(1320163941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2640327880n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 3 (1320163941, 1320163941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1320163941n);
    input.add32(1320163941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2640327882n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 4 (1320163941, 1320163939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1320163941n);
    input.add32(1320163939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2640327880n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 1 (567654749, 567654749)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(567654749n);
    input.add32(567654749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 2 (567654749, 567654745)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(567654749n);
    input.add32(567654745n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 1 (2146837243, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2146837243n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4293674486n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 2 (43811, 43811)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(43811n);
    input.add32(43811n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1919403721n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 3 (43811, 43811)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(43811n);
    input.add32(43811n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1919403721n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 4 (43811, 43811)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(43811n);
    input.add32(43811n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1919403721n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 1 (18443071316001237667, 862862162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18443071316001237667n);
    input.add32(862862162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(828516866n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 2 (862862158, 862862162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(862862158n);
    input.add32(862862162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(862862146n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 3 (862862162, 862862162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(862862162n);
    input.add32(862862162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(862862162n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 4 (862862162, 862862158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(862862162n);
    input.add32(862862158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(862862146n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 1 (18438499835326114733, 1086790514)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438499835326114733n);
    input.add32(1086790514n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18438499836404120575n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 2 (1086790510, 1086790514)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1086790510n);
    input.add32(1086790514n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1086790526n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 3 (1086790514, 1086790514)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1086790514n);
    input.add32(1086790514n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1086790514n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 4 (1086790514, 1086790510)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1086790514n);
    input.add32(1086790510n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1086790526n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 1 (18441642819691562677, 106112975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18441642819691562677n);
    input.add32(106112975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18441642819585728890n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 2 (106112971, 106112975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(106112971n);
    input.add32(106112975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 3 (106112975, 106112975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(106112975n);
    input.add32(106112975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 4 (106112975, 106112971)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(106112975n);
    input.add32(106112971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 1 (18445215407114453543, 1077121481)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445215407114453543n);
    input.add32(1077121481n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 2 (1077121477, 1077121481)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1077121477n);
    input.add32(1077121481n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 3 (1077121481, 1077121481)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1077121481n);
    input.add32(1077121481n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 4 (1077121481, 1077121477)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1077121481n);
    input.add32(1077121477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });
});
