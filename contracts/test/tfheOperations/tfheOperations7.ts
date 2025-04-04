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

describe('TFHE operations 7', function () {
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

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 1 (2146966221, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2146966221n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4293932442n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 2 (59588, 59588)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(59588n);
    input.add32(59588n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(3550729744n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 3 (59588, 59588)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(59588n);
    input.add32(59588n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(3550729744n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 4 (59588, 59588)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(59588n);
    input.add32(59588n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(3550729744n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 1 (18442706417918331985, 2859978597)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442706417918331985n);
    input.add32(2859978597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(5688385n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 2 (2859978593, 2859978597)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2859978593n);
    input.add32(2859978597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(2859978593n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 3 (2859978597, 2859978597)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2859978597n);
    input.add32(2859978597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(2859978597n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 4 (2859978597, 2859978593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2859978597n);
    input.add32(2859978593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(2859978593n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 1 (18441396808997578683, 2708085028)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441396808997578683n);
    input.add32(2708085028n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18441396809016983487n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 2 (2708085024, 2708085028)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2708085024n);
    input.add32(2708085028n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(2708085028n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 3 (2708085028, 2708085028)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2708085028n);
    input.add32(2708085028n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(2708085028n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 4 (2708085028, 2708085024)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2708085028n);
    input.add32(2708085024n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(2708085028n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 1 (18444462586984911097, 490410607)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444462586984911097n);
    input.add32(490410607n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18444462586663322262n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 2 (490410603, 490410607)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(490410603n);
    input.add32(490410607n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 3 (490410607, 490410607)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(490410607n);
    input.add32(490410607n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 4 (490410607, 490410603)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(490410607n);
    input.add32(490410603n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 1 (18441607370878891769, 4149130990)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441607370878891769n);
    input.add32(4149130990n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 2 (4149130986, 4149130990)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4149130986n);
    input.add32(4149130990n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 3 (4149130990, 4149130990)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4149130990n);
    input.add32(4149130990n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 4 (4149130990, 4149130986)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4149130990n);
    input.add32(4149130986n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 1 (18443971655667612065, 3464232054)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443971655667612065n);
    input.add32(3464232054n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 2 (3464232050, 3464232054)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(3464232050n);
    input.add32(3464232054n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 3 (3464232054, 3464232054)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(3464232054n);
    input.add32(3464232054n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 4 (3464232054, 3464232050)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(3464232054n);
    input.add32(3464232050n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 1 (18444684815543689895, 2275366247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444684815543689895n);
    input.add32(2275366247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 2 (2275366243, 2275366247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2275366243n);
    input.add32(2275366247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 3 (2275366247, 2275366247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2275366247n);
    input.add32(2275366247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 4 (2275366247, 2275366243)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2275366247n);
    input.add32(2275366243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 1 (18437835454415206791, 9423578)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18437835454415206791n);
    input.add32(9423578n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 2 (9423574, 9423578)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9423574n);
    input.add32(9423578n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 3 (9423578, 9423578)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9423578n);
    input.add32(9423578n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 4 (9423578, 9423574)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9423578n);
    input.add32(9423574n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (18439733429675438659, 2253067908)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439733429675438659n);
    input.add32(2253067908n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (2253067904, 2253067908)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2253067904n);
    input.add32(2253067908n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (2253067908, 2253067908)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2253067908n);
    input.add32(2253067908n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (2253067908, 2253067904)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2253067908n);
    input.add32(2253067904n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (18440138933123760425, 1621625975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440138933123760425n);
    input.add32(1621625975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (1621625971, 1621625975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(1621625971n);
    input.add32(1621625975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (1621625975, 1621625975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(1621625975n);
    input.add32(1621625975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (1621625975, 1621625971)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(1621625975n);
    input.add32(1621625971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (18446290895922224733, 524974181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18446290895922224733n);
    input.add32(524974181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(524974181n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (524974177, 524974181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(524974177n);
    input.add32(524974181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(524974177n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (524974181, 524974181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(524974181n);
    input.add32(524974181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(524974181n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (524974181, 524974177)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(524974181n);
    input.add32(524974177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(524974177n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (18445393137642544791, 3865165477)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445393137642544791n);
    input.add32(3865165477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18445393137642544791n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (3865165473, 3865165477)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(3865165473n);
    input.add32(3865165477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(3865165477n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (3865165477, 3865165477)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(3865165477n);
    input.add32(3865165477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(3865165477n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (3865165477, 3865165473)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(3865165477n);
    input.add32(3865165473n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(3865165477n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (9218958508139779008, 9219044270046174287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9218958508139779008n);
    input.add64(9219044270046174287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438002778185953295n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (9218958508139779006, 9218958508139779008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9218958508139779006n);
    input.add64(9218958508139779008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437917016279558014n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (9218958508139779008, 9218958508139779008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9218958508139779008n);
    input.add64(9218958508139779008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437917016279558016n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (9218958508139779008, 9218958508139779006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9218958508139779008n);
    input.add64(9218958508139779006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437917016279558014n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (18441609894834603537, 18441609894834603537)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441609894834603537n);
    input.add64(18441609894834603537n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (18441609894834603537, 18441609894834603533)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441609894834603537n);
    input.add64(18441609894834603533n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (4292982372, 4294945214)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4292982372n);
    input.add64(4294945214n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438124092407767608n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (4292982372, 4292982372)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4292982372n);
    input.add64(4292982372n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18429697646302746384n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (4292982372, 4292982372)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4292982372n);
    input.add64(4292982372n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18429697646302746384n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (4292982372, 4292982372)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4292982372n);
    input.add64(4292982372n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18429697646302746384n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (18438288845569948835, 18446709820652278105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438288845569948835n);
    input.add64(18446709820652278105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438264643294089217n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (18438288845569948831, 18438288845569948835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438288845569948831n);
    input.add64(18438288845569948835n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438288845569948803n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (18438288845569948835, 18438288845569948835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438288845569948835n);
    input.add64(18438288845569948835n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438288845569948835n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (18438288845569948835, 18438288845569948831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438288845569948835n);
    input.add64(18438288845569948831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438288845569948803n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 1 (18441718910132730219, 18443559364541208497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441718910132730219n);
    input.add64(18443559364541208497n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18446374259834945531n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 2 (18441718910132730215, 18441718910132730219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441718910132730215n);
    input.add64(18441718910132730219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18441718910132730223n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 3 (18441718910132730219, 18441718910132730219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441718910132730219n);
    input.add64(18441718910132730219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18441718910132730219n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 4 (18441718910132730219, 18441718910132730215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441718910132730219n);
    input.add64(18441718910132730215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18441718910132730223n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 1 (18439273159029534023, 18439622247668308437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439273159029534023n);
    input.add64(18439622247668308437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1063788387333266n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 2 (18439273159029534019, 18439273159029534023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439273159029534019n);
    input.add64(18439273159029534023n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 3 (18439273159029534023, 18439273159029534023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439273159029534023n);
    input.add64(18439273159029534023n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 4 (18439273159029534023, 18439273159029534019)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439273159029534023n);
    input.add64(18439273159029534019n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 1 (18441380505660187447, 18441750104658674599)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441380505660187447n);
    input.add64(18441750104658674599n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 2 (18441380505660187443, 18441380505660187447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441380505660187443n);
    input.add64(18441380505660187447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 3 (18441380505660187447, 18441380505660187447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441380505660187447n);
    input.add64(18441380505660187447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 4 (18441380505660187447, 18441380505660187443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441380505660187447n);
    input.add64(18441380505660187443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (18442006653940621261, 18444786865163705679)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442006653940621261n);
    input.add64(18444786865163705679n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (18442006653940621257, 18442006653940621261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442006653940621257n);
    input.add64(18442006653940621261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (18442006653940621261, 18442006653940621261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442006653940621261n);
    input.add64(18442006653940621261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (18442006653940621261, 18442006653940621257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442006653940621261n);
    input.add64(18442006653940621257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 1 (18443459697052755653, 18440630546892693043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443459697052755653n);
    input.add64(18440630546892693043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 2 (18440630546892693039, 18440630546892693043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440630546892693039n);
    input.add64(18440630546892693043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 3 (18440630546892693043, 18440630546892693043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440630546892693043n);
    input.add64(18440630546892693043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 4 (18440630546892693043, 18440630546892693039)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440630546892693043n);
    input.add64(18440630546892693039n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (18440239741167735311, 18439106717100863709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440239741167735311n);
    input.add64(18439106717100863709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (18439106717100863705, 18439106717100863709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439106717100863705n);
    input.add64(18439106717100863709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (18439106717100863709, 18439106717100863709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439106717100863709n);
    input.add64(18439106717100863709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (18439106717100863709, 18439106717100863705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439106717100863709n);
    input.add64(18439106717100863705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (18443095947977784597, 18440873805049812055)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443095947977784597n);
    input.add64(18440873805049812055n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (18440873805049812051, 18440873805049812055)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440873805049812051n);
    input.add64(18440873805049812055n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (18440873805049812055, 18440873805049812055)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440873805049812055n);
    input.add64(18440873805049812055n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (18440873805049812055, 18440873805049812051)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440873805049812055n);
    input.add64(18440873805049812051n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 1 (18438938996100953011, 18444747685424544131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438938996100953011n);
    input.add64(18444747685424544131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 2 (18438938996100953007, 18438938996100953011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438938996100953007n);
    input.add64(18438938996100953011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 3 (18438938996100953011, 18438938996100953011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438938996100953011n);
    input.add64(18438938996100953011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 4 (18438938996100953011, 18438938996100953007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438938996100953011n);
    input.add64(18438938996100953007n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 1 (18443917910351942171, 18438866711307263083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443917910351942171n);
    input.add64(18438866711307263083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438866711307263083n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 2 (18438866711307263079, 18438866711307263083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438866711307263079n);
    input.add64(18438866711307263083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438866711307263079n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 3 (18438866711307263083, 18438866711307263083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438866711307263083n);
    input.add64(18438866711307263083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438866711307263083n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 4 (18438866711307263083, 18438866711307263079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438866711307263083n);
    input.add64(18438866711307263079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438866711307263079n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (18444366681014095179, 18439526748827954075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444366681014095179n);
    input.add64(18439526748827954075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18444366681014095179n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (18439526748827954071, 18439526748827954075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439526748827954071n);
    input.add64(18439526748827954075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439526748827954075n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (18439526748827954075, 18439526748827954075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439526748827954075n);
    input.add64(18439526748827954075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439526748827954075n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (18439526748827954075, 18439526748827954071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439526748827954075n);
    input.add64(18439526748827954071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439526748827954075n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 1 (2, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 2 (9222232531875126755, 9222232531875126757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9222232531875126755n);
    input.add128(9222232531875126757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18444465063750253512n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 3 (9222232531875126757, 9222232531875126757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9222232531875126757n);
    input.add128(9222232531875126757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18444465063750253514n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 4 (9222232531875126757, 9222232531875126755)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9222232531875126757n);
    input.add128(9222232531875126755n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18444465063750253512n);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 1 (18445113378266904443, 18445113378266904443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445113378266904443n);
    input.add128(18445113378266904443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 2 (18445113378266904443, 18445113378266904439)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445113378266904443n);
    input.add128(18445113378266904439n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 1 (2, 4611686018427387905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2n);
    input.add128(4611686018427387905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 2 (4292910636, 4292910636)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4292910636n);
    input.add128(4292910636n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18429081728681924496n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 3 (4292910636, 4292910636)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4292910636n);
    input.add128(4292910636n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18429081728681924496n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 4 (4292910636, 4292910636)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4292910636n);
    input.add128(4292910636n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18429081728681924496n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 1 (18445190107809278269, 340282366920938463463369013695160618779)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445190107809278269n);
    input.add128(340282366920938463463369013695160618779n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18440024425920268569n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 2 (18445190107809278265, 18445190107809278269)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445190107809278265n);
    input.add128(18445190107809278269n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18445190107809278265n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 3 (18445190107809278269, 18445190107809278269)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445190107809278269n);
    input.add128(18445190107809278269n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18445190107809278269n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 4 (18445190107809278269, 18445190107809278265)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445190107809278269n);
    input.add128(18445190107809278265n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18445190107809278265n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 1 (18441605824315670977, 340282366920938463463372293646972708037)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441605824315670977n);
    input.add128(340282366920938463463372293646972708037n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(340282366920938463463374607294316504517n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 2 (18441605824315670973, 18441605824315670977)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441605824315670973n);
    input.add128(18441605824315670977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18441605824315671037n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 3 (18441605824315670977, 18441605824315670977)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441605824315670977n);
    input.add128(18441605824315670977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18441605824315670977n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 4 (18441605824315670977, 18441605824315670973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441605824315670977n);
    input.add128(18441605824315670973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18441605824315671037n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 1 (18445019613130202671, 340282366920938463463368327658156122767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445019613130202671n);
    input.add128(340282366920938463463368327658156122767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(340282366920938463444932489042117393568n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 2 (18445019613130202667, 18445019613130202671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445019613130202667n);
    input.add128(18445019613130202671n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 3 (18445019613130202671, 18445019613130202671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445019613130202671n);
    input.add128(18445019613130202671n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 4 (18445019613130202671, 18445019613130202667)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445019613130202671n);
    input.add128(18445019613130202667n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 1 (18438514843724478293, 340282366920938463463372609285430294123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438514843724478293n);
    input.add128(340282366920938463463372609285430294123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 2 (18438514843724478289, 18438514843724478293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438514843724478289n);
    input.add128(18438514843724478293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 3 (18438514843724478293, 18438514843724478293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438514843724478293n);
    input.add128(18438514843724478293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 4 (18438514843724478293, 18438514843724478289)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438514843724478293n);
    input.add128(18438514843724478289n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 1 (18443262285694729645, 340282366920938463463371962526871054987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443262285694729645n);
    input.add128(340282366920938463463371962526871054987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 2 (18443262285694729641, 18443262285694729645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443262285694729641n);
    input.add128(18443262285694729645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 3 (18443262285694729645, 18443262285694729645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443262285694729645n);
    input.add128(18443262285694729645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 4 (18443262285694729645, 18443262285694729641)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443262285694729645n);
    input.add128(18443262285694729641n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 1 (18439390130120947945, 340282366920938463463366923053583862035)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439390130120947945n);
    input.add128(340282366920938463463366923053583862035n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 2 (18439390130120947941, 18439390130120947945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439390130120947941n);
    input.add128(18439390130120947945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 3 (18439390130120947945, 18439390130120947945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439390130120947945n);
    input.add128(18439390130120947945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 4 (18439390130120947945, 18439390130120947941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439390130120947945n);
    input.add128(18439390130120947941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 1 (18440565480861012001, 340282366920938463463370448959227301227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440565480861012001n);
    input.add128(340282366920938463463370448959227301227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 2 (18440565480861011997, 18440565480861012001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440565480861011997n);
    input.add128(18440565480861012001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 3 (18440565480861012001, 18440565480861012001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440565480861012001n);
    input.add128(18440565480861012001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 4 (18440565480861012001, 18440565480861011997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440565480861012001n);
    input.add128(18440565480861011997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 1 (18444780776581676955, 340282366920938463463368519222463591043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444780776581676955n);
    input.add128(340282366920938463463368519222463591043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 2 (18444780776581676951, 18444780776581676955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444780776581676951n);
    input.add128(18444780776581676955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 3 (18444780776581676955, 18444780776581676955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444780776581676955n);
    input.add128(18444780776581676955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 4 (18444780776581676955, 18444780776581676951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444780776581676955n);
    input.add128(18444780776581676951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 1 (18443860279711044617, 340282366920938463463373732346608666265)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443860279711044617n);
    input.add128(340282366920938463463373732346608666265n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 2 (18443860279711044613, 18443860279711044617)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443860279711044613n);
    input.add128(18443860279711044617n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 3 (18443860279711044617, 18443860279711044617)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443860279711044617n);
    input.add128(18443860279711044617n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 4 (18443860279711044617, 18443860279711044613)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443860279711044617n);
    input.add128(18443860279711044613n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 1 (18442685315399583047, 340282366920938463463371719574448688881)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442685315399583047n);
    input.add128(340282366920938463463371719574448688881n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18442685315399583047n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 2 (18442685315399583043, 18442685315399583047)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442685315399583043n);
    input.add128(18442685315399583047n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18442685315399583043n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 3 (18442685315399583047, 18442685315399583047)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442685315399583047n);
    input.add128(18442685315399583047n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18442685315399583047n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 4 (18442685315399583047, 18442685315399583043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442685315399583047n);
    input.add128(18442685315399583043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18442685315399583043n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 1 (18437954209638887591, 340282366920938463463370847202592982843)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18437954209638887591n);
    input.add128(340282366920938463463370847202592982843n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(340282366920938463463370847202592982843n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 2 (18437954209638887587, 18437954209638887591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18437954209638887587n);
    input.add128(18437954209638887591n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18437954209638887591n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 3 (18437954209638887591, 18437954209638887591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18437954209638887591n);
    input.add128(18437954209638887591n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18437954209638887591n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 4 (18437954209638887591, 18437954209638887587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18437954209638887591n);
    input.add128(18437954209638887587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.res128());
    expect(res).to.equal(18437954209638887591n);
  });

  it('test operator "add" overload (euint64, euint256) => euint256 test 1 (2, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2n);
    input.add256(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint64, euint256) => euint256 test 2 (9220197293325347346, 9220197293325347348)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9220197293325347346n);
    input.add256(9220197293325347348n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18440394586650694694n);
  });

  it('test operator "add" overload (euint64, euint256) => euint256 test 3 (9220197293325347348, 9220197293325347348)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9220197293325347348n);
    input.add256(9220197293325347348n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18440394586650694696n);
  });

  it('test operator "add" overload (euint64, euint256) => euint256 test 4 (9220197293325347348, 9220197293325347346)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9220197293325347348n);
    input.add256(9220197293325347346n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18440394586650694694n);
  });

  it('test operator "sub" overload (euint64, euint256) => euint256 test 1 (18442249009404799033, 18442249009404799033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442249009404799033n);
    input.add256(18442249009404799033n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint256) => euint256 test 2 (18442249009404799033, 18442249009404799029)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442249009404799033n);
    input.add256(18442249009404799029n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint256) => euint256 test 1 (2, 4611686018427387905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2n);
    input.add256(4611686018427387905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint64, euint256) => euint256 test 2 (4294583552, 4294583552)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4294583552n);
    input.add256(4294583552n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18443447885108936704n);
  });

  it('test operator "mul" overload (euint64, euint256) => euint256 test 3 (4294583552, 4294583552)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4294583552n);
    input.add256(4294583552n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18443447885108936704n);
  });

  it('test operator "mul" overload (euint64, euint256) => euint256 test 4 (4294583552, 4294583552)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4294583552n);
    input.add256(4294583552n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18443447885108936704n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 1 (18440312427812267645, 115792089237316195423570985008687907853269984665640564039457581704498105021397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440312427812267645n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581704498105021397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18438018358636487253n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 2 (18440312427812267641, 18440312427812267645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440312427812267641n);
    input.add256(18440312427812267645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18440312427812267641n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 3 (18440312427812267645, 18440312427812267645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440312427812267645n);
    input.add256(18440312427812267645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18440312427812267645n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 4 (18440312427812267645, 18440312427812267641)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440312427812267645n);
    input.add256(18440312427812267641n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18440312427812267641n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 1 (18438599529152616781, 115792089237316195423570985008687907853269984665640564039457577752782790981689)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438599529152616781n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577752782790981689n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578333324944079229n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 2 (18438599529152616777, 18438599529152616781)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438599529152616777n);
    input.add256(18438599529152616781n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18438599529152616781n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 3 (18438599529152616781, 18438599529152616781)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438599529152616781n);
    input.add256(18438599529152616781n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18438599529152616781n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 4 (18438599529152616781, 18438599529152616777)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438599529152616781n);
    input.add256(18438599529152616777n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18438599529152616781n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 1 (18446137277968957231, 115792089237316195423570985008687907853269984665640564039457582391179466649585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18446137277968957231n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582391179466649585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039439139402980215492830n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 2 (18446137277968957227, 18446137277968957231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18446137277968957227n);
    input.add256(18446137277968957231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 3 (18446137277968957231, 18446137277968957231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18446137277968957231n);
    input.add256(18446137277968957231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 4 (18446137277968957231, 18446137277968957227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18446137277968957231n);
    input.add256(18446137277968957227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 1 (18440809890833213527, 115792089237316195423570985008687907853269984665640564039457581890147785076985)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440809890833213527n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581890147785076985n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 2 (18440809890833213523, 18440809890833213527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440809890833213523n);
    input.add256(18440809890833213527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 3 (18440809890833213527, 18440809890833213527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440809890833213527n);
    input.add256(18440809890833213527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 4 (18440809890833213527, 18440809890833213523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440809890833213527n);
    input.add256(18440809890833213523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 1 (18438910137149192715, 115792089237316195423570985008687907853269984665640564039457577240295220380101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438910137149192715n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577240295220380101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 2 (18438910137149192711, 18438910137149192715)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438910137149192711n);
    input.add256(18438910137149192715n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 3 (18438910137149192715, 18438910137149192715)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438910137149192715n);
    input.add256(18438910137149192715n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 4 (18438910137149192715, 18438910137149192711)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438910137149192715n);
    input.add256(18438910137149192711n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint256) => ebool test 1 (18444988057831383485, 115792089237316195423570985008687907853269984665640564039457577605212785504979)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444988057831383485n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577605212785504979n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint256) => ebool test 2 (18444988057831383481, 18444988057831383485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444988057831383481n);
    input.add256(18444988057831383485n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint256) => ebool test 3 (18444988057831383485, 18444988057831383485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444988057831383485n);
    input.add256(18444988057831383485n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint256) => ebool test 4 (18444988057831383485, 18444988057831383481)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444988057831383485n);
    input.add256(18444988057831383481n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint256) => ebool test 1 (18442368386102086735, 115792089237316195423570985008687907853269984665640564039457576020329542043969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442368386102086735n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576020329542043969n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint256) => ebool test 2 (18442368386102086731, 18442368386102086735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442368386102086731n);
    input.add256(18442368386102086735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint256) => ebool test 3 (18442368386102086735, 18442368386102086735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442368386102086735n);
    input.add256(18442368386102086735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint256) => ebool test 4 (18442368386102086735, 18442368386102086731)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442368386102086735n);
    input.add256(18442368386102086731n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint256) => ebool test 1 (18439546002364665747, 115792089237316195423570985008687907853269984665640564039457577884357267949935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439546002364665747n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577884357267949935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint256) => ebool test 2 (18439546002364665743, 18439546002364665747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439546002364665743n);
    input.add256(18439546002364665747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint256) => ebool test 3 (18439546002364665747, 18439546002364665747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439546002364665747n);
    input.add256(18439546002364665747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint256) => ebool test 4 (18439546002364665747, 18439546002364665743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439546002364665747n);
    input.add256(18439546002364665743n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint256) => ebool test 1 (18444881698053030039, 115792089237316195423570985008687907853269984665640564039457583131510872584601)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444881698053030039n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583131510872584601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint256) => ebool test 2 (18444881698053030035, 18444881698053030039)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444881698053030035n);
    input.add256(18444881698053030039n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint256) => ebool test 3 (18444881698053030039, 18444881698053030039)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444881698053030039n);
    input.add256(18444881698053030039n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint256) => ebool test 4 (18444881698053030039, 18444881698053030035)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444881698053030039n);
    input.add256(18444881698053030035n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint256) => euint256 test 1 (18441649092848148817, 115792089237316195423570985008687907853269984665640564039457579822258438079223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441649092848148817n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579822258438079223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18441649092848148817n);
  });

  it('test operator "min" overload (euint64, euint256) => euint256 test 2 (18441649092848148813, 18441649092848148817)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441649092848148813n);
    input.add256(18441649092848148817n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18441649092848148813n);
  });

  it('test operator "min" overload (euint64, euint256) => euint256 test 3 (18441649092848148817, 18441649092848148817)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441649092848148817n);
    input.add256(18441649092848148817n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18441649092848148817n);
  });

  it('test operator "min" overload (euint64, euint256) => euint256 test 4 (18441649092848148817, 18441649092848148813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441649092848148817n);
    input.add256(18441649092848148813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18441649092848148813n);
  });

  it('test operator "max" overload (euint64, euint256) => euint256 test 1 (18438491249517222761, 115792089237316195423570985008687907853269984665640564039457581594046194597391)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438491249517222761n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581594046194597391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581594046194597391n);
  });

  it('test operator "max" overload (euint64, euint256) => euint256 test 2 (18438491249517222757, 18438491249517222761)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438491249517222757n);
    input.add256(18438491249517222761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18438491249517222761n);
  });

  it('test operator "max" overload (euint64, euint256) => euint256 test 3 (18438491249517222761, 18438491249517222761)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438491249517222761n);
    input.add256(18438491249517222761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18438491249517222761n);
  });

  it('test operator "max" overload (euint64, euint256) => euint256 test 4 (18438491249517222761, 18438491249517222757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438491249517222761n);
    input.add256(18438491249517222757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.res256());
    expect(res).to.equal(18438491249517222761n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (9218958508139779008, 9219149395740790603)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9218958508139779008n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_uint64(
      encryptedAmount.handles[0],
      9219149395740790603n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438107903880569611n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (9218958508139779006, 9218958508139779008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9218958508139779006n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_uint64(
      encryptedAmount.handles[0],
      9218958508139779008n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437917016279558014n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (9218958508139779008, 9218958508139779008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9218958508139779008n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_uint64(
      encryptedAmount.handles[0],
      9218958508139779008n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437917016279558016n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (9218958508139779008, 9218958508139779006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9218958508139779008n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_uint64(
      encryptedAmount.handles[0],
      9218958508139779006n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437917016279558014n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 1 (9219492719392409961, 9219149395740790603)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(9219149395740790603n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint64_euint64(
      9219492719392409961n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438642115133200564n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 2 (9218958508139779006, 9218958508139779008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(9218958508139779008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint64_euint64(
      9218958508139779006n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437917016279558014n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 3 (9218958508139779008, 9218958508139779008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(9218958508139779008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint64_euint64(
      9218958508139779008n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437917016279558016n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 4 (9218958508139779008, 9218958508139779006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(9218958508139779006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint64_euint64(
      9218958508139779008n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437917016279558014n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (18441609894834603537, 18441609894834603537)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441609894834603537n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18441609894834603537n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (18441609894834603537, 18441609894834603533)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441609894834603537n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18441609894834603533n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (18441609894834603537, 18441609894834603537)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441609894834603537n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint64_euint64(
      18441609894834603537n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (18441609894834603537, 18441609894834603533)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441609894834603533n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint64_euint64(
      18441609894834603537n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (4292982372, 4293794443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4292982372n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4293794443n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18433183852790558796n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (4292982372, 4292982372)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4292982372n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4292982372n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18429697646302746384n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (4292982372, 4292982372)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4292982372n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4292982372n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18429697646302746384n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (4292982372, 4292982372)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4292982372n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4292982372n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18429697646302746384n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 1 (4294959986, 4293794443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(4293794443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint64_euint64(
      4294959986n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18441675320794157798n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 2 (4292982372, 4292982372)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(4292982372n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint64_euint64(
      4292982372n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18429697646302746384n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 3 (4292982372, 4292982372)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(4292982372n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint64_euint64(
      4292982372n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18429697646302746384n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 4 (4292982372, 4292982372)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(4292982372n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint64_euint64(
      4292982372n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18429697646302746384n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (18439493296588317485, 18437926869096203691)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439493296588317485n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint64_uint64(
      encryptedAmount.handles[0],
      18437926869096203691n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (18439493296588317481, 18439493296588317485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439493296588317481n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint64_uint64(
      encryptedAmount.handles[0],
      18439493296588317485n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (18439493296588317485, 18439493296588317485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439493296588317485n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint64_uint64(
      encryptedAmount.handles[0],
      18439493296588317485n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (18439493296588317485, 18439493296588317481)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439493296588317485n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint64_uint64(
      encryptedAmount.handles[0],
      18439493296588317481n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 1 (18443497963676535693, 18444271018460588387)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443497963676535693n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18444271018460588387n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18443497963676535693n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 2 (18443497963676535689, 18443497963676535693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443497963676535689n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18443497963676535693n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18443497963676535689n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 3 (18443497963676535693, 18443497963676535693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443497963676535693n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18443497963676535693n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 4 (18443497963676535693, 18443497963676535689)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443497963676535693n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18443497963676535689n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });
});
