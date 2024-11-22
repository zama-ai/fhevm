import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHETestSuite1 } from '../../types/contracts/tests/TFHETestSuite1';
import type { TFHETestSuite2 } from '../../types/contracts/tests/TFHETestSuite2';
import type { TFHETestSuite3 } from '../../types/contracts/tests/TFHETestSuite3';
import type { TFHETestSuite4 } from '../../types/contracts/tests/TFHETestSuite4';
import type { TFHETestSuite5 } from '../../types/contracts/tests/TFHETestSuite5';
import type { TFHETestSuite6 } from '../../types/contracts/tests/TFHETestSuite6';
import type { TFHETestSuite7 } from '../../types/contracts/tests/TFHETestSuite7';
import type { TFHETestSuite8 } from '../../types/contracts/tests/TFHETestSuite8';
import type { TFHETestSuite9 } from '../../types/contracts/tests/TFHETestSuite9';
import type { TFHETestSuite10 } from '../../types/contracts/tests/TFHETestSuite10';
import type { TFHETestSuite11 } from '../../types/contracts/tests/TFHETestSuite11';
import { createInstances, decrypt64, decrypt128, decrypt256, decryptBool } from '../instance';
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

async function deployTfheTestFixture8(): Promise<TFHETestSuite8> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite8');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture9(): Promise<TFHETestSuite9> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite9');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture10(): Promise<TFHETestSuite10> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite10');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture11(): Promise<TFHETestSuite11> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite11');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('TFHE operations 8', function () {
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

    const contract8 = await deployTfheTestFixture8();
    this.contract8Address = await contract8.getAddress();
    this.contract8 = contract8;

    const contract9 = await deployTfheTestFixture9();
    this.contract9Address = await contract9.getAddress();
    this.contract9 = contract9;

    const contract10 = await deployTfheTestFixture10();
    this.contract10Address = await contract10.getAddress();
    this.contract10 = contract10;

    const contract11 = await deployTfheTestFixture11();
    this.contract11Address = await contract11.getAddress();
    this.contract11 = contract11;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 1 (18443073273307378367, 1540281447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443073273307378367n);
    input.add32(1540281447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 2 (1540281443, 1540281447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1540281443n);
    input.add32(1540281447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 3 (1540281447, 1540281447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1540281447n);
    input.add32(1540281447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 4 (1540281447, 1540281443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1540281447n);
    input.add32(1540281443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 1 (18443424543399945811, 2475089443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443424543399945811n);
    input.add32(2475089443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 2 (2475089439, 2475089443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2475089439n);
    input.add32(2475089443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 3 (2475089443, 2475089443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2475089443n);
    input.add32(2475089443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 4 (2475089443, 2475089439)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2475089443n);
    input.add32(2475089439n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 1 (18439654895803536359, 1948462184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439654895803536359n);
    input.add32(1948462184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 2 (1948462180, 1948462184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1948462180n);
    input.add32(1948462184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 3 (1948462184, 1948462184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1948462184n);
    input.add32(1948462184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 4 (1948462184, 1948462180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1948462184n);
    input.add32(1948462180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 1 (18445739090666652013, 2575338339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445739090666652013n);
    input.add32(2575338339n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 2 (2575338335, 2575338339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2575338335n);
    input.add32(2575338339n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 3 (2575338339, 2575338339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2575338339n);
    input.add32(2575338339n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 4 (2575338339, 2575338335)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2575338339n);
    input.add32(2575338335n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (18442023388952183851, 1762835382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18442023388952183851n);
    input.add32(1762835382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (1762835378, 1762835382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1762835378n);
    input.add32(1762835382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (1762835382, 1762835382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1762835382n);
    input.add32(1762835382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (1762835382, 1762835378)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1762835382n);
    input.add32(1762835378n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (18441377707016357467, 2876988629)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441377707016357467n);
    input.add32(2876988629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (2876988625, 2876988629)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2876988625n);
    input.add32(2876988629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (2876988629, 2876988629)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2876988629n);
    input.add32(2876988629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (2876988629, 2876988625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2876988629n);
    input.add32(2876988625n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (18439611592084278651, 1078993829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439611592084278651n);
    input.add32(1078993829n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(1078993829n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (1078993825, 1078993829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1078993825n);
    input.add32(1078993829n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(1078993825n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (1078993829, 1078993829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1078993829n);
    input.add32(1078993829n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(1078993829n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (1078993829, 1078993825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(1078993829n);
    input.add32(1078993825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(1078993825n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (18438142521959169981, 4265760124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438142521959169981n);
    input.add32(4265760124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18438142521959169981n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (4265760120, 4265760124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4265760120n);
    input.add32(4265760124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(4265760124n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (4265760124, 4265760124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4265760124n);
    input.add32(4265760124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(4265760124n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (4265760124, 4265760120)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4265760124n);
    input.add32(4265760120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(4265760124n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (9222005500490563257, 9222323180954998973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9222005500490563257n);
    input.add64(9222323180954998973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18444328681445562230n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (9222005500490563255, 9222005500490563257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9222005500490563255n);
    input.add64(9222005500490563257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18444011000981126512n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (9222005500490563257, 9222005500490563257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9222005500490563257n);
    input.add64(9222005500490563257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18444011000981126514n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (9222005500490563257, 9222005500490563255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9222005500490563257n);
    input.add64(9222005500490563255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18444011000981126512n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (18440691834306324731, 18440691834306324731)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440691834306324731n);
    input.add64(18440691834306324731n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (18440691834306324731, 18440691834306324727)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440691834306324731n);
    input.add64(18440691834306324727n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (4294482875, 4294789499)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4294482875n);
    input.add64(4294789499n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18443899955185329625n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (4294482875, 4294482875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4294482875n);
    input.add64(4294482875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442583163668265625n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (4294482875, 4294482875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4294482875n);
    input.add64(4294482875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442583163668265625n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (4294482875, 4294482875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4294482875n);
    input.add64(4294482875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442583163668265625n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (18444124970074897317, 18442657213736743287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444124970074897317n);
    input.add64(18442657213736743287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442294351260348709n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (18442657213736743283, 18442657213736743287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18442657213736743283n);
    input.add64(18442657213736743287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442657213736743283n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (18442657213736743287, 18442657213736743287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18442657213736743287n);
    input.add64(18442657213736743287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442657213736743287n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (18442657213736743287, 18442657213736743283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18442657213736743287n);
    input.add64(18442657213736743283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442657213736743283n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 1 (18445851190683093315, 18440297605680046453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445851190683093315n);
    input.add64(18440297605680046453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18446141532670781815n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 2 (18440297605680046449, 18440297605680046453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440297605680046449n);
    input.add64(18440297605680046453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18440297605680046453n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 3 (18440297605680046453, 18440297605680046453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440297605680046453n);
    input.add64(18440297605680046453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18440297605680046453n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 4 (18440297605680046453, 18440297605680046449)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440297605680046453n);
    input.add64(18440297605680046449n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18440297605680046453n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 1 (18439520894560746701, 18440669858368892063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439520894560746701n);
    input.add64(18440669858368892063n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(3444795783188562n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 2 (18439520894560746697, 18439520894560746701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439520894560746697n);
    input.add64(18439520894560746701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 3 (18439520894560746701, 18439520894560746701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439520894560746701n);
    input.add64(18439520894560746701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 4 (18439520894560746701, 18439520894560746697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439520894560746701n);
    input.add64(18439520894560746697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 1 (18443282870504090991, 18446028700927461615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443282870504090991n);
    input.add64(18446028700927461615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 2 (18443282870504090987, 18443282870504090991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443282870504090987n);
    input.add64(18443282870504090991n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 3 (18443282870504090991, 18443282870504090991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443282870504090991n);
    input.add64(18443282870504090991n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 4 (18443282870504090991, 18443282870504090987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443282870504090991n);
    input.add64(18443282870504090987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (18445289136242885897, 18441860560804132295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445289136242885897n);
    input.add64(18441860560804132295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (18441860560804132291, 18441860560804132295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441860560804132291n);
    input.add64(18441860560804132295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (18441860560804132295, 18441860560804132295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441860560804132295n);
    input.add64(18441860560804132295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (18441860560804132295, 18441860560804132291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441860560804132295n);
    input.add64(18441860560804132291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 1 (18444613618003912273, 18441817383047397973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444613618003912273n);
    input.add64(18441817383047397973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 2 (18441817383047397969, 18441817383047397973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441817383047397969n);
    input.add64(18441817383047397973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 3 (18441817383047397973, 18441817383047397973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441817383047397973n);
    input.add64(18441817383047397973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 4 (18441817383047397973, 18441817383047397969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441817383047397973n);
    input.add64(18441817383047397969n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (18444129545450155179, 18438304556640407461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444129545450155179n);
    input.add64(18438304556640407461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (18438304556640407457, 18438304556640407461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438304556640407457n);
    input.add64(18438304556640407461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (18438304556640407461, 18438304556640407461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438304556640407461n);
    input.add64(18438304556640407461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (18438304556640407461, 18438304556640407457)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438304556640407461n);
    input.add64(18438304556640407457n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (18440402951089431437, 18444684457474043493)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440402951089431437n);
    input.add64(18444684457474043493n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (18440402951089431433, 18440402951089431437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440402951089431433n);
    input.add64(18440402951089431437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (18440402951089431437, 18440402951089431437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440402951089431437n);
    input.add64(18440402951089431437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (18440402951089431437, 18440402951089431433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440402951089431437n);
    input.add64(18440402951089431433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 1 (18443308354694945505, 18438494040818835505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443308354694945505n);
    input.add64(18438494040818835505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 2 (18438494040818835501, 18438494040818835505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438494040818835501n);
    input.add64(18438494040818835505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 3 (18438494040818835505, 18438494040818835505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438494040818835505n);
    input.add64(18438494040818835505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 4 (18438494040818835505, 18438494040818835501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438494040818835505n);
    input.add64(18438494040818835501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 1 (18438422972288340001, 18441615262395055283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438422972288340001n);
    input.add64(18441615262395055283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18438422972288340001n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 2 (18438422972288339997, 18438422972288340001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438422972288339997n);
    input.add64(18438422972288340001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18438422972288339997n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 3 (18438422972288340001, 18438422972288340001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438422972288340001n);
    input.add64(18438422972288340001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18438422972288340001n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 4 (18438422972288340001, 18438422972288339997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438422972288340001n);
    input.add64(18438422972288339997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18438422972288339997n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (18445852065991763771, 18440937538047974893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445852065991763771n);
    input.add64(18440937538047974893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18445852065991763771n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (18440937538047974889, 18440937538047974893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440937538047974889n);
    input.add64(18440937538047974893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18440937538047974893n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (18440937538047974893, 18440937538047974893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440937538047974893n);
    input.add64(18440937538047974893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18440937538047974893n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (18440937538047974893, 18440937538047974889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440937538047974893n);
    input.add64(18440937538047974889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18440937538047974893n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 1 (2, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 2 (9220019285142748118, 9220019285142748120)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9220019285142748118n);
    input.add128(9220019285142748120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18440038570285496238n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 3 (9220019285142748120, 9220019285142748120)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9220019285142748120n);
    input.add128(9220019285142748120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18440038570285496240n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 4 (9220019285142748120, 9220019285142748118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9220019285142748120n);
    input.add128(9220019285142748118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18440038570285496238n);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 1 (18445847707614326699, 18445847707614326699)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445847707614326699n);
    input.add128(18445847707614326699n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 2 (18445847707614326699, 18445847707614326695)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445847707614326699n);
    input.add128(18445847707614326695n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 1 (2, 4611686018427387905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2n);
    input.add128(4611686018427387905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 2 (4293652736, 4293652736)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4293652736n);
    input.add128(4293652736n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18435453817360285696n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 3 (4293652736, 4293652736)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4293652736n);
    input.add128(4293652736n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18435453817360285696n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 4 (4293652736, 4293652736)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4293652736n);
    input.add128(4293652736n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18435453817360285696n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 1 (18438964653643998977, 340282366920938463463371580179294281733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438964653643998977n);
    input.add128(340282366920938463463371580179294281733n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18438894284827860993n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 2 (18438964653643998973, 18438964653643998977)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438964653643998973n);
    input.add128(18438964653643998977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18438964653643998721n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 3 (18438964653643998977, 18438964653643998977)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438964653643998977n);
    input.add128(18438964653643998977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18438964653643998977n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 4 (18438964653643998977, 18438964653643998973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438964653643998977n);
    input.add128(18438964653643998973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18438964653643998721n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 1 (18444253937643069461, 340282366920938463463369573351070308735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444253937643069461n);
    input.add128(340282366920938463463369573351070308735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463374396049659246975n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 2 (18444253937643069457, 18444253937643069461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444253937643069457n);
    input.add128(18444253937643069461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18444253937643069461n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 3 (18444253937643069461, 18444253937643069461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444253937643069461n);
    input.add128(18444253937643069461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18444253937643069461n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 4 (18444253937643069461, 18444253937643069457)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444253937643069461n);
    input.add128(18444253937643069457n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18444253937643069461n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 1 (18445744178466976977, 340282366920938463463371486789808692655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445744178466976977n);
    input.add128(340282366920938463463371486789808692655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463444930285940062611838n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 2 (18445744178466976973, 18445744178466976977)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445744178466976973n);
    input.add128(18445744178466976977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 3 (18445744178466976977, 18445744178466976977)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445744178466976977n);
    input.add128(18445744178466976977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 4 (18445744178466976977, 18445744178466976973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445744178466976977n);
    input.add128(18445744178466976973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 1 (18440180042915486669, 340282366920938463463371210059252201113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440180042915486669n);
    input.add128(340282366920938463463371210059252201113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 2 (18440180042915486665, 18440180042915486669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440180042915486665n);
    input.add128(18440180042915486669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 3 (18440180042915486669, 18440180042915486669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440180042915486669n);
    input.add128(18440180042915486669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 4 (18440180042915486669, 18440180042915486665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440180042915486669n);
    input.add128(18440180042915486665n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 1 (18439484097328106945, 340282366920938463463374251414120336649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439484097328106945n);
    input.add128(340282366920938463463374251414120336649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 2 (18439484097328106941, 18439484097328106945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439484097328106941n);
    input.add128(18439484097328106945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 3 (18439484097328106945, 18439484097328106945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439484097328106945n);
    input.add128(18439484097328106945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 4 (18439484097328106945, 18439484097328106941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439484097328106945n);
    input.add128(18439484097328106941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 1 (18442500358670120549, 340282366920938463463373603431929658391)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18442500358670120549n);
    input.add128(340282366920938463463373603431929658391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 2 (18442500358670120545, 18442500358670120549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18442500358670120545n);
    input.add128(18442500358670120549n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 3 (18442500358670120549, 18442500358670120549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18442500358670120549n);
    input.add128(18442500358670120549n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 4 (18442500358670120549, 18442500358670120545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18442500358670120549n);
    input.add128(18442500358670120545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 1 (18440808032965074109, 340282366920938463463365853534758019559)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440808032965074109n);
    input.add128(340282366920938463463365853534758019559n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 2 (18440808032965074105, 18440808032965074109)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440808032965074105n);
    input.add128(18440808032965074109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 3 (18440808032965074109, 18440808032965074109)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440808032965074109n);
    input.add128(18440808032965074109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 4 (18440808032965074109, 18440808032965074105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440808032965074109n);
    input.add128(18440808032965074105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 1 (18443662339869468477, 340282366920938463463371042076884293495)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443662339869468477n);
    input.add128(340282366920938463463371042076884293495n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 2 (18443662339869468473, 18443662339869468477)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443662339869468473n);
    input.add128(18443662339869468477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 3 (18443662339869468477, 18443662339869468477)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443662339869468477n);
    input.add128(18443662339869468477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 4 (18443662339869468477, 18443662339869468473)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443662339869468477n);
    input.add128(18443662339869468473n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 1 (18444814117403628673, 340282366920938463463365637360413874149)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444814117403628673n);
    input.add128(340282366920938463463365637360413874149n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 2 (18444814117403628669, 18444814117403628673)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444814117403628669n);
    input.add128(18444814117403628673n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 3 (18444814117403628673, 18444814117403628673)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444814117403628673n);
    input.add128(18444814117403628673n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 4 (18444814117403628673, 18444814117403628669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444814117403628673n);
    input.add128(18444814117403628669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 1 (18444900314492723695, 340282366920938463463367540489775579127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444900314492723695n);
    input.add128(340282366920938463463367540489775579127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18444900314492723695n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 2 (18444900314492723691, 18444900314492723695)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444900314492723691n);
    input.add128(18444900314492723695n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18444900314492723691n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 3 (18444900314492723695, 18444900314492723695)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444900314492723695n);
    input.add128(18444900314492723695n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18444900314492723695n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 4 (18444900314492723695, 18444900314492723691)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444900314492723695n);
    input.add128(18444900314492723691n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18444900314492723691n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 1 (18444589551829336117, 340282366920938463463371310064926977589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444589551829336117n);
    input.add128(340282366920938463463371310064926977589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463371310064926977589n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 2 (18444589551829336113, 18444589551829336117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444589551829336113n);
    input.add128(18444589551829336117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18444589551829336117n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 3 (18444589551829336117, 18444589551829336117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444589551829336117n);
    input.add128(18444589551829336117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18444589551829336117n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 4 (18444589551829336117, 18444589551829336113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444589551829336117n);
    input.add128(18444589551829336113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(18444589551829336117n);
  });

  it('test operator "add" overload (euint64, euint256) => euint256 test 1 (2, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2n);
    input.add256(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint64, euint256) => euint256 test 2 (9223002406300982591, 9223002406300982593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9223002406300982591n);
    input.add256(9223002406300982593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18446004812601965184n);
  });

  it('test operator "add" overload (euint64, euint256) => euint256 test 3 (9223002406300982593, 9223002406300982593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9223002406300982593n);
    input.add256(9223002406300982593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18446004812601965186n);
  });

  it('test operator "add" overload (euint64, euint256) => euint256 test 4 (9223002406300982593, 9223002406300982591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9223002406300982593n);
    input.add256(9223002406300982591n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18446004812601965184n);
  });

  it('test operator "sub" overload (euint64, euint256) => euint256 test 1 (18445224051732608013, 18445224051732608013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445224051732608013n);
    input.add256(18445224051732608013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint256) => euint256 test 2 (18445224051732608013, 18445224051732608009)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445224051732608013n);
    input.add256(18445224051732608009n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint256) => euint256 test 1 (2, 4611686018427387905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2n);
    input.add256(4611686018427387905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint64, euint256) => euint256 test 2 (4292932541, 4292932541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4292932541n);
    input.add256(4292932541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18429269801576716681n);
  });

  it('test operator "mul" overload (euint64, euint256) => euint256 test 3 (4292932541, 4292932541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4292932541n);
    input.add256(4292932541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18429269801576716681n);
  });

  it('test operator "mul" overload (euint64, euint256) => euint256 test 4 (4292932541, 4292932541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4292932541n);
    input.add256(4292932541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18429269801576716681n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 1 (18443960605514453875, 115792089237316195423570985008687907853269984665640564039457575145704576396421)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443960605514453875n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575145704576396421n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18437737227820728321n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 2 (18443960605514453871, 18443960605514453875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443960605514453871n);
    input.add256(18443960605514453875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18443960605514453859n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 3 (18443960605514453875, 18443960605514453875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443960605514453875n);
    input.add256(18443960605514453875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18443960605514453875n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 4 (18443960605514453875, 18443960605514453871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443960605514453875n);
    input.add256(18443960605514453871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18443960605514453859n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 1 (18444891827197945405, 115792089237316195423570985008687907853269984665640564039457581376324451236725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444891827197945405n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581376324451236725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583990276981260157n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 2 (18444891827197945401, 18444891827197945405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444891827197945401n);
    input.add256(18444891827197945405n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18444891827197945405n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 3 (18444891827197945405, 18444891827197945405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444891827197945405n);
    input.add256(18444891827197945405n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18444891827197945405n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 4 (18444891827197945405, 18444891827197945401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444891827197945405n);
    input.add256(18444891827197945401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18444891827197945405n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 1 (18438921176678700429, 115792089237316195423570985008687907853269984665640564039457575984328616481271)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438921176678700429n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575984328616481271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039439139317222450136186n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 2 (18438921176678700425, 18438921176678700429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438921176678700425n);
    input.add256(18438921176678700429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 3 (18438921176678700429, 18438921176678700429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438921176678700429n);
    input.add256(18438921176678700429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 4 (18438921176678700429, 18438921176678700425)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438921176678700429n);
    input.add256(18438921176678700425n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 1 (18443851863635995939, 115792089237316195423570985008687907853269984665640564039457575030888379042775)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443851863635995939n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575030888379042775n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 2 (18443851863635995935, 18443851863635995939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443851863635995935n);
    input.add256(18443851863635995939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 3 (18443851863635995939, 18443851863635995939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443851863635995939n);
    input.add256(18443851863635995939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 4 (18443851863635995939, 18443851863635995935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443851863635995939n);
    input.add256(18443851863635995935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 1 (18446211817960475589, 115792089237316195423570985008687907853269984665640564039457576433191959532665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18446211817960475589n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576433191959532665n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 2 (18446211817960475585, 18446211817960475589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18446211817960475585n);
    input.add256(18446211817960475589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 3 (18446211817960475589, 18446211817960475589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18446211817960475589n);
    input.add256(18446211817960475589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 4 (18446211817960475589, 18446211817960475585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18446211817960475589n);
    input.add256(18446211817960475585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint256) => ebool test 1 (18444770112796649545, 115792089237316195423570985008687907853269984665640564039457581281801379635505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444770112796649545n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581281801379635505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint256) => ebool test 2 (18444770112796649541, 18444770112796649545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444770112796649541n);
    input.add256(18444770112796649545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint256) => ebool test 3 (18444770112796649545, 18444770112796649545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444770112796649545n);
    input.add256(18444770112796649545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint256) => ebool test 4 (18444770112796649545, 18444770112796649541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444770112796649545n);
    input.add256(18444770112796649541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint256) => ebool test 1 (18443327594956328675, 115792089237316195423570985008687907853269984665640564039457581268418001741927)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443327594956328675n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581268418001741927n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint256) => ebool test 2 (18443327594956328671, 18443327594956328675)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443327594956328671n);
    input.add256(18443327594956328675n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint256) => ebool test 3 (18443327594956328675, 18443327594956328675)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443327594956328675n);
    input.add256(18443327594956328675n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint256) => ebool test 4 (18443327594956328675, 18443327594956328671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443327594956328675n);
    input.add256(18443327594956328671n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint256) => ebool test 1 (18443803742985479181, 115792089237316195423570985008687907853269984665640564039457581800529684723115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443803742985479181n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581800529684723115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint256) => ebool test 2 (18443803742985479177, 18443803742985479181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443803742985479177n);
    input.add256(18443803742985479181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint256) => ebool test 3 (18443803742985479181, 18443803742985479181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443803742985479181n);
    input.add256(18443803742985479181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint256) => ebool test 4 (18443803742985479181, 18443803742985479177)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443803742985479181n);
    input.add256(18443803742985479177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint256) => ebool test 1 (18440844601726997069, 115792089237316195423570985008687907853269984665640564039457578884493044868087)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440844601726997069n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578884493044868087n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint256) => ebool test 2 (18440844601726997065, 18440844601726997069)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440844601726997065n);
    input.add256(18440844601726997069n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint256) => ebool test 3 (18440844601726997069, 18440844601726997069)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440844601726997069n);
    input.add256(18440844601726997069n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint256) => ebool test 4 (18440844601726997069, 18440844601726997065)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440844601726997069n);
    input.add256(18440844601726997065n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint256) => euint256 test 1 (18446426094775234589, 115792089237316195423570985008687907853269984665640564039457578688132986416989)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18446426094775234589n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578688132986416989n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18446426094775234589n);
  });

  it('test operator "min" overload (euint64, euint256) => euint256 test 2 (18446426094775234585, 18446426094775234589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18446426094775234585n);
    input.add256(18446426094775234589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18446426094775234585n);
  });

  it('test operator "min" overload (euint64, euint256) => euint256 test 3 (18446426094775234589, 18446426094775234589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18446426094775234589n);
    input.add256(18446426094775234589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18446426094775234589n);
  });

  it('test operator "min" overload (euint64, euint256) => euint256 test 4 (18446426094775234589, 18446426094775234585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18446426094775234589n);
    input.add256(18446426094775234585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18446426094775234585n);
  });

  it('test operator "max" overload (euint64, euint256) => euint256 test 1 (18437962634473522913, 115792089237316195423570985008687907853269984665640564039457582464243297466475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18437962634473522913n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582464243297466475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582464243297466475n);
  });

  it('test operator "max" overload (euint64, euint256) => euint256 test 2 (18437962634473522909, 18437962634473522913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18437962634473522909n);
    input.add256(18437962634473522913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18437962634473522913n);
  });

  it('test operator "max" overload (euint64, euint256) => euint256 test 3 (18437962634473522913, 18437962634473522913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18437962634473522913n);
    input.add256(18437962634473522913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18437962634473522913n);
  });

  it('test operator "max" overload (euint64, euint256) => euint256 test 4 (18437962634473522913, 18437962634473522909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18437962634473522913n);
    input.add256(18437962634473522909n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18437962634473522913n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (9222005500490563257, 9221821229831123671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9222005500490563257n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_uint64(
      encryptedAmount.handles[0],
      9221821229831123671n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18443826730321686928n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (9222005500490563255, 9222005500490563257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9222005500490563255n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_uint64(
      encryptedAmount.handles[0],
      9222005500490563257n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18444011000981126512n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (9222005500490563257, 9222005500490563257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9222005500490563257n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_uint64(
      encryptedAmount.handles[0],
      9222005500490563257n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18444011000981126514n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (9222005500490563257, 9222005500490563255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(9222005500490563257n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_uint64(
      encryptedAmount.handles[0],
      9222005500490563255n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18444011000981126512n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 1 (9221960957990697049, 9221821229831123671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(9221821229831123671n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_uint64_euint64(
      9221960957990697049n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18443782187821820720n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 2 (9222005500490563255, 9222005500490563257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(9222005500490563257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_uint64_euint64(
      9222005500490563255n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18444011000981126512n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 3 (9222005500490563257, 9222005500490563257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(9222005500490563257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_uint64_euint64(
      9222005500490563257n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18444011000981126514n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 4 (9222005500490563257, 9222005500490563255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(9222005500490563255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_uint64_euint64(
      9222005500490563257n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18444011000981126512n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (18440691834306324731, 18440691834306324731)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440691834306324731n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18440691834306324731n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (18440691834306324731, 18440691834306324727)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440691834306324731n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18440691834306324727n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (18440691834306324731, 18440691834306324731)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18440691834306324731n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_uint64_euint64(
      18440691834306324731n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (18440691834306324731, 18440691834306324727)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18440691834306324727n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_uint64_euint64(
      18440691834306324731n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (4294482875, 4294108167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4294482875n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294108167n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18440973986579140125n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (4294482875, 4294482875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4294482875n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294482875n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442583163668265625n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (4294482875, 4294482875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4294482875n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294482875n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442583163668265625n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (4294482875, 4294482875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4294482875n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294482875n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442583163668265625n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 1 (4293014414, 4294108167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(4294108167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_uint64_euint64(
      4293014414n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18434668256206119138n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 2 (4294482875, 4294482875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(4294482875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_uint64_euint64(
      4294482875n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442583163668265625n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 3 (4294482875, 4294482875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(4294482875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_uint64_euint64(
      4294482875n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442583163668265625n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 4 (4294482875, 4294482875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(4294482875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_uint64_euint64(
      4294482875n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442583163668265625n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (18444804438871636581, 18440669978814863107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444804438871636581n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.div_euint64_uint64(
      encryptedAmount.handles[0],
      18440669978814863107n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (18444804438871636577, 18444804438871636581)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444804438871636577n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.div_euint64_uint64(
      encryptedAmount.handles[0],
      18444804438871636581n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (18444804438871636581, 18444804438871636581)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444804438871636581n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.div_euint64_uint64(
      encryptedAmount.handles[0],
      18444804438871636581n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (18444804438871636581, 18444804438871636577)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444804438871636581n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.div_euint64_uint64(
      encryptedAmount.handles[0],
      18444804438871636577n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 1 (18446008186519174615, 18445049964172995155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18446008186519174615n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18445049964172995155n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(958222346179460n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 2 (18443920463081385689, 18443920463081385693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443920463081385689n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18443920463081385693n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18443920463081385689n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 3 (18443920463081385693, 18443920463081385693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443920463081385693n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18443920463081385693n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 4 (18443920463081385693, 18443920463081385689)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443920463081385693n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18443920463081385689n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 1 (18444124970074897317, 18444452212053822561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444124970074897317n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_uint64(
      encryptedAmount.handles[0],
      18444452212053822561n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18444089373216524321n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 2 (18442657213736743283, 18442657213736743287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18442657213736743283n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_uint64(
      encryptedAmount.handles[0],
      18442657213736743287n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442657213736743283n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 3 (18442657213736743287, 18442657213736743287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18442657213736743287n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_uint64(
      encryptedAmount.handles[0],
      18442657213736743287n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442657213736743287n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 4 (18442657213736743287, 18442657213736743283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18442657213736743287n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint64_uint64(
      encryptedAmount.handles[0],
      18442657213736743283n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442657213736743283n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 1 (18445812436048048525, 18444452212053822561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18444452212053822561n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint64_euint64(
      18445812436048048525n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18443525253958221825n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 2 (18442657213736743283, 18442657213736743287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18442657213736743287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint64_euint64(
      18442657213736743283n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442657213736743283n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 3 (18442657213736743287, 18442657213736743287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18442657213736743287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint64_euint64(
      18442657213736743287n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442657213736743287n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 4 (18442657213736743287, 18442657213736743283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18442657213736743283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint64_euint64(
      18442657213736743287n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18442657213736743283n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 1 (18445851190683093315, 18441158928097514419)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445851190683093315n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_uint64(
      encryptedAmount.handles[0],
      18441158928097514419n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18445895250664217587n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 2 (18440297605680046449, 18440297605680046453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440297605680046449n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_uint64(
      encryptedAmount.handles[0],
      18440297605680046453n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18440297605680046453n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 3 (18440297605680046453, 18440297605680046453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440297605680046453n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_uint64(
      encryptedAmount.handles[0],
      18440297605680046453n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18440297605680046453n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 4 (18440297605680046453, 18440297605680046449)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18440297605680046453n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint64_uint64(
      encryptedAmount.handles[0],
      18440297605680046449n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18440297605680046453n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 1 (18444816838000851275, 18441158928097514419)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18441158928097514419n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint64_euint64(
      18444816838000851275n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18445951836858858491n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 2 (18440297605680046449, 18440297605680046453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18440297605680046453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint64_euint64(
      18440297605680046449n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18440297605680046453n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 3 (18440297605680046453, 18440297605680046453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18440297605680046453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint64_euint64(
      18440297605680046453n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18440297605680046453n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 4 (18440297605680046453, 18440297605680046449)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18440297605680046449n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint64_euint64(
      18440297605680046453n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(18440297605680046453n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 1 (18439520894560746701, 18445914762272407037)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439520894560746701n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18445914762272407037n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(7700087946789168n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 2 (18439520894560746697, 18439520894560746701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439520894560746697n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18439520894560746701n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 3 (18439520894560746701, 18439520894560746701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439520894560746701n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18439520894560746701n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 4 (18439520894560746701, 18439520894560746697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18439520894560746701n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18439520894560746697n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 1 (18446426713533628951, 18445914762272407037)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18445914762272407037n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint64_euint64(
      18446426713533628951n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(1076293857951722n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 2 (18439520894560746697, 18439520894560746701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18439520894560746701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint64_euint64(
      18439520894560746697n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 3 (18439520894560746701, 18439520894560746701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18439520894560746701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint64_euint64(
      18439520894560746701n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 4 (18439520894560746701, 18439520894560746697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18439520894560746697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint64_euint64(
      18439520894560746701n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract7.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (18443282870504090991, 18439130415590833625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443282870504090991n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18439130415590833625n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (18443282870504090987, 18443282870504090991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443282870504090987n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18443282870504090991n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (18443282870504090991, 18443282870504090991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443282870504090991n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18443282870504090991n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (18443282870504090991, 18443282870504090987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443282870504090991n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18443282870504090987n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (18442875587397628573, 18439130415590833625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18439130415590833625n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint64_euint64(
      18442875587397628573n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (18443282870504090987, 18443282870504090991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18443282870504090991n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint64_euint64(
      18443282870504090987n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (18443282870504090991, 18443282870504090991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18443282870504090991n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint64_euint64(
      18443282870504090991n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (18443282870504090991, 18443282870504090987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18443282870504090987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint64_euint64(
      18443282870504090991n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (18445289136242885897, 18440743017590143725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445289136242885897n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18440743017590143725n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (18441860560804132291, 18441860560804132295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441860560804132291n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18441860560804132295n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (18441860560804132295, 18441860560804132295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441860560804132295n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18441860560804132295n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (18441860560804132295, 18441860560804132291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441860560804132295n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18441860560804132291n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (18442171622930650065, 18440743017590143725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18440743017590143725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint64_euint64(
      18442171622930650065n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (18441860560804132291, 18441860560804132295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18441860560804132295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint64_euint64(
      18441860560804132291n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (18441860560804132295, 18441860560804132295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18441860560804132295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint64_euint64(
      18441860560804132295n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (18441860560804132295, 18441860560804132291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18441860560804132291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint64_euint64(
      18441860560804132295n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (18444613618003912273, 18444784193665775113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444613618003912273n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18444784193665775113n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (18441817383047397969, 18441817383047397973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441817383047397969n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18441817383047397973n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (18441817383047397973, 18441817383047397973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441817383047397973n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18441817383047397973n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (18441817383047397973, 18441817383047397969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18441817383047397973n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18441817383047397969n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (18439321097396363619, 18444784193665775113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18444784193665775113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint64_euint64(
      18439321097396363619n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (18441817383047397969, 18441817383047397973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18441817383047397973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint64_euint64(
      18441817383047397969n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (18441817383047397973, 18441817383047397973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18441817383047397973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint64_euint64(
      18441817383047397973n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (18441817383047397973, 18441817383047397969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add64(18441817383047397969n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint64_euint64(
      18441817383047397973n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (18444129545450155179, 18441052996137337147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18444129545450155179n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18441052996137337147n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (18438304556640407457, 18438304556640407461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18438304556640407457n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18438304556640407461n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (18438304556640407461, 18438304556640407461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18438304556640407461n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18438304556640407461n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (18438304556640407461, 18438304556640407457)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18438304556640407461n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18438304556640407457n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 1 (18440314139404889003, 18441052996137337147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18441052996137337147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_uint64_euint64(
      18440314139404889003n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 2 (18438304556640407457, 18438304556640407461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18438304556640407461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_uint64_euint64(
      18438304556640407457n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 3 (18438304556640407461, 18438304556640407461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18438304556640407461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_uint64_euint64(
      18438304556640407461n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 4 (18438304556640407461, 18438304556640407457)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18438304556640407457n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_uint64_euint64(
      18438304556640407461n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (18440402951089431437, 18443808705297812893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18440402951089431437n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint64_uint64(
      encryptedAmount.handles[0],
      18443808705297812893n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (18440402951089431433, 18440402951089431437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18440402951089431433n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint64_uint64(
      encryptedAmount.handles[0],
      18440402951089431437n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (18440402951089431437, 18440402951089431437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18440402951089431437n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint64_uint64(
      encryptedAmount.handles[0],
      18440402951089431437n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (18440402951089431437, 18440402951089431433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18440402951089431437n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint64_uint64(
      encryptedAmount.handles[0],
      18440402951089431433n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (18443941229431137849, 18443808705297812893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18443808705297812893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_uint64_euint64(
      18443941229431137849n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (18440402951089431433, 18440402951089431437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18440402951089431437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_uint64_euint64(
      18440402951089431433n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (18440402951089431437, 18440402951089431437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18440402951089431437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_uint64_euint64(
      18440402951089431437n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (18440402951089431437, 18440402951089431433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18440402951089431433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_uint64_euint64(
      18440402951089431437n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (18443308354694945505, 18438426856220799235)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18443308354694945505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18438426856220799235n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (18438494040818835501, 18438494040818835505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18438494040818835501n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18438494040818835505n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (18438494040818835505, 18438494040818835505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18438494040818835505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18438494040818835505n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (18438494040818835505, 18438494040818835501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18438494040818835505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18438494040818835501n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (18444682732892858227, 18438426856220799235)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18438426856220799235n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_uint64_euint64(
      18444682732892858227n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (18438494040818835501, 18438494040818835505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18438494040818835505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_uint64_euint64(
      18438494040818835501n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (18438494040818835505, 18438494040818835505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18438494040818835505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_uint64_euint64(
      18438494040818835505n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (18438494040818835505, 18438494040818835501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18438494040818835501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_uint64_euint64(
      18438494040818835505n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 1 (18438422972288340001, 18439986643027034037)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18438422972288340001n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint64_uint64(
      encryptedAmount.handles[0],
      18439986643027034037n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18438422972288340001n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 2 (18438422972288339997, 18438422972288340001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18438422972288339997n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint64_uint64(
      encryptedAmount.handles[0],
      18438422972288340001n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18438422972288339997n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 3 (18438422972288340001, 18438422972288340001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18438422972288340001n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint64_uint64(
      encryptedAmount.handles[0],
      18438422972288340001n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18438422972288340001n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 4 (18438422972288340001, 18438422972288339997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18438422972288340001n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint64_uint64(
      encryptedAmount.handles[0],
      18438422972288339997n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18438422972288339997n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (18443338893508802671, 18439986643027034037)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18439986643027034037n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_uint64_euint64(
      18443338893508802671n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18439986643027034037n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (18438422972288339997, 18438422972288340001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18438422972288340001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_uint64_euint64(
      18438422972288339997n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18438422972288339997n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (18438422972288340001, 18438422972288340001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18438422972288340001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_uint64_euint64(
      18438422972288340001n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18438422972288340001n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (18438422972288340001, 18438422972288339997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18438422972288339997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_uint64_euint64(
      18438422972288340001n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18438422972288339997n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (18445852065991763771, 18440623444591001569)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18445852065991763771n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint64_uint64(
      encryptedAmount.handles[0],
      18440623444591001569n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18445852065991763771n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (18440937538047974889, 18440937538047974893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18440937538047974889n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint64_uint64(
      encryptedAmount.handles[0],
      18440937538047974893n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18440937538047974893n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (18440937538047974893, 18440937538047974893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18440937538047974893n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint64_uint64(
      encryptedAmount.handles[0],
      18440937538047974893n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18440937538047974893n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (18440937538047974893, 18440937538047974889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18440937538047974893n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint64_uint64(
      encryptedAmount.handles[0],
      18440937538047974889n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18440937538047974893n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (18444013465281910433, 18440623444591001569)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18440623444591001569n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_uint64_euint64(
      18444013465281910433n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18444013465281910433n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18440937538047974889, 18440937538047974893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18440937538047974893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_uint64_euint64(
      18440937538047974889n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18440937538047974893n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18440937538047974893, 18440937538047974893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18440937538047974893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_uint64_euint64(
      18440937538047974893n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18440937538047974893n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18440937538047974893, 18440937538047974889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add64(18440937538047974889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_uint64_euint64(
      18440937538047974893n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18440937538047974893n);
  });
});
