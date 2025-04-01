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

describe('TFHE operations 4', function () {
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

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (2499145589, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2499145589n);
    input.add8(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (170, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(170n);
    input.add8(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 3 (174, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(174n);
    input.add8(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 4 (174, 170)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(174n);
    input.add8(170n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (2586137777, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2586137777n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (40, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(40n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 3 (44, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(44n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 4 (44, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(44n);
    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (849637253, 168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(849637253n);
    input.add8(168n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (164, 168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(164n);
    input.add8(168n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (168, 168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(168n);
    input.add8(168n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 4 (168, 164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(168n);
    input.add8(164n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (2755945115, 105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2755945115n);
    input.add8(105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (101, 105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(101n);
    input.add8(105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (105, 105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(105n);
    input.add8(105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 4 (105, 101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(105n);
    input.add8(101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (1314066261, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1314066261n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (3655464961, 218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3655464961n);
    input.add8(218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (214, 218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(214n);
    input.add8(218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (218, 218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(218n);
    input.add8(218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 4 (218, 214)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(218n);
    input.add8(214n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (2593855758, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2593855758n);
    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(42n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (38, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(38n);
    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(38n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (42, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(42n);
    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(42n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (42, 38)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(42n);
    input.add8(38n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(38n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (77856294, 242)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(77856294n);
    input.add8(242n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(77856294n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (238, 242)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(238n);
    input.add8(242n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(242n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (242, 242)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(242n);
    input.add8(242n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(242n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 4 (242, 238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(242n);
    input.add8(238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(242n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (61882, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(61882n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(61884n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (17571, 17575)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(17571n);
    input.add16(17575n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(35146n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (17575, 17575)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(17575n);
    input.add16(17575n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(35150n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (17575, 17571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(17575n);
    input.add16(17571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(35146n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (61851, 61851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(61851n);
    input.add16(61851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (61851, 61847)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(61851n);
    input.add16(61847n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (6472, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(6472n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(51776n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (202, 202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(202n);
    input.add16(202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(40804n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (202, 202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(202n);
    input.add16(202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(40804n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (202, 202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(202n);
    input.add16(202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(40804n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (524605178, 56557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(524605178n);
    input.add16(56557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(54504n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (56553, 56557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(56553n);
    input.add16(56557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(56553n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (56557, 56557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(56557n);
    input.add16(56557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(56557n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (56557, 56553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(56557n);
    input.add16(56553n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(56553n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (4070973995, 39385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(4070973995n);
    input.add16(39385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4071013371n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (39381, 39385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(39381n);
    input.add16(39385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(39389n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 3 (39385, 39385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(39385n);
    input.add16(39385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(39385n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 4 (39385, 39381)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(39385n);
    input.add16(39381n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(39389n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (3299437980, 57151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3299437980n);
    input.add16(57151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(3299455651n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (57147, 57151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(57147n);
    input.add16(57151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (57151, 57151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(57151n);
    input.add16(57151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (57151, 57147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(57151n);
    input.add16(57147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (437033026, 4295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(437033026n);
    input.add16(4295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (4291, 4295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(4291n);
    input.add16(4295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (4295, 4295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(4295n);
    input.add16(4295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (4295, 4291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(4295n);
    input.add16(4291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (2200329317, 10570)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2200329317n);
    input.add16(10570n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (10566, 10570)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(10566n);
    input.add16(10570n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 3 (10570, 10570)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(10570n);
    input.add16(10570n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 4 (10570, 10566)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(10570n);
    input.add16(10566n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (3725101705, 48046)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3725101705n);
    input.add16(48046n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (48042, 48046)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(48042n);
    input.add16(48046n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (48046, 48046)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(48046n);
    input.add16(48046n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 4 (48046, 48042)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(48046n);
    input.add16(48042n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (459976621, 17023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(459976621n);
    input.add16(17023n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (17019, 17023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(17019n);
    input.add16(17023n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (17023, 17023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(17023n);
    input.add16(17023n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 4 (17023, 17019)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(17023n);
    input.add16(17019n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (2760951255, 11647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2760951255n);
    input.add16(11647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (11643, 11647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(11643n);
    input.add16(11647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (11647, 11647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(11647n);
    input.add16(11647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (11647, 11643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(11647n);
    input.add16(11643n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (1746725591, 42753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1746725591n);
    input.add16(42753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (42749, 42753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(42749n);
    input.add16(42753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (42753, 42753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(42753n);
    input.add16(42753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (42753, 42749)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(42753n);
    input.add16(42749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (1271396225, 37739)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1271396225n);
    input.add16(37739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(37739n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (37735, 37739)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(37735n);
    input.add16(37739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(37735n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (37739, 37739)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(37739n);
    input.add16(37739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(37739n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (37739, 37735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(37739n);
    input.add16(37735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(37735n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (1929184521, 5853)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1929184521n);
    input.add16(5853n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(1929184521n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (5849, 5853)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(5849n);
    input.add16(5853n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(5853n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (5853, 5853)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(5853n);
    input.add16(5853n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(5853n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (5853, 5849)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(5853n);
    input.add16(5849n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(5853n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (103200885, 1671527490)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(103200885n);
    input.add32(1671527490n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(1774728375n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (103200881, 103200885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(103200881n);
    input.add32(103200885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(206401766n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (103200885, 103200885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(103200885n);
    input.add32(103200885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(206401770n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (103200885, 103200881)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(103200885n);
    input.add32(103200881n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(206401766n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (3448348036, 3448348036)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3448348036n);
    input.add32(3448348036n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (3448348036, 3448348032)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3448348036n);
    input.add32(3448348032n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (51487, 77044)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(51487n);
    input.add32(77044n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(3966764428n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 2 (51487, 51487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(51487n);
    input.add32(51487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(2650911169n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 3 (51487, 51487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(51487n);
    input.add32(51487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(2650911169n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 4 (51487, 51487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(51487n);
    input.add32(51487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(2650911169n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (1969251182, 3254160081)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1969251182n);
    input.add32(3254160081n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1096811072n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (1969251178, 1969251182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1969251178n);
    input.add32(1969251182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1969251178n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 3 (1969251182, 1969251182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1969251182n);
    input.add32(1969251182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1969251182n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 4 (1969251182, 1969251178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1969251182n);
    input.add32(1969251178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1969251178n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (1112287315, 1079582008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1112287315n);
    input.add32(1079582008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1113406843n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (1079582004, 1079582008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1079582004n);
    input.add32(1079582008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1079582012n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 3 (1079582008, 1079582008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1079582008n);
    input.add32(1079582008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1079582008n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 4 (1079582008, 1079582004)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1079582008n);
    input.add32(1079582004n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1079582012n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (3137527105, 3879506182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3137527105n);
    input.add32(3879506182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1547586631n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (3137527101, 3137527105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3137527101n);
    input.add32(3137527105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (3137527105, 3137527105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3137527105n);
    input.add32(3137527105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (3137527105, 3137527101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3137527105n);
    input.add32(3137527101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(124n);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (4198345697, 2959962293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4198345697n);
    input.add32(2959962293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (2959962289, 2959962293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2959962289n);
    input.add32(2959962293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (2959962293, 2959962293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2959962293n);
    input.add32(2959962293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (2959962293, 2959962289)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2959962293n);
    input.add32(2959962289n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (4142346425, 3056969939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4142346425n);
    input.add32(3056969939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (3056969935, 3056969939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3056969935n);
    input.add32(3056969939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 3 (3056969939, 3056969939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3056969939n);
    input.add32(3056969939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 4 (3056969939, 3056969935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3056969939n);
    input.add32(3056969935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (3417794908, 2711241123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3417794908n);
    input.add32(2711241123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (2711241119, 2711241123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2711241119n);
    input.add32(2711241123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (2711241123, 2711241123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2711241123n);
    input.add32(2711241123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (2711241123, 2711241119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2711241123n);
    input.add32(2711241119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (1992276575, 1415326401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1992276575n);
    input.add32(1415326401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (1415326397, 1415326401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1415326397n);
    input.add32(1415326401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (1415326401, 1415326401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1415326401n);
    input.add32(1415326401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 4 (1415326401, 1415326397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1415326401n);
    input.add32(1415326397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (2411753996, 3974172316)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2411753996n);
    input.add32(3974172316n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (2411753992, 2411753996)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2411753992n);
    input.add32(2411753996n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (2411753996, 2411753996)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2411753996n);
    input.add32(2411753996n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 4 (2411753996, 2411753992)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2411753996n);
    input.add32(2411753992n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (1147819287, 3545265248)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1147819287n);
    input.add32(3545265248n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (1147819283, 1147819287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1147819283n);
    input.add32(1147819287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (1147819287, 1147819287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1147819287n);
    input.add32(1147819287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (1147819287, 1147819283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1147819287n);
    input.add32(1147819283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (1318813997, 2273629365)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1318813997n);
    input.add32(2273629365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1318813997n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (1318813993, 1318813997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1318813993n);
    input.add32(1318813997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1318813993n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (1318813997, 1318813997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1318813997n);
    input.add32(1318813997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1318813997n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (1318813997, 1318813993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1318813997n);
    input.add32(1318813993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1318813993n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (2603395582, 1657657823)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2603395582n);
    input.add32(1657657823n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(2603395582n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (1657657819, 1657657823)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1657657819n);
    input.add32(1657657823n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1657657823n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (1657657823, 1657657823)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1657657823n);
    input.add32(1657657823n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1657657823n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (1657657823, 1657657819)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1657657823n);
    input.add32(1657657819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1657657823n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 1 (2, 4293131319)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add64(4293131319n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4293131321n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 2 (1405930973, 1405930975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1405930973n);
    input.add64(1405930975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2811861948n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 3 (1405930975, 1405930975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1405930975n);
    input.add64(1405930975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2811861950n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 4 (1405930975, 1405930973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1405930975n);
    input.add64(1405930973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2811861948n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (2974194807, 2974194807)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2974194807n);
    input.add64(2974194807n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (2974194807, 2974194803)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2974194807n);
    input.add64(2974194803n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (2, 2146512324)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add64(2146512324n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4293024648n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (53064, 53064)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(53064n);
    input.add64(53064n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2815788096n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (53064, 53064)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(53064n);
    input.add64(53064n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2815788096n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (53064, 53064)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(53064n);
    input.add64(53064n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2815788096n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 1 (1759499542, 18445364766477752951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1759499542n);
    input.add64(18445364766477752951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4821014n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 2 (1759499538, 1759499542)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1759499538n);
    input.add64(1759499542n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1759499538n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 3 (1759499542, 1759499542)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1759499542n);
    input.add64(1759499542n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1759499542n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 4 (1759499542, 1759499538)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1759499542n);
    input.add64(1759499538n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1759499538n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 1 (2004378166, 18440784962293287469)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2004378166n);
    input.add64(18440784962293287469n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18440784963218139711n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 2 (2004378162, 2004378166)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2004378162n);
    input.add64(2004378166n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2004378166n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 3 (2004378166, 2004378166)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2004378166n);
    input.add64(2004378166n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2004378166n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 4 (2004378166, 2004378162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2004378166n);
    input.add64(2004378162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2004378166n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (4209186687, 18444052551934801319)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4209186687n);
    input.add64(18444052551934801319n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18444052553975390936n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (4209186683, 4209186687)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4209186683n);
    input.add64(4209186687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (4209186687, 4209186687)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4209186687n);
    input.add64(4209186687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (4209186687, 4209186683)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4209186687n);
    input.add64(4209186683n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 1 (705656839, 18441158771086587965)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(705656839n);
    input.add64(18441158771086587965n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 2 (705656835, 705656839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(705656835n);
    input.add64(705656839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 3 (705656839, 705656839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(705656839n);
    input.add64(705656839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 4 (705656839, 705656835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(705656839n);
    input.add64(705656835n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (2068683831, 18444580827586901921)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2068683831n);
    input.add64(18444580827586901921n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (2068683827, 2068683831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2068683827n);
    input.add64(2068683831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (2068683831, 2068683831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2068683831n);
    input.add64(2068683831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (2068683831, 2068683827)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2068683831n);
    input.add64(2068683827n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 1 (479810334, 18443782316442378483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(479810334n);
    input.add64(18443782316442378483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 2 (479810330, 479810334)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(479810330n);
    input.add64(479810334n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 3 (479810334, 479810334)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(479810334n);
    input.add64(479810334n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 4 (479810334, 479810330)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(479810334n);
    input.add64(479810330n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 1 (2156872283, 18445860408658936469)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2156872283n);
    input.add64(18445860408658936469n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 2 (2156872279, 2156872283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2156872279n);
    input.add64(2156872283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 3 (2156872283, 2156872283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2156872283n);
    input.add64(2156872283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 4 (2156872283, 2156872279)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2156872283n);
    input.add64(2156872279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 1 (4196120393, 18438431658296226465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4196120393n);
    input.add64(18438431658296226465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 2 (4196120389, 4196120393)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4196120389n);
    input.add64(4196120393n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 3 (4196120393, 4196120393)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4196120393n);
    input.add64(4196120393n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 4 (4196120393, 4196120389)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4196120393n);
    input.add64(4196120389n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 1 (2035597656, 18445338238135615763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2035597656n);
    input.add64(18445338238135615763n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 2 (2035597652, 2035597656)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2035597652n);
    input.add64(2035597656n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 3 (2035597656, 2035597656)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2035597656n);
    input.add64(2035597656n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 4 (2035597656, 2035597652)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2035597656n);
    input.add64(2035597652n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 1 (549888748, 18442272904329810035)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(549888748n);
    input.add64(18442272904329810035n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(549888748n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 2 (549888744, 549888748)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(549888744n);
    input.add64(549888748n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(549888744n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 3 (549888748, 549888748)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(549888748n);
    input.add64(549888748n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(549888748n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 4 (549888748, 549888744)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(549888748n);
    input.add64(549888744n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(549888744n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (3607219253, 18437754079015064051)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3607219253n);
    input.add64(18437754079015064051n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18437754079015064051n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (3607219249, 3607219253)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3607219249n);
    input.add64(3607219253n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3607219253n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (3607219253, 3607219253)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3607219253n);
    input.add64(3607219253n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3607219253n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (3607219253, 3607219249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3607219253n);
    input.add64(3607219249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3607219253n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 1 (2, 2147483649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add128(2147483649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(2147483651n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 2 (1670316926, 1670316928)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1670316926n);
    input.add128(1670316928n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(3340633854n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 3 (1670316928, 1670316928)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1670316928n);
    input.add128(1670316928n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(3340633856n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 4 (1670316928, 1670316926)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1670316928n);
    input.add128(1670316926n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(3340633854n);
  });

  it('test operator "sub" overload (euint32, euint128) => euint128 test 1 (2666249862, 2666249862)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2666249862n);
    input.add128(2666249862n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint128) => euint128 test 2 (2666249862, 2666249858)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2666249862n);
    input.add128(2666249858n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(4n);
  });
});
