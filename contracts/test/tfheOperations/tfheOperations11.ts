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

describe('TFHE operations 11', function () {
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

  it('test operator "ge" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576096227272675885, 18444904697733794557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576096227272675885n);
    input.add64(18444904697733794557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint64) => ebool test 2 (18444904697733794553, 18444904697733794557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18444904697733794553n);
    input.add64(18444904697733794557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint64) => ebool test 3 (18444904697733794557, 18444904697733794557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18444904697733794557n);
    input.add64(18444904697733794557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint64) => ebool test 4 (18444904697733794557, 18444904697733794553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18444904697733794557n);
    input.add64(18444904697733794553n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583295799701215083, 18442381377709361653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583295799701215083n);
    input.add64(18442381377709361653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint64) => ebool test 2 (18442381377709361649, 18442381377709361653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18442381377709361649n);
    input.add64(18442381377709361653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint64) => ebool test 3 (18442381377709361653, 18442381377709361653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18442381377709361653n);
    input.add64(18442381377709361653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint64) => ebool test 4 (18442381377709361653, 18442381377709361649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18442381377709361653n);
    input.add64(18442381377709361649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583571879129953411, 18440967931273598709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583571879129953411n);
    input.add64(18440967931273598709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint256, euint64) => ebool test 2 (18440967931273598705, 18440967931273598709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18440967931273598705n);
    input.add64(18440967931273598709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint64) => ebool test 3 (18440967931273598709, 18440967931273598709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18440967931273598709n);
    input.add64(18440967931273598709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint64) => ebool test 4 (18440967931273598709, 18440967931273598705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18440967931273598709n);
    input.add64(18440967931273598705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581831565021273765, 18440482229448051801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581831565021273765n);
    input.add64(18440482229448051801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint64) => ebool test 2 (18440482229448051797, 18440482229448051801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18440482229448051797n);
    input.add64(18440482229448051801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, euint64) => ebool test 3 (18440482229448051801, 18440482229448051801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18440482229448051801n);
    input.add64(18440482229448051801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint64) => ebool test 4 (18440482229448051801, 18440482229448051797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18440482229448051801n);
    input.add64(18440482229448051797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582092821123896169, 18446104662848676487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582092821123896169n);
    input.add64(18446104662848676487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18446104662848676487n);
  });

  it('test operator "min" overload (euint256, euint64) => euint256 test 2 (18446104662848676483, 18446104662848676487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18446104662848676483n);
    input.add64(18446104662848676487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18446104662848676483n);
  });

  it('test operator "min" overload (euint256, euint64) => euint256 test 3 (18446104662848676487, 18446104662848676487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18446104662848676487n);
    input.add64(18446104662848676487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18446104662848676487n);
  });

  it('test operator "min" overload (euint256, euint64) => euint256 test 4 (18446104662848676487, 18446104662848676483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18446104662848676487n);
    input.add64(18446104662848676483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18446104662848676483n);
  });

  it('test operator "max" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580677795582710749, 18441147301030989339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580677795582710749n);
    input.add64(18441147301030989339n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457580677795582710749n);
  });

  it('test operator "max" overload (euint256, euint64) => euint256 test 2 (18441147301030989335, 18441147301030989339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18441147301030989335n);
    input.add64(18441147301030989339n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18441147301030989339n);
  });

  it('test operator "max" overload (euint256, euint64) => euint256 test 3 (18441147301030989339, 18441147301030989339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18441147301030989339n);
    input.add64(18441147301030989339n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18441147301030989339n);
  });

  it('test operator "max" overload (euint256, euint64) => euint256 test 4 (18441147301030989339, 18441147301030989335)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18441147301030989339n);
    input.add64(18441147301030989335n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18441147301030989339n);
  });

  it('test operator "add" overload (euint256, euint128) => euint256 test 1 (170141183460469231731687303715884105729, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);
    input.add128(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(170141183460469231731687303715884105731n);
  });

  it('test operator "add" overload (euint256, euint128) => euint256 test 2 (170141183460469231731685235499546332350, 170141183460469231731685235499546332352)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(170141183460469231731685235499546332350n);
    input.add128(170141183460469231731685235499546332352n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(340282366920938463463370470999092664702n);
  });

  it('test operator "add" overload (euint256, euint128) => euint256 test 3 (170141183460469231731685235499546332352, 170141183460469231731685235499546332352)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(170141183460469231731685235499546332352n);
    input.add128(170141183460469231731685235499546332352n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(340282366920938463463370470999092664704n);
  });

  it('test operator "add" overload (euint256, euint128) => euint256 test 4 (170141183460469231731685235499546332352, 170141183460469231731685235499546332350)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(170141183460469231731685235499546332352n);
    input.add128(170141183460469231731685235499546332350n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(340282366920938463463370470999092664702n);
  });

  it('test operator "sub" overload (euint256, euint128) => euint256 test 1 (340282366920938463463368481567631642621, 340282366920938463463368481567631642621)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(340282366920938463463368481567631642621n);
    input.add128(340282366920938463463368481567631642621n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, euint128) => euint256 test 2 (340282366920938463463368481567631642621, 340282366920938463463368481567631642617)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(340282366920938463463368481567631642621n);
    input.add128(340282366920938463463368481567631642617n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, euint128) => euint256 test 1 (85070591730234615865843651857942052865, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(85070591730234615865843651857942052865n);
    input.add128(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(170141183460469231731687303715884105730n);
  });

  it('test operator "mul" overload (euint256, euint128) => euint256 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint256, euint128) => euint256 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint256, euint128) => euint256 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577962511369021075, 340282366920938463463370601447284280593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577962511369021075n);
    input.add128(340282366920938463463370601447284280593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(340282366920938463463365746003796296721n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 2 (340282366920938463463370601447284280589, 340282366920938463463370601447284280593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(340282366920938463463370601447284280589n);
    input.add128(340282366920938463463370601447284280593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(340282366920938463463370601447284280577n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 3 (340282366920938463463370601447284280593, 340282366920938463463370601447284280593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(340282366920938463463370601447284280593n);
    input.add128(340282366920938463463370601447284280593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(340282366920938463463370601447284280593n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 4 (340282366920938463463370601447284280593, 340282366920938463463370601447284280589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(340282366920938463463370601447284280593n);
    input.add128(340282366920938463463370601447284280589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(340282366920938463463370601447284280577n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583284103678065013, 340282366920938463463374528785081380165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583284103678065013n);
    input.add128(340282366920938463463374528785081380165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457584005692613459317n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 2 (340282366920938463463374528785081380161, 340282366920938463463374528785081380165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463374528785081380161n);
    input.add128(340282366920938463463374528785081380165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(340282366920938463463374528785081380165n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 3 (340282366920938463463374528785081380165, 340282366920938463463374528785081380165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463374528785081380165n);
    input.add128(340282366920938463463374528785081380165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(340282366920938463463374528785081380165n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 4 (340282366920938463463374528785081380165, 340282366920938463463374528785081380161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463374528785081380165n);
    input.add128(340282366920938463463374528785081380161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(340282366920938463463374528785081380165n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576256116597363861, 340282366920938463463369953718945361073)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576256116597363861n);
    input.add128(340282366920938463463369953718945361073n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907852929702298719625575994212499666228754468n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 2 (340282366920938463463369953718945361069, 340282366920938463463369953718945361073)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463369953718945361069n);
    input.add128(340282366920938463463369953718945361073n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 3 (340282366920938463463369953718945361073, 340282366920938463463369953718945361073)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463369953718945361073n);
    input.add128(340282366920938463463369953718945361073n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 4 (340282366920938463463369953718945361073, 340282366920938463463369953718945361069)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463369953718945361073n);
    input.add128(340282366920938463463369953718945361069n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581361607585561697, 340282366920938463463366843615844178229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581361607585561697n);
    input.add128(340282366920938463463366843615844178229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 2 (340282366920938463463366843615844178225, 340282366920938463463366843615844178229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463366843615844178225n);
    input.add128(340282366920938463463366843615844178229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 3 (340282366920938463463366843615844178229, 340282366920938463463366843615844178229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463366843615844178229n);
    input.add128(340282366920938463463366843615844178229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 4 (340282366920938463463366843615844178229, 340282366920938463463366843615844178225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463366843615844178229n);
    input.add128(340282366920938463463366843615844178225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575748917038833099, 340282366920938463463366150915279503405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575748917038833099n);
    input.add128(340282366920938463463366150915279503405n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 2 (340282366920938463463366150915279503401, 340282366920938463463366150915279503405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463366150915279503401n);
    input.add128(340282366920938463463366150915279503405n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 3 (340282366920938463463366150915279503405, 340282366920938463463366150915279503405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463366150915279503405n);
    input.add128(340282366920938463463366150915279503405n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 4 (340282366920938463463366150915279503405, 340282366920938463463366150915279503401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463366150915279503405n);
    input.add128(340282366920938463463366150915279503401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457578825306788754867, 340282366920938463463371974709233703663)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578825306788754867n);
    input.add128(340282366920938463463371974709233703663n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint128) => ebool test 2 (340282366920938463463371974709233703659, 340282366920938463463371974709233703663)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463371974709233703659n);
    input.add128(340282366920938463463371974709233703663n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint128) => ebool test 3 (340282366920938463463371974709233703663, 340282366920938463463371974709233703663)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463371974709233703663n);
    input.add128(340282366920938463463371974709233703663n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint128) => ebool test 4 (340282366920938463463371974709233703663, 340282366920938463463371974709233703659)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463371974709233703663n);
    input.add128(340282366920938463463371974709233703659n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581376037545855385, 340282366920938463463374112751128461429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581376037545855385n);
    input.add128(340282366920938463463374112751128461429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint128) => ebool test 2 (340282366920938463463374112751128461425, 340282366920938463463374112751128461429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463374112751128461425n);
    input.add128(340282366920938463463374112751128461429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint128) => ebool test 3 (340282366920938463463374112751128461429, 340282366920938463463374112751128461429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463374112751128461429n);
    input.add128(340282366920938463463374112751128461429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint128) => ebool test 4 (340282366920938463463374112751128461429, 340282366920938463463374112751128461425)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463374112751128461429n);
    input.add128(340282366920938463463374112751128461425n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576166618757285903, 340282366920938463463371146235632523129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576166618757285903n);
    input.add128(340282366920938463463371146235632523129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint256, euint128) => ebool test 2 (340282366920938463463371146235632523125, 340282366920938463463371146235632523129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463371146235632523125n);
    input.add128(340282366920938463463371146235632523129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint128) => ebool test 3 (340282366920938463463371146235632523129, 340282366920938463463371146235632523129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463371146235632523129n);
    input.add128(340282366920938463463371146235632523129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint128) => ebool test 4 (340282366920938463463371146235632523129, 340282366920938463463371146235632523125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463371146235632523129n);
    input.add128(340282366920938463463371146235632523125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457578717161849931529, 340282366920938463463370050615725449131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578717161849931529n);
    input.add128(340282366920938463463370050615725449131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint128) => ebool test 2 (340282366920938463463370050615725449127, 340282366920938463463370050615725449131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463370050615725449127n);
    input.add128(340282366920938463463370050615725449131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, euint128) => ebool test 3 (340282366920938463463370050615725449131, 340282366920938463463370050615725449131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463370050615725449131n);
    input.add128(340282366920938463463370050615725449131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint128) => ebool test 4 (340282366920938463463370050615725449131, 340282366920938463463370050615725449127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463370050615725449131n);
    input.add128(340282366920938463463370050615725449127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580727698993564447, 340282366920938463463367758600088042883)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580727698993564447n);
    input.add128(340282366920938463463367758600088042883n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(340282366920938463463367758600088042883n);
  });

  it('test operator "min" overload (euint256, euint128) => euint256 test 2 (340282366920938463463367758600088042879, 340282366920938463463367758600088042883)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463367758600088042879n);
    input.add128(340282366920938463463367758600088042883n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(340282366920938463463367758600088042879n);
  });

  it('test operator "min" overload (euint256, euint128) => euint256 test 3 (340282366920938463463367758600088042883, 340282366920938463463367758600088042883)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463367758600088042883n);
    input.add128(340282366920938463463367758600088042883n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(340282366920938463463367758600088042883n);
  });

  it('test operator "min" overload (euint256, euint128) => euint256 test 4 (340282366920938463463367758600088042883, 340282366920938463463367758600088042879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463367758600088042883n);
    input.add128(340282366920938463463367758600088042879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(340282366920938463463367758600088042879n);
  });

  it('test operator "max" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578391821014626789, 340282366920938463463366527423723432423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578391821014626789n);
    input.add128(340282366920938463463366527423723432423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578391821014626789n);
  });

  it('test operator "max" overload (euint256, euint128) => euint256 test 2 (340282366920938463463366527423723432419, 340282366920938463463366527423723432423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463366527423723432419n);
    input.add128(340282366920938463463366527423723432423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(340282366920938463463366527423723432423n);
  });

  it('test operator "max" overload (euint256, euint128) => euint256 test 3 (340282366920938463463366527423723432423, 340282366920938463463366527423723432423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463366527423723432423n);
    input.add128(340282366920938463463366527423723432423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(340282366920938463463366527423723432423n);
  });

  it('test operator "max" overload (euint256, euint128) => euint256 test 4 (340282366920938463463366527423723432423, 340282366920938463463366527423723432419)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(340282366920938463463366527423723432423n);
    input.add128(340282366920938463463366527423723432419n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(340282366920938463463366527423723432423n);
  });

  it('test operator "add" overload (euint256, euint256) => euint256 test 1 (57896044618658097711785492504343953926634992332820282019728787827687038090937, 57896044618658097711785492504343953926634992332820282019728790977191141367262)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728787827687038090937n);
    input.add256(57896044618658097711785492504343953926634992332820282019728790977191141367262n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578804878179458199n);
  });

  it('test operator "add" overload (euint256, euint256) => euint256 test 2 (57896044618658097711785492504343953926634992332820282019728787827687038090935, 57896044618658097711785492504343953926634992332820282019728787827687038090937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728787827687038090935n);
    input.add256(57896044618658097711785492504343953926634992332820282019728787827687038090937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575655374076181872n);
  });

  it('test operator "add" overload (euint256, euint256) => euint256 test 3 (57896044618658097711785492504343953926634992332820282019728787827687038090937, 57896044618658097711785492504343953926634992332820282019728787827687038090937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728787827687038090937n);
    input.add256(57896044618658097711785492504343953926634992332820282019728787827687038090937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575655374076181874n);
  });

  it('test operator "add" overload (euint256, euint256) => euint256 test 4 (57896044618658097711785492504343953926634992332820282019728787827687038090937, 57896044618658097711785492504343953926634992332820282019728787827687038090935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728787827687038090937n);
    input.add256(57896044618658097711785492504343953926634992332820282019728787827687038090935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575655374076181872n);
  });

  it('test operator "sub" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575938904758977495, 115792089237316195423570985008687907853269984665640564039457575938904758977495)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575938904758977495n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575938904758977495n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575938904758977495, 115792089237316195423570985008687907853269984665640564039457575938904758977491)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575938904758977495n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575938904758977491n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, euint256) => euint256 test 1 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);
    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (euint256, euint256) => euint256 test 2 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);
    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (euint256, euint256) => euint256 test 3 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);
    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (euint256, euint256) => euint256 test 4 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);
    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582230404505334331, 115792089237316195423570985008687907853269984665640564039457578924727181125233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582230404505334331n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578924727181125233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577710307828310577n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457578924727181125229, 115792089237316195423570985008687907853269984665640564039457578924727181125233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578924727181125229n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578924727181125233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578924727181125217n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457578924727181125233, 115792089237316195423570985008687907853269984665640564039457578924727181125233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578924727181125233n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578924727181125233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578924727181125233n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457578924727181125233, 115792089237316195423570985008687907853269984665640564039457578924727181125229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578924727181125233n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578924727181125229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578924727181125217n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583238131496013849, 115792089237316195423570985008687907853269984665640564039457582794471900706475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583238131496013849n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582794471900706475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583994597653083835n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457582794471900706471, 115792089237316195423570985008687907853269984665640564039457582794471900706475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582794471900706471n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582794471900706475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582794471900706479n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457582794471900706475, 115792089237316195423570985008687907853269984665640564039457582794471900706475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582794471900706475n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582794471900706475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582794471900706475n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457582794471900706475, 115792089237316195423570985008687907853269984665640564039457582794471900706471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582794471900706475n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582794471900706471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582794471900706479n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575053127883308623, 115792089237316195423570985008687907853269984665640564039457577085548183544005)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575053127883308623n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577085548183544005n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(2049003373715082n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575053127883308619, 115792089237316195423570985008687907853269984665640564039457575053127883308623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575053127883308619n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575053127883308623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575053127883308623, 115792089237316195423570985008687907853269984665640564039457575053127883308623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575053127883308623n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575053127883308623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575053127883308623, 115792089237316195423570985008687907853269984665640564039457575053127883308619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575053127883308623n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575053127883308619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582065856019738465, 115792089237316195423570985008687907853269984665640564039457582405115329528379)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582065856019738465n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582405115329528379n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457582065856019738461, 115792089237316195423570985008687907853269984665640564039457582065856019738465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582065856019738461n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582065856019738465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457582065856019738465, 115792089237316195423570985008687907853269984665640564039457582065856019738465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582065856019738465n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582065856019738465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457582065856019738465, 115792089237316195423570985008687907853269984665640564039457582065856019738461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582065856019738465n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582065856019738461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575918633215654475, 115792089237316195423570985008687907853269984665640564039457581646253057648843)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575918633215654475n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581646253057648843n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575918633215654471, 115792089237316195423570985008687907853269984665640564039457575918633215654475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575918633215654471n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575918633215654475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575918633215654475, 115792089237316195423570985008687907853269984665640564039457575918633215654475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575918633215654475n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575918633215654475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575918633215654475, 115792089237316195423570985008687907853269984665640564039457575918633215654471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575918633215654475n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575918633215654471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577616693175358637, 115792089237316195423570985008687907853269984665640564039457580763146256955661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577616693175358637n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580763146256955661n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457577616693175358633, 115792089237316195423570985008687907853269984665640564039457577616693175358637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577616693175358633n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577616693175358637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457577616693175358637, 115792089237316195423570985008687907853269984665640564039457577616693175358637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577616693175358637n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577616693175358637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457577616693175358637, 115792089237316195423570985008687907853269984665640564039457577616693175358633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577616693175358637n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577616693175358633n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582931411410180425, 115792089237316195423570985008687907853269984665640564039457578248801468140041)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582931411410180425n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578248801468140041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457578248801468140037, 115792089237316195423570985008687907853269984665640564039457578248801468140041)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578248801468140037n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578248801468140041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457578248801468140041, 115792089237316195423570985008687907853269984665640564039457578248801468140041)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578248801468140041n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578248801468140041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457578248801468140041, 115792089237316195423570985008687907853269984665640564039457578248801468140037)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578248801468140041n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578248801468140037n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575943118443967733, 115792089237316195423570985008687907853269984665640564039457581615760476374113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575943118443967733n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581615760476374113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575943118443967729, 115792089237316195423570985008687907853269984665640564039457575943118443967733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575943118443967729n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575943118443967733n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575943118443967733, 115792089237316195423570985008687907853269984665640564039457575943118443967733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575943118443967733n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575943118443967733n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575943118443967733, 115792089237316195423570985008687907853269984665640564039457575943118443967729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575943118443967733n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575943118443967729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575244791569769895, 115792089237316195423570985008687907853269984665640564039457581326236260944735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575244791569769895n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581326236260944735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575244791569769891, 115792089237316195423570985008687907853269984665640564039457575244791569769895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575244791569769891n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575244791569769895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575244791569769895, 115792089237316195423570985008687907853269984665640564039457575244791569769895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575244791569769895n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575244791569769895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575244791569769895, 115792089237316195423570985008687907853269984665640564039457575244791569769891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575244791569769895n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575244791569769891n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578019618244793955, 115792089237316195423570985008687907853269984665640564039457580396349159096767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578019618244793955n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580396349159096767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578019618244793955n);
  });

  it('test operator "min" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457578019618244793951, 115792089237316195423570985008687907853269984665640564039457578019618244793955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578019618244793951n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578019618244793955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578019618244793951n);
  });

  it('test operator "min" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457578019618244793955, 115792089237316195423570985008687907853269984665640564039457578019618244793955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578019618244793955n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578019618244793955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578019618244793955n);
  });

  it('test operator "min" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457578019618244793955, 115792089237316195423570985008687907853269984665640564039457578019618244793951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578019618244793955n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578019618244793951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578019618244793951n);
  });

  it('test operator "max" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577650256858684559, 115792089237316195423570985008687907853269984665640564039457581101490491934149)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581101490491934149n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581101490491934149n);
  });

  it('test operator "max" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457577650256858684555, 115792089237316195423570985008687907853269984665640564039457577650256858684559)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577650256858684555n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
  });

  it('test operator "max" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457577650256858684559, 115792089237316195423570985008687907853269984665640564039457577650256858684559)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
  });

  it('test operator "max" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457577650256858684559, 115792089237316195423570985008687907853269984665640564039457577650256858684555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577650256858684555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
  });

  it('test operator "add" overload (euint256, uint256) => euint256 test 1 (57896044618658097711785492504343953926634992332820282019728787827687038090937, 57896044618658097711785492504343953926634992332820282019728787914503321183587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728787827687038090937n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint256_uint256(
      encryptedAmount.handles[0],
      57896044618658097711785492504343953926634992332820282019728787914503321183587n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575742190359274524n);
  });

  it('test operator "add" overload (euint256, uint256) => euint256 test 2 (57896044618658097711785492504343953926634992332820282019728787827687038090935, 57896044618658097711785492504343953926634992332820282019728787827687038090937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728787827687038090935n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint256_uint256(
      encryptedAmount.handles[0],
      57896044618658097711785492504343953926634992332820282019728787827687038090937n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575655374076181872n);
  });

  it('test operator "add" overload (euint256, uint256) => euint256 test 3 (57896044618658097711785492504343953926634992332820282019728787827687038090937, 57896044618658097711785492504343953926634992332820282019728787827687038090937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728787827687038090937n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint256_uint256(
      encryptedAmount.handles[0],
      57896044618658097711785492504343953926634992332820282019728787827687038090937n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575655374076181874n);
  });

  it('test operator "add" overload (euint256, uint256) => euint256 test 4 (57896044618658097711785492504343953926634992332820282019728787827687038090937, 57896044618658097711785492504343953926634992332820282019728787827687038090935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728787827687038090937n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint256_uint256(
      encryptedAmount.handles[0],
      57896044618658097711785492504343953926634992332820282019728787827687038090935n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575655374076181872n);
  });

  it('test operator "add" overload (uint256, euint256) => euint256 test 1 (57896044618658097711785492504343953926634992332820282019728789385142520081006, 57896044618658097711785492504343953926634992332820282019728787914503321183587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(57896044618658097711785492504343953926634992332820282019728787914503321183587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_uint256_euint256(
      57896044618658097711785492504343953926634992332820282019728789385142520081006n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577299645841264593n);
  });

  it('test operator "add" overload (uint256, euint256) => euint256 test 2 (57896044618658097711785492504343953926634992332820282019728787827687038090935, 57896044618658097711785492504343953926634992332820282019728787827687038090937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(57896044618658097711785492504343953926634992332820282019728787827687038090937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_uint256_euint256(
      57896044618658097711785492504343953926634992332820282019728787827687038090935n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575655374076181872n);
  });

  it('test operator "add" overload (uint256, euint256) => euint256 test 3 (57896044618658097711785492504343953926634992332820282019728787827687038090937, 57896044618658097711785492504343953926634992332820282019728787827687038090937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(57896044618658097711785492504343953926634992332820282019728787827687038090937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_uint256_euint256(
      57896044618658097711785492504343953926634992332820282019728787827687038090937n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575655374076181874n);
  });

  it('test operator "add" overload (uint256, euint256) => euint256 test 4 (57896044618658097711785492504343953926634992332820282019728787827687038090937, 57896044618658097711785492504343953926634992332820282019728787827687038090935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(57896044618658097711785492504343953926634992332820282019728787827687038090935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_uint256_euint256(
      57896044618658097711785492504343953926634992332820282019728787827687038090937n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575655374076181872n);
  });

  it('test operator "sub" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575938904758977495, 115792089237316195423570985008687907853269984665640564039457575938904758977495)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575938904758977495n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575938904758977495n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575938904758977495, 115792089237316195423570985008687907853269984665640564039457575938904758977491)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575938904758977495n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575938904758977491n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575938904758977495, 115792089237316195423570985008687907853269984665640564039457575938904758977495)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575938904758977495n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575938904758977495n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575938904758977495, 115792089237316195423570985008687907853269984665640564039457575938904758977491)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575938904758977491n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575938904758977495n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, uint256) => euint256 test 1 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint256_uint256(
      encryptedAmount.handles[0],
      170141183460469231731687303715884105729n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (euint256, uint256) => euint256 test 2 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint256_uint256(
      encryptedAmount.handles[0],
      170141183460469231731687303715884105729n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (euint256, uint256) => euint256 test 3 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint256_uint256(
      encryptedAmount.handles[0],
      170141183460469231731687303715884105729n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (euint256, uint256) => euint256 test 4 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint256_uint256(
      encryptedAmount.handles[0],
      170141183460469231731687303715884105729n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (uint256, euint256) => euint256 test 1 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_uint256_euint256(
      170141183460469231731687303715884105729n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (uint256, euint256) => euint256 test 2 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_uint256_euint256(
      170141183460469231731687303715884105729n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (uint256, euint256) => euint256 test 3 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_uint256_euint256(
      170141183460469231731687303715884105729n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (uint256, euint256) => euint256 test 4 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_uint256_euint256(
      170141183460469231731687303715884105729n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "div" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581849763780519797, 115792089237316195423570985008687907853269984665640564039457580587150312894311)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581849763780519797n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.div_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580587150312894311n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575058724392622669, 115792089237316195423570985008687907853269984665640564039457575058724392622673)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575058724392622669n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.div_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575058724392622673n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575058724392622673, 115792089237316195423570985008687907853269984665640564039457575058724392622673)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575058724392622673n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.div_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575058724392622673n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575058724392622673, 115792089237316195423570985008687907853269984665640564039457575058724392622669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575058724392622673n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.div_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575058724392622669n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576827418490805777, 115792089237316195423570985008687907853269984665640564039457578071505060399051)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576827418490805777n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rem_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578071505060399051n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576827418490805777n);
  });

  it('test operator "rem" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457576827418490805773, 115792089237316195423570985008687907853269984665640564039457576827418490805777)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576827418490805773n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rem_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576827418490805777n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576827418490805773n);
  });

  it('test operator "rem" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457576827418490805777, 115792089237316195423570985008687907853269984665640564039457576827418490805777)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576827418490805777n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rem_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576827418490805777n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457576827418490805777, 115792089237316195423570985008687907853269984665640564039457576827418490805773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576827418490805777n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rem_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576827418490805773n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582230404505334331, 115792089237316195423570985008687907853269984665640564039457579050772278546725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582230404505334331n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457579050772278546725n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577291003110887457n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457578924727181125229, 115792089237316195423570985008687907853269984665640564039457578924727181125233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578924727181125229n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578924727181125233n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578924727181125217n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457578924727181125233, 115792089237316195423570985008687907853269984665640564039457578924727181125233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578924727181125233n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578924727181125233n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578924727181125233n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457578924727181125233, 115792089237316195423570985008687907853269984665640564039457578924727181125229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578924727181125233n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578924727181125229n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578924727181125217n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575586864219065957, 115792089237316195423570985008687907853269984665640564039457579050772278546725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579050772278546725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575586864219065957n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575564769782923301n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457578924727181125229, 115792089237316195423570985008687907853269984665640564039457578924727181125233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578924727181125233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578924727181125229n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578924727181125217n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457578924727181125233, 115792089237316195423570985008687907853269984665640564039457578924727181125233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578924727181125233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578924727181125233n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578924727181125233n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457578924727181125233, 115792089237316195423570985008687907853269984665640564039457578924727181125229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578924727181125229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578924727181125233n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578924727181125217n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583238131496013849, 115792089237316195423570985008687907853269984665640564039457576576673855677545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583238131496013849n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576576673855677545n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583405329217877113n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457582794471900706471, 115792089237316195423570985008687907853269984665640564039457582794471900706475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582794471900706471n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582794471900706475n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582794471900706479n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457582794471900706475, 115792089237316195423570985008687907853269984665640564039457582794471900706475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582794471900706475n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582794471900706475n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582794471900706475n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457582794471900706475, 115792089237316195423570985008687907853269984665640564039457582794471900706471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582794471900706475n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582794471900706471n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582794471900706479n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577930293493540127, 115792089237316195423570985008687907853269984665640564039457576576673855677545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457576576673855677545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577930293493540127n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579497166388831615n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457582794471900706471, 115792089237316195423570985008687907853269984665640564039457582794471900706475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582794471900706475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582794471900706471n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582794471900706479n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457582794471900706475, 115792089237316195423570985008687907853269984665640564039457582794471900706475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582794471900706475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582794471900706475n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582794471900706475n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457582794471900706475, 115792089237316195423570985008687907853269984665640564039457582794471900706471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582794471900706471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582794471900706475n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582794471900706479n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575053127883308623, 115792089237316195423570985008687907853269984665640564039457582693062447420985)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575053127883308623n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582693062447420985n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(7735673892989046n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575053127883308619, 115792089237316195423570985008687907853269984665640564039457575053127883308623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575053127883308619n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575053127883308623n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575053127883308623, 115792089237316195423570985008687907853269984665640564039457575053127883308623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575053127883308623n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575053127883308623n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575053127883308623, 115792089237316195423570985008687907853269984665640564039457575053127883308619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575053127883308623n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575053127883308619n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579603747779953889, 115792089237316195423570985008687907853269984665640564039457582693062447420985)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582693062447420985n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579603747779953889n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(3111931991528152n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575053127883308619, 115792089237316195423570985008687907853269984665640564039457575053127883308623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575053127883308623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575053127883308619n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575053127883308623, 115792089237316195423570985008687907853269984665640564039457575053127883308623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575053127883308623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575053127883308623n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575053127883308623, 115792089237316195423570985008687907853269984665640564039457575053127883308619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575053127883308619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575053127883308623n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582065856019738465, 115792089237316195423570985008687907853269984665640564039457583089948692960283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582065856019738465n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457583089948692960283n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457582065856019738461, 115792089237316195423570985008687907853269984665640564039457582065856019738465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582065856019738461n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582065856019738465n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457582065856019738465, 115792089237316195423570985008687907853269984665640564039457582065856019738465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582065856019738465n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582065856019738465n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457582065856019738465, 115792089237316195423570985008687907853269984665640564039457582065856019738461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582065856019738465n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582065856019738461n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457578146759630923049, 115792089237316195423570985008687907853269984665640564039457583089948692960283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457583089948692960283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578146759630923049n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457582065856019738461, 115792089237316195423570985008687907853269984665640564039457582065856019738465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582065856019738465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582065856019738461n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457582065856019738465, 115792089237316195423570985008687907853269984665640564039457582065856019738465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582065856019738465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582065856019738465n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457582065856019738465, 115792089237316195423570985008687907853269984665640564039457582065856019738461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582065856019738461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582065856019738465n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575918633215654475, 115792089237316195423570985008687907853269984665640564039457577056640214377539)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575918633215654475n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577056640214377539n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575918633215654471, 115792089237316195423570985008687907853269984665640564039457575918633215654475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575918633215654471n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575918633215654475n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575918633215654475, 115792089237316195423570985008687907853269984665640564039457575918633215654475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575918633215654475n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575918633215654475n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575918633215654475, 115792089237316195423570985008687907853269984665640564039457575918633215654471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575918633215654475n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575918633215654471n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583650031168556033, 115792089237316195423570985008687907853269984665640564039457577056640214377539)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577056640214377539n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457583650031168556033n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575918633215654471, 115792089237316195423570985008687907853269984665640564039457575918633215654475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575918633215654475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575918633215654471n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575918633215654475, 115792089237316195423570985008687907853269984665640564039457575918633215654475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575918633215654475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575918633215654475n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575918633215654475, 115792089237316195423570985008687907853269984665640564039457575918633215654471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575918633215654471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575918633215654475n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577616693175358637, 115792089237316195423570985008687907853269984665640564039457581347336976584833)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577616693175358637n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457581347336976584833n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457577616693175358633, 115792089237316195423570985008687907853269984665640564039457577616693175358637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577616693175358633n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577616693175358637n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457577616693175358637, 115792089237316195423570985008687907853269984665640564039457577616693175358637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577616693175358637n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577616693175358637n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457577616693175358637, 115792089237316195423570985008687907853269984665640564039457577616693175358633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577616693175358637n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577616693175358633n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457578018264760015699, 115792089237316195423570985008687907853269984665640564039457581347336976584833)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457581347336976584833n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578018264760015699n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457577616693175358633, 115792089237316195423570985008687907853269984665640564039457577616693175358637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577616693175358637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577616693175358633n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457577616693175358637, 115792089237316195423570985008687907853269984665640564039457577616693175358637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577616693175358637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577616693175358637n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457577616693175358637, 115792089237316195423570985008687907853269984665640564039457577616693175358633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577616693175358633n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577616693175358637n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582931411410180425, 115792089237316195423570985008687907853269984665640564039457580507830604829387)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582931411410180425n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580507830604829387n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457578248801468140037, 115792089237316195423570985008687907853269984665640564039457578248801468140041)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578248801468140037n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578248801468140041n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457578248801468140041, 115792089237316195423570985008687907853269984665640564039457578248801468140041)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578248801468140041n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578248801468140041n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457578248801468140041, 115792089237316195423570985008687907853269984665640564039457578248801468140037)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578248801468140041n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578248801468140037n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582637907791232553, 115792089237316195423570985008687907853269984665640564039457580507830604829387)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580507830604829387n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582637907791232553n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457578248801468140037, 115792089237316195423570985008687907853269984665640564039457578248801468140041)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578248801468140041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578248801468140037n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457578248801468140041, 115792089237316195423570985008687907853269984665640564039457578248801468140041)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578248801468140041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578248801468140041n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457578248801468140041, 115792089237316195423570985008687907853269984665640564039457578248801468140037)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578248801468140037n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578248801468140041n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575943118443967733, 115792089237316195423570985008687907853269984665640564039457583384895141454393)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575943118443967733n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457583384895141454393n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575943118443967729, 115792089237316195423570985008687907853269984665640564039457575943118443967733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575943118443967729n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575943118443967733n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575943118443967733, 115792089237316195423570985008687907853269984665640564039457575943118443967733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575943118443967733n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575943118443967733n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575943118443967733, 115792089237316195423570985008687907853269984665640564039457575943118443967729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575943118443967733n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575943118443967729n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581662363772747145, 115792089237316195423570985008687907853269984665640564039457583384895141454393)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457583384895141454393n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457581662363772747145n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575943118443967729, 115792089237316195423570985008687907853269984665640564039457575943118443967733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575943118443967733n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575943118443967729n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575943118443967733, 115792089237316195423570985008687907853269984665640564039457575943118443967733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575943118443967733n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575943118443967733n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575943118443967733, 115792089237316195423570985008687907853269984665640564039457575943118443967729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575943118443967729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575943118443967733n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575244791569769895, 115792089237316195423570985008687907853269984665640564039457583152934737488999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575244791569769895n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457583152934737488999n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575244791569769891, 115792089237316195423570985008687907853269984665640564039457575244791569769895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575244791569769891n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575244791569769895n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575244791569769895, 115792089237316195423570985008687907853269984665640564039457575244791569769895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575244791569769895n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575244791569769895n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575244791569769895, 115792089237316195423570985008687907853269984665640564039457575244791569769891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575244791569769895n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575244791569769891n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582076191045123599, 115792089237316195423570985008687907853269984665640564039457583152934737488999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457583152934737488999n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582076191045123599n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575244791569769891, 115792089237316195423570985008687907853269984665640564039457575244791569769895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575244791569769895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575244791569769891n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575244791569769895, 115792089237316195423570985008687907853269984665640564039457575244791569769895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575244791569769895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575244791569769895n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575244791569769895, 115792089237316195423570985008687907853269984665640564039457575244791569769891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575244791569769891n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575244791569769895n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578019618244793955, 115792089237316195423570985008687907853269984665640564039457578265266508789443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578019618244793955n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578265266508789443n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578019618244793955n);
  });

  it('test operator "min" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457578019618244793951, 115792089237316195423570985008687907853269984665640564039457578019618244793955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578019618244793951n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578019618244793955n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578019618244793951n);
  });

  it('test operator "min" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457578019618244793955, 115792089237316195423570985008687907853269984665640564039457578019618244793955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578019618244793955n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578019618244793955n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578019618244793955n);
  });

  it('test operator "min" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457578019618244793955, 115792089237316195423570985008687907853269984665640564039457578019618244793951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578019618244793955n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578019618244793951n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578019618244793951n);
  });

  it('test operator "min" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579893020183470257, 115792089237316195423570985008687907853269984665640564039457578265266508789443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578265266508789443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579893020183470257n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578265266508789443n);
  });

  it('test operator "min" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457578019618244793951, 115792089237316195423570985008687907853269984665640564039457578019618244793955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578019618244793955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578019618244793951n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578019618244793951n);
  });

  it('test operator "min" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457578019618244793955, 115792089237316195423570985008687907853269984665640564039457578019618244793955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578019618244793955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578019618244793955n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578019618244793955n);
  });

  it('test operator "min" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457578019618244793955, 115792089237316195423570985008687907853269984665640564039457578019618244793951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578019618244793951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578019618244793955n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578019618244793951n);
  });
});
