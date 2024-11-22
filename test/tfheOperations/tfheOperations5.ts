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
import { createInstances, decrypt16, decrypt32, decrypt64, decrypt128, decrypt256, decryptBool } from '../instance';
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

  it('test operator "ge" overload (euint16, euint64) => ebool test 1 (14081, 18444389415407077285)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(14081n);
    input.add64(18444389415407077285n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 2 (14077, 14081)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(14077n);
    input.add64(14081n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 3 (14081, 14081)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(14081n);
    input.add64(14081n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 4 (14081, 14077)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(14081n);
    input.add64(14077n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 1 (45007, 18445308410216818905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(45007n);
    input.add64(18445308410216818905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 2 (45003, 45007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(45003n);
    input.add64(45007n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 3 (45007, 45007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(45007n);
    input.add64(45007n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 4 (45007, 45003)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(45007n);
    input.add64(45003n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 1 (33530, 18441393339688273439)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(33530n);
    input.add64(18441393339688273439n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 2 (33526, 33530)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(33526n);
    input.add64(33530n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 3 (33530, 33530)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(33530n);
    input.add64(33530n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 4 (33530, 33526)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(33530n);
    input.add64(33526n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 1 (7192, 18440977990475901673)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(7192n);
    input.add64(18440977990475901673n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 2 (7188, 7192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(7188n);
    input.add64(7192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 3 (7192, 7192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(7192n);
    input.add64(7192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 4 (7192, 7188)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(7192n);
    input.add64(7188n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 1 (64768, 18439640713575288835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(64768n);
    input.add64(18439640713575288835n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(64768n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 2 (64764, 64768)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(64764n);
    input.add64(64768n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(64764n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 3 (64768, 64768)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(64768n);
    input.add64(64768n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(64768n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 4 (64768, 64764)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(64768n);
    input.add64(64764n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(64764n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 1 (43725, 18443087667786395005)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(43725n);
    input.add64(18443087667786395005n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18443087667786395005n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 2 (43721, 43725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(43721n);
    input.add64(43725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(43725n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 3 (43725, 43725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(43725n);
    input.add64(43725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(43725n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 4 (43725, 43721)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(43725n);
    input.add64(43721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(43725n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 1 (2, 32769)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(2n);
    input.add128(32769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(32771n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 2 (25454, 25456)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(25454n);
    input.add128(25456n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(50910n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 3 (25456, 25456)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(25456n);
    input.add128(25456n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(50912n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 4 (25456, 25454)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(25456n);
    input.add128(25454n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(50910n);
  });

  it('test operator "sub" overload (euint16, euint128) => euint128 test 1 (45485, 45485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(45485n);
    input.add128(45485n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint128) => euint128 test 2 (45485, 45481)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(45485n);
    input.add128(45481n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 1 (2, 16385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(2n);
    input.add128(16385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 2 (206, 206)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(206n);
    input.add128(206n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(42436n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 3 (206, 206)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(206n);
    input.add128(206n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(42436n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 4 (206, 206)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(206n);
    input.add128(206n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(42436n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 1 (10041, 340282366920938463463370138658990657653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(10041n);
    input.add128(340282366920938463463370138658990657653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(49n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 2 (10037, 10041)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(10037n);
    input.add128(10041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(10033n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 3 (10041, 10041)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(10041n);
    input.add128(10041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(10041n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 4 (10041, 10037)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(10041n);
    input.add128(10037n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(10033n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 1 (65101, 340282366920938463463368378388212688025)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(65101n);
    input.add128(340282366920938463463368378388212688025n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(340282366920938463463368378388212743901n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 2 (65097, 65101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(65097n);
    input.add128(65101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(65101n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 3 (65101, 65101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(65101n);
    input.add128(65101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(65101n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 4 (65101, 65097)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(65101n);
    input.add128(65097n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(65101n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 1 (57137, 340282366920938463463372706296613530555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(57137n);
    input.add128(340282366920938463463372706296613530555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(340282366920938463463372706296613549194n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 2 (57133, 57137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(57133n);
    input.add128(57137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 3 (57137, 57137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(57137n);
    input.add128(57137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 4 (57137, 57133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(57137n);
    input.add128(57133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 1 (47922, 340282366920938463463372663915711616945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(47922n);
    input.add128(340282366920938463463372663915711616945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 2 (47918, 47922)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(47918n);
    input.add128(47922n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 3 (47922, 47922)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(47922n);
    input.add128(47922n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 4 (47922, 47918)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(47922n);
    input.add128(47918n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 1 (58939, 340282366920938463463367826166753900405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(58939n);
    input.add128(340282366920938463463367826166753900405n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 2 (58935, 58939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(58935n);
    input.add128(58939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 3 (58939, 58939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(58939n);
    input.add128(58939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 4 (58939, 58935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(58939n);
    input.add128(58935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 1 (25628, 340282366920938463463367234559875664879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(25628n);
    input.add128(340282366920938463463367234559875664879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 2 (25624, 25628)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(25624n);
    input.add128(25628n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 3 (25628, 25628)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(25628n);
    input.add128(25628n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 4 (25628, 25624)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(25628n);
    input.add128(25624n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 1 (29493, 340282366920938463463370329441548615949)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(29493n);
    input.add128(340282366920938463463370329441548615949n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 2 (29489, 29493)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(29489n);
    input.add128(29493n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 3 (29493, 29493)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(29493n);
    input.add128(29493n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 4 (29493, 29489)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(29493n);
    input.add128(29489n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 1 (63634, 340282366920938463463365626120025172451)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(63634n);
    input.add128(340282366920938463463365626120025172451n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 2 (63630, 63634)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(63630n);
    input.add128(63634n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 3 (63634, 63634)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(63634n);
    input.add128(63634n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 4 (63634, 63630)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(63634n);
    input.add128(63630n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 1 (16589, 340282366920938463463371297893972390287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(16589n);
    input.add128(340282366920938463463371297893972390287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 2 (16585, 16589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(16585n);
    input.add128(16589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 3 (16589, 16589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(16589n);
    input.add128(16589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 4 (16589, 16585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(16589n);
    input.add128(16585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 1 (47213, 340282366920938463463367215507048923025)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(47213n);
    input.add128(340282366920938463463367215507048923025n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(47213n);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 2 (47209, 47213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(47209n);
    input.add128(47213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(47209n);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 3 (47213, 47213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(47213n);
    input.add128(47213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(47213n);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 4 (47213, 47209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(47213n);
    input.add128(47209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(47209n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 1 (41694, 340282366920938463463370312817461607687)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(41694n);
    input.add128(340282366920938463463370312817461607687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(340282366920938463463370312817461607687n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 2 (41690, 41694)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(41690n);
    input.add128(41694n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(41694n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 3 (41694, 41694)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(41694n);
    input.add128(41694n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(41694n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 4 (41694, 41690)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(41694n);
    input.add128(41690n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(41694n);
  });

  it('test operator "add" overload (euint16, euint256) => euint256 test 1 (2, 32769)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(2n);
    input.add256(32769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(32771n);
  });

  it('test operator "add" overload (euint16, euint256) => euint256 test 2 (20019, 20021)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(20019n);
    input.add256(20021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(40040n);
  });

  it('test operator "add" overload (euint16, euint256) => euint256 test 3 (20021, 20021)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(20021n);
    input.add256(20021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(40042n);
  });

  it('test operator "add" overload (euint16, euint256) => euint256 test 4 (20021, 20019)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(20021n);
    input.add256(20019n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(40040n);
  });

  it('test operator "sub" overload (euint16, euint256) => euint256 test 1 (40273, 40273)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(40273n);
    input.add256(40273n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint256) => euint256 test 2 (40273, 40269)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(40273n);
    input.add256(40269n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint256) => euint256 test 1 (2, 16385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(2n);
    input.add256(16385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint16, euint256) => euint256 test 2 (228, 228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(228n);
    input.add256(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(51984n);
  });

  it('test operator "mul" overload (euint16, euint256) => euint256 test 3 (228, 228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(228n);
    input.add256(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(51984n);
  });

  it('test operator "mul" overload (euint16, euint256) => euint256 test 4 (228, 228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(228n);
    input.add256(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(51984n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 1 (12579, 115792089237316195423570985008687907853269984665640564039457582458858424079051)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(12579n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582458858424079051n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(4099n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 2 (12575, 12579)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(12575n);
    input.add256(12579n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(12547n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 3 (12579, 12579)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(12579n);
    input.add256(12579n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(12579n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 4 (12579, 12575)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(12579n);
    input.add256(12575n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(12547n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 1 (31079, 115792089237316195423570985008687907853269984665640564039457575275996784888971)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(31079n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575275996784888971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575275996784917999n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 2 (31075, 31079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(31075n);
    input.add256(31079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(31079n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 3 (31079, 31079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(31079n);
    input.add256(31079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(31079n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 4 (31079, 31075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(31079n);
    input.add256(31075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(31079n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 1 (21805, 115792089237316195423570985008687907853269984665640564039457581252703610907733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(21805n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581252703610907733n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581252703610927480n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 2 (21801, 21805)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(21801n);
    input.add256(21805n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 3 (21805, 21805)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(21805n);
    input.add256(21805n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 4 (21805, 21801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(21805n);
    input.add256(21801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 1 (55138, 115792089237316195423570985008687907853269984665640564039457576200268434748919)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(55138n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576200268434748919n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 2 (55134, 55138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(55134n);
    input.add256(55138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 3 (55138, 55138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(55138n);
    input.add256(55138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 4 (55138, 55134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(55138n);
    input.add256(55134n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 1 (41071, 115792089237316195423570985008687907853269984665640564039457577958679700892209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(41071n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577958679700892209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 2 (41067, 41071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(41067n);
    input.add256(41071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 3 (41071, 41071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(41071n);
    input.add256(41071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 4 (41071, 41067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(41071n);
    input.add256(41067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint256) => ebool test 1 (50106, 115792089237316195423570985008687907853269984665640564039457582313908377810967)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(50106n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582313908377810967n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint256) => ebool test 2 (50102, 50106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(50102n);
    input.add256(50106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint256) => ebool test 3 (50106, 50106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(50106n);
    input.add256(50106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint256) => ebool test 4 (50106, 50102)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(50106n);
    input.add256(50102n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint256) => ebool test 1 (30875, 115792089237316195423570985008687907853269984665640564039457575028224206757889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(30875n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575028224206757889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint256) => ebool test 2 (30871, 30875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(30871n);
    input.add256(30875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint256) => ebool test 3 (30875, 30875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(30875n);
    input.add256(30875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint256) => ebool test 4 (30875, 30871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(30875n);
    input.add256(30871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint256) => ebool test 1 (10398, 115792089237316195423570985008687907853269984665640564039457582877023510899415)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(10398n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582877023510899415n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint256) => ebool test 2 (10394, 10398)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(10394n);
    input.add256(10398n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint256) => ebool test 3 (10398, 10398)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(10398n);
    input.add256(10398n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint256) => ebool test 4 (10398, 10394)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(10398n);
    input.add256(10394n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint256) => ebool test 1 (58145, 115792089237316195423570985008687907853269984665640564039457578303118040864491)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(58145n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578303118040864491n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint256) => ebool test 2 (58141, 58145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(58141n);
    input.add256(58145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint256) => ebool test 3 (58145, 58145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(58145n);
    input.add256(58145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint256) => ebool test 4 (58145, 58141)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(58145n);
    input.add256(58141n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint256) => euint256 test 1 (56701, 115792089237316195423570985008687907853269984665640564039457576701213568693639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(56701n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576701213568693639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(56701n);
  });

  it('test operator "min" overload (euint16, euint256) => euint256 test 2 (56697, 56701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(56697n);
    input.add256(56701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(56697n);
  });

  it('test operator "min" overload (euint16, euint256) => euint256 test 3 (56701, 56701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(56701n);
    input.add256(56701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(56701n);
  });

  it('test operator "min" overload (euint16, euint256) => euint256 test 4 (56701, 56697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(56701n);
    input.add256(56697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(56697n);
  });

  it('test operator "max" overload (euint16, euint256) => euint256 test 1 (35663, 115792089237316195423570985008687907853269984665640564039457581066242393293579)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(35663n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581066242393293579n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581066242393293579n);
  });

  it('test operator "max" overload (euint16, euint256) => euint256 test 2 (35659, 35663)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(35659n);
    input.add256(35663n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(35663n);
  });

  it('test operator "max" overload (euint16, euint256) => euint256 test 3 (35663, 35663)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(35663n);
    input.add256(35663n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(35663n);
  });

  it('test operator "max" overload (euint16, euint256) => euint256 test 4 (35663, 35659)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(35663n);
    input.add256(35659n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(35663n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (26028, 15417)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(26028n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint16_uint16(encryptedAmount.handles[0], 15417n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(41445n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (26024, 26028)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(26024n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint16_uint16(encryptedAmount.handles[0], 26028n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(52052n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (26028, 26028)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(26028n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint16_uint16(encryptedAmount.handles[0], 26028n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(52056n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (26028, 26024)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(26028n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint16_uint16(encryptedAmount.handles[0], 26024n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(52052n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (22897, 15417)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add16(15417n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_uint16_euint16(22897n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(38314n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 2 (26024, 26028)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add16(26028n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_uint16_euint16(26024n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(52052n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 3 (26028, 26028)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add16(26028n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_uint16_euint16(26028n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(52056n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 4 (26028, 26024)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add16(26024n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_uint16_euint16(26028n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(52052n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (14486, 14486)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(14486n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint16_uint16(encryptedAmount.handles[0], 14486n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 2 (14486, 14482)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(14486n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint16_uint16(encryptedAmount.handles[0], 14482n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (14486, 14486)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add16(14486n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_uint16_euint16(14486n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (14486, 14482)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add16(14482n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_uint16_euint16(14486n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (233, 168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(233n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint16_uint16(encryptedAmount.handles[0], 168n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(39144n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(233n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint16_uint16(encryptedAmount.handles[0], 233n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(54289n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(233n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint16_uint16(encryptedAmount.handles[0], 233n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(54289n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add16(233n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint16_uint16(encryptedAmount.handles[0], 233n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(54289n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (140, 168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add16(168n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_uint16_euint16(140n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(23520n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 2 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add16(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_uint16_euint16(233n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(54289n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 3 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add16(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_uint16_euint16(233n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(54289n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 4 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add16(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_uint16_euint16(233n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract4.res16());
    expect(res).to.equal(54289n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (20304, 59132)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(20304n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 59132n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (20300, 20304)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(20300n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 20304n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (20304, 20304)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(20304n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 20304n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (20304, 20300)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(20304n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 20300n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (41581, 40239)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(41581n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 40239n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(1342n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 2 (40817, 40821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(40817n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 40821n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(40817n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 3 (40821, 40821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(40821n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 40821n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 4 (40821, 40817)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(40821n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 40817n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 1 (25969, 39616)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(25969n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 39616n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(64n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 2 (25965, 25969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(25965n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 25969n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(25953n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 3 (25969, 25969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(25969n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 25969n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(25969n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 4 (25969, 25965)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(25969n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 25965n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(25953n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 1 (12551, 39616)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(39616n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(12551n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(4096n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 2 (25965, 25969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(25969n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(25965n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(25953n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 3 (25969, 25969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(25969n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(25969n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(25969n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 4 (25969, 25965)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(25965n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(25969n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(25953n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 1 (41436, 27184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(41436n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 27184n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(60412n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 2 (37563, 37567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(37563n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 37567n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(37567n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 3 (37567, 37567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(37567n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 37567n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(37567n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 4 (37567, 37563)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(37567n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 37563n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(37567n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 1 (65174, 27184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(27184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(65174n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(65206n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 2 (37563, 37567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(37567n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(37563n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(37567n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 3 (37567, 37567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(37567n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(37567n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(37567n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 4 (37567, 37563)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(37563n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(37567n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(37567n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 1 (65327, 31297)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(65327n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 31297n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(34158n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 2 (25542, 25546)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(25542n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 25546n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 3 (25546, 25546)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(25546n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 25546n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 4 (25546, 25542)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(25546n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 25542n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 1 (33651, 31297)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(31297n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(33651n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(63794n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 2 (25542, 25546)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(25546n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(25542n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 3 (25546, 25546)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(25546n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(25546n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 4 (25546, 25542)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(25542n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(25546n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (45930, 25309)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(45930n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 25309n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (43841, 43845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(43841n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 43845n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (43845, 43845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(43845n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 43845n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (43845, 43841)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(43845n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 43841n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (50532, 25309)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(25309n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint16_euint16(50532n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (43841, 43845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(43845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint16_euint16(43841n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 3 (43845, 43845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(43845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint16_euint16(43845n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 4 (43845, 43841)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(43841n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint16_euint16(43845n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (16378, 35706)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16378n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint16_uint16(encryptedAmount.handles[0], 35706n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (16374, 16378)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16374n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint16_uint16(encryptedAmount.handles[0], 16378n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 3 (16378, 16378)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16378n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint16_uint16(encryptedAmount.handles[0], 16378n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 4 (16378, 16374)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16378n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint16_uint16(encryptedAmount.handles[0], 16374n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (43595, 35706)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(35706n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint16_euint16(43595n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (16374, 16378)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(16378n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint16_euint16(16374n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 3 (16378, 16378)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(16378n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint16_euint16(16378n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 4 (16378, 16374)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(16374n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint16_euint16(16378n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (51651, 48781)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(51651n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint16_uint16(encryptedAmount.handles[0], 48781n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (13064, 13068)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(13064n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint16_uint16(encryptedAmount.handles[0], 13068n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (13068, 13068)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(13068n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint16_uint16(encryptedAmount.handles[0], 13068n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 4 (13068, 13064)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(13068n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint16_uint16(encryptedAmount.handles[0], 13064n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (60450, 48781)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(48781n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint16_euint16(60450n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (13064, 13068)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(13068n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint16_euint16(13064n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (13068, 13068)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(13068n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint16_euint16(13068n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 4 (13068, 13064)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(13064n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint16_euint16(13068n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (49523, 10749)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(49523n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint16_uint16(encryptedAmount.handles[0], 10749n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (49519, 49523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(49519n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint16_uint16(encryptedAmount.handles[0], 49523n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (49523, 49523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(49523n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint16_uint16(encryptedAmount.handles[0], 49523n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 4 (49523, 49519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(49523n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint16_uint16(encryptedAmount.handles[0], 49519n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (38042, 10749)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(10749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint16_euint16(38042n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (49519, 49523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(49523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint16_euint16(49519n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (49523, 49523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(49523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint16_euint16(49523n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 4 (49523, 49519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(49519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint16_euint16(49523n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (20803, 27442)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(20803n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint16_uint16(encryptedAmount.handles[0], 27442n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (869, 873)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(869n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint16_uint16(encryptedAmount.handles[0], 873n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (873, 873)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(873n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint16_uint16(encryptedAmount.handles[0], 873n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 4 (873, 869)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(873n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint16_uint16(encryptedAmount.handles[0], 869n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (36363, 27442)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(27442n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint16_euint16(36363n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (869, 873)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(873n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint16_euint16(869n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (873, 873)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(873n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint16_euint16(873n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (873, 869)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(869n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint16_euint16(873n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (34482, 38499)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(34482n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint16_uint16(encryptedAmount.handles[0], 38499n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (5142, 5146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(5142n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint16_uint16(encryptedAmount.handles[0], 5146n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (5146, 5146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(5146n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint16_uint16(encryptedAmount.handles[0], 5146n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 4 (5146, 5142)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(5146n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint16_uint16(encryptedAmount.handles[0], 5142n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (21462, 38499)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(38499n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint16_euint16(21462n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (5142, 5146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(5146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint16_euint16(5142n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (5146, 5146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(5146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint16_euint16(5146n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 4 (5146, 5142)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(5142n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint16_euint16(5146n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (32718, 54097)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(32718n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint16_uint16(encryptedAmount.handles[0], 54097n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(32718n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (8265, 8269)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8265n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint16_uint16(encryptedAmount.handles[0], 8269n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(8265n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (8269, 8269)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8269n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint16_uint16(encryptedAmount.handles[0], 8269n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(8269n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 4 (8269, 8265)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(8269n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint16_uint16(encryptedAmount.handles[0], 8265n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(8265n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (24240, 54097)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(54097n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint16_euint16(24240n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(24240n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (8265, 8269)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(8269n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint16_euint16(8265n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(8265n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (8269, 8269)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(8269n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint16_euint16(8269n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(8269n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (8269, 8265)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(8265n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint16_euint16(8269n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(8265n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (37775, 25405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(37775n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint16_uint16(encryptedAmount.handles[0], 25405n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(37775n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (22428, 22432)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(22428n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint16_uint16(encryptedAmount.handles[0], 22432n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(22432n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (22432, 22432)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(22432n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint16_uint16(encryptedAmount.handles[0], 22432n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(22432n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (22432, 22428)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(22432n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint16_uint16(encryptedAmount.handles[0], 22428n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(22432n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (1122, 25405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(25405n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint16_euint16(1122n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(25405n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (22428, 22432)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(22432n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint16_euint16(22428n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(22432n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (22432, 22432)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(22432n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint16_euint16(22432n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(22432n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (22432, 22428)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(22428n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint16_euint16(22432n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.res16());
    expect(res).to.equal(22432n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 1 (10, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(10n);
    input.add4(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(5n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint32, euint4) => euint32 test 1 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint4) => euint32 test 2 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 1 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(6n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 2 (3, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint32, euint4) => euint32 test 4 (5, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(5n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 1 (3337809992, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3337809992n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 1 (1671920698, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1671920698n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1671920699n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 1 (4009369108, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4009369108n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(4009369106n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 1 (1592167767, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1592167767n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 1 (2761853602, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2761853602n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(9n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(13n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint4) => ebool test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(13n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 1 (417230890, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(417230890n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 1 (3015562136, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3015562136n);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 1 (1297036148, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1297036148n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 1 (2282621038, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2282621038n);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 1 (52471261, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(52471261n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(7n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(11n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint32, euint4) => euint32 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(11n);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(7n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 1 (1089745104, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1089745104n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1089745104n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint32, euint4) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (129, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (79, 81)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(79n);
    input.add8(81n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(160n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (81, 81)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(81n);
    input.add8(81n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(162n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (81, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(81n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(160n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (69, 69)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(69n);
    input.add8(69n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (69, 65)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(69n);
    input.add8(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(4n);
  });
});
